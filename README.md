# Arisu

A cute desktop pet animation powered by Bevy, featuring smooth frame-based animations that runs on your desktop.

## Features

- 🎬 Smooth frame-based animation
- 🖱️ Draggable window - click and drag to move anywhere on screen
- 📌 Auto snap to taskbar on startup
- 🪟 Always on top, transparent window
- 💾 Embedded assets - all frames bundled in the executable

## Usage

Simply run the executable. The animation will appear above your taskbar. Click and drag to move it to any position on your screen.

## Installation

### From Source

Requires Rust toolchain (1.70+):

```bash
git clone https://github.com/mewisme/arisu.git
cd arisu
cargo build --release
```

The compiled executable will be in `target/release/Arisu.exe`

### From Release

Download the latest `Arisu.exe` from [Releases](https://github.com/mewisme/arisu/releases) and run it.

## Requirements

- Windows 10/11 (tested)
- Display with taskbar support for auto-snap feature

## Project Structure

```
arisu/
├── assets/
│   └── frames/          # Animation frames (640 PNG files)
├── src/
│   ├── main.rs         # Entry point
│   ├── anim.rs         # Animation system
│   ├── drag.rs         # Window dragging
│   ├── state.rs        # Application state
│   ├── system.rs       # System integration
│   └── taskbar.rs      # Taskbar detection (Windows)
└── res/
    └── icon.ico        # Application icon
```

## Technologies

- [Bevy](https://bevyengine.org/) - Game engine for rendering
- [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets) - Asset embedding
- [Windows API](https://github.com/microsoft/windows-rs) - Windows integration

## License

MIT License - see [LICENSE](LICENSE) file for details

## Author

Mew <mauminh.nguyen@gmail.com>

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
