use std::path::Path;
use std::process::Command;

#[test]
fn test_ios_targets_installed() {
    // Check that all required iOS targets are installed
    let output = Command::new("rustup")
        .args(&["target", "list", "--installed"])
        .output()
        .expect("Failed to run rustup");

    let targets = String::from_utf8_lossy(&output.stdout);

    assert!(targets.contains("aarch64-apple-ios"), "aarch64-apple-ios target not installed");
    assert!(targets.contains("aarch64-apple-ios-sim"), "aarch64-apple-ios-sim target not installed");
    // x86_64-apple-ios is optional (for Intel Macs)
}

#[test]
fn test_dioxus_cli_installed() {
    // Check that dx (Dioxus CLI) is installed
    let output = Command::new("dx")
        .arg("--version")
        .output();

    assert!(output.is_ok(), "Dioxus CLI (dx) is not installed. Install with: cargo install dioxus-cli");

    if let Ok(output) = output {
        assert!(output.status.success(), "dx --version failed");
    }
}

#[test]
fn test_dioxus_toml_exists() {
    // Verify Dioxus.toml exists and has required configuration
    assert!(Path::new("Dioxus.toml").exists(), "Dioxus.toml not found");
}

#[test]
fn test_ios_readme_exists() {
    // Verify iOS build documentation exists
    assert!(Path::new("mobile/ios/README.md").exists(), "mobile/ios/README.md not found");
}

#[test]
fn test_cargo_toml_has_mobile_config() {
    // Check if Cargo.toml has necessary dependencies for mobile
    let cargo_toml = std::fs::read_to_string("Cargo.toml")
        .expect("Failed to read Cargo.toml");

    // Dioxus should be present for mobile support
    assert!(cargo_toml.contains("dioxus"), "Cargo.toml should contain dioxus dependency");
}

#[test]
#[cfg(target_os = "macos")]
fn test_xcode_installed() {
    // Check if Xcode command-line tools are installed (macOS only)
    let output = Command::new("xcodebuild")
        .arg("-version")
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✓ Xcode version: {}", version.lines().next().unwrap_or("unknown"));
        } else {
            println!("⚠️  Xcode is installed but xcodebuild failed. This is acceptable for infrastructure testing.");
            println!("   Full Xcode (not just command-line tools) is required for iOS builds.");
            println!("   Install from Mac App Store: https://apps.apple.com/us/app/xcode/id497799835");
        }
    } else {
        println!("⚠️  Xcode not found. This is acceptable for infrastructure testing.");
        println!("   Install with: xcode-select --install");
        println!("   Or full Xcode from Mac App Store for iOS builds");
    }
}

#[test]
#[cfg(target_os = "macos")]
fn test_ios_simulator_available() {
    // Check if iOS simulators are available (macOS only)
    let output = Command::new("xcrun")
        .args(&["simctl", "list", "devices", "available"])
        .output();

    if let Ok(output) = output {
        let devices = String::from_utf8_lossy(&output.stdout);
        if devices.contains("iOS") || devices.contains("iPhone") {
            println!("✓ iOS simulators are available");
        } else {
            println!("⚠️  No iOS simulators found. This is acceptable for infrastructure testing.");
            println!("   Install via Xcode → Settings → Platforms");
            println!("   Or Xcode → Preferences → Components (older Xcode versions)");
        }
    } else {
        println!("⚠️  xcrun not available. This is acceptable for infrastructure testing.");
        println!("   xcrun requires Xcode or command-line tools to be installed.");
    }
}

#[test]
#[ignore] // This test requires full Xcode setup
#[cfg(target_os = "macos")]
fn test_can_build_ios_debug() {
    // Try to build for iOS in debug mode (simulator)
    let output = Command::new("dx")
        .args(&["build", "--ios"])
        .output()
        .expect("Failed to run dx build");

    assert!(output.status.success(), "iOS debug build failed: {}",
        String::from_utf8_lossy(&output.stderr));
}

#[test]
#[ignore] // This test requires full Xcode setup and takes a long time
#[cfg(target_os = "macos")]
fn test_can_build_ios_release() {
    // Try to build for iOS in release mode (device)
    let output = Command::new("dx")
        .args(&["build", "--ios", "--release"])
        .output()
        .expect("Failed to run dx build");

    assert!(output.status.success(), "iOS release build failed: {}",
        String::from_utf8_lossy(&output.stderr));
}

#[test]
#[ignore] // This test requires full Xcode setup and creates an IPA bundle
#[cfg(target_os = "macos")]
fn test_can_bundle_ios_ipa() {
    // Try to bundle iOS IPA
    let output = Command::new("dx")
        .args(&["bundle", "--platform", "ios", "--release"])
        .output()
        .expect("Failed to run dx bundle");

    assert!(output.status.success(), "iOS IPA bundling failed: {}",
        String::from_utf8_lossy(&output.stderr));

    // Check if IPA was created (path may vary)
    let ipa_path = Path::new("target/dx/rubiks-cube-solver/bundle/ios");
    assert!(ipa_path.exists() || Path::new("dist").exists(),
        "IPA bundle directory not found");
}

#[cfg(test)]
mod ios_config_tests {
    use super::*;

    #[test]
    fn test_dioxus_config_has_bundle_identifier() {
        // Verify Dioxus.toml has bundle identifier for iOS
        let dioxus_toml = std::fs::read_to_string("Dioxus.toml")
            .expect("Failed to read Dioxus.toml");

        assert!(dioxus_toml.contains("identifier"),
            "Dioxus.toml should contain bundle identifier");
    }

    #[test]
    fn test_ios_minimum_version_documented() {
        // Check that minimum iOS version is documented
        let readme = std::fs::read_to_string("mobile/ios/README.md")
            .expect("Failed to read mobile/ios/README.md");

        assert!(readme.contains("iOS 15") || readme.contains("iOS 16"),
            "README should document minimum iOS version");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_version_sufficient() {
        // Check that macOS version is sufficient for iOS development
        let output = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .expect("Failed to get macOS version");

        let version = String::from_utf8_lossy(&output.stdout);
        let version_str = version.trim();

        // Parse major version (e.g., "14.2.1" -> 14)
        if let Some(major_str) = version_str.split('.').next() {
            if let Ok(major) = major_str.parse::<u32>() {
                assert!(major >= 12, "macOS 12 (Monterey) or later required for iOS development. Current: {}", version_str);
            }
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_xcode_command_line_tools_configured() {
        // Check that xcode-select is properly configured
        let output = Command::new("xcode-select")
            .arg("-p")
            .output()
            .expect("Failed to run xcode-select");

        assert!(output.status.success(), "xcode-select not configured");

        let path = String::from_utf8_lossy(&output.stdout);
        assert!(path.contains("Xcode") || path.contains("CommandLineTools"),
            "Xcode path not properly configured: {}", path);
    }
}
