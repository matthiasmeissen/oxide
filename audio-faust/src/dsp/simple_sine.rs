/* ------------------------------------------------------------
name: "simple_sine"
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

type F32 = f32;
type F64 = f64;

#[derive(Copy, Clone)]
pub struct ParamIndex(pub i32);

pub struct Soundfile<'a,T> {
    fBuffers: &'a&'a T,
    fLength: &'a i32,
    fSR: &'a i32,
    fOffset: &'a i32,
    fChannels: i32
}

pub trait FaustDsp {
    type T;

    fn new() -> Self where Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32) where Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_horizontal_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct SimpleSine {
	iVec1: [i32;2],
	fHslider0: F32,
	fSampleRate: i32,
	fConst0: F32,
	fRec1: [F32;2],
}

pub type FaustFloat = F32;

pub struct mydspSIG0 {
	iVec0: [i32;2],
	iRec0: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	pub fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec0[l1 as usize] = 0;
		}
	}
	
	pub fn fillmydspSIG0(&mut self, count: i32, table: &mut[FaustFloat]) {
		for i1 in 0..count {
			self.iVec0[0] = 1;
			self.iRec0[0] = (i32::wrapping_add(self.iVec0[1], self.iRec0[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec0[0]) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.iRec0[1] = self.iRec0[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iVec0: [0;2],
		iRec0: [0;2],
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
pub const FAUST_ACTIVES: usize = 1;
pub const FAUST_PASSIVES: usize = 0;


impl SimpleSine {
		
	pub fn new() -> SimpleSine { 
		SimpleSine {
			iVec1: [0;2],
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fRec1: [0.0;2],
		}
	}
	pub fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/version", r"1.22.0");
		m.declare("compile_options", r"-a /usr/local/share/faust/rust/jack-float.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"simple_sine.dsp");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.9.0");
		m.declare("name", r"simple_sine");
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
		self.fHslider0 = 4.4e+02;
	}
	pub fn instance_clear(&mut self) {
		for l2 in 0..2 {
			self.iVec1[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.fRec1[l3 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 1 static var(s)
		let ftbl0mydspSIG0_guard = ftbl0mydspSIG0.read().unwrap();
		self.fSampleRate = sample_rate;
		self.fConst0 = 1.0 / F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
	}
	pub fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	pub fn init(&mut self, sample_rate: i32) {
		SimpleSine::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
		ui_interface.open_vertical_box("simple_sine");
		ui_interface.add_horizontal_slider("freq", ParamIndex(0), 4.4e+02, 4e+01, 2e+03, 1.0);
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			0 => Some(self.fHslider0),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			0 => { self.fHslider0 = value }
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
		let mut fSlow0: F32 = self.fConst0 * self.fHslider0;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec1[0] = 1;
			let mut fTemp0: F32 = (if i32::wrapping_sub(1, self.iVec1[1]) != 0 {0.0} else {fSlow0 + self.fRec1[1]});
			self.fRec1[0] = fTemp0 - F32::floor(fTemp0);
			let mut fTemp1: F32 = 0.2 * ftbl0mydspSIG0_guard[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec1[0]) as i32, 65535))) as usize];
			*output0 = fTemp1;
			*output1 = fTemp1;
			self.iVec1[1] = self.iVec1[0];
			self.fRec1[1] = self.fRec1[0];
		}
		
	}

}

impl FaustDsp for SimpleSine {
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
