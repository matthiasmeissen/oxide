import("stdfaust.lib");

map(x, out_min, out_max) = x * (out_max - out_min) + out_min;
map_exp(x, out_min, out_max) = (exp(x * log(out_max - out_min + 1)) - 1) + out_min;

// Input
v0 = hslider("[0] Kick Pitch Decay", 0.2, 0.0, 1.0, 0.01);
v1 = hslider("[1] V2", 0.2, 0.0, 1.0, 0.01);
v2 = hslider("[2] V3", 0.2, 0.0, 1.0, 0.01);
v3 = hslider("[3] V4", 0.2, 0.0, 1.0, 0.01);
v4 = button("[4] Kick");
v5 = button("[5] Gate2");
v6 = button("[6] Gate3");
v7 = button("[7] Gate4");

val = (v0 + v1 + v2 + v3 + v4 + v5 + v6 + v7) * 0.00001;

kick(gate, pitch_decay) = ((oscillator * amp_env) + click) 
with {
    pitch       = 35;   // hslider("Pitch [unit:Hz][scale:log]", 35, 20, 100, 0.1);
    pitch_sweep = 80;   // hslider("Pitch Sweep [unit:Hz]", 80, 0, 500, 1);
    amp_decay   = 250;  // hslider("Amp Env Decay [unit:ms][scale:log]", 250, 1, 2000, 1);
    click_level = 0.1;  // hslider("Click Level [style:knob]", 0.2, 0, 1, 0.01);

    pitch_decay_sec = pitch_decay / 1000.0;
    amp_decay_sec = amp_decay / 1000.0;

    pitch_env = en.ar(0.001, pitch_decay_sec, gate);
    frequency = pitch_env * pitch_sweep + pitch;

    oscillator = os.osc(frequency);
    amp_env = en.ar(0.001, amp_decay_sec, gate);

    click_env = en.ar(0.001, 0.005, gate);
    click = no.noise * click_env * click_level;
};

//pitch_decay = hslider("Pitch Env Decay [unit:ms][scale:log]", 200, 100, 500, 0.1);
pitch_decay = map_exp(v0, 100.0, 500.0); 

process = val + kick(v4, pitch_decay) <: _,_;