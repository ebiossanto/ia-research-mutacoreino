#!/usr/bin/env python3
"""
Simulação Termodinâmica do MutaCore - Acoplamento ΔT ↔ ℛₗ

Implementa a equação fundamental do metabolismo termodinâmico:
ℛₗ = (Σ ΔTᵢ²) · exp((ρ_T - B_T) / (C_T · R_T)) · k_B T ln 2

Onde:
- ΔTᵢ: Componentes ortogonais do Gap Topológico Narrativo
- ρ_T: Limiar de colagem de prova atual
- B_T, C_T, R_T: Constantes do espectro de Krajicek
- k_B: Constante de Boltzmann (1.380649×10⁻²³ J/K)
- T: Temperatura operacional do nó (Kelvin)
"""

import numpy as np
import json
import argparse
from dataclasses import dataclass, asdict
from typing import List, Optional, Tuple
from pathlib import Path
import logging

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# Constantes físicas
K_B = 1.380649e-23  # J/K - Constante de Boltzmann
LN2 = np.log(2)     # ln(2)


@dataclass
class KrajicekThresholds:
    """Espectro de limiares de Krajicek"""
    rho_t: float = 10000.0  # ρ_T - Limiar de colagem
    b_t: float = 5000.0     # B_T - Complexidade base
    c_t: float = 2000.0     # C_T - Contração
    r_t: float = 3000.0     # R_T - Reflexão

    def to_dict(self) -> dict:
        return asdict(self)

    @classmethod
    def from_dict(cls, data: dict) -> 'KrajicekThresholds':
        return cls(**data)


@dataclass
class ThermodynamicState:
    """Estado termodinâmico do sistema"""
    delta_t_components: List[float]  # Componentes ΔTᵢ
    thresholds: KrajicekThresholds
    temperature: float = 300.0  # Kelvin

    # Resultados computados
    delta_t_magnitude: float = 0.0
    landauer_residue: float = 0.0  # ℛₗ em Joules
    metabolic_state: str = "UNKNOWN"  # HOMEOSTASIS, ANXIETY, EXPLORATION
    energy_cost_per_bit: float = 0.0


