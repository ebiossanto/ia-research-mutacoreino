# ADR 001: Use Rust for Core Overlay Network

## Status
Aceito

## Contexto
O MutaCore requer uma camada de transporte (L2) com:
- Performance determinística (latência sub-ms)
- Segurança de memória sem garbage collection
- Verificação formal facilitada
- Interoperabilidade com Python (PyO3) e WASM
- Ecossistema maduro de criptografia (ring, sha2)

Alternativas consideradas: C++, Go, Python, Node.js

## Decisão
Usar **Rust 2021 edition** para todo o core (`origo-overlay` crate).

## Consequências

### Positivas
- Memory safety garantida pelo compilador
- Zero-cost abstractions para hot paths (hash, validação, packing)
- `proptest` para property-based testing nativo
- `criterion` para benchmarking estatístico
- PyO3 bindings maduros para Python
- `wasm32-unknown-unknown` target tier 1
- `thiserror`/`serde` para error handling e serialização robustos

### Negativas
- Curva de aprendizado mais íngreme
- Compilação mais lenta
- Ecossistema científico (linear algebra) menos maduro que Python/NumPy

### Riscos
- Dependência de `nalgebra` para Hodge (mitigado: uso mínimo no core)
- FFI complexity para WASM/Python (mitigado: PyO3/wasm-bindgen bem estabelecidos)

## Referências
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [PyO3 Documentation](https://pyo3.rs/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)