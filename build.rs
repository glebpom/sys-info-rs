extern crate cc;

use std::env;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_os = target.split('-').nth(2).unwrap();

    let c_path = Path::new("c");

    let mut builder = cc::Build::new();
    let c_file= match target_os {
        "linux" => "linux.c",
        "darwin" => "macos.c",
        "windows" => {
            println!("cargo:rustc-flags=-l psapi");
            "windows.c"
        },
        _ => panic!("Unsupported system")
    };
    builder.file(c_path.join(c_file).to_str().unwrap());
    builder.compile("info");
}
