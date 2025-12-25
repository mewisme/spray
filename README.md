# Spray

A cute desktop pet animation powered by Bevy, featuring smooth frame-based animations that runs on your desktop.

## Features

- 🎬 Smooth frame-based animation
- 🖱️ Draggable window - click and drag to move anywhere on screen
- 📌 Auto snap to taskbar on startup
- 🪟 Always on top, transparent window
- 💾 Embedded assets - default GIF animation bundled in the executable
- 🎨 **Customizable frames, GIF, FPS, and scale** - use your own animations!
- 🔄 **Hot reload config** - changes apply instantly without restart!
- 🎯 **Scale control** - adjust animation size with `scale_percent` config

## Usage

Simply run the executable. The animation will appear above your taskbar. Click and drag to move it to any position on your screen.

### Customization

You can customize the animation frames and FPS:

1. **Custom GIF**: Place a GIF file in `assets/` folder (default: `evernight.gif`)
2. **Custom Frames**: Create a folder `assets/frames/` and put PNG frames there (named `frame_0001.png`, `frame_0002.png`, etc.)
3. **Custom FPS**: Edit `spray.config.json` (auto-created) to adjust the animation speed
4. **Scale**: Adjust animation size with `scale_percent` (default: 40.0 = 40%)

See [CUSTOM_FRAMES.md](CUSTOM_FRAMES.md) for detailed instructions.

**Default behavior:**
- Priority order: External GIF → External Frames → Embedded GIF (evernight.gif) → Embedded frames
- Default: Embedded GIF (evernight.gif) with config: fps=12, size=200x250, scale=40%
- All settings (FPS, window size, scale) are configurable via `spray.config.json`

## Installation

### From Source

Requires Rust toolchain (1.70+):

```bash
git clone https://github.com/mewisme/spray.git
cd spray
cargo build --release
```

The compiled executable will be in `target/release/Spray.exe`

### From Release

Download the latest `Spray.exe` from [Releases](https://github.com/mewisme/spray/releases) and run it.

## Requirements

- Windows 10/11 (tested)
- Display with taskbar support for auto-snap feature

## Project Structure

```
spray/
├── assets/
│   ├── evernight.gif         # Default embedded GIF animation
│   ├── frames/               # Optional: Custom PNG frames
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
