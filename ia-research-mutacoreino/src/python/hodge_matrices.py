#!/usr/bin/env python3
"""
Matrizes de Bloco de Hodge para Hipercubos de Alta Dimensão

Implementa computação de matrizes de Hodge para teste de equivariância
em dimensões n > 6, usado na validação da Camada 1 (Substrato Lógico).

A decomposição de Hodge em hipercubos Q_n permite analisar:
- Formas harmônicas (cohomologia)
- Equivariância sob ação do grupo de simetria
- Espectro do Laplaciano de Hodge
"""

import numpy as np
import scipy.sparse as sp
import scipy.sparse.linalg as spla
from dataclasses import dataclass
from typing import List, Tuple, Dict, Optional
import argparse
import json
import logging
from pathlib import Path
from itertools import combinations, product

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


@dataclass
class HodgeBlock:
    """Bloco da decomposição de Hodge"""
    degree: int  # Grau k (k-formas)
    dimension: int  # Dimensão do espaço
    eigenvalues: np.ndarray  # Autovalores do Laplaciano
    eigenvectors: Optional[np.ndarray] = None  # Autovetores (opcional, grande)
    betti_number: int = 0  # Número de Betti (dimensão harmônica)


@dataclass
class HypercubeHodge:
    """Decomposição de Hodge completa do hipercubo Q_n"""
    dimension: int  # n do hipercubo Q_n
    blocks: Dict[int, HodgeBlock]  # Por grau k = 0..n
    total_harmonic_forms: int
    spectral_gap: float  # Gap espectral (menor autovalor não-zero)
    equivariance_score: float  # Score de equivariância


def hypercube_vertices(n: int) -> List[Tuple[int, ...]]:
    """Gera vértices do hipercubo Q_n como tuplas binárias"""
    return list(product([0, 1], repeat=n))


def hypercube_edges(n: int) -> List[Tuple[int, int]]:
    """Gera arestas do hipercubo Q_n (índices de vértices)"""
    vertices = hypercube_vertices(n)
    vertex_to_idx = {v: i for i, v in enumerate(vertices)}
    edges = []

    for i, v in enumerate(vertices):
        for dim in range(n):
            # Flip bit na dimensão dim
            neighbor = list(v)
            neighbor[dim] = 1 - neighbor[dim]
            neighbor = tuple(neighbor)
            j = vertex_to_idx[neighbor]
            if i < j:  # Evita duplicatas
                edges.append((i, j))

    return edges


def hypercube_k_cells(n: int, k: int) -> List[Tuple[int, ...]]:
    """
    Gera k-células (k-faces) do hipercubo Q_n.
    Cada k-célula é identificada por um padrão com k asteriscos (*).
    """
    cells = []
    # Posições dos asteriscos (dimensões variáveis)
    for star_positions in combinations(range(n), k):
        # Valores fixos para as outras dimensões
        for fixed_values in product([0, 1], repeat=n - k):
            pattern = ['*'] * n
            fixed_idx = 0
            for i in range(n):
                if i not in star_positions:
                    pattern[i] = str(fixed_values[fixed_idx])
                    fixed_idx += 1
            cells.append(tuple(pattern))
    return cells


def cell_to_vertices(pattern: Tuple[str, ...], vertex_to_idx: Dict[Tuple[int, ...], int]) -> List[int]:
    """Converte padrão de k-célula para lista de índices de vértices"""
    star_positions = [i for i, c in enumerate(pattern) if c == '*']
    fixed = {i: int(c) for i, c in enumerate(pattern) if c != '*'}

    vertices = []
    for bits in product([0, 1], repeat=len(star_positions)):
        v = [0] * len(pattern)
        for i, val in fixed.items():
            v[i] = val
        for pos, bit in zip(star_positions, bits):
            v[pos] = bit
        vertices.append(vertex_to_idx[tuple(v)])

    return vertices


def boundary_matrix(n: int, k: int) -> sp.csr_matrix:
    """
    Matriz de fronteira ∂_k: C_k → C_{k-1}
    Linhas: (k-1)-células, Colunas: k-células
    """
    if k == 0:
        return sp.csr_matrix((len(hypercube_vertices(n)), 0))

    k_cells = hypercube_k_cells(n, k)
    k_minus_1_cells = hypercube_k_cells(n, k - 1)

    cell_to_idx_k = {cell: i for i, cell in enumerate(k_cells)}
    cell_to_idx_km1 = {cell: i for i, cell in enumerate(k_minus_1_cells)}

    vertices = hypercube_vertices(n)
    vertex_to_idx = {v: i for i, v in enumerate(vertices)}

    rows, cols, data = [], [], []

    for col_idx, cell in enumerate(k_cells):
        cell_vertices = cell_to_vertices(cell, vertex_to_idx)

        # Fronteira: soma alternada das (k-1)-faces
        for i, star_pos in enumerate([j for j, c in enumerate(cell) if c == '*']):
            # Remove a i-ésima dimensão variável
            for bit in [0, 1]:
                face = list(cell)
                face[star_pos] = str(bit)
                face = tuple(face)

                row_idx = cell_to_idx_km1[face]
                sign = 1 if (i + bit) % 2 == 0 else -1  # Sinal alternado
                rows.append(row_idx)
                cols.append(col_idx)
                data.append(sign)

    return sp.csr_matrix((data, (rows, cols)),
                         shape=(len(k_minus_1_cells), len(k_cells)))


