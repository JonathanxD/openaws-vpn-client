use crate::dns::DnsResolver;
use crate::{LocalConfig, VpnApp};
use gtk::prelude::*;
use gtk::FileChooserButton;
use std::rc::Rc;

pub struct OnFileChooseHandler {
    pub app: Rc<VpnApp>,
    pub dns: Rc<DnsResolver>,
}

impl OnFileChooseHandler {
    pub fn choose(&self, chooser: &FileChooserButton) {
        let file = chooser.file();

        if let Some(file) = file {
            let path = file.path().unwrap();
            LocalConfig::save_last_file(&path);
            self.app.config.save_config(path);
            self.dns.resolve_addresses();
        }
    }
}
