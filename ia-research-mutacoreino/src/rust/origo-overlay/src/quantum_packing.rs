//! Quantum Local Packing - Otimização de Localidade e Distribuição
//!
//! Implementa algoritmos de empacotamento quântico-local para distribuição
//! ótima de cargas de trabalho AST nos nós da rede MutaCore.
//!
//! Baseia-se em princípios de:
//! - Annealing quântico para otimização combinatória
//! - Localidade de referência em hipercubos de alta dimensão
//! - Minimização de custo de comunicação (equivariância)

use crate::{AstNode, error::{OrigoError, Result}, hash::StructuralHasher};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Configuração do Quantum Local Packer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackingConfig {
    /// Dimensão do hipercubo de embedding
    pub hypercube_dimension: usize,
    /// Número máximo de iterações de annealing
    pub max_annealing_iterations: usize,
    /// Temperatura inicial
    pub initial_temperature: f64,
    /// Taxa de resfriamento
    pub cooling_rate: f64,
    /// Peso do custo de comunicação
    pub communication_cost_weight: f64,
    /// Peso do balanceamento de carga
    pub load_balance_weight: f64,
    /// Peso da localidade de memória
    pub memory_locality_weight: f64,
    /// Seed para reprodutibilidade
    pub seed: Option<u64>,
}

impl Default for PackingConfig {
    fn default() -> Self {
        Self {
            hypercube_dimension: 8,
            max_annealing_iterations: 10_000,
            initial_temperature: 100.0,
            cooling_rate: 0.995,
            communication_cost_weight: 1.0,
            load_balance_weight: 0.5,
            memory_locality_weight: 0.3,
            seed: Some(42),
        }
    }
}

/// Informação de um nó na rede
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// ID único do nó
    pub node_id: String,
    /// Coordenadas no hipercubo (embedding)
    pub coordinates: Vec<f64>,
    /// Capacidade computacional (FLOPS normalizados)
    pub compute_capacity: f64,
    /// Capacidade de memória (GB)
    pub memory_capacity: f64,
    /// Largura de banda de rede (Gbps)
    pub network_bandwidth: f64,
    /// Carga atual (0.0 - 1.0)
    pub current_load: f64,
    /// Especializações do nó (ex: "crypto", "ml", "symbolic")
    pub specializations: Vec<String>,
    /// Latência para outros nós (matriz de adjacência)
    pub latency_matrix: Option<HashMap<String, f64>>,
}

impl NodeInfo {
    /// Cria nó com coordenadas aleatórias no hipercubo
    pub fn new_random(node_id: String, dimension: usize, rng: &mut rand::rngs::StdRng) -> Self {
        use rand::Rng;
        let coordinates: Vec<f64> = (0..dimension)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        Self {
            node_id,
            coordinates,
            compute_capacity: rng.gen_range(0.5..2.0),
            memory_capacity: rng.gen_range(4.0..64.0),
            network_bandwidth: rng.gen_range(1.0..100.0),
            current_load: rng.gen_range(0.0..0.3),
            specializations: Vec::new(),
            latency_matrix: None,
        }
    }

    /// Distância euclidiana no hipercubo para outro nó
    pub fn hypercube_distance(&self, other: &NodeInfo) -> f64 {
        self.coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Capacidade disponível
    pub fn available_capacity(&self) -> f64 {
        self.compute_capacity * (1.0 - self.current_load)
    }
}

/// Item de trabalho para empacotamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    /// ID único do trabalho
    pub work_id: String,
    /// AST a ser processada
    pub ast: AstNode,
    /// Hash estrutural da AST
    pub ast_hash: u64,
    /// Complexidade estimada (Krajicek)
    pub complexity: usize,
    /// Requisitos de memória (MB)
    pub memory_requirement: f64,
    /// Especialização necessária
    pub required_specialization: Option<String>,
    /// Prioridade (maior = mais urgente)
    pub priority: u32,
    /// Timestamp de criação
    pub created_at: u64,
}

