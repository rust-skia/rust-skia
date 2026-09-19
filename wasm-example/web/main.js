/**
 * Make a canvas element fit to the display window.
 */
function resizeCanvasToDisplaySize(canvas) {
  const width = canvas.clientWidth || 1;
  const height = canvas.clientHeight || 1;
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
    return true;
  }
  return false;
}

// This loads and initialize our WASM module
createRustSkiaModule().then((RustSkia) => {
  // Create the WebGL context
  let context;
  const canvas = document.querySelector("#glcanvas");
  context = canvas.getContext("webgl2", {
    antialias: true,
    depth: true,
    stencil: true,
    // The Skia surface is cleared to transparent each frame so the page
    // background shows through.
    alpha: true,
    premultipliedAlpha: true,
  });

  // Register the context with emscripten
  handle = RustSkia.GL.registerContext(context, { majorVersion: 2 });
  RustSkia.GL.makeContextCurrent(handle);

  // Fit the canvas to the viewport
  resizeCanvasToDisplaySize(canvas);

  // Initialize Skia
  const state = RustSkia._init(canvas.width, canvas.height);

  // Draw the animated logo at the last mouse position (default: center).
  let mouseX = (canvas.width / 2) | 0;
  let mouseY = (canvas.height / 2) | 0;
  window.addEventListener("mousemove", (event) => {
    const canvasPos = canvas.getBoundingClientRect();
    mouseX = event.clientX - canvasPos.x;
    mouseY = event.clientY - canvasPos.y;
  });

  // Animation loop: the requestAnimationFrame timestamp drives the animation
  // phase, the cursor drives the position.
  function drawFrame(timestampMs) {
    resizeCanvasToDisplaySize(canvas) &&
      RustSkia._resize_surface(state, canvas.width, canvas.height);
    RustSkia._draw_logo(state, mouseX, mouseY, timestampMs);
    requestAnimationFrame(drawFrame);
  }
  requestAnimationFrame(drawFrame);
});
