# Hướng dẫn Custom Frames

## Tổng quan

Arisu hỗ trợ custom frames và FPS thông qua folder external và file config.

## Cách hoạt động

### Chế độ Builtin (Mặc định)
- Frames: 620 frames đã được embed sẵn trong binary
- FPS, Width, Height: Đọc từ `arisu.config.json` (mặc định: 5 FPS, 128x128)

### Chế độ Custom (Người dùng tự tạo)
- Frames: Load từ folder `assets/frames/`
- FPS, Width, Height, Frame Digits: Đọc từ file `arisu.config.json`

**Lưu ý:** Cả hai chế độ đều tuân theo config về FPS và kích thước!

## Cách Custom

### 1. Tạo cấu trúc thư mục

Tạo folder `assets/frames/` cùng cấp với file `Arisu.exe`:

```
📁 Thư mục chứa Arisu
├── Arisu.exe
├── arisu.config.json          (sẽ được tạo tự động)
└── 📁 assets
    └── 📁 frames
        ├── frame_0001.png
        ├── frame_0002.png
        ├── frame_0003.png
        └── ...
```

### 2. Tạo frames từ video (với FFmpeg)

Nếu bạn có video và muốn chuyển thành frames, dùng FFmpeg:

**Cài đặt FFmpeg:**
- Download từ: https://ffmpeg.org/download.html
- Hoặc dùng: `winget install FFmpeg` (Windows 11)

**Tạo frames cơ bản:**
```bash
ffmpeg -i video.mp4 -vf "fps=5,scale=128:128" frames/frame_%04d.png
```

**Giải thích:**
- `-i video.mp4`: File video đầu vào
- `fps=5`: Lấy 5 frames mỗi giây
- `scale=128:128`: Resize về 128x128 pixels
- `frame_%04d.png`: Tên output (frame_0001.png, frame_0002.png, ...)

**Ví dụ nâng cao:**

```bash
# FPS cao (30 fps), kích thước lớn (256x256)
ffmpeg -i video.mp4 -vf "fps=30,scale=256:256" frames/frame_%04d.png

# FPS thấp (2 fps), giữ nguyên tỷ lệ, crop vuông
ffmpeg -i video.mp4 -vf "fps=2,crop=ih:ih,scale=128:128" frames/frame_%04d.png

# Chỉ lấy 10 giây đầu của video
ffmpeg -i video.mp4 -t 10 -vf "fps=5,scale=128:128" frames/frame_%04d.png

# Nhiều frames (>9999), dùng 5 chữ số
ffmpeg -i video.mp4 -vf "fps=60,scale=128:128" frames/frame_%05d.png

# Convert từ GIF animation
ffmpeg -i animation.gif -vf "fps=10,scale=128:128" frames/frame_%04d.png
```

**Sau khi tạo frames:**
1. Copy folder `frames/` vào `assets/frames/` cùng cấp với exe
2. Chỉnh config phù hợp với fps và kích thước đã dùng

### 3. Đặt frames vào folder (nếu tạo thủ công)

- Đặt các file PNG vào `assets/frames/`
- Format tên file: `frame_0001.png`, `frame_0002.png`, etc.
- Số lượng frames: Tùy ý (sẽ tự động đếm)
- Kích thước khuyến nghị: 128x128 pixels

### 4. Cấu hình

File `arisu.config.json` sẽ được tạo tự động khi chạy lần đầu:

```json
{
  "fps": 5,
  "auto_startup": false,
  "frame_digits": 4,
  "frame_width": 128.0,
  "frame_height": 128.0,
  "window_title": "Arisu"
}
```

#### Cấu hình FPS

Bạn có thể sửa giá trị `fps` theo ý muốn:
- `fps: 5` = 5 khung hình/giây (mượt, chậm)
- `fps: 10` = 10 khung hình/giây
- `fps: 30` = 30 khung hình/giây (rất mượt, nhanh)

#### Auto Startup

- `auto_startup: false` = Không tự khởi động cùng Windows
- `auto_startup: true` = Tự động chạy khi mở máy

**Lưu ý:** Khi thay đổi `auto_startup` từ `false` sang `true`, app sẽ tự động thêm vào Windows Registry. Không cần quyền admin cho HKEY_CURRENT_USER.

#### Frame Digits

Số chữ số trong tên file frame:
- `frame_digits: 4` = `frame_0001.png`, `frame_0002.png`, ... (mặc định, tối đa 9999 frames)
- `frame_digits: 5` = `frame_00001.png`, `frame_00002.png`, ... (tối đa 99999 frames)
- `frame_digits: 6` = `frame_000001.png`, ... (tối đa 999999 frames)

**Lưu ý:** Tên file frames của bạn phải khớp với số chữ số này!

#### Frame Size

Kích thước hiển thị của animation (đơn vị: pixels):
- `frame_width: 128.0` = Chiều rộng 128 pixels (mặc định)
- `frame_height: 128.0` = Chiều cao 128 pixels (mặc định)

Bạn có thể thay đổi để làm animation to/nhỏ hơn:
- `frame_width: 256.0, frame_height: 256.0` = Animation to gấp đôi
- `frame_width: 64.0, frame_height: 64.0` = Animation nhỏ hơn một nửa

**Lưu ý:** Kích thước này quyết định cả kích thước cửa sổ!

#### Window Title

