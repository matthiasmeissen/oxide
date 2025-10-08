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
	iVec0: [i32;2],
	fButton0: F32,
	fVec1: [F32;2],
	iRec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fConst3: F32,
	fConst4: F32,
	iRec1: [i32;2],
	fConst5: F32,
	fHslider0: F32,
	fConst6: F32,
	fRec3: [F32;2],
	fButton1: F32,
	fButton2: F32,
	fButton3: F32,
	fHslider1: F32,
	fHslider2: F32,
	fHslider3: F32,
}

pub type FaustFloat = F32;

pub struct mydspSIG0 {
	iVec2: [i32;2],
	iRec2: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	pub fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l4 in 0..2 {
			self.iVec2[l4 as usize] = 0;
		}
		for l5 in 0..2 {
			self.iRec2[l5 as usize] = 0;
		}
	}
	
	pub fn fillmydspSIG0(&mut self, count: i32, table: &mut[FaustFloat]) {
		for i1 in 0..count {
			self.iVec2[0] = 1;
			self.iRec2[0] = (i32::wrapping_add(self.iVec2[1], self.iRec2[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec2[0]) as F32);
			self.iVec2[1] = self.iVec2[0];
			self.iRec2[1] = self.iRec2[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iVec2: [0;2],
		iRec2: [0;2],
	}
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
			iVec0: [0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			iRec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			iRec1: [0;2],
			fConst5: 0.0,
			fHslider0: 0.0,
			fConst6: 0.0,
			fRec3: [0.0;2],
			fButton1: 0.0,
			fButton2: 0.0,
			fButton3: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fHslider3: 0.0,
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
		self.fButton2 = 0.0;
		self.fButton3 = 0.0;
		self.fHslider1 = 0.2;
		self.fHslider2 = 0.2;
		self.fHslider3 = 0.2;
	}
	pub fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.iRec0[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.iRec1[l3 as usize] = 0;
		}
		for l6 in 0..2 {
			self.fRec3[l6 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 1 static var(s)
		let ftbl0mydspSIG0_guard = ftbl0mydspSIG0.read().unwrap();
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 0.001 * self.fConst0;
		self.fConst2 = F32::max(1.0, self.fConst1);
		self.fConst3 = 1.0 / self.fConst2;
		self.fConst4 = 1.0 / F32::max(1.0, 0.005 * self.fConst0);
		self.fConst5 = 1.0 / F32::max(1.0, 0.25 * self.fConst0);
		self.fConst6 = 1.0 / self.fConst0;
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
		ui_interface.add_horizontal_slider("Kick Pitch Decay", ParamIndex(0), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("V2", ParamIndex(1), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("V3", ParamIndex(2), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.add_horizontal_slider("V4", ParamIndex(3), 0.2, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "4", "");
		ui_interface.add_button("Kick", ParamIndex(4));
		ui_interface.declare(Some(ParamIndex(5)), "5", "");
		ui_interface.add_button("Gate2", ParamIndex(5));
		ui_interface.declare(Some(ParamIndex(6)), "6", "");
		ui_interface.add_button("Gate3", ParamIndex(6));
		ui_interface.declare(Some(ParamIndex(7)), "7", "");
		ui_interface.add_button("Gate4", ParamIndex(7));
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			4 => Some(self.fButton0),
			7 => Some(self.fButton1),
			6 => Some(self.fButton2),
			5 => Some(self.fButton3),
			0 => Some(self.fHslider0),
			3 => Some(self.fHslider1),
			2 => Some(self.fHslider2),
			1 => Some(self.fHslider3),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			4 => { self.fButton0 = value }
			7 => { self.fButton1 = value }
			6 => { self.fButton2 = value }
			5 => { self.fButton3 = value }
			0 => { self.fHslider0 = value }
			3 => { self.fHslider1 = value }
			2 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
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
		let mut fSlow1: F32 = self.fHslider0;
		let mut fSlow2: F32 = 1.0 / F32::max(1.0, self.fConst1 * (F32::exp(5.9939613 * fSlow1) + 99.0));
		let mut fSlow3: F32 = 1e-05 * (self.fHslider3 + self.fHslider2 + self.fHslider1 + self.fButton3 + self.fButton2 + self.fButton1 + fSlow1 + fSlow0);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			self.iRec0[0] = ((fSlow0 > self.fVec1[1]) as i32) + ((fSlow0 <= self.fVec1[1]) as i32) * (i32::wrapping_add(self.iRec0[1], (self.iRec0[1] > 0) as i32));
			let mut fTemp0: F32 = (self.iRec0[0]) as F32;
			let mut fTemp1: F32 = self.fConst3 * fTemp0;
			let mut fTemp2: F32 = self.fConst2 - fTemp0;
			self.iRec1[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec1[1]), 12345);
			let mut fTemp3: F32 = (if i32::wrapping_sub(1, self.iVec0[1]) != 0 {0.0} else {self.fRec3[1] + self.fConst6 * (8e+01 * F32::max(0.0, F32::min(fTemp1, fSlow2 * fTemp2 + 1.0)) + 35.0)});
			self.fRec3[0] = fTemp3 - F32::floor(fTemp3);
			let mut fTemp4: F32 = fSlow3 + ftbl0mydspSIG0_guard[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec3[0]) as i32, 65535))) as usize] * F32::max(0.0, F32::min(fTemp1, self.fConst5 * fTemp2 + 1.0)) + 4.656613e-11 * (self.iRec1[0]) as F32 * F32::max(0.0, F32::min(fTemp1, self.fConst4 * fTemp2 + 1.0));
			*output0 = fTemp4;
			*output1 = fTemp4;
			self.iVec0[1] = self.iVec0[0];
			self.fVec1[1] = self.fVec1[0];
			self.iRec0[1] = self.iRec0[0];
			self.iRec1[1] = self.iRec1[0];
			self.fRec3[1] = self.fRec3[0];
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
