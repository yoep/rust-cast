use std::env;
extern crate protoc_rust;

fn main() {
    let generate_proto = env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string());
    if generate_proto == "true" {
        protoc_rust::Codegen::new()
            .out_dir("src/cast")
            .inputs(&[
                "protobuf/authority_keys.proto",
                "protobuf/cast_channel.proto",
            ])
            .includes(&["protobuf"])
            .run()
            .expect("protoc");
    }

    println!("rerun-if-env-changed=GENERATE_PROTO");
    println!("rerun-if-changed=protobuf/authority_keys.proto");
    println!("rerun-if-changed=protobuf/cast_channel.proto");
}
