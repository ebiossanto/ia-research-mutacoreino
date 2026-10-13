#!/usr/bin/env python3
"""
Testes de Estabilidade Bijectiva do Cifrador SPECTRUM-1024

Implementa validação contínua da prova de estabilidade bijectiva
para o cifrador SPECTRUM-1024, usado na Camada 1 (Substrato Criptográfico).

Testes incluem:
- Verificação de bijectividade em todas as rodadas
- Análise de avalanche (difusão)
- Resistência a ataques diferenciais/lineares
- Validação de propriedades algebraicas
- Testes de propriedade de permutação
"""

import numpy as np
import hashlib
import secrets
import logging
from dataclasses import dataclass, asdict
from typing import List, Tuple, Dict, Optional, Callable
from pathlib import Path
import json
import argparse
from abc import ABC, abstractmethod

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


# ============================================================================
# SPECTRUM-1024 Core Implementation
# ============================================================================

BLOCK_SIZE = 128  # bytes = 1024 bits
ROUNDS = 16
KEY_SIZE = 128    # bytes

# Constantes S-box (derivadas de curvas elípticas)
SBOX = np.array([
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5,
    0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0,
    0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    # ... (completar 256 valores)
], dtype=np.uint8)

# Completar S-box com valores padrão AES-like se necessário
if len(SBOX) < 256:
    # Gerar S-box completa baseada em inverso multiplicativo em GF(2^8)
    def gf_inv(x: int) -> int:
        if x == 0:
            return 0
        # Inverso multiplicativo em GF(2^8) com polinômio x^8 + x^4 + x^3 + x + 1
        for y in range(1, 256):
            if gf_mult(x, y) == 1:
                return y
        return 0

    def gf_mult(a: int, b: int) -> int:
        result = 0
        while b:
            if b & 1:
                result ^= a
            a = (a << 1) ^ (0x11B if (a & 0x80) else 0)
            b >>= 1
        return result

    def affine_transform(x: int) -> int:
        y = x
        for i in range(4):
            y ^= ((x >> (i + 4)) & 0x0F) | ((x << (4 - i)) & 0xF0)
        return y ^ 0x63

    SBOX_FULL = np.array([affine_transform(gf_inv(i)) for i in range(256)], dtype=np.uint8)
else:
    SBOX_FULL = SBOX


INV_SBOX_FULL = np.zeros(256, dtype=np.uint8)
for i in range(256):
    INV_SBOX_FULL[SBOX_FULL[i]] = i


# Matriz MDS para MixColumns (Cauchy matrix)
MDS_MATRIX = np.array([
    [0x02, 0x03, 0x01, 0x01],
    [0x01, 0x02, 0x03, 0x01],
    [0x01, 0x01, 0x02, 0x03],
    [0x03, 0x01, 0x01, 0x02],
], dtype=np.uint8)

INV_MDS_MATRIX = np.array([
    [0x0E, 0x0B, 0x0D, 0x09],
    [0x09, 0x0E, 0x0B, 0x0D],
    [0x0D, 0x09, 0x0E, 0x0B],
    [0x0B, 0x0D, 0x09, 0x0E],
], dtype=np.uint8)


def gf_mult(a: int, b: int) -> int:
    """Multiplicação em GF(2^8)"""
    result = 0
    a = a & 0xFF
    b = b & 0xFF
    while b:
        if b & 1:
            result ^= a
        hi_bit = a & 0x80
        a = (a << 1) & 0xFF
        if hi_bit:
            a ^= 0x1B
        b >>= 1
    return result


def sub_bytes(state: np.ndarray) -> np.ndarray:
    """SubBytes: aplica S-box a cada byte"""
    return SBOX_FULL[state]


def inv_sub_bytes(state: np.ndarray) -> np.ndarray:
    """InvSubBytes: aplica S-box inversa"""
    return INV_SBOX_FULL[state]


def shift_rows(state: np.ndarray) -> np.ndarray:
    """ShiftRows: permuta linhas da matriz 16x8"""
    # state shape: (16, 8) - 16 linhas, 8 colunas (128 bytes)
    result = state.copy()
    for r in range(16):
        shift = (r % 4) * 2  # Padrão de shift personalizado
        if shift > 0:
            result[r] = np.roll(state[r], -shift)
    return result


def inv_shift_rows(state: np.ndarray) -> np.ndarray:
    """InvShiftRows"""
    result = state.copy()
    for r in range(16):
        shift = (r % 4) * 2
        if shift > 0:
            result[r] = np.roll(state[r], shift)
    return result


