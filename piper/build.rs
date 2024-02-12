fn main() {
    let crate_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let espeak_ng_path = format!(
        "{}/runtime-dependencies/{}/espeak-ng-build/{}",
        crate_path,
        build_target::target_os().unwrap(),
        build_target::target_arch().unwrap()
    );

    println!("cargo:rustc-link-lib=espeak-ng");
    println!("cargo:rustc-link-search=native={}/lib", espeak_ng_path);

    #[cfg(target_os = "linux")]
    let espeak_ng_path = format!("{}/lib/libespeak-ng.so", espeak_ng_path);
    #[cfg(target_os = "macos")]
    let espeak_ng_path = format!("{}/lib/libespeak-ng.dylib", espeak_ng_path);
    #[cfg(target_os = "windows")]
    let espeak_ng_path = format!("{}/bin/espeak-ng.dll", espeak_ng_path);
    fs_extra::copy_items(
        &[
            espeak_ng_path,
        ],
        format!("{}/../../..", std::env::var("OUT_DIR").unwrap()),
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    )
    .unwrap();
}
