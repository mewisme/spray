# Custom Frames Guide

## Overview

Spray supports custom frames and FPS through external folders and config files.

## How It Works

Spray supports 3 animation loading modes, controlled by the `mode` config:

### Auto Mode (Default: `mode: "auto"`)
Automatically finds animation in priority order:
1. **External GIF** - Decode from GIF file at `assets/{gif_path}` (default: `assets/evernight.gif`)
2. **PNG Frames** - Load from folder `assets/{frame_folder}/` (default: `assets/frames/`)
3. **Embedded GIF** - Use `evernight.gif` embedded in the binary
4. **Embedded Frames** - Fallback to 620 PNG frames embedded in the binary

### Frame Mode (`mode: "frame"`)
Load only from PNG frames:
- Looks for frames in `assets/{frame_folder}/`
- If not found → app reports error and stops
- Does not search for GIF or embedded frames

### GIF Mode (`mode: "gif"`)
Load only from GIF file:
- Looks for GIF at `assets/{gif_path}` (or `assets/evernight.gif` if `gif_path` is `null`)
- If not found or decode fails → app reports error and stops
- Does not search for frames or embedded frames

**Note:** All modes follow config settings for FPS and size!

## Customization Methods

There are 2 ways to customize animations:

### Method 1: Using GIF File (Simplest) ⭐

Simply place a GIF file in the `assets/` folder:

```
📁 Folder containing Spray
├── Spray.exe
├── spray.config.json          (auto-created)
└── 📁 assets
    └── evernight.gif          (or any .gif file, set gif_path in config)
```

**Advantages:**
- ✅ Simple, only needs 1 file
- ✅ No manual frame extraction needed
- ✅ Automatically decodes all frames from GIF

**Example:**
1. Download your favorite GIF animation
2. Place it in the `assets/` folder next to `Spray.exe`
3. If the filename is `evernight.gif`, no config needed (default)
4. If using a different filename, set `gif_path` in config (e.g., `"gif_path": "my_animation.gif"`)
5. Run Spray - the app will automatically decode and display the animation!

**Note:**
- Default behavior looks for `assets/evernight.gif` if `gif_path` is not set or is `null`
- FPS from config will override the original GIF FPS. For example, if the GIF has 10 FPS but config sets `fps: 5`, animation will run at 5 FPS.

### Method 2: Using PNG Frames (More Flexible)

Create a folder containing PNG frames (default is `assets/frames/`):

```
📁 Folder containing Spray
├── Spray.exe
├── spray.config.json          (auto-created)
└── 📁 assets
    └── 📁 frames              (can be renamed in config: frame_folder)
        ├── frame_0001.png
        ├── frame_0002.png
        ├── frame_0003.png
        └── ...
```

**Advantages:**
- ✅ Control each frame individually
- ✅ Can edit frames separately
- ✅ Supports more frames (depending on frame_digits)
- ✅ Can place frames in a different folder by setting `frame_folder` in config

**Example custom folder:**
- Set `"frame_folder": "my_animations"` → Looks for frames in `assets/my_animations/`
- Set `"frame_folder": "pets/cat"` → Looks for frames in `assets/pets/cat/`

### Creating Frames from Video (with FFmpeg)

If you have a video and want to convert it to frames, use FFmpeg:

**Install FFmpeg:**
- Download from: https://ffmpeg.org/download.html
- Or use: `winget install FFmpeg` (Windows 11)

**Basic frame creation:**
```bash
ffmpeg -i video.mp4 -vf "fps=5,scale=128:128" frames/frame_%04d.png
```

**Explanation:**
- `-i video.mp4`: Input video file
- `fps=5`: Extract 5 frames per second
- `scale=128:128`: Resize to 128x128 pixels
- `frame_%04d.png`: Output filename (frame_0001.png, frame_0002.png, ...)

