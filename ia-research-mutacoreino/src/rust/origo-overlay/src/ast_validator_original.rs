// ast_validator.rs - MutaCore Alpha-Structural AST Validator
// Crate: origo-overlay v0.3.0

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    Variable(String),
    Abstraction(String, Box<AstNode>),
    Application(Box<AstNode>, Box<AstNode>),
    Constant(u64),
    KrajicekGenerator {
        proof_len: usize,
        threshold: usize,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    ProofThresholdExceeded { actual: usize, max: usize },
    AlphaIsomorphismFailure,
    GluingFailure,
}

pub struct AlphaContext {
    rename_map: HashMap<String, String>,
    counter: usize,
}

impl AlphaContext {
    pub fn new() -> Self {
        Self {
            rename_map: HashMap::new(),
            counter: 0,
        }
    }

    pub fn canonicalize(&mut self, node: &AstNode) -> AstNode {
        match node {
            AstNode::Variable(name) => {
                let anon_name = self.rename_map.get(name).cloned().unwrap_or_else(|| name.clone());
                AstNode::Variable(anon_name)
            }
            AstNode::Abstraction(param, body) => {
                self.counter += 1;
                let new_param = format!("α_{}", self.counter);
                let old_param = self.rename_map.insert(param.clone(), new_param.clone());
                
                let canonical_body = self.canonicalize(body);
                
                if let Some(old) = old_param {
                    self.rename_map.insert(param.clone(), old);
                } else {
                    self.rename_map.remove(param);
                }
                
                AstNode::Abstraction(new_param, Box::new(canonical_body))
            }
            AstNode::Application(fun, arg) => {
                AstNode::Application(
                    Box::new(self.canonicalize(fun)),
                    Box::new(self.canonicalize(arg)),
                )
            }
            AstNode::Constant(val) => AstNode::Constant(*val),
            AstNode::KrajicekGenerator { proof_len, threshold } => {
                AstNode::KrajicekGenerator { proof_len: *proof_len, threshold: *threshold }
            }
        }
    }
}

pub struct OrigoAstValidator {
    max_krajicek_threshold: usize,
}

impl OrigoAstValidator {
    pub fn new(max_threshold: usize) -> Self {
        Self { max_krajicek_threshold: max_threshold }
    }

    pub fn validate_node(&self, node: &AstNode) -> Result<(), ValidationError> {
        match node {
            AstNode::KrajicekGenerator { proof_len, threshold } => {
                if *proof_len > *threshold || *threshold > self.max_krajicek_threshold {
                    return Err(ValidationError::ProofThresholdExceeded {
                        actual: *proof_len,
                        max: self.max_krajicek_threshold,
                    });
                }
                Ok(())
            }
            AstNode::Abstraction(_, body) => self.validate_node(body),
            AstNode::Application(fun, arg) => {
                self.validate_node(fun)?;
                self.validate_node(arg)
            }
            _ => Ok(()),
        }
    }
}
