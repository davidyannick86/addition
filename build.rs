use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let descritor_path = std::path::PathBuf::from("proto_descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(descritor_path)
        .compile_protos(&["proto/calculator.proto"], &["proto"])?;

    Ok(())
}
