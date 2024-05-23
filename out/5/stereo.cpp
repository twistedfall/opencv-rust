#include "ocvrs_common.hpp"
#include <opencv2/stereo.hpp>
#include "stereo_types.hpp"

extern "C" {
	// cv::filterSpeckles(InputOutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:429
	// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff"], ["const cv::_InputOutputArray*", "double", "int", "double"]), _)]),
	void cv_filterSpeckles_const__InputOutputArrayR_double_int_double(const cv::_InputOutputArray* img, double newVal, int maxSpeckleSize, double maxDiff, ResultVoid* ocvrs_return) {
		try {
			cv::filterSpeckles(*img, newVal, maxSpeckleSize, maxDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filterSpeckles(InputOutputArray, double, int, double, InputOutputArray)(InputOutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:429
	// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff", "buf"], ["const cv::_InputOutputArray*", "double", "int", "double", "const cv::_InputOutputArray*"]), _)]),
	void cv_filterSpeckles_const__InputOutputArrayR_double_int_double_const__InputOutputArrayR(const cv::_InputOutputArray* img, double newVal, int maxSpeckleSize, double maxDiff, const cv::_InputOutputArray* buf, ResultVoid* ocvrs_return) {
		try {
			cv::filterSpeckles(*img, newVal, maxSpeckleSize, maxDiff, *buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fisheye::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:233
	// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* tvec, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::stereoRectify(*K1, *D1, *K2, *D2, *imageSize, *R, *tvec, *R1, *R2, *P1, *P2, *Q, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectify(InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, const Size &, double, double)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:233
	// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags", "newImageSize", "balance", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::Size*", "double", "double"]), _)]),
	void cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double(const cv::_InputArray* K1, const cv::_InputArray* D1, const cv::_InputArray* K2, const cv::_InputArray* D2, const cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* tvec, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, const cv::Size* newImageSize, double balance, double fov_scale, ResultVoid* ocvrs_return) {
		try {
			cv::fisheye::stereoRectify(*K1, *D1, *K2, *D2, *imageSize, *R, *tvec, *R1, *R2, *P1, *P2, *Q, flags, *newImageSize, balance, fov_scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getValidDisparityROI(Rect, Rect, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:434
	// ("cv::getValidDisparityROI", vec![(pred!(mut, ["roi1", "roi2", "minDisparity", "numberOfDisparities", "blockSize"], ["cv::Rect", "cv::Rect", "int", "int", "int"]), _)]),
	void cv_getValidDisparityROI_Rect_Rect_int_int_int(cv::Rect* roi1, cv::Rect* roi2, int minDisparity, int numberOfDisparities, int blockSize, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::getValidDisparityROI(*roi1, *roi2, minDisparity, numberOfDisparities, blockSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rectify3Collinear(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArrayOfArrays, InputArrayOfArrays, Size, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double, Size, Rect *, Rect *, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:188
	// ("cv::rectify3Collinear", vec![(pred!(mut, ["_cameraMatrix1", "_distCoeffs1", "_cameraMatrix2", "_distCoeffs2", "_cameraMatrix3", "_distCoeffs3", "_imgpt1", "_imgpt3", "imageSize", "_Rmat12", "_Tmat12", "_Rmat13", "_Tmat13", "_Rmat1", "_Rmat2", "_Rmat3", "_Pmat1", "_Pmat2", "_Pmat3", "_Qmat", "alpha", "newImgSize", "roi1", "roi2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Size", "cv::Rect*", "cv::Rect*", "int"]), _)]),
	void cv_rectify3Collinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double_Size_RectX_RectX_int(const cv::_InputArray* _cameraMatrix1, const cv::_InputArray* _distCoeffs1, const cv::_InputArray* _cameraMatrix2, const cv::_InputArray* _distCoeffs2, const cv::_InputArray* _cameraMatrix3, const cv::_InputArray* _distCoeffs3, const cv::_InputArray* _imgpt1, const cv::_InputArray* _imgpt3, cv::Size* imageSize, const cv::_InputArray* _Rmat12, const cv::_InputArray* _Tmat12, const cv::_InputArray* _Rmat13, const cv::_InputArray* _Tmat13, const cv::_OutputArray* _Rmat1, const cv::_OutputArray* _Rmat2, const cv::_OutputArray* _Rmat3, const cv::_OutputArray* _Pmat1, const cv::_OutputArray* _Pmat2, const cv::_OutputArray* _Pmat3, const cv::_OutputArray* _Qmat, double alpha, cv::Size* newImgSize, cv::Rect* roi1, cv::Rect* roi2, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = cv::rectify3Collinear(*_cameraMatrix1, *_distCoeffs1, *_cameraMatrix2, *_distCoeffs2, *_cameraMatrix3, *_distCoeffs3, *_imgpt1, *_imgpt3, *imageSize, *_Rmat12, *_Tmat12, *_Rmat13, *_Tmat13, *_Rmat1, *_Rmat2, *_Rmat3, *_Pmat1, *_Pmat2, *_Pmat3, *_Qmat, alpha, *newImgSize, roi1, roi2, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:482
	// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* disparity, const cv::_OutputArray* _3dImage, const cv::_InputArray* Q, ResultVoid* ocvrs_return) {
		try {
			cv::reprojectImageTo3D(*disparity, *_3dImage, *Q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reprojectImageTo3D(InputArray, OutputArray, InputArray, bool, int)(InputArray, OutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:482
	// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q", "handleMissingValues", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "bool", "int"]), _)]),
	void cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_int(const cv::_InputArray* disparity, const cv::_OutputArray* _3dImage, const cv::_InputArray* Q, bool handleMissingValues, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::reprojectImageTo3D(*disparity, *_3dImage, *Q, handleMissingValues, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoRectifyUncalibrated(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:182
	// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* F, cv::Size* imgSize, const cv::_OutputArray* H1, const cv::_OutputArray* H2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::stereoRectifyUncalibrated(*points1, *points2, *F, *imgSize, *H1, *H2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectifyUncalibrated(InputArray, InputArray, InputArray, Size, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:182
	// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2", "threshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* points1, const cv::_InputArray* points2, const cv::_InputArray* F, cv::Size* imgSize, const cv::_OutputArray* H1, const cv::_OutputArray* H2, double threshold, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::stereoRectifyUncalibrated(*points1, *points2, *F, *imgSize, *H1, *H2, threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:144
	// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, ResultVoid* ocvrs_return) {
		try {
			cv::stereoRectify(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *R1, *R2, *P1, *P2, *Q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stereoRectify(InputArray, InputArray, InputArray, InputArray, Size, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, double, Size, Rect *, Rect *)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:144
	// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q", "flags", "alpha", "newImageSize", "validPixROI1", "validPixROI2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "cv::Size", "cv::Rect*", "cv::Rect*"]), _)]),
	void cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_Size_RectX_RectX(const cv::_InputArray* cameraMatrix1, const cv::_InputArray* distCoeffs1, const cv::_InputArray* cameraMatrix2, const cv::_InputArray* distCoeffs2, cv::Size* imageSize, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* R1, const cv::_OutputArray* R2, const cv::_OutputArray* P1, const cv::_OutputArray* P2, const cv::_OutputArray* Q, int flags, double alpha, cv::Size* newImageSize, cv::Rect* validPixROI1, cv::Rect* validPixROI2, ResultVoid* ocvrs_return) {
		try {
			cv::stereoRectify(*cameraMatrix1, *distCoeffs1, *cameraMatrix2, *distCoeffs2, *imageSize, *R, *T, *R1, *R2, *P1, *P2, *Q, flags, alpha, *newImageSize, validPixROI1, validPixROI2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::validateDisparity(InputOutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:439
	// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int(const cv::_InputOutputArray* disparity, const cv::_InputArray* cost, int minDisparity, int numberOfDisparities, ResultVoid* ocvrs_return) {
		try {
			cv::validateDisparity(*disparity, *cost, minDisparity, numberOfDisparities);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// validateDisparity(InputOutputArray, InputArray, int, int, int)(InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:439
	// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities", "disp12MaxDisp"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
	void cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int_int(const cv::_InputOutputArray* disparity, const cv::_InputArray* cost, int minDisparity, int numberOfDisparities, int disp12MaxDisp, ResultVoid* ocvrs_return) {
		try {
			cv::validateDisparity(*disparity, *cost, minDisparity, numberOfDisparities, disp12MaxDisp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPreFilterType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:289
	// ("cv::StereoBM::getPreFilterType", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getPreFilterType_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:290
	// ("cv::StereoBM::setPreFilterType", vec![(pred!(mut, ["preFilterType"], ["int"]), _)]),
	void cv_StereoBM_setPreFilterType_int(cv::StereoBM* instance, int preFilterType, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterType(preFilterType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPreFilterSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:292
	// ("cv::StereoBM::getPreFilterSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getPreFilterSize_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:293
	// ("cv::StereoBM::setPreFilterSize", vec![(pred!(mut, ["preFilterSize"], ["int"]), _)]),
	void cv_StereoBM_setPreFilterSize_int(cv::StereoBM* instance, int preFilterSize, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterSize(preFilterSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:295
	// ("cv::StereoBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getPreFilterCap_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterCap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:296
	// ("cv::StereoBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	void cv_StereoBM_setPreFilterCap_int(cv::StereoBM* instance, int preFilterCap, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterCap(preFilterCap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTextureThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:298
	// ("cv::StereoBM::getTextureThreshold", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getTextureThreshold_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTextureThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTextureThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:299
	// ("cv::StereoBM::setTextureThreshold", vec![(pred!(mut, ["textureThreshold"], ["int"]), _)]),
	void cv_StereoBM_setTextureThreshold_int(cv::StereoBM* instance, int textureThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setTextureThreshold(textureThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:301
	// ("cv::StereoBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getUniquenessRatio_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getUniquenessRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:302
	// ("cv::StereoBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	void cv_StereoBM_setUniquenessRatio_int(cv::StereoBM* instance, int uniquenessRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmallerBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:304
	// ("cv::StereoBM::getSmallerBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getSmallerBlockSize_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmallerBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmallerBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:305
	// ("cv::StereoBM::setSmallerBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	void cv_StereoBM_setSmallerBlockSize_int(cv::StereoBM* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSmallerBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getROI1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:307
	// ("cv::StereoBM::getROI1", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getROI1_const(const cv::StereoBM* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setROI1(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:308
	// ("cv::StereoBM::setROI1", vec![(pred!(mut, ["roi1"], ["cv::Rect"]), _)]),
	void cv_StereoBM_setROI1_Rect(cv::StereoBM* instance, cv::Rect* roi1, ResultVoid* ocvrs_return) {
		try {
			instance->setROI1(*roi1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getROI2()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:310
	// ("cv::StereoBM::getROI2", vec![(pred!(const, [], []), _)]),
	void cv_StereoBM_getROI2_const(const cv::StereoBM* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setROI2(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:311
	// ("cv::StereoBM::setROI2", vec![(pred!(mut, ["roi2"], ["cv::Rect"]), _)]),
	void cv_StereoBM_setROI2_Rect(cv::StereoBM* instance, cv::Rect* roi2, ResultVoid* ocvrs_return) {
		try {
			instance->setROI2(*roi2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:326
	// ("cv::StereoBM::create", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
	void cv_StereoBM_create_int_int(int numDisparities, int blockSize, Result<cv::Ptr<cv::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create(numDisparities, blockSize);
			Ok(new cv::Ptr<cv::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoBM::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:326
	// ("cv::StereoBM::create", vec![(pred!(mut, [], []), _)]),
	void cv_StereoBM_create(Result<cv::Ptr<cv::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create();
			Ok(new cv::Ptr<cv::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoBM::to_Algorithm() generated
	// ("cv::StereoBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_StereoBM_to_Algorithm(cv::StereoBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::StereoBM::to_StereoMatcher() generated
	// ("cv::StereoBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_StereoBM_to_StereoMatcher(cv::StereoBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::StereoBM::delete() generated
	// ("cv::StereoBM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereoBM_delete(cv::StereoBM* instance) {
			delete instance;
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:256
	// ("cv::StereoMatcher::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::StereoMatcher* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDisparity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:259
	// ("cv::StereoMatcher::getMinDisparity", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getMinDisparity_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinDisparity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDisparity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:260
	// ("cv::StereoMatcher::setMinDisparity", vec![(pred!(mut, ["minDisparity"], ["int"]), _)]),
	void cv_StereoMatcher_setMinDisparity_int(cv::StereoMatcher* instance, int minDisparity, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDisparity(minDisparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:262
	// ("cv::StereoMatcher::getNumDisparities", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getNumDisparities_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumDisparities();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:263
	// ("cv::StereoMatcher::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
	void cv_StereoMatcher_setNumDisparities_int(cv::StereoMatcher* instance, int numDisparities, ResultVoid* ocvrs_return) {
		try {
			instance->setNumDisparities(numDisparities);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:265
	// ("cv::StereoMatcher::getBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getBlockSize_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:266
	// ("cv::StereoMatcher::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	void cv_StereoMatcher_setBlockSize_int(cv::StereoMatcher* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSpeckleWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:268
	// ("cv::StereoMatcher::getSpeckleWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getSpeckleWindowSize_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSpeckleWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSpeckleWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:269
	// ("cv::StereoMatcher::setSpeckleWindowSize", vec![(pred!(mut, ["speckleWindowSize"], ["int"]), _)]),
	void cv_StereoMatcher_setSpeckleWindowSize_int(cv::StereoMatcher* instance, int speckleWindowSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSpeckleWindowSize(speckleWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSpeckleRange()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:271
	// ("cv::StereoMatcher::getSpeckleRange", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getSpeckleRange_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSpeckleRange();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSpeckleRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:272
	// ("cv::StereoMatcher::setSpeckleRange", vec![(pred!(mut, ["speckleRange"], ["int"]), _)]),
	void cv_StereoMatcher_setSpeckleRange_int(cv::StereoMatcher* instance, int speckleRange, ResultVoid* ocvrs_return) {
		try {
			instance->setSpeckleRange(speckleRange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDisp12MaxDiff()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:274
	// ("cv::StereoMatcher::getDisp12MaxDiff", vec![(pred!(const, [], []), _)]),
	void cv_StereoMatcher_getDisp12MaxDiff_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDisp12MaxDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDisp12MaxDiff(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:275
	// ("cv::StereoMatcher::setDisp12MaxDiff", vec![(pred!(mut, ["disp12MaxDiff"], ["int"]), _)]),
	void cv_StereoMatcher_setDisp12MaxDiff_int(cv::StereoMatcher* instance, int disp12MaxDiff, ResultVoid* ocvrs_return) {
		try {
			instance->setDisp12MaxDiff(disp12MaxDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoMatcher::to_StereoBM() generated
	// ("cv::StereoMatcher::to_StereoBM", vec![(pred!(mut, [], []), _)]),
	cv::StereoBM* cv_StereoMatcher_to_StereoBM(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::StereoBM*>(instance);
	}

	// cv::StereoMatcher::to_StereoSGBM() generated
	// ("cv::StereoMatcher::to_StereoSGBM", vec![(pred!(mut, [], []), _)]),
	cv::StereoSGBM* cv_StereoMatcher_to_StereoSGBM(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::StereoSGBM*>(instance);
	}

	// cv::StereoMatcher::to_Algorithm() generated
	// ("cv::StereoMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_StereoMatcher_to_Algorithm(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::StereoMatcher::delete() generated
	// ("cv::StereoMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereoMatcher_delete(cv::StereoMatcher* instance) {
			delete instance;
	}

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:358
	// ("cv::StereoSGBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getPreFilterCap_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterCap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:359
	// ("cv::StereoSGBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	void cv_StereoSGBM_setPreFilterCap_int(cv::StereoSGBM* instance, int preFilterCap, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterCap(preFilterCap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:361
	// ("cv::StereoSGBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getUniquenessRatio_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getUniquenessRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:362
	// ("cv::StereoSGBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	void cv_StereoSGBM_setUniquenessRatio_int(cv::StereoSGBM* instance, int uniquenessRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getP1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:364
	// ("cv::StereoSGBM::getP1", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getP1_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getP1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setP1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:365
	// ("cv::StereoSGBM::setP1", vec![(pred!(mut, ["P1"], ["int"]), _)]),
	void cv_StereoSGBM_setP1_int(cv::StereoSGBM* instance, int P1, ResultVoid* ocvrs_return) {
		try {
			instance->setP1(P1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getP2()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:367
	// ("cv::StereoSGBM::getP2", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getP2_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getP2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setP2(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:368
	// ("cv::StereoSGBM::setP2", vec![(pred!(mut, ["P2"], ["int"]), _)]),
	void cv_StereoSGBM_setP2_int(cv::StereoSGBM* instance, int P2, ResultVoid* ocvrs_return) {
		try {
			instance->setP2(P2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:370
	// ("cv::StereoSGBM::getMode", vec![(pred!(const, [], []), _)]),
	void cv_StereoSGBM_getMode_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:371
	// ("cv::StereoSGBM::setMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	void cv_StereoSGBM_setMode_int(cv::StereoSGBM* instance, int mode, ResultVoid* ocvrs_return) {
		try {
			instance->setMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, int, int, int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:410
	// ("cv::StereoSGBM::create", vec![(pred!(mut, ["minDisparity", "numDisparities", "blockSize", "P1", "P2", "disp12MaxDiff", "preFilterCap", "uniquenessRatio", "speckleWindowSize", "speckleRange", "mode"], ["int", "int", "int", "int", "int", "int", "int", "int", "int", "int", "int"]), _)]),
	void cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(int minDisparity, int numDisparities, int blockSize, int P1, int P2, int disp12MaxDiff, int preFilterCap, int uniquenessRatio, int speckleWindowSize, int speckleRange, int mode, Result<cv::Ptr<cv::StereoSGBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create(minDisparity, numDisparities, blockSize, P1, P2, disp12MaxDiff, preFilterCap, uniquenessRatio, speckleWindowSize, speckleRange, mode);
			Ok(new cv::Ptr<cv::StereoSGBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoSGBM::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/stereo.hpp:410
	// ("cv::StereoSGBM::create", vec![(pred!(mut, [], []), _)]),
	void cv_StereoSGBM_create(Result<cv::Ptr<cv::StereoSGBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create();
			Ok(new cv::Ptr<cv::StereoSGBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::StereoSGBM::to_Algorithm() generated
	// ("cv::StereoSGBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_StereoSGBM_to_Algorithm(cv::StereoSGBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::StereoSGBM::to_StereoMatcher() generated
	// ("cv::StereoSGBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_StereoSGBM_to_StereoMatcher(cv::StereoSGBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::StereoSGBM::delete() generated
	// ("cv::StereoSGBM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_StereoSGBM_delete(cv::StereoSGBM* instance) {
			delete instance;
	}

}
