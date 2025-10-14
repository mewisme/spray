# Arisu

A cute desktop pet animation powered by Bevy, featuring smooth frame-based animations that runs on your desktop.

## Features

- 🎬 Smooth frame-based animation
- 🖱️ Draggable window - click and drag to move anywhere on screen
- 📌 Auto snap to taskbar on startup
- 🪟 Always on top, transparent window
- 💾 Embedded assets - all frames bundled in the executable
- 🎨 **Customizable frames and FPS** - use your own animations!
- 🔄 **Hot reload config** - changes apply instantly without restart!

## Usage

Simply run the executable. The animation will appear above your taskbar. Click and drag to move it to any position on your screen.

### Customization

You can customize the animation frames and FPS:

1. **Custom Frames**: Create a folder `assets/frames/` next to the executable and put your PNG frames there (named `frame_0001.png`, `frame_0002.png`, etc.)
2. **Custom FPS**: Edit `arisu.config.json` (auto-created) to adjust the animation speed

See [CUSTOM_FRAMES.md](CUSTOM_FRAMES.md) for detailed instructions.

**Default behavior:**
- Builtin: 620 frames (embedded), FPS/size from config
- Custom: Auto-detect frame count, all settings from config

**Note:** All config settings (FPS, window size) apply to both builtin and custom modes!

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
│   ├── frames/                # Animation frames (620 PNG files, embedded)
│   └── icon.ico              # Application icon
└── src/
    ├── main.rs               # Entry point
    ├── animation/            # Animation system
    │   ├── mod.rs
    │   └── anim.rs          # Frame animation logic
    ├── config/               # Configuration management
    │   ├── mod.rs
    │   ├── config.rs        # Config load/save
    │   ├── config_watcher.rs # Hot reload watcher
    │   └── config_applier.rs # Apply config changes
    ├── window/               # Window management
    │   ├── mod.rs
    │   ├── drag.rs          # Window dragging
    │   └── state.rs         # Application state
    └── platform/             # Platform-specific (Windows)
        ├── mod.rs
        ├── system.rs        # System integration
        └── taskbar.rs       # Taskbar detection
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
