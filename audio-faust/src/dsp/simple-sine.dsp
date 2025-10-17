import("stdfaust.lib");

freq = hslider("freq", 440.0, 40, 2000, 1);
osc = os.osc(freq);
amp = 0.2;
voice = osc * amp;

process = voice, voice;