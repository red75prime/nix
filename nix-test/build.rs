extern crate cc;

use std::env;

pub fn main() {
    let target = env::var("TARGET").unwrap();

    let os = if target.contains("linux") {
        "LINUX"
    } else if target.contains("darwin") {
        "DARWIN"
    } else {
        "UNKNOWN"
    };

    cc::Build::new()
        .file("src/const.c")
        .file("src/sizes.c")
        .define(os, None)
        .compile("libnixtest.a");
}
