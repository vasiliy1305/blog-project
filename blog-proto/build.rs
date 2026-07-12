fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/blog.proto");

    tonic_prost_build::compile_protos("proto/blog.proto")?;

    Ok(())
}
