import init, { State, uses_webgl } from "./pkg/wasm_unknown_example.js";

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
  const webgl = uses_webgl();
  const ctx = webgl
    ? canvas.getContext("webgl2", {
        alpha: true,
        antialias: false,
        depth: true,
        stencil: true,
        premultipliedAlpha: true,
        preserveDrawingBuffer: false,
      })
    : canvas.getContext("2d");
  if (!ctx) {
    throw new Error(webgl ? "WebGL2 not available" : "2D canvas context not available");
  }
  const state = new State(canvas.width, canvas.height, ctx);
  let latestMouse = { x: canvas.width * 0.5, y: canvas.height * 0.5 };
  let frameRequested = false;
  let pointerActive = false;

  const drawNow = () => {
    state.draw(latestMouse.x, latestMouse.y);
  };

  const scheduleDraw = () => {
    if (!webgl) {
      drawNow();
      return;
    }
    if (frameRequested || !pointerActive) {
      return;
    }
    frameRequested = true;
    requestAnimationFrame(() => {
      frameRequested = false;
      drawNow();
    });
  };

  canvas.addEventListener("pointerenter", (e) => {
    pointerActive = true;
    const rect = canvas.getBoundingClientRect();
    latestMouse = { x: e.clientX - rect.left, y: e.clientY - rect.top };
    scheduleDraw();
  });
  canvas.addEventListener("pointermove", (e) => {
    const rect = canvas.getBoundingClientRect();
    latestMouse = { x: e.clientX - rect.left, y: e.clientY - rect.top };
    scheduleDraw();
  });
  canvas.addEventListener("pointerleave", () => {
    pointerActive = false;
  });

  window.addEventListener("resize", () => {
    if (resizeCanvas(canvas)) {
      state.resize(canvas.width, canvas.height);
      latestMouse = { x: canvas.width * 0.5, y: canvas.height * 0.5 };
      if (webgl && pointerActive) {
        scheduleDraw();
      }
    }
  });
}

main().catch(console.error);
