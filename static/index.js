import init, { Game, draw_square_at } from "./pkg/wasm_tetris.js";

let game;

function update() {
    game.update();

    const x = game.current_x();
    const y = game.current_y();
    draw_square_at(x, y);

    const xs = game.blocks_x();
    const ys = game.blocks_y();
    for (let i = 0; i < xs.length; i++) {
        draw_square_at(xs[i], ys[i]);
    }

    requestAnimationFrame(update);
}

init().then(() => {
    game = new Game();

    window.addEventListener("keydown", (e) => {
        if (e.key === "ArrowLeft") game.move_left();
        if (e.key === "ArrowRight") game.move_right();
        if (e.key === "ArrowDown") game.move_down();
    });

    requestAnimationFrame(update);
});
