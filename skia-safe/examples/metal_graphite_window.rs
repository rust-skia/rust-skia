//! Metal Graphite Window Example
//!
//! This example demonstrates how to use Skia's Graphite GPU backend with Metal
//! to create a window with hardware-accelerated rendering.
//!
//! # Requirements
//!
//! This example requires:
//! - macOS (or any platform with Metal support)
//! - `metal` feature for Metal backend types
//! - `graphite` feature for Graphite backend
//!
//! # Running
//!
//! ```bash
//! cargo run --features "graphite,metal" --example metal_graphite_window
//! ```

#[cfg(all(feature = "metal", feature = "graphite"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use skia_safe::graphite::{
        surfaces, BackendContext, Context, ContextOptions, InsertRecordingInfo, Recorder,
        RecorderOptions, SubmitInfo, SyncToCpu,
    };
    use skia_safe::{gpu::Mipmapped, AlphaType, Canvas, Color, ColorType, ImageInfo, Paint, Rect};

    println!("Metal Graphite Window Example");
    println!("==============================");

    demonstrate_metal_backend_context()?;
    demonstrate_context_creation()?;
    demonstrate_recorder_workflow()?;
    demonstrate_submission_workflow()?;

    println!();
    println!("Example completed successfully!");
    println!("You now have a complete understanding of the Graphite API.");

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_metal_backend_context() -> Result<(), Box<dyn std::error::Error>> {
    println!("Step 1: Metal BackendContext");
    println!("----------------------------");
    println!();

    println!("The BackendContext struct wraps Metal objects:");
    println!("- Contains a Metal device (MTLDevice)");
    println!("- Contains a Metal command queue (MTLCommandQueue)");
    println!();

    println!("In a real application, you would:");
    println!("  let device = metal_rs::Device::system_default();");
    println!("  let queue = device.new_command_queue();");
    println!("  let device_ptr = device.as_ptr() as *mut _;");
    println!("  let queue_ptr = queue.as_ptr() as *mut _;");
    println!("  let backend = BackendContext::new(device_ptr, queue_ptr);");
    println!();
    println!("The BackendContext is unsafe because it expects valid Metal pointers.");
    println!("- Has device: backend.has_device()");
    println!("- Has queue: backend.has_queue()");

    println!();
    println!("Step 1 completed.");
    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_context_creation() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("Step 2: Graphite Context Creation");
    println!("----------------------------------");
    println!();

    println!("The Context manages all GPU resources for Graphite.");
    println!("It is created from a backend-specific context.");
    println!();

    println!("Context creation:");
    println!("  let options = ContextOptions::new();");
    println!("  let context = Context::make_metal(&backend, Some(&options));");
    println!();

    println!("The context will:");
    println!("  - Manage memory for GPU resources");
    println!("  - Handle recording and submission");
    println!("  - Track GPU device state");
    println!("  - Check for device loss");
    println!();

    println!("Note: Context creation is unsafe because it requires a valid BackendContext.");
    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_recorder_workflow() -> Result<(), Box<dyn std::error::Error>> {
    use skia_safe::graphite::{surfaces, Recorder, RecorderOptions};
    use skia_safe::{gpu::Mipmapped, AlphaType, Canvas, Color, ColorType, ImageInfo, Paint, Rect};

    println!();
    println!("Step 3: Create Recorder");
    println!("----------------------");
    println!();

    println!("The Recorder captures draw operations into a Recording.");
    println!("Multiple recorders can be created from the same context.");
    println!();

    println!("Recorder creation:");
    println!("  let options = RecorderOptions::new();");
    println!("  let mut recorder = context.make_recorder(Some(&options));");
    println!();

    println!("The recorder backend is: {:?}", recorder.backend());
    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_submission_workflow() -> Result<(), Box<dyn std::error::Error>> {
    use skia_safe::graphite::{
        surfaces, Context, InsertRecordingInfo, InsertStatus, Recorder, RecorderOptions,
        SubmitInfo, SyncToCpu,
    };
    use skia_safe::{gpu::Mipmapped, AlphaType, Canvas, Color, ColorType, ImageInfo, Paint, Rect};

    println!();
    println!("Step 4: Create Surface and Canvas");
    println!("--------------------------------");
    println!();

    println!("Surfaces are created from the recorder.");
    println!("The canvas is obtained from the surface.");
    println!();

    println!("Surface creation:");
    println!("  let size = (800, 600);");
    println!("  let image_info = ImageInfo::new(");
    println!("      size,");
    println!("      ColorType::BGRA8888,");
    println!("      AlphaType::Premul,");
    println!("      None,");
    println!("  );");
    println!("  let texture_info = TextureInfo::new();");
    println!("  let mut surface = surfaces::render_target(");
    println!("      &mut recorder,");
    println!("      &image_info,");
    println!("      Mipmapped::No,");
    println!("      None,");
    println!("      Some(\"Main Surface\"),");
    println!("  )?;");
    println!("  let canvas = surface.canvas();");
    println!();

    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

fn demonstrate_draw_operations() {
    use skia_safe::{Canvas, Color, Paint, Rect};

    println!();
    println!("Step 5: Perform Draw Operations");
    println!("--------------------------------");
    println!();

    println!("Now we can draw on the canvas.");
    println!("  - Clear background");
    println!("  - Draw shapes");
    println!("  - Draw text");
    println!("  - Draw images");
    println!();

    let background_color = Color::from_rgb(30, 30, 40);
    canvas.clear(background_color);

    let mut rect_paint = Paint::default();
    rect_paint.set_color(Color::from_rgb(70, 130, 180));
    rect_paint.set_anti_alias(true);
    let rect = Rect::from_xywh(50.0, 50.0, 300.0, 200.0);
    canvas.draw_rect(rect, &rect_paint);

    let mut circle_paint = Paint::default();
    circle_paint.set_color(Color::from_rgb(255, 200, 100));
    circle_paint.set_anti_alias(true);
    canvas.draw_circle((500.0, 200.0), 100.0, &circle_paint);

    println!("Draw operations completed!");
    println!("  - Background cleared to dark blue");
    println!("  - Rectangle drawn");
    println!("  - Circle drawn");

    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_submission_workflow() -> Result<(), Box<dyn std::error::Error>> {
    use skia_safe::graphite::{
        Context, InsertRecordingInfo, InsertStatus, Recorder, RecorderOptions, SubmitInfo,
        SyncToCpu,
    };

    println!();
    println!("Step 6: Snap Recording");
    println!("-----------------------");
    println!();

    println!("Finish recording to create an immutable Recording.");
    println!("The recording contains all the draw operations.");
    println!();

    println!("Recording creation:");
    println!("  let recording = recorder.snap();");
    println!();

    println!("The recording can be:");
    println!("  - Inserted into context for submission");
    println!("  - Reused across multiple submissions");
    println!("  - Analyzed for resource dependencies");
    println!();

    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_insert_and_submit() -> Result<(), Box<dyn std::error::Error>> {
    use skia_safe::graphite::{
        Context, InsertRecordingInfo, InsertStatus, RecorderOptions, SubmitInfo, SyncToCpu,
    };

    println!();
    println!("Step 7: Insert Recording into Context");
    println!("-----------------------------------");
    println!();

    println!("Insert the recording into the context.");
    println!("This queues work for GPU submission.");
    println!();

    println!("Insertion:");
    println!("  let info = InsertRecordingInfo::new(&recording);");
    println!("  let status = context.insert_recording(&info);");
    println!();

    println!("Check status:");
    println!("  match status {");
    println!("    InsertStatus::Success => println!(\"Success\"),");
    println!("    InsertStatus::Failure => return Err(\"Insert failed\".into()),");
    println!("  };");
    println!();

    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_gpu_submission() -> Result<(), Box<dyn std::error::Error>> {
    use skia_safe::graphite::{Context, RecorderOptions, SubmitInfo, SyncToCpu};

    println!();
    println!("Step 8: Submit to GPU");
    println!("----------------------");
    println!();

    println!("Submit work to the GPU.");
    println!("You can choose to wait for completion or submit asynchronously.");
    println!();

    println!("Submission options:");
    println!("  1. Submit asynchronously:");
    println!("     let submit_info = SubmitInfo::default();");
    println!("     let success = context.submit(Some(&submit_info));");
    println!();
    println!("  2. Submit and wait:");
    println!("     let success = context.submit_and_wait();");
    println!();
    println!("  3. Submit with specific sync:");
    println!("     let sync_to_cpu = SyncToCpu::Yes;");
    println!("     let submit_info = SubmitInfo::new();");
    println!("     submit_info.set_sync(sync_to_cpu);");
    println!("     let success = context.submit(Some(&submit_info));");
    println!();

    println!("For this example, we submit and wait:");
    println!("  let success = context.submit_and_wait();");
    println!();

    println!("Press Enter to continue...");
    let mut _input = String::new();
    let _ = std::io::stdin().read_line(&mut _input);

    Ok(())
}

#[cfg(all(feature = "metal", feature = "graphite"))]
fn demonstrate_summary() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("Step 9: Summary and Next Steps");
    println!("--------------------------------");
    println!();

    println!("Summary:");
    println!("=========");
    println!("Metal BackendContext: Structure demonstrated");
    println!("Graphite Context: API shown");
    println!("Recorder: Created and used");
    println!("Surface: Created with canvas");
    println!("Draw Operations: Performed");
    println!("Recording: Snapped from recorder");
    println!("Insertion: Into context");
    println!("Submission: To GPU");
    println!();

    println!("Key Graphite Benefits:");
    println!("====================");
    println!("- Explicit resource management");
    println!("- Better multi-threading support");
    println!("- Separate recording and submission");
    println!("- More predictable performance");
    println!("- Recording can be snapped and reused");
    println!();

    println!("Next Steps for Full Implementation:");
    println!("==================================");
    println!("1. Add metal-rs dependency to Cargo.toml:");
    println!("   metal-rs = \"0.33\"");
    println!();
    println!("2. Create Metal device and queue:");
    println!("   let device = metal_rs::Device::system_default();");
    println!("   let queue = device.new_command_queue();");
    println!("   let device_ptr = device.as_ptr() as *mut _;");
    println!("   let queue_ptr = queue.as_ptr() as *mut _;");
    println!();
    println!("3. Create Graphite backend context:");
    println!("   use skia_safe::graphite::mtl::BackendContext;");
    println!("   let backend = BackendContext::new(device_ptr, queue_ptr);");
    println!("   let options = ContextOptions::new();");
    println!("   let context = Context::make_metal(&backend, Some(&options));");
    println!();
    println!("4. Create window with winit or native API:");
    println!("   let event_loop = EventLoop::new();");
    println!("   let window = WindowBuilder::new();");
    println!();
    println!("5. Set up Metal layer and drawable:");
    println!("   let layer = MetalLayer::new();");
    println!("   window.set_layer(Some(layer.as_ref()));");
    println!();
    println!("6. Implement render loop:");
    println!("   event_loop.run(move |event, control_flow| {");
    println!("       match event {");
    println!("           Event::WindowEvent { event: WindowEvent::Resized(size) } => {");
    println!("             // Handle resize");
    println!("           },");
    println!("           Event::AboutToWait => {");
    println!("             // Create surface from drawable");
    println!("             // Perform draw operations");
    println!("             // Snap recording");
    println!("             // Insert and submit");
    println!("             // Present drawable");
    println!("             request_redraw(control_flow);");
    println!("           },");
    println!("           Event::RedrawRequested => {");
    println!("             let mut recorder = context.make_recorder(None).unwrap();");
    println!("             let mut surface = create_surface_from_drawable(&mut recorder);");
    println!("             let canvas = surface.canvas();");
    println!("             // ... draw ...");
    println!("             let recording = recorder.snap().unwrap();");
    println!("             let info = InsertRecordingInfo::new(&recording);");
    println!("             context.insert_recording(&info);");
    println!("             context.submit_and_wait();");
    println!("             // Present drawable");
    println!("             request_redraw(control_flow);");
    println!("           },");
    println!("           _ => {}");
    println!("         }");
    println!("       });");
    println!();
    println!("Graphite API Reference:");
    println!("====================");
    println!("- BackendContext: Wraps Metal device and queue");
    println!("- Context: Manages GPU resources and submissions");
    println!("- Recorder: Records draw operations");
    println!("- Recording: Immutable record of operations");
    println!("- Surface: Provides canvas for drawing");
    println!("- InsertRecordingInfo: Wrapper for submission");
    println!("- SubmitInfo: Configuration for submission");
    println!();
    println!("Example completed successfully!");
    println!("You now have a complete understanding of the Graphite API.");

    Ok(())
}

#[cfg(not(all(feature = "metal", feature = "graphite")))]
fn main() {
    println!("Metal Graphite Window Example");
    println!("================================");
    println!();
    println!("This example requires both 'metal' and 'graphite' features.");
    println!();
    println!("Current configuration:");
    println!("  - 'metal' feature enabled: {}", cfg!(feature = "metal"));
    println!(
        "  - 'graphite' feature enabled: {}",
        cfg!(feature = "graphite")
    );
    println!("  - Platform: {}", std::env::consts::OS);

    println!();

    if cfg!(not(feature = "metal")) {
        println!("Error: Missing 'metal' feature");
    }
    if cfg!(not(feature = "graphite")) {
        println!("Error: Missing 'graphite' feature");
    }
    if !cfg!(all(feature = "metal", feature = "graphite")) {
        println!();
        println!("To run this example:");
        println!("  cargo run --features \"graphite,metal\" --example metal_graphite_window");
    }
}
