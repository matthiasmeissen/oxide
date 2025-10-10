/* ------------------------------------------------------------
name: "drum_engine"
Code generated with Faust 2.81.10 (https://faust.grame.fr)
Compilation options: -a /usr/local/share/faust/rust/jack-float.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
/************************************************************************
 FAUST Architecture File
 Copyright (C) 2003-2024 GRAME, Centre National de Creation Musicale
 ---------------------------------------------------------------------
 This Architecture section is free software; you can redistribute it
 and/or modify it under the terms of the GNU General Public License
 as published by the Free Software Foundation; either version 3 of
 the License, or (at your option) any later version.
 
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with this program; If not, see <http://www.gnu.org/licenses/>.
 
 EXCEPTION : As a special exception, you may create a larger work
 that contains this FAUST architecture section and distribute
 that work under terms of your choice, so long as this FAUST
 architecture section is not modified.
 
 ************************************************************************
 ************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use crate::dsp::*;

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct DrumEngine {
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	iRec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fConst3: F32,
	fConst4: F32,
	fHslider0: F32,
	fRec1: [F32;2],
	fConst5: F32,
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	fConst23: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	iRec4: [i32;2],
	fRec3: [F32;5],
	fRec2: [F32;3],
	fConst30: F32,
	fButton1: F32,
	fVec2: [F32;2],
	iRec5: [i32;2],
	fConst31: F32,
	fConst32: F32,
	fHslider1: F32,
	fRec6: [F32;2],
	fConst33: F32,
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fConst37: F32,
	fConst38: F32,
	fConst39: F32,
	fConst40: F32,
	fConst41: F32,
	fConst42: F32,
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fConst46: F32,
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec9: [F32;3],
	fConst50: F32,
	fRec8: [F32;3],
	fConst51: F32,
	fRec7: [F32;3],
	fConst52: F32,
	fButton2: F32,
	fVec3: [F32;2],
	iRec10: [i32;2],
	fHslider2: F32,
	fRec11: [F32;2],
	fConst53: F32,
	fConst54: F32,
	fConst55: F32,
	fConst56: F32,
	fConst57: F32,
	fConst58: F32,
	fConst59: F32,
	fConst60: F32,
	fConst61: F32,
	fConst62: F32,
	fConst63: F32,
	fConst64: F32,
	fConst65: F32,
	fConst66: F32,
	fRec13: [F32;3],
	fConst67: F32,
	fRec12: [F32;3],
	fConst68: F32,
	fConst69: F32,
	fConst70: F32,
	fRec15: [F32;2],
	fConst71: F32,
	fRec16: [F32;2],
	fButton3: F32,
	fVec5: [F32;2],
	iRec17: [i32;2],
	fConst72: F32,
	fHslider3: F32,
	fRec18: [F32;2],
	fConst73: F32,
	fConst74: F32,
	fRec19: [F32;2],
}

pub type FaustFloat = F32;

pub struct mydspSIG0 {
	iVec4: [i32;2],
	iRec14: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	pub fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l18 in 0..2 {
			self.iVec4[l18 as usize] = 0;
		}
		for l19 in 0..2 {
			self.iRec14[l19 as usize] = 0;
		}
	}
	
	pub fn fillmydspSIG0(&mut self, count: i32, table: &mut[FaustFloat]) {
		for i1 in 0..count {
			self.iVec4[0] = 1;
			self.iRec14[0] = (i32::wrapping_add(self.iVec4[1], self.iRec14[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec14[0]) as F32);
			self.iVec4[1] = self.iVec4[0];
			self.iRec14[1] = self.iRec14[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iVec4: [0;2],
		iRec14: [0;2],
	}
}
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
fn mydsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}
fn mydsp_faustpower3_f(value: F32) -> F32 {
	return value * value * value;
}
static ftbl0mydspSIG0: std::sync::RwLock<[F32;65536]>  = std::sync::RwLock::new([0.0;65536]);
mod ffi {
	use std::os::raw::c_float;
	// Conditionally compile the link attribute only on non-Windows platforms
	#[cfg_attr(not(target_os = "windows"), link(name = "m"))]
	unsafe extern "C" {
		pub fn remainderf(from: c_float, to: c_float) -> c_float;
		pub fn rintf(val: c_float) -> c_float;
	}
}
fn remainder_f32(from: f32, to: f32) -> f32 {
	unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
	unsafe { ffi::rintf(val) }
}

pub const FAUST_INPUTS: usize = 0;
pub const FAUST_OUTPUTS: usize = 2;
pub const FAUST_ACTIVES: usize = 8;
pub const FAUST_PASSIVES: usize = 0;


impl DrumEngine {
		
	pub fn new() -> DrumEngine { 
		DrumEngine {
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			iRec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fHslider0: 0.0,
			fRec1: [0.0;2],
			fConst5: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			iRec4: [0;2],
			fRec3: [0.0;5],
			fRec2: [0.0;3],
			fConst30: 0.0,
			fButton1: 0.0,
			fVec2: [0.0;2],
			iRec5: [0;2],
			fConst31: 0.0,
			fConst32: 0.0,
			fHslider1: 0.0,
			fRec6: [0.0;2],
			fConst33: 0.0,
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fConst37: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fConst40: 0.0,
			fConst41: 0.0,
			fConst42: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fConst46: 0.0,
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec9: [0.0;3],
			fConst50: 0.0,
			fRec8: [0.0;3],
			fConst51: 0.0,
			fRec7: [0.0;3],
			fConst52: 0.0,
			fButton2: 0.0,
			fVec3: [0.0;2],
			iRec10: [0;2],
			fHslider2: 0.0,
			fRec11: [0.0;2],
			fConst53: 0.0,
			fConst54: 0.0,
			fConst55: 0.0,
			fConst56: 0.0,
			fConst57: 0.0,
			fConst58: 0.0,
			fConst59: 0.0,
			fConst60: 0.0,
			fConst61: 0.0,
			fConst62: 0.0,
			fConst63: 0.0,
			fConst64: 0.0,
			fConst65: 0.0,
			fConst66: 0.0,
			fRec13: [0.0;3],
			fConst67: 0.0,
			fRec12: [0.0;3],
			fConst68: 0.0,
			fConst69: 0.0,
			fConst70: 0.0,
			fRec15: [0.0;2],
			fConst71: 0.0,
			fRec16: [0.0;2],
			fButton3: 0.0,
			fVec5: [0.0;2],
			iRec17: [0;2],
			fConst72: 0.0,
			fHslider3: 0.0,
			fRec18: [0.0;2],
			fConst73: 0.0,
			fConst74: 0.0,
			fRec19: [0.0;2],
		}
	}
	pub fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/version", r"1.22.0");
		m.declare("compile_options", r"-a /usr/local/share/faust/rust/jack-float.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("envelopes.lib/ar:author", r"Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", r"GRAME");
		m.declare("envelopes.lib/copyright", r"GRAME");
		m.declare("envelopes.lib/license", r"LGPL with exception");
		m.declare("envelopes.lib/name", r"Faust Envelope Library");
		m.declare("envelopes.lib/version", r"1.3.0");
		m.declare("filename", r"drum_engine.dsp");
		m.declare("filters.lib/bandpass0_bandstop1:author", r"Julius O. Smith III");
		m.declare("filters.lib/bandpass0_bandstop1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/bandpass0_bandstop1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/bandpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/bandpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/bandpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/resonbp:author", r"Julius O. Smith III");
		m.declare("filters.lib/resonbp:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/resonbp:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2sb:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2sb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2sb:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.7.1");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.9.0");
		m.declare("name", r"drum_engine");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.5.0");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/version", r"1.6.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.6.0");
	}

	pub fn get_sample_rate(&self) -> i32 { self.fSampleRate as i32}
	
	pub fn class_init(sample_rate: i32) {
		// Obtaining locks on 1 static var(s)
		let mut ftbl0mydspSIG0_guard = ftbl0mydspSIG0.write().unwrap();
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(65536, ftbl0mydspSIG0_guard.as_mut());
	}
	pub fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.2;
		self.fButton1 = 0.0;
		self.fHslider1 = 0.2;
		self.fButton2 = 0.0;
		self.fHslider2 = 0.2;
		self.fButton3 = 0.0;
		self.fHslider3 = 0.2;
	}
	pub fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.iRec0[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.fRec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.iRec4[l4 as usize] = 0;
		}
		for l5 in 0..5 {
			self.fRec3[l5 as usize] = 0.0;
		}
		for l6 in 0..3 {
			self.fRec2[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fVec2[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.iRec5[l8 as usize] = 0;
		}
		for l9 in 0..2 {
			self.fRec6[l9 as usize] = 0.0;
		}
		for l10 in 0..3 {
			self.fRec9[l10 as usize] = 0.0;
		}
		for l11 in 0..3 {
			self.fRec8[l11 as usize] = 0.0;
		}
		for l12 in 0..3 {
			self.fRec7[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fVec3[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.iRec10[l14 as usize] = 0;
		}
		for l15 in 0..2 {
			self.fRec11[l15 as usize] = 0.0;
		}
		for l16 in 0..3 {
			self.fRec13[l16 as usize] = 0.0;
		}
		for l17 in 0..3 {
			self.fRec12[l17 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec15[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec16[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec5[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.iRec17[l23 as usize] = 0;
		}
		for l24 in 0..2 {
			self.fRec18[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec19[l25 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 1 static var(s)
		let ftbl0mydspSIG0_guard = ftbl0mydspSIG0.read().unwrap();
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::max(1.0, 0.01 * self.fConst0);
		self.fConst2 = 1.0 / self.fConst1;
		self.fConst3 = 44.1 / self.fConst0;
		self.fConst4 = 1.0 - self.fConst3;
		self.fConst5 = 0.0004 * self.fConst0;
		self.fConst6 = F32::max(1.0, 0.0005 * self.fConst0);
		self.fConst7 = 1.0 / self.fConst6;
		self.fConst8 = 0.001 * self.fConst0;
		self.fConst9 = F32::tan(2513.2742 / self.fConst0);
		self.fConst10 = mydsp_faustpower2_f(self.fConst9);
		self.fConst11 = 2.0 * (1.0 - 1.0 / self.fConst10);
		self.fConst12 = 1.0 / self.fConst9;
		self.fConst13 = (self.fConst12 + -1.4142135) / self.fConst9 + 1.0;
		self.fConst14 = (self.fConst12 + 1.4142135) / self.fConst9 + 1.0;
		self.fConst15 = 1.0 / self.fConst14;
		self.fConst16 = 3.1415927 / self.fConst0;
		self.fConst17 = 4.0 * mydsp_faustpower2_f(self.fConst0);
		self.fConst18 = 0.5 / self.fConst0;
		self.fConst19 = 2.0 * self.fConst0;
		self.fConst20 = 1.0 / self.fConst0;
		self.fConst21 = mydsp_faustpower2_f(self.fConst20);
		self.fConst22 = mydsp_faustpower4_f(self.fConst20);
		self.fConst23 = mydsp_faustpower3_f(self.fConst20);
		self.fConst24 = 2.828427 * self.fConst23;
		self.fConst25 = 11.313708 / self.fConst0;
		self.fConst26 = 5.656854 * self.fConst23;
		self.fConst27 = 22.627417 / self.fConst0;
		self.fConst28 = 4.0 * self.fConst22;
		self.fConst29 = 6.0 * self.fConst22;
		self.fConst30 = 0.6 / (self.fConst10 * self.fConst14);
		self.fConst31 = F32::max(1.0, self.fConst8);
		self.fConst32 = 1.0 / self.fConst31;
		self.fConst33 = 0.005 * self.fConst0;
		self.fConst34 = F32::tan(25132.742 / self.fConst0);
		self.fConst35 = 2.0 * (1.0 - 1.0 / mydsp_faustpower2_f(self.fConst34));
		self.fConst36 = 1.0 / self.fConst34;
		self.fConst37 = (self.fConst36 + -0.4) / self.fConst34 + 1.0;
		self.fConst38 = (self.fConst36 + 0.4) / self.fConst34 + 1.0;
		self.fConst39 = 1.0 / self.fConst38;
		self.fConst40 = F32::tan(14137.167 / self.fConst0);
		self.fConst41 = mydsp_faustpower2_f(self.fConst40);
		self.fConst42 = 2.0 * (1.0 - 1.0 / self.fConst41);
		self.fConst43 = 1.0 / self.fConst40;
		self.fConst44 = (self.fConst43 + -0.76536685) / self.fConst40 + 1.0;
		self.fConst45 = (self.fConst43 + 0.76536685) / self.fConst40 + 1.0;
		self.fConst46 = 1.0 / self.fConst45;
		self.fConst47 = (self.fConst43 + -1.847759) / self.fConst40 + 1.0;
		self.fConst48 = (self.fConst43 + 1.847759) / self.fConst40 + 1.0;
		self.fConst49 = 1.0 / self.fConst48;
		self.fConst50 = 1.0 / (self.fConst41 * self.fConst48);
		self.fConst51 = 1.0 / (self.fConst41 * self.fConst45);
		self.fConst52 = 0.48 / (self.fConst34 * self.fConst38);
		self.fConst53 = 0.02 * self.fConst0;
		self.fConst54 = F32::tan(9424.778 / self.fConst0);
		self.fConst55 = 2.0 * (1.0 - 1.0 / mydsp_faustpower2_f(self.fConst54));
		self.fConst56 = 1.0 / self.fConst54;
		self.fConst57 = (self.fConst56 + -0.8333333) / self.fConst54 + 1.0;
		self.fConst58 = (self.fConst56 + 0.8333333) / self.fConst54 + 1.0;
		self.fConst59 = 1.0 / self.fConst58;
		self.fConst60 = F32::tan(4712.389 / self.fConst0);
		self.fConst61 = mydsp_faustpower2_f(self.fConst60);
		self.fConst62 = 2.0 * (1.0 - 1.0 / self.fConst61);
		self.fConst63 = 1.0 / self.fConst60;
		self.fConst64 = (self.fConst63 + -1.4142135) / self.fConst60 + 1.0;
		self.fConst65 = (self.fConst63 + 1.4142135) / self.fConst60 + 1.0;
		self.fConst66 = 1.0 / self.fConst65;
		self.fConst67 = 1.0 / (self.fConst61 * self.fConst65);
		self.fConst68 = 0.28 / (self.fConst54 * self.fConst58);
		self.fConst69 = 1.0 / F32::max(1.0, 0.05 * self.fConst0);
		self.fConst70 = 2.7e+02 / self.fConst0;
		self.fConst71 = 1.8e+02 / self.fConst0;
		self.fConst72 = 1.0 / F32::max(1.0, self.fConst33);
		self.fConst73 = 0.2 * self.fConst0;
		self.fConst74 = 0.1 * self.fConst0;
	}
	pub fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	pub fn init(&mut self, sample_rate: i32) {
		DrumEngine::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
		ui_interface.open_vertical_box("drum_engine");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_horizontal_slider("Kick Decay", ParamIndex(0), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("Snare Decay", ParamIndex(1), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("Hihat Decay", ParamIndex(2), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.add_horizontal_slider("Clap Decay", ParamIndex(3), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "4", "");
		ui_interface.add_button("Kick", ParamIndex(4));
		ui_interface.declare(Some(ParamIndex(5)), "5", "");
		ui_interface.add_button("Snare", ParamIndex(5));
		ui_interface.declare(Some(ParamIndex(6)), "6", "");
		ui_interface.add_button("Hihat", ParamIndex(6));
		ui_interface.declare(Some(ParamIndex(7)), "7", "");
		ui_interface.add_button("Clap", ParamIndex(7));
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			7 => Some(self.fButton0),
			6 => Some(self.fButton1),
			5 => Some(self.fButton2),
			4 => Some(self.fButton3),
			3 => Some(self.fHslider0),
			2 => Some(self.fHslider1),
			1 => Some(self.fHslider2),
			0 => Some(self.fHslider3),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			7 => { self.fButton0 = value }
			6 => { self.fButton1 = value }
			5 => { self.fButton2 = value }
			4 => { self.fButton3 = value }
			3 => { self.fHslider0 = value }
			2 => { self.fHslider1 = value }
			1 => { self.fHslider2 = value }
			0 => { self.fHslider3 = value }
			_ => {}
		}
	}
	
	pub fn compute(
		&mut self,
		count: usize,
		inputs: &[impl AsRef<[FaustFloat]>],
		outputs: &mut[impl AsMut<[FaustFloat]>],
	) {
		
		// Obtaining locks on 1 static var(s)
		let ftbl0mydspSIG0_guard = ftbl0mydspSIG0.read().unwrap();
		let [outputs0, outputs1, .. ] = outputs.as_mut() else { panic!("wrong number of output buffers"); };
		let outputs0 = outputs0.as_mut()[..count].iter_mut();
		let outputs1 = outputs1.as_mut()[..count].iter_mut();
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = self.fConst3 * self.fHslider0;
		let mut fSlow2: F32 = self.fButton1;
		let mut fSlow3: F32 = self.fConst3 * self.fHslider1;
		let mut fSlow4: F32 = self.fButton2;
		let mut fSlow5: F32 = self.fConst3 * self.fHslider2;
		let mut fSlow6: F32 = self.fButton3;
		let mut fSlow7: F32 = self.fConst3 * self.fHslider3;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.iVec1[0] = 1;
			self.iRec0[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec0[1], (self.iRec0[1] > 0) as i32), (fSlow0 <= self.fVec0[1]) as i32), (fSlow0 > self.fVec0[1]) as i32);
			let mut fTemp0: F32 = (self.iRec0[0]) as F32;
			self.fRec1[0] = fSlow1 + self.fConst4 * self.fRec1[1];
			let mut fTemp1: F32 = 1.4e+02 * self.fRec1[0] + 6e+01;
			let mut fTemp2: F32 = 0.5 * (2.5e+03 - 1e+03 * self.fRec1[0]);
			let mut fTemp3: F32 = 1.5e+03 * F32::exp(1.6739764 * self.fRec1[0]);
			let mut fTemp4: F32 = F32::tan(self.fConst16 * (fTemp3 + fTemp2));
			let mut fTemp5: F32 = F32::sqrt(self.fConst17 * F32::tan(self.fConst16 * (fTemp3 - fTemp2)) * fTemp4);
			let mut fTemp6: F32 = mydsp_faustpower2_f(fTemp5);
			let mut fTemp7: F32 = self.fConst19 * fTemp4 - self.fConst18 * (fTemp6 / fTemp4);
			let mut fTemp8: F32 = mydsp_faustpower2_f(fTemp7);
			let mut fTemp9: F32 = mydsp_faustpower4_f(fTemp5);
			let mut fTemp10: F32 = self.fConst22 * fTemp9 + self.fConst21 * (4.0 * fTemp8 + 8.0 * fTemp6);
			let mut fTemp11: F32 = fTemp7 * (self.fConst25 + self.fConst24 * fTemp6);
			let mut fTemp12: F32 = fTemp11 + fTemp10 + 16.0;
			let mut fTemp13: F32 = self.fConst26 * fTemp6;
			let mut fTemp14: F32 = self.fConst28 * fTemp9;
			self.iRec4[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec4[1]), 12345);
			let mut fTemp15: F32 = (self.iRec4[0]) as F32;
			let mut fTemp16: F32 = 4.656613e-10 * fTemp15;
			self.fRec3[0] = fTemp16 - (self.fRec3[1] * (fTemp14 + fTemp7 * (fTemp13 - self.fConst27) + -64.0) + self.fRec3[2] * (self.fConst29 * fTemp9 + (96.0 - self.fConst21 * (8.0 * fTemp8 + 16.0 * fTemp6))) + self.fRec3[3] * (fTemp14 + fTemp7 * (self.fConst27 - fTemp13) + -64.0) + self.fRec3[4] * (fTemp10 + (16.0 - fTemp11))) / fTemp12;
			self.fRec2[0] = self.fConst21 * (fTemp8 * (4.0 * self.fRec3[0] - 8.0 * self.fRec3[2] + 4.0 * self.fRec3[4]) / fTemp12) - self.fConst15 * (self.fConst13 * self.fRec2[2] + self.fConst11 * self.fRec2[1]);
			self.fVec2[0] = fSlow2;
			self.iRec5[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec5[1], (self.iRec5[1] > 0) as i32), (fSlow2 <= self.fVec2[1]) as i32), (fSlow2 > self.fVec2[1]) as i32);
			let mut fTemp17: F32 = (self.iRec5[0]) as F32;
			self.fRec6[0] = fSlow3 + self.fConst4 * self.fRec6[1];
			self.fRec9[0] = fTemp16 - self.fConst49 * (self.fConst47 * self.fRec9[2] + self.fConst42 * self.fRec9[1]);
			self.fRec8[0] = self.fConst50 * (self.fRec9[2] + (self.fRec9[0] - 2.0 * self.fRec9[1])) - self.fConst46 * (self.fConst44 * self.fRec8[2] + self.fConst42 * self.fRec8[1]);
			self.fRec7[0] = self.fConst51 * (self.fRec8[2] + (self.fRec8[0] - 2.0 * self.fRec8[1])) - self.fConst39 * (self.fConst37 * self.fRec7[2] + self.fConst35 * self.fRec7[1]);
			self.fVec3[0] = fSlow4;
			self.iRec10[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec10[1], (self.iRec10[1] > 0) as i32), (fSlow4 <= self.fVec3[1]) as i32), (fSlow4 > self.fVec3[1]) as i32);
			let mut fTemp18: F32 = (self.iRec10[0]) as F32;
			let mut fTemp19: F32 = self.fConst32 * fTemp18;
			self.fRec11[0] = fSlow5 + self.fConst4 * self.fRec11[1];
			let mut fTemp20: F32 = self.fConst31 - fTemp18;
			self.fRec13[0] = fTemp16 - self.fConst66 * (self.fConst64 * self.fRec13[2] + self.fConst62 * self.fRec13[1]);
			self.fRec12[0] = self.fConst67 * (self.fRec13[2] + (self.fRec13[0] - 2.0 * self.fRec13[1])) - self.fConst59 * (self.fConst57 * self.fRec12[2] + self.fConst55 * self.fRec12[1]);
			let mut iTemp21: i32 = i32::wrapping_sub(1, self.iVec1[1]);
			let mut fTemp22: F32 = (if iTemp21 != 0 {0.0} else {self.fConst70 + self.fRec15[1]});
			self.fRec15[0] = fTemp22 - F32::floor(fTemp22);
			let mut fTemp23: F32 = (if iTemp21 != 0 {0.0} else {self.fConst71 + self.fRec16[1]});
			self.fRec16[0] = fTemp23 - F32::floor(fTemp23);
			self.fVec5[0] = fSlow6;
			self.iRec17[0] = ((fSlow6 > self.fVec5[1]) as i32) + ((fSlow6 <= self.fVec5[1]) as i32) * (i32::wrapping_add(self.iRec17[1], (self.iRec17[1] > 0) as i32));
			let mut fTemp24: F32 = (self.iRec17[0]) as F32;
			let mut fTemp25: F32 = self.fConst32 * fTemp24;
			let mut fTemp26: F32 = self.fConst31 - fTemp24;
			self.fRec18[0] = fSlow7 + self.fConst4 * self.fRec18[1];
			let mut fTemp27: F32 = (if iTemp21 != 0 {0.0} else {self.fRec19[1] + self.fConst20 * (8e+01 * F32::max(0.0, F32::min(fTemp25, fTemp26 / F32::max(1.0, self.fConst74 * F32::exp(1.609438 * self.fRec18[0])) + 1.0)) + 35.0)});
			self.fRec19[0] = fTemp27 - F32::floor(fTemp27);
			let mut fTemp28: F32 = 0.8 * (ftbl0mydspSIG0_guard[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec19[0]) as i32, 65535))) as usize] * F32::max(0.0, F32::min(fTemp25, fTemp26 / F32::max(1.0, self.fConst73 * F32::exp(1.0986123 * self.fRec18[0])) + 1.0)) + 4.656613e-11 * fTemp15 * F32::max(0.0, F32::min(fTemp25, self.fConst72 * fTemp26 + 1.0))) + 0.9 * (0.3 * (ftbl0mydspSIG0_guard[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec16[0]) as i32, 65535))) as usize] + ftbl0mydspSIG0_guard[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec15[0]) as i32, 65535))) as usize]) * F32::max(0.0, F32::min(fTemp19, self.fConst69 * fTemp20 + 1.0)) + self.fConst68 * (self.fRec12[0] - self.fRec12[2]) * F32::max(0.0, F32::min(fTemp19, fTemp20 / F32::max(1.0, self.fConst53 * F32::exp(3.1135154 * self.fRec11[0])) + 1.0))) + self.fConst52 * (self.fRec7[0] - self.fRec7[2]) * F32::max(0.0, F32::min(self.fConst32 * fTemp17, (self.fConst31 - fTemp17) / F32::max(1.0, self.fConst33 * F32::exp(5.075174 * self.fRec6[0])) + 1.0)) + self.fConst30 * (self.fRec2[2] + (self.fRec2[0] - 2.0 * self.fRec2[1])) * (F32::max(0.0, F32::min(self.fConst7 * fTemp0, (self.fConst6 - fTemp0) / F32::max(1.0, self.fConst8 * fTemp1) + 1.0)) + 0.4 * F32::max(0.0, F32::min(self.fConst2 * fTemp0, (self.fConst1 - fTemp0) / F32::max(1.0, self.fConst5 * fTemp1) + 1.0)));
			*output0 = fTemp28;
			*output1 = fTemp28;
			self.fVec0[1] = self.fVec0[0];
			self.iVec1[1] = self.iVec1[0];
			self.iRec0[1] = self.iRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.iRec4[1] = self.iRec4[0];
			for j0 in (1..=4).rev() {
				self.fRec3[j0 as usize] = self.fRec3[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.fVec2[1] = self.fVec2[0];
			self.iRec5[1] = self.iRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec9[2] = self.fRec9[1];
			self.fRec9[1] = self.fRec9[0];
			self.fRec8[2] = self.fRec8[1];
			self.fRec8[1] = self.fRec8[0];
			self.fRec7[2] = self.fRec7[1];
			self.fRec7[1] = self.fRec7[0];
			self.fVec3[1] = self.fVec3[0];
			self.iRec10[1] = self.iRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fVec5[1] = self.fVec5[0];
			self.iRec17[1] = self.iRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
		}
		
	}

}

impl FaustDsp for DrumEngine {
	type T = FaustFloat;
	fn new() -> Self where Self: Sized {
		Self::new()
	}
	fn metadata(&self, m: &mut dyn Meta) {
		self.metadata(m)
	}
	fn get_sample_rate(&self) -> i32 {
		self.get_sample_rate()
	}
	fn get_num_inputs(&self) -> i32 {
		FAUST_INPUTS as i32
	}
	fn get_num_outputs(&self) -> i32 {
		FAUST_OUTPUTS as i32
	}
	fn class_init(sample_rate: i32) where Self: Sized {
		Self::class_init(sample_rate);
	}
	fn instance_reset_params(&mut self) {
		self.instance_reset_params()
	}
	fn instance_clear(&mut self) {
		self.instance_clear()
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate)
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_init(sample_rate)
	}
	fn init(&mut self, sample_rate: i32) {
		self.init(sample_rate)
	}
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		self.build_user_interface(ui_interface)
	}
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized {
		Self::build_user_interface_static(ui_interface);
	}
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		self.get_param(param)
	}
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		self.set_param(param, value)
	}
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
		self.compute(count as usize, inputs, outputs)
	}
}
