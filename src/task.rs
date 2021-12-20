use crate::cmd::{kill_openvpn, ProcessInfo};
use crate::Log;
use std::sync::Arc;

pub struct OavcTask<T> {
    pub name: String,
    pub handle: tokio::task::JoinHandle<T>,
    pub log: Arc<Log>,
}

pub struct OavcProcessTask<T> {
    pub name: String,
    pub handle: tokio::task::JoinHandle<T>,
    pub log: Arc<Log>,
    pub info: Arc<ProcessInfo>,
}

impl<T> OavcTask<T> {
    pub fn abort(&self, log: bool) {
        self.handle.abort();
        if log {
            self.log
                .append(format!("Stopped '{}'!", self.name).as_str());
        }
    }
}

impl<T> OavcProcessTask<T> {
    pub fn new(
        name: String,
        handle: tokio::task::JoinHandle<T>,
        log: Arc<Log>,
        info: Arc<ProcessInfo>,
    ) -> Self {
        Self {
            name,
            handle,
            log,
            info,
        }
    }

    pub fn abort(&self, log: bool) {
        self.handle.abort();
        {
            let pid = self.info.pid.lock().unwrap();

            if let Some(ref pid) = *pid {
                kill_openvpn(*pid)
            }

            if log {
                self.log
                    .append(format!("Stopped '{}' pid '{:?}'!", self.name, pid).as_str());
            }
        }
    }
}

unsafe impl<T> Send for OavcTask<T> {}
unsafe impl<T> Sync for OavcTask<T> {}
