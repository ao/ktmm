use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

use ktmm::{KtmmError, MouseMover, MouseMoverConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for version flag
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("ktmm {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config = parse_args();

    // Create a new MouseMover with the parsed configuration
    let mut mouse_mover = MouseMover::new(config);

    // Set up signal handling for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    setup_signal_handlers(running.clone());

    // Check for necessary permissions
    if let Err(e) = mouse_mover.check_permissions() {
        match e {
            #[cfg(target_os = "macos")]
            KtmmError::AccessibilityPermissionError => {
                eprintln!("Error: macOS accessibility permissions not granted.");
                eprintln!("{}", ktmm::platform::get_accessibility_guidance());
                return Err(e.into());
            }
            KtmmError::PlatformError(msg) => {
                eprintln!("Platform-specific error: {}", msg);
                eprintln!("{}", ktmm::platform::get_accessibility_guidance());
                return Err(KtmmError::PlatformError(msg).into());
            }
            _ => {
                eprintln!("Error checking permissions: {}", e);
                return Err(e.into());
            }
        }
    }

    println!("KTMM is running. Press Ctrl+C to exit.");
    
    // Main loop - runs in the current thread
    while running.load(Ordering::SeqCst) {
        // Instead of one long sleep, use shorter sleeps and check the running flag frequently
        let total_sleep_secs = mouse_mover.config.interval_secs;
        let check_interval_ms = 100; // Check every 100ms for interruption
        let iterations = (total_sleep_secs * 1000) / check_interval_ms;
        
        for _ in 0..iterations {
            if !running.load(Ordering::SeqCst) {
                break; // Exit the sleep loop if we've been signaled to stop
            }
            thread::sleep(Duration::from_millis(check_interval_ms));
        }
        
        // Only move the mouse if we're still running
        if running.load(Ordering::SeqCst) {
            if let Err(e) = mouse_mover.move_mouse_once() {
                eprintln!("Error moving mouse: {}", e);
                // Continue running despite errors
            }
        }
    }

    println!("KTMM has been cleanly shut down.");
    Ok(())
}

fn parse_args() -> MouseMoverConfig {
    // For now, we'll just use the default configuration
    // In a real implementation, we would use clap to parse command-line arguments
    MouseMoverConfig::default()

    // When clap is properly set up, we would do:
    /*
    let args = Args::parse();

    MouseMoverConfig {
        interval_secs: args.interval,
        movement_pixels: (args.x_movement, args.y_movement),
        return_delay_ms: args.delay,
    }
    */
}

fn setup_signal_handlers(running: Arc<AtomicBool>) {
    // Use ctrlc crate for all platforms for simplicity
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, shutting down...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");
}