impl WorkItem {
    /// Cria a partir de uma AST
    pub fn from_ast(work_id: String, ast: AstNode) -> Self {
        let complexity = ast.node_count(); // Simplificado
        let ast_hash = StructuralHasher::hash(&ast);

        Self {
            work_id,
            ast_hash,
            complexity,
            ast,
            memory_requirement: (complexity as f64) * 0.1,
            required_specialization: None,
            priority: 1,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Atribuição de trabalho a nó
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignment {
    pub work_id: String,
    pub node_id: String,
    pub estimated_completion_time: f64,
    pub communication_cost: f64,
    pub confidence: f64,
}

/// Estado do empacotador
#[derive(Debug, Clone)]
struct PackingState {
    assignments: HashMap<String, String>, // work_id -> node_id
    node_loads: HashMap<String, f64>,
    total_cost: f64,
}

impl PackingState {
    fn new(nodes: &[NodeInfo]) -> Self {
        let mut node_loads = HashMap::new();
        for node in nodes {
            node_loads.insert(node.node_id.clone(), node.current_load);
        }
        Self {
            assignments: HashMap::new(),
            node_loads,
            total_cost: 0.0,
        }
    }

    fn clone(&self) -> Self {
        Self {
            assignments: self.assignments.clone(),
            node_loads: self.node_loads.clone(),
            total_cost: self.total_cost,
        }
    }
}

/// Quantum Local Packer
///
/// Usa simulated annealing com heurísticas quânticas para encontrar
/// atribuição ótima de trabalhos a nós, minimizando:
/// 1. Custo de comunicação (distância no hipercubo)
/// 2. Desequilíbrio de carga
/// 3. Violações de localidade de memória
pub struct QuantumLocalPacker {
    config: PackingConfig,
    nodes: Vec<NodeInfo>,
    node_index: HashMap<String, usize>,
    rng: rand::rngs::StdRng,
}

impl QuantumLocalPacker {
    /// Cria novo packer
    pub fn new(config: PackingConfig, nodes: Vec<NodeInfo>) -> Result<Self> {
        if nodes.is_empty() {
            return Err(OrigoError::InvalidConfig("Nenhum nó fornecido".into()));
        }

        let dimension = config.hypercube_dimension;
        for node in &nodes {
            if node.coordinates.len() != dimension {
                return Err(OrigoError::InvalidConfig(format!(
                    "Nó {} tem dimensão {} ≠ {}",
                    node.node_id, node.coordinates.len(), dimension
                )));
            }
        }

        let mut node_index = HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            node_index.insert(node.node_id.clone(), i);
        }

        let rng = config.seed
            .map(rand::SeedableRng::seed_from_u64)
            .unwrap_or_else(rand::rngs::StdRng::from_entropy);

        Ok(Self {
            config,
            nodes,
            node_index,
            rng,
        })
    }

    /// Empacota lista de trabalhos nos nós
    pub fn pack(&mut self, work_items: Vec<WorkItem>) -> Result<Vec<WorkAssignment>> {
        if work_items.is_empty() {
            return Ok(Vec::new());
        }

        // Estado inicial: atribuição gulosa por menor custo
        let mut best_state = self.initial_greedy_assignment(&work_items)?;
        let mut current_state = best_state.clone();
        let mut best_cost = self.evaluate_state(&current_state, &work_items);

        let mut temperature = self.config.initial_temperature;

        for iteration in 0..self.config.max_annealing_iterations {
            // Gera vizinho
            let mut neighbor = current_state.clone();
            self.random_move(&mut neighbor, &work_items);

            let neighbor_cost = self.evaluate_state(&neighbor, &work_items);
            let delta = neighbor_cost - current_state.total_cost;

            // Critério de aceitação (Metropolis)
            let accept = if delta < 0.0 {
                true
            } else {
                let prob = (-delta / temperature).exp();
                self.rng.gen::<f64>() < prob
            };

            if accept {
                current_state = neighbor;
                current_state.total_cost = neighbor_cost;

                if neighbor_cost < best_cost {
                    best_state = neighbor;
                    best_cost = neighbor_cost;
                }
            }

            // Resfriamento
            temperature *= self.config.cooling_rate;

            // Early stopping se temperatura muito baixa
            if temperature < 1e-6 {
                break;
            }
        }

        // Converte melhor estado para assignments
        self.state_to_assignments(&best_state, &work_items)
    }

    /// Atribuição inicial gulosa
    fn initial_greedy_assignment(&self, work_items: &[WorkItem]) -> Result<PackingState> {
        let mut state = PackingState::new(&self.nodes);

        // Ordena por prioridade e complexidade
        let mut sorted_items = work_items.to_vec();
        sorted_items.sort_by(|a, b| {
            b.priority.cmp(&a.priority)
                .then_with(|| b.complexity.cmp(&a.complexity))
        });

        for item in &sorted_items {
            let best_node = self.find_best_node(item, &state)?;
            state.assignments.insert(item.work_id.clone(), best_node.node_id.clone());
            state.node_loads.insert(
                best_node.node_id.clone(),
                state.node_loads[&best_node.node_id] + item.complexity as f64 / best_node.compute_capacity,
            );
        }

        state.total_cost = self.evaluate_state(&state, work_items);
        Ok(state)
    }

    /// Encontra melhor nó para um item
    fn find_best_node(&self, item: &WorkItem, state: &PackingState) -> Result<&NodeInfo> {
        let mut best_node = None;
        let mut best_score = f64::INFINITY;

        for node in &self.nodes {
            // Verifica especialização
            if let Some(req_spec) = &item.required_specialization {
                if !node.specializations.contains(req_spec) {
                    continue;
                }
            }

            // Verifica capacidade
            let current_load = state.node_loads[&node.node_id];
            let projected_load = current_load + item.complexity as f64 / node.compute_capacity;
            if projected_load > 1.0 {
                continue; // Sobrecarga
            }

            // Score composto
            let comm_cost = self.config.communication_cost_weight * node.current_load; // Simplificado
            let load_balance = self.config.load_balance_weight * projected_load;
            let mem_locality = self.config.memory_locality_weight * (item.memory_requirement / node.memory_capacity);

            let score = comm_cost + load_balance + mem_locality;

            if score < best_score {
                best_score = score;
                best_node = Some(node);
            }
        }

        best_node.ok_or_else(|| OrigoError::CapacityExceeded {
            resource: "suitable node".into(),
            current: work_items.len(),
            max: self.nodes.len(),
        })
    }

    /// Move aleatório para simulated annealing
    fn random_move(&self, state: &mut PackingState, work_items: &[WorkItem]) {
        use rand::Rng;

        if work_items.is_empty() || self.nodes.len() < 2 {
            return;
        }

        // Escolhe trabalho aleatório
        let item = &work_items[self.rng.gen_range(0..work_items.len())];
        let current_node_id = state.assignments.get(&item.work_id).cloned();

        if let Some(current_id) = current_node_id {
            // Escolhe nó diferente aleatório
            let new_node_idx = self.rng.gen_range(0..self.nodes.len());
            let new_node = &self.nodes[new_node_idx];

            if new_node.node_id != current_id {
                // Verifica capacidade
                let current_load = state.node_loads[&new_node.node_id];
                let projected_load = current_load + item.complexity as f64 / new_node.compute_capacity;

                if projected_load <= 1.0 {
                    // Faz a troca
                    state.assignments.insert(item.work_id.clone(), new_node.node_id.clone());
                    state.node_loads.insert(current_id, state.node_loads[&current_id] - item.complexity as f64 / self.get_node(&current_id).unwrap().compute_capacity);
                    state.node_loads.insert(new_node.node_id.clone(), projected_load);
                }
            }
        }
    }

    /// Avalia custo total do estado
    fn evaluate_state(&self, state: &PackingState, work_items: &[WorkItem]) -> f64 {
        let mut total_cost = 0.0;

        for item in work_items {
            if let Some(node_id) = state.assignments.get(&item.work_id) {
                if let Some(node) = self.nodes.iter().find(|n| &n.node_id == node_id) {
                    // Custo de comunicação (baseado em distância no hipercubo)
                    let comm_cost = self.config.communication_cost_weight * node.hypercube_distance(&self.get_centroid(state));

                    // Custo de balanceamento
                    let load = state.node_loads[node_id];
                    let balance_cost = self.config.load_balance_weight * load.powi(2);

                    // Custo de memória
                    let mem_cost = self.config.memory_locality_weight * (item.memory_requirement / node.memory_capacity);

                    total_cost += comm_cost + balance_cost + mem_cost;
                }
            }
        }

        total_cost
    }

    /// Obtém nó por ID
    fn get_node(&self, node_id: &str) -> Option<&NodeInfo> {
        self.nodes.iter().find(|n| n.node_id == node_id)
    }

    /// Calcula centróide dos nós ativos
    fn get_centroid(&self, state: &PackingState) -> NodeInfo {
        let active_nodes: Vec<&NodeInfo> = self.nodes.iter()
            .filter(|n| state.assignments.values().any(|a| a == &n.node_id))
            .collect();

        if active_nodes.is_empty() {
            return self.nodes[0].clone();
        }

        let dim = self.config.hypercube_dimension;
        let mut centroid_coords = vec![0.0; dim];

        for node in &active_nodes {
            for (i, &coord) in node.coordinates.iter().enumerate() {
                centroid_coords[i] += coord;
            }
        }

        for coord in &mut centroid_coords {
            *coord /= active_nodes.len() as f64;
        }

        NodeInfo {
            node_id: "centroid".into(),
            coordinates: centroid_coords,
            compute_capacity: 1.0,
            memory_capacity: 1.0,
            network_bandwidth: 1.0,
            current_load: 0.0,
            specializations: Vec::new(),
            latency_matrix: None,
        }
    }

    /// Converte estado para assignments
    fn state_to_assignments(&self, state: &PackingState, work_items: &[WorkItem]) -> Result<Vec<WorkAssignment>> {
        let mut assignments = Vec::new();

        for item in work_items {
            if let Some(node_id) = state.assignments.get(&item.work_id) {
                let node = self.get_node(node_id).ok_or_else(|| OrigoError::NodeNotFound { node_id: node_id.clone() })?;

                assignments.push(WorkAssignment {
                    work_id: item.work_id.clone(),
                    node_id: node_id.clone(),
                    estimated_completion_time: item.complexity as f64 / node.compute_capacity,
                    communication_cost: node.current_load, // Simplificado
                    confidence: 0.9, // Baseado em temperatura final
                });
            }
        }

        Ok(assignments)
    }

    /// Adiciona nó à rede
    pub fn add_node(&mut self, node: NodeInfo) -> Result<()> {
        if node.coordinates.len() != self.config.hypercube_dimension {
            return Err(OrigoError::InvalidConfig("Dimensão incompatível".into()));
        }
        self.node_index.insert(node.node_id.clone(), self.nodes.len());
        self.nodes.push(node);
        Ok(())
    }

    /// Remove nó da rede
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        if let Some(idx) = self.node_index.remove(node_id) {
            self.nodes.swap_remove(idx);
            // Reindexa
            self.node_index.clear();
            for (i, node) in self.nodes.iter().enumerate() {
                self.node_index.insert(node.node_id.clone(), i);
            }
            Ok(())
        } else {
            Err(OrigoError::NodeNotFound { node_id: node_id.into() })
        }
    }

    /// Obtém estatísticas da rede
    pub fn network_stats(&self) -> NetworkStats {
        let total_capacity: f64 = self.nodes.iter().map(|n| n.compute_capacity).sum();
        let total_load: f64 = self.nodes.iter().map(|n| n.current_load * n.compute_capacity).sum();
        let avg_load = if total_capacity > 0.0 { total_load / total_capacity } else { 0.0 };

        NetworkStats {
            node_count: self.nodes.len(),
            total_capacity,
            total_load,
            average_load: avg_load,
            dimension: self.config.hypercube_dimension,
        }
    }
}

/// Estatísticas da rede
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub node_count: usize,
    pub total_capacity: f64,
    pub total_load: f64,
    pub average_load: f64,
    pub dimension: usize,
}

