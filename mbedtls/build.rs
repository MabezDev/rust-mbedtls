/* Copyright (c) Fortanix, Inc.
 *
 * Licensed under the GNU General Public License, version 2 <LICENSE-GPL or
 * https://www.gnu.org/licenses/gpl-2.0.html> or the Apache License, Version
 * 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>, at your
 * option. This file may not be copied, modified, or distributed except
 * according to those terms. */

use std::collections::{HashMap, HashSet};
use std::env::{self, VarError};

fn main() {
    // env::vars().for_each(|(a, b)| println!("cargo:warning={}={}",a, b));
    let env_components = env::var("DEP_MBEDTLS_PLATFORM_COMPONENTS").or(env::var("DEP_ESP_IDF_PLATFORM_COMPONENTS")).unwrap();
    let mut sys_platform_components = HashMap::<_, HashSet<_>>::new();
    for mut kv in env_components.split(",").map(|component| component.splitn(2, "=")) {
        let k = kv.next().unwrap();
        let v = kv.next().unwrap();
        sys_platform_components.entry(k).or_insert_with(Default::default).insert(v);
        println!(r#"cargo:rustc-cfg=sys_{}="{}""#, k, v);
    }

    // let mut b = cc::Build::new();
    // let b = match env::var_os("DEP_MBEDTLS_INCLUDE") {
    //     Some(i) => {
    //         b.include(i)
    //     }
    //     None => {
    //         let args = env::var("DEP_ESP_IDF_EMBUILD_C_INCLUDE_ARGS").expect("Missing include variable");

    //         let dirs = args.split_whitespace().filter_map(|s| {
    //             // println!("cargo:warning={}",s);
    //             // for some reason starts_with "-I" did not work...
    //             Some(&s[2..])
    //         });

    //         println!("cargo:warning=num_include={}", dirs.clone().count());
    //         // for d in dirs.clone() {
    //         //     println!("cargo:warning={}",d);
    //         // }
            
    //         b.includes(dirs)
    //     }
    // };

    // // TODO make generic with OUT_DIR
    // b.include("/home/mabez/development/rust/embedded/experiments/esp-mbed/target/xtensa-esp32-espidf/debug/build/esp-idf-sys-e3ca4f3ffff72ed3/out/build/config/");

    // // have to hardcode esp config as definitions are bundled with INCLUDE_ARGS
    // // eg: cargo:warning=DEP_ESP_IDF_EMBUILD_C_INCLUDE_ARGS=-DHAVE_CONFIG_H -DMBEDTLS_CONFIG_FILE="mbedtls/esp_config.h" -DUNITY_INCLUDE_CONFIG_H -DWITH_POSIX "-I/some/file/location"
    // let config_file = format!(r#""{}""#, env::var("DEP_MBEDTLS_CONFIG_H").or::<VarError>(Ok("/home/mabez/development/rust/embedded/experiments/esp-mbed/.embuild/espressif/esp-idf-release/v4.4/components/mbedtls/port/include/mbedtls/esp_config.h".to_owned())).unwrap());
    // b.define("MBEDTLS_CONFIG_FILE",
    //          Some(config_file.as_str()));
    
    // b.file("src/mbedtls_malloc.c");
    // b.file("src/rust_printf.c");
    // if sys_platform_components.get("c_compiler").map_or(false, |comps| comps.contains("freestanding")) {
    //     b.flag("-U_FORTIFY_SOURCE")
    //         .define("_FORTIFY_SOURCE", Some("0"))
    //         .flag("-ffreestanding");
    // }
    // b.compile("librust-mbedtls.a");
    // // Force correct link order for mbedtls_printf
    // println!("cargo:rustc-link-lib=static=mbedtls");
    println!("cargo:rustc-link-lib=static=mbedx509");
    println!("cargo:rustc-link-lib=static=mbedcrypto");
}
