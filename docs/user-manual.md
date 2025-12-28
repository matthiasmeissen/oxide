# Oxide User Manual

Oxide is an audiovisual synthesizer that generates real-time graphics and sound driven by the same set of parameters. Control it using keyboard, mouse, MIDI controllers, or custom hardware on Raspberry Pi.

## Quick Start

Run from source:
```bash
cargo run --release
```

## Control Parameters

Oxide uses 8 shared parameters that simultaneously control both visuals and audio:

- **CV 1-4**: Continuous values (0.0 to 1.0) - typically controlled by knobs or mouse position
- **Gate 1-4**: Trigger values (0.0 or 1.0) - typically controlled by buttons or keys

## Input Methods

### Keyboard and Mouse

**Mouse**
- Mouse position maps to CV 1-4 based on quadrant positioning:
  - CV 1: Horizontal position (left to right)
  - CV 2: Vertical position (top to bottom)
  - CV 3: Inverse horizontal (right to left)
  - CV 4: Inverse vertical (bottom to top)

**Keyboard - Parameter Control**
- `1` `2` `3` `4` - Trigger Gate 1-4 (hold for on, release for off)

**Keyboard - Navigation**
- `Left Arrow` / `Right Arrow` - Navigate between menu pages
- `Space` - Enter selection mode or confirm choice
- `Up Arrow` - Switch to next shader (quick access)

**Keyboard - Display**
- `F` - Toggle fullscreen mode
- `O` - Toggle on-screen display preview (shows Raspberry Pi OLED screen)
- `Esc` - Exit application

### MIDI Controllers

Oxide automatically detects and configures supported MIDI devices. Connect your controller before starting the application.

**Supported Devices**
- Novation Launch Control XL
- Teenage Engineering OP-Z
- Synthstrom Deluge

**MIDI Mapping**
- **Continuous Control (CC)** messages map to CV 1-4
- **Note On/Off** messages map to Gate 1-4
- **Navigation controls** (device-specific) for menu navigation

Device-specific CC and note numbers are pre-configured. Refer to the source code for detailed mappings.

### Custom Hardware (Raspberry Pi Only)

When running on Raspberry Pi, Oxide supports custom I2C hardware:

**Hardware Inputs**
- 4 potentiometers - Control CV 1-4
- 4 buttons - Control Gate 1-4
- Rotary encoder - Menu navigation (rotate for prev/next, push to enter)

**Display Output**
- 128x64 OLED display (SH1106) shows current state and menu screens

The hardware interface uses I2C bus 1 at address 0x08.

## User Interface

The interface consists of three main screens, navigable using Left/Right arrow keys or MIDI/hardware controls.

### Home Screen

Displays current system state:
- CV 1-4 values with visual indicators
- Gate 1-4 status indicators
- Current frame rate (FPS)
- Active shader index
- Active audio engine

### Shader Screen

Shows the currently selected visual shader. Press Space to enter selection mode.

**Selection Mode**
- Navigate through available shaders using Left/Right
- Preview each shader before confirming
- Press Space to confirm and return to shader screen

**Quick Access**: Press Up Arrow from any screen to cycle directly to the next shader without entering the menu.

### Audio Screen

Shows the currently selected audio engine. Press Space to enter selection mode.

**Available Engines**
- Simple Sine - Basic sine wave oscillator
- Basic FM - Frequency modulation synthesizer
- Drum Engine - Percussive sound generator

**Selection Mode**
- Navigate through audio engines using Left/Right
- Press Space to confirm and return to audio screen

## Navigation Flow

```
Home <-> Shader <-> Audio
         |           |
         v           v
    Shader Select   Audio Select
```

- Use Left/Right arrows to move between Home, Shader, and Audio screens
- Press Space on Shader or Audio screens to enter selection mode
- Press Space again in selection mode to confirm your choice
- Selection mode automatically returns to the main screen

## Tips

**Performance**: Run with `--release` flag for optimal frame rate and audio quality.

**Fullscreen**: Press `F` to hide window decorations and use the full screen for visuals.

**OLED Preview**: Press `O` to see what the Raspberry Pi OLED display shows, useful for debugging or understanding the interface when working on desktop.

**Parameter Exploration**: Each shader and audio engine responds differently to the 8 parameters. Experiment with different combinations to discover unique audiovisual textures.

**MIDI Setup**: If your MIDI controller is not automatically detected, it will prompt for manual selection. The application must be restarted to detect newly connected MIDI devices.

## Technical Specifications

**Visual Output**
- OpenGL-based shader rendering
- GLSL ES 100 fragment shaders
- Real-time parameter mapping to shader uniforms

**Audio Output**
- Sample-accurate synthesis
- Faust-generated DSP engines
- Stereo audio output

**Platform Support**
- Cross-platform: macOS, Linux, Windows (desktop features)
- Extended features on Linux/Raspberry Pi (I2C hardware, OLED display)

## Troubleshooting

**No MIDI input detected**
- Ensure MIDI device is connected before starting the application
- Check that the device appears in your system's MIDI device list
- Restart the application after connecting MIDI hardware

**Low frame rate**
- Run with `--release` flag for optimized performance
- Some shaders are more computationally intensive than others
- Close other applications to free system resources

**No audio output**
- Check system audio settings and default output device
- Ensure audio device is not locked by another application
- Try switching audio engines to verify the issue is not shader-specific

**Custom hardware not responding (Raspberry Pi)**
- Verify I2C is enabled: `sudo raspi-config` > Interface Options > I2C
- Check hardware connection on I2C bus 1, address 0x08
- Verify hardware is powered and functioning

## Additional Resources

- [Architecture Documentation](architecture.md) - Technical system overview
- [Adding Shaders Guide](adding-shaders.md) - How to create custom shaders

## Version Information

This manual corresponds to the current version of Oxide. Parameter mappings, keyboard shortcuts, and available features are subject to change in future releases.
