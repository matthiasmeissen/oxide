# AV Synth

An audiovisual synthesizer built in rust. 

It generates real-time graphics and sound driven by unified parameters and runs on desktop and Raspberry Pi.

## Quick Start

```bash
cargo run --release
```

## Documentation

- [User Manual](docs/user-manual.md) - How to use AV Synth
- [Architecture](docs/architecture.md) - System design and technical overview
- [Adding Shaders](docs/adding-shaders.md) - Guide for creating custom shaders
- [Adding Audio Engines](docs/adding-audio-engines.md) - Guide for creating custom audio engines
- [Release Workflow](docs/release-workflow.md) - Branching, versioning, and changelog conventions
- [Hardware](hardware/) - Custom I2C control surface and enclosure for Raspberry Pi

## Features

- Real-time shader-based visuals
- Faust-generated audio synthesis
- 8 unified parameters control both audio and visuals
- Multiple input methods: keyboard, mouse, MIDI, custom hardware
- Cross-platform with extended Raspberry Pi support (OLED display, I2C hardware)

## Requirements

- Rust toolchain (latest stable)
- Audio output device
- Optional: MIDI controller, Raspberry Pi with custom hardware
