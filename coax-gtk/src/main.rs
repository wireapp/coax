#![feature(conservative_impl_trait, type_ascription)]

extern crate chrono;
extern crate clap;
extern crate coax_actor;
extern crate coax_api;
extern crate coax_api_proto;
extern crate coax_client;
extern crate coax_data;
extern crate coax_net;
extern crate cookie;
extern crate cryptobox;
extern crate fnv;
extern crate futures;
extern crate futures_cpupool;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;
extern crate gtk_sys;
extern crate json;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate notify_rust;
extern crate pango_sys;
extern crate proteus;
extern crate quick_error;
extern crate rand;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate url;

use std::error::Error;

use coax_actor::config;
use clap::{App, Arg, ArgMatches};
use gio::ApplicationExt;
use slog::{Logger, DrainExt};

#[macro_use]
mod util;

mod ffi;
mod channel;
mod contact;
mod coax;
mod poll;
mod profile;
mod res;

fn main() {
    let args = App::new("coax")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Configuration file (TOML format)")
             .takes_value(true))
        .get_matches();

    match load_config(&args) {
        Ok(cfg) => {
            let drain  = slog_term::streamer().compact().build().fuse();
            let logger = Logger::root(drain, o!("context" => "Coax"));
            match coax::Coax::new(&logger, cfg) {
                Ok(c)  => { c.run(0, &[]); }
                Err(e) => {
                    println!("error: {}", e);
                    std::process::exit(1)
                }
            }
        }
        Err(e) => {
            println!("error reading config: {}", e);
            std::process::exit(1)
        }
    }
}

fn load_config(a: &ArgMatches) -> Result<config::Main, Box<Error>> {
    if let Some(c) = a.value_of("config") {
        config::Main::load(&c)
    } else {
        config::Main::load_default()
    }
}
