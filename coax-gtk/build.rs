extern crate gcc;
extern crate pkg_config;

fn main() {
    let mut cfg = gcc::Config::new();
    let     gio = pkg_config::Config::new().probe("gio-2.0").unwrap();
    for p in &gio.include_paths {
        cfg.include(p);
    }
    cfg.file("src/gtk/resources.c").compile("libresources.a")
}
