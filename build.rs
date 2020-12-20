use std::collections::HashSet;
use std::fs;
use std::path::Path;

use glob;
use protoc_rust::{Codegen,Customize};

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
fn copy_proto_files_for_path(
    src_path: &Path,
    src_path_trim_prefix: &Path,
    dst_root: &Path,
    exclude_paths: Option<&HashSet<&Path>>,
) {
    if let Some(paths) = exclude_paths {
        if paths.contains(src_path) {
            return;
        }
    }

    let file_name = src_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("file_name");
    if src_path.is_dir() {
        for entry in fs::read_dir(src_path).expect("read_dir") {
            copy_proto_files_for_path(
                (entry.expect("entry")).path().as_path(),
                src_path_trim_prefix,
                dst_root,
                exclude_paths,
            );
        }
    } else if file_name.ends_with(".proto") {
        let dst_sub_dir = src_path
            .strip_prefix(src_path_trim_prefix)
            .expect("dst_sub_dir")
            .parent()
            .expect("parent");
        let dst_path = dst_root.join(dst_sub_dir);
        fs::create_dir_all(dst_path.as_path()).expect("create_dir_all");
        println!(
            "copying: '{}' => '{}'",
            src_path.display(),
            dst_path.join(file_name).display()
        );
        fs::copy(src_path, dst_path.join(file_name)).expect("copy");
    }
}

fn build_proto_farm() {
    let dst_root = Path::new("proto_deps/src");
    fs::remove_dir_all(dst_root).expect("remove_dir_all");
    fs::create_dir_all(dst_root).expect("create_dir_all");
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protoc-gen-validate/validate"),
        Path::new("proto_deps/repos/protoc-gen-validate"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/data-plane-api/envoy"),
        Path::new("proto_deps/repos/data-plane-api"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/udpa/udpa"),
        Path::new("proto_deps/repos/udpa/"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/udpa/xds"),
        Path::new("proto_deps/repos/udpa/"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/client_model"),
        Path::new("proto_deps/repos/client_model"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/opencensus-proto/src/opencensus/proto/"),
        Path::new("proto_deps/repos/opencensus-proto/src"),
        dst_root,
        None,
    );

    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/any.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/api.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/descriptor.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/duration.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/empty.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/field_mask.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/source_context.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/struct.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/timestamp.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/type.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/protobuf/src/google/protobuf/wrappers.proto"),
        Path::new("proto_deps/repos/protobuf/src"),
        dst_root,
        None,
    );

    copy_proto_files_for_path(
        Path::new("proto_deps/repos/googleapis/google/api/annotations.proto"),
        Path::new("proto_deps/repos/googleapis"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/googleapis/google/api/http.proto"),
        Path::new("proto_deps/repos/googleapis"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/googleapis/google/api/expr/v1alpha1/syntax.proto"),
        Path::new("proto_deps/repos/googleapis"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/googleapis/google/api/expr/v1alpha1/checked.proto"),
        Path::new("proto_deps/repos/googleapis"),
        dst_root,
        None,
    );
    copy_proto_files_for_path(
        Path::new("proto_deps/repos/googleapis/google/rpc/status.proto"),
        Path::new("proto_deps/repos/googleapis"),
        dst_root,
        None,
    );
}

fn compile_proto_farm() {
    let dst_root = Path::new("src/api");
    fs::remove_dir_all(dst_root).expect("remove_dir_all");
    fs::create_dir_all(dst_root).expect("create_dir_all");

    let src_prefix = "proto_deps/src";
    let inputs = glob::glob(&format!("{}/**/*.proto", src_prefix))
        .expect("glob")
        .map(|entry| entry.expect("glob entry"));

    inputs.for_each(|input| {
        let out_path = dst_root.join(input.parent().expect("parent").strip_prefix(src_prefix).expect("strip_prefix"));
        let out_path = out_path.as_path();
        fs::create_dir_all(out_path).expect("create_dir_all");
        Codegen::new().out_dir(out_path).include(src_prefix).input(input).customize(Customize{
                .. Customize::default()
        }).run().expect("codegen");
    });
}

fn main() {
    build_proto_farm();
    compile_proto_farm();
}
