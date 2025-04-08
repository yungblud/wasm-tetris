import init, { GameState } from './pkg/wasm_tetris.js';

const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d');
const scale = 30;

function drawBlocks(blocks) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = 'cyan';

  blocks.forEach(block => {
    const { x, y } = block;
    ctx.fillRect(x * scale, y * scale, scale, scale);
  });
}

async function start() {
  await init();
  const game = new GameState();

  function render() {
    const blocks = game.blocks();
    const current = game.current();
    drawBlocks(blocks.concat(current));
    game.update();
    requestAnimationFrame(render);
  }

  render();
}
start();
