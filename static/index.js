import init, { draw_square_at } from "../pkg/wasm_tetris.js";

let x = 0;
let y = 1.0; // 화면 상단부터 시작
let speed = 0.01; // 낙하 속도
let lastTime = 0;

let keys = {
    left: false,
    right: false,
    down: false,
};

function handleInput() {
    if (keys.left) {
        x -= 0.02;
        keys.left = false;
    }
    if (keys.right) {
        x += 0.02;
        keys.right = false;
    }
    if (keys.down) {
        y -= 0.05;
        keys.down = false;
    }
}

function update(time) {
    let delta = (time - lastTime) / 1000;
    lastTime = time;

    handleInput();

    y -= speed;

    draw_square_at(x, y);

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
