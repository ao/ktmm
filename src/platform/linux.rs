//! Linux-specific functionality for KTMM

use crate::KtmmError;

/// Check if the application has the necessary permissions on Linux
pub fn check_accessibility_permissions() -> Result<(), KtmmError> {
    // Linux typically requires X11 permissions, but these are usually
    // granted by default. We could check for X11 availability here.

    // For Wayland, additional permissions might be needed
    #[cfg(target_os = "linux")]
    {
        // Check if we're running under Wayland
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // Wayland might restrict mouse control
            return Err(KtmmError::PlatformError(
                "Running under Wayland, which may restrict mouse control".to_string(),
            ));
        }
    }

    Ok(())
}

/// Get Linux-specific guidance for enabling permissions
pub fn get_accessibility_guidance() -> String {
    String::from(
        "Linux permissions for mouse control depend on your display server:\n\
        \n\
        For X11 (most common):\n\
        - No special permissions are typically needed\n\
        - If using a security-enhanced setup, check X11 access controls\n\
        \n\
        For Wayland:\n\
        - Mouse control may be restricted by design\n\
        - Consider using X11 instead for this application\n\
        - Some compositors may provide accessibility APIs\n\
        \n\
        If running in a virtual machine or remote desktop:\n\
        - Ensure the host system allows mouse control",
    )
}
