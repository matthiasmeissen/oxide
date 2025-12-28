# Oxide Project Plan

How to build an audiovisual synthesizer in rust.

## Project Vision

To create a standalone, real-time audiovisual instrument built on a Raspberry Pi. 
This device will procedurally generate music and visuals simultaneously. 
It will be controllable via external MIDI controllers and a custom-built physical interface (knobs and buttons) mounted in a 3D-printed enclosure. 
A small integrated screen will provide real-time feedback on parameters.

The project's heart will be the Rust programming language, chosen for its performance, safety, and modern ecosystem for graphics, audio, and hardware interaction. 
The final product will be a polished, creative tool where sound and light are deeply intertwined—a true "synesthesizer."


## Core Technologies & Concepts

This is a multi-disciplinary project. A foundational understanding of these areas is essential.

### Rust Programming
- **The Ownership Model:** Deeply understand ownership, borrowing, and lifetimes. This is non-negotiable for writing correct and safe Rust.
- **Structs, Enums, and Traits:** These are the building blocks of your application's architecture.
- **Concurrency:** Master threading, `Arc<Mutex<T>>` for shared state, and message passing with channels (`std::sync::mpsc` or `crossbeam-channel`). This is critical for running the audio and graphics loops independently without conflicts.
- **Error Handling:** Get comfortable with `Result<T, E>` and the `?` operator.
- **Cargo and the Crate Ecosystem:** Know how to find, evaluate, and use external libraries (crates).

### Graphics Programming Concepts
- **The Render Pipeline:** Understand the basic flow from vertices to pixels on the screen.
- **Shaders (GLSL):** Learn the basics of the GLSL shading language.
- **Vertex Shaders:** What they do (positioning vertices). 
- **Fragment (or Pixel) Shaders:** What they do (coloring pixels).
- **Uniforms:** How to pass data (like time, color, mouse position) from your Rust code to the shader.
- **Core `wgpu` Concepts:** Understand the main components: Instance, Adapter, Device, Queue, and Surface.

### Digital Audio Concepts
- **Basics of Sound:** Sample Rate, Bit Depth, Channels (Mono/Stereo).
- **Synthesis Building Blocks:** Oscillators, Envelopes (ADSR), Filters, LFOs (Low-Frequency Oscillators)
- **FAUST:** Programming language for cross platform audio things

### Hardware & Electronics
- **Raspberry Pi GPIO:** What the pins are and the difference between input and output.
- **Basic Circuits:** How to connect a button with a pull-down or pull-up resistor to prevent a "floating" state.
- **Debouncing:** Understand why physical buttons produce noisy signals and the software techniques to clean them up.
- **Analog to Digital Conversion (ADC):** The Raspberry Pi's GPIO pins are digital (on/off). To read a value from a potentiometer (knob), you need an ADC chip (like the MCP3008) that communicates with the Pi via the SPI protocol.

### Embedded Displays & Graphics
- **Communication Protocols:** Basic understanding of SPI (Serial Peripheral Interface), which is commonly used for small displays.
- **Embedded Graphics Libraries:** Familiarity with the concepts of a framebuffer and drawing primitives (text, lines, shapes) as provided by libraries like `embedded-graphics`.

### 3D CAD & Manufacturing
- **3D CAD Modeling:** Basics of a Computer-Aided Design program. **Tinkercad** is great for beginners; **Fusion 360** is a powerful next step.
- **3D Printing Slicers:** Understanding how software like **Ultimaker Cura** or **PrusaSlicer** prepares a model for printing.


## Hardware & Software Tools Needed

