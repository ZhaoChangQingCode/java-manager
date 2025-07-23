use std::{env, path::PathBuf};

use crate::mac::JAVA_VM_DIR;

#[inline]
#[must_use]
pub fn java_home() -> Option<PathBuf> {
    env::var_os("JAVA_HOME").map(|var| PathBuf::from(var))
}

#[inline]
#[must_use]
pub fn jvm_respo() -> Option<PathBuf> {
    Some(PathBuf::from(JAVA_VM_DIR))
}
