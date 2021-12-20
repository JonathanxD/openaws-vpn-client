use lazy_static::lazy_static;
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs::{create_dir_all, remove_file, File};
use std::io::Read;
use std::io::Write;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

type StdMutex<T> = std::sync::Mutex<T>;
type TokioMutex<T> = tokio::sync::Mutex<T>;

lazy_static! {
    static ref CLEAN_KEYS: HashSet<String> = {
        let mut set = HashSet::new();
        set.insert("remote ".to_string());
        set.insert("remote-random-hostname".to_string());
        set.insert("auth-user-pass".to_string());
        set.insert("auth-federate".to_string());
        set.insert("auth-retry interact".to_string());
        set
    };
}

pub struct Config {
    pub addresses: Arc<StdMutex<Option<Vec<IpAddr>>>>,
    pub remote: Arc<StdMutex<Option<(String, u16)>>>,
    pub config: Arc<StdMutex<Option<PathBuf>>>,
    pub pwd: Arc<TokioMutex<Option<Pwd>>>,
}

pub struct Pwd {
    pub pwd: String,
}

unsafe impl Send for Pwd {}
unsafe impl Sync for Pwd {}

impl Config {
    pub fn new() -> Config {
        Config {
            addresses: Arc::new(StdMutex::new(None)),
            remote: Arc::new(StdMutex::new(None)),
            config: Arc::new(StdMutex::new(None)),
            pwd: Arc::new(TokioMutex::new(None)),
        }
    }

    pub fn save_config<P: AsRef<Path>>(&self, file: P) {
        let path = file.as_ref();
        let content = {
            let mut file = File::open(path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        };

        let full_name = {
            let file_name = path.file_stem().unwrap();
            let extension = path.extension().unwrap();
            let mut str = OsString::new();
            str.push(file_name);
            str.push(OsStr::new("-oavc"));
            str.push(OsStr::new("."));
            str.push(extension);
            PathBuf::from(str)
        };

        let remote = get_remote(&content);
        let new_contents = content
            .lines()
            .filter(|l| !has_key(l.to_string()))
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let file_dir = dirs::data_local_dir()
            .map(|v| v.join("openaws-vpn-client").join(full_name.clone()))
            .unwrap_or(full_name);

        if file_dir.exists() {
            remove_file(file_dir.clone()).unwrap();
        }

        if let Some(parent) = file_dir.parent() {
            create_dir_all(parent).unwrap();
        }

        let mut file = File::create(file_dir.clone()).unwrap();
        write!(file, "{}", new_contents).unwrap();
        println!("Saved at {:?}", &file_dir);
        println!("Remote {:?}", &remote);
        let mut config = self.config.lock().unwrap();
        *config = Some(file_dir);
        let mut re = self.remote.lock().unwrap();
        *re = Some(remote);
    }
}

fn has_key(key: String) -> bool {
    for k in CLEAN_KEYS.iter() {
        if key.starts_with(k) {
            return true;
        }
    }

    return false;
}

fn get_remote(content: &String) -> (String, u16) {
    return content
        .lines()
        .filter(|p| p.starts_with("remote "))
        .map(|p| {
            let addr = (&p["remote ".len()..p.rfind(" ").unwrap()]).to_string();
            let port = (&p[p.rfind(" ").unwrap() + 1..]).parse::<u16>().unwrap();
            (addr, port)
        })
        .next()
        .unwrap();
}
