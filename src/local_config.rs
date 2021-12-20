use dirs::config_dir;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::Write;
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};

pub struct LocalConfig {}

impl LocalConfig {
    pub fn read_last_file() -> Option<PathBuf> {
        LocalConfig::config_file("last_ovpn")
            .filter(|f| f.metadata().map(|m| m.len()).unwrap_or(0) > 0)
            .map(|mut f| {
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();
                s.replace("\n", "")
            })
            .filter(|s| s.len() > 0)
            .map(|s| PathBuf::from(s))
            .filter(|p| p.exists())
    }

    pub fn save_last_file<P: AsRef<Path>>(last: P) {
        let p = LocalConfig::config_file("last_ovpn");

        if let Ok(c) = std::fs::canonicalize(last) {
            if let Some(mut p) = p {
                p.set_len(0).unwrap();
                let os_str = c.into_os_string();
                let all = os_str.as_bytes();
                p.write_all(all).unwrap();
            }
        }
    }

    pub fn read_last_pid() -> Option<u32> {
        LocalConfig::config_file("last_ovpn_pid")
            .filter(|f| f.metadata().map(|m| m.len()).unwrap_or(0) > 0)
            .map(|mut f| {
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();
                s.replace("\n", "")
            })
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<u32>())
            .filter(|p| p.is_ok())
            .map(|p| p.unwrap())
    }

    pub fn save_last_pid(last: Option<u32>) {
        let p = LocalConfig::config_file("last_ovpn_pid");

        if let Some(mut p) = p {
            p.set_len(0).unwrap();
            if let Some(last) = last {
                write!(p, "{}", last).unwrap();
            } else {
                write!(p, "").unwrap();
            }
        }
    }

    fn config_file(name: &str) -> Option<File> {
        config_dir()
            .map(|d| d.join("openaws-vpn-client"))
            .map(|d| {
                if !d.exists() {
                    create_dir_all(&d).unwrap();
                }
                d.join(name)
            })
            .map(|d| {
                File::options()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(&d)
                    .unwrap()
            })
    }
}