class MutaCoreThermodynamics:
    """
    Motor termodinâmico do MutaCore.

    Modela o metabolismo computacional onde:
    - Gap Topológico Narrativo (ΔT) representa dissonância cognitiva
    - Resíduo de Landauer (ℛₗ) representa custo energético da perda de informação
    - Limiares de Krajicek modulam a sensibilidade termodinâmica
    """

    def __init__(self, thresholds: Optional[KrajicekThresholds] = None,
                 temperature: float = 300.0):
        self.thresholds = thresholds or KrajicekThresholds()
        self.temperature = temperature
        self.history: List[ThermodynamicState] = []

    def compute_delta_t_magnitude(self, components: List[float]) -> float:
        """Calcula magnitude do Gap Topológico: ||ΔT||² = Σ ΔTᵢ²"""
        return sum(c**2 for c in components)

    def compute_krajicek_modulation(self, thresholds: KrajicekThresholds) -> float:
        """
        Computa fator de modulação exponencial:
        exp((ρ_T - B_T) / (C_T · R_T))

        Este termo captura como a arquitetura lógica (Krajicek)
        modula a resposta termodinâmica.
        """
        numerator = thresholds.rho_t - thresholds.b_t
        denominator = thresholds.c_t * thresholds.r_t

        if denominator == 0:
            logger.warning("Denominador zero na modulação de Krajicek, usando 1.0")
            return 1.0

        return np.exp(numerator / denominator)

    def compute_landauer_residue(self, state: ThermodynamicState) -> float:
        """
        Equação fundamental: ℛₗ = (Σ ΔTᵢ²) · exp((ρ_T - B_T)/(C_T·R_T)) · k_B T ln 2
        """
        delta_t_sq = state.delta_t_magnitude
        modulation = self.compute_krajicek_modulation(state.thresholds)

        # Energia de Landauer por bit: k_B T ln 2
        landauer_per_bit = K_B * state.temperature * LN2

        # Resíduo total
        residue = delta_t_sq * modulation * landauer_per_bit

        return residue

    def classify_metabolic_state(self, residue: float, delta_t_mag: float) -> str:
        """
        Classifica estado metabólico baseado em resíduo e ΔT:

        - HOMEOSTASIS: Baixo resíduo, baixo ΔT (equilíbrio)
        - ANXIETY: Alto resíduo ou alto ΔT (estresse termodinâmico)
        - EXPLORATION: Baixo resíduo, ΔT moderado (busca ativa)
        """
        # Limiares heurísticos (calibrar empiricamente)
        residue_threshold = 1e-18  # ~1 attojoule
        delta_t_threshold = 100.0

        if residue > residue_threshold or delta_t_mag > delta_t_threshold:
            return "ANXIETY"
        elif delta_t_mag > 10.0:  # ΔT moderado
            return "EXPLORATION"
        else:
            return "HOMEOSTASIS"

    def step(self, delta_t_components: List[float],
             thresholds: Optional[KrajicekThresholds] = None) -> ThermodynamicState:
        """
        Executa um passo da simulação termodinâmica.
        """
        th = thresholds or self.thresholds

        state = ThermodynamicState(
            delta_t_components=delta_t_components,
            thresholds=th,
            temperature=self.temperature
        )

        # Computa magnitude ΔT
        state.delta_t_magnitude = self.compute_delta_t_magnitude(delta_t_components)

        # Computa resíduo de Landauer
        state.landauer_residue = self.compute_landauer_residue(state)

        # Energia por bit
        state.energy_cost_per_bit = K_B * self.temperature * LN2

        # Classifica estado metabólico
        state.metabolic_state = self.classify_metabolic_state(
            state.landauer_residue, state.delta_t_magnitude
        )

        self.history.append(state)
        return state

    def simulate_trajectory(self, steps: int,
                           delta_t_generator=None,
                           threshold_schedule=None) -> List[ThermodynamicState]:
        """
        Simula trajetória temporal do metabolismo.
        """
        trajectory = []

        for i in range(steps):
            # Gera ΔT (padrão: random walk com drift)
            if delta_t_generator:
                components = delta_t_generator(i, len(self.thresholds.__dict__))
            else:
                # Default: 4 componentes (ρ_T, B_T, C_T, R_T)
                n_components = 4
                components = np.random.normal(0, 10, n_components).tolist()

            # Atualiza limiares se houver schedule
            if threshold_schedule:
                th = threshold_schedule(i)
            else:
                th = None

            state = self.step(components, th)
            trajectory.append(state)

            if i % 100 == 0:
                logger.info(f"Step {i}: ΔT²={state.delta_t_magnitude:.2e}, "
                          f"ℛₗ={state.landauer_residue:.2e} J, "
                          f"State={state.metabolic_state}")

        return trajectory

    def export_results(self, filepath: str):
        """Exporta histórico para JSON"""
        data = {
            "thresholds": self.thresholds.to_dict(),
            "temperature": self.temperature,
            "constants": {
                "k_B": K_B,
                "ln2": LN2
            },
            "history": [asdict(s) for s in self.history]
        }
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
        logger.info(f"Resultados exportados para {filepath}")


def default_delta_t_generator(step: int, n_components: int) -> List[float]:
    """Gerador padrão de ΔT com dinâmica interessante"""
    # Combina: oscilação + ruído + drift lento
    t = step * 0.1
    base = 50 * np.sin(t) + 20 * np.cos(0.3 * t)
    noise = np.random.normal(0, 5, n_components)
    drift = 0.1 * step

    return [base + n + drift for n in noise]


def threshold_schedule_example(step: int) -> KrajicekThresholds:
    """Exemplo de schedule adaptativo de limiares"""
    # Limiares que se adaptam baseado no step
    factor = 1.0 + 0.001 * np.sin(step * 0.05)
    return KrajicekThresholds(
        rho_t=10000 * factor,
        b_t=5000 * factor,
        c_t=2000 * factor,
        r_t=3000 * factor
    )


