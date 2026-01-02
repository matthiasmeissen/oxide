# Adding Audio Engines

This guide walks through the complete process of adding a new audio engine (DSP) to the AV Synth.

Audio engines are written in Faust and compiled to Rust using the Faust Web IDE.

---

## Quick Checklist

1. Write Faust DSP code and save to `assets/faust/`
2. Export to Rust using Faust Web IDE
3. Clean up exported Rust file
4. Add module to `src/dsp.rs`
5. Update `src/state.rs` enum
6. Update `src/audio.rs` factory
7. Update `src/coordinator.rs` constant (NUM_DSP)
8. Update audio engine spritesheet bitmap
9. Build and test

**Note**: `screens.rs` automatically uses `NUM_DSP` - no manual update needed!

---

## Step-by-Step Process

### Step 1: Write Faust DSP Code

Create your audio engine in Faust and save the source file.

**Location**: `assets/faust/my-engine.dsp`

**Parameter Requirements**:
- Your Faust code should expose exactly 8 parameters
- Parameters are mapped to AV Synth's unified control system
- Use horizontal sliders for parameters

**Example Faust Template**:
```faust
import("stdfaust.lib");

// Use explicit [index] notation to ensure correct parameter mapping
v0 = hslider("[0] Cv1", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v1 = hslider("[1] Cv2", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v2 = hslider("[2] Cv3", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v3 = hslider("[3] Cv4", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v4 = button("[4] Gate1") : si.smoo;
v5 = button("[5] Gate2") : si.smoo;
v6 = button("[6] Gate3") : si.smoo;
v7 = button("[7] Gate4") : si.smoo;

// Your DSP logic here using the parameters
process = /* ... */;
```

### Step 2: Export from Faust Web IDE

