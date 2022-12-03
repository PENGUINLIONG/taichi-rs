use std::{env, fs, path};
use dotenvy::dotenv;

pub fn is_docs_rs() -> bool {
    env::var("DOCS_RS").is_ok()
}
pub fn out_dir() -> String {
    env::var("OUT_DIR").unwrap()
}
pub fn target_os() -> String {
    env::var("CARGO_CFG_TARGET_OS").unwrap()
}
pub fn taichi_c_api_install_dir() -> String {
    env::var("TAICHI_C_API_INSTALL_DIR")
        .expect("'TAICHI_C_API_INSTALL_DIR' is not specified")
}

fn proc_taichi_c_api() -> Option<String> {
    let target_os = target_os();

    // Deduce linking library and runtime library names on the target platform.
    let lib_name = match target_os.as_str() {
        "macos" => "/lib/libtaichi_c_api.dylib",
        "linux" => "/lib/libtaichi_c_api.so",
        "windows" => "/lib/taichi_c_api.lib",
        _ => panic!("unsupported os '{}'", &target_os),
    };
    let rt_name = match target_os.as_str() {
        "macos" => "/lib/libtaichi_c_api.dylib",
        "linux" => "/lib/libtaichi_c_api.so",
        "windows" => "/bin/taichi_c_api.dll",
        _ => panic!("unsupported os '{}'", &target_os),
    };

    // Find `taichi_c_api`.
    let lib_path = {
        let path = taichi_c_api_install_dir() + lib_name;
        fs::canonicalize(path).ok()
            .map(|x| x.to_string_lossy().to_string())
            .filter(|x| path::Path::new(x).exists())
            .expect("cannot find c-api linking library")
    };
    let rt_path =  {
        let path = taichi_c_api_install_dir() + rt_name;
        fs::canonicalize(path).ok()
            .map(|x| x.to_string_lossy().to_string())
            .filter(|x| path::Path::new(x).exists())
            .expect("cannot find c-api runtime library")
    };

    println!("cargo:rerun-if-changed={}", &lib_path);
    let search_path = path::Path::new(&lib_path)
        .parent()
        .unwrap()
        .to_string_lossy();
    println!("cargo:rustc-link-search=native={}", &search_path);
    let rt_file_name = path::Path::new(&rt_path)
        .file_name()
        .unwrap()
        .to_string_lossy();
    println!("cargo:rustc-link-lib=dylib=taichi_c_api");
    // FIXME: (penguinliong) This relative path is a hack.
    fs::copy(&rt_path, out_dir() + "/../../../deps/" + &rt_file_name).unwrap();

    None
}

fn main() {
    let _ = dotenv();

    if is_docs_rs() {
        return;
    }

    proc_taichi_c_api();
}