def mix_columns(state: np.ndarray) -> np.ndarray:
    """MixColumns: multiplicação matricial MDS por coluna"""
    result = np.zeros_like(state)
    # Processa cada coluna de 4 bytes independentemente
    for c in range(8):
        col = state[:, c].reshape(4, 4)  # 4 grupos de 4
        for i in range(4):
            for j in range(4):
                val = 0
                for k in range(4):
                    val ^= gf_mult(MDS_MATRIX[i, k], col[k, j])
                result[i*4:(i+1)*4, c] ^= (val & 0xFF)
    return result


def inv_mix_columns(state: np.ndarray) -> np.ndarray:
    """InvMixColumns"""
    result = np.zeros_like(state)
    for c in range(8):
        col = state[:, c].reshape(4, 4)
        for i in range(4):
            for j in range(4):
                val = 0
                for k in range(4):
                    val ^= gf_mult(INV_MDS_MATRIX[i, k], col[k, j])
                result[i*4:(i+1)*4, c] ^= (val & 0xFF)
    return result


def add_round_key(state: np.ndarray, round_key: np.ndarray) -> np.ndarray:
    """AddRoundKey: XOR com chave de rodada"""
    return state ^ round_key


def key_expansion(master_key: bytes) -> List[np.ndarray]:
    """
    Expansão de chave para SPECTRUM-1024.
    Gera ROUNDS+1 chaves de rodada de 128 bytes cada.
    """
    if len(master_key) != KEY_SIZE:
        raise ValueError(f"Chave deve ter {KEY_SIZE} bytes")

    # Converte para array de bytes
    key_bytes = np.frombuffer(master_key, dtype=np.uint8)

    # Gera material de chave usando HKDF-like derivation
    round_keys = []
    current = key_bytes.copy()

    for round_num in range(ROUNDS + 1):
        # Deriva próxima chave usando hash criptográfico
        hasher = hashlib.blake2b(digest_size=KEY_SIZE)
        hasher.update(current.tobytes())
        hasher.update(round_num.to_bytes(4, 'big'))
        hasher.update(b"SPECTRUM-1024-KEY-SCHEDULE")
        next_key = np.frombuffer(hasher.digest(), dtype=np.uint8)

        round_keys.append(current.reshape(16, 8))
        current = next_key

    return round_keys


def spectrum1024_encrypt_block(plaintext: bytes, master_key: bytes) -> bytes:
    """Criptografa um bloco de 128 bytes"""
    if len(plaintext) != BLOCK_SIZE:
        raise ValueError(f"Bloco deve ter {BLOCK_SIZE} bytes")

    if len(master_key) != KEY_SIZE:
        raise ValueError(f"Chave deve ter {KEY_SIZE} bytes")

    # Estado inicial: matriz 16x8
    state = np.frombuffer(plaintext, dtype=np.uint8).reshape(16, 8)
    round_keys = key_expansion(master_key)

    # Rodada inicial
    state = add_round_key(state, round_keys[0])

    # Rodadas principais
    for round_num in range(1, ROUNDS):
        state = sub_bytes(state)
        state = shift_rows(state)
        state = mix_columns(state)
        state = add_round_key(state, round_keys[round_num])

    # Rodada final (sem MixColumns)
    state = sub_bytes(state)
    state = shift_rows(state)
    state = add_round_key(state, round_keys[ROUNDS])

    return state.tobytes()


def spectrum1024_decrypt_block(ciphertext: bytes, master_key: bytes) -> bytes:
    """Descriptografa um bloco de 128 bytes"""
    if len(ciphertext) != BLOCK_SIZE:
        raise ValueError(f"Bloco deve ter {BLOCK_SIZE} bytes")

    state = np.frombuffer(ciphertext, dtype=np.uint8).reshape(16, 8)
    round_keys = key_expansion(master_key)

    # Rodada final inversa
    state = add_round_key(state, round_keys[ROUNDS])
    state = inv_shift_rows(state)
    state = inv_sub_bytes(state)

    # Rodadas principais inversas
    for round_num in range(ROUNDS - 1, 0, -1):
        state = add_round_key(state, round_keys[round_num])
        state = inv_mix_columns(state)
        state = inv_shift_rows(state)
        state = inv_sub_bytes(state)

    # Rodada inicial inversa
    state = add_round_key(state, round_keys[0])

    return state.tobytes()


