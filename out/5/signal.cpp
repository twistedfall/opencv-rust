#include "ocvrs_common.hpp"
#include <opencv2/signal.hpp>
#include "signal_types.hpp"

extern "C" {
	// resampleSignal(InputArray, OutputArray, const int, const int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/signal/signal_resample.hpp:26
	// ("cv::signal::resampleSignal", vec![(pred!(mut, ["inputSignal", "outSignal", "inFreq", "outFreq"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const int", "const int"]), _)]),
	void cv_signal_resampleSignal_const__InputArrayR_const__OutputArrayR_const_int_const_int(const cv::_InputArray* inputSignal, const cv::_OutputArray* outSignal, const int inFreq, const int outFreq, ResultVoid* ocvrs_return) {
		try {
			cv::signal::resampleSignal(*inputSignal, *outSignal, inFreq, outFreq);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