def hodge_laplacian(n: int, k: int) -> sp.csr_matrix:
    """
    Laplaciano de Hodge Δ_k = ∂_{k+1}∂_{k+1}^T + ∂_k^T ∂_k
    Atua em k-formas (k-cocadeias)
    """
    if k == 0:
        # Δ_0 = ∂_1 ∂_1^T
        d1 = boundary_matrix(n, 1)
        return d1 @ d1.T

    if k == n:
        # Δ_n = ∂_n^T ∂_n
        dn = boundary_matrix(n, n)
        return dn.T @ dn

    # Caso geral
    dk = boundary_matrix(n, k)
    dk1 = boundary_matrix(n, k + 1)

    return dk1 @ dk1.T + dk.T @ dk


def compute_hodge_block(n: int, k: int, compute_eigenvectors: bool = False,
                       max_eigenvalues: Optional[int] = None) -> HodgeBlock:
    """Computa bloco de Hodge para grau k"""
    logger.info(f"Computando bloco Hodge k={k} para Q_{n}...")

    laplacian = hodge_laplacian(n, k)
    dim = laplacian.shape[0]

    if dim == 0:
        return HodgeBlock(degree=k, dimension=0, eigenvalues=np.array([]),
                         betti_number=0)

    # Computa autovalores
    if max_eigenvalues and max_eigenvalues < dim:
        # Usa ARPACK para maiores autovalores
        try:
            eigenvals = spla.eigsh(laplacian, k=min(max_eigenvalues, dim-1),
                                   which='SM', return_eigenvectors=compute_eigenvectors)
            if compute_eigenvectors:
                eigenvalues, eigenvectors = eigenvals
            else:
                eigenvalues = eigenvals
                eigenvectors = None
        except Exception as e:
            logger.warning(f"ARPACK falhou, usando denso: {e}")
            eigenvals_dense = np.linalg.eigvalsh(laplacian.toarray())
            eigenvalues = np.sort(eigenvals_dense)[:max_eigenvalues]
            eigenvectors = None
    else:
        # Denso para matrizes pequenas
        eigenvals_dense = np.linalg.eigvalsh(laplacian.toarray())
        eigenvalues = np.sort(eigenvals_dense)
        eigenvectors = None

    # Formas harmônicas = kernel do Laplaciano (autovalor ≈ 0)
    harmonic_dim = np.sum(eigenvalues < 1e-10)

    return HodgeBlock(
        degree=k,
        dimension=dim,
        eigenvalues=eigenvalues,
        eigenvectors=eigenvectors,
        betti_number=harmonic_dim
    )


def compute_hypercube_hodge(n: int, max_k: Optional[int] = None,
                           compute_eigenvectors: bool = False,
                           max_eigenvalues_per_block: int = 50) -> HypercubeHodge:
    """
    Computa decomposição de Hodge completa do hipercubo Q_n.

    Para n > 6, usa aproximações esparsas.
    """
    if max_k is None:
        max_k = n

    logger.info(f"Iniciando decomposição de Hodge para Q_{n} (k=0..{max_k})")

    blocks = {}
    total_harmonic = 0

    for k in range(max_k + 1):
        block = compute_hodge_block(n, k, compute_eigenvectors, max_eigenvalues_per_block)
        blocks[k] = block
        total_harmonic += block.betti_number
        logger.info(f"  k={k}: dim={block.dimension}, β_k={block.betti_number}, "
                   f"λ_min={block.eigenvalues[0] if len(block.eigenvalues) > 0 else 'N/A':.2e}")

    # Gap espectral global
    all_nonzero_evals = []
    for block in blocks.values():
        for ev in block.eigenvalues:
            if ev > 1e-10:
                all_nonzero_evals.append(ev)

    spectral_gap = min(all_nonzero_evals) if all_nonzero_evals else 0.0

    # Score de equivariância: baseado na multiplicidade de autovalores
    # Alto score = muitos autovalores degenerados = alta simetria
    equivariance_score = compute_equivariance_score(blocks)

    return HypercubeHodge(
        dimension=n,
        blocks=blocks,
        total_harmonic_forms=total_harmonic,
        spectral_gap=spectral_gap,
        equivariance_score=equivariance_score
    )


