use gtk::glib::idle_add;
use gtk::prelude::*;
use gtk::{ScrolledWindow, TextView};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Log {
    view: Arc<View>,
}

pub struct View {
    view: Arc<Mutex<Option<TextView>>>,
}

unsafe impl Send for Log {}
unsafe impl Sync for Log {}

unsafe impl Send for View {}
unsafe impl Sync for View {}

impl Log {
    pub fn new() -> Log {
        Log {
            view: Arc::new(View {
                view: Arc::new(Mutex::new(None)),
            }),
        }
    }

    pub fn set_view(&self, view: TextView) {
        view.connect_size_allocate(|f, _| {
            let parent = f.parent().unwrap();
            let window = parent.dynamic_cast_ref::<ScrolledWindow>();
            if let Some(window) = window {
                let adjustment = window.vadjustment();
                adjustment.set_value(adjustment.upper() - adjustment.page_size());
            }
        });

        let mut lock = self.view.view.lock().unwrap();
        *lock = Some(view);
    }

    pub fn append<S: AsRef<str>>(&self, text: S) {
        let text = text.as_ref().to_string();
        let view = self.view.clone();

        idle_add(move || {
            let view = view.view.lock().unwrap();
            let text = text.clone();
            view.as_ref().unwrap().buffer().inspect(move |v| {
                let mut end_iter = v.end_iter();
                v.insert(&mut end_iter, format!("{}\n", text.clone()).as_str());
            });

            Continue(false)
        });
    }

    pub fn append_process(&self, pid: u32, text: &str) {
        let text = format!("[{}] {}", pid, text);
        let view = self.view.clone();

        idle_add(move || {
            let view = view.view.lock().unwrap();
            let text = text.clone();
            view.as_ref().unwrap().buffer().inspect(move |v| {
                let mut end_iter = v.end_iter();
                v.insert(&mut end_iter, format!("{}\n", text).as_str());
            });

            Continue(false)
        });
    }
}
