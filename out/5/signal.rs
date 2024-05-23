//! # Signal Processing
//! This module includes signal processing algorithms.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
}

/// Signal resampling
///
/// ## Parameters
/// * inputSignal: Array with input signal.
/// * outSignal:[out] Array with output signal
/// * inFreq: Input signal frequency.
/// * outFreq: Output signal frequency.
///  Signal resampling implemented a cubic interpolation function and a filtering function based on Kaiser window and Bessel function, used to construct a FIR filter.
///  Result is similar to `scipy.signal.resample`.
///
/// Detail: <https://en.wikipedia.org/wiki/Sample-rate_conversion>
// resampleSignal(InputArray, OutputArray, const int, const int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/signal/signal_resample.hpp:26
// ("cv::signal::resampleSignal", vec![(pred!(mut, ["inputSignal", "outSignal", "inFreq", "outFreq"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const int", "const int"]), _)]),
#[inline]
pub fn resample_signal(input_signal: &impl ToInputArray, out_signal: &mut impl ToOutputArray, in_freq: i32, out_freq: i32) -> Result<()> {
	input_array_arg!(input_signal);
	output_array_arg!(out_signal);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_signal_resampleSignal_const__InputArrayR_const__OutputArrayR_const_int_const_int(input_signal.as_raw__InputArray(), out_signal.as_raw__OutputArray(), in_freq, out_freq, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}
