import("stdfaust.lib");

map(x, out_min, out_max) = x * (out_max - out_min) + out_min;
map_exp(x, out_min, out_max) = out_min * exp(x * log(out_max / out_min));

v0 = hslider("[0] Kick Decay", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v1 = hslider("[1] Snare Decay", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v2 = hslider("[2] Hihat Decay", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v3 = hslider("[3] Clap Decay", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v4 = button("[4] Kick");
v5 = button("[5] Snare");
v6 = button("[6] Hihat");
v7 = button("[7] Clap");

shared_noise = no.noise;

kick(gate, cv) = ((oscillator * amp_env) + click) 
with {
    pitch       = 35;
    pitch_sweep = 80;
    click_level = 0.1;

    amp_decay   = map_exp(cv, 200.0, 600.0);
    pitch_decay = map_exp(cv, 100.0, 500.0); 
    pitch_decay_sec = pitch_decay / 1000.0;
    amp_decay_sec = amp_decay / 1000.0;

    pitch_env = en.ar(0.001, pitch_decay_sec, gate);
    frequency = pitch_env * pitch_sweep + pitch;

    oscillator = os.osc(frequency);
    amp_env = en.ar(0.001, amp_decay_sec, gate);

    click_env = en.ar(0.001, 0.005, gate);
    click = shared_noise * click_env * click_level;
};

snare(gate, cv) = (tone_component * tone_level) + (noise_component * noise_level)
with {
    decay_ms = map_exp(cv, 20.0, 450.0);
    decay_sec = decay_ms / 1000.0;
    amp_env = en.ar(0.001, decay_sec, gate);

    tone_freq = 180;
    tone_env = en.ar(0.001, 0.05, gate);
    tone_component = (os.osc(tone_freq) + os.osc(tone_freq * 1.5)) * 0.5 * tone_env;

    filtered_noise = shared_noise : fi.highpass(2, 1500) : fi.resonbp(3000, 1.2, 0.7);
    noise_component = filtered_noise * amp_env;

    tone_level = 0.6;
    noise_level = 0.4;
};

hihat(gate, cv) = filtered_noise * amp_env * 0.8
with {
    decay_ms = map_exp(cv, 5.0, 800.0);
    decay_sec = decay_ms / 1000.0;
    filtered_noise = shared_noise : fi.highpass(4, 4500) : fi.resonbp(8000, 2.5, 1.0);
    amp_env = en.ar(0.001, decay_sec, gate);
};

clap(gate, cv) = filtered_noise * env
with {
    center_freq = map_exp(cv, 1500.0, 8000.0);
    decay_ms = map(cv, 60.0, 200.0);
    decay_sec = decay_ms / 1000.0;

    bandwidth = map(cv, 2500.0, 1500.0);
    fl = center_freq - bandwidth / 2.0;
    fu = center_freq + bandwidth / 2.0;
    filtered_noise = shared_noise : fi.bandpass(2, fl, fu) : fi.highpass(2, 800);

    env = en.ar(0.0005, decay_sec, gate) + (en.ar(0.01, decay_sec * 0.4, gate) * 0.4);
};

drums = kick(v4, v0) * 0.8 
      + snare(v5, v1) * 0.9 
      + hihat(v6, v2) * 0.6 
      + clap(v7, v3) * 0.6;

process = drums <: _,_;