**Advanced examples:**
```bash
# High FPS (30 fps), large size (256x256)
ffmpeg -i video.mp4 -vf "fps=30,scale=256:256" frames/frame_%04d.png

# Low FPS (2 fps), maintain aspect ratio, square crop
ffmpeg -i video.mp4 -vf "fps=2,crop=ih:ih,scale=128:128" frames/frame_%04d.png

# Only first 10 seconds of video
ffmpeg -i video.mp4 -t 10 -vf "fps=5,scale=128:128" frames/frame_%04d.png

# Many frames (>9999), use 5 digits
ffmpeg -i video.mp4 -vf "fps=60,scale=128:128" frames/frame_%05d.png

# Convert from GIF animation (if you want to use PNG frames)
ffmpeg -i animation.gif -vf "fps=10,scale=128:128" frames/frame_%04d.png

# Note: If you already have a GIF file, you can use it directly!
# Just place the GIF file in assets/ and rename it to evernight.gif (or set gif_path in config)
# No need to convert to PNG frames anymore!
```

**After creating frames:**
1. Copy the `frames/` folder to `assets/frames/` next to the exe
2. Adjust config to match the fps and size used

### Placing Frames Manually (if creating manually)

- Place PNG files in `assets/frames/`
- Filename format: `frame_0001.png`, `frame_0002.png`, etc.
- Number of frames: Any (automatically counted)
- Recommended size: 128x128 pixels

## Configuration

The `spray.config.json` file will be auto-created on first run:

```json
{
  "fps": 12,
  "auto_startup": false,
  "frame_digits": 4,
  "frame_width": 200.0,
  "frame_height": 250.0,
  "window_title": "Spray",
  "frame_folder": "frames",
  "gif_path": "evernight.gif",
  "mode": "auto",
  "scale_percent": 40.0
}
```

### Mode (Animation Loading Mode)

Controls how the app searches for frames/GIF:

- `mode: "auto"` = Auto-detect (default)
  - Priority: External GIF → PNG frames → Embedded GIF → Embedded frames
  - If external GIF exists → use external GIF
  - If no external GIF but frames exist → use frames
  - If neither exists → use embedded GIF (evernight.gif)
  - Last resort → use embedded frames

- `mode: "frame"` = Load only from PNG frames
  - Only searches for frames in `assets/{frame_folder}/`
  - If not found → app reports error and stops
  - Does not search for GIF or embedded frames

- `mode: "gif"` = Load only from GIF file
  - Only searches for GIF at `assets/{gif_path}` (or `assets/evernight.gif` if `gif_path` is `null`)
  - If not found or decode fails → app reports error and stops
  - Does not search for frames or embedded frames

**Example:**
```json
// Auto-detect (priority: frames > gif > embedded)
"mode": "auto"

// Only use PNG frames, error if not found
"mode": "frame"

// Only use GIF, error if not found
"mode": "gif"
```

### FPS Configuration

You can adjust the `fps` value as desired:
- `fps: 5` = 5 frames per second (smooth, slow)
- `fps: 10` = 10 frames per second
- `fps: 30` = 30 frames per second (very smooth, fast)

### Auto Startup

- `auto_startup: false` = Do not start automatically with Windows
- `auto_startup: true` = Automatically run when computer starts

**Note:** When changing `auto_startup` from `false` to `true`, the app will automatically add itself to Windows Registry. Admin rights are not required for HKEY_CURRENT_USER.

### Frame Digits

Number of digits in frame filenames:
- `frame_digits: 4` = `frame_0001.png`, `frame_0002.png`, ... (default, max 9999 frames)
- `frame_digits: 5` = `frame_00001.png`, `frame_00002.png`, ... (max 99999 frames)
- `frame_digits: 6` = `frame_000001.png`, ... (max 999999 frames)

**Note:** Your frame filenames must match this number of digits!

### Frame Size

Display size of animation window (in pixels):
- `frame_width: 200.0` = Width 200 pixels (default)
- `frame_height: 250.0` = Height 250 pixels (default)

### Scale Percent

Scale factor for the animation sprite (percentage):
- `scale_percent: 40.0` = 40% of original size (default)
- `scale_percent: 100.0` = 100% of original size (full size)
- `scale_percent: 200.0` = 200% of original size (double size)

**Note:** This controls the visual size of the animation sprite, while `frame_width` and `frame_height` control the window size.

You can change to make animation larger/smaller:
- `frame_width: 256.0, frame_height: 256.0` = Double the animation size
- `frame_width: 64.0, frame_height: 64.0` = Half the animation size

