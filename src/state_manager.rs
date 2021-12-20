use crate::consts::*;
use crate::log::Log;
use crate::State;
use gtk::glib::idle_add_once;
use gtk::prelude::*;
use gtk::{Button, Label};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct StateManager {
    pub label: Rc<Label>,
    pub log: Arc<Log>,
    pub btn: Rc<Button>,
    pub state: RefCell<State>,
}

unsafe impl Send for StateManager {}
unsafe impl Sync for StateManager {}

impl StateManager {
    pub fn new(label: Rc<Label>, log: Arc<Log>, btn: Rc<Button>) -> StateManager {
        let manager = StateManager {
            label,
            log,
            btn,
            state: RefCell::new(State::Disconnected),
        };
        return manager;
    }

    pub fn change_state<F>(f: F)
    where
        F: Fn() + std::marker::Send + 'static,
    {
        idle_add_once(move || {
            f();
        });
    }
}

impl StateManager {
    pub fn set_connecting(&self) {
        self.state.replace(State::Connecting);
        self.label.set_label(CONNECTING);
        self.btn.set_label(BTN_DISCONNECT);
    }

    pub fn set_disconnected(&self) {
        self.state.replace(State::Disconnected);
        self.label.set_label(DISCONNECTED);
        self.btn.set_label(BTN_CONNECT);
    }

    pub fn set_connected(&self) {
        self.state.replace(State::Connected);
        self.label.set_label(CONNECTED);
        self.btn.set_label(BTN_DISCONNECT);
    }
}