def compute_equivariance_score(blocks: Dict[int, HodgeBlock]) -> float:
    """
    Computa score de equivariância baseado em degenerescência espectral.
    Em Q_n, a ação do grupo de simetria (hiperoctaédrico) cria alta degenerescência.
    """
    total_degeneracy = 0
    total_eigenvals = 0

    for block in blocks.values():
        if len(block.eigenvalues) == 0:
            continue

        # Conta multiplicidades
        unique_vals, counts = np.unique(np.round(block.eigenvalues, decimals=10), return_counts=True)
        for count in counts:
            if count > 1:
                total_degeneracy += count - 1
        total_eigenvals += len(block.eigenvalues)

    if total_eigenvals == 0:
        return 0.0

    # Normaliza: 1.0 = totalmente equivariante (máxima degenerescência teórica)
    # Para Q_n, degenerescência esperada cresce com n
    max_theoretical = sum(sp.binomial(blocks[k].dimension, 2) for k in blocks if blocks[k].dimension > 1)
    return min(total_degeneracy / max(1, max_theoretical), 1.0)


def analyze_equivariance_breaking(n: int, perturbation_strength: float = 0.1) -> Dict:
    """
    Analisa quebra de equivariância sob perturbação.
    Simula ruído/imperfeições no hipercubo.
    """
    logger.info(f"Analisando quebra de equivariância em Q_{n}...")

    # Hodge original
    hodge_orig = compute_hypercube_hodge(n)

    # Laplaciano perturbado
    perturbed_blocks = {}
    for k, block in hodge_orig.blocks.items():
        if block.dimension == 0:
            perturbed_blocks[k] = block
            continue

        laplacian = hodge_laplacian(n, k)
        # Adiciona ruído aleatório simétrico
        noise = sp.random(laplacian.shape[0], laplacian.shape[1],
                         density=0.01, format='csr', dtype=float)
        noise = (noise + noise.T) / 2 * perturbation_strength
        perturbed_laplacian = laplacian + noise

        # Recomputa autovalores
        try:
            eigenvals = spla.eigsh(perturbed_laplacian, k=min(20, laplacian.shape[0]-1),
                                   which='SM', return_eigenvectors=False)
            eigenvals = np.sort(eigenvals)
        except:
            eigenvals = np.linalg.eigvalsh(perturbed_laplacian.toarray())
            eigenvals = np.sort(eigenvals)

        harmonic_dim = np.sum(eigenvals < 1e-8)
        perturbed_blocks[k] = HodgeBlock(
            degree=k,
            dimension=block.dimension,
            eigenvalues=eigenvals,
            betti_number=harmonic_dim
        )

    hodge_perturbed = HypercubeHodge(
        dimension=n,
        blocks=perturbed_blocks,
        total_harmonic_forms=sum(b.betti_number for b in perturbed_blocks.values()),
        spectral_gap=min([ev for b in perturbed_blocks.values()
                         for ev in b.eigenvalues if ev > 1e-8], default=0),
        equivariance_score=compute_equivariance_score(perturbed_blocks)
    )

    return {
        "original": hodge_orig,
        "perturbed": hodge_perturbed,
        "equivariance_loss": hodge_orig.equivariance_score - hodge_perturbed.equivariance_score,
        "spectral_gap_change": hodge_perturbed.spectral_gap - hodge_orig.spectral_gap,
        "betti_changes": {
            k: hodge_perturbed.blocks[k].betti_number - hodge_orig.blocks[k].betti_number
            for k in hodge_orig.blocks
        }
    }


def export_hodge_results(hodge: HypercubeHodge, filepath: str):
    """Exporta resultados para JSON"""
    data = {
        "dimension": hodge.dimension,
        "total_harmonic_forms": hodge.total_harmonic_forms,
        "spectral_gap": hodge.spectral_gap,
        "equivariance_score": hodge.equivariance_score,
        "blocks": {}
    }

    for k, block in hodge.blocks.items():
        data["blocks"][k] = {
            "degree": block.degree,
            "dimension": block.dimension,
            "betti_number": block.betti_number,
            "eigenvalues": block.eigenvalues.tolist() if len(block.eigenvalues) > 0 else [],
            "min_eigenvalue": float(block.eigenvalues[0]) if len(block.eigenvalues) > 0 else 0,
            "max_eigenvalue": float(block.eigenvalues[-1]) if len(block.eigenvalues) > 0 else 0,
        }

    with open(filepath, 'w') as f:
        json.dump(data, f, indent=2)
    logger.info(f"Resultados exportados para {filepath}")


