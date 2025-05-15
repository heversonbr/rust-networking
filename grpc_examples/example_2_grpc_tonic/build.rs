fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())

    // This tells tonic-build to compile your protobufs when you build your Rust project.
    // While you can configure this build process in a number of ways. 


    


}