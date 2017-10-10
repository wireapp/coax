#![feature(conservative_impl_trait, type_ascription)]

extern crate chrono;
extern crate clap;
extern crate chashmap;
extern crate coax_actor;
extern crate coax_api;
extern crate coax_api_proto;
extern crate coax_client;
extern crate coax_data;
extern crate coax_net;
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
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate mime;
extern crate notify_rust;
extern crate pango;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::error::Error;
use std::sync::Arc;

use coax_actor::config;
use clap::{App, Arg, ArgMatches};
use gio::ApplicationExt;
use slog::{Drain, Logger};

#[macro_use]
mod util;

mod ffi;
mod channel;
mod contact;
mod coax;
mod error;
mod poll;
mod profile;
mod res;
mod signals;

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
            let deco   = slog_term::TermDecorator::new().build();
            let format = slog_term::CompactFormat::new(deco).use_utc_timestamp().build().fuse();
            let drain  = slog_async::Async::new(format).chan_size(4096).build().fuse();
            let logger = Logger::root(Arc::new(drain), o!("context" => "Coax"));
            match coax::Coax::new(&logger, cfg) {
                Ok(c)  => { c.run(&[]); }
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
