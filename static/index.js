import init, { greet } from "../pkg/wasm_tetris.js";

async function run() {
    await init(); // WebAssembly 모듈 로드
    console.log(greet()); // Rust에서 보낸 문자열 출력
}

run();
