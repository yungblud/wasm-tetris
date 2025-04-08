import init, { init_webgl, draw_block } from "../pkg/wasm_tetris.js";

async function run() {
    await init(); // WebAssembly 모듈 로드

    init_webgl();
    console.log("✅ WebGL 초기화 성공 (Rust)");

    let x = 0.0;
    let y = 0.0;

    function gameLoop() {
        draw_block(x, y);
        x += 0.01;
        y += 0.01;
        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();
