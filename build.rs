extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_arch = target.split('-').nth(0).unwrap();
    let target_os = target.split('-').nth(2).unwrap();

    let mut builder = cc::Build::new();
    match target_os {
        "linux" => builder.file("c/linux.c"),
        "darwin" => builder.file("c/macos.c"),
        "windows" => {
            println!("cargo:rustc-flags=-l psapi");
            builder.file("c/windows.c");
            match target_arch {
                "i686" => builder.file("c/windows_i686.c"),
                "x86_64" => builder.file("c/windows_x86_64.c"),
                _ => panic!("Unknown windows arch"),
            }
        }
        _ => panic!("Unsupported system")
    };
    builder.compile("info");
}