def run_demo():
    """Executa demonstração para várias dimensões"""
    print("=" * 60)
    print("Hodge Decomposition on Hypercubes Q_n")
    print("Testing equivariance for n > 6")
    print("=" * 60)

    results = {}

    for n in range(3, 9):  # 3 a 8
        print(f"\n--- Q_{n} ---")
        hodge = compute_hypercube_hodge(n, max_eigenvalues_per_block=30)

        print(f"  Dimensão total: {sum(b.dimension for b in hodge.blocks.values())}")
        print(f"  Formas harmônicas totais: {hodge.total_harmonic_forms}")
        print(f"  Gap espectral: {hodge.spectral_gap:.4e}")
        print(f"  Score equivariância: {hodge.equivariance_score:.4f}")

        # Números de Betti
        betti_str = ", ".join(f"β_{k}={b.betti_number}" for k, b in hodge.blocks.items())
        print(f"  Betti: {betti_str}")

        export_hodge_results(hodge, f"hodge_Q{n}.json")
        results[n] = {
            "betti_numbers": {k: b.betti_number for k, b in hodge.blocks.items()},
            "spectral_gap": hodge.spectral_gap,
            "equivariance_score": hodge.equivariance_score
        }

    # Análise de quebra para n=6
    print(f"\n--- Análise de Quebra de Equivariância (Q_6) ---")
    breaking = analyze_equivariance_breaking(6, perturbation_strength=0.05)
    print(f"  Perda de equivariância: {breaking['equivariance_loss']:.4f}")
    print(f"  Mudança gap espectral: {breaking['spectral_gap_change']:.4e}")
    print(f"  Mudanças Betti: {breaking['betti_changes']}")

    with open("hodge_summary.json", 'w') as f:
        json.dump(results, f, indent=2)

    return results


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Hodge Decomposition on Hypercubes")
    parser.add_argument("--dimension", "-n", type=int, default=6, help="Dimensão do hipercubo")
    parser.add_argument("--max-k", type=int, help="Grau máximo (default: n)")
    parser.add_argument("--max-eigs", type=int, default=30, help="Max autovalores por bloco")
    parser.add_argument("--eigenvectors", action="store_true", help="Computar autovetores")
    parser.add_argument("--output", type=str, help="Arquivo de saída JSON")
    parser.add_argument("--analyze-breaking", action="store_true", help="Analisar quebra de equivariância")
    parser.add_argument("--perturbation", type=float, default=0.1, help="Força da perturbação")
    parser.add_argument("--demo", action="store_true", help="Executa demo completo")

    args = parser.parse_args()

    if args.demo:
        run_demo()
    elif args.analyze_breaking:
        result = analyze_equivariance_breaking(args.dimension, args.perturbation)
        print(f"Equivariance loss: {result['equivariance_loss']:.4f}")
        print(f"Spectral gap change: {result['spectral_gap_change']:.4e}")
        if args.output:
            with open(args.output, 'w') as f:
                # Serialização customizada para objetos numpy
                import json
                class NumpyEncoder(json.JSONEncoder):
                    def default(self, obj):
                        if isinstance(obj, np.ndarray):
                            return obj.tolist()
                        if isinstance(obj, np.floating):
                            return float(obj)
                        if isinstance(obj, np.integer):
                            return int(obj)
                        return super().default(obj)
                json.dump({
                    "equivariance_loss": result['equivariance_loss'],
                    "spectral_gap_change": result['spectral_gap_change'],
                    "betti_changes": result['betti_changes']
                }, f, cls=NumpyEncoder, indent=2)
    else:
        hodge = compute_hypercube_hodge(
            args.dimension,
            max_k=args.max_k,
            compute_eigenvectors=args.eigenvectors,
            max_eigenvalues_per_block=args.max_eigs
        )

        print(f"Q_{args.dimension} Hodge Decomposition:")
        print(f"  Blocks: {len(hodge.blocks)}")
        print(f"  Total harmonic forms: {hodge.total_harmonic_forms}")
        print(f"  Spectral gap: {hodge.spectral_gap:.4e}")
        print(f"  Equivariance score: {hodge.equivariance_score:.4f}")

        for k, block in hodge.blocks.items():
            print(f"  k={k}: dim={block.dimension}, β={block.betti_number}, "
                  f"λ∈[{block.eigenvalues[0]:.2e}, {block.eigenvalues[-1]:.2e}]")

        if args.output:
            export_hodge_results(hodge, args.output)