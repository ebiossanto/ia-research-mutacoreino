#!/usr/bin/env python3
"""
Testes para Ricemotions Pipeline v0.8.0
"""

import pytest
import numpy as np
from src.python.ricemotions_pipeline import (
    run_ricemotions_cycle,
    validate_pipeline,
    visualize_emotion,
    EmotionalState,
    landauer_loss,
    metabolic_to_emotional,
)


class TestLandauerLoss:
    """Testes para cálculo de perda de Landauer"""
    
    def test_positive_loss(self):
        loss = landauer_loss(1000.0, 500.0, 300.0)
        assert loss > 0.0
    
    def test_zero_loss_when_output_greater(self):
        loss = landauer_loss(500.0, 1000.0, 300.0)
        assert loss == 0.0
    
    def test_zero_loss_when_equal(self):
        loss = landauer_loss(1000.0, 1000.0, 300.0)
        assert loss == 0.0
    
    def test_temperature_scaling(self):
        loss_300 = landauer_loss(1000.0, 500.0, 300.0)
        loss_600 = landauer_loss(1000.0, 500.0, 600.0)
        assert loss_600 == 2 * loss_300


class TestMetabolicToEmotional:
    """Testes para mapeamento metabólico → emocional"""
    
    def test_anxiety_high_landauer(self):
        state = metabolic_to_emotional(1e-17, 50.0)
        assert state == EmotionalState.ANXIETY
    
    def test_anxiety_high_delta_t(self):
        state = metabolic_to_emotional(1e-19, 20000.0)
        assert state == EmotionalState.ANXIETY
    
    def test_exploration(self):
        state = metabolic_to_emotional(1e-19, 200.0)
        assert state == EmotionalState.EXPLORATION
    
    def test_curiosity(self):
        state = metabolic_to_emotional(1e-19, 50.0)
        assert state == EmotionalState.CURIOSITY
    
    def test_satisfaction(self):
        state = metabolic_to_emotional(1e-21, 5.0)
        assert state == EmotionalState.SATISFACTION
    
    def test_homeostasis(self):
        state = metabolic_to_emotional(1e-19, 50.0)
        assert state == EmotionalState.HOMEOSTASIS


class TestVisualizeEmotion:
    """Testes para visualização emocional"""
    
    def test_anxiety_visualization(self):
        vis = visualize_emotion(EmotionalState.ANXIETY)
        assert vis["color_rgb"] == (255, 50, 50)
        assert vis["animation"] == "pulsing"
    
    def test_exploration_visualization(self):
        vis = visualize_emotion(EmotionalState.EXPLORATION)
        assert vis["color_rgb"] == (0, 100, 255)
        assert vis["animation"] == "expanding"
    
    def test_homeostasis_visualization(self):
        vis = visualize_emotion(EmotionalState.HOMEOSTASIS)
        assert vis["color_rgb"] == (0, 255, 0)
        assert vis["animation"] == "breathing"


class TestRicemotionsCycle:
    """Testes para ciclo completo do pipeline"""
    
    def test_single_cycle(self):
        result = run_ricemotions_cycle("test_organism", "test_exogenous")
        
        assert "pipeline" in result
        assert "emotional_state" in result
        assert "cycle_time_ms" in result
        assert "landauer" in result
        
        # Verifica estrutura
        pipeline = result["pipeline"]
        assert hasattr(pipeline, 'pixel_input')
        assert hasattr(pipeline, 'ric_intermediate')
        assert hasattr(pipeline, 'teoa_output')
        assert hasattr(pipeline, 'landauer_monitor')
        
        # Verifica tipos
        assert pipeline.pixel_input.tensor.shape == (64, 64)
        assert pipeline.teoa_output.semantic_vector.shape == (512,)
        assert pipeline.cycle_time_ms >= 0
    
    def test_deterministic_output(self):
        """Mesmo input deve gerar mesmo output"""
        result1 = run_ricemotions_cycle("organism_1", "exogenous_1")
        result2 = run_ricemotions_cycle("organism_1", "exogenous_1")
        
        # Pipeline deve ser determinístico
        assert result1["pipeline"].pixel_input.tensor.tolist() == result2["pipeline"].pixel_input.tensor.tolist()
        assert result1["pipeline"].teoa_output.semantic_vector.tolist() == result2["pipeline"].teoa_output.semantic_vector.tolist()
    
    def test_different_inputs_different_outputs(self):
        """Inputs diferentes devem gerar outputs diferentes"""
        result1 = run_ricemotions_cycle("org_1", "exo_1")
        result2 = run_ricemotions_cycle("org_2", "exo_2")
        
        assert result1["pipeline"].pixel_input.tensor.tolist() != result2["pipeline"].pixel_input.tensor.tolist()
        assert result1["pipeline"].teoa_output.concept_id != result2["pipeline"].teoa_output.concept_id


class TestValidatePipeline:
    """Testes para validação completa"""
    
    def test_validation_small(self):
        result = validate_pipeline(10)
        
        assert result["trials"] == 10
        assert "cycle_time_mean" in result
        assert "cycle_time_max" in result
        assert "landauer_loss_mean" in result
        assert "state_distribution" in result
        assert "target_met" in result
        assert "performance_target_met" in result
        
        # Verifica distribuição de estados
        for state in ["HOMEOSTASIS", "EXPLORATION", "ANXIETY", "CURIOSITY", "SATISFACTION"]:
            assert state in result["state_distribution"]
    
    def test_performance_targets(self):
        result = validate_pipeline(20)
        
        # Targets v0.8.0
        assert result["cycle_time_mean"] < 100  # < 100ms mean
        assert result["cycle_time_max"] < 100   # < 100ms max
        # Note: performance_target_met requires mean < 50ms


class TestLandauerMonitor:
    """Testes para monitor de Landauer"""
    
    def test_threshold_check(self):
        from src.python.ricemotions_pipeline import LandauerMonitor
        
        monitor = LandauerMonitor(
            pixel_to_ric_loss=1e-19,
            ric_to_teoa_loss=1e-19,
            total_loss=2e-19,
            threshold=1e-18
        )
        assert not monitor.check_threshold()
        
        monitor_high = LandauerMonitor(
            pixel_to_ric_loss=1e-17,
            ric_to_teoa_loss=1e-17,
            total_loss=2e-17,
            threshold=1e-18
        )
        assert monitor_high.check_threshold()


class TestRicCode:
    """Testes para RicCode"""
    
    def test_compression_ratio(self):
        result = run_ricemotions_cycle("test", "test")
        ric = result["ric"]
        
        assert 0 < ric["compression_ratio"] < 1
        assert ric["information_loss"] >= 0
        assert len(ric["semantic_hash"]) > 0


class TestTeoaObject:
    """Testes para TeoaObject"""
    
    def test_semantic_vector(self):
        result = run_ricemotions_cycle("test", "test")
        teoa = result["teoa"]
        
        assert teoa["concept_id"].startswith("concept_")
        assert 0.5 <= teoa["epistemic_confidence"] <= 1.0
        assert abs(teoa["vector_norm"] - 1.0) < 1e-5  # Normalizado


if __name__ == "__main__":
    pytest.main([__file__, "-v"])