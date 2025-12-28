# Oxide - Architecture Overview

## Core Concept

**Unified Parameter System**: 8 shared parameters (`values[0-7]`) drive both visuals (shaders) and audio (DSP engines) simultaneously, creating a tightly integrated audiovisual experience.

---

## System Architecture

### 1. Thread Structure (6 concurrent threads)

```
Input Threads                Coordinator             Output Threads
─────────────               ─────────────            ──────────────
MIDI Thread ───┐                                  ┌─→ Graphics Thread
I2C Thread ────┼─→ Messages → Coordinator ──Triple─┼─→ Audio Thread
Keyboard/Mouse─┘             (State Hub)   Buffers └─→ Display Thread
```

### 2. Core Components

**State Management** ([coordinator.rs](src/coordinator.rs)):
- Central hub receiving messages from all inputs
- Updates `State` (8 parameters + metadata)
- Distributes via lock-free triple buffers

**Visual System** ([graphics.rs](src/graphics.rs)):
- Renders GLSL shaders fullscreen
- Parameters → shader uniforms (`u_cv1-4`, `u_gate1-4`)
- Hot-swappable shaders from [assets/shaders/](assets/shaders/)

**Audio System** ([audio.rs](src/audio.rs)):
- Faust-generated DSP engines (SimpleSine, BasicFM, DrumEngine)
- Same 8 parameters control synthesis
- Real-time audio with `cpal`

### 3. Input Sources

