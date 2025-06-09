//! Fallback implementation for unsupported platforms

use crate::KtmmError;

/// Check if the application has the necessary permissions on unsupported platforms
pub fn check_accessibility_permissions() -> Result<(), KtmmError> {
    Err(KtmmError::PlatformError(
        "This platform is not officially supported by KTMM".to_string(),
    ))
}

/// Get guidance for unsupported platforms
pub fn get_accessibility_guidance() -> String {
    String::from(
        "Your platform is not officially supported by KTMM.\n\
        \n\
        KTMM is designed to work on:\n\
        - Windows\n\
        - macOS\n\
        - Linux (X11)\n\
        \n\
        While the application may still function on your platform,\n\
        we cannot guarantee full compatibility or provide specific guidance.\n\
        \n\
        If you encounter issues, please report them on our GitHub repository\n\
        with details about your operating system and environment.",
    )
}
