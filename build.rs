fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("find vendored protoc");
    std::env::set_var("PROTOC", protoc);
    println!("cargo:rerun-if-changed=proto/portless/v1/control.proto");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/portless/v1/control.proto"], &["proto"])
        .expect("compile portless protobuf contracts");
}
