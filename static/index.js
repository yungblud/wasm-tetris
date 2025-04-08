import init, { draw_square_at } from "./pkg/wasm_tetris.js";

let blocks = [];
let current = { x: 0, y: 1.0 };
let speed = 0.01;
let lastTime = 0;

let keys = {
    left: false,
    right: false,
    down: false,
};

function handleInput() {
    if (keys.left) {
        current.x -= 0.05;
        keys.left = false;
    }
    if (keys.right) {
        current.x += 0.05;
        keys.right = false;
    }
    if (keys.down) {
        current.y -= 0.05;
        keys.down = false;
    }
}

function update(time) {
    let delta = (time - lastTime) / 1000;
    lastTime = time;

    handleInput();

    current.y -= speed;

    // 바닥에 닿으면 고정
    if (current.y <= -0.9) {
        blocks.push({ ...current });
        current = { x: 0, y: 1.0 }; // 새로운 블록 생성
    }

    // 화면 클리어
    draw_square_at(current.x, current.y);

    // 기존 블록 다시 그리기
    for (const b of blocks) {
        draw_square_at(b.x, b.y);
    }

    requestAnimationFrame(update);
}

function setup() {
    window.addEventListener("keydown", (e) => {
        if (e.key === "ArrowLeft") keys.left = true;
        if (e.key === "ArrowRight") keys.right = true;
        if (e.key === "ArrowDown") keys.down = true;
    });

    requestAnimationFrame(update);
}

init().then(setup);