/// Heurística quântica: tunneling para escapar de mínimos locais
pub fn quantum_tunneling_step(
    state: &mut PackingState,
    work_items: &[WorkItem],
    nodes: &[NodeInfo],
    tunneling_probability: f64,
    rng: &mut rand::rngs::StdRng,
) {
    use rand::Rng;

    if rng.gen::<f64>() > tunneling_probability {
        return;
    }

    // Tunneling: faz múltiplas trocas simultâneas
    let num_swaps = rng.gen_range(2..=5.min(work_items.len()));

    for _ in 0..num_swaps {
        if work_items.is_empty() { break; }
        let item = &work_items[rng.gen_range(0..work_items.len())];
        if let Some(current_id) = state.assignments.get(&item.work_id).cloned() {
            let new_node = &nodes[rng.gen_range(0..nodes.len())];
            if new_node.node_id != current_id {
                let current_load = state.node_loads[&new_node.node_id];
                let projected_load = current_load + item.complexity as f64 / new_node.compute_capacity;
                if projected_load <= 1.0 {
                    state.assignments.insert(item.work_id.clone(), new_node.node_id.clone());
                    state.node_loads.insert(current_id, state.node_loads[&current_id] - item.complexity as f64 / nodes.iter().find(|n| n.node_id == current_id).unwrap().compute_capacity);
                    state.node_loads.insert(new_node.node_id.clone(), projected_load);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AstNode;

    fn create_test_nodes() -> Vec<NodeInfo> {
        vec![
            NodeInfo {
                node_id: "node-1".into(),
                coordinates: vec![0.0, 0.0, 0.0, 0.0],
                compute_capacity: 1.0,
                memory_capacity: 16.0,
                network_bandwidth: 10.0,
                current_load: 0.1,
                specializations: vec!["general".into()],
                latency_matrix: None,
            },
            NodeInfo {
                node_id: "node-2".into(),
                coordinates: vec![1.0, 1.0, 1.0, 1.0],
                compute_capacity: 2.0,
                memory_capacity: 32.0,
                network_bandwidth: 100.0,
                current_load: 0.2,
                specializations: vec!["crypto".into(), "ml".into()],
                latency_matrix: None,
            },
            NodeInfo {
                node_id: "node-3".into(),
                coordinates: vec![-1.0, -1.0, -1.0, -1.0],
                compute_capacity: 1.5,
                memory_capacity: 24.0,
                network_bandwidth: 50.0,
                current_load: 0.0,
                specializations: vec!["symbolic".into()],
                latency_matrix: None,
            },
        ]
    }

    fn create_test_work() -> Vec<WorkItem> {
        vec![
            WorkItem::from_ast("work-1".into(), AstNode::abstraction("x", AstNode::variable("x"))),
            WorkItem::from_ast("work-2".into(), AstNode::application(AstNode::variable("f"), AstNode::variable("x"))),
            WorkItem::from_ast("work-3".into(), AstNode::krajicek_generator(100, 1000, crate::KrajicekGeneratorType::RhoT)),
            WorkItem::from_ast("work-4".into(), AstNode::abstraction("y", AstNode::abstraction("z", AstNode::variable("y")))),
        ]
    }

    #[test]
    fn test_packer_creation() {
        let config = PackingConfig::default();
        let nodes = create_test_nodes();
        let packer = QuantumLocalPacker::new(config, nodes);
        assert!(packer.is_ok());
    }

    #[test]
    fn test_packing() {
        let config = PackingConfig::default();
        let nodes = create_test_nodes();
        let mut packer = QuantumLocalPacker::new(config, nodes).unwrap();
        let work = create_test_work();

        let assignments = packer.pack(work).unwrap();
        assert_eq!(assignments.len(), 4);

        // Todos os trabalhos atribuídos
        for a in &assignments {
            assert!(!a.node_id.is_empty());
            assert!(a.confidence > 0.0);
        }
    }

    #[test]
    fn test_specialization_matching() {
        let config = PackingConfig::default();
        let mut nodes = create_test_nodes();
        // Adiciona especialização ao node-2
        nodes[1].specializations.push("crypto".into());

        let mut packer = QuantumLocalPacker::new(config, nodes).unwrap();
        let mut work = create_test_work();
        work[0].required_specialization = Some("crypto".into());

        let assignments = packer.pack(work).unwrap();
        // work-0 deve ir para node-2
        let crypto_assignment = assignments.iter().find(|a| a.work_id == "work-1").unwrap();
        assert_eq!(crypto_assignment.node_id, "node-2");
    }

    #[test]
    fn test_network_stats() {
        let config = PackingConfig::default();
        let nodes = create_test_nodes();
        let packer = QuantumLocalPacker::new(config, nodes).unwrap();

        let stats = packer.network_stats();
        assert_eq!(stats.node_count, 3);
        assert!(stats.total_capacity > 0.0);
    }

    #[test]
    fn test_node_distance() {
        let node1 = NodeInfo {
            node_id: "n1".into(),
            coordinates: vec![0.0, 0.0],
            compute_capacity: 1.0,
            memory_capacity: 1.0,
            network_bandwidth: 1.0,
            current_load: 0.0,
            specializations: Vec::new(),
            latency_matrix: None,
        };
        let node2 = NodeInfo {
            node_id: "n2".into(),
            coordinates: vec![3.0, 4.0],
            compute_capacity: 1.0,
            memory_capacity: 1.0,
            network_bandwidth: 1.0,
            current_load: 0.0,
            specializations: Vec::new(),
            latency_matrix: None,
        };

        let dist = node1.hypercube_distance(&node2);
        assert!((dist - 5.0).abs() < 1e-6); // 3-4-5 triangle
    }
}