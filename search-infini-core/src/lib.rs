// #![deny(warnings)]

use std::{
    env::{var, var_os},
    path::PathBuf,
};

pub fn find_infini_rt() -> Option<PathBuf> {
    find_header_lib("infinirt.h", "infinirt")
}

pub fn find_infini_op() -> Option<PathBuf> {
    find_header_lib("infiniop.h", "infiniop")
}

pub fn find_infini_core() -> Option<PathBuf> {
    find_header_lib("infinicore.h", "infinicore")
}

fn find_header_lib(header: &str, lib: &str) -> Option<PathBuf> {
    const HOME: &str = if cfg!(windows) { "HOMEPATH" } else { "HOME" };
    let root = match find_env("INFINI_ROOT") {
        Some(path) => PathBuf::from(path),
        None => var_os(HOME).map(PathBuf::from).unwrap().join(".infini"),
    };

    if !root.join("include").join(header).exists() {
        return None;
    }
    let mut contains = false;
    for res in root.join("lib").read_dir().ok()? {
        if res.ok()?.file_name().to_str()?.contains(lib) {
            contains = true;
            break;
        }
    }
    if contains {
        Some(root)
    } else {
        None
    }
}

fn find_env(key: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={key}");
    var(key).ok()
}