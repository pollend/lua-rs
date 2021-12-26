extern crate cc;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut cfg = cc::Build::new();
    // let target = env::var("TARGET").unwrap();
    cfg.warnings(false);

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lua_dir = PathBuf::from("lua");

    println!("cargo:rerun-if-changed={}", lua_dir.to_string_lossy());
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-F{}",lua_dir.to_string_lossy()))
        .header(lua_dir.join("lua.h").to_string_lossy())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    cfg.include("lua")
    .file(lua_dir.join("lapi.c"))
    .file(lua_dir.join("lbaselib.c"))
    .file(lua_dir.join("lcode.c"))
    .file(lua_dir.join("lcorolib.c"))
    .file(lua_dir.join("lctype.c"))
    .file(lua_dir.join("ldblib.c"))
    .file(lua_dir.join("ldebug.c"))
    .file(lua_dir.join("ldo.c"))
    .file(lua_dir.join("ldump.c"))
    .file(lua_dir.join("lfunc.c"))
    .file(lua_dir.join("lgc.c"))
    .file(lua_dir.join("linit.c"))
    .file(lua_dir.join("liolib.c"))
    .file(lua_dir.join("llex.c"))
    .file(lua_dir.join("lmathlib.c"))
    .file(lua_dir.join("lmem.c"))
    .file(lua_dir.join("loadlib.c"))
    .file(lua_dir.join("lobject.c"))
    .file(lua_dir.join("lopcodes.c"))
    .file(lua_dir.join("loslib.c"))
    .file(lua_dir.join("lparser.c"))
    .file(lua_dir.join("lstate.c"))
    .file(lua_dir.join("lstring.c"))
    .file(lua_dir.join("lstrlib.c"))
    .file(lua_dir.join("ltable.c"))
    .file(lua_dir.join("ltablib.c"))
    .file(lua_dir.join("ltests.c"))
    .file(lua_dir.join("ltm.c"))
    .file(lua_dir.join("lundump.c"))
    .file(lua_dir.join("lutf8lib.c"))
    .file(lua_dir.join("lvm.c"))
    .file(lua_dir.join("lzio.c"))
    .out_dir(dst.join("lib"))
    .compile("liblua.a");

    let src = env::current_dir().unwrap().join("lua");
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("lua.h"), dst.join("include/lua.h")).unwrap();
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst.join("include").display());
}