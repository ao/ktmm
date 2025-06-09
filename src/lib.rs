use device_query::{DeviceQuery, DeviceState};
use enigo::{Enigo, MouseControllable};
use std::{thread, time::Duration};

// Platform-specific functionality
pub mod platform;

/// Error types for KTMM operations
#[derive(Debug)]
pub enum KtmmError {
    /// Error when mouse control fails
    MouseControlError(String),
    /// Error when platform-specific operations fail
    PlatformError(String),
    /// Error when accessibility permissions are not granted (macOS)
    #[cfg(target_os = "macos")]
    AccessibilityPermissionError,
    /// Generic error
    Other(String),
}

impl std::fmt::Display for KtmmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KtmmError::MouseControlError(msg) => write!(f, "Mouse control error: {}", msg),
            KtmmError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            #[cfg(target_os = "macos")]
            KtmmError::AccessibilityPermissionError => {
                write!(f, "macOS accessibility permission not granted")
            }
            KtmmError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for KtmmError {}

/// Configuration for mouse movement
#[derive(Debug, Clone)]
pub struct MouseMoverConfig {
    /// Time to wait between mouse movements in seconds
    pub interval_secs: u64,
    /// Distance to move the mouse in pixels (x, y)
    pub movement_pixels: (i32, i32),
    /// Time to wait before moving back in milliseconds
    pub return_delay_ms: u64,
}

impl Default for MouseMoverConfig {
    fn default() -> Self {
        Self {
            interval_secs: 60,
            movement_pixels: (1, 1),
            return_delay_ms: 6,
        }
    }
}

/// The main mouse mover struct
pub struct MouseMover {
    pub config: MouseMoverConfig,
    enigo: Enigo,
    device_state: DeviceState,
    running: bool,
}

impl MouseMover {
    /// Create a new MouseMover with the given configuration
    pub fn new(config: MouseMoverConfig) -> Self {
        Self {
            config,
            enigo: Enigo::new(),
            device_state: DeviceState::new(),
            running: false,
        }
    }

    /// Create a new MouseMover with default configuration
    pub fn default() -> Self {
        Self::new(MouseMoverConfig::default())
    }

    /// Check if the application has the necessary permissions
    pub fn check_permissions(&self) -> Result<(), KtmmError> {
        // Use platform-specific implementation
        platform::check_accessibility_permissions()
    }

    /// Perform a single mouse movement cycle
    pub fn move_mouse_once(&mut self) -> Result<(), KtmmError> {
        // Get current mouse position
        let mouse_pos = self.device_state.get_mouse();
        let (x, y) = (mouse_pos.coords.0, mouse_pos.coords.1);

        // Move mouse by the configured amount
        let (dx, dy) = self.config.movement_pixels;
        self.enigo.mouse_move_to(x + dx, y + dy);

        // Sleep for the configured delay
        thread::sleep(Duration::from_millis(self.config.return_delay_ms));

        // Move mouse back to original position
        self.enigo.mouse_move_to(x, y);

        Ok(())
    }

    /// Start the mouse mover loop
    pub fn start(&mut self) -> Result<(), KtmmError> {
        // Check permissions first
        self.check_permissions()?;

        self.running = true;

        while self.running {
            // Sleep for the configured interval
            thread::sleep(Duration::from_secs(self.config.interval_secs));

            // Move the mouse
            if let Err(e) = self.move_mouse_once() {
                eprintln!("Error moving mouse: {}", e);
                // Continue running despite errors
            }
        }

        Ok(())
    }

    /// Stop the mouse mover loop
    pub fn stop(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This mock implementation is used in tests/mock_tests.rs
    // We're keeping it here as a reference for how to mock the dependencies
    #[allow(dead_code)]
    struct MockEnigo {
        positions: Arc<Mutex<Vec<(i32, i32)>>>,
    }

    #[allow(dead_code)]
    impl MockEnigo {
        fn new() -> Self {
            Self {
                positions: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn mouse_move_to(&mut self, x: i32, y: i32) {
            self.positions.lock().unwrap().push((x, y));
        }

        fn get_positions(&self) -> Vec<(i32, i32)> {
            self.positions.lock().unwrap().clone()
        }
    }

    #[test]
    fn test_config_default() {
        let config = MouseMoverConfig::default();
        assert_eq!(config.interval_secs, 60);
        assert_eq!(config.movement_pixels, (1, 1));
        assert_eq!(config.return_delay_ms, 6);
    }

    #[test]
    fn test_custom_config() {
        let config = MouseMoverConfig {
            interval_secs: 30,
            movement_pixels: (2, 3),
            return_delay_ms: 10,
        };

        assert_eq!(config.interval_secs, 30);
        assert_eq!(config.movement_pixels, (2, 3));
        assert_eq!(config.return_delay_ms, 10);
    }
}
