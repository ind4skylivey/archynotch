# 🎸 ArchyNotch

**ArchyNotch** is a cyberpunk-inspired, interactive music overlay for Linux. It integrates seamlessly with your desktop, providing a sleek, "always-on-top" notch that displays your current playing track, album art, and playback controls.

![ArchyNotch Banner](assets/banner.png)

## Preview (Wayland / KDE Plasma)

![ArchyNotch Preview](assets/preview_wayland.png)

## ✨ Features

- **Cyberpunk Aesthetics**: Neon colors, glassmorphism, and smooth animations.
- **MPRIS Integration**: Works with Spotify, VLC, Firefox, and any MPRIS-compatible player.
- **Interactive Notch**:
  - **Collapsed Mode**: Minimalist "notch" showing album art and basic info.
  - **Expanded Mode**: Full playback controls (Play/Pause, Next, Prev) and detailed track info.
- **Visualizers**: (Coming Soon) Real-time audio visualization (Spectrum, Waveform).
- **Lightweight**: Built with Rust and Iced for high performance and low resource usage.

## 📦 Installation

### Arch Linux (AUR)

You can install `archynotch` directly using the provided `PKGBUILD`:

```bash
git clone https://github.com/ind4skylivey/archynotch.git
cd archynotch
makepkg -si
```

### From Source (Cargo)

If you have Rust installed:

```bash
cargo install --path .
```

## 🚀 Usage

Simply run the application from your terminal or application launcher:

```bash
archynotch
```

The notch will appear at the top of your screen.

- **Click** the notch to expand/collapse it.
- **Hover** to see glow effects.
- **Controls** appear when expanded.

## ⚙️ Configuration

Configuration is loaded from `~/.config/archynotch/config.toml` (created on first run).

```toml
[window]
width = 800
height = 600
```

## 📜 License

This project is licensed under the **GPL-3.0** License.

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