1. Open [Faust Web IDE](https://faustide.grame.fr/)
2. Load or paste your Faust code
3. Click Export
4. Configure export settings:
   - **Platform**: `rust`
   - **Architecture**: `jack`
5. Download the exported `.rs` file

**Why jack architecture?** The Faust IDE doesn't support `cpal` directly. We use `jack` architecture and manually modify the file to work with AV Synth's `cpal`-based audio system.

### Step 3: Clean Up Exported Rust File

The exported file needs significant modifications to integrate with AV Synth.

**Save exported file as**: `src/dsp/my_engine.rs`

#### Required Modifications

**A. Remove Jack Dependencies**

Remove these lines:
```rust
//! Faust JACK architecture file
extern crate jack;
use jack::prelude as j;
use std::io;
```

Replace with:
```rust
use crate::dsp::*;
```

**B. Rename the Struct**

Find the main DSP struct (named `mydsp` by default) and rename it:
```rust
// Before
pub struct mydsp {
    // fields...
}

// After
pub struct MyEngine {
    // fields...
}
```

Update all occurrences:
- `mydsp::new()` → `MyEngine::new()`
- `fn new() -> mydsp` → `fn new() -> MyEngine`
- `impl mydsp` → `impl MyEngine`
- `impl FaustDsp for mydsp` → `impl FaustDsp for MyEngine`

**C. Remove Main Function**

Delete the entire `main()` function at the end of the file (typically lines 400-500).

This includes all the Jack client setup code:
```rust
fn main() {
    // ... delete everything here
}
```

**D. Remove Generic Code Duplication**

The exported file includes trait definitions that are already in `src/dsp.rs`. Remove:

- `pub trait FaustDsp { ... }` (entire trait definition)
- `pub trait Meta { ... }` (entire trait definition)
- `pub trait UI<T> { ... }` (entire trait definition)
- Type aliases if they conflict (keep `type FaustFloat = F32;`)
- `#[derive(Copy, Clone)] pub struct ParamIndex(pub i32);` (already in dsp.rs)
- `pub struct Soundfile<'a,T> { ... }` (already in dsp.rs)

**E. Update Type Definitions**

Ensure these match the parent module:
```rust
type F32 = f32;
type F64 = f64;
pub type FaustFloat = F32;
```

The exported `compute` signature is already correct and doesn't need changes.

### Step 4: Register Module in dsp.rs

Add your new module to `src/dsp.rs`:

```rust
pub mod simple_sine;
pub mod basic_fm;
pub mod drum_engine;
pub mod my_engine;  // Add this line
```

### Step 5: Update State Enum

Add your engine to the `DspType` enum in `src/state.rs`:

**A. Add Enum Variant** (around line 91):
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DspType {
    SimpleSine,
    BasicFm,
    DrumEngine,
    MyEngine,  // Add this
}
```

**B. Update get_index Method** (around line 98):
```rust
impl DspType {
    pub fn get_index(&self) -> usize {
        match self {
            DspType::SimpleSine => 0,
            DspType::BasicFm => 1,
            DspType::DrumEngine => 2,
            DspType::MyEngine => 3,  // Add this
        }
    }
}
```

**C. Update Display Trait** (around line 114):
```rust
impl fmt::Display for DspType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DspType::SimpleSine => write!(f, "Simple Sine"),
            DspType::BasicFm => write!(f, "Basic FM"),
            DspType::DrumEngine => write!(f, "Drum Engine"),
            DspType::MyEngine => write!(f, "My Engine"),  // Add this
        }
    }
}
```

### Step 6: Update Audio Factory

Add your engine to the factory in `src/audio.rs`:

**A. Import Statement** (around line 6):
```rust
use crate::dsp::{
    basic_fm::BasicFm,
    simple_sine::SimpleSine,
    drum_engine::DrumEngine,
    my_engine::MyEngine,  // Add this
};
```

**B. Factory Match** (around line 16):
```rust
fn dsp_factory(dsp_type: DspType, sample_rate: u32) -> Box<dyn FaustDsp<T = f32> + Send> {
    let mut dsp: Box<dyn FaustDsp<T = f32> + Send> = match dsp_type {
        DspType::BasicFm => Box::new(BasicFm::new()),
        DspType::SimpleSine => Box::new(SimpleSine::new()),
        DspType::DrumEngine => Box::new(DrumEngine::new()),
        DspType::MyEngine => Box::new(MyEngine::new()),  // Add this
    };
    dsp.init(sample_rate as i32);
    dsp
}
```

### Step 7: Update Coordinator Constant

Update the audio engine count in `src/coordinator.rs`:

```rust
// Line 8
pub const NUM_DSP: usize = 4;  // Changed from 3
```

This constant is used for UI navigation wraparound when cycling through audio engines.

### Step 8: Update Screens Spritesheet Count

Implemented this part, so not element of guideline.

### Step 9: Update Audio Spritesheet

Similar to shaders, audio engines use a spritesheet for UI previews.

**Current System**:
- File: `assets/bitmaps/audio-001/audio-001.bmp`
- Current dimensions: 384 × 64 pixels (3 engines)
- Structure: Horizontal spritesheet (3 × 128×64)

**Adding 4th Engine**:
1. Expand spritesheet to 512 × 64 pixels (4 × 128×64)
2. Create preview image for your engine (128×64)
3. Arrange horizontally: SimpleSine | BasicFm | DrumEngine | MyEngine
4. Export as BMP, 24-bit BGR format
5. Convert: `ffmpeg -i source.bmp -pix_fmt bgr24 assets/bitmaps/audio-001/audio-001.bmp`

### Step 10: Build and Test

```bash
cargo build --release
cargo run --release
```

**Testing Checklist**:
- Audio engine appears in audio selection menu
- Switching to your engine produces sound
- All 8 parameters control the synthesis correctly
- UI displays correct engine name
- OLED preview shows correct bitmap
- No audio glitches when switching engines

---

## Understanding Faust Parameter Mapping

Use explicit parameter indices in your Faust code to ensure correct mapping.

**Best Practice - Use Explicit Indices**:
```faust
v0 = hslider("[0] Cv1", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v1 = hslider("[1] Cv2", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v2 = hslider("[2] Cv3", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v3 = hslider("[3] Cv4", 0.5, 0.0, 1.0, 0.01) : si.smoo;
v4 = button("[4] Gate1") : si.smoo;
v5 = button("[5] Gate2") : si.smoo;
v6 = button("[6] Gate3") : si.smoo;
v7 = button("[7] Gate4") : si.smoo;
```

The `[0]` syntax explicitly sets the ParamIndex, ensuring:
- `v0` → ParamIndex(0) → controlled by `values[0]`
- `v1` → ParamIndex(1) → controlled by `values[1]`
- etc.

This matches AV Synth's parameter system where:
- Mouse X/Y and MIDI knobs control `values[0-3]` (CV)
- Keyboard 1-4 keys and MIDI buttons control `values[4-7]` (Gates)

---

## Common Issues

**Compilation errors about trait bounds**
- Ensure your struct properly implements the `FaustDsp` trait
- The trait implementation block should be at the bottom of the file

**Audio glitches or silence**
- Check that `get_num_outputs()` returns 2 (stereo)
- Verify parameter indices match (0-7)
- Ensure `init()` is called with correct sample rate
- Check compute function is generating non-zero samples

**Parameters don't respond**
- Verify `set_param()` method maps ParamIndex correctly
- Faust parameter order may be reversed
- Check parameter ranges are 0.0-1.0

**Wrong bitmap shows in UI**
- Verify spritesheet frame count matches NUM_DSP
- Check that `get_index()` returns correct position
- Ensure bitmap dimensions are correct

---

## File Checklist

When adding a new audio engine, you'll modify these files:

- `assets/faust/my-engine.dsp` (new file)
- `src/dsp/my_engine.rs` (new file, cleaned from export)
- `src/dsp.rs` (add module declaration)
- `src/state.rs` (add enum variant, update methods)
- `src/audio.rs` (add import and factory case)
- `src/coordinator.rs` (update NUM_DSP constant)
- `src/screens.rs` (import NUM_DSP, update spritesheet calls)
- `assets/bitmaps/audio-001/audio-001.bmp` (expand spritesheet)
