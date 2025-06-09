//! Platform-specific functionality for KTMM

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub mod unsupported;

use crate::KtmmError;

/// Check if the application has the necessary permissions to control the mouse
pub fn check_accessibility_permissions() -> Result<(), KtmmError> {
    #[cfg(target_os = "macos")]
    return macos::check_accessibility_permissions();

    #[cfg(target_os = "windows")]
    return windows::check_accessibility_permissions();

    #[cfg(target_os = "linux")]
    return linux::check_accessibility_permissions();

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    return unsupported::check_accessibility_permissions();
}

/// Get platform-specific guidance for enabling accessibility permissions
pub fn get_accessibility_guidance() -> String {
    #[cfg(target_os = "macos")]
    return macos::get_accessibility_guidance();

    #[cfg(target_os = "windows")]
    return windows::get_accessibility_guidance();

    #[cfg(target_os = "linux")]
    return linux::get_accessibility_guidance();

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    return unsupported::get_accessibility_guidance();
}
