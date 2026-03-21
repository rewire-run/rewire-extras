fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from("src/proto/v1");
    std::fs::create_dir_all(&out_dir)?;

    tonic_prost_build::configure()
        .out_dir(&out_dir)
        .compile_protos(&["proto/rewire/v1/rewire.proto"], &["proto"])?;

    std::fs::rename(out_dir.join("rewire.v1.rs"), out_dir.join("rewire.rs"))?;

    println!("Generated proto code at src/proto/v1/rewire.rs");
    Ok(())
}
