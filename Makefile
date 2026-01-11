# Makefile for Archy Notch

APP_NAME = archynotch
VERSION = 0.1.0
PREFIX ?= /usr/local
BIN_DIR = $(PREFIX)/bin
SHARE_DIR = $(PREFIX)/share
DESKTOP_DIR = $(SHARE_DIR)/applications
ICON_DIR = $(SHARE_DIR)/icons/hicolor/512x512/apps

all: build

build:
	cargo build --release

install: build
	@echo "Installing $(APP_NAME)..."
	# Create directories
	install -d $(BIN_DIR)
	install -d $(DESKTOP_DIR)
	install -d $(ICON_DIR)
	
	# Install binary
	install -m 755 target/release/$(APP_NAME) $(BIN_DIR)/$(APP_NAME)
	
	# Install desktop file
	install -m 644 extra/$(APP_NAME).desktop $(DESKTOP_DIR)/$(APP_NAME).desktop
	
	# Install icon (assuming we have one, otherwise skipping)
	if [ -f assets/icon.png ]; then \
		install -m 644 assets/icon.png $(ICON_DIR)/$(APP_NAME).png; \
	fi
	
	@echo "Installation complete!"
	@echo "You may need to restart your shell or window manager."

uninstall:
	@echo "Uninstalling $(APP_NAME)..."
	rm -f $(BIN_DIR)/$(APP_NAME)
	rm -f $(DESKTOP_DIR)/$(APP_NAME).desktop
	rm -f $(ICON_DIR)/$(APP_NAME).png
	@echo "Uninstalled."

clean:
	cargo clean
