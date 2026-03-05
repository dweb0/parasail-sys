// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// This software may be modified and distributed under the terms of the MIT license.  See the
// LICENSE file for details.

use std::env;
use std::fs::copy;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=parasail_c/CMakeLists.txt");

    let out_dir = env::var("OUT_DIR").unwrap();
    let c_src_path = Path::new("parasail_c");

    let mut config = cmake::Config::new(&c_src_path);

    // Ensure static lib and bypass cmake error 
    // "Compatibility with CMake < 3.5 has been removed from CMake."
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5");

    let dst = config.build();

    let lib_file = dst.join("lib/libparasail.a");
    let target_file = format!("{}/libparasail.a", out_dir);
    copy(lib_file, target_file)
        .expect("Problem copying library to target directoy.");
    
    // let cargo know that it can find the file in the out directory
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=parasail");
}
