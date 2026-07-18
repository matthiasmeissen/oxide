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

---

## Archive

This guide walks through the complete process of adding a new shader to the AV Synth.

---

## TL;DR - Quick Checklist

1. ✅ Add `.glsl` file to `assets/shaders/`
2. ✅ Create (NUM_SHADERS x 128) × 64 BMP spritesheet with shader previews
3. ✅ Update `NUM_SHADERS` constant in [coordinator.rs](../src/coordinator.rs)
4. ✅ Build and run

**Note**: `screens.rs` automatically uses `NUM_SHADERS` - no manual update needed!

---

## Step-by-Step Process

### Step 1: Create the Shader File

1. Create a new `.glsl` file in `assets/shaders/`
2. Use the next number in sequence (e.g., `shader-04.glsl`)
3. Include all required uniforms
4. Test your shader logic

**Example**: Copy an existing shader as a template:
```bash
cp assets/shaders/shader-01.glsl assets/shaders/shader-04.glsl
```

Then edit `shader-04.glsl` with your custom shader code.

---

### Step 2: Create Thumbnail Spritesheet

The UI displays shader previews using a **horizontal spritesheet** system.

#### Current System (as of now)

All shader previews are stored in a **single spritesheet**:
- File: `assets/bitmaps/shader-001/shader-001.bmp`
- Current dimensions: **384 × 64 pixels** (3 frames of 128×64)
- Structure: 3 shader previews arranged horizontally

#### Adding a new Shader Preview

You need to expand the spritesheet to **512 × 64 pixels** (4 frames of 128×64).

**Process**:

1. **Create preview images** in Aseprite (or any image editor):
   - Create 4 separate images showing different states/variations of each shader
   - Dimensions: 128 × 64 pixels each
   - Arrange them horizontally: Shader1 | Shader2 | Shader3 | **Shader4**
   - Total canvas: 512 × 64 pixels

2. **Export as BMP**:
   - Format: Windows BMP
   - Color depth: 24-bit
   - **Critical**: Must be BGR24 pixel format

3. **Convert to correct format**:
   ```bash
   ffmpeg -i source.bmp -pix_fmt bgr24 assets/bitmaps/shader-001/shader-001.bmp
   ```

4. **Verify format**:
   ```bash
   file assets/bitmaps/shader-001/shader-001.bmp
   # Should show: PC bitmap, Windows 3.x format, 512 x 64 x 24
   ```

#### Spritesheet Structure

```
┌─────────┬─────────┬─────────┬─────────┐
│ Shader1 │ Shader2 │ Shader3 │ Shader4 │
│ 128×64  │ 128×64  │ 128×64  │ 128×64  │
└─────────┴─────────┴─────────┴─────────┘
Total: 512 × 64 pixels
```

The system uses `shader_index` to select which 128px slice to display:
- Index 0 → pixels 0-127 (Shader1)
- Index 1 → pixels 128-255 (Shader2)
- Index 2 → pixels 256-383 (Shader3)
- Index 3 → pixels 384-511 (Shader4)

---

### Step 3: Update Code

You only need to update **1 file**:

#### [coordinator.rs](../src/coordinator.rs)

Update the shader count constant:

```rust
// Line 7
pub const NUM_SHADERS: usize = 4;  // Changed from 3
```

**Why this matters**:
- Used for UI navigation wraparound (Next/Prev cycling with modulo arithmetic)
- Automatically imported by `screens.rs` to set spritesheet frame count
- Single source of truth for shader count

**That's it!** The `screens.rs` file imports this constant and uses it automatically:
```rust
// screens.rs:1
use crate::coordinator::NUM_SHADERS;

// screens.rs:101 & 118
comp_spritesheet(..., NUM_SHADERS as i32, ...);
```

---

## Future Improvements

Currently, the system requires manual updates to `NUM_SHADERS` constant. Potential improvements:

1. **Automatic shader counting**: Remove `NUM_SHADERS` constant, use `shaders.len()` from runtime discovery
2. **Individual thumbnail files**: One BMP per shader instead of spritesheet
3. **Dynamic spritesheet**: Auto-generate spritesheet from individual frames
4. **Shader metadata**: JSON file with shader name, description, thumbnail path