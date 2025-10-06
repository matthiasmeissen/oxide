import("stdfaust.lib");

osc = os.osc(440.0);
amp = 0.2;
voice = osc * amp;

process = voice, voice;