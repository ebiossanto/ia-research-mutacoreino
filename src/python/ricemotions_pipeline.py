#!/usr/bin/env python3
"""
Ricemotions Pipeline — v0.8.0
PIXEL → RIC → TEOA com monitoramento de Landauer
Target: < 100ms por ciclo metabólico
"""

import numpy as np
import time
import logging
import hashlib
from dataclasses import dataclass, field
from typing import List, Tuple, Optional, Dict
from enum import Enum
import json

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Constantes de Landauer
K_B = 1.380649e-23
LN2 = np.log(2)


class EmotionalState(Enum):
    HOMEOSTASIS = "HOMEOSTASIS"
    EXPLORATION = "EXPLORATION"
    ANXIETY = "ANXIETY"
    CURIOSITY = "CURIOSITY"
    SATISFACTION = "SATISFACTION"


@dataclass
class PixelState:
    """Stage 1: Representação visual/espacial do estado"""
    tensor: np.ndarray          # (64, 64) visual tensor
    timestamp: int
    entropy: float
    emotional_signature: str
    metadata: Dict = field(default_factory=dict)


@dataclass
class RicCode:
    """Stage 2: Código Informacional Reduzido (compressão com perda controlada)"""
    compressed_ast: str
    compression_ratio: float
    information_loss: float
    semantic_hash: str
    metadata: Dict = field(default_factory=dict)


@dataclass
class TeoaObject:
    """Stage 3: Teoria Epistemológica de Objetos Abstratos"""
    concept_id: str
    semantic_vector: np.ndarray  # (512,)
    epistemic_confidence: float
    abstract_relations: List[Tuple[str, str]]
    metadata: Dict = field(default_factory=dict)


@dataclass
class LandauerMonitor:
    """Monitoramento de Landauer por estágio"""
    pixel_to_ric_loss: float
    ric_to_teoa_loss: float
    total_loss: float
    threshold: float = 1e-18  # attojoule
    
    def check_threshold(self) -> bool:
        return self.total_loss > self.threshold


@dataclass
class RicemotionsPipeline:
    """Pipeline completo PIXEL → RIC → TEOA"""
    pixel_input: PixelState
    ric_intermediate: RicCode
    teoa_output: TeoaObject
    landauer_monitor: LandauerMonitor
    metabolic_state: 'EmotionalState'
    cycle_time_ms: int
    timestamp: int = field(default_factory=lambda: int(time.time()))


def landauer_loss(input_bits: float, output_bits: float, temperature: float = 300.0) -> float:
    """Energia de Landauer por bit perdido"""
    if input_bits <= output_bits:
        return 0.0
    return (input_bits - output_bits) * K_B * temperature * LN2


def pixel_encode(organism_state: str, exogenous_data: str) -> PixelState:
    """
    Stage 1: Visual/Spatial encoding
    Converte estado do organismo + dados exógenos em tensor visual 64x64
    """
    # Seed determinístico para reprodutibilidade
    seed = hashlib.sha256(f"{organism_state}{exogenous_data}".encode()).hexdigest()
    rng = np.random.default_rng(int(seed[:16], 16))
    
    # Gera tensor 64x64 baseado no estado
    tensor = rng.normal(0, 1, size=(64, 64)).astype(np.float32)
    
    # Entropia visual
    abs_tensor = np.abs(tensor) + 1e-10
    entropy = float(-np.sum(tensor * np.log(abs_tensor)))
    
    # Assinatura emocional baseada em estatísticas do tensor
    emotional_sig = hashlib.md5(tensor.tobytes()).hexdigest()[:16]
    
    return PixelState(
        tensor=tensor,
        timestamp=int(time.time()),
        entropy=entropy,
        emotional_signature=emotional_sig,
        metadata={"organism_state": organism_state, "exogenous_data": exogenous_data}
    )


def ric_compress(pixel: PixelState, ast: str) -> RicCode:
    """
    Stage 2: AST → RIC compression com monitoramento de Landauer
    Comprime AST mantendo hash semântico
    """
    # Bits de entrada: tensor 64x64 float32
    input_bits = 64 * 64 * 32
    
    # Compressão: mantém apenas hash semântico + estrutura essencial
    semantic_hash = hashlib.sha256(f"{ast}{pixel.emotional_signature}".encode()).hexdigest()
    compressed_ast = ast[:1024]  # Trunca para ~1KB
    
    output_bits = 1024 * 8  # 1KB compressed
    loss = landauer_loss(input_bits, output_bits)
    
    compression_ratio = output_bits / input_bits
    
    return RicCode(
        compressed_ast=compressed_ast,
        compression_ratio=compression_ratio,
        information_loss=loss,
        semantic_hash=semantic_hash,
        metadata={"original_ast_len": len(ast), "pixel_entropy": pixel.entropy}
    )