# ============================================================================
# Test Framework
# ============================================================================

@dataclass
class TestResult:
    name: str
    passed: bool
    details: str
    metrics: Dict[str, float]


class SpectrumTest(ABC):
    """Classe base para testes do SPECTRUM-1024"""

    @abstractmethod
    def run(self) -> TestResult:
        pass


class BijectivityTest(SpectrumTest):
    """Teste de bijectividade: cada entrada única mapeia para saída única"""

    def __init__(self, num_samples: int = 1000):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de bijectividade ({self.num_samples} amostras)...")

        key = secrets.token_bytes(KEY_SIZE)
        seen_outputs = set()
        collisions = 0

        for _ in range(self.num_samples):
            plaintext = secrets.token_bytes(BLOCK_SIZE)
            ciphertext = spectrum1024_encrypt_block(plaintext, key)

            if ciphertext in seen_outputs:
                collisions += 1
            seen_outputs.add(ciphertext)

        passed = collisions == 0
        details = f"Colisões: {collisions}/{self.num_samples}"

        return TestResult(
            name="Bijectivity Test",
            passed=passed,
            details=details,
            metrics={"collisions": collisions, "samples": self.num_samples, "collision_rate": collisions/self.num_samples}
        )


class RoundTripTest(SpectrumTest):
    """Teste de ida-e-volta: decrypt(encrypt(x)) == x"""

    def __init__(self, num_samples: int = 1000):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de round-trip ({self.num_samples} amostras)...")

        failures = 0
        for _ in range(self.num_samples):
            key = secrets.token_bytes(KEY_SIZE)
            plaintext = secrets.token_bytes(BLOCK_SIZE)

            ciphertext = spectrum1024_encrypt_block(plaintext, key)
            decrypted = spectrum1024_decrypt_block(ciphertext, key)

            if decrypted != plaintext:
                failures += 1

        passed = failures == 0
        return TestResult(
            name="Round-trip Test",
            passed=passed,
            details=f"Falhas: {failures}/{self.num_samples}",
            metrics={"failures": failures, "samples": self.num_samples}
        )


class AvalancheTest(SpectrumTest):
    """Teste de efeito avalanche: 1 bit flip → ~50% bits mudam"""

    def __init__(self, num_samples: int = 500):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de avalanche ({self.num_samples} amostras)...")

        bit_changes = []

        for _ in range(self.num_samples):
            key = secrets.token_bytes(KEY_SIZE)
            plaintext = secrets.token_bytes(BLOCK_SIZE)

            ciphertext1 = spectrum1024_encrypt_block(plaintext, key)

            # Flip 1 bit aleatório
            byte_idx = secrets.randbelow(BLOCK_SIZE)
            bit_idx = secrets.randbelow(8)
            modified = bytearray(plaintext)
            modified[byte_idx] ^= (1 << bit_idx)

            ciphertext2 = spectrum1024_encrypt_block(bytes(modified), key)

            # Conta bits diferentes
            diff = np.frombuffer(ciphertext1, dtype=np.uint8) ^ np.frombuffer(ciphertext2, dtype=np.uint8)
            changed_bits = np.sum([bin(b).count('1') for b in diff])
            bit_changes.append(changed_bits)

        avg_change = np.mean(bit_changes)
        expected = BLOCK_SIZE * 8 * 0.5  # 50% dos 1024 bits
        deviation = abs(avg_change - expected) / expected

        # Passa se desvio < 10%
        passed = deviation < 0.10

        return TestResult(
            name="Avalanche Test",
            passed=passed,
            details=f"Média bits alterados: {avg_change:.1f}/{expected} (desvio: {deviation*100:.1f}%)",
            metrics={
                "avg_bits_changed": avg_change,
                "expected": expected,
                "deviation_pct": deviation * 100,
                "min_changed": min(bit_changes),
                "max_changed": max(bit_changes)
            }
        )


