import("stdfaust.lib");

// Utility Functions
map(x, out_min, out_max) = x * (out_max - out_min) + out_min;

// Input CV
v0 = hslider("[0] Freq", 0.2, 0.0, 1.0, 0.01);
v1 = hslider("[1] Mod", 0.2, 0.0, 1.0, 0.01);
v2 = hslider("[2] Attack", 0.2, 0.0, 1.0, 0.01);
v3 = hslider("[3] Release", 0.2, 0.0, 1.0, 0.01);
v4 = button("[4] Trigger");


// Remap Inputs
freq = map(v0, 40.0, 2000);
mod_value = map(v1, 20.0, 80.0);
attack = map(v2, 0.01, 0.2);
release = map(v3, 0.1, 0.8);

// Synth Engine
mod = os.osc(mod_value);
osc = os.osc(freq * mod);
env = v4 : en.adsr(attack, 0.0, 1.0, release);
vca = 0.2 * env;
voice = osc * vca;

process = voice, voice;