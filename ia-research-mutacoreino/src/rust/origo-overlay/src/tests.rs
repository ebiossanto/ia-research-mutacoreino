// Integration tests for origo-overlay crate
// Run with: cargo test --test tests

#[cfg(test)]
mod integration_tests {
    use crate::{
        AlphaContext, OrigoAstValidator, AstNode, KrajicekThresholds,
        KrajicekGeneratorType, StructuralHasher, QuantumLocalPacker,
        PackingConfig, NodeInfo, WorkItem,
    };

    #[test]
    fn test_full_pipeline_ast_validation_and_hashing() {
        // 1. Create a complex AST
        let ast = AstNode::abstraction(
            "x",
            AstNode::abstraction(
                "y",
                AstNode::application(
                    AstNode::application(
                        AstNode::variable("f"),
                        AstNode::variable("x"),
                    ),
                    AstNode::variable("y"),
                ),
            ),
        );

        // 2. Canonicalize
        let mut ctx = AlphaContext::new();
        let canonical = ctx.canonicalize(&ast);

        // 3. Validate
        let thresholds = KrajicekThresholds::standard();
        let mut validator = OrigoAstValidator::new(thresholds);
        let result = validator.validate(&canonical).unwrap();
        assert!(result.valid);

        // 4. Hash
        let hash = StructuralHasher::hash(&canonical);
        assert_ne!(hash, 0);

        // 5. Alpha-equivalence check
        let ast2 = AstNode::abstraction(
            "a",
            AstNode::abstraction(
                "b",
                AstNode::application(
                    AstNode::application(
                        AstNode::variable("g"),
                        AstNode::variable("a"),
                    ),
                    AstNode::variable("b"),
                ),
            ),
        );
        assert!(ctx.is_alpha_isomorphic(&canonical, &ast2));
    }

    #[test]
    fn test_krajicek_thresholds_validation() {
        let mut validator = OrigoAstValidator::new_simple(100);
        
        // Within threshold
        let ast_ok = AstNode::krajicek_generator(50, 100, KrajicekGeneratorType::RhoT);
        assert!(validator.validate(&ast_ok).unwrap().valid);
        
        // Exceeds threshold
        let ast_fail = AstNode::krajicek_generator(150, 100, KrajicekGeneratorType::RhoT);
        assert!(validator.validate(&ast_fail).is_err());
    }

    #[test]
    fn test_gluing_failure_detection() {
        let mut validator = OrigoAstValidator::new_simple(100);
        
        let ast1 = AstNode::krajicek_generator(60, 100, KrajicekGeneratorType::RhoT);
        let ast2 = AstNode::krajicek_generator(60, 100, KrajicekGeneratorType::RhoT);
        
        // Combined = 120 > 100 (ρ_T)
        let result = validator.validate_gluing(&ast1, &ast2);
        assert!(result.is_err());
    }

    #[test]
    fn test_structural_hash_alpha_invariance() {
        // These are alpha-equivalent
        let ast1 = AstNode::abstraction("x", AstNode::variable("x"));
        let ast2 = AstNode::abstraction("y", AstNode::variable("y"));
        
        assert_eq!(StructuralHasher::hash(&ast1), StructuralHasher::hash(&ast2));
        
        // These are NOT alpha-equivalent
        let ast3 = AstNode::abstraction("x", AstNode::abstraction("y", AstNode::variable("x")));
        let ast4 = AstNode::abstraction("x", AstNode::abstraction("y", AstNode::variable("y")));
        
        assert_ne!(StructuralHasher::hash(&ast3), StructuralHasher::hash(&ast4));
    }

    #[test]
    fn test_merkle_tree_proof() {
        use crate::MerkleHasher;
        
        let mut merkle = MerkleHasher::new();
        merkle.add_leaf(StructuralHasher::hash(&AstNode::variable("x")));
        merkle.add_leaf(StructuralHasher::hash(&AstNode::variable("y")));
        merkle.add_leaf(StructuralHasher::hash(&AstNode::variable("z")));
        
        let root = merkle.root_hash();
        assert_ne!(root, 0);
        
        let proof = merkle.generate_proof(1).unwrap();
        assert!(MerkleHasher::verify_proof(merkle.leaf_hashes[1], &proof));
    }

    #[test]
    fn test_quantum_packing_basic() {
        let config = PackingConfig::default();
        let nodes = vec![
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
                specializations: vec!["crypto".into()],
                latency_matrix: None,
            },
        ];
        
        let mut packer = QuantumLocalPacker::new(config, nodes).unwrap();
        
        let work = vec![
            WorkItem::from_ast("work-1".into(), AstNode::variable("x")),
            WorkItem::from_ast("work-2".into(), AstNode::application(
                AstNode::variable("f"), AstNode::variable("x")
            )),
        ];
        
        let assignments = packer.pack(work).unwrap();
        assert_eq!(assignments.len(), 2);
        
        for a in &assignments {
            assert!(!a.node_id.is_empty());
            assert!(a.confidence > 0.0);
        }
    }

    #[test]
    fn test_quantum_packing_specialization() {
        let config = PackingConfig::default();
        let mut nodes = vec![
            NodeInfo {
                node_id: "general-node".into(),
                coordinates: vec![0.0, 0.0],
                compute_capacity: 1.0,
                memory_capacity: 16.0,
                network_bandwidth: 10.0,
                current_load: 0.0,
                specializations: vec!["general".into()],
                latency_matrix: None,
            },
            NodeInfo {
                node_id: "crypto-node".into(),
                coordinates: vec![1.0, 1.0],
                compute_capacity: 2.0,
                memory_capacity: 32.0,
                network_bandwidth: 100.0,
                current_load: 0.0,
                specializations: vec!["crypto".into()],
                latency_matrix: None,
            },
        ];
        
        let mut packer = QuantumLocalPacker::new(config, nodes).unwrap();
        
        let mut work = vec![
            WorkItem::from_ast("crypto-work".into(), AstNode::variable("x")),
        ];
        work[0].required_specialization = Some("crypto".into());
        
        let assignments = packer.pack(work).unwrap();
        assert_eq!(assignments[0].node_id, "crypto-node");
    }

    #[test]
    fn test_consensus_hash_order_invariant() {
        let asts = vec![
            AstNode::variable("x"),
            AstNode::variable("y"),
            AstNode::variable("z"),
        ];
        
        let refs: Vec<&AstNode> = asts.iter().collect();
        let hash1 = crate::consensus_hash(&refs);
        
        let refs_reversed: Vec<&AstNode> = asts.iter().rev().collect();
        let hash2 = crate::consensus_hash(&refs_reversed);
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_krajicek_thresholds_builder() {
        let thresholds = KrajicekThresholds::builder()
            .rho_t(5000)
            .b_t(2500)
            .c_t(1000)
            .r_t(1500)
            .build();
        
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value, 5000);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::BT).max_value, 2500);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::CT).max_value, 1000);
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RT).max_value, 1500);
    }

    #[test]
    fn test_thresholds_toml_roundtrip() {
        let thresholds = KrajicekThresholds::standard();
        let toml = thresholds.to_toml();
        let parsed = KrajicekThresholds::from_toml(&toml).unwrap();
        
        assert_eq!(thresholds.get_threshold(KrajicekGeneratorType::RhoT).max_value,
                   parsed.get_threshold(KrajicekGeneratorType::RhoT).max_value);
    }
}