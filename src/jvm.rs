use std::borrow::Borrow;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::dirs2;
use crate::mac::{self as sys, JAVA_VM_DIR};

pub const VENDOR_KEY: &str = "IMPLEMENTOR";
pub const VERSION_KEY: &str = "JAVA_VERSION";
pub const VARIANT_KEY: &str = "JVM_VARIANT";

#[inline]
pub fn current_metadata() -> io::Result<Metadata> {
    sys::current_metadata()
}

#[inline]
pub fn read_jvms() -> io::Result<Vec<Metadata>> {
    let mut jvms = Vec::new();
    // 读取目录中的所有条目
    let entries = fs::read_dir(JAVA_VM_DIR)?;
    for entry in entries {
        let entry = entry?;
        if entry.file_name() == "Current" {
            continue;
        }
        jvms.push(Metadata::new(entry.path())?);
    }
    Ok(jvms)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Metadata {
    pub(crate) vendor: String,
    pub(crate) version: String,
    pub(crate) variant: String,
    pub(crate) path: String,
}

impl Metadata {
    #[must_use]
    #[inline]
    pub fn new<P: AsRef<Path>>(dir: P) -> io::Result<Self> {
        sys::metadata(dir)
    }

    #[inline]
    #[must_use]
    pub fn version(&self) -> &str {
        self.version.borrow()
    }

    #[inline]
    #[must_use]
    pub fn vendor(&self) -> &str {
        self.vendor.borrow()
    }

    #[inline]
    #[must_use]
    pub fn variant(&self) -> &str {
        self.variant.borrow()
    }

    #[inline]
    #[must_use]
    pub fn is_current(&self) -> io::Result<bool> {
        let md = current_metadata()?;
        Ok(self.vendor() == md.vendor()
            && self.version() == md.version()
            && self.variant() == md.variant())
    }

    #[inline]
    #[must_use]
    pub fn path(&self) -> &String {
        &self.path
    }

    #[inline]
    #[must_use]
    pub fn file_name(&self) -> &str {
        self.path.split("/").nth(4).unwrap()
    }
}

pub struct JvmInfo(Vec<PathBuf>);

#[allow(unused)]
impl JvmInfo {
    pub fn new() -> io::Result<Self> {
        let path = dirs2::jvm_respo().unwrap();
        Ok(Self(
            fs::read_dir(&path)?
                .map(|entry| entry.unwrap().file_name())
                .filter(|entry| !entry.to_str().unwrap().contains("Current"))
                .map(|name| path.join(name))
                .collect(),
        ))
    }

    pub fn remove_at(&self, idx: usize) -> io::Result<&PathBuf> {
        let current = self.0.get(idx).unwrap();
        fs::remove_dir(current)?;
        Ok(current)
    }

    #[inline]
    pub fn as_vec(&self) -> &Vec<PathBuf> {
        &self.0
    }

    pub fn load_at(&self, idx: usize) -> io::Result<&PathBuf> {
        self.unload()?;
        let current = self.0.get(idx).unwrap();
        #[allow(deprecated)]
        fs::soft_link(current, dirs2::jvm_respo().unwrap().join("Current"))?;
        Ok(current)
    }

    pub fn metadata_at(&self, idx: usize) -> io::Result<Metadata> {
        Metadata::new(self.0.get(idx).unwrap())
    }

    pub fn unload(&self) -> io::Result<()> {
        fs::remove_file(Path::new(JAVA_VM_DIR).join("Current"))
    }

    pub fn into_vec(self) -> Vec<PathBuf> {
        self.0
    }
}

impl Deref for JvmInfo {
    type Target = Vec<PathBuf>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_vec()
    }
}
