# Diretrizes Python — MutaCore

> **Objetivo:** Código científico reproduzível, tipado, performático

---

## Princípios Fundamentais

1. **Type Hints Obrigatórios** — Toda função pública tem assinatura tipada
2. **Reproducibilidade** — Seeds fixos, ambientes isolados, versões pinadas
3. **Vectorização NumPy** — Loops Python evitados em hot paths
4. **Dataclasses/Attrs** — Estruturas de dados imutáveis quando possível
5. **Validação Pydantic** — Configs e inputs validados na fronteira

---

## Setup do Ambiente

```bash
# pyproject.toml (preferido sobre requirements.txt)
[build-system]
requires = ["setuptools>=68", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "mutacore-simulations"
version = "0.3.0"
requires-python = ">=3.11"
dependencies = [
    "numpy>=1.26.0",
    "scipy>=1.11.0",
    "pydantic>=2.5.0",
    # ...
]

[tool.pytest.ini_options]
asyncio_mode = "auto"
testpaths = ["tests"]
python_files = ["test_*.py"]
python_functions = ["test_*"]
addopts = "-v --strict-markers --cov=src --cov-report=term-missing"

[tool.mypy]
python_version = "3.11"
strict = true
warn_return_any = true
warn_unused_configs = true

[tool.ruff]
line-length = 100
target-version = "py311"
select = ["E", "F", "I", "N", "W", "UP", "B", "C4", "SIM", "T20"]
ignore = ["E501"]  # line too long (managed by formatter)

[tool.black]
line-length = 100
target-version = ["py311"]
```

```bash
# Instalação
pip install -e ".[dev]"  # ou pip install -r requirements.txt
pre-commit install
```

---

## Estilo de Código

### Type Hints

```python
# ✓ SEMPRE type hints em funções públicas
from typing import List, Optional, Dict, Tuple
from dataclasses import dataclass
import numpy as np
from numpy.typing import NDArray

@dataclass(frozen=True, slots=True)
class ThermodynamicState:
    """Estado termodinâmico imutável."""
    delta_t_components: List[float]
    thresholds: KrajicekThresholds
    temperature: float = 300.0
    
    # Propriedades computadas
    @property
    def delta_t_magnitude(self) -> float:
        return sum(c**2 for c in self.delta_t_components)

def compute_landauer_residue(
    state: ThermodynamicState,
    *,
    boltzmann_const: float = 1.380649e-23
) -> float:
    """Computa resíduo de Landauer ℛₗ.
    
    Args:
        state: Estado termodinâmico atual
        boltzmann_const: k_B (J/K), padrão valor CODATA 2018
        
    Returns:
        Resíduo em Joules
    """
    delta_t_sq = state.delta_t_magnitude
    modulation = np.exp((state.thresholds.rho_t - state.thresholds.b_t) / 
                        (state.thresholds.c_t * state.thresholds.r_t))
    return delta_t_sq * modulation * boltzmann_const * state.temperature * np.log(2)
```

### Docstrings (Formato Google)

```python
def simulate_trajectory(
    self,
    steps: int,
    delta_t_generator: Optional[Callable[[int, int], List[float]]] = None,
    threshold_schedule: Optional[Callable[[int], KrajicekThresholds]] = None,
) -> List[ThermodynamicState]:
    """Simula trajetória temporal do metabolismo MutaCore.
    
    Args:
        steps: Número de passos de simulação.
        delta_t_generator: Função(step, n_components) -> List[float].
            Padrão: random walk com oscilação e drift.
        threshold_schedule: Função(step) -> KrajicekThresholds.
            Padrão: limiares constantes.
            
    Returns:
        Lista de estados termodinâmicos (histórico completo).
        
    Raises:
        ValueError: Se steps <= 0.
        
    Example:
        >>> thermo = MutaCoreThermodynamics()
        >>> trajectory = thermo.simulate_trajectory(100)
        >>> len(trajectory)
        100
    """
```

### Imports

