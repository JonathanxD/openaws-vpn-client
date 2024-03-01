mod app;
mod cmd;
mod config;
mod consts;
mod dns;
mod handlers;
mod local_config;
mod log;
mod manager;
mod saml_server;
mod state_manager;
mod storage;
mod task;

use crate::app::{State, VpnApp};
use crate::cmd::kill_openvpn;
use crate::consts::*;
use crate::handlers::OnFileChooseHandler;
use crate::local_config::LocalConfig;
use crate::log::Log;
use crate::manager::ConnectionManager;
use crate::saml_server::SamlServer;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, FileChooserButton, FileFilter, Grid, Label,
    ScrolledWindow, TextView,
};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let vpn_app = Rc::new(VpnApp::new());
    let saml_server = SamlServer::new();
    saml_server.start_server(vpn_app.clone());

    let app = Application::builder()
        .application_id("com.github.JonathanxD.OpenAwsVpnClient")
        .build();

    let vpn_app = vpn_app.clone();
    let connection_manager = ConnectionManager::new();
    connection_manager.set_app(vpn_app.clone());
    vpn_app.set_connection_manager(connection_manager);

    let win_container = Arc::new(WinContainer::new());

    {
        let manager = vpn_app.connection_manager.clone();
        let win_container = win_container.clone();
        ctrlc::set_handler(move || {
            let win = win_container.win.lock().unwrap();
            if let Some(ref w) = *win {
                w.close();
            } else {
                let mngr = manager.lock().unwrap();
                if let Some(ref mngr) = *mngr {
                    mngr.force_disconnect();
                }
            }
        })
        .unwrap();
    }

    {
        let vpn_app = vpn_app.clone();
        let win_container = win_container.clone();
        app.connect_activate(move |app| {
            let win = ApplicationWindow::builder()
                .application(app)
                .default_width(320)
                .default_height(260)
                .title("Open AWS VPN Client")
                .decorated(true)
                .app_paintable(false)
                .margin_top(8)
                .margin_bottom(8)
                .margin_start(8)
                .margin_end(8)
                .build();

            let main_grid = build_main_grid(vpn_app.clone());

            win.add(&main_grid.grid);
            win.show_all();
            win_container.win.lock().unwrap().replace(win);

            {
                if let Some(p) = LocalConfig::read_last_pid() {
                    vpn_app
                        .log
                        .append_process(p, "Last OpenVPN session was not closed properly...");
                    vpn_app
                        .log
                        .append_process(p, "Asking to kill it in 5 seconds...");
                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_secs(5));
                        kill_openvpn(p);
                    });
                }
            }
        });

        app.run();
    }

    let manager = vpn_app.connection_manager.lock().unwrap();
    if let Some(manager) = manager.as_ref() {
        manager.force_disconnect();
    }
}

fn build_main_grid(app: Rc<VpnApp>) -> MainGrid {
    let main_grid = build_regular_grid(true);
    let status_grid = build_regular_grid(false);
    let connect_grid = build_regular_grid(false);
    let log_grid = build_regular_grid(false);

    main_grid.add(&status_grid);
    main_grid.attach_next_to(
        &connect_grid,
        Some(&status_grid),
        gtk::PositionType::Bottom,
        1,
        1,
    );
    main_grid.attach_next_to(
        &log_grid,
        Some(&connect_grid),
        gtk::PositionType::Bottom,
        16,
        1,
    );

    let status_label = Label::builder()
        .label("Status:")
        .hexpand(true)
        .margin_bottom(8)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    let status_text = Label::builder()
        .name(CONNECTION_STATUS_NAME)
        .label(DISCONNECTED)
        .hexpand(true)
        .margin_bottom(8)
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .build();

    status_grid.add(&status_label);
    status_grid.add(&status_text);

    let ovpn_chooser = FileChooserButton::builder()
        .filter(&ovpn_filter())
        .hexpand(true)
        .margin_bottom(8)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    let connect_button = Button::builder()
        .hexpand(true)
        .margin_bottom(8)
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .label("Connect")
        .build();

    connect_grid.add(&ovpn_chooser);
    connect_grid.add(&connect_button);

    let scroll_view = ScrolledWindow::builder().build();
    let log_view = TextView::builder()
        .name(LOG_VIEW_NAME)
        .hexpand(true)
        .vexpand(true)
        .editable(false)
        .build();

    log_grid.add(&scroll_view);
    scroll_view.add(&log_view);
    app.log.set_view(log_view);

    {
        let handler = Rc::new(OnFileChooseHandler {
            app: app.clone(),
            dns: app.dns.clone(),
        });

        let c = handler.clone();
        ovpn_chooser.connect_file_set(move |f| c.choose(f));

        if let Some(c) = LocalConfig::read_last_file() {
            ovpn_chooser.set_filename(&c);
            handler.choose(&ovpn_chooser);
        }
    }

    let status_text_rc = Rc::new(status_text);

    {
        let status = status_text_rc.clone();
        let app = app.clone();
        let manager = app.connection_manager.lock().unwrap();
        manager.as_ref().unwrap().set_label(status);
    };

    {
        let app = app.clone();
        connect_button.connect_clicked(move |_| {
            let manager = app.connection_manager.lock().unwrap();
            manager.as_ref().unwrap().change_connect_state();
        });
    }

    app.setup_state_manager(status_text_rc, app.log.clone(), Rc::new(connect_button));

    MainGrid {
        grid: main_grid,
        _status_grid: status_grid,
        _connect_grid: connect_grid,
        _log_grid: log_grid,
        _log_scroll_window: scroll_view,
    }
}

struct MainGrid {
    grid: Grid,
    _status_grid: Grid,
    _connect_grid: Grid,
    _log_grid: Grid,
    _log_scroll_window: ScrolledWindow,
}

struct WinContainer {
    win: Mutex<Option<ApplicationWindow>>,
}

impl WinContainer {
    fn new() -> Self {
        Self {
            win: Mutex::new(None),
        }
    }
}

unsafe impl Send for WinContainer {}
unsafe impl Sync for WinContainer {}

fn ovpn_filter() -> FileFilter {
    let filter = FileFilter::new();
    filter.add_pattern("*.ovpn");
    filter.add_pattern("*.txt");
    filter.add_mime_type("application/txt");
    filter.add_mime_type("application/x-openvpn-profile");
    return filter;
}

fn build_regular_grid(expand: bool) -> Grid {
    return Grid::builder()
        .hexpand(expand)
        .vexpand(expand)
        .hexpand_set(expand)
        .vexpand_set(expand)
        .expand(expand)
        .margin_start(4)
        .margin_end(4)
        .build();
}
