# Oxide

An audiovisual synthesiser built in rust.

## How to use

Run the command `cargo run` to start the program from source.

Input Types:
- Keyboard and Mouse
- Midi
- Custom Board (only on Raspberry PI)

**Keyboard and Mouse**
- `MouseX` `MouseY` Four cvs based on quadrant position
- `1` `2` `3` `4` Four gates
- `Arrow Left` `Arrow Right` Previous or next in menu
- `Space` Enter in menu
- `F` Toggle fullscreen
- `O` Toggle menu preview

### Menu
The menu has three pages.
- Home showing cv and gate values, current fps, shader and audio engine
- Shader overview shows selected shader - Shader select showing preview of shaders and lets select new shader 
- Audio overview shows selected audio - Audio select showing preview of audio and lets select new audio 


## Development Notes

**Previous Exploration**

Early exploration and experiments are preserved in the `archive/exploration` branch.


**Convert Bitmaps**

The exported bmp files from Aesprite have the wrong pixel format. 

Use ffmpeg to convert them: `ffmpeg -i source.bmp -pix_fmt bgr24 target.bmp`

To flip images you can do:
- `ffmpeg -i src.bmp -vf "hflip" flipped_horizontal.bmp`
- `ffmpeg -i src.bmp -vf "vflip" flipped_vertical.bmp`
- `ffmpeg -i src.bmp -vf "hflip,vflip" flipped_both.bmp`


**Autostart Program on Raspberry Pi**

- Make program executable: `chmod +x dev/oxide/target/release/oxide`
- Create autostart folder in .config directory: `mkdir .config/autostart`
- Create myscript.desktop file in autostart directory: `touch myscript.desktop`

```
[Desktop Entry]
Type=Application
Name=My Rust App
Exec=bash -c "cd dev/oxide  && ./target/release/oxide"
```