**Hardware:**
- **Raspberry Pi:** A Raspberry Pi 5 with 8GB of RAM.
- **Power & Storage:** A proper USB-C power supply for the Pi 4 and a high-quality 16GB or larger microSD Card.
- **MIDI Controller:** Any USB MIDI keyboard or controller with knobs/faders.
- **Prototyping:** Breadboard & Jumper Wires.
- **Physical Controls:** Standard tactile push-buttons. Linear taper potentiometers (e.g., 10kΩ). Pack of resistors (e.g., 10kΩ for pull-downs).
- **Analog Input:** An **MCP3008** Analog-to-Digital Converter. This is essential for reading potentiometer values.
- **Display:** A small **SPI Display Hat** (e.g., a 1.3" 240x240 color TFT LCD with an ST7789 controller).
- **Enclosure & Assembly:** A spool of **PLA** filament. A set of **M3 screws and nuts** for mounting.
- **Essential Tools:** A tool for measurements of components for the enclosure design.

**Software & Tooling:**
- **OS:** Raspberry Pi OS "Lite" (64-bit) is recommended to save system resources.
- **Rust Toolchain:** Installed via `rustup.rs`.
- **Cross-Compilation Toolchain (Highly Recommended):** Set up your primary computer to cross-compile for the Raspberry Pi. This will save you hours of waiting.
- **Code Editor:** VS Code with the `rust-analyzer` extension.
- **3D CAD Software:** Tinkercad, Fusion 360, Onshape, or FreeCAD.
- **3D Slicer:** Ultimaker Cura or PrusaSlicer.


## A Note on Timing and Effort

This is a significant but rewarding undertaking. 
The timeline is highly dependent on your prior experience outside of Rust (in electronics, CAD, etc.). 
As a Rust expert, you will move faster through the pure software phases.

- **Total Estimated Time:** For someone working on this during weekends and evenings, expect this project to take **2 to 4 months** to get to a polished v1.0 state.
- **Bottlenecks:** The most time-consuming parts will likely be learning CAD for the first time, debugging hardware wiring, and the physical print/assembly time for the enclosure. Be patient and treat each phase as a learning opportunity.


## Phased Development Plan

**Phase 1: The Graphics Foundation ("Hello, Window!")**
- **Objective:** Create a window on the Raspberry Pi and clear it to a solid color using `wgpu`.
- **Key Crates:** `winit`, `wgpu`.
- **Estimated Time:** 1 weekend. (Faster for a Rust expert familiar with async).
- **Success Criteria:** Running your program shows a black window on the main display.

**Phase 2: The Shader Player**
- **Objective:** Load a GLSL fragment shader and display it on a full-screen quad. Introduce a `time` uniform to create animation.
- **Key Crates:** `naga`.
- **Estimated Time:** 1 weekend.
- **Success Criteria:** An animated, full-screen pattern appears, driven by your GLSL code.

**Phase 3: The Synthesizer**
- **Objective:** Generate a continuous audio tone in a separate, non-blocking thread.
- **Key Crates:** `cpal`, `fundsp`.
- **Estimated Time:** 1 weekend. (Concurrency can be tricky, but the goal is simple).
- **Success Criteria:** You hear a constant tone while your visuals are running smoothly.

**Phase 4: MIDI Integration**
- **Objective:** Control a shader uniform in real-time using a knob on a MIDI controller.
- **Key Crates:** `midir`, `crossbeam-channel` or `Arc<Mutex<T>>`.
- **Estimated Time:** 1-2 weekends. (Involves I/O and thread-safe communication).
- **Success Criteria:** Turning a MIDI knob visibly changes a parameter (e.g., color, speed) of your shader.

**Phase 5: GPIO Integration**
- **Objective:** Read input from a physical button and a potentiometer via an ADC.
- **Key Crates:** `rppal`.
- **Estimated Time:** 1-2 weekends. (Involves wiring and hardware debugging).
- **Success Criteria:** Pressing a physical button and turning a knob on your breadboard controls parameters in your application.

**Phase 6: The Dashboard (Parameter Display Screen)**
- **Objective:** Display real-time parameter values on the small SPI screen hat.
- **Key Crates:** `embedded-graphics`, and a driver crate for your screen (e.g., `st7789`).
- **Estimated Time:** 1-2 weekends. (Involves learning the `embedded-graphics` ecosystem and hardware interfacing).
- **Steps:**
    1.  Wire the screen to the Pi's GPIO header according to its pinout diagram.
    2.  In your Rust code, use `rppal` to initialize the SPI interface.
    3.  Instantiate your screen driver with the SPI interface to get a `display` object.
    4.  In the main loop, read values from your shared state (`Arc<Mutex<...>>`).
    5.  Use `embedded-graphics` to format and draw text (e.g., `Text::new(...)`) for those values onto the display object.
- **Success Criteria:** The small screen displays text (e.g., "Filter: 0.75") that updates in real-time as you turn a MIDI or GPIO knob.

**Phase 7: The Grand Unification - Audio-Reactive Core**
- **Objective:** Link all systems together. Controls should affect both audio and visuals, and the audio output should drive visual parameters.
- **Estimated Time:** 2-3 weekends. (This is the most complex software integration part).
- **Steps:**
    1.  Expand your shared state (`Arc<Mutex<AppState>>`) to hold all controllable parameters (note frequency, filter cutoff, colors, animation speed, etc.).
    2.  **Control -> Audio:** In the audio thread, read parameters from the shared state to control the synthesizer. A MIDI key press should set the note frequency; a knob should control the filter.
    3.  **Audio -> Visuals:** In the audio thread, perform simple analysis on the generated audio (e.g., calculate the peak amplitude of the last buffer). Write this analysis data back into the shared state.
    4.  **Audio -> Render:** In the main thread's render loop, read the audio analysis data (e.g., peak amplitude) and use it to drive a shader uniform.
- **Success Criteria:** The instrument feels "alive." Visuals pulse with the music, and controls feel like they are "playing" the entire audiovisual system.

**Phase 8: The Physical Form (Custom Enclosure)**
- **Objective:** Design and 3D print a custom enclosure for all components.
- **Note:** This phase must come *after* all the electronics are working so you can take precise measurements.
- **Estimated Time:** 2-4 weekends. (Highly variable based on your CAD skill and print time).
- **Steps:**
    1.  **Measure:** Use a tool to precisely measure the Pi, screen, and all control components.
    2.  **Model:** Design the enclosure in CAD, including port cutouts, mounting posts for the Pi, and holes for your buttons and knobs on a faceplate. Ensure you add ventilation holes.
    3.  **Print:** Slice the model and print the parts. Be prepared to do a few small test prints to dial in tolerances for a perfect fit.
- **Success Criteria:** You have physical case parts that fit all your electronics snugly and correctly.

**Phase 9: Final Assembly & Polish**
- **Objective:** Assemble the hardware into the enclosure and add final software touches to create a standalone product.
- **Estimated Time:** 1-2 weekends.
- **Steps:**
    1.  **Assembly:** Solder any connections from your breadboard to be permanent. Mount the Pi, screen, buttons, and knobs in the enclosure and carefully connect everything.
    2.  **Software Polish:** Refactor code for clarity. Create a simple configuration system (e.g., a `config.toml` file) so you don't need to recompile to change settings.
    3.  **Boot to Application:** Configure the Raspberry Pi OS to launch your synthesizer application automatically on startup, making it a true appliance.
- **Success Criteria:** You have a single, self-contained box. You can plug it into power and speakers, and it boots directly into your audiovisual instrument, ready to be played.