**Desktop** (all platforms):
- **Mouse**: X/Y → `values[0-3]` (position + inverse)
- **Keyboard**: Keys 1-4 → `values[4-7]` (gates)
- **MIDI**: Auto-detects Launch Control XL, OP-Z, Deluge
  - Custom mappings per device in [coordinator.rs:88-173](src/coordinator.rs#L88-L173)

**Raspberry Pi** (Linux only):
- **Custom I2C Hardware**: ESP32-based control surface
  - 4 potentiometers → `values[0-3]`
  - 4 buttons → `values[4-7]`
  - Rotary encoder → UI navigation
- **OLED Display**: 128x64 SH1106 showing current state

### 4. Data Flow

```
1. Input Source → Message::SetValue(index, value)
2. Coordinator → updates State.values[index]
3. Triple Buffers → broadcast to Graphics/Audio
4. Graphics → maps to shader uniforms
5. Audio → maps to DSP parameters
```

### 5. Key Design Patterns

✓ **Lock-free communication**: Triple buffers prevent blocking
✓ **Hot-swapping**: Change shaders/DSP engines at runtime
✓ **Normalized values**: All parameters 0.0-1.0
✓ **Platform conditional**: Full desktop UI + extended Pi hardware
✓ **Message-passing**: Crossbeam channels serialize state changes

---

## Project Structure

```
oxide/
├── src/
│   ├── main.rs                    # Entry point and thread orchestration
│   ├── state.rs                   # Core data structures and message types
│   ├── coordinator.rs             # Central state management hub
│   ├── graphics.rs                # Visual rendering with miniquad
│   ├── audio.rs                   # Audio output with cpal
│   ├── dsp.rs                     # DSP trait definitions
│   │   ├── simple_sine.rs         # Faust-generated sine oscillator
│   │   ├── basic_fm.rs            # Faust-generated FM synthesizer
│   │   └── drum_engine.rs         # Faust-generated drum engine
│   ├── midi.rs                    # MIDI input handling
│   ├── i2c.rs                     # Custom hardware I2C interface (Linux only)
│   ├── display.rs                 # OLED display output (Linux only)
│   └── screens.rs                 # UI rendering logic
├── assets/
│   ├── shaders/                   # GLSL fragment shaders for visuals
│   ├── bitmaps/                   # UI graphics for OLED display
│   ├── faust/                     # Faust DSP source files
│   └── board/                     # Custom hardware designs
└── Cargo.toml
```

---

## Key Files

- [main.rs](src/main.rs) - Entry point, spawns all threads
- [state.rs](src/state.rs) - Core data structures (`State`, `Message`)
- [coordinator.rs](src/coordinator.rs) - State management + MIDI mappings
- [graphics.rs](src/graphics.rs) - Shader rendering + keyboard/mouse
- [audio.rs](src/audio.rs) - Audio synthesis loop
- [dsp/](src/dsp/) - Faust-generated DSP engines
- [midi.rs](src/midi.rs) - MIDI input handling
- [i2c.rs](src/i2c.rs) - Custom hardware (Pi only)
- [display.rs](src/display.rs) - OLED output (Pi only)
- [screens.rs](src/screens.rs) - UI screen rendering

---

## Shared State Structure

```rust
pub struct State {
    pub time: f64,              // Global time counter
    pub resolution: [f32; 2],   // Window resolution
    pub values: [f32; 8],       // 8 shared parameters (0.0-1.0)
    pub shader_index: usize,    // Currently active shader
    pub fps: f32,               // Frames per second
    pub dsp_type: DspType,      // Currently active audio engine
}
```

**Parameter Semantics:**
- `values[0-3]`: Continuous values (CVs) - typically knobs/sliders/mouse position
- `values[4-7]`: Gate/trigger values - typically buttons/keys

---

## Communication Patterns

**Triple Buffers** (coordinator → consumers):
- Lock-free, wait-free reads
- Always provides latest state
- Graphics and audio threads read at their own rates

**Message Channel** (inputs → coordinator):
- Bounded channel (size 5)
- Back-pressure for overwhelming input
- Serializes all state mutations
- Single receiver ensures consistency

---

## Platform Support

**Cross-Platform** (Desktop + Raspberry Pi):
- Graphics rendering (miniquad)
- Audio synthesis (cpal)
- MIDI input (midir)
- Keyboard/mouse input

**Linux-Only** (Raspberry Pi):
- OLED Display (SH1106 128x64)
- Custom I2C Hardware (ESP32-based control surface)

---

## Key Dependencies

- `miniquad` - Cross-platform graphics/window management
- `cpal` - Cross-platform audio I/O
- `midir` - MIDI input
- `triple_buffer` - Lock-free triple buffering
- `crossbeam-channel` - Message passing
- `embedded-graphics`, `sh1106` - OLED display (Linux)
- `linux-embedded-hal` - I2C hardware (Linux)

---

## Execution Flow

### Initialization (main.rs)

1. Create message channel for input → coordinator
2. Create triple buffers for coordinator → consumers
3. Spawn coordinator thread
4. Spawn MIDI thread
5. Spawn audio thread
6. Spawn I2C + display threads (Linux only)
7. Start graphics thread (blocking, runs event loop)

### Runtime Loop

**Coordinator Thread:**
```
loop {
    receive message from any input
    update app_state or ui_state
    publish changes via triple buffers
}
```

**Graphics Thread:**
```
update() {
    check for shader changes
    update OLED preview
    calculate FPS
}

draw() {
    read current state
    send time update
    render shader with uniforms
    draw OLED overlay
}
```

**Audio Thread:**
```
audio_callback() {
    read current state
    hot-swap DSP if changed
    update DSP parameters
    generate audio samples
}
```

**Input Threads:**
```
loop {
    read input device
    parse input
    send Message to coordinator
}
```

---

## Design Philosophy

- **Single source of truth**: The coordinator manages all state
- **Lock-free data flow**: Triple buffers for high-performance state distribution
- **Unified parameter model**: Same 8 values drive both visuals and audio
- **Hot-swappable engines**: Change shaders and DSP algorithms at runtime
- **Multi-input support**: Keyboard, mouse, MIDI, and custom hardware
- **Cross-platform**: Runs on desktop with full features, extends on Raspberry Pi with hardware I/O
- **Real-time performance**: Lock-free communication, optimized rendering
