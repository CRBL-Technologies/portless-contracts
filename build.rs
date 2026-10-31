fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("find vendored protoc");
    std::env::set_var("PROTOC", protoc);
    println!("cargo:rerun-if-changed=proto/portless/v1/control.proto");
    let grpc = std::env::var_os("CARGO_FEATURE_GRPC").is_some();
    tonic_prost_build::configure()
        .build_client(grpc)
        .build_server(grpc)
        .compile_protos(&["proto/portless/v1/control.proto"], &["proto"])
        .expect("compile portless protobuf contracts");
}
