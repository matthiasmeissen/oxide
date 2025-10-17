import("stdfaust.lib");

map(x, out_min, out_max) = x * (out_max - out_min) + out_min;
map_exp(x, out_min, out_max) = out_min * exp(x * log(out_max / out_min));

v0 = hslider("[0] Osc1", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v1 = hslider("[1] Osc2", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v2 = hslider("[2] Osc3", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v3 = hslider("[3] Osc4", 0.2, 0.0, 1.0, 0.01) : si.smoo;
v4 = button("[4] Gate 1") : si.smoo;
v5 = button("[5] Gate 2") : si.smoo;
v6 = button("[6] Gate 3") : si.smoo;
v7 = button("[7] Gate 4") : si.smoo;

base_freq = 40.0;

freq1 = map_exp(v0, 20.0, 800.0);
osc1 = os.osc(base_freq + freq1);

freq2 = map_exp(v1, 20.0, 800.0);
osc2 = os.osc(base_freq + freq2);

freq3 = map_exp(v2, 20.0, 800.0);
osc3 = os.osc(base_freq + freq3);

freq4 = map_exp(v3, 20.0, 800.0);
osc4 = os.osc(base_freq + freq4);

waves = (osc1 + osc2 + osc3 + osc4) * 0.1;

process = waves <: _,_;