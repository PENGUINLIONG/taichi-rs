import argparse

import taichi as ti

WIDTH = 16
HEIGHT = 16

def compile_graph_aot(arch):
    ti.init(arch=arch)

    if ti.lang.impl.current_cfg().arch != arch:
        return

    @ti.kernel
    def chess_board(arr: ti.types.ndarray(field_dim=2)):
        for i, j in arr:
            value = ti.cast((j * (WIDTH + 1) + i) % 2, ti.i32)
            arr[i, j] = value


    _arr = ti.graph.Arg(ti.graph.ArgKind.NDARRAY,
                        'arr',
                        ti.i32,
                        field_dim=2,
                        element_shape=())

    g_builder = ti.graph.GraphBuilder()
    g_builder.dispatch(chess_board, _arr)
    run_graph = g_builder.compile()

    mod = ti.aot.Module(arch)
    mod.add_graph('g_run', run_graph)
    mod.save("assets/chess_board/module", '')


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--arch", type=str)
    args = parser.parse_args()

    if args.arch == "vulkan":
        compile_graph_aot(arch=ti.vulkan)
    else:
        assert False
