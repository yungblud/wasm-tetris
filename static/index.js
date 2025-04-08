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

    // 바닥 또는 다른 블록과 충돌하면 고정
    const hit_bottom = current.y <= -0.9;
    const hit_block = check_collision(blocks, current);

    if (hit_bottom || hit_block) {
        blocks.push({ ...current });
        current = { x: 0, y: 1.0 };
    }

    // 화면 초기화 및 다시 그리기
    draw_square_at(current.x, current.y); // 현재 블록
    for (const b of blocks) {
        draw_square_at(b.x, b.y); // 기존 블록들
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

function check_collision(blocks, current) {
    for (const b of blocks) {
        const dy = b.y - current.y;
        const dx = Math.abs(b.x - current.x);
        if (dy < 0.2 && dy > 0 && dx < 0.2) {
            return true;
        }
    }
    return false;
}

init().then(setup);
