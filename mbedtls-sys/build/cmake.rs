/* Copyright (c) Fortanix, Inc.
 *
 * Licensed under the GNU General Public License, version 2 <LICENSE-GPL or
 * https://www.gnu.org/licenses/gpl-2.0.html> or the Apache License, Version
 * 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>, at your
 * option. This file may not be copied, modified, or distributed except
 * according to those terms. */

#![allow(unused)]

#[cfg(feature = "build")]
use cmake;

use crate::have_feature;

#[cfg(feature = "build")]
impl super::BuildConfig {
    pub fn cmake(&self) {
        let mut cmk = cmake::Config::new(&self.mbedtls_src);
        // cmk.no_default_flags();
        cmk.cflag(format!(
            r#"-DMBEDTLS_CONFIG_FILE="<{}>""#,
            self.config_h.to_str().expect("config.h UTF-8 error")
        ))
        .define("ENABLE_PROGRAMS", "OFF")
        .define("ENABLE_TESTING", "OFF")
        .define("CMAKE_C_COMPILER_WORKS", "1")
        // .define("BUILD_SHARED_LIBS", "OFF")
        // .define("CMAKE_POSITION_INDEPENDENT_CODE", "OFF")
        // .define("CMAKE_EXE_LINKER_FLAGS", "-static")
        .pic(false)
        .build_target("lib");
        if !have_feature("std")
            || ::std::env::var("TARGET")
                .map(|s| (s == "x86_64-unknown-none-gnu") || (s == "x86_64-fortanix-unknown-sgx"))
                == Ok(true)
        {
            // println!("cargo:rustc-link-lib=gcc");
            cmk.cflag("-fno-builtin")
                .cflag("-D_FORTIFY_SOURCE=0")
                .cflag("-fno-stack-protector")
                .cflag("-ffreestanding")
                .cflag("-fno-exceptions"); // dont unwind on no std targets
        }
        let mut dst = cmk.build();
        dst.push("build");
        dst.push("library");
        println!(
            "cargo:rustc-link-search=native={}",
            dst.to_str().expect("link-search UTF-8 error")
        );

        let mut dst = cmk.build();
        dst.push("build");
        dst.push("crypto");
        dst.push("library");
        println!(
            "cargo:rustc-link-search=native={}",
            dst.to_str().expect("link-search UTF-8 error")
        );

        println!("cargo:rustc-link-lib=mbedtls");
        println!("cargo:rustc-link-lib=mbedx509");
        println!("cargo:rustc-link-lib=mbedcrypto");
    }
}
