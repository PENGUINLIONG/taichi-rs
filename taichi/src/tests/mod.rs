use taichi_sys::*;
use crate::*;

fn get_platform_arch() -> TiArch {
    if cfg!(target_os = "macos") {
        TiArch::Metal
    } else {
        TiArch::Vulkan
    }
}

#[test]
fn test_create_runtime() {
    Runtime::new(get_platform_arch()).unwrap();
}
#[test]
fn test_host_accessible_memory_read_write() {
    let runtime = Runtime::new(get_platform_arch()).unwrap();
    let memory = runtime.allocate_memory()
        .size(128 * std::mem::size_of::<u32>())
        .host_read(true)
        .host_write(true)
        .build()
        .unwrap();

    let input = (0..128).into_iter().collect::<Vec<u32>>();
    memory.write(&input).unwrap();
    let mut output = [0u32; 128].to_vec();
    memory.read(&mut output).unwrap();
    assert_eq!(input, output);
}
#[test]
fn test_host_accessible_ndarray_read_write() {
    let runtime = Runtime::new(get_platform_arch()).unwrap();
    let ndarray = runtime.allocate_ndarray::<u32>()
        .shape([128, ])
        .host_read(true)
        .host_write(true)
        .build()
        .unwrap();

    let input = (0..128).into_iter().collect::<Vec<u32>>();
    ndarray.write(&input).unwrap();
    let mut output = [0u32; 128].to_vec();
    ndarray.read(&mut output).unwrap();
    assert_eq!(input, output);
}
#[test]
fn test_load_aot_module() {
    let runtime = Runtime::new(get_platform_arch()).unwrap();
    runtime.load_aot_module("../assets/chess_board/module").unwrap();
}
#[test]
fn test_launch_compute_graph() {
    let runtime = Runtime::new(get_platform_arch()).unwrap();
    let ndarray = runtime.allocate_ndarray::<i32>()
        .shape([16, 16])
        .host_read(true)
        .build()
        .unwrap();
    let module = runtime.load_aot_module("../assets/chess_board/module").unwrap();
    let mut g_run = module.get_compute_graph("g_run").unwrap();
    g_run.set_arg_ndarray("arr", &ndarray).unwrap();
    g_run.launch().unwrap();
    runtime.wait().unwrap();

    let mut expect_data = Vec::new();
    for i in 0..16 {
        for j in 0..16 {
            let x = (j * (16 + 1) + i) % 2;
            expect_data.push(x);
        }
    }
    let mut actual_data = [0; 16 * 16].to_vec();
    ndarray.read(&mut actual_data).unwrap();
    assert_eq!(expect_data, actual_data);
}
