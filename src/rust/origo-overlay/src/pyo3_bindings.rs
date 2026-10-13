// PyO3 Bindings para Python - MutaCore Python Integration
// Compilação: cargo build --release --features pyo3
// Uso: python -c "import origo_overlay; ..."

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use pyo3::types::{PyDict, PyList, PyBytes};
#[cfg(feature = "pyo3")]
use serde_json;
#[cfg(feature = "pyo3")]
use crate::{
    AlphaContext, OrigoAstValidator, AstNode, KrajicekThresholds,
    StructuralHasher, QuantumLocalPacker, PackingConfig, NodeInfo, WorkItem,
    ValidationResult, ValidationConfig, ThresholdConfig,
    error::{OrigoError, Result},
};

#[cfg(feature = "pyo3")]
#[pyfunction]
fn validate_ast(ast_json: &str, rho_t: usize) -> PyResult<String> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid AST JSON: {}", e)))?;
    
    let thresholds = KrajicekThresholds::custom(rho_t, rho_t / 2, rho_t / 5, rho_t / 3);
    let mut validator = OrigoAstValidator::new(thresholds);
    let result = validator.validate(&ast)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    serde_json::to_string(&result)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn validate_ast_with_thresholds(ast_json: &str, thresholds_json: &str) -> PyResult<String> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid AST JSON: {}", e)))?;
    
    let thresholds: KrajicekThresholds = serde_json::from_str(thresholds_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid thresholds JSON: {}", e)))?;
    
    let mut validator = OrigoAstValidator::new(thresholds);
    let result = validator.validate(&ast)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    serde_json::to_string(&result)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn alpha_canonicalize(ast_json: &str) -> PyResult<String> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid AST JSON: {}", e)))?;
    
    let mut ctx = AlphaContext::new();
    let canonical = ctx.canonicalize(&ast);
    
    serde_json::to_string(&canonical)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn alpha_isomorphic(ast1_json: &str, ast2_json: &str) -> PyResult<bool> {
    let ast1: AstNode = serde_json::from_str(ast1_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid AST1 JSON: {}", e)))?;
    let ast2: AstNode = serde_json::from_str(ast2_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid AST2 JSON: {}", e)))?;
    
    let mut ctx = AlphaContext::new();
    Ok(ctx.is_alpha_isomorphic(&ast1, &ast2))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn structural_hash(ast_json: &str) -> PyResult<String> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid AST JSON: {}", e)))?;
    
    let hash = StructuralHasher::hash(&ast);
    Ok(format!("{:016x}", hash))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn compute_landauer_residue(delta_t_json: &str, thresholds_json: &str, temperature: f64) -> PyResult<f64> {
    let delta_t: Vec<f64> = serde_json::from_str(delta_t_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid delta_t JSON: {}", e)))?;
    
    let thresholds: KrajicekThresholds = serde_json::from_str(thresholds_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid thresholds JSON: {}", e)))?;
    
    let k_b = 1.380649e-23;
    let ln2 = 2.0_f64.ln();
    
    let delta_t_sq: f64 = delta_t.iter().map(|x| x * x).sum();
    let modulation = ((thresholds.rho_t as f64 - thresholds.b_t as f64) / 
                     (thresholds.c_t as f64 * thresholds.r_t as f64)).exp();
    
    Ok(delta_t_sq * modulation * k_b * temperature * ln2)
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn pack_work_items(work_items_json: &str, nodes_json: &str, config_json: Option<&str>) -> PyResult<String> {
    let work_items: Vec<WorkItem> = serde_json::from_str(work_items_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid work_items JSON: {}", e)))?;
    
    let nodes: Vec<NodeInfo> = serde_json::from_str(nodes_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid nodes JSON: {}", e)))?;
    
    let config = if let Some(cfg) = config_json {
        serde_json::from_str(cfg)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid config JSON: {}", e)))?
    } else {
        PackingConfig::default()
    };
    
    let mut packer = QuantumLocalPacker::new(config, nodes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    let assignments = packer.pack(work_items)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    serde_json::to_string(&assignments)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn default_thresholds() -> PyResult<String> {
    let thresholds = KrajicekThresholds::standard();
    serde_json::to_string(&thresholds)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn conservative_thresholds() -> PyResult<String> {
    let thresholds = KrajicekThresholds::conservative();
    serde_json::to_string(&thresholds)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn aggressive_thresholds() -> PyResult<String> {
    let thresholds = KrajicekThresholds::aggressive();
    serde_json::to_string(&thresholds)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pymodule]
fn origo_overlay(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_ast, m)?)?;
    m.add_function(wrap_pyfunction!(validate_ast_with_thresholds, m)?)?;
    m.add_function(wrap_pyfunction!(alpha_canonicalize, m)?)?;
    m.add_function(wrap_pyfunction!(alpha_isomorphic, m)?)?;
    m.add_function(wrap_pyfunction!(structural_hash, m)?)?;
    m.add_function(wrap_pyfunction!(compute_landauer_residue, m)?)?;
    m.add_function(wrap_pyfunction!(pack_work_items, m)?)?;
    m.add_function(wrap_pyfunction!(default_thresholds, m)?)?;
    m.add_function(wrap_pyfunction!(conservative_thresholds, m)?)?;
    m.add_function(wrap_pyfunction!(aggressive_thresholds, m)?)?;
    Ok(())
}