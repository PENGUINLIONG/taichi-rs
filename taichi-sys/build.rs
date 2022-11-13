use std::{env, fs, path};
use dotenvy::dotenv;

fn main() {
    let _ = dotenv();

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let mut search_paths = vec![
        "/usr/local/lib/".to_owned(),
        "/usr/lib64/".to_owned(),
        "/usr/lib/".to_owned(),
    ];
    if let Ok(taichi_c_api_install_dir) = env::var("TAICHI_C_API_INSTALL_DIR") {
        search_paths.push(taichi_c_api_install_dir + "/lib");
    }

    match target_os.as_str() {
        "macos" => {
            let path = search_paths.iter().filter_map(|search_path| {
                    let path = search_path.clone() + "/libtaichi_c_api.dylib";
                    fs::canonicalize(path).ok()
                        .map(|x| x.to_string_lossy().to_string())
                        .filter(|x| path::Path::new(x).exists())
                })
                .next()
                .expect("cannot find taichi runtime library");

            println!("cargo:rerun-if-changed={}", &path);

            let search_path = path::Path::new(&path).parent().unwrap().to_string_lossy();
            println!("cargo:rustc-link-search=native={search_path}");
            println!("cargo:rustc-link-lib=dylib=taichi_c_api");
        },
        _ => panic!("unsupported os"),
    };

}
