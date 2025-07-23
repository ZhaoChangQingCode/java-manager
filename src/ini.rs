use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub struct Ini(HashMap<String, String>);

impl Ini {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Ini> {
        #[inline]
        fn _parse<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
            let mut ret: HashMap<String, String> = HashMap::new();
            for line in fs::read_to_string(path)?.split("\n") {
                let mut iter = line.split("=");

                if let Some(key) = iter.next() {
                    if let Some(val) = iter.next() {
                        ret.insert(key.trim_matches('"').into(), val.trim_matches('"').into());
                    }
                }
            }
            Ok(ret)
        }
        _parse(path).map(|md| Ini(md))
    }

    #[inline]
    #[must_use]
    pub fn into_hash_map(self) -> HashMap<String, String> {
        self.0
    }
}