def run_demo():
    """Executa demonstração completa"""
    print("=" * 60)
    print("MutaCore - Simulação Termodinâmica")
    print("Acoplamento ΔT ↔ ℛₗ (Landauer)")
    print("=" * 60)

    # Configuração
    thermo = MutaCoreThermodynamics(temperature=300.0)

    # Simulação
    print("\nExecutando simulação (500 passos)...")
    trajectory = thermo.simulate_trajectory(
        steps=500,
        delta_t_generator=default_delta_t_generator,
        threshold_schedule=threshold_schedule_example
    )

    # Estatísticas
    states = [s.metabolic_state for s in trajectory]
    residues = [s.landauer_residue for s in trajectory]
    delta_ts = [s.delta_t_magnitude for s in trajectory]

    print(f"\n--- Estatísticas ---")
    print(f"Passos: {len(trajectory)}")
    print(f"HOMEOSTASIS: {states.count('HOMEOSTASIS')}")
    print(f"EXPLORATION: {states.count('EXPLORATION')}")
    print(f"ANXIETY: {states.count('ANXIETY')}")
    print(f"ℛₗ médio: {np.mean(residues):.2e} J")
    print(f"ℛₗ máximo: {np.max(residues):.2e} J")
    print(f"ΔT² médio: {np.mean(delta_ts):.2f}")

    # Exporta
    thermo.export_results("thermodynamics_results.json")

    # Análise de correlação
    print(f"\n--- Correlação ΔT² ↔ ℛₗ ---")
    corr = np.corrcoef(delta_ts, residues)[0, 1]
    print(f"Correlação de Pearson: {corr:.4f}")

    return thermo


def run_single_calculation(delta_t: List[float], rho_t: float = 10000,
                          b_t: float = 5000, c_t: float = 2000, r_t: float = 3000,
                          temperature: float = 300.0) -> ThermodynamicState:
    """Calcula estado termodinâmico para parâmetros específicos"""
    thresholds = KrajicekThresholds(rho_t=rho_t, b_t=b_t, c_t=c_t, r_t=r_t)
    thermo = MutaCoreThermodynamics(thresholds=thresholds, temperature=temperature)
    return thermo.step(delta_t)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="MutaCore Thermodynamics Simulation")
    parser.add_argument("--steps", type=int, default=500, help="Número de passos")
    parser.add_argument("--temp", type=float, default=300.0, help="Temperatura (K)")
    parser.add_argument("--rho-t", type=float, default=10000, help="ρ_T")
    parser.add_argument("--b-t", type=float, default=5000, help="B_T")
    parser.add_argument("--c-t", type=float, default=2000, help="C_T")
    parser.add_argument("--r-t", type=float, default=3000, help="R_T")
    parser.add_argument("--output", type=str, default="thermodynamics_results.json", help="Arquivo de saída")
    parser.add_argument("--single", action="store_true", help="Cálculo único ao invés de simulação")
    parser.add_argument("--delta-t", type=float, nargs="+", default=[10, 20, 15, 5], help="Componentes ΔT")

    args = parser.parse_args()

    if args.single:
        state = run_single_calculation(
            delta_t=args.delta_t,
            rho_t=args.rho_t,
            b_t=args.b_t,
            c_t=args.c_t,
            r_t=args.r_t,
            temperature=args.temp
        )
        print(f"ΔT² = {state.delta_t_magnitude:.4f}")
        print(f"ℛₗ = {state.landauer_residue:.4e} J")
        print(f"Estado Metabólico: {state.metabolic_state}")
        print(f"Energia/bit: {state.energy_cost_per_bit:.4e} J")
    else:
        thermo = MutaCoreThermodynamics(
            thresholds=KrajicekThresholds(
                rho_t=args.rho_t, b_t=args.b_t, c_t=args.c_t, r_t=args.r_t
            ),
            temperature=args.temp
        )
        thermo.simulate_trajectory(args.steps)
        thermo.export_results(args.output)