```python
# Stdlib first
import logging
from dataclasses import dataclass
from typing import ...

# Third-party
import numpy as np
import scipy.sparse as sp
from pydantic import BaseModel, Field

# Local (relative imports dentro do package)
from .thresholds import KrajicekThresholds
from ..utils.math import safe_exp
```

---

## Computação Científica

### NumPy Best Practices

```python
# ✓ VECTORIZADO: Use broadcasting
def compute_delta_t_magnitude(components: NDArray[np.float64]) -> float:
    return float(np.sum(components**2))

# ✗ EVITE: Loops Python em arrays
def bad_magnitude(components: List[float]) -> float:
    total = 0.0
    for c in components:
        total += c * c
    return total

# ✓ PRE-ALOCAR arrays grandes
def simulate_batch(n_steps: int, n_components: int) -> NDArray[np.float64]:
    results = np.empty((n_steps, n_components), dtype=np.float64)
    for i in range(n_steps):
        results[i] = generate_delta_t(i)
    return results
```

### SciPy Sparse (Hodge Matrices)

```python
from scipy.sparse import csr_matrix
from scipy.sparse.linalg import eigsh

# Use matrizes esparsas para hipercubos grandes
def hodge_laplacian_sparse(n: int, k: int) -> csr_matrix:
    """Laplaciano esparso para Q_n grau k."""
    # ... construção eficiente ...
    return laplacian.tocsr()

# ARPACK para autovalores (k menor)
def compute_smallest_eigenvalues(laplacian: csr_matrix, k: int = 20) -> NDArray:
    """Menores autovalores via ARPACK (which='SM')."""
    try:
        eigenvals = eigsh(laplacian, k=k, which='SM', return_eigenvectors=False)
        return np.sort(eigenvals)
    except Exception as e:
        logging.warning(f"ARPACK falhou: {e}, fallback para denso")
        return np.sort(np.linalg.eigvalsh(laplacian.toarray()))[:k]
```

### Reproducibilidade

```python
# SEMPRE seed explícito para resultados determinísticos
def create_rng(seed: int = 42) -> np.random.Generator:
    return np.random.default_rng(seed)

# Em testes: seed fixo
@pytest.fixture
def rng() -> np.random.Generator:
    return create_rng(12345)

# Em simulações: seed como parâmetro
def simulate_trajectory(..., seed: int = 42, ...) -> List[ThermodynamicState]:
    rng = create_rng(seed)
    # ...
```

---

## Validação Pydantic

```python
from pydantic import BaseModel, Field, field_validator
from typing import Annotated

class PackingConfig(BaseModel):
    """Configuração do Quantum Local Packer."""
    hypercube_dimension: Annotated[int, Field(ge=2, le=16, default=8)]
    max_annealing_iterations: Annotated[int, Field(ge=100, default=10_000)]
    initial_temperature: Annotated[float, Field(gt=0, default=100.0)]
    cooling_rate: Annotated[float, Field(gt=0, lt=1, default=0.995)]
    communication_cost_weight: Annotated[float, Field(ge=0, default=1.0)]
    load_balance_weight: Annotated[float, Field(ge=0, default=0.5)]
    memory_locality_weight: Annotated[float, Field(ge=0, default=0.3)]
    seed: Optional[int] = None
    
    @field_validator('cooling_rate')
    @classmethod
    def validate_cooling(cls, v: float) -> float:
        if not 0 < v < 1:
            raise ValueError('cooling_rate deve estar em (0, 1)')
        return v
```

---

## Testes

### pytest Fixtures

```python
# tests/conftest.py
import pytest
import numpy as np

@pytest.fixture
def rng() -> np.random.Generator:
    return np.random.default_rng(42)

@pytest.fixture
def standard_thresholds() -> KrajicekThresholds:
    return KrajicekThresholds.standard()

@pytest.fixture
def simple_thermodynamic_state(standard_thresholds) -> ThermodynamicState:
    return ThermodynamicState(
        delta_t_components=[10.0, 20.0, 15.0, 5.0],
        thresholds=standard_thresholds,
        temperature=300.0
    )
```

