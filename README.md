# 🎸 ArchyNotch

> **The Ultimate Cyberpunk Music Overlay for Linux**

![ArchyNotch Banner](assets/banner.png)

**ArchyNotch** isn't just a music player—it's a statement. Inspired by the neon-soaked aesthetics of cyberpunk culture, this interactive "notch" sits elegantly on your desktop, providing seamless control over your media without breaking your flow.

Built with **Rust** 🦀 and **Iced** 🧊, it's blazing fast, lightweight, and looks absolutely stunning.

---

## 📸 Preview

### Wayland / KDE Plasma

![ArchyNotch Preview](assets/preview_wayland.png)

---

## ✨ Features

- **🔮 Cyberpunk Aesthetics**: Glassmorphism, neon glows, and smooth animations that breathe life into your desktop.
- **🎵 Universal Control**: Seamlessly integrates with **Spotify**, **VLC**, **Firefox**, and any other MPRIS-compatible player.
- **🖱️ Interactive Notch**:
  - **Collapsed**: A sleek, unobtrusive pill showing album art.
  - **Expanded**: Full playback controls and track details at your fingertips.
- **🖐️ Drag & Drop**: Place it anywhere! Click and drag the body to position it perfectly on your screen.
- **🚀 High Performance**: Native code, minimal resource usage.

---

## 🚀 Getting Started

### 📦 Installation

#### Option 1: Arch Linux (Recommended)

Use the provided `PKGBUILD` for a clean system install:

```bash
git clone https://github.com/ind4skylivey/archynotch.git
cd archynotch
makepkg -si
```

#### Option 2: Manual Install (Universal)

If you have Rust installed, you can build and install manually:

```bash
git clone https://github.com/ind4skylivey/archynotch.git
cd archynotch
cargo build --release
sudo make install
```

---

## 🎮 Controls

| Action                | Interaction                                   |
| :-------------------- | :-------------------------------------------- |
| **Expand / Collapse** | Click the **Album Art / Icon**                |
| **Move Window**       | Click and drag the **Main Body**              |
| **Play / Pause**      | Click the **Play** button (Expanded)          |
| **Next / Prev**       | Click the **<<** or **>>** buttons (Expanded) |

---

## ⚙️ Configuration

On first run, a configuration file is created at `~/.config/archynotch/config.toml`. You can tweak the starting position and size:

```toml
[window]
width = 320   # Default collapsed width
height = 90   # Default collapsed height
x = 800       # Horizontal position (adjust for your monitor)
y = 0         # Vertical position (0 = top of screen)
```

---

## 🔧 Troubleshooting

### "I see a black box around the notch!" (DWM / X11 Users)

If you are using a window manager like **DWM**, **i3**, or **bspwm** that doesn't support transparency by default, you might see a black background.

**Solution:** Install and run a compositor like `picom`.

```bash
sudo pacman -S picom
picom -b
```

ArchyNotch relies on a compositor for its transparency and glow effects.

---

## 🗺️ Roadmap

- [ ] **Real-time Audio Visualizers** (Spectrum, Waveform)
- [ ] **Theming Support** (Customize colors via config)
- [ ] **Vertical Mode** (For side-bars)

---

## 📜 License

This project is proudly open-source under the **GPL-3.0** License.

---

<div align="center">
  <sub>Built with ❤️ by <a href="https://github.com/ind4skylivey">ind4skylivey</a></sub>
</div>