class KeyAvalancheTest(SpectrumTest):
    """Teste de avalanche de chave: 1 bit na chave → ~50% bits mudam no output"""

    def __init__(self, num_samples: int = 500):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de avalanche de chave ({self.num_samples} amostras)...")

        bit_changes = []

        for _ in range(self.num_samples):
            plaintext = secrets.token_bytes(BLOCK_SIZE)
            key1 = secrets.token_bytes(KEY_SIZE)

            ciphertext1 = spectrum1024_encrypt_block(plaintext, key1)

            # Flip 1 bit na chave
            byte_idx = secrets.randbelow(KEY_SIZE)
            bit_idx = secrets.randbelow(8)
            key2 = bytearray(key1)
            key2[byte_idx] ^= (1 << bit_idx)

            ciphertext2 = spectrum1024_encrypt_block(plaintext, bytes(key2))

            diff = np.frombuffer(ciphertext1, dtype=np.uint8) ^ np.frombuffer(ciphertext2, dtype=np.uint8)
            changed_bits = np.sum([bin(b).count('1') for b in diff])
            bit_changes.append(changed_bits)

        avg_change = np.mean(bit_changes)
        expected = BLOCK_SIZE * 8 * 0.5
        deviation = abs(avg_change - expected) / expected

        passed = deviation < 0.15  # Chave pode ter desvio maior

        return TestResult(
            name="Key Avalanche Test",
            passed=passed,
            details=f"Média bits alterados: {avg_change:.1f}/{expected} (desvio: {deviation*100:.1f}%)",
            metrics={
                "avg_bits_changed": avg_change,
                "expected": expected,
                "deviation_pct": deviation * 100
            }
        )


class DifferentialUniformityTest(SpectrumTest):
    """Teste de uniformidade diferencial (resistência a ataque diferencial)"""

    def __init__(self, num_samples: int = 2000):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de uniformidade diferencial ({self.num_samples} amostras)...")

        # Testa diferença de entrada fixa
        input_diff = np.zeros(BLOCK_SIZE, dtype=np.uint8)
        input_diff[0] = 0x01  # Diferença no primeiro byte

        output_diffs = {}

        for _ in range(self.num_samples):
            key = secrets.token_bytes(KEY_SIZE)
            p1 = secrets.token_bytes(BLOCK_SIZE)
            p2 = bytes(a ^ b for a, b in zip(p1, input_diff))

            c1 = spectrum1024_encrypt_block(p1, key)
            c2 = spectrum1024_encrypt_block(p2, key)

            out_diff = bytes(a ^ b for a, b in zip(c1, c2))
            output_diffs[out_diff] = output_diffs.get(out_diff, 0) + 1

        # Encontra diferença de saída mais provável
        max_prob = max(output_diffs.values()) / self.num_samples
        num_output_diffs = len(output_diffs)

        # Para cifra ideal: max_prob ≈ 1/2^1024 (praticamente 0)
        # Para S-box 8-bit: max diferencial ≈ 4/256 = 1/64
        # Após múltiplas rodadas: deve ser negligenciável
        passed = max_prob < 0.01  # < 1%

        return TestResult(
            name="Differential Uniformity Test",
            passed=passed,
            details=f"Max prob diferencial: {max_prob:.6f}, Saídas únicas: {num_output_diffs}",
            metrics={
                "max_differential_prob": max_prob,
                "unique_output_diffs": num_output_diffs,
                "samples": self.num_samples
            }
        )


class LinearApproximationTest(SpectrumTest):
    """Teste de aproximação linear (resistência a ataque linear)"""

    def __init__(self, num_samples: int = 5000):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de aproximação linear ({self.num_samples} amostras)...")

        # Máscaras lineares aleatórias
        input_mask = np.frombuffer(secrets.token_bytes(BLOCK_SIZE), dtype=np.uint8)
        output_mask = np.frombuffer(secrets.token_bytes(BLOCK_SIZE), dtype=np.uint8)

        bias_sum = 0

        for _ in range(self.num_samples):
            key = secrets.token_bytes(KEY_SIZE)
            plaintext = secrets.token_bytes(BLOCK_SIZE)
            ciphertext = spectrum1024_encrypt_block(plaintext, key)

            # Produto interno (paridade) das máscaras
            p_parity = self._parity(np.frombuffer(plaintext, dtype=np.uint8) & input_mask)
            c_parity = self._parity(np.frombuffer(ciphertext, dtype=np.uint8) & output_mask)

            if p_parity == c_parity:
                bias_sum += 1

        bias = abs(bias_sum / self.num_samples - 0.5)
        # Viés ideal = 0 (probabilidade 0.5)
        passed = bias < 0.02  # < 2% bias

        return TestResult(
            name="Linear Approximation Test",
            passed=passed,
            details=f"Viés linear: {bias:.6f}",
            metrics={"bias": bias, "samples": self.num_samples}
        )

    @staticmethod
    def _parity(arr: np.ndarray) -> int:
        return int(np.sum([bin(b).count('1') for b in arr]) % 2)


