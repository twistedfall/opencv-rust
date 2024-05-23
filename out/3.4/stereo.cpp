#include "ocvrs_common.hpp"
#include <opencv2/stereo.hpp>
#include "stereo_types.hpp"

extern "C" {
	// censusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:22
	// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "image2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* image1, const cv::Mat* image2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::censusTransform(*image1, *image2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// censusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:24
	// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_censusTransform_const_MatR_int_MatR_const_int(const cv::Mat* image1, int kernelSize, cv::Mat* dist1, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::censusTransform(*image1, kernelSize, *dist1, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereo::modifiedCensusTransform(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:29
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// modifiedCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int, int, const Mat &, const Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:29
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type", "t", "integralImage1", "integralImage2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int", "int", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, int t, const cv::Mat* integralImage1, const cv::Mat* integralImage2, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type, t, *integralImage1, *integralImage2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereo::modifiedCensusTransform(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:31
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// modifiedCensusTransform(const Mat &, int, Mat &, const int, int, const Mat &)(TraitClass, Primitive, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:31
	// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type", "t", "integralImage"], ["const cv::Mat*", "int", "cv::Mat*", "const int", "int", "const cv::Mat*"]), _)]),
	void cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, const int type, int t, const cv::Mat* integralImage, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type, t, *integralImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// starCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:39
	// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::starCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// starCensusTransform(const Mat &, int, Mat &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:41
	// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist"], ["const cv::Mat*", "int", "cv::Mat*"]), _)]),
	void cv_stereo_starCensusTransform_const_MatR_int_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::starCensusTransform(*img1, kernelSize, *dist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// symetricCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:35
	// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::symetricCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// symetricCensusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:37
	// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
	void cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist1, const int type, ResultVoid* ocvrs_return) {
		try {
			cv::stereo::symetricCensusTransform(*img1, kernelSize, *dist1, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
