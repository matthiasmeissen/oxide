import("stdfaust.lib");

freq = hslider("[0] freq", 440.0, 40, 2000, 1);
mod_value = hslider("[1] mod_value", 40, 20, 80, 0.1);
mod = os.osc(mod_value);
osc = os.osc(freq * mod);
amp = 0.2;
voice = osc * amp;

process = voice, voice;