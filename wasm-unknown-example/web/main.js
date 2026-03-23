import init, { render_scene } from "./pkg/wasm_unknown_example.js";

function resizeCanvasToDisplaySize(canvas) {
  const width = Math.max(1, canvas.clientWidth);
  const height = Math.max(1, canvas.clientHeight);
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
    return true;
  }
  return false;
}

function draw(canvas, context) {
  resizeCanvasToDisplaySize(canvas);
  const width = canvas.width;
  const height = canvas.height;
  const expectedLength = width * height * 4;
  const pixels = render_scene(width, height);
  const actualLength = pixels?.length ?? 0;

  if (expectedLength === 0 || actualLength !== expectedLength) {
    console.error(
      "render_scene returned unexpected pixel buffer length",
      { width, height, expectedLength, actualLength },
    );
    return;
  }

  try {
    const clamped =
      pixels instanceof Uint8Array
        ? new Uint8ClampedArray(pixels.buffer, pixels.byteOffset, pixels.byteLength)
        : new Uint8ClampedArray(pixels);
    const imageData = new ImageData(clamped, width, height);
    context.putImageData(imageData, 0, 0);
  } catch (error) {
    console.error("Failed to draw rendered pixels", error);
  }
}

async function main() {
  await init();

  const canvas = document.getElementById("skia-canvas");
  const context = canvas.getContext("2d");
  if (!context) {
    throw new Error("2D canvas context is not available");
  }

  const render = () => draw(canvas, context);
  render();
  window.addEventListener("resize", render);
}

main().catch((error) => {
  console.error(error);
});
