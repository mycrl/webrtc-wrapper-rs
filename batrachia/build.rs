use std::env;

fn main() {
    let is_debug = env::var("DEBUG").map(|str| str == "true").unwrap_or(true);
    let build_type = if is_debug { "Debug" } else { "Release" };

    println!(
        "cargo:rustc-link-search=all=batrachiatc/third_party/webrtc/src/out/\
         {}/obj",
        build_type
    );

    println!("cargo:rustc-link-search=all=batrachiatc/out/{}", build_type);
    println!("cargo:rustc-link-lib=static=batrachiatc");
    println!("cargo:rustc-link-lib=static=webrtc");

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=wmcodecdspuuid");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=secur32");
        println!("cargo:rustc-link-lib=msdmo");
        println!("cargo:rustc-link-lib=dmoguids");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }
}
