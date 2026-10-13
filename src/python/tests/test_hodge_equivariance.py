#!/usr/bin/env python3
"""
Testes para Hodge Decomposition e Equivariância v0.5.0
"""

import pytest
import numpy as np
from src.python.hodge_matrices import (
    compute_hypercube_hodge,
    compute_equivariance_score,
    analyze_equivariance_breaking,
    fwht_transform,
    fwht_inverse,
    HypercubeHodge,
    HodgeBlock,
    EquivarianceMonitor,
)


class TestFWHT:
    """Testes para Fast Walsh-Hadamard Transform"""
    
    def test_fwht_basic(self):
        signal = np.array([1.0, 2.0, 3.0, 4.0])
        transformed = fwht_transform(signal)
        reconstructed = fwht_inverse(transformed)
        np.testing.assert_allclose(signal, reconstructed, rtol=1e-10)
    
    def test_fwht_identity(self):
        signal = np.array([1.0, 0.0, 0.0, 0.0])
        transformed = fwht_transform(signal)
        # FWHT of basis vector
        assert np.allclose(transformed, [1.0, 1.0, 1.0, 1.0])
    
    def test_fwht_scaling(self):
        signal = np.ones(8)
        transformed = fwht_transform(signal)
        # FWHT of constant vector
        expected = np.zeros(8)
        expected[0] = 8.0
        np.testing.assert_allclose(transformed, expected)


class TestHypercubeHodge:
    """Testes para decomposição de Hodge"""
    
    @pytest.mark.parametrize("n", [3, 4, 5, 6])
    def test_hodge_decomposition(self, n):
        hodge = compute_hypercube_hodge(n, max_eigenvalues_per_block=10)
        
        assert isinstance(hodge, HypercubeHodge)
        assert hodge.dimension == n
        assert len(hodge.blocks) == n + 1  # k = 0..n
        assert hodge.total_harmonic_forms >= 1  # β_0 = 1
        assert hodge.spectral_gap >= 0
        assert 0 <= hodge.equivariance_score <= 1
        assert hodge.computation_time > 0
    
    def test_betti_numbers(self):
        """Q_n é contrátil: β_0 = 1, β_k = 0 para k > 0"""
        hodge = compute_hypercube_hodge(4, max_eigenvalues_per_block=10)
        
        assert hodge.blocks[0].betti_number == 1
        for k in range(1, 5):
            assert hodge.blocks[k].betti_number == 0
    
    def test_spectral_gap_positive(self):
        hodge = compute_hypercube_hodge(5, max_eigenvalues_per_block=10)
        assert hodge.spectral_gap > 0
    
    def test_equivariance_score_range(self):
        hodge = compute_hypercube_hodge(6, max_eigenvalues_per_block=10)
        assert 0 <= hodge.equivariance_score <= 1


class TestEquivarianceScore:
    """Testes para score de equivariância"""
    
    def test_perfect_equivariance(self):
        # Cria blocos com degenerescência perfeita
        blocks = {
            0: HodgeBlock(0, 1, np.array([0.0]), betti_number=1),
            1: HodgeBlock(1, 4, np.array([2.0]*4), betti_number=0),
            2: HodgeBlock(2, 6, np.array([4.0]*6), betti_number=0),
        }
        score = compute_equivariance_score(blocks)
        assert score > 0.9  # Alta equivariância
    
    def test_broken_equivariance(self):
        # Blocos sem degenerescência
        blocks = {
            0: HodgeBlock(0, 1, np.array([0.0]), betti_number=1),
            1: HodgeBlock(1, 4, np.array([2.0, 2.1, 2.2, 2.3]), betti_number=0),
        }
        score = compute_equivariance_score(blocks)
        assert score < 0.5  # Baixa equivariância


class TestEquivarianceBreaking:
    """Testes para análise de quebra de equivariância"""
    
    def test_perturbation_analysis(self):
        result = analyze_equivariance_breaking(4, perturbation_strength=0.05, num_trials=3)
        
        assert result["dimension"] == 4
        assert "original_score" in result
        assert "perturbed_scores" in result
        assert len(result["perturbed_scores"]) == 3
        assert "mean_perturbed_score" in result
        assert "score_loss" in result
        assert "equivariance_preserved" in result
        assert result["perturbation_strength"] == 0.05
    
    def test_equivariance_preserved_small_perturbation(self):
        result = analyze_equivariance_breaking(4, perturbation_strength=0.01, num_trials=3)
        # Pequena perturbação deve preservar equivariância
        assert result["equivariance_preserved"] == True


class TestEquivarianceMonitor:
    """Testes para monitor de equivariância"""
    
    def test_monitor_alert(self):
        monitor = EquivarianceMonitor(threshold=0.9)
        
        # Score acima do threshold - sem alerta
        alerted = monitor.update(0.95)
        assert not alerted
        assert len(monitor.alerts) == 0
        
        # Score abaixo do threshold - alerta
        alerted = monitor.update(0.85)
        assert alerted
        assert len(monitor.alerts) == 1
        assert monitor.alerts[0]["score"] == 0.85
    
    def test_monitor_stats(self):
        monitor = EquivarianceMonitor()
        monitor.update(0.9)
        monitor.update(0.85)
        monitor.update(0.95)
        
        stats = monitor.get_stats()
        assert stats["history_length"] == 3
        assert stats["min_score"] == 0.85
        assert stats["mean_score"] == (0.9 + 0.85 + 0.95) / 3
        assert stats["alerts_count"] == 1


class TestHodgeBlockProperties:
    """Testes para propriedades dos blocos Hodge"""
    
    def test_block_degeneracy(self):
        hodge = compute_hypercube_hodge(4, max_eigenvalues_per_block=20)
        
        for k, block in hodge.blocks.items():
            if block.dimension > 0:
                # Verifica se degenerescência está correta
                rounded = np.round(block.eigenvalues, decimals=10)
                unique, counts = np.unique(rounded, return_counts=True)
                expected_degen = {float(v): int(c) for v, c in zip(unique, counts) if c > 1}
                assert block.degeneracy == expected_degen


class TestIntegrationWithLandauer:
    """Testes de integração com metabolismo Landauer"""
    
    def test_delta_t_fwht(self):
        from src.python.hodge_matrices import compute_delta_t_fwht
        
        organism = np.random.randn(8)
        exogenous = np.random.randn(8)
        
        delta_t = compute_delta_t_fwht(organism, exogenous)
        assert delta_t >= 0
        
        # Zero para mesmo estado
        delta_t_zero = compute_delta_t_fwht(organism, organism)
        assert delta_t_zero == 0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])