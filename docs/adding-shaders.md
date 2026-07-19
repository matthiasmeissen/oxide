# Adding New Shaders to AV Synth


## Overview

1. Add new `.glsl` file into /assets/shaders
2. (Optional) Add .bmp file with same name into /assets/shaders

## Details

Use an existing shader as template to see the required uniforms.

Use `aseprite` to draw preview images and store in `/assets/bitmaps/shader-preview-src/filename.aseprite` with an exported `.bmp` in the same directory.

You need to convert the .bmp to a different format and move into the preview directory:
```bash
   ffmpeg -i assets/bitmaps/shader-preview-src/shader-001-src.bmp -pix_fmt bgr24 assets/shaders/shader-001.bmp
```

Verify the format (must be `128 x 64 x 24`):
```bash
   file assets/shaders/shader-001.bmp
```

## How it works

- `ShaderLibrary::scan` ([src/shaders.rs](../src/shaders.rs)) reads `assets/shaders/*.glsl`
  at startup, sorts by name, and pairs each with its sibling `<name>.bmp` if present.
- A shader without a `.bmp` shows the bundled default (`assets/shaders/shader-default.bmp`).
- The library is shared immutably (`Arc`) with the coordinator (UI wraparound uses
  `shaders.len()`), the graphics thread, and the display/preview code. There is no
  `NUM_SHADERS` constant — the count is the number of `.glsl` files found.
- Adding/removing shaders is picked up on **restart** (no live folder watch). A shader whose
  GLSL fails to compile is logged and the previous shader stays on screen.