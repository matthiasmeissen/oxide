# Oxide

An audiovisual synthesiser built in rust.

## Previous Exploration
Early exploration and experiments are preserved in the `archive/exploration` branch.

## Instructions

### Convert Bitmaps

The exported bmp files from Aesprite have the wrong pixel format.
Use ffmpeg to convert them.

`ffmpeg -i source.bmp -pix_fmt bgr24 target.bmp`

### Autostart Program on Raspberry Pi

Make program executable: `chmod +x dev/oxide/target/release/oxide`

Create autostart folder in .config directory: `mkdir .config/autostart`

Create myscript.desktop file in autostart directory: `touch myscript.desktop`

```
[Desktop Entry]
Type=Application
Name=My Rust App
Exec=bash -c "cd dev/oxide  && ./target/release/oxide"
```
