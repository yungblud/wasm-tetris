import init, { draw_square_at } from "../pkg/wasm_tetris.js";


let x = 0;
let y = 0.0;

async function run() {
    await init(); // WebAssembly 모듈 로드
    draw_square_at(x, y);

    window.addEventListener("keydown", (e) => {
        if (e.key === "ArrowLeft") x -= 0.05;
        else if (e.key === "ArrowRight") x += 0.05;
        else if (e.key === "ArrowDown") y -= 0.05;
        else if (e.key === "ArrowUp") y += 0.05;

        draw_square_at(x, y);
    });
}

run();
