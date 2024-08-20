const MAKEFILE_INNER: &str = "Makefile.inner";

fn main() {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    #[allow(non_snake_case)]
    let _IDE_RUN = std::env::var("__CFBundleIdentifier").is_ok();

    // make stuff
    //
    let st = std::process::Command::new("make")
        .arg("-f").arg(MAKEFILE_INNER)
        .arg("tmp/libsome.a")
        .arg("src/some.rs")
        .output()
        .expect("could not spawn `make`")   // shown if 'make' not found on PATH
        .status;

    assert!(st.success(), "[ERROR]: Running 'make' failed");    // shown if 'make' returns a non-zero

    // Link arguments
    {
        #[allow(unused_mut)]
        let mut link_args: Vec<&str> = vec!(    // 'mut' in case we wish to conditionally add stuff
            "-Tlinkall.x",
            "-Tdefmt.x"     // required by 'defmt'
        );

        link_args.iter().for_each(|s| {
            println!("cargo::rustc-link-arg={}", s);
        });
    }

    println!("cargo:rustc-link-search=tmp");
    println!("cargo:rustc-link-lib=static=some");
}
