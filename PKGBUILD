# Maintainer: Livey <your@email.com>
pkgname=archynotch
pkgver=0.1.0
pkgrel=1
pkgdesc="Interactive cyberpunk music overlay for Linux"
arch=('x86_64')
url="https://github.com/yourusername/archynotch"
license=('GPL3')
depends=('gtk3' 'alsa-lib' 'openssl' 'dbus')
makedepends=('cargo')
source=("file://$(pwd)/") # Local source for now
sha256sums=('SKIP')

prepare() {
    cd "$srcdir"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$srcdir"
    
    # Install Binary
    install -Dm755 target/release/archynotch "$pkgdir/usr/bin/archynotch"
    
    # Install Desktop Entry
    install -Dm644 extra/archynotch.desktop "$pkgdir/usr/share/applications/archynotch.desktop"
    
    # Install Icon (if exists)
    if [ -f assets/icon.png ]; then
        install -Dm644 assets/icon.png "$pkgdir/usr/share/icons/hicolor/512x512/apps/archynotch.png"
    fi
}