### Testes Parametrizados

```python
# tests/test_landauer.py
import pytest
import numpy as np

class TestLandauerCoupling:
    @pytest.mark.parametrize("delta_t,expected_state", [
        ([1, 2, 3], "HOMEOSTASIS"),
        ([50, 60, 70], "EXPLORATION"),
        ([200, 300, 400], "ANXIETY"),
    ])
    def test_metabolic_state_classification(
        self, delta_t, expected_state, standard_thresholds
    ):
        thermo = MutaCoreThermodynamics(thresholds=standard_thresholds)
        state = thermo.step(delta_t)
        assert state.metabolic_state == expected_state
    
    def test_landauer_residue_positive(self, simple_thermodynamic_state):
        residue = compute_landauer_residue(simple_thermodynamic_state)
        assert residue > 0
    
    @pytest.mark.slow
    def test_long_trajectory_convergence(self, standard_thresholds):
        thermo = MutaCoreThermodynamics(thresholds=standard_thresholds)
        trajectory = thermo.simulate_trajectory(1000)
        # Verifica convergência para equilíbrio
        last_residues = [s.landauer_residue for s in trajectory[-100:]]
        assert np.std(last_residues) < 1e-20  # Estável
```

### Property-Based (Hypothesis)

```python
# tests/test_property_based.py
from hypothesis import given, strategies as st
import numpy as np

@given(
    st.lists(st.floats(-100, 100), min_size=1, max_size=10),
    st.integers(1, 10000)
)
def test_delta_t_magnitude_nonnegative(components, _):
    magnitude = sum(c**2 for c in components)
    assert magnitude >= 0

@given(st.integers(2, 8))
def test_hodge_blocks_dimension(n):
    hodge = compute_hypercube_hodge(n, max_eigenvalues_per_block=10)
    assert hodge.dimension == n
```

---

## Performance

### Profiling

```python
# utils/profiling.py
import cProfile
import pstats
from contextlib import contextmanager

@contextmanager
def profile_context(output_file: str = "profile.stats"):
    profiler = cProfile.Profile()
    profiler.enable()
    try:
        yield
    finally:
        profiler.disable()
        profiler.dump_stats(output_file)

# Uso
with profile_context("landauer_profile.stats"):
    thermo.simulate_trajectory(10000)

# Análise
# python -m pstats profile.stats
```

### JIT com Numba (Opcional)

```python
from numba import jit

@jit(nopython=True, cache=True)
def _landauer_residue_numba(delta_t_sq: float, rho_t: float, b_t: float, 
                            c_t: float, r_t: float, temp: float) -> float:
    modulation = np.exp((rho_t - b_t) / (c_t * r_t))
    return delta_t_sq * modulation * 1.380649e-23 * temp * 0.69314718056
```

---

## Logging

```python
import logging

# Configuração centralizada
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)

logger = logging.getLogger(__name__)

# Uso
logger.info("Iniciando simulação: %d passos", steps)
logger.debug("DeltaT components: %s", components)
logger.warning("Threshold próximo do limite: %.1f%%", ratio * 100)
logger.error("Falha na validação: %s", error)
```

---

## Empacotamento e Distribuição

```bash
# Build
python -m build

# Verifica
twine check dist/*

# Publica (TestPyPI)
twine upload --repository testpypi dist/*

# Publica (PyPI)
twine upload dist/*
```

---

## Checklist de PR Python

- [ ] `ruff check src/` passa
- [ ] `black --check src/` passa
- [ ] `mypy src/` passa (strict mode)
- [ ] `pytest tests/ -v` passa com coverage ≥ 80%
- [ ] Type hints em todas as funções públicas
- [ ] Docstrings Google-style em funções/classes públicas
- [ ] Seeds fixos em testes/fixtures
- [ ] Nenhum `print()` — use `logging`
- [ ] Dataclasses `frozen=True` para imutabilidade
- [ ] `numpy.typing.NDArray` para arrays tipados
- [ ] `pytest.mark.slow` para testes > 5s