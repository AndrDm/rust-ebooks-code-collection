

pub fn main() {
    {
        let ist_arm: bool = {
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            { true }
            #[cfg(
                all(
                    not(target_arch = "arm"),
                    not(target_arch = "aarch64")
                )
            )]
            { false }
        };

        println!("Log: wird auf ARM ausgeführt: {ist_arm}");
    }

    let ist_arm: bool = cfg!(
        any(target_arch = "arm", target_arch = "aarch64")
    );
    // ...

    if cfg!(windows) {
        // ...
        println!("Windows");
    }
    else if cfg!(any(
        unix,
        target_os = "linux",
        target_os = "macos")
    ) {
        // ...
        println!("Das ist ein Unix-System");
    }
    else {
        println!("Ein anderes System");
    }
}