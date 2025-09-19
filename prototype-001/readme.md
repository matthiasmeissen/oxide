# Oxide 001

First working prototype.

It has a coordinator thread that handles the sending an receiving of data from and to other threads.

Elements:
- Coordinator (Handles distribution of data)
- Audio Thread (Create audio with cpal and fundsp)
- Window Thread (Creates a window and runs the shader using miniquad)
- Midi Thread (Sets up midi connection and sends data)