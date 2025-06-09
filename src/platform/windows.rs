//! Windows-specific functionality for KTMM

use crate::KtmmError;

/// Check if the application has the necessary permissions on Windows
pub fn check_accessibility_permissions() -> Result<(), KtmmError> {
    // Windows typically doesn't require special accessibility permissions
    // for mouse control, but we could check for admin rights if needed
    Ok(())
}

/// Get Windows-specific guidance for enabling permissions
pub fn get_accessibility_guidance() -> String {
    String::from(
        "Windows typically doesn't require special permissions for mouse control.\n\
        If you're experiencing issues:\n\
        1. Try running the application as Administrator\n\
        2. Check if any security software is blocking the application\n\
        3. Ensure no other software is capturing mouse input exclusively",
    )
}
