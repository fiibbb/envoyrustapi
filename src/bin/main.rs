use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use glob;
use protoc_rust as proto;

fn must_glob_proto_files_recursive(prefix: &Path, path: &Path, files: &mut Vec<PathBuf>) {
    let file_name = path
        .file_name()
        .expect("file_name")
        .to_str()
        .expect("to_str");
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("read_dir") {
            let sub_dir = &entry.expect("entry").path();
            must_glob_proto_files_recursive(prefix, sub_dir, files);
        }
    } else if file_name.ends_with(".proto") {
        let file = path.strip_prefix(prefix).expect("strip_prefix");
        files.push(PathBuf::from(file));
    }
}

fn must_glob_proto_files(root_paths: Vec<&Path>) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut files = HashMap::new();
    for root_path in root_paths {
        let mut files_under_path = Vec::new();
        let root_path_parent = root_path.parent().expect("parent");
        must_glob_proto_files_recursive(root_path_parent, root_path, &mut files_under_path);
        files.insert(PathBuf::from(root_path), files_under_path);
    }
    files
}

fn must_build_proto_farm(files: HashMap<PathBuf, Vec<PathBuf>>, farm_root: &Path) {
    for (dir, dir_files) in files {
        for dir_file in dir_files {
            let file_path = PathBuf::from(dir.parent().expect("parent")).join(&dir_file);
            let tmp_file_path = farm_root.join(dir_file.as_path());
            println!(
                "copying into proto farm: '{}' -> '{}'",
                file_path.as_path().display(),
                tmp_file_path.as_path().display()
            );
            fs::create_dir_all(tmp_file_path.parent().expect("parent")).expect("create_dir_all");
            fs::copy(file_path, tmp_file_path).expect("copy");
        }
    }
}

fn must_compile_protos(src_dir: &Path, dst_dir: &Path) {
    let pattern = format!(
        "{}/**/*.proto",
        src_dir.as_os_str().to_str().expect("to_str")
    );
    let inputs = glob::glob(&pattern)
        .expect("glob")
        .map(|x| x.expect("pathbuf unwrap"));
    fs::create_dir_all(dst_dir).expect("create_dir_all");

    inputs.for_each(|input| {
        println!(
            "compiling: {} -> {}, using {}",
            input.display(),
            dst_dir.display(),
            src_dir.display()
        );
        let mut compiler = proto::Codegen::new();
        compiler.out_dir(dst_dir);
        compiler.include(src_dir);
        compiler.input(input);
        compiler.run().expect("compilation failed");
    });
}

fn main() {
    let root_paths = vec![
        Path::new("proto_deps/protobuf/src/google"),
        Path::new("proto_deps/googleapis/google"),
        Path::new("proto_deps/protoc-gen-validate/validate"),
        Path::new("proto_deps/opencensus-proto/src/opencensus"),
        Path::new("proto_deps/udpa/udpa"),
        Path::new("proto_deps/udpa/xds"),
        Path::new("proto_deps/data-plane-api/envoy"),
    ];
    let farm_root = &Path::new("protos/");
    let out_root = &Path::new("out/");
    must_build_proto_farm(must_glob_proto_files(root_paths), farm_root);
    must_compile_protos(farm_root, out_root);
}