class PermutationPropertyTest(SpectrumTest):
    """Testa se a cifra é uma permutação para chave fixa"""

    def __init__(self, num_samples: int = 2000):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando teste de propriedade de permutação ({self.num_samples} amostras)...")

        key = secrets.token_bytes(KEY_SIZE)
        outputs = set()

        for _ in range(self.num_samples):
            plaintext = secrets.token_bytes(BLOCK_SIZE)
            ciphertext = spectrum1024_encrypt_block(plaintext, key)
            outputs.add(ciphertext)

        # Para permutação: todas as saídas devem ser únicas
        unique_outputs = len(outputs)
        passed = unique_outputs == self.num_samples

        return TestResult(
            name="Permutation Property Test",
            passed=passed,
            details=f"Saídas únicas: {unique_outputs}/{self.num_samples}",
            metrics={"unique_outputs": unique_outputs, "samples": self.num_samples}
        )


class RoundFunctionBijectivityTest(SpectrumTest):
    """Testa bijectividade de cada função de rodada isoladamente"""

    def run(self) -> TestResult:
        logger.info("Testando bijectividade de funções de rodada...")

        results = {}
        key = secrets.token_bytes(KEY_SIZE)
        round_keys = key_expansion(key)

        # Testa cada componente
        test_state = np.random.randint(0, 256, (16, 8), dtype=np.uint8)

        # SubBytes
        sub_out = sub_bytes(test_state)
        sub_inv = inv_sub_bytes(sub_out)
        results['sub_bytes'] = np.array_equal(test_state, sub_inv)

        # ShiftRows
        shift_out = shift_rows(test_state)
        shift_inv = inv_shift_rows(shift_out)
        results['shift_rows'] = np.array_equal(test_state, shift_inv)

        # MixColumns
        mix_out = mix_columns(test_state)
        mix_inv = inv_mix_columns(mix_out)
        results['mix_columns'] = np.array_equal(test_state, mix_inv)

        # Round completa (sem key addition)
        state = test_state.copy()
        state = sub_bytes(state)
        state = shift_rows(state)
        state = mix_columns(state)
        state = inv_mix_columns(state)
        state = inv_shift_rows(state)
        state = inv_sub_bytes(state)
        results['full_round_no_key'] = np.array_equal(test_state, state)

        all_passed = all(results.values())

        return TestResult(
            name="Round Function Bijectivity Test",
            passed=all_passed,
            details=f"Resultados: {results}",
            metrics=results
        )


class StatisticalRandomnessTest(SpectrumTest):
    """Testes estatísticos de aleatoriedade no output"""

    def __init__(self, num_samples: int = 10000):
        self.num_samples = num_samples

    def run(self) -> TestResult:
        logger.info(f"Executando testes estatísticos ({self.num_samples} amostras)...")

        key = secrets.token_bytes(KEY_SIZE)
        all_bytes = []

        for _ in range(self.num_samples):
            plaintext = secrets.token_bytes(BLOCK_SIZE)
            ciphertext = spectrum1024_encrypt_block(plaintext, key)
            all_bytes.extend(ciphertext)

        all_bytes = np.array(all_bytes, dtype=np.uint8)

        # Teste qui-quadrado para distribuição uniforme
        observed, _ = np.histogram(all_bytes, bins=256, range=(0, 256))
        expected = len(all_bytes) / 256
        chi2 = np.sum((observed - expected) ** 2 / expected)

        # Graus de liberdade = 255
        # Valor crítico para p=0.05: ~293
        passed = chi2 < 350  # Margem de segurança

        # Entropia de Shannon
        probs = observed / len(all_bytes)
        probs = probs[probs > 0]
        entropy = -np.sum(probs * np.log2(probs))
        max_entropy = 8.0  # bits por byte

        return TestResult(
            name="Statistical Randomness Test",
            passed=passed,
            details=f"Chi²: {chi2:.1f}, Entropia: {entropy:.4f}/{max_entropy}",
            metrics={
                "chi_squared": chi2,
                "entropy": entropy,
                "max_entropy": max_entropy,
                "efficiency": entropy / max_entropy
            }
        )


