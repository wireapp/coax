SHELL         := /usr/bin/env bash
OS	          := $(shell uname -s | tr '[:upper:]' '[:lower:]')
CARGO_HOME    ?= $(HOME)/.cargo
XDG_DATA_HOME ?= $(HOME)/.local/share
INSTALL_TYPE  ?= --features=prod # --debug | --features=prod


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
	mkdir -p $(XDG_DATA_HOME)/icons/hicolor/scalable/apps
	cp src/gtk/coax.svg $(XDG_DATA_HOME)/icons/hicolor/scalable/apps
	mkdir -p $(XDG_DATA_HOME)/icons/hicolor/48x48/apps
	-rsvg-convert -w 48 -h 48 src/gtk/coax.svg -o $(XDG_DATA_HOME)/icons/hicolor/48x48/apps/coax.png

install-darwin:
	cargo install --force $(INSTALL_TYPE) --root=$(CARGO_HOME)

uninstall: uninstall-$(OS)

uninstall-linux:
	cargo uninstall coax-gtk

uninstall-darwin:
	cargo uninstall coax-gtk

