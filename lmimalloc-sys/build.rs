use cmake::Config;

fn main() {
    let mut dst = Config::new("mimalloc")
        .define("MI_OVERRIDE", "OFF")
        .define("MI_SECURE", "OFF")
        .build_target("mimalloc-static")
        .build();

    dst.push("./build");

    let out_name = "mimalloc";
    #[cfg(debug_assertions)]
    let out_name = "mimalloc-debug";

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static={}", out_name);
}
