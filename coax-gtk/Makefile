SHELL         := /usr/bin/env bash
OS	          := $(shell uname -s | tr '[:upper:]' '[:lower:]')
CARGO_HOME    ?= $(HOME)/.cargo
XDG_DATA_HOME ?= $(HOME)/.local/share
INSTALL_TYPE  ?= --debug # --debug | --features=prod


all: build

clean:
	cargo clean

build:
	cargo build

build-release:
	cargo build --release

install: install-$(OS)

install-linux:
	cargo install --force $(INSTALL_TYPE) --root=$(CARGO_HOME)
	sed "s%@bindir@%$(CARGO_HOME)/bin%g" src/gtk/coax.desktop > $(XDG_DATA_HOME)/applications/coax.desktop

install-darwin:
	cargo install --force $(INSTALL_TYPE) --root=$(CARGO_HOME)

uninstall: uninstall-$(OS)

uninstall-linux:
	cargo uninstall coax-gtk

uninstall-darwin:
	cargo uninstall coax-gtk

