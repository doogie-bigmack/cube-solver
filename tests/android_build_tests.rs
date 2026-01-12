use std::env;
use std::path::Path;
use std::process::Command;

#[test]
fn test_android_targets_installed() {
    // Check that all required Android targets are installed
    let output = Command::new("rustup")
        .args(&["target", "list", "--installed"])
        .output()
        .expect("Failed to run rustup");

    let targets = String::from_utf8_lossy(&output.stdout);

    assert!(targets.contains("aarch64-linux-android"), "aarch64-linux-android target not installed");
    assert!(targets.contains("armv7-linux-androideabi"), "armv7-linux-androideabi target not installed");
    assert!(targets.contains("i686-linux-android"), "i686-linux-android target not installed");
    assert!(targets.contains("x86_64-linux-android"), "x86_64-linux-android target not installed");
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
fn test_android_readme_exists() {
    // Verify Android build documentation exists
    assert!(Path::new("mobile/android/README.md").exists(), "mobile/android/README.md not found");
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
#[ignore] // This test requires Android SDK to be fully set up
fn test_android_home_set() {
    // Check if ANDROID_HOME environment variable is set
    let android_home = env::var("ANDROID_HOME");
    assert!(android_home.is_ok(), "ANDROID_HOME environment variable not set");

    if let Ok(path) = android_home {
        assert!(Path::new(&path).exists(), "ANDROID_HOME path does not exist: {}", path);
    }
}

#[test]
#[ignore] // This test requires Android NDK to be fully set up
fn test_android_ndk_home_set() {
    // Check if ANDROID_NDK_HOME environment variable is set
    let ndk_home = env::var("ANDROID_NDK_HOME");
    assert!(ndk_home.is_ok(), "ANDROID_NDK_HOME environment variable not set");

    if let Ok(path) = ndk_home {
        assert!(Path::new(&path).exists(), "ANDROID_NDK_HOME path does not exist: {}", path);
    }
}

#[test]
#[ignore] // This test requires full Android SDK setup with Java
fn test_can_build_android_debug() {
    // Try to build for Android in debug mode
    let output = Command::new("dx")
        .args(&["build", "--android"])
        .output()
        .expect("Failed to run dx build");

    assert!(output.status.success(), "Android debug build failed: {}",
        String::from_utf8_lossy(&output.stderr));
}

#[test]
#[ignore] // This test requires full Android SDK setup with Java
fn test_can_build_android_release() {
    // Try to build for Android in release mode
    let output = Command::new("dx")
        .args(&["build", "--android", "--release"])
        .output()
        .expect("Failed to run dx build");

    assert!(output.status.success(), "Android release build failed: {}",
        String::from_utf8_lossy(&output.stderr));
}

#[test]
#[ignore] // This test requires full Android SDK setup with Java
fn test_can_bundle_android_apk() {
    // Try to bundle Android APK
    let output = Command::new("dx")
        .args(&["bundle", "--android", "--release"])
        .output()
        .expect("Failed to run dx bundle");

    assert!(output.status.success(), "Android APK bundling failed: {}",
        String::from_utf8_lossy(&output.stderr));

    // Check if APK was created (path may vary)
    let apk_path = Path::new("target/dx/rubiks-cube-solver/bundle/android");
    assert!(apk_path.exists() || Path::new("dist").exists(),
        "APK bundle directory not found");
}

#[cfg(test)]
mod android_config_tests {
    use super::*;

    #[test]
    fn test_dioxus_config_has_bundle_identifier() {
        // Verify Dioxus.toml has bundle identifier for Android
        let dioxus_toml = std::fs::read_to_string("Dioxus.toml")
            .expect("Failed to read Dioxus.toml");

        assert!(dioxus_toml.contains("identifier"),
            "Dioxus.toml should contain bundle identifier");
    }

    #[test]
    fn test_android_minimum_version_documented() {
        // Check that minimum Android version is documented
        let readme = std::fs::read_to_string("mobile/android/README.md")
            .expect("Failed to read mobile/android/README.md");

        assert!(readme.contains("Android 10") || readme.contains("API level 29"),
            "README should document minimum Android version");
    }
}