def teoa_map(ric: RicCode) -> TeoaObject:
    """
    Stage 3: RIC → TEOA semantic mapping
    Mapeia código reduzido para objetos epistemológicos abstratos
    """
    input_bits = 1024 * 8
    
    # Gera vetor semântico 512-dim determinístico
    seed = int(ric.semantic_hash[:16], 16)
    rng = np.random.default_rng(seed)
    semantic_vector = rng.normal(0, 1, size=512).astype(np.float32)
    # Normaliza
    semantic_vector = semantic_vector / (np.linalg.norm(semantic_vector) + 1e-10)
    
    output_bits = 512 * 32
    loss = landauer_loss(input_bits, output_bits)
    
    # Conceito baseado no hash
    concept_id = f"concept_{ric.semantic_hash[:8]}"
    
    # Confiança epistemológica baseada na compressão
    epistemic_confidence = max(0.5, 1.0 - ric.compression_ratio)
    
    return TeoaObject(
        concept_id=concept_id,
        semantic_vector=semantic_vector,
        epistemic_confidence=epistemic_confidence,
        abstract_relations=[],
        metadata={"ric_hash": ric.semantic_hash, "ric_loss": ric.information_loss}
    )


def metabolic_to_emotional(landauer_residue: float, delta_t_magnitude: float) -> 'EmotionalState':
    """Mapeia resíduo de Landauer + ΔT para estado emocional"""
    from enum import Enum
    
    class EmotionalState(Enum):
        HOMEOSTASIS = "HOMEOSTASIS"
        EXPLORATION = "EXPLORATION"
        ANXIETY = "ANXIETY"
        CURIOSITY = "CURIOSITY"
        SATISFACTION = "SATISFACTION"
    
    if landauer_residue > 1e-18 or delta_t_magnitude > 10000.0:
        return EmotionalState.ANXIETY
    elif delta_t_magnitude > 100.0:
        return EmotionalState.EXPLORATION
    elif delta_t_magnitude > 10.0:
        return EmotionalState.CURIOSITY
    elif landauer_residue < 1e-20 and delta_t_magnitude < 10.0:
        return EmotionalState.SATISFACTION
    else:
        return EmotionalState.HOMEOSTASIS


def run_ricemotions_cycle(
    organism_state: str,
    exogenous_data: str,
    temperature: float = 300.0
) -> Dict:
    """
    Executa pipeline completo PIXEL → RIC → TEOA
    Retorna dicionário com resultados detalhados
    """
    start = time.perf_counter()
    
    # Stage 1: PIXEL
    pixel = pixel_encode(organism_state, exogenous_data)
    
    # Stage 2: RIC
    ric = ric_compress(pixel, organism_state)
    
    # Stage 3: TEOA
    teoa = teoa_map(ric)
    
    # Landauer accounting detalhado
    pixel_bits = 64 * 64 * 32      # 64x64 float32
    ric_bits = 1024 * 8            # 1KB compressed
    teoa_bits = 512 * 32           # 512-dim float32
    
    loss_1 = landauer_loss(pixel_bits, ric_bits, temperature)
    loss_2 = landauer_loss(ric_bits, teoa_bits, temperature)
    total_loss = loss_1 + loss_2
    
    landauer_monitor = LandauerMonitor(
        pixel_to_ric_loss=loss_1,
        ric_to_teoa_loss=loss_2,
        total_loss=total_loss
    )
    
    # Estado emocional a partir da termodinâmica
    delta_t_mag = float(np.sum(pixel.tensor**2))
    emotional_state = metabolic_to_emotional(total_loss, delta_t_mag)
    
    cycle_time_ms = int((time.perf_counter() - start) * 1000)
    
    pipeline = RicemotionsPipeline(
        pixel_input=pixel,
        ric_intermediate=ric,
        teoa_output=teoa,
        landauer_monitor=landauer_monitor,
        metabolic_state=emotional_state,
        cycle_time_ms=cycle_time_ms
    )
    
    # Retorna dicionário serializável
    return {
        "pipeline": pipeline,
        "pixel": {
            "entropy": pixel.entropy,
            "shape": pixel.tensor.shape,
            "emotional_signature": pixel.emotional_signature
        },
        "ric": {
            "compression_ratio": ric.compression_ratio,
            "information_loss": ric.information_loss,
            "semantic_hash": ric.semantic_hash
        },
        "teoa": {
            "concept_id": teoa.concept_id,
            "epistemic_confidence": teoa.epistemic_confidence,
            "vector_norm": float(np.linalg.norm(teoa.semantic_vector))
        },
        "landauer": {
            "pixel_to_ric": loss_1,
            "ric_to_teoa": loss_2,
            "total": total_loss,
            "threshold_check": landauer_monitor.check_threshold(),
            "threshold": landauer_monitor.threshold
        },
        "emotional_state": emotional_state.value,
        "cycle_time_ms": cycle_time_ms
    }


