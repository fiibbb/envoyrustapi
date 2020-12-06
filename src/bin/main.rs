use std::fs;
use std::path::{Path, PathBuf};

use protobuf_codegen_pure;

fn must_glob_proto_files_recusive(prefix: &Path, path: &Path, files: &mut Vec<PathBuf>) {
    let file_name = path
        .file_name()
        .expect("file_name")
        .to_str()
        .expect("to_str");
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("read_dir") {
            let sub_dir = &entry.expect("entry").path();
            must_glob_proto_files_recusive(prefix, sub_dir, files);
        }
    } else if file_name.ends_with(".proto") {
        let file = path.strip_prefix(prefix).expect("strip_prefix");
        files.push(PathBuf::from(file));
        println!("found file: {}", file.display());
    }
}

fn must_glob_proto_files(paths: Vec<&Path>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        must_glob_proto_files_recusive(path.parent().expect("parent"), path, &mut files);
    }
    files
}

fn main() {
    let root_paths = vec![
        Path::new("proto_deps/protobuf/src/google"),
        Path::new("proto_deps/protoc-gen-validate/validate"),
        Path::new("proto_deps/opencensus-proto/src/opencensus"),
        Path::new("proto_deps/udpa/udpa"),
        Path::new("proto_deps/udpa/xds"),
        Path::new("proto_deps/data-plane-api/envoy"),
    ];
    must_glob_proto_files(root_paths);
}
