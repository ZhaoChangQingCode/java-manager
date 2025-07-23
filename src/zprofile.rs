use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub struct Profile(File);

impl Profile {
    #[must_use]
    pub fn open() -> io::Result<Self> {
        let path = dirs::home_dir().unwrap().join(".zprofile");
        File::open(path).map(|f| Profile(f))
    }

    pub fn write_var(&mut self, name: &str, value: &str) -> io::Result<()> {
        write!(self.0, "EXPORT {}={}", name, value,)
    }
}
