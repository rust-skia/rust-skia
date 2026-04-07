import init, {
  test_plain_rust_alloc,
  test_sk_data_alloc,
  test_surface_alloc,
  test_surface_alloc_sequential,
} from "./pkg/wasm_unknown_testing.js";

function print(log, msg) {
  const text = typeof msg === "string" ? msg : String(msg);
  const line = document.createElement("pre");
  const ok = text.startsWith("ok:");
  line.style.color = ok ? "#2a2" : "#c22";
  line.textContent = text;
  log.appendChild(line);
  console.log(text);
}

function formatSimResult(w, h, n, failedAt) {
  if (failedAt < 0) {
    const bytes = w * h * 4;
    const total = n * bytes;
    return `ok: allocated and drew on ${n} surfaces (${w}x${h}, ${bytes} bytes each, ${total} total)`;
  }

  const bytes = w * h * 4;
  const total = failedAt * bytes;
  return `FAILED at surface ${failedAt}/${n} (${w}x${h}, ${bytes} bytes each, ${total} total allocated)`;
}

function formatSeqResult(w, h, n, failedAt) {
  if (failedAt < 0) {
    return `ok: allocated ${n} surfaces sequentially (${w}x${h})`;
  }
  return `FAILED at surface ${failedAt}/${n} (${w}x${h}) — sequential alloc`;
}

function formatPlainAlloc(bytes, rounds, failedAt) {
  if (failedAt < 0) {
    return `ok: plain-rust-alloc ${rounds} x ${bytes} bytes`;
  }
  return `FAILED: plain-rust-alloc at round ${failedAt}/${rounds} (${bytes} bytes)`;
}

function formatSkDataAlloc(bytes, rounds, failedAt) {
  if (failedAt < 0) {
    return `ok: sk-data-alloc ${rounds} x ${bytes} bytes`;
  }
  return `FAILED: sk-data-alloc at round ${failedAt}/${rounds} (${bytes} bytes)`;
}

async function main() {
  await init();
  const log = document.getElementById("log");

  print(log, formatPlainAlloc(4 * 1024 * 1024, 8, test_plain_rust_alloc(4 * 1024 * 1024, 8)));

  print(log, formatSkDataAlloc(256 * 1024, 32, test_sk_data_alloc(256 * 1024, 32)));
  print(log, "=== simultaneous allocation ===");
  for (const [w, h, n] of [
    [256, 256, 4],
    [512, 512, 4],
    [1024, 1024, 4],
    [1920, 1080, 4],
    [1920, 1080, 8],
    [1920, 1080, 16],
    [3840, 2160, 4],
    [3840, 2160, 8],
  ]) {
    const failedAt = test_surface_alloc(w, h, n);
    print(log, formatSimResult(w, h, n, failedAt));
  }

  print(log, "");
  print(log, "=== sequential allocation (alloc+drop per iteration) ===");
  for (const [w, h, n] of [
    [1920, 1080, 16],
    [1920, 1080, 64],
    [3840, 2160, 16],
    [3840, 2160, 64],
  ]) {
    const failedAt = test_surface_alloc_sequential(w, h, n);
    print(log, formatSeqResult(w, h, n, failedAt));
  }
}

main().catch((e) => {
  console.error(e);
  const log = document.getElementById("log");
  const line = document.createElement("pre");
  line.style.color = "#c22";
  line.textContent = e?.stack || e?.message || String(e);
  log.appendChild(line);
});
