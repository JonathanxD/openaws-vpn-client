use crate::cmd::{connect_ovpn, ProcessInfo};
use crate::config::Pwd;
use crate::state_manager::StateManager;
use crate::task::{OavcProcessTask, OavcTask};
use crate::VpnApp;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::reply::WithStatus;
use warp::{Filter, Rejection};

pub struct SamlServer {}

impl SamlServer {
    pub fn new() -> SamlServer {
        SamlServer {}
    }

    pub fn start_server(&self, app: Rc<VpnApp>) {
        app.log.append("Starting SAML server at 0.0.0.0:35001...");
        let (tx, rx) = std::sync::mpsc::sync_channel::<Saml>(1);

        println!("Starting server");
        let sender = warp::any().map(move || tx.clone());

        let pwd = app.config.pwd.clone();
        let pwd = warp::any().map(move || pwd.clone());
        let runtime = app.runtime.clone();

        let saml = warp::post()
            .and(warp::body::form())
            .and(sender)
            .and(pwd)
            .and_then(move |data: HashMap<String, String>,
                            sender: SyncSender<Saml>,
                            pwd: Arc<Mutex<Option<Pwd>>>| {
                async move {
                    let pwd = pwd.lock().await;
                    let saml = Saml {
                        data: data["SAMLResponse"].clone(),
                        pwd: pwd.deref().as_ref().unwrap().pwd.clone(),
                    };
                    sender.send(saml).unwrap();
                    println!("Got SAML data!");

                    Result::<WithStatus<_>, Rejection>::Ok(warp::reply::with_status(
                        "Got SAMLResponse field, it is now safe to close this window",
                        StatusCode::OK,
                    ))
                }
            });

        let handle = runtime.spawn(warp::serve(saml).run(([0, 0, 0, 0], 35001)));

        let log = app.log.clone();
        let join = OavcTask {
            name: "SAML Server".to_string(),
            handle,
            log,
        };

        app.server.replace(Some(join));
        let log = app.log.clone();
        let addr = app.config.addresses.clone();
        let port = app.config.remote.clone();
        let config = app.config.config.clone();
        let st = app.openvpn_connection.clone();
        let stager = app.state.clone();
        let manager = app.connection_manager.clone();

        std::thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            {
                log.append(format!("SAML Data: {:?}...", &data.data[..6]).as_str());
            }

            let addr = {
                let addr = addr.clone();
                let addr = addr.lock().unwrap();
                addr.as_ref().unwrap()[0].to_string()
            };
            let config = {
                let config = config.clone();
                let config = config.lock().unwrap();
                config.as_ref().unwrap().clone()
            };
            let port = {
                let port = port.clone();
                let port = port.lock().unwrap();
                port.as_ref().unwrap().clone().1
            };

            let info = Arc::new(ProcessInfo::new());

            let handle = {
                let info = info.clone();
                let log = log.clone();
                let manager = manager.clone();
                runtime.clone().spawn(async move {
                    let con = connect_ovpn(log.clone(), config, addr, port, data, info).await;
                    let man = manager.lock().unwrap();
                    man.as_ref().unwrap().try_disconnect();
                    con
                })
            };

            let task =
                OavcProcessTask::new("OpenVPN Connection".to_string(), handle, log.clone(), info);
            {
                let mut st = st.lock().unwrap();
                *st = Some(task);
            }

            let state_manager = stager.clone();
            StateManager::change_state(move || {
                let stager = state_manager.lock().unwrap();
                stager.as_ref().unwrap().set_connected();
            });
        });
    }
}

#[derive(Debug, Clone)]
pub struct Saml {
    pub data: String,
    pub pwd: String,
}

unsafe impl Send for Saml {}
unsafe impl Sync for Saml {}
