// WASM Bindings para WebAssembly - MutaCore Browser/Node.js Integration
// Compilação: cargo build --target wasm32-unknown-unknown --release --features wasm
// Uso: import { validate_ast, alpha_canonicalize, ... } from 'origo_overlay'

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "wasm")]
use crate::{
    AlphaContext, OrigoAstValidator, AstNode, KrajicekThresholds,
    StructuralHasher, QuantumLocalPacker, PackingConfig, NodeInfo, WorkItem,
    ValidationResult, ValidationConfig,
    error::Result,
};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[derive(Serialize, Deserialize)]
pub struct WasmValidationResult {
    pub valid: bool,
    pub rho_t_checked: bool,
    pub b_t_checked: bool,
    pub c_t_checked: bool,
    pub r_t_checked: bool,
    pub failures: Vec<String>,
    pub total_complexity: usize,
}

#[cfg(feature = "wasm")]
impl From<ValidationResult> for WasmValidationResult {
    fn from(result: ValidationResult) -> Self {
        Self {
            valid: result.valid,
            rho_t_checked: result.rho_t_checked,
            b_t_checked: result.b_t_checked,
            c_t_checked: result.c_t_checked,
            r_t_checked: result.r_t_checked,
            failures: result.failures,
            total_complexity: result.total_complexity,
        }
    }
}

#[cfg(feature = "wasm")]
#[derive(Serialize, Deserialize)]
pub struct WasmWorkAssignment {
    pub work_id: String,
    pub node_id: String,
    pub estimated_completion_time: f64,
    pub communication_cost: f64,
    pub confidence: f64,
}

#[cfg(feature = "wasm")]
impl From<crate::WorkAssignment> for WasmWorkAssignment {
    fn from(a: crate::WorkAssignment) -> Self {
        Self {
            work_id: a.work_id,
            node_id: a.node_id,
            estimated_completion_time: a.estimated_completion_time,
            communication_cost: a.communication_cost,
            confidence: a.confidence,
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_ast(ast_json: &str, rho_t: usize) -> Result<JsValue, JsValue> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid AST JSON: {}", e)))?;
    
    let thresholds = KrajicekThresholds::custom(rho_t, rho_t / 2, rho_t / 5, rho_t / 3);
    let mut validator = OrigoAstValidator::new(thresholds);
    let result = validator.validate(&ast)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let wasm_result: WasmValidationResult = result.into();
    serde_wasm_bindgen::to_value(&wasm_result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_ast_with_thresholds(ast_json: &str, thresholds_json: &str) -> Result<JsValue, JsValue> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid AST JSON: {}", e)))?;
    
    let thresholds: KrajicekThresholds = serde_json::from_str(thresholds_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid thresholds JSON: {}", e)))?;
    
    let mut validator = OrigoAstValidator::new(thresholds);
    let result = validator.validate(&ast)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let wasm_result: WasmValidationResult = result.into();
    serde_wasm_bindgen::to_value(&wasm_result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn alpha_canonicalize(ast_json: &str) -> Result<JsValue, JsValue> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid AST JSON: {}", e)))?;
    
    let mut ctx = AlphaContext::new();
    let canonical = ctx.canonicalize(&ast);
    
    serde_wasm_bindgen::to_value(&canonical)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn alpha_isomorphic(ast1_json: &str, ast2_json: &str) -> Result<bool, JsValue> {
    let ast1: AstNode = serde_json::from_str(ast1_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid AST1 JSON: {}", e)))?;
    let ast2: AstNode = serde_json::from_str(ast2_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid AST2 JSON: {}", e)))?;
    
    let mut ctx = AlphaContext::new();
    Ok(ctx.is_alpha_isomorphic(&ast1, &ast2))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn structural_hash(ast_json: &str) -> Result<String, JsValue> {
    let ast: AstNode = serde_json::from_str(ast_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid AST JSON: {}", e)))?;
    
    let hash = StructuralHasher::hash(&ast);
    Ok(format!("{:016x}", hash))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn compute_landauer_residue(delta_t_json: &str, thresholds_json: &str, temperature: f64) -> Result<f64, JsValue> {
    let delta_t: Vec<f64> = serde_json::from_str(delta_t_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid delta_t JSON: {}", e)))?;
    
    let thresholds: KrajicekThresholds = serde_json::from_str(thresholds_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid thresholds JSON: {}", e)))?;
    
    let k_b = 1.380649e-23;
    let ln2 = 2.0_f64.ln();
    
    let delta_t_sq: f64 = delta_t.iter().map(|x| x * x).sum();
    let modulation = ((thresholds.rho_t as f64 - thresholds.b_t as f64) / 
                     (thresholds.c_t as f64 * thresholds.r_t as f64)).exp();
    
    Ok(delta_t_sq * modulation * k_b * temperature * ln2)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn pack_work_items(work_items_json: &str, nodes_json: &str, config_json: Option<String>) -> Result<JsValue, JsValue> {
    let work_items: Vec<WorkItem> = serde_json::from_str(work_items_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid work_items JSON: {}", e)))?;
    
    let nodes: Vec<NodeInfo> = serde_json::from_str(nodes_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid nodes JSON: {}", e)))?;
    
    let config = if let Some(cfg) = config_json {
        serde_json::from_str(&cfg)
            .map_err(|e| JsValue::from_str(&format!("Invalid config JSON: {}", e)))?
    } else {
        PackingConfig::default()
    };
    
    let mut packer = QuantumLocalPacker::new(config, nodes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let assignments = packer.pack(work_items)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let wasm_assignments: Vec<WasmWorkAssignment> = assignments.into_iter().map(Into::into).collect();
    serde_wasm_bindgen::to_value(&wasm_assignments)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn default_thresholds() -> Result<JsValue, JsValue> {
    let thresholds = KrajicekThresholds::standard();
    serde_wasm_bindgen::to_value(&thresholds)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn conservative_thresholds() -> Result<JsValue, JsValue> {
    let thresholds = KrajicekThresholds::conservative();
    serde_wasm_bindgen::to_value(&thresholds)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn aggressive_thresholds() -> Result<JsValue, JsValue> {
    let thresholds = KrajicekThresholds::aggressive();
    serde_wasm_bindgen::to_value(&thresholds)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    log("MutaCore origo-overlay WASM initialized");
}