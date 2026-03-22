// feature_b.rs

// Testmodul
// #![cfg(test)]
// ...

// feature_a.rs
struct FeatureA;

#[cfg(test)]
mod tests {
    // Tests zu Feature A
    // ...
}

#[cfg(target_vendor = "apple")]
struct InfoApple;

#[cfg(target_family = "windows")]
struct WindowsWerkzeuge;

// #[cfg(target_os = "macos")]
// #[cfg(target_arch = "aarch64")]
// mod apple_silicon {
//     // ...
// }

#[cfg(target_os = "macos")]
#[cfg(target_arch = "x86_64")]
mod apple_legacy {
    // ...
}

#[cfg(not(feature_a))]
#[cfg(target_os = "windows")]
#[cfg(target_arch = "arm")]
mod windows_arm {
    // ...
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
mod windows_x86_64 {
    // ...
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "aarch64")]
mod windows_on_apple_silicon {}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod apple_silicon {
    // ...
}

#[cfg(target_os = "macos")]
#[cfg_attr(
    target_arch = "aarch64",
    path = "feature_apple_silicon.rs"
)]
#[cfg_attr(
    target_arch = "x86_64",
    path = "feature_apple_legacy.rs"
)]
mod apple_gemeinsam;

pub fn main() {

}

