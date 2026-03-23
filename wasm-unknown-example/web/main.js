import init, { State } from "./pkg/wasm_unknown_example.js";

function resizeCanvas(canvas) {
  const width = Math.max(1, canvas.clientWidth);
  const height = Math.max(1, canvas.clientHeight);
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
    return true;
  }
  return false;
}

async function main() {
  await init();

  const canvas = document.getElementById("glcanvas");
  resizeCanvas(canvas);
  const ctx = canvas.getContext("2d");
  const state = new State(canvas.width, canvas.height, ctx);

  window.addEventListener("mousemove", (e) => {
    const rect = canvas.getBoundingClientRect();
    state.draw(e.clientX - rect.x, e.clientY - rect.y);
  });

  window.addEventListener("resize", () => {
    if (resizeCanvas(canvas)) {
      state.resize(canvas.width, canvas.height);
    }
  });
}

main().catch(console.error);