def visualize_emotion(state: 'EmotionalState') -> Dict:
    """Visualização do estado emocional para UI"""
    from enum import Enum
    
    class EmotionalState(Enum):
        HOMEOSTASIS = "HOMEOSTASIS"
        EXPLORATION = "EXPLORATION"
        ANXIETY = "ANXIETY"
        CURIOSITY = "CURIOSITY"
        SATISFACTION = "SATISFACTION"
    
    visualizations = {
        EmotionalState.HOMEOSTASIS: {
            "state": state.value, "color_rgb": (0, 255, 0), 
            "intensity": 0.3, "animation": "breathing"
        },
        EmotionalState.EXPLORATION: {
            "state": state.value, "color_rgb": (0, 100, 255), 
            "intensity": 0.7, "animation": "expanding"
        },
        EmotionalState.ANXIETY: {
            "state": state.value, "color_rgb": (255, 50, 50), 
            "intensity": 0.9, "animation": "pulsing"
        },
        EmotionalState.CURIOSITY: {
            "state": state.value, "color_rgb": (255, 200, 0), 
            "intensity": 0.6, "animation": "sparkling"
        },
        EmotionalState.SATISFACTION: {
            "state": state.value, "color_rgb": (100, 255, 100), 
            "intensity": 0.4, "animation": "glowing"
        }
    }
    return visualizations.get(state, visualizations[EmotionalState.HOMEOSTASIS])


def validate_pipeline(num_trials: int = 100) -> Dict:
    """Validação empírica completa do pipeline"""
    times = []
    losses = []
    states = []
    
    for i in range(num_trials):
        result = run_ricemotions_cycle(f"organism_{i}", f"exogenous_{i}")
        times.append(result["cycle_time_ms"])
        losses.append(result["landauer"]["total"])
        states.append(result["emotional_state"])
    
    from enum import Enum
    class EmotionalState(Enum):
        HOMEOSTASIS = "HOMEOSTASIS"
        EXPLORATION = "EXPLORATION"
        ANXIETY = "ANXIETY"
        CURIOSITY = "CURIOSITY"
        SATISFACTION = "SATISFACTION"
    
    state_dist = {s.value: states.count(s.value) for s in EmotionalState}
    
    return {
        "trials": num_trials,
        "cycle_time_mean": float(np.mean(times)),
        "cycle_time_median": float(np.median(times)),
        "cycle_time_max": int(np.max(times)),
        "cycle_time_p95": float(np.percentile(times, 95)),
        "cycle_time_p99": float(np.percentile(times, 99)),
        "landauer_loss_mean": float(np.mean(losses)),
        "landauer_loss_max": float(np.max(losses)),
        "state_distribution": state_dist,
        "all_under_100ms": all(t < 100 for t in times),
        "all_under_50ms": all(t < 50 for t in times),
        "target_met": np.mean(times) < 100 and np.max(times) < 100,
        "performance_target_met": np.mean(times) < 50 and np.max(times) < 100
    }


def run_continuous_monitoring(duration_seconds: int = 60, interval_ms: int = 1000) -> List[Dict]:
    """Monitoramento contínuo do pipeline"""
    results = []
    end_time = time.time() + duration_seconds
    
    while time.time() < end_time:
        result = run_ricemotions_cycle("monitoring_state", "monitoring_data")
        results.append({
            "timestamp": result["pipeline"].timestamp,
            "emotional_state": result["emotional_state"],
            "landauer_total": result["landauer"]["total"],
            "cycle_time_ms": result["cycle_time_ms"],
            "threshold_alert": result["landauer"]["threshold_check"]
        })
        time.sleep(interval_ms / 1000.0)
    
    return results


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Ricemotions Pipeline v0.8.0")
    parser.add_argument("--trials", type=int, default=100, help="Número de trials para validação")
    parser.add_argument("--validate", action="store_true", help="Executar validação completa")
    parser.add_argument("--monitor", action="store_true", help="Monitoramento contínuo")
    parser.add_argument("--duration", type=int, default=60, help="Duração do monitoramento (segundos)")
    parser.add_argument("--output", type=str, help="Arquivo de saída JSON")
    
    args = parser.parse_args()
    
    if args.validate:
        result = validate_pipeline(args.trials)
        print(f"Validation Result:")
        print(json.dumps(result, indent=2))
        
        if args.output:
            with open(args.output, 'w') as f:
                json.dump(result, f, indent=2)
    elif args.monitor:
        print(f"Starting continuous monitoring for {args.duration}s...")
        results = run_continuous_monitoring(args.duration)
        print(f"Monitoring complete. {len(results)} cycles recorded.")
        
        if args.output:
            with open(args.output, 'w') as f:
                json.dump(results, f, indent=2)
    else:
        result = run_ricemotions_cycle("organism_1", "exogenous_input")
        print("Single Cycle Result:")
        print(json.dumps({
            "emotional_state": result["emotional_state"],
            "cycle_time_ms": result["cycle_time_ms"],
            "landauer_total": result["landauer"]["total"],
            "threshold_alert": result["landauer"]["threshold_check"],
            "teoa_concept": result["teoa"]["concept_id"]
        }, indent=2))