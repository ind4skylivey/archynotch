# Maintainer: ind4skylivey <https://github.com/ind4skylivey>
pkgname=archynotch
pkgver=0.1.0
pkgrel=1
pkgdesc="Interactive cyberpunk music overlay for Linux"
arch=('x86_64')
url="https://github.com/ind4skylivey/archynotch"
license=('GPL3')
depends=('gtk3' 'alsa-lib' 'openssl' 'dbus')
makedepends=('cargo' 'git')
# Use GitHub repository as source
source=("git+https://github.com/ind4skylivey/archynotch.git")
sha256sums=('SKIP')

build() {
    # Enter the directory downloaded by git
    cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo build --release
}

package() {
    cd "$pkgname"

    # Install Binary
    install -Dm755 "target/release/archynotch" "$pkgdir/usr/bin/archynotch"

    # Install Desktop Entry
    install -Dm644 "extra/archynotch.desktop" "$pkgdir/usr/share/applications/archynotch.desktop"

    # Install Icon
    if [ -f "assets/icon.png" ]; then
        install -Dm644 "assets/icon.png" "$pkgdir/usr/share/icons/hicolor/512x512/apps/archynotch.png"
    fi
}