class SpectreStabilityTest(SpectrumTest):
    """Teste de estabilidade da prova bijectiva sob composição"""

    def __init__(self, num_iterations: int = 100):
        self.num_iterations = num_iterations

    def run(self) -> TestResult:
        logger.info(f"Executando teste de estabilidade da prova ({self.num_iterations} iterações)...")

        key = secrets.token_bytes(KEY_SIZE)
        plaintext = secrets.token_bytes(BLOCK_SIZE)

        # Aplica cifra múltiplas vezes
        current = plaintext
        states = {current}

        for i in range(self.num_iterations):
            current = spectrum1024_encrypt_block(current, key)
            if current in states:
                return TestResult(
                    name="Spectre Stability Test",
                    passed=False,
                    details=f"Ciclo detectado na iteração {i}",
                    metrics={"cycle_at": i, "unique_states": len(states)}
                )
            states.add(current)

        return TestResult(
            name="Spectre Stability Test",
            passed=True,
            details=f"Nenhum ciclo em {self.num_iterations} iterações",
            metrics={"iterations": self.num_iterations, "unique_states": len(states)}
        )


# ============================================================================
# Test Runner
# ============================================================================

class SpectrumTestSuite:
    """Suite completa de testes SPECTRUM-1024"""

    def __init__(self):
        self.tests: List[SpectrumTest] = []
        self.results: List[TestResult] = []

    def add_test(self, test: SpectrumTest):
        self.tests.append(test)

    def run_all(self) -> Dict:
        """Executa todos os testes"""
        logger.info("=" * 60)
        logger.info("SPECTRUM-1024 Bijective Stability Test Suite")
        logger.info("=" * 60)

        self.results = []
        passed = 0
        failed = 0

        for test in self.tests:
            try:
                result = test.run()
                self.results.append(result)
                if result.passed:
                    logger.info(f"  ✓ {result.name}: {result.details}")
                    passed += 1
                else:
                    logger.error(f"  ✗ {result.name}: {result.details}")
                    failed += 1
            except Exception as e:
                logger.exception(f"  ✗ {test.__class__.__name__}: Erro - {e}")
                self.results.append(TestResult(
                    name=test.__class__.__name__,
                    passed=False,
                    details=f"Exception: {e}",
                    metrics={}
                ))
                failed += 1

        logger.info("=" * 60)
        logger.info(f"RESULTADO: {passed} passed, {failed} failed")
        logger.info("=" * 60)

        return {
            "total": len(self.tests),
            "passed": passed,
            "failed": failed,
            "success_rate": passed / len(self.tests) if self.tests else 0,
            "results": [asdict(r) for r in self.results]
        }

    def export_results(self, filepath: str):
        """Exporta resultados para JSON"""
        data = {
            "summary": {
                "total": len(self.tests),
                "passed": sum(1 for r in self.results if r.passed),
                "failed": sum(1 for r in self.results if not r.passed)
            },
            "results": [asdict(r) for r in self.results]
        }
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
        logger.info(f"Resultados exportados para {filepath}")


def create_default_suite() -> SpectrumTestSuite:
    """Cria suite padrão de testes"""
    suite = SpectrumTestSuite()
    suite.add_test(BijectivityTest(1000))
    suite.add_test(RoundTripTest(1000))
    suite.add_test(AvalancheTest(500))
    suite.add_test(KeyAvalancheTest(500))
    suite.add_test(DifferentialUniformityTest(2000))
    suite.add_test(LinearApproximationTest(5000))
    suite.add_test(PermutationPropertyTest(2000))
    suite.add_test(RoundFunctionBijectivityTest())
    suite.add_test(StatisticalRandomnessTest(10000))
    suite.add_test(SpectreStabilityTest(100))
    return suite


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="SPECTRUM-1024 Bijective Stability Tests")
    parser.add_argument("--suite", choices=["default", "quick", "full"], default="default")
    parser.add_argument("--output", type=str, help="Arquivo de saída JSON")
    parser.add_argument("--samples", type=int, help="Número de amostras (override)")

    args = parser.parse_args()

    if args.suite == "quick":
        suite = SpectrumTestSuite()
        suite.add_test(BijectivityTest(100))
        suite.add_test(RoundTripTest(100))
        suite.add_test(AvalancheTest(50))
    elif args.suite == "full":
        suite = create_default_suite()
        # Aumenta amostras
        for test in suite.tests:
            if hasattr(test, 'num_samples'):
                test.num_samples *= 5
    else:
        suite = create_default_suite()

    results = suite.run_all()

    if args.output:
        suite.export_results(args.output)

    # Exit code baseado em resultado
    exit(0 if results["failed"] == 0 else 1)