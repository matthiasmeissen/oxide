# AV Synth Custom Hardware

Custom I2C control surface for AV Synth audiovisual synthesizer on Raspberry Pi.

## Overview

- 4 potentiometers (CV 1-4)
- 4 push buttons (Gate 1-4)
- 1 rotary encoder with button (navigation)
- ESP32 microcontroller
- SH1106 128x64 OLED display
- I2C communication at address 0x08

## Quick Start

1. Order PCB using gerbers in `pcb/gerbers/`
2. Source components from BOM in `pcb/bom/`
3. Flash firmware from `firmware/esp_inputs/`
4. Connect to Raspberry Pi I2C bus 1

## Files

### PCB
- `pcb/gerbers/` - Manufacturing files for PCB fabrication
- `pcb/bom/` - Bill of materials (component list)
- `pcb/design-files/` - Original design files (EasyEDA)

### Firmware
- `firmware/esp_inputs/` - ESP32 Arduino sketch for I2C control
- `firmware/standalone_device/` - Standalone version with display

## Specifications

**PCB**
- 2-layer board
- I2C communication (address 0x08)

**Power**
- 5V via USB or external supply

**Connections**
- I2C: SDA (GPIO 21), SCL (GPIO 22)
- Display: I2C (address 0x3C)
- Potentiometers: Analog inputs
- Buttons: Digital inputs with debouncing
