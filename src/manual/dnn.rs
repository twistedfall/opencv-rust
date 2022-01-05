use std::{
	ffi::c_void,
	fmt,
};

use crate::{
	dnn::{DictValue, LayerParams},
	prelude::*,
	Result,
	sys,
};

impl fmt::Debug for DictValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut d = f.debug_struct("DictValue");
		d.field("is_int", &self.is_int().map_err(|_| fmt::Error)?)
			.field("is_real", &self.is_real().map_err(|_| fmt::Error)?)
			.field("is_string", &self.is_string().map_err(|_| fmt::Error)?);
		if self.is_string().map_err(|_| fmt::Error)? {
			d.field("value", &self.get_str(-1).map_err(|_| fmt::Error)?);
		} else if self.is_int().map_err(|_| fmt::Error)? {
			d.field("value", &self.get_i64(-1).map_err(|_| fmt::Error)?);
		} else if self.is_real().map_err(|_| fmt::Error)? {
			d.field("value", &self.get_f64(-1).map_err(|_| fmt::Error)?);
		}
		d.finish()
	}
}

impl LayerParams {
	pub fn default() -> Result<Self> {
		extern "C" { fn cv_dnn_LayerParams_LayerParams(ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_dnn_LayerParams_LayerParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { LayerParams::from_raw(ptr) })
	}
}
