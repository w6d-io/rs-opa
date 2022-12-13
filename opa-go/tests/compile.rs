use std::{fs, path::PathBuf};
use opa_go::wasm::Wasm;

#[test]
fn test_opa_compiler_compile() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bytes = Wasm::new("data.tests.allow", &root.join("tests/empty.rego"))
        .build()
        .unwrap();
    let expected = fs::read(&root.join("tests/empty.wasm")).unwrap();
    assert_eq!(expected, bytes);
}
