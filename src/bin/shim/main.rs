//! Main entry point for the `tmkms` executable

use shim::application::APP;

/// Boot the `tmkms` application
fn main() {
    abscissa_core::boot(&APP);
}
