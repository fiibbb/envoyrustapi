use std::{fs};
use std::path::{Path};

// eg: ("proto_deps/repos/client_model", "proto_deps/repos/client_model", "src")
//     => src/metrics.proto
//
// eg: ("proto_deps/repos/googleapis/google/api", "proto_deps/repos/googleapis", "src")
//     => src/google/api/[http.proto|context.proto|...]
//
// eg: ("proto_deps/repos/udpa/udpa", "proto_deps/repos/udpa", "src")
//     => src/upda/[...]
//
// eg: ("proto_deps/repos/udpa/xds", "proto_deps/repos/udpa", "src")
//     => src/xds/[...]
fn copy_proto_files_for_path(src_path: &Path, src_path_trim_prefix: &Path, dst_root: &Path) {
    let file_name = src_path.file_name()
                        .and_then(|name| name.to_str())
                        .expect("file_name");
    if src_path.is_dir() {
        for entry in fs::read_dir(src_path).expect("read_dir") {
            copy_proto_files_for_path((entry.expect("entry")).path().as_path(), src_path_trim_prefix, dst_root);
        }
    } else if file_name.ends_with(".proto") {
        let dst_sub_dir = src_path.strip_prefix(src_path_trim_prefix).expect("dst_sub_dir").parent().expect("parent");
        let dst_path = dst_root.join(dst_sub_dir);
        fs::create_dir_all(dst_path.as_path()).expect("create_dir_all");
        println!("copying: '{}' => '{}'", src_path.display(), dst_path.join(file_name).display());
        fs::copy(src_path, dst_path.join(file_name)).expect("copy");
    }
}

fn build_proto_farm() {
    let dst_root = Path::new("proto_deps/src");
    fs::create_dir_all(dst_root).expect("create_dir_all");
    copy_proto_files_for_path(Path::new("proto_deps/repos/client_model"), Path::new("proto_deps/repos/client_model"), dst_root);
    copy_proto_files_for_path(Path::new("proto_deps/repos/data-plane-api/envoy"), Path::new("proto_deps/repos/data-plane-api"), dst_root);
    copy_proto_files_for_path(Path::new("proto_deps/repos/googleapis/google/api"), Path::new("proto_deps/repos/googleapis"), dst_root);
    // copy_proto_files_for_path(Path::new("proto_deps/repos/opencensus-proto/src/opencensus/proto/"), Path::new("proto_deps/repos/opencensus-proto/src/opencensus"), dst_root);
    copy_proto_files_for_path(Path::new("proto_deps/repos/protobuf/src/google/protobuf"), Path::new("proto_deps/repos/protobuf/src"), dst_root);
    copy_proto_files_for_path(Path::new("proto_deps/repos/protoc-gen-validate/validate"), Path::new("proto_deps/repos/protoc-gen-validate"), dst_root);
    copy_proto_files_for_path(Path::new("proto_deps/repos/udpa/udpa"), Path::new("proto_deps/repos/udpa/"), dst_root);
    copy_proto_files_for_path(Path::new("proto_deps/repos/udpa/xds"), Path::new("proto_deps/repos/udpa/"), dst_root);
}

fn compile_proto_farm() {
    fs::create_dir_all(Path::new("src/api"));

}

fn main() {
    build_proto_farm();
    compile_proto_farm();
}

