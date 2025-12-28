# Adding New Shaders to Oxide

This guide walks through the complete process of adding a new shader to the Oxide audiovisual synthesizer.

---

## TL;DR - Quick Checklist

1. ✅ Add `.glsl` file to `assets/shaders/`
2. ✅ Create (NUM_SHADERS x 128) × 64 BMP spritesheet with shader previews
3. ✅ Update `NUM_SHADERS` constant in [coordinator.rs](../src/coordinator.rs)
4. ✅ Update spritesheet frame count in [screens.rs](../src/screens.rs)
5. ✅ Build and run

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

You need to update **2 files**:

#### File 1: [coordinator.rs](../src/coordinator.rs)

Update the shader count constant:

```rust
// Line 7
const NUM_SHADERS: usize = 4;  // Changed from 3
```

**Why this matters**: This constant is used for UI navigation wraparound. When you press Next/Prev in the shader selection screen, it uses modulo arithmetic to cycle through shaders.

#### File 2: [screens.rs](../src/screens.rs)

Update the spritesheet frame count in **two places**:

```rust
// Line 97 - screen_shader() function
comp_spritesheet(
    display_buffer,
    Point::new(0, 0),
    state.shader_index,
    4,    // Changed from 3 - number of frames in spritesheet
    128,  // Width of each frame
    64,   // Height
    SHADER
);

// Line 106 - screen_shader_select() function
comp_spritesheet(
    display_buffer,
    Point::new(0, 0),
    state.selected_index,
    4,    // Changed from 3 - number of frames in spritesheet
    128,
    64,
    SHADER
);
```

**Why this matters**: The `comp_spritesheet()` function needs to know how many frames exist to calculate the correct horizontal offset for each shader preview.

---

## Future Improvements

Currently, the system requires manual updates to `NUM_SHADERS` and spritesheet frame counts. Potential improvements:

1. **Automatic shader counting**: Remove `NUM_SHADERS` constant, use `shaders.len()` from runtime discovery
2. **Individual thumbnail files**: One BMP per shader instead of spritesheet
3. **Dynamic spritesheet**: Auto-generate spritesheet from individual frames
4. **Shader metadata**: JSON file with shader name, description, thumbnail path