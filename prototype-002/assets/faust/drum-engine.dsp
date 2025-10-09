import("stdfaust.lib");

map(x, out_min, out_max) = x * (out_max - out_min) + out_min;
map_exp(x, out_min, out_max) = (exp(x * log(out_max - out_min + 1)) - 1) + out_min;

// Input
v0 = hslider("[0] Kick Pitch Decay", 0.2, 0.0, 1.0, 0.01);
v1 = hslider("[1] Snare Amp Decay", 0.2, 0.0, 1.0, 0.01);
v2 = hslider("[2] V3", 0.2, 0.0, 1.0, 0.01);
v3 = hslider("[3] V4", 0.2, 0.0, 1.0, 0.01);
v4 = button("[4] Kick");
v5 = button("[5] Snare");
v6 = button("[6] Gate3");
v7 = button("[7] Gate4");

val = (v0 + v1 + v2 + v3 + v4 + v5 + v6 + v7) * 0.00001;

kick(gate, cv) = ((oscillator * amp_env) + click) 
with {
    pitch       = 35;   // hslider("Pitch [unit:Hz][scale:log]", 35, 20, 100, 0.1);
    pitch_sweep = 80;   // hslider("Pitch Sweep [unit:Hz]", 80, 0, 500, 1);
    amp_decay   = 250;  // hslider("Amp Env Decay [unit:ms][scale:log]", 250, 1, 2000, 1);
    click_level = 0.1;  // hslider("Click Level [style:knob]", 0.2, 0, 1, 0.01);

    pitch_decay = map_exp(v0, 100.0, 500.0); 
    pitch_decay_sec = pitch_decay / 1000.0;
    amp_decay_sec = amp_decay / 1000.0;

    pitch_env = en.ar(0.001, pitch_decay_sec, gate);
    frequency = pitch_env * pitch_sweep + pitch;

    oscillator = os.osc(frequency);
    amp_env = en.ar(0.001, amp_decay_sec, gate);

    click_env = en.ar(0.001, 0.005, gate);
    click = no.noise * click_env * click_level;
};

snare(gate, cv) = (tone_component * tone_level) + (noise_component * noise_level)
with {
    decay_ms = map_exp(cv, 20.0, 450.0);
    decay_sec = decay_ms / 1000.0;
    amp_env = en.ar(0.001, decay_sec, gate);

    tone_freq = 200;
    tone_env = en.ar(0.001, 0.05, gate);
    tone_component = os.osc(tone_freq) * tone_env;

    noise_source = no.noise;
    filtered_noise = noise_source : fi.highpass(1, 2000);
    noise_component = filtered_noise * amp_env;

    tone_level = 0.6;
    noise_level = 0.4;
};

drums = kick(v4, v0) + snare(v5, v1);

process = val + drums <: _,_;