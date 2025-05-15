fn main() {    

    tonic_build::compile_protos("src/proto/example.proto").unwrap();

}
