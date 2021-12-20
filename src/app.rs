use crate::config::Config;
use crate::dns::DnsResolver;
use crate::state_manager::StateManager;
use crate::task::{OavcProcessTask, OavcTask};
use crate::{ConnectionManager, Log};
use gtk::{Button, Label};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

pub struct VpnApp {
    pub log: Arc<Log>,
    pub config: Rc<Config>,
    pub server: RefCell<Option<OavcTask<()>>>,
    pub openvpn: RefCell<Option<OavcTask<()>>>,
    pub openvpn_connection: Arc<Mutex<Option<OavcProcessTask<i32>>>>,
    pub runtime: Arc<Runtime>,
    pub dns: Rc<DnsResolver>,
    pub state: Arc<Mutex<Option<StateManager>>>,
    pub connection_manager: Arc<Mutex<Option<ConnectionManager>>>,
}

impl VpnApp {
    pub fn new() -> VpnApp {
        let log = Arc::new(Log::new());
        let config = Rc::new(Config::new());
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        );
        VpnApp {
            log: log.clone(),
            config: config.clone(),
            server: RefCell::new(None),
            openvpn: RefCell::new(None),
            openvpn_connection: Arc::new(Mutex::new(None)),
            runtime: runtime.clone(),
            dns: Rc::new(DnsResolver::new(config, log, runtime)),
            state: Arc::new(Mutex::new(None)),
            connection_manager: Arc::new(Mutex::new(None)),
        }
    }

    pub fn setup_state_manager(&self, label: Rc<Label>, log: Arc<Log>, btn: Rc<Button>) {
        let mut b = self.state.lock().unwrap();
        *b = Some(StateManager::new(label, log, btn));
    }

    pub fn set_connection_manager(&self, manager: ConnectionManager) {
        let mut current = self.connection_manager.lock().unwrap();
        *current = Some(manager)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    Connecting,
    Connected,
    Disconnected,
}
