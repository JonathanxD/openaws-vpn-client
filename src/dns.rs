use crate::config::Config;
use crate::Log;
use domain::base::iana::Class;
use domain::base::{Dname, Rtype};
use domain::rdata::A;
use rand::prelude::*;
use std::net::IpAddr;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct DnsResolver {
    pub config: Rc<Config>,
    pub log: Arc<Log>,
    pub runtime: Arc<Runtime>,
}

impl DnsResolver {
    pub fn new(config: Rc<Config>, log: Arc<Log>, runtime: Arc<Runtime>) -> Self {
        Self {
            config,
            log,
            runtime,
        }
    }

    pub fn resolve_addresses(&self) {
        let random_start = rng_domain();
        let remote = { self.config.remote.lock().unwrap().deref().clone() }
            .map(|d| format!("{}.{}", random_start, d.0));

        self.log
            .append(format!("Looking up into '{}'...", remote.clone().unwrap()).as_str());

        let resolver = domain::resolv::StubResolver::new();
        let d: domain::base::Dname<Vec<u8>> = Dname::from_str(remote.unwrap().as_str()).unwrap();
        let r = self
            .runtime
            .block_on(async { resolver.query((d, Rtype::A, Class::In)).await })
            .unwrap();

        let msg = r.into_message();
        let ans = msg.answer().unwrap().limit_to::<A>();
        let all = ans
            .filter(|v| v.is_ok())
            .map(|v| v.unwrap())
            .map(|v| v.into_data())
            .map(|v| v.addr())
            .map(|v| IpAddr::V4(v))
            .inspect(|v| self.log.append(format!("Resolved '{}'.", v).as_str()))
            .collect::<Vec<_>>();

        let mut br = self.config.addresses.lock().unwrap();
        *br = Some(all);
    }
}

fn rng_domain() -> String {
    let mut rng = thread_rng();
    let mut bts = [0u8; 12];
    rng.fill_bytes(&mut bts);
    hex::encode(bts)
}