Tên hiển thị của cửa sổ:
- `window_title: "Arisu"` = Hiển thị "Arisu" (mặc định)
- `window_title: "My Pet"` = Hiển thị "My Pet"
- `window_title: "🐱 Neko"` = Có thể dùng emoji

**Lưu ý:** Title có thể thay đổi ngay khi save config (hot reload)!

#### Hot Reload

**App tự động reload config khi file thay đổi!**

Khi bạn edit và save `arisu.config.json`, app sẽ:
- 🔄 Tự động phát hiện file đã thay đổi
- ⚡ Áp dụng config mới ngay lập tức
- 🎬 Update FPS, window size, auto startup ngay

**Không cần restart app!** Chỉ cần save file config.

### 5. Chạy chương trình

Khi có folder `assets/frames/`:
1. Arisu sẽ tự động phát hiện và đếm số frames
2. Load config từ `arisu.config.json`
3. Áp dụng FPS từ config

Console sẽ hiển thị log:
```
✨ Đang load 120 frames từ folder custom: "C:\path\to\assets\frames"
⚙️ FPS từ config: 10
🎬 Animation: 120 frames @ 10 FPS (100ms per frame)
```

## Workflow: Từ Video đến Desktop Pet

### Ví dụ hoàn chỉnh: Tạo pet từ video

**Bước 1: Chuẩn bị video**
- Có file `mypet.mp4` (ví dụ: GIF hoặc video animation của character yêu thích)

**Bước 2: Tạo frames bằng FFmpeg**
```bash
ffmpeg -i mypet.mp4 -vf "fps=10,scale=128:128" frames/frame_%04d.png
```
→ Tạo ra folder `frames/` với các file frame_0001.png, frame_0002.png, ...

**Bước 3: Cấu trúc thư mục**
```
📁 Arisu/
├── Arisu.exe
├── arisu.config.json  (sẽ tự tạo)
└── 📁 assets
    └── 📁 frames
        ├── frame_0001.png
        ├── frame_0002.png
        └── ...
```

**Bước 4: Chạy lần đầu**
- Double-click `Arisu.exe`
- File `arisu.config.json` sẽ được tạo tự động

**Bước 5: Chỉnh config (tùy chọn)**
```json
{
  "fps": 10,
  "auto_startup": false,
  "frame_digits": 4,
  "frame_width": 128.0,
  "frame_height": 128.0,
  "window_title": "My Pet"
}
```

**Bước 6: Chỉnh config bất cứ lúc nào**
- App sẽ **tự động reload** khi bạn save file config
- Không cần restart app!

---

## Ví dụ Chi tiết

### Ví dụ 1: Animation chậm, ít frame
```
assets/frames/
  ├── frame_0001.png
  ├── frame_0002.png
  └── frame_0003.png

arisu.config.json:
{
  "fps": 2,
  "auto_startup": false,
  "frame_digits": 4,
  "frame_width": 128.0,
  "frame_height": 128.0,
  "window_title": "Slow Animation"
}
```
→ 3 frames, 2 FPS = mỗi frame hiển thị 500ms, kích thước 128x128

### Ví dụ 2: Animation nhanh, nhiều frame, auto startup
```
assets/frames/
  ├── frame_0001.png ... frame_0100.png

arisu.config.json:
{
  "fps": 30,
  "auto_startup": true,
  "frame_digits": 4,
  "frame_width": 128.0,
  "frame_height": 128.0,
  "window_title": "Arisu"
}
```
→ 100 frames, 30 FPS = mỗi frame hiển thị ~33ms, tự động chạy khi khởi động Windows

### Ví dụ 3: Animation cực nhiều frame (> 9999)
```
assets/frames/
  ├── frame_00001.png ... frame_15000.png

arisu.config.json:
{
  "fps": 24,
  "auto_startup": false,
  "frame_digits": 5,
  "frame_width": 128.0,
  "frame_height": 128.0,
  "window_title": "Movie Pet"
}
```
→ 15000 frames, 24 FPS, sử dụng 5 chữ số cho tên file

### Ví dụ 4: Animation kích thước lớn với emoji
```
assets/frames/
  ├── frame_0001.png ... frame_0060.png (mỗi file 256x256)

arisu.config.json:
{
  "fps": 12,
  "auto_startup": false,
  "frame_digits": 4,
  "frame_width": 256.0,
  "frame_height": 256.0,
  "window_title": "🐱 Neko Chan"
}
```
→ 60 frames, 12 FPS, hiển thị ở kích thước 256x256 (to gấp đôi mặc định)

## Lưu ý

- **Config luôn được áp dụng:** FPS, width, height từ `arisu.config.json` được dùng cho cả builtin và custom frames
- Nếu không có folder `assets/frames/` → dùng builtin (620 frames embedded)
- File config sẽ tự động tạo nếu chưa có (với giá trị mặc định)
- Frames custom phải có format `frame_XXXX.png` với số chữ số khớp với `frame_digits` trong config
- Số frame được đếm tự động từ số file PNG trong folder
- `frame_digits` chỉ áp dụng cho custom frames (builtin luôn dùng 4 chữ số)
- Mặc định `frame_digits = 4` (hỗ trợ đến 9999 frames)
- Nếu cần nhiều hơn 9999 frames, tăng `frame_digits` lên 5, 6, etc.

