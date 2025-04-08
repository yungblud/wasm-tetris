import init, { draw_square } from "../pkg/wasm_tetris.js";

async function run() {
    await init(); // WebAssembly 모듈 로드
    draw_square();
}

run();