**Note:** This size determines the window size as well!

### Window Title

Window display title:
- `window_title: "Spray"` = Display "Spray" (default)
- `window_title: "My Pet"` = Display "My Pet"
- `window_title: "🐱 Neko"` = Can use emoji

**Note:** Title can change immediately when saving config (hot reload)!

### Frame Folder

Folder containing PNG frames (used when `mode` is `"frame"` or `"auto"`):
- `frame_folder: "frames"` = Look for frames in `assets/frames/` (default)
- `frame_folder: "my_animations"` = Look for frames in `assets/my_animations/`
- `frame_folder: "pets/cat"` = Look for frames in `assets/pets/cat/`

**Note:**
- Root folder is always `assets/`
- Only applies when `mode` is `"frame"` or `"auto"`
- If `mode: "frame"` and folder doesn't exist → app reports error
- If `mode: "auto"` and folder doesn't exist → app will try GIF or embedded frames

### GIF Path

Path to GIF file (used when `mode` is `"gif"` or `"auto"`):
- `gif_path: "evernight.gif"` = Look for `assets/evernight.gif` (default)
- `gif_path: null` = Use default `evernight.gif`
- `gif_path: "animation.gif"` = Look for `assets/animation.gif`
- `gif_path: "pets/cat.gif"` = Look for `assets/pets/cat.gif`

**Note:**
- Root folder is always `assets/`
- If `gif_path` is `null` or not set, defaults to `evernight.gif`
- Only applies when `mode` is `"gif"` or `"auto"`
- If `mode: "gif"` and file doesn't exist → app reports error
- If `mode: "auto"` and file doesn't exist → app will try embedded frames

### Hot Reload

**App automatically reloads config when the file changes!**

When you edit and save `spray.config.json`, the app will:
- 🔄 Automatically detect file changes
- ⚡ Apply new config immediately
- 🎬 Update FPS, window size, auto startup instantly

**No need to restart the app!** Just save the config file.

## Running the Program

**If using PNG frames (folder `assets/frames/`):**
1. Spray will automatically detect and count frames
2. Load config from `spray.config.json`
3. Apply FPS from config

Console will display log:
```
✨ Loading 120 custom frames from: "C:\path\to\assets\frames"
⚙️ FPS from config: 10
🎬 Animation: 120 frames @ 10 FPS (100ms per frame)
```

**If using GIF (file in `assets/`):**
1. Spray will automatically detect GIF file
2. Decode all frames from GIF
3. Load config from `spray.config.json`
4. Apply FPS from config (override original GIF FPS)

Console will display log:
```
🎬 Loading GIF animation from: "C:\path\to\assets\animation.gif"
✅ Decoded 60 frames from GIF
⚙️ FPS from config: 5
🎬 Animation: 60 frames @ 5 FPS (200ms per frame)
```

## Workflow: From Video/GIF to Desktop Pet

### Method 1: Using GIF Directly (Simplest) ⭐

**Step 1: Prepare GIF file**
- Have a GIF animation file (e.g., `mypet.gif`)

**Step 2: Place in assets folder**
- Copy GIF file to `assets/` folder next to `Spray.exe`
- Rename to `evernight.gif` (or keep original name and set `gif_path` in config)

**Step 3: Run Spray**
- Run `Spray.exe` - app will automatically decode and display animation!

**Done!** No need to extract frames or do anything else.

### Method 2: Creating Pet from Video (More Flexible)

**Step 1: Prepare video**
- Have a `mypet.mp4` file (e.g., video animation of your favorite character)

**Step 2: Create frames with FFmpeg**
```bash
ffmpeg -i mypet.mp4 -vf "fps=10,scale=128:128" frames/frame_%04d.png
```
→ Creates `frames/` folder with files frame_0001.png, frame_0002.png, ...

**Step 3: Copy to assets folder**
- Copy `frames/` folder to `assets/frames/` next to exe

**Step 4: Configure**
- Edit `spray.config.json` to match fps and size used (e.g., `fps: 10`, `frame_width: 128.0`, `frame_height: 128.0`)

**Step 5: Run Spray**
- Run `Spray.exe` and enjoy your custom desktop pet!

