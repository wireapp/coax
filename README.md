-----

:warning: *THIS PROJECT IS NOT OFFICIALLY SUPPORTED! ALL CODE IN HERE IS
IN DEVELOPMENT AND LIKELY TO CHANGE WITHOUT WARNING. USE AT YOUR OWN
RISK.* :warning:

-----


# Coax

*A (barely working) native Wire client for Unix*

In here you find the source code of Coax, a native client for Wire. The
repository consists of a single Cargo workspace which contains several
libraries which may or may not be useful individually. Together they
form a client with a GTK+ UI for Unix-like operating systems.


## Motivation

This client is meant to be hackable with a small and (hopefully) easy to
understand code base written in a compiled language (Rust). It may be
useful for testing purposes. It can not replace any of the existing Wire
clients. Its feature set is currently severely limited and basically just
allows for sending and receiving text messages.


## Features

Not much is supported right now. In particular this client is currently
**NOT SECURE**. There is no TLS cerificate pinning, nor is it possible
to compare identity keys.

- [x] Receive messages
- [x] Send messages
- [ ] Everything else


## Outline

This workspace consists of the following libaries:

**[coax-data](https://github.com/wireapp/coax/tree/master/coax-data)**:
Storage support based on SQLite.

**[coax-api](https://github.com/wireapp/coax/tree/master/coax-api)**:
Wire API types with JSON serialisation.

**[coax-api-proto](https://github.com/wireapp/coax/tree/master/coax-api-proto)**:
Protocol buffers API corresponding to Wire's
[generic-message-proto](https://github.com/wireapp/generic-message-proto).

**[coax-client](https://github.com/wireapp/coax/tree/master/coax-client)**:
Client for Wire's HTTP API.

**[coax-actor](https://github.com/wireapp/coax/tree/master/coax-actor)**:
Higher level client which also utilises local storage and asynchronous
notifications over websockets.

**[coax-gtk](https://github.com/wireapp/coax/tree/master/coax-gtk)**:
A simplistic GUI to test the whole stack interactively.

## Build instructions

Building the whole client depends on recent versions of

- Rust nightly + Cargo
- libsodium
- sqlite
- Gtk+
- OpenSSL

Individual libraries may have less dependencies. To start the build cd
into `coax-gtk` and type `make install`. The resulting executable is
installed into `$HOME/.cargo/bin`. On startup a configuration file
`coax.toml` is written to `$HOME/.config/coax/`.

## License

All code in here is subject to the terms and conditions of the
GNU General Public License version 3.

## Contributing

Contributions to any part of this project are very much welcome. However
it is necessary to sign a contributor agreement first. The actual agreement
can be found here:

  https://github.com/wireapp/wire/raw/master/assets/Wire%20Contributor%20Agreement.pdf

When you submit your first pull request [CLAssistant](https://github.com/CLAassistant)
will present you a link to a submission form. Once the agreement has been submitted
your contribution and any possible future ones will be gladly considered for
inclusion into this project.

