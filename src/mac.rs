use std::collections::HashMap;
use std::io::{self, Result, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use crate::{
    ini::Ini,
    jvm::{self, Metadata},
};

pub const JAVA_VM_DIR: &str = "/Library/Java/JavaVirtualMachines";

pub fn stdin<F>(prompt: &str) -> std::result::Result<F, <F as FromStr>::Err>
where
    F: FromStr,
    <F as FromStr>::Err: std::fmt::Debug,
{
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.trim().parse::<F>()
}

pub fn current_metadata() -> Result<Metadata> {
    metadata(Path::new(JAVA_VM_DIR).join("Current"))
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    #[inline]
    fn _find_md_file<P: AsRef<Path>>(path: P) -> Result<Metadata> {
        let map = Ini::parse(&path)?.into_hash_map();
        let vm_type = _try_guess_vm_type(&map);

        let vendor = map
            .get(jvm::VENDOR_KEY)
            .unwrap_or(&String::from("<unknown>"))
            .clone();
        let variant = map.get(jvm::VARIANT_KEY).unwrap_or(&vm_type.into()).clone();
        let version = map
            .get(jvm::VERSION_KEY)
            .unwrap_or(&String::from("<unknown>"))
            .clone();

        Ok(Metadata {
            vendor,
            variant,
            version,
            path: path.as_ref().to_str().unwrap().into(),
        })
    }

    #[inline]
    fn _try_guess_vm_type(map: &HashMap<String, String>) -> &str {
        #[inline]
        fn contains_any(map: &HashMap<String, String>, pat: &str) -> bool {
            map.values().any(|val| val.contains(pat))
        }
        if contains_any(map, "graalvm") {
            // Oracle's new VM - GraalVM
            "GraalVM"
        } else if contains_any(map, "azul") {
            // Microsoft Azul Zulu
            "Zulu"
        } else {
            "Hotspot"
        }
    }

    _find_md_file(path.as_ref().join("Contents").join("Home").join("release"))
}

#[allow(dead_code)]
pub fn rmxattr<P: AsRef<Path>>(path: P) -> io::Result<()> {
    Command::new("xattr")
        .args(&[
            "-rd",
            "com.apple.quarantine",
            path.as_ref().to_str().unwrap(),
        ])
        .output()
        .map(|_| ())
}

#[allow(dead_code)]
pub fn mv_all<P, Q>(src: P, dest: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    Command::new("mv")
        .args(&[
            src.as_ref().to_str().unwrap(),
            dest.as_ref().to_str().unwrap(),
        ])
        .output()
        .map(|_| ())
}

// pub fn input(prompt: &str) -> Result<usize, ParseIntError> {
//     let mut buf = String::new();
//     print!("{}", prompt);
//     let _ = io::stdin().read_line(&mut buf).unwrap();
//     buf.parse::<usize>()
// }

macro_rules! warnx {
    ($fmt:expr) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), $fmt)
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), format!($fmt, $($arg)*))
    };
}
