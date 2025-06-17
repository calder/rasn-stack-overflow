use rasn_compiler::prelude::*;

fn main() {
    println!("cargo::rerun-if-changed=src/main.asn1");
    Compiler::<RasnBackend, _>::new()
        .add_asn_by_path("src/main.asn1")
        .set_output_path("src")
        .compile()
        .unwrap();
}
