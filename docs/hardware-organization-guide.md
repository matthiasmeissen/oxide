
# Hardware Organization

Directory Structure:

```
hardware/
├── pcb/              (Gerbers, BOM, design files)
├── enclosure/        (STL, STEP, CAD files)
├── firmware/         (ESP32 code)
├── docs/             (Assembly guide, photos)
└── datasheets/       (Component specs)
```

File Naming:
- Include version numbers: av-synth-control-v1.0-gerbers.zip
- Use descriptive names: av-synth-case-top.stl (not top.stl)
- Archive gerbers as single ZIP file

Essential Files:
- Gerber files (manufacturing)
- BOM in CSV/Excel format
- STL files for 3D printing
- Source CAD files (STEP format)
- Assembly guide with photos
- Hardware LICENSE file

Best Practices:
- Use semantic versioning (v1.0, v1.1, v2.0)
- Include both source files and exports
- Document in hardware/README.md
- Add clear assembly instructions
- Provide component sourcing information

The guide includes a complete directory structure example, file naming conventions, and a migration plan from your current assets/board/ structure to a proper hardware/ directory. Sources:
- [OSHWA Best Practices](https://oshwa.org/resources/sharing-best-practices/)
- [Open Hardware Documentation Template](https://github.com/SanliFaez/Open-Hardware-Documentation-Template)
- [PCB Manufacturing Files Guide](https://resources.pcb.cadence.com/blog/2025-pcb-manufacturing-files)
- [Gerber Files Explanation](https://resources.altium.com/p/what-gerber-file-pcb-fabrication-process)



## Details

This guide outlines the recommended structure for organizing custom hardware files in the av-synth repository, following open-source hardware best practices.

## Recommended Directory Structure

```
hardware/
├── README.md                      # Overview and getting started
├── LICENSE                        # Hardware license (e.g., CERN-OHL-S-2.0)
├── pcb/
│   ├── design-files/             # Source files (KiCad, Eagle, Altium, etc.)
│   │   ├── av-synth-control.kicad_pcb
│   │   ├── av-synth-control.kicad_sch
│   │   └── av-synth-control.kicad_pro
│   ├── gerbers/                  # Manufacturing files
│   │   ├── av-synth-control-v1.0-gerbers.zip
│   │   └── README.md             # Gerber contents description
│   ├── bom/
│   │   ├── av-synth-control-v1.0-bom.csv
│   │   ├── av-synth-control-v1.0-bom.xlsx
│   │   └── README.md             # BOM notes and sourcing info
│   └── assembly/
│       ├── av-synth-control-v1.0-cpl.csv    # Component placement list
│       └── assembly-notes.pdf             # Assembly instructions
├── enclosure/
│   ├── design-files/             # Source CAD files
│   │   ├── av-synth-case-top.step
│   │   ├── av-synth-case-bottom.step
│   │   └── av-synth-case.f3d        # Fusion 360 source
│   ├── stl/                      # 3D printable files
│   │   ├── av-synth-case-top.stl
│   │   └── av-synth-case-bottom.stl
│   └── drawings/                 # 2D technical drawings
│       ├── av-synth-case-dimensions.pdf
│       └── av-synth-case-assembly.pdf
├── firmware/
│   ├── esp32-control/
│   │   ├── src/
│   │   ├── platformio.ini
│   │   └── README.md
│   └── flashing-guide.md
├── docs/
│   ├── assembly-guide.md         # Step-by-step assembly
│   ├── testing-guide.md          # How to test assembled board
│   ├── troubleshooting.md        # Common issues
│   └── images/                   # Photos and diagrams
│       ├── assembled-board.jpg
│       ├── wiring-diagram.png
│       └── component-placement.jpg
└── datasheets/                   # Component datasheets
    ├── esp32-s3-datasheet.pdf
    ├── sh1106-oled-datasheet.pdf
    └── README.md                  # Index of datasheets
```

## File Naming Conventions

### Version Control

Include version numbers in manufacturing files:
- `av-synth-control-v1.0-gerbers.zip`
- `av-synth-control-v1.0-bom.csv`
- `av-synth-case-v1.1-top.stl`

Use semantic versioning:
- Major version: Incompatible design changes
- Minor version: Backward-compatible improvements
- Patch version: Bug fixes or documentation updates

### Descriptive Names

Use clear, descriptive names:
- `av-synth-control-pcb` (not `board` or `pcb1`)
- `av-synth-case-top.stl` (not `top.stl`)
- `assembly-guide.md` (not `guide.md`)

### Layer Naming for Gerbers

When generating Gerbers, use standard layer names:
- `Top_Copper.gbr`
- `Bottom_Copper.gbr`
- `Top_Soldermask.gbr`
- `Bottom_Soldermask.gbr`
- `Top_Silkscreen.gbr`
- `Bottom_Silkscreen.gbr`
- `Edge_Cuts.gbr`
- `Drill_PTH.drl` (plated through-holes)
- `Drill_NPTH.drl` (non-plated through-holes)

## Essential Files

### PCB Manufacturing Files

**Gerber Files (Required)**
- All copper layers
- Solder mask layers (top/bottom)
- Silkscreen layers (top/bottom)
- Edge cuts / board outline
- Drill files (PTH and NPTH)

Package as single ZIP archive: `av-synth-control-v1.0-gerbers.zip`

**Bill of Materials (Required)**
- Component designators
- Component values
- Manufacturer part numbers
- Supplier part numbers (Mouser, Digikey, etc.)
- Quantities
- Optional: Unit cost, total cost

Format: CSV or XLSX (Excel)

**Pick and Place File (Optional but recommended)**
- Component positions (X, Y coordinates)
- Rotation angles
- Layer (top/bottom)
- Reference designators

Format: CSV

### Enclosure Files

**3D Printable Files (Required)**
- STL format for slicers
- Proper orientation for printing
- Separate files for each part

**Source CAD Files (Recommended)**
- STEP format (universal)
- Native format (Fusion 360, FreeCAD, etc.)

**2D Drawings (Recommended)**
- Dimensions PDF
- Assembly diagrams
- Mounting hole locations

### Documentation Files

**Assembly Guide (Required)**
- Step-by-step instructions with photos
- Required tools list
- Soldering notes and tips
- Wiring diagrams
- I2C address configuration

**BOM README (Required)**
- Sourcing notes
- Acceptable substitutions
- Where to buy components
- Cost estimates

**Testing Guide (Required)**
- How to verify board functionality
- I2C communication test
- Power consumption checks
- Pin continuity tests

## Hardware README Template

Your `hardware/README.md` should include:

```markdown
# av-synth Custom Hardware

Custom I2C control surface for av-synth audiovisual synthesizer.

## Overview

- 4 potentiometers (10kΩ linear)
- 4 push buttons
- 1 rotary encoder with button
- ESP32-S3 microcontroller
- SH1106 128x64 OLED display
- I2C communication at address 0x08

## Quick Start

1. Order PCB using gerbers in `pcb/gerbers/`
2. Source components from BOM in `pcb/bom/`
3. Print enclosure parts from `enclosure/stl/`
4. Follow assembly guide in `docs/assembly-guide.md`
5. Flash firmware from `firmware/esp32-control/`
6. Connect to Raspberry Pi I2C bus 1

## Manufacturing

**PCB Specifications**
- 2-layer board
- 1.6mm thickness
- HASL or ENIG finish
- Minimum trace: 0.2mm
- Minimum clearance: 0.2mm

**Estimated Costs** (as of 2025)
- PCB: $2-5 per board (qty 5)
- Components: $15-20 per board
- Enclosure: $2-3 in filament

## Documentation

- [Assembly Guide](docs/assembly-guide.md)
- [Testing Guide](docs/testing-guide.md)
- [Troubleshooting](docs/troubleshooting.md)

## License

Hardware licensed under CERN-OHL-S-2.0
```

## License Recommendations

For open-source hardware, consider:

**CERN Open Hardware License v2**
- CERN-OHL-S-2.0 (Strongly Reciprocal) - Similar to GPL
- CERN-OHL-W-2.0 (Weakly Reciprocal) - Similar to LGPL
- CERN-OHL-P-2.0 (Permissive) - Similar to MIT/Apache

**Creative Commons**
- CC BY-SA 4.0 (Attribution-ShareAlike) - Common for hardware

Include LICENSE file in `hardware/` directory and reference in all documentation.

## Best Practices

**Design Files**
- Always include editable source files, not just exports
- Use open or widely-available tools when possible
- Document which software version was used

**Gerber Archives**
- Include README.txt inside ZIP with layer descriptions
- Test gerbers with online viewer before publishing
- Include board revision in filename

**BOM Management**
- Use manufacturer part numbers, not just generic descriptions
- Include multiple supplier options when possible
- Note any hard-to-source or obsolete parts
- Specify acceptable substitutions

**Version Control**
- Tag releases that match hardware versions
- Keep design files in version control
- Don't commit large binary files directly (use Git LFS if needed)
- Maintain changelog for hardware revisions

**Documentation**
- Include photos of assembled hardware
- Provide clear assembly instructions
- Document known issues or limitations
- Include dimensional drawings for mechanical integration

## Migration from Current Structure

Your current structure:
```
assets/board/
├── esp_inputs/
│   └── esp_inputs.ino
└── standalone_device/
    └── standalone_device.ino
```

Suggested migration:
1. Create `hardware/` directory at repository root
2. Move firmware to `hardware/firmware/`
3. Add PCB files to `hardware/pcb/`
4. Add enclosure files to `hardware/enclosure/`
5. Create comprehensive documentation
6. Add hardware README and LICENSE
7. Update main README to reference hardware documentation

## References

- [OSHWA Best Practices](https://oshwa.org/resources/sharing-best-practices/)
- [Open Hardware Documentation Template](https://github.com/SanliFaez/Open-Hardware-Documentation-Template)
- [PCB Manufacturing Files Guide](https://resources.pcb.cadence.com/blog/2025-pcb-manufacturing-files)
- [Gerber Files Explanation](https://resources.altium.com/p/what-gerber-file-pcb-fabrication-process)
