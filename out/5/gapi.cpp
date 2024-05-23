#include "gapi.hpp"
#include "gapi_types.hpp"

extern "C" {
	// descr_of(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:281
	// ("cv::descr_of", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
	void cv_descr_of_const_MatR(const cv::Mat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descr_of(const MediaFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:107
	// ("cv::descr_of", vec![(pred!(mut, ["frame"], ["const cv::MediaFrame*"]), _)]),
	void cv_descr_of_const_MediaFrameR(const cv::MediaFrame* frame, Result<cv::GFrameDesc*>* ocvrs_return) {
		try {
			cv::GFrameDesc ret = cv::descr_of(*frame);
			Ok(new cv::GFrameDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descr_of(const RMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:278
	// ("cv::descr_of", vec![(pred!(mut, ["mat"], ["const cv::RMat*"]), _)]),
	void cv_descr_of_const_RMatR(const cv::RMat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descr_of(const cv::Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:134
	// ("cv::descr_of", vec![(pred!(mut, ["scalar"], ["const cv::Scalar*"]), _)]),
	void cv_descr_of_const_ScalarR(const cv::Scalar* scalar, Result<cv::GScalarDesc*>* ocvrs_return) {
		try {
			cv::GScalarDesc ret = cv::descr_of(*scalar);
			Ok(new cv::GScalarDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descr_of(const cv::UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:269
	// ("cv::descr_of", vec![(pred!(mut, ["mat"], ["const cv::UMat*"]), _)]),
	void cv_descr_of_const_UMatR(const cv::UMat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty_array_desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garray.hpp:45
	// ("cv::empty_array_desc", vec![(pred!(mut, [], []), _)]),
	void cv_empty_array_desc(Result<cv::GArrayDesc*>* ocvrs_return) {
		try {
			cv::GArrayDesc ret = cv::empty_array_desc();
			Ok(new cv::GArrayDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty_gopaque_desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gopaque.hpp:46
	// ("cv::empty_gopaque_desc", vec![(pred!(mut, [], []), _)]),
	void cv_empty_gopaque_desc(Result<cv::GOpaqueDesc*>* ocvrs_return) {
		try {
			cv::GOpaqueDesc ret = cv::empty_gopaque_desc();
			Ok(new cv::GOpaqueDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty_scalar_desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:132
	// ("cv::empty_scalar_desc", vec![(pred!(mut, [], []), _)]),
	void cv_empty_scalar_desc(Result<cv::GScalarDesc*>* ocvrs_return) {
		try {
			cv::GScalarDesc ret = cv::empty_scalar_desc();
			Ok(new cv::GScalarDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BGR2Gray(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1415
	// ("cv::gapi::BGR2Gray", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_BGR2Gray_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2Gray(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BGR2I420(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1448
	// ("cv::gapi::BGR2I420", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_BGR2I420_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2I420(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BGR2LUV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1510
	// ("cv::gapi::BGR2LUV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_BGR2LUV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2LUV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BGR2RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1375
	// ("cv::gapi::BGR2RGB", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_BGR2RGB_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2RGB(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BGR2YUV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1552
	// ("cv::gapi::BGR2YUV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_BGR2YUV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2YUV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BayerGR2RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1639
	// ("cv::gapi::BayerGR2RGB", vec![(pred!(mut, ["src_gr"], ["const cv::GMat*"]), _)]),
	void cv_gapi_BayerGR2RGB_const_GMatR(const cv::GMat* src_gr, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BayerGR2RGB(*src_gr);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::Canny(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1026
	// ("cv::gapi::Canny", vec![(pred!(mut, ["image", "threshold1", "threshold2"], ["const cv::GMat*", "double", "double"]), _)]),
	void cv_gapi_Canny_const_GMatR_double_double(const cv::GMat* image, double threshold1, double threshold2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Canny(*image, threshold1, threshold2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Canny(const GMat &, double, double, int, bool)(TraitClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1026
	// ("cv::gapi::Canny", vec![(pred!(mut, ["image", "threshold1", "threshold2", "apertureSize", "L2gradient"], ["const cv::GMat*", "double", "double", "int", "bool"]), _)]),
	void cv_gapi_Canny_const_GMatR_double_double_int_bool(const cv::GMat* image, double threshold1, double threshold2, int apertureSize, bool L2gradient, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Canny(*image, threshold1, threshold2, apertureSize, L2gradient);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// I4202BGR(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1480
	// ("cv::gapi::I4202BGR", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_I4202BGR_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::I4202BGR(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// I4202RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1496
	// ("cv::gapi::I4202RGB", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_I4202RGB_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::I4202RGB(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LUT(const GMat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1699
	// ("cv::gapi::LUT", vec![(pred!(mut, ["src", "lut"], ["const cv::GMat*", "const cv::Mat*"]), _)]),
	void cv_gapi_LUT_const_GMatR_const_MatR(const cv::GMat* src, const cv::Mat* lut, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::LUT(*src, *lut);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LUV2BGR(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1524
	// ("cv::gapi::LUV2BGR", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_LUV2BGR_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::LUV2BGR(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::Laplacian(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:967
	// ("cv::gapi::Laplacian", vec![(pred!(mut, ["src", "ddepth"], ["const cv::GMat*", "int"]), _)]),
	void cv_gapi_Laplacian_const_GMatR_int(const cv::GMat* src, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Laplacian(*src, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Laplacian(const GMat &, int, int, double, double, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:967
	// ("cv::gapi::Laplacian", vec![(pred!(mut, ["src", "ddepth", "ksize", "scale", "delta", "borderType"], ["const cv::GMat*", "int", "int", "double", "double", "int"]), _)]),
	void cv_gapi_Laplacian_const_GMatR_int_int_double_double_int(const cv::GMat* src, int ddepth, int ksize, double scale, double delta, int borderType, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Laplacian(*src, ddepth, ksize, scale, delta, borderType);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NV12toBGR(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1625
	// ("cv::gapi::NV12toBGR", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_NV12toBGR_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::NV12toBGR(*src_y, *src_uv);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NV12toBGRp(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1703
	// ("cv::gapi::NV12toBGRp", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_NV12toBGRp_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::NV12toBGRp(*src_y, *src_uv);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NV12toGray(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1610
	// ("cv::gapi::NV12toGray", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_NV12toGray_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::NV12toGray(*src_y, *src_uv);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NV12toRGB(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1595
	// ("cv::gapi::NV12toRGB", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_NV12toRGB_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::NV12toRGB(*src_y, *src_uv);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NV12toRGBp(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1685
	// ("cv::gapi::NV12toRGBp", vec![(pred!(mut, ["src_y", "src_uv"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_NV12toRGBp_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::NV12toRGBp(*src_y, *src_uv);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2Gray(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1388
	// ("cv::gapi::RGB2Gray", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_RGB2Gray_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2Gray(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2Gray(const GMat &, float, float, float)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1402
	// ("cv::gapi::RGB2Gray", vec![(pred!(mut, ["src", "rY", "gY", "bY"], ["const cv::GMat*", "float", "float", "float"]), _)]),
	void cv_gapi_RGB2Gray_const_GMatR_float_float_float(const cv::GMat* src, float rY, float gY, float bY, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2Gray(*src, rY, gY, bY);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2HSV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1653
	// ("cv::gapi::RGB2HSV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_RGB2HSV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2HSV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2I420(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1464
	// ("cv::gapi::RGB2I420", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_RGB2I420_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2I420(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2Lab(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1566
	// ("cv::gapi::RGB2Lab", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_RGB2Lab_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2Lab(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2YUV422(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1667
	// ("cv::gapi::RGB2YUV422", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_RGB2YUV422_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2YUV422(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RGB2YUV(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1432
	// ("cv::gapi::RGB2YUV", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_RGB2YUV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2YUV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::SobelXY(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:937
	// ("cv::gapi::SobelXY", vec![(pred!(mut, ["src", "ddepth", "order"], ["const cv::GMat*", "int", "int"]), _)]),
	void cv_gapi_SobelXY_const_GMatR_int_int(const cv::GMat* src, int ddepth, int order, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::SobelXY(*src, ddepth, order);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SobelXY(const GMat &, int, int, int, double, double, int, const Scalar &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:937
	// ("cv::gapi::SobelXY", vec![(pred!(mut, ["src", "ddepth", "order", "ksize", "scale", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "int", "double", "double", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_SobelXY_const_GMatR_int_int_int_double_double_int_const_ScalarR(const cv::GMat* src, int ddepth, int order, int ksize, double scale, double delta, int borderType, const cv::Scalar* borderValue, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::SobelXY(*src, ddepth, order, ksize, scale, delta, borderType, *borderValue);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::Sobel(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:886
	// ("cv::gapi::Sobel", vec![(pred!(mut, ["src", "ddepth", "dx", "dy"], ["const cv::GMat*", "int", "int", "int"]), _)]),
	void cv_gapi_Sobel_const_GMatR_int_int_int(const cv::GMat* src, int ddepth, int dx, int dy, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Sobel(*src, ddepth, dx, dy);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Sobel(const GMat &, int, int, int, int, double, double, int, const Scalar &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:886
	// ("cv::gapi::Sobel", vec![(pred!(mut, ["src", "ddepth", "dx", "dy", "ksize", "scale", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "int", "int", "double", "double", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_Sobel_const_GMatR_int_int_int_int_double_double_int_const_ScalarR(const cv::GMat* src, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Sobel(*src, ddepth, dx, dy, ksize, scale, delta, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// YUV2BGR(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1538
	// ("cv::gapi::YUV2BGR", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_YUV2BGR_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::YUV2BGR(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// YUV2RGB(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1580
	// ("cv::gapi::YUV2RGB", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_YUV2RGB_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::YUV2RGB(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// absDiffC(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1254
	// ("cv::gapi::absDiffC", vec![(pred!(mut, ["src", "c"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_absDiffC_const_GMatR_const_GScalarR(const cv::GMat* src, const cv::GScalar* c, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::absDiffC(*src, *c);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// absDiff(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1237
	// ("cv::gapi::absDiff", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_absDiff_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::absDiff(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::addC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:636
	// ("cv::gapi::addC", vec![(pred!(mut, ["src1", "c"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_addC_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* c, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addC(*src1, *c);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addC(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:636
	// ("cv::gapi::addC", vec![(pred!(mut, ["src1", "c", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
	void cv_gapi_addC_const_GMatR_const_GScalarR_int(const cv::GMat* src1, const cv::GScalar* c, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addC(*src1, *c, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::addC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:638
	// ("cv::gapi::addC", vec![(pred!(mut, ["c", "src1"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_gapi_addC_const_GScalarR_const_GMatR(const cv::GScalar* c, const cv::GMat* src1, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addC(*c, *src1);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addC(const GScalar &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:638
	// ("cv::gapi::addC", vec![(pred!(mut, ["c", "src1", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "int"]), _)]),
	void cv_gapi_addC_const_GScalarR_const_GMatR_int(const cv::GScalar* c, const cv::GMat* src1, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addC(*c, *src1, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::addWeighted(TraitClass, Primitive, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1302
	// ("cv::gapi::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma"], ["const cv::GMat*", "double", "const cv::GMat*", "double", "double"]), _)]),
	void cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double(const cv::GMat* src1, double alpha, const cv::GMat* src2, double beta, double gamma, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addWeighted(*src1, alpha, *src2, beta, gamma);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addWeighted(const GMat &, double, const GMat &, double, double, int)(TraitClass, Primitive, TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1302
	// ("cv::gapi::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "ddepth"], ["const cv::GMat*", "double", "const cv::GMat*", "double", "double", "int"]), _)]),
	void cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double_int(const cv::GMat* src1, double alpha, const cv::GMat* src2, double beta, double gamma, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addWeighted(*src1, alpha, *src2, beta, gamma, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::add(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:615
	// ("cv::gapi::add", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_add_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::add(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(const GMat &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:615
	// ("cv::gapi::add", vec![(pred!(mut, ["src1", "src2", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "int"]), _)]),
	void cv_gapi_add_const_GMatR_const_GMatR_int(const cv::GMat* src1, const cv::GMat* src2, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::add(*src1, *src2, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::bilateralFilter(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1001
	// ("cv::gapi::bilateralFilter", vec![(pred!(mut, ["src", "d", "sigmaColor", "sigmaSpace"], ["const cv::GMat*", "int", "double", "double"]), _)]),
	void cv_gapi_bilateralFilter_const_GMatR_int_double_double(const cv::GMat* src, int d, double sigmaColor, double sigmaSpace, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bilateralFilter(*src, d, sigmaColor, sigmaSpace);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bilateralFilter(const GMat &, int, double, double, int)(TraitClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1001
	// ("cv::gapi::bilateralFilter", vec![(pred!(mut, ["src", "d", "sigmaColor", "sigmaSpace", "borderType"], ["const cv::GMat*", "int", "double", "double", "int"]), _)]),
	void cv_gapi_bilateralFilter_const_GMatR_int_double_double_int(const cv::GMat* src, int d, double sigmaColor, double sigmaSpace, int borderType, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bilateralFilter(*src, d, sigmaColor, sigmaSpace, borderType);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_and(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1089
	// ("cv::gapi::bitwise_and", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_bitwise_and_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_and(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_and(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1095
	// ("cv::gapi::bitwise_and", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_bitwise_and_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_and(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_not(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1165
	// ("cv::gapi::bitwise_not", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_bitwise_not_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_not(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_or(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1113
	// ("cv::gapi::bitwise_or", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_bitwise_or_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_or(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_or(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1119
	// ("cv::gapi::bitwise_or", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_bitwise_or_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_or(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_xor(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1138
	// ("cv::gapi::bitwise_xor", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_bitwise_xor_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_xor(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_xor(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1144
	// ("cv::gapi::bitwise_xor", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_bitwise_xor_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_xor(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::blur(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:657
	// ("cv::gapi::blur", vec![(pred!(mut, ["src", "ksize"], ["const cv::GMat*", "const cv::Size*"]), _)]),
	void cv_gapi_blur_const_GMatR_const_SizeR(const cv::GMat* src, const cv::Size* ksize, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::blur(*src, *ksize);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blur(const GMat &, const Size &, const Point &, int, const Scalar &)(TraitClass, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:657
	// ("cv::gapi::blur", vec![(pred!(mut, ["src", "ksize", "anchor", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Size*", "const cv::Point*", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_blur_const_GMatR_const_SizeR_const_PointR_int_const_ScalarR(const cv::GMat* src, const cv::Size* ksize, const cv::Point* anchor, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::blur(*src, *ksize, *anchor, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::boxFilter(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:630
	// ("cv::gapi::boxFilter", vec![(pred!(mut, ["src", "dtype", "ksize"], ["const cv::GMat*", "int", "const cv::Size*"]), _)]),
	void cv_gapi_boxFilter_const_GMatR_int_const_SizeR(const cv::GMat* src, int dtype, const cv::Size* ksize, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::boxFilter(*src, dtype, *ksize);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// boxFilter(const GMat &, int, const Size &, const Point &, bool, int, const Scalar &)(TraitClass, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:630
	// ("cv::gapi::boxFilter", vec![(pred!(mut, ["src", "dtype", "ksize", "anchor", "normalize", "borderType", "borderValue"], ["const cv::GMat*", "int", "const cv::Size*", "const cv::Point*", "bool", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_boxFilter_const_GMatR_int_const_SizeR_const_PointR_bool_int_const_ScalarR(const cv::GMat* src, int dtype, const cv::Size* ksize, const cv::Point* anchor, bool normalize, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::boxFilter(*src, dtype, *ksize, *anchor, normalize, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::cartToPolar(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:881
	// ("cv::gapi::cartToPolar", vec![(pred!(mut, ["x", "y"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cartToPolar_const_GMatR_const_GMatR(const cv::GMat* x, const cv::GMat* y, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::cartToPolar(*x, *y);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cartToPolar(const GMat &, const GMat &, bool)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:881
	// ("cv::gapi::cartToPolar", vec![(pred!(mut, ["x", "y", "angleInDegrees"], ["const cv::GMat*", "const cv::GMat*", "bool"]), _)]),
	void cv_gapi_cartToPolar_const_GMatR_const_GMatR_bool(const cv::GMat* x, const cv::GMat* y, bool angleInDegrees, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::cartToPolar(*x, *y, angleInDegrees);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpEQ(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1041
	// ("cv::gapi::cmpEQ", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cmpEQ_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpEQ(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpEQ(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1045
	// ("cv::gapi::cmpEQ", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_cmpEQ_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpEQ(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpGE(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:989
	// ("cv::gapi::cmpGE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cmpGE_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpGE(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:993
	// ("cv::gapi::cmpGE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_cmpGE_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpGT(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:937
	// ("cv::gapi::cmpGT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cmpGT_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpGT(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:941
	// ("cv::gapi::cmpGT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_cmpGT_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpLE(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1015
	// ("cv::gapi::cmpLE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cmpLE_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpLE(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1019
	// ("cv::gapi::cmpLE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_cmpLE_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpLT(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:963
	// ("cv::gapi::cmpLT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cmpLT_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpLT(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:967
	// ("cv::gapi::cmpLT", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_cmpLT_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpNE(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1067
	// ("cv::gapi::cmpNE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_cmpNE_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpNE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cmpNE(const GMat &, const GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1071
	// ("cv::gapi::cmpNE", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_cmpNE_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpNE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// combine(const cv::GKernelPackage &, const cv::GKernelPackage &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:420
	// ("cv::gapi::combine", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GKernelPackage*", "const cv::GKernelPackage*"]), _)]),
	void cv_gapi_combine_const_GKernelPackageR_const_GKernelPackageR(const cv::GKernelPackage* lhs, const cv::GKernelPackage* rhs, Result<cv::GKernelPackage*>* ocvrs_return) {
		try {
			cv::GKernelPackage ret = cv::gapi::combine(*lhs, *rhs);
			Ok(new cv::GKernelPackage(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concatHor(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1632
	// ("cv::gapi::concatHor", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_concatHor_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatHor(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concatHor(const std::vector<GMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1640
	// ("cv::gapi::concatHor", vec![(pred!(mut, ["v"], ["const std::vector<cv::GMat>*"]), _)]),
	void cv_gapi_concatHor_const_vectorLGMatGR(const std::vector<cv::GMat>* v, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatHor(*v);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concatVert(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1672
	// ("cv::gapi::concatVert", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_concatVert_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatVert(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concatVert(const std::vector<GMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1680
	// ("cv::gapi::concatVert", vec![(pred!(mut, ["v"], ["const std::vector<cv::GMat>*"]), _)]),
	void cv_gapi_concatVert_const_vectorLGMatGR(const std::vector<cv::GMat>* v, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatVert(*v);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1716
	// ("cv::gapi::convertTo", vec![(pred!(mut, ["src", "rdepth"], ["const cv::GMat*", "int"]), _)]),
	void cv_gapi_convertTo_const_GMatR_int(const cv::GMat* src, int rdepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::convertTo(*src, rdepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(const GMat &, int, double, double)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1716
	// ("cv::gapi::convertTo", vec![(pred!(mut, ["src", "rdepth", "alpha", "beta"], ["const cv::GMat*", "int", "double", "double"]), _)]),
	void cv_gapi_convertTo_const_GMatR_int_double_double(const cv::GMat* src, int rdepth, double alpha, double beta, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::convertTo(*src, rdepth, alpha, beta);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copy(const GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:88
	// ("cv::gapi::copy", vec![(pred!(mut, ["in"], ["const cv::GFrame*"]), _)]),
	void cv_gapi_copy_const_GFrameR(const cv::GFrame* in, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = cv::gapi::copy(*in);
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copy(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:77
	// ("cv::gapi::copy", vec![(pred!(mut, ["in"], ["const cv::GMat*"]), _)]),
	void cv_gapi_copy_const_GMatR(const cv::GMat* in, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::copy(*in);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// crop(const GMat &, const Rect &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1604
	// ("cv::gapi::crop", vec![(pred!(mut, ["src", "rect"], ["const cv::GMat*", "const cv::Rect*"]), _)]),
	void cv_gapi_crop_const_GMatR_const_RectR(const cv::GMat* src, const cv::Rect* rect, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::crop(*src, *rect);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::dilate3x3(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:804
	// ("cv::gapi::dilate3x3", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_dilate3x3_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::dilate3x3(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dilate3x3(const GMat &, int, int, const Scalar &)(TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:804
	// ("cv::gapi::dilate3x3", vec![(pred!(mut, ["src", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_dilate3x3_const_GMatR_int_int_const_ScalarR(const cv::GMat* src, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::dilate3x3(*src, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::dilate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:780
	// ("cv::gapi::dilate", vec![(pred!(mut, ["src", "kernel"], ["const cv::GMat*", "const cv::Mat*"]), _)]),
	void cv_gapi_dilate_const_GMatR_const_MatR(const cv::GMat* src, const cv::Mat* kernel, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::dilate(*src, *kernel);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dilate(const GMat &, const Mat &, const Point &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:780
	// ("cv::gapi::dilate", vec![(pred!(mut, ["src", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Point*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_dilate_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::dilate(*src, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::divC(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:788
	// ("cv::gapi::divC", vec![(pred!(mut, ["src", "divisor", "scale"], ["const cv::GMat*", "const cv::GScalar*", "double"]), _)]),
	void cv_gapi_divC_const_GMatR_const_GScalarR_double(const cv::GMat* src, const cv::GScalar* divisor, double scale, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::divC(*src, *divisor, scale);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divC(const GMat &, const GScalar &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:788
	// ("cv::gapi::divC", vec![(pred!(mut, ["src", "divisor", "scale", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "double", "int"]), _)]),
	void cv_gapi_divC_const_GMatR_const_GScalarR_double_int(const cv::GMat* src, const cv::GScalar* divisor, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::divC(*src, *divisor, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::divRC(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:809
	// ("cv::gapi::divRC", vec![(pred!(mut, ["divident", "src", "scale"], ["const cv::GScalar*", "const cv::GMat*", "double"]), _)]),
	void cv_gapi_divRC_const_GScalarR_const_GMatR_double(const cv::GScalar* divident, const cv::GMat* src, double scale, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::divRC(*divident, *src, scale);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divRC(const GScalar &, const GMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:809
	// ("cv::gapi::divRC", vec![(pred!(mut, ["divident", "src", "scale", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "double", "int"]), _)]),
	void cv_gapi_divRC_const_GScalarR_const_GMatR_double_int(const cv::GScalar* divident, const cv::GMat* src, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::divRC(*divident, *src, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::div(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:767
	// ("cv::gapi::div", vec![(pred!(mut, ["src1", "src2", "scale"], ["const cv::GMat*", "const cv::GMat*", "double"]), _)]),
	void cv_gapi_div_const_GMatR_const_GMatR_double(const cv::GMat* src1, const cv::GMat* src2, double scale, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::div(*src1, *src2, scale);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// div(const GMat &, const GMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:767
	// ("cv::gapi::div", vec![(pred!(mut, ["src1", "src2", "scale", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "double", "int"]), _)]),
	void cv_gapi_div_const_GMatR_const_GMatR_double_int(const cv::GMat* src1, const cv::GMat* src2, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::div(*src1, *src2, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// equalizeHist(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1101
	// ("cv::gapi::equalizeHist", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_equalizeHist_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::equalizeHist(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::erode3x3(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:753
	// ("cv::gapi::erode3x3", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_erode3x3_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::erode3x3(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erode3x3(const GMat &, int, int, const Scalar &)(TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:753
	// ("cv::gapi::erode3x3", vec![(pred!(mut, ["src", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_erode3x3_const_GMatR_int_int_const_ScalarR(const cv::GMat* src, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::erode3x3(*src, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::erode(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:733
	// ("cv::gapi::erode", vec![(pred!(mut, ["src", "kernel"], ["const cv::GMat*", "const cv::Mat*"]), _)]),
	void cv_gapi_erode_const_GMatR_const_MatR(const cv::GMat* src, const cv::Mat* kernel, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::erode(*src, *kernel);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erode(const GMat &, const Mat &, const Point &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:733
	// ("cv::gapi::erode", vec![(pred!(mut, ["src", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Point*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_erode_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::erode(*src, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::filter2D(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:596
	// ("cv::gapi::filter2D", vec![(pred!(mut, ["src", "ddepth", "kernel"], ["const cv::GMat*", "int", "const cv::Mat*"]), _)]),
	void cv_gapi_filter2D_const_GMatR_int_const_MatR(const cv::GMat* src, int ddepth, const cv::Mat* kernel, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::filter2D(*src, ddepth, *kernel);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filter2D(const GMat &, int, const Mat &, const Point &, const Scalar &, int, const Scalar &)(TraitClass, Primitive, TraitClass, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:596
	// ("cv::gapi::filter2D", vec![(pred!(mut, ["src", "ddepth", "kernel", "anchor", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "const cv::Mat*", "const cv::Point*", "const cv::Scalar*", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_filter2D_const_GMatR_int_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(const cv::GMat* src, int ddepth, const cv::Mat* kernel, const cv::Point* anchor, const cv::Scalar* delta, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::filter2D(*src, ddepth, *kernel, *anchor, *delta, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// flip(const GMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1590
	// ("cv::gapi::flip", vec![(pred!(mut, ["src", "flipCode"], ["const cv::GMat*", "int"]), _)]),
	void cv_gapi_flip_const_GMatR_int(const cv::GMat* src, int flipCode, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::flip(*src, flipCode);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::gaussianBlur(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:690
	// ("cv::gapi::gaussianBlur", vec![(pred!(mut, ["src", "ksize", "sigmaX"], ["const cv::GMat*", "const cv::Size*", "double"]), _)]),
	void cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double(const cv::GMat* src, const cv::Size* ksize, double sigmaX, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::gaussianBlur(*src, *ksize, sigmaX);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gaussianBlur(const GMat &, const Size &, double, double, int, const Scalar &)(TraitClass, SimpleClass, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:690
	// ("cv::gapi::gaussianBlur", vec![(pred!(mut, ["src", "ksize", "sigmaX", "sigmaY", "borderType", "borderValue"], ["const cv::GMat*", "const cv::Size*", "double", "double", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double_double_int_const_ScalarR(const cv::GMat* src, const cv::Size* ksize, double sigmaX, double sigmaY, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::gaussianBlur(*src, *ksize, sigmaX, sigmaY, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inRange(const GMat &, const GScalar &, const GScalar &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1441
	// ("cv::gapi::inRange", vec![(pred!(mut, ["src", "threshLow", "threshUp"], ["const cv::GMat*", "const cv::GScalar*", "const cv::GScalar*"]), _)]),
	void cv_gapi_inRange_const_GMatR_const_GScalarR_const_GScalarR(const cv::GMat* src, const cv::GScalar* threshLow, const cv::GScalar* threshUp, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::inRange(*src, *threshLow, *threshUp);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::integral(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1390
	// ("cv::gapi::integral", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_integral_const_GMatR(const cv::GMat* src, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::integral(*src);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integral(const GMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1390
	// ("cv::gapi::integral", vec![(pred!(mut, ["src", "sdepth", "sqdepth"], ["const cv::GMat*", "int", "int"]), _)]),
	void cv_gapi_integral_const_GMatR_int_int(const cv::GMat* src, int sdepth, int sqdepth, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::integral(*src, sdepth, sqdepth);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mask(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:822
	// ("cv::gapi::mask", vec![(pred!(mut, ["src", "mask"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_mask_const_GMatR_const_GMatR(const cv::GMat* src, const cv::GMat* mask, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mask(*src, *mask);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1220
	// ("cv::gapi::max", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_max_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::max(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mean(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:835
	// ("cv::gapi::mean", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_mean_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::mean(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// medianBlur(const GMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:707
	// ("cv::gapi::medianBlur", vec![(pred!(mut, ["src", "ksize"], ["const cv::GMat*", "int"]), _)]),
	void cv_gapi_medianBlur_const_GMatR_int(const cv::GMat* src, int ksize, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::medianBlur(*src, ksize);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// merge3(const GMat &, const GMat &, const GMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1484
	// ("cv::gapi::merge3", vec![(pred!(mut, ["src1", "src2", "src3"], ["const cv::GMat*", "const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_merge3_const_GMatR_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, const cv::GMat* src3, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::merge3(*src1, *src2, *src3);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// merge4(const GMat &, const GMat &, const GMat &, const GMat &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1465
	// ("cv::gapi::merge4", vec![(pred!(mut, ["src1", "src2", "src3", "src4"], ["const cv::GMat*", "const cv::GMat*", "const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_merge4_const_GMatR_const_GMatR_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, const cv::GMat* src3, const cv::GMat* src4, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::merge4(*src1, *src2, *src3, *src4);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(const GMat &, const GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1203
	// ("cv::gapi::min", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_min_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::min(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::morphologyEx(TraitClass, Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:834
	// ("cv::gapi::morphologyEx", vec![(pred!(mut, ["src", "op", "kernel"], ["const cv::GMat*", "const cv::MorphTypes", "const cv::Mat*"]), _)]),
	void cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR(const cv::GMat* src, const cv::MorphTypes op, const cv::Mat* kernel, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::morphologyEx(*src, op, *kernel);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// morphologyEx(const GMat &, const MorphTypes, const Mat &, const Point &, const int, const BorderTypes, const Scalar &)(TraitClass, Enum, TraitClass, SimpleClass, Primitive, Enum, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:834
	// ("cv::gapi::morphologyEx", vec![(pred!(mut, ["src", "op", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::GMat*", "const cv::MorphTypes", "const cv::Mat*", "const cv::Point*", "const int", "const cv::BorderTypes", "const cv::Scalar*"]), _)]),
	void cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR_const_PointR_const_int_const_BorderTypes_const_ScalarR(const cv::GMat* src, const cv::MorphTypes op, const cv::Mat* kernel, const cv::Point* anchor, const int iterations, const cv::BorderTypes borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::morphologyEx(*src, op, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::mulC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:742
	// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_mulC_const_GMatR_const_GScalarR(const cv::GMat* src, const cv::GScalar* multiplier, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*src, *multiplier);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulC(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:742
	// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
	void cv_gapi_mulC_const_GMatR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* multiplier, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*src, *multiplier, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::mulC(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:740
	// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier"], ["const cv::GMat*", "double"]), _)]),
	void cv_gapi_mulC_const_GMatR_double(const cv::GMat* src, double multiplier, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*src, multiplier);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulC(const GMat &, double, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:740
	// ("cv::gapi::mulC", vec![(pred!(mut, ["src", "multiplier", "ddepth"], ["const cv::GMat*", "double", "int"]), _)]),
	void cv_gapi_mulC_const_GMatR_double_int(const cv::GMat* src, double multiplier, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*src, multiplier, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::mulC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:744
	// ("cv::gapi::mulC", vec![(pred!(mut, ["multiplier", "src"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_gapi_mulC_const_GScalarR_const_GMatR(const cv::GScalar* multiplier, const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*multiplier, *src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulC(const GScalar &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:744
	// ("cv::gapi::mulC", vec![(pred!(mut, ["multiplier", "src", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "int"]), _)]),
	void cv_gapi_mulC_const_GScalarR_const_GMatR_int(const cv::GScalar* multiplier, const cv::GMat* src, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*multiplier, *src, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::mul(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:722
	// ("cv::gapi::mul", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_mul_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mul(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mul(const GMat &, const GMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:722
	// ("cv::gapi::mul", vec![(pred!(mut, ["src1", "src2", "scale", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "double", "int"]), _)]),
	void cv_gapi_mul_const_GMatR_const_GMatR_double_int(const cv::GMat* src1, const cv::GMat* src2, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mul(*src1, *src2, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normInf(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1370
	// ("cv::gapi::normInf", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_normInf_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::normInf(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normL1(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1325
	// ("cv::gapi::normL1", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_normL1_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::normL1(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normL2(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1347
	// ("cv::gapi::normL2", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_normL2_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::normL2(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::normalize(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1738
	// ("cv::gapi::normalize", vec![(pred!(mut, ["src", "alpha", "beta", "norm_type"], ["const cv::GMat*", "double", "double", "int"]), _)]),
	void cv_gapi_normalize_const_GMatR_double_double_int(const cv::GMat* src, double alpha, double beta, int norm_type, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::normalize(*src, alpha, beta, norm_type);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalize(const GMat &, double, double, int, int)(TraitClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1738
	// ("cv::gapi::normalize", vec![(pred!(mut, ["src", "alpha", "beta", "norm_type", "ddepth"], ["const cv::GMat*", "double", "double", "int", "int"]), _)]),
	void cv_gapi_normalize_const_GMatR_double_double_int_int(const cv::GMat* src, double alpha, double beta, int norm_type, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::normalize(*src, alpha, beta, norm_type, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::phase(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:899
	// ("cv::gapi::phase", vec![(pred!(mut, ["x", "y"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_phase_const_GMatR_const_GMatR(const cv::GMat* x, const cv::GMat* y, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::phase(*x, *y);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// phase(const GMat &, const GMat &, bool)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:899
	// ("cv::gapi::phase", vec![(pred!(mut, ["x", "y", "angleInDegrees"], ["const cv::GMat*", "const cv::GMat*", "bool"]), _)]),
	void cv_gapi_phase_const_GMatR_const_GMatR_bool(const cv::GMat* x, const cv::GMat* y, bool angleInDegrees, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::phase(*x, *y, angleInDegrees);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::polarToCart(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:857
	// ("cv::gapi::polarToCart", vec![(pred!(mut, ["magnitude", "angle"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_polarToCart_const_GMatR_const_GMatR(const cv::GMat* magnitude, const cv::GMat* angle, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::polarToCart(*magnitude, *angle);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// polarToCart(const GMat &, const GMat &, bool)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:857
	// ("cv::gapi::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "angleInDegrees"], ["const cv::GMat*", "const cv::GMat*", "bool"]), _)]),
	void cv_gapi_polarToCart_const_GMatR_const_GMatR_bool(const cv::GMat* magnitude, const cv::GMat* angle, bool angleInDegrees, Result<std::tuple<cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat> ret = cv::gapi::polarToCart(*magnitude, *angle, angleInDegrees);
			Ok(new std::tuple<cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::remap(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1551
	// ("cv::gapi::remap", vec![(pred!(mut, ["src", "map1", "map2", "interpolation"], ["const cv::GMat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int(const cv::GMat* src, const cv::Mat* map1, const cv::Mat* map2, int interpolation, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::remap(*src, *map1, *map2, interpolation);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// remap(const GMat &, const Mat &, const Mat &, int, int, const Scalar &)(TraitClass, TraitClass, TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1551
	// ("cv::gapi::remap", vec![(pred!(mut, ["src", "map1", "map2", "interpolation", "borderMode", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Mat*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* map1, const cv::Mat* map2, int interpolation, int borderMode, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::remap(*src, *map1, *map2, interpolation, borderMode, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::resizeP(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1763
	// ("cv::gapi::resizeP", vec![(pred!(mut, ["src", "dsize"], ["const cv::GMatP*", "const cv::Size*"]), _)]),
	void cv_gapi_resizeP_const_GMatPR_const_SizeR(const cv::GMatP* src, const cv::Size* dsize, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::resizeP(*src, *dsize);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resizeP(const GMatP &, const Size &, int)(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1763
	// ("cv::gapi::resizeP", vec![(pred!(mut, ["src", "dsize", "interpolation"], ["const cv::GMatP*", "const cv::Size*", "int"]), _)]),
	void cv_gapi_resizeP_const_GMatPR_const_SizeR_int(const cv::GMatP* src, const cv::Size* dsize, int interpolation, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::resizeP(*src, *dsize, interpolation);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::resize(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1745
	// ("cv::gapi::resize", vec![(pred!(mut, ["src", "dsize"], ["const cv::GMat*", "const cv::Size*"]), _)]),
	void cv_gapi_resize_const_GMatR_const_SizeR(const cv::GMat* src, const cv::Size* dsize, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::resize(*src, *dsize);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resize(const GMat &, const Size &, double, double, int)(TraitClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:1745
	// ("cv::gapi::resize", vec![(pred!(mut, ["src", "dsize", "fx", "fy", "interpolation"], ["const cv::GMat*", "const cv::Size*", "double", "double", "int"]), _)]),
	void cv_gapi_resize_const_GMatR_const_SizeR_double_double_int(const cv::GMat* src, const cv::Size* dsize, double fx, double fy, int interpolation, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::resize(*src, *dsize, fx, fy, interpolation);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// select(const GMat &, const GMat &, const GMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1181
	// ("cv::gapi::select", vec![(pred!(mut, ["src1", "src2", "mask"], ["const cv::GMat*", "const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_select_const_GMatR_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, const cv::GMat* mask, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::select(*src1, *src2, *mask);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::sepFilter(TraitClass, Primitive, TraitClass, TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:559
	// ("cv::gapi::sepFilter", vec![(pred!(mut, ["src", "ddepth", "kernelX", "kernelY", "anchor", "delta"], ["const cv::GMat*", "int", "const cv::Mat*", "const cv::Mat*", "const cv::Point*", "const cv::Scalar*"]), _)]),
	void cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR(const cv::GMat* src, int ddepth, const cv::Mat* kernelX, const cv::Mat* kernelY, const cv::Point* anchor, const cv::Scalar* delta, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sepFilter(*src, ddepth, *kernelX, *kernelY, *anchor, *delta);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sepFilter(const GMat &, int, const Mat &, const Mat &, const Point &, const Scalar &, int, const Scalar &)(TraitClass, Primitive, TraitClass, TraitClass, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/imgproc.hpp:559
	// ("cv::gapi::sepFilter", vec![(pred!(mut, ["src", "ddepth", "kernelX", "kernelY", "anchor", "delta", "borderType", "borderValue"], ["const cv::GMat*", "int", "const cv::Mat*", "const cv::Mat*", "const cv::Point*", "const cv::Scalar*", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(const cv::GMat* src, int ddepth, const cv::Mat* kernelX, const cv::Mat* kernelY, const cv::Point* anchor, const cv::Scalar* delta, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sepFilter(*src, ddepth, *kernelX, *kernelY, *anchor, *delta, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// split3(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1518
	// ("cv::gapi::split3", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_split3_const_GMatR(const cv::GMat* src, Result<std::tuple<cv::GMat, cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat, cv::GMat> ret = cv::gapi::split3(*src);
			Ok(new std::tuple<cv::GMat, cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// split4(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1501
	// ("cv::gapi::split4", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_split4_const_GMatR(const cv::GMat* src, Result<std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat> ret = cv::gapi::split4(*src);
			Ok(new std::tuple<cv::GMat, cv::GMat, cv::GMat, cv::GMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqrt(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:910
	// ("cv::gapi::sqrt", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_sqrt_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sqrt(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BGR(const cv::GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:43
	// ("cv::gapi::streaming::BGR", vec![(pred!(mut, ["in"], ["const cv::GFrame*"]), _)]),
	void cv_gapi_streaming_BGR_const_GFrameR(const cv::GFrame* in, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::BGR(*in);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UV(const cv::GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:63
	// ("cv::gapi::streaming::UV", vec![(pred!(mut, ["frame"], ["const cv::GFrame*"]), _)]),
	void cv_gapi_streaming_UV_const_GFrameR(const cv::GFrame* frame, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::UV(*frame);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Y(const cv::GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:53
	// ("cv::gapi::streaming::Y", vec![(pred!(mut, ["frame"], ["const cv::GFrame*"]), _)]),
	void cv_gapi_streaming_Y_const_GFrameR(const cv::GFrame* frame, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::Y(*frame);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// desync(const GFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/desync.hpp:80
	// ("cv::gapi::streaming::desync", vec![(pred!(mut, ["f"], ["const cv::GFrame*"]), _)]),
	void cv_gapi_streaming_desync_const_GFrameR(const cv::GFrame* f, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = cv::gapi::streaming::desync(*f);
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// desync(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/desync.hpp:79
	// ("cv::gapi::streaming::desync", vec![(pred!(mut, ["g"], ["const cv::GMat*"]), _)]),
	void cv_gapi_streaming_desync_const_GMatR(const cv::GMat* g, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::desync(*g);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kernels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/streaming/format.hpp:16
	// ("cv::gapi::streaming::kernels", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_streaming_kernels(Result<cv::GKernelPackage*>* ocvrs_return) {
		try {
			cv::GKernelPackage ret = cv::gapi::streaming::kernels();
			Ok(new cv::GKernelPackage(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::subC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:682
	// ("cv::gapi::subC", vec![(pred!(mut, ["src", "c"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_gapi_subC_const_GMatR_const_GScalarR(const cv::GMat* src, const cv::GScalar* c, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::subC(*src, *c);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subC(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:682
	// ("cv::gapi::subC", vec![(pred!(mut, ["src", "c", "ddepth"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
	void cv_gapi_subC_const_GMatR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* c, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::subC(*src, *c, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::subRC(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:701
	// ("cv::gapi::subRC", vec![(pred!(mut, ["c", "src"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_gapi_subRC_const_GScalarR_const_GMatR(const cv::GScalar* c, const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::subRC(*c, *src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subRC(const GScalar &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:701
	// ("cv::gapi::subRC", vec![(pred!(mut, ["c", "src", "ddepth"], ["const cv::GScalar*", "const cv::GMat*", "int"]), _)]),
	void cv_gapi_subRC_const_GScalarR_const_GMatR_int(const cv::GScalar* c, const cv::GMat* src, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::subRC(*c, *src, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::sub(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:663
	// ("cv::gapi::sub", vec![(pred!(mut, ["src1", "src2"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_gapi_sub_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sub(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sub(const GMat &, const GMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:663
	// ("cv::gapi::sub", vec![(pred!(mut, ["src1", "src2", "ddepth"], ["const cv::GMat*", "const cv::GMat*", "int"]), _)]),
	void cv_gapi_sub_const_GMatR_const_GMatR_int(const cv::GMat* src1, const cv::GMat* src2, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sub(*src1, *src2, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sum(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1266
	// ("cv::gapi::sum", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_sum_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::sum(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// threshold(const GMat &, const GScalar &, const GScalar &, int)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1419
	// ("cv::gapi::threshold", vec![(pred!(mut, ["src", "thresh", "maxval", "type"], ["const cv::GMat*", "const cv::GScalar*", "const cv::GScalar*", "int"]), _)]),
	void cv_gapi_threshold_const_GMatR_const_GScalarR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* thresh, const cv::GScalar* maxval, int type, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::threshold(*src, *thresh, *maxval, type);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// threshold(const GMat &, const GScalar &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1424
	// ("cv::gapi::threshold", vec![(pred!(mut, ["src", "maxval", "type"], ["const cv::GMat*", "const cv::GScalar*", "int"]), _)]),
	void cv_gapi_threshold_const_GMatR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* maxval, int type, Result<std::tuple<cv::GMat, cv::GScalar>*>* ocvrs_return) {
		try {
			std::tuple<cv::GMat, cv::GScalar> ret = cv::gapi::threshold(*src, *maxval, type);
			Ok(new std::tuple<cv::GMat, cv::GScalar>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transpose(const GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1876
	// ("cv::gapi::transpose", vec![(pred!(mut, ["src"], ["const cv::GMat*"]), _)]),
	void cv_gapi_transpose_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::transpose(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::warpAffine(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1787
	// ("cv::gapi::warpAffine", vec![(pred!(mut, ["src", "M", "dsize"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*"]), _)]),
	void cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR(const cv::GMat* src, const cv::Mat* M, const cv::Size* dsize, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::warpAffine(*src, *M, *dsize);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpAffine(const GMat &, const Mat &, const Size &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1787
	// ("cv::gapi::warpAffine", vec![(pred!(mut, ["src", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::warpAffine(*src, *M, *dsize, flags, borderMode, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::warpPerspective(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1762
	// ("cv::gapi::warpPerspective", vec![(pred!(mut, ["src", "M", "dsize"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*"]), _)]),
	void cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR(const cv::GMat* src, const cv::Mat* M, const cv::Size* dsize, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::warpPerspective(*src, *M, *dsize);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPerspective(const GMat &, const Mat &, const Size &, int, int, const Scalar &)(TraitClass, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/core.hpp:1762
	// ("cv::gapi::warpPerspective", vec![(pred!(mut, ["src", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::GMat*", "const cv::Mat*", "const cv::Size*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::warpPerspective(*src, *M, *dsize, flags, borderMode, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:16
	// ("cv::operator+", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorA_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator+(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:18
	// ("cv::operator+", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorA_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator+(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:19
	// ("cv::operator+", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorA_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator+(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:33
	// ("cv::operator/", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorD_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator/(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:31
	// ("cv::operator/", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorD_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator/(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:32
	// ("cv::operator/", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorD_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator/(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:52
	// ("cv::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorEQ_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator==(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:59
	// ("cv::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorEQ_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator==(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:66
	// ("cv::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorEQ_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator==(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>=(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:49
	// ("cv::operator>=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorGE_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator>=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>=(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:56
	// ("cv::operator>=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorGE_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator>=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>=(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:63
	// ("cv::operator>=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorGE_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator>=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:48
	// ("cv::operator>", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorG_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator>(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:55
	// ("cv::operator>", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorG_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator>(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:62
	// ("cv::operator>", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorG_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator>(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<=(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:51
	// ("cv::operator<=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorLE_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator<=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<=(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:58
	// ("cv::operator<=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorLE_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator<=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<=(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:65
	// ("cv::operator<=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorLE_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator<=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:50
	// ("cv::operator<", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorL_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator<(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:57
	// ("cv::operator<", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorL_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator<(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:64
	// ("cv::operator<", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorL_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator<(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:53
	// ("cv::operator!=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorNE_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator!=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:60
	// ("cv::operator!=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorNE_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator!=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:67
	// ("cv::operator!=", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorNE_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator!=(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator~(const cv::GMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:38
	// ("cv::operator~", vec![(pred!(mut, ["lhs"], ["const cv::GMat*"]), _)]),
	void cv_operatorNOTB_const_GMatR(const cv::GMat* lhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator~(*lhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator|(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:36
	// ("cv::operator|", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorOR_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator|(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator|(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:45
	// ("cv::operator|", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorOR_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator|(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator|(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:41
	// ("cv::operator|", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorOR_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator|(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator&(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:35
	// ("cv::operator&", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorR_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator&(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator&(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:44
	// ("cv::operator&", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorR_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator&(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator&(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:40
	// ("cv::operator&", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorR_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator&(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:21
	// ("cv::operator-", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorS_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator-(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:23
	// ("cv::operator-", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorS_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator-(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:24
	// ("cv::operator-", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorS_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator-(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator^(const cv::GMat &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:37
	// ("cv::operator^", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GMat*"]), _)]),
	void cv_operatorXOR_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator^(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator^(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:46
	// ("cv::operator^", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorXOR_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator^(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator^(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:42
	// ("cv::operator^", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorXOR_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator^(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const cv::GMat &, const cv::GScalar &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:28
	// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "const cv::GScalar*"]), _)]),
	void cv_operatorX_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const cv::GMat &, float)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:26
	// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GMat*", "float"]), _)]),
	void cv_operatorX_const_GMatR_float(const cv::GMat* lhs, float rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(*lhs, rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const cv::GScalar &, const cv::GMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:29
	// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::GScalar*", "const cv::GMat*"]), _)]),
	void cv_operatorX_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(float, const cv::GMat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/operators.hpp:27
	// ("cv::operator*", vec![(pred!(mut, ["lhs", "rhs"], ["float", "const cv::GMat*"]), _)]),
	void cv_operatorX_float_const_GMatR(float lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// validate_input_arg(const GRunArg &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gproto.hpp:154
	// ("cv::validate_input_arg", vec![(pred!(mut, ["arg"], ["const cv::GRunArg*"]), _)]),
	void cv_validate_input_arg_const_GRunArgR(const cv::GRunArg* arg, ResultVoid* ocvrs_return) {
		try {
			cv::validate_input_arg(*arg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// validate_input_args(const GRunArgs &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gproto.hpp:155
	// ("cv::validate_input_args", vec![(pred!(mut, ["args"], ["const cv::GRunArgs*"]), _)]),
	void cv_validate_input_args_const_GRunArgsR(const cv::GRunArgs* args, ResultVoid* ocvrs_return) {
		try {
			cv::validate_input_args(*args);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:49
	// ("cv::GArg::GArg", vec![(pred!(mut, [], []), _)]),
	void cv_GArg_GArg(Result<cv::GArg*>* ocvrs_return) {
		try {
			cv::GArg* ret = new cv::GArg();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GArg::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:87
	// ("cv::GArg::kind", vec![(pred!(const, [], []), _)]),
	void cv_GArg_propKind_const(const cv::GArg* instance, cv::detail::ArgKind* ocvrs_return) {
			cv::detail::ArgKind ret = instance->kind;
			*ocvrs_return = ret;
	}

	// cv::GArg::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:87
	// ("cv::GArg::setKind", vec![(pred!(mut, ["val"], ["const cv::detail::ArgKind"]), _)]),
	void cv_GArg_propKind_const_ArgKind(cv::GArg* instance, const cv::detail::ArgKind val) {
			instance->kind = val;
	}

	// cv::GArg::opaque_kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:88
	// ("cv::GArg::opaque_kind", vec![(pred!(const, [], []), _)]),
	void cv_GArg_propOpaque_kind_const(const cv::GArg* instance, cv::detail::OpaqueKind* ocvrs_return) {
			cv::detail::OpaqueKind ret = instance->opaque_kind;
			*ocvrs_return = ret;
	}

	// cv::GArg::setOpaque_kind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:88
	// ("cv::GArg::setOpaque_kind", vec![(pred!(mut, ["val"], ["const cv::detail::OpaqueKind"]), _)]),
	void cv_GArg_propOpaque_kind_const_OpaqueKind(cv::GArg* instance, const cv::detail::OpaqueKind val) {
			instance->opaque_kind = val;
	}

	// cv::GArg::delete() generated
	// ("cv::GArg::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GArg_delete(cv::GArg* instance) {
			delete instance;
	}

	// operator==(const GArrayDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garray.hpp:42
	// ("cv::GArrayDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GArrayDesc*"]), _)]),
	void cv_GArrayDesc_operatorEQ_const_const_GArrayDescR(const cv::GArrayDesc* instance, const cv::GArrayDesc* unnamed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GArrayDesc::implicitClone() generated
	// ("cv::GArrayDesc::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GArrayDesc* cv_GArrayDesc_implicitClone_const(const cv::GArrayDesc* instance) {
			return new cv::GArrayDesc(*instance);
	}

	// cv::GArrayDesc::defaultNew() generated
	// ("cv::GArrayDesc::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GArrayDesc* cv_GArrayDesc_defaultNew_const() {
			cv::GArrayDesc* ret = new cv::GArrayDesc();
			return ret;
	}

	// cv::GArrayDesc::delete() generated
	// ("cv::GArrayDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GArrayDesc_delete(cv::GArrayDesc* instance) {
			delete instance;
	}

	// GCall(const GKernel &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:31
	// ("cv::GCall::GCall", vec![(pred!(mut, ["k"], ["const cv::GKernel*"]), _)]),
	void cv_GCall_GCall_const_GKernelR(const cv::GKernel* k, Result<cv::GCall*>* ocvrs_return) {
		try {
			cv::GCall* ret = new cv::GCall(*k);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// yield(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:42
	// ("cv::GCall::yield", vec![(pred!(mut, ["output"], ["int"]), _)]),
	void cv_GCall_yield_int(cv::GCall* instance, int output, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = instance->yield(output);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GCall::yield() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:42
	// ("cv::GCall::yield", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_yield(cv::GCall* instance, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = instance->yield();
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// yieldP(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:43
	// ("cv::GCall::yieldP", vec![(pred!(mut, ["output"], ["int"]), _)]),
	void cv_GCall_yieldP_int(cv::GCall* instance, int output, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = instance->yieldP(output);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GCall::yieldP() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:43
	// ("cv::GCall::yieldP", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_yieldP(cv::GCall* instance, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = instance->yieldP();
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// yieldScalar(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:44
	// ("cv::GCall::yieldScalar", vec![(pred!(mut, ["output"], ["int"]), _)]),
	void cv_GCall_yieldScalar_int(cv::GCall* instance, int output, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = instance->yieldScalar(output);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GCall::yieldScalar() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:44
	// ("cv::GCall::yieldScalar", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_yieldScalar(cv::GCall* instance, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = instance->yieldScalar();
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// yieldFrame(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:45
	// ("cv::GCall::yieldFrame", vec![(pred!(mut, ["output"], ["int"]), _)]),
	void cv_GCall_yieldFrame_int(cv::GCall* instance, int output, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = instance->yieldFrame(output);
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GCall::yieldFrame() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:45
	// ("cv::GCall::yieldFrame", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_yieldFrame(cv::GCall* instance, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = instance->yieldFrame();
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:63
	// ("cv::GCall::kernel", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_kernel(cv::GCall* instance, Result<cv::GKernel*>* ocvrs_return) {
		try {
			cv::GKernel ret = instance->kernel();
			Ok(new cv::GKernel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:64
	// ("cv::GCall::params", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_params(cv::GCall* instance, Result<cv::util::any*>* ocvrs_return) {
		try {
			cv::util::any ret = instance->params();
			Ok(new cv::util::any(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setArgs(std::vector<GArg> &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcall.hpp:66
	// ("cv::GCall::setArgs", vec![(pred!(mut, ["args"], ["std::vector<cv::GArg>*"]), _)]),
	void cv_GCall_setArgs_vectorLGArgGRR(cv::GCall* instance, std::vector<cv::GArg>* args, ResultVoid* ocvrs_return) {
		try {
			instance->setArgs(std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GCall::delete() generated
	// ("cv::GCall::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GCall_delete(cv::GCall* instance) {
			delete instance;
	}

	// GCompileArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:162
	// ("cv::GCompileArg::GCompileArg", vec![(pred!(mut, [], []), _)]),
	cv::GCompileArg* cv_GCompileArg_GCompileArg() {
			cv::GCompileArg* ret = new cv::GCompileArg();
			return ret;
	}

	// cv::GCompileArg::tag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:164
	// ("cv::GCompileArg::tag", vec![(pred!(const, [], []), _)]),
	void* cv_GCompileArg_propTag_const(const cv::GCompileArg* instance) {
			std::string ret = instance->tag;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::GCompileArg::setTag(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:164
	// ("cv::GCompileArg::setTag", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_GCompileArg_propTag_const_string(cv::GCompileArg* instance, const char* val) {
			instance->tag = std::string(val);
	}

	// cv::GCompileArg::delete() generated
	// ("cv::GCompileArg::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GCompileArg_delete(cv::GCompileArg* instance) {
			delete instance;
	}

	// GCompiled()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:75
	// ("cv::GCompiled::GCompiled", vec![(pred!(mut, [], []), _)]),
	void cv_GCompiled_GCompiled(Result<cv::GCompiled*>* ocvrs_return) {
		try {
			cv::GCompiled* ret = new cv::GCompiled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(cv::Mat, cv::Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:107
	// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Mat*"]), _)]),
	void cv_GCompiled_operator___Mat_MatR(cv::GCompiled* instance, cv::Mat* in, cv::Mat* out, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*in, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(cv::Mat, cv::Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:117
	// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Scalar*"]), _)]),
	void cv_GCompiled_operator___Mat_ScalarR(cv::GCompiled* instance, cv::Mat* in, cv::Scalar* out, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*in, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(cv::Mat, cv::Mat, cv::Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:128
	// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Mat*"]), _)]),
	void cv_GCompiled_operator___Mat_Mat_MatR(cv::GCompiled* instance, cv::Mat* in1, cv::Mat* in2, cv::Mat* out, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*in1, *in2, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(cv::Mat, cv::Mat, cv::Scalar &)(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:139
	// ("cv::GCompiled::operator()", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Scalar*"]), _)]),
	void cv_GCompiled_operator___Mat_Mat_ScalarR(cv::GCompiled* instance, cv::Mat* in1, cv::Mat* in2, cv::Scalar* out, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*in1, *in2, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const std::vector<cv::Mat> &, const std::vector<cv::Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:154
	// ("cv::GCompiled::operator()", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::Mat>*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_GCompiled_operator___const_vectorLMatGR_const_vectorLMatGR(cv::GCompiled* instance, const std::vector<cv::Mat>* ins, const std::vector<cv::Mat>* outs, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*ins, *outs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator bool()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:165
	// ("cv::GCompiled::operator bool", vec![(pred!(const, [], []), _)]),
	void cv_GCompiled_operator_bool_const(const cv::GCompiled* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator bool();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// canReshape()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:196
	// ("cv::GCompiled::canReshape", vec![(pred!(const, [], []), _)]),
	void cv_GCompiled_canReshape_const(const cv::GCompiled* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->canReshape();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareForNewStream()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcompiled.hpp:222
	// ("cv::GCompiled::prepareForNewStream", vec![(pred!(mut, [], []), _)]),
	void cv_GCompiled_prepareForNewStream(cv::GCompiled* instance, ResultVoid* ocvrs_return) {
		try {
			instance->prepareForNewStream();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GCompiled::delete() generated
	// ("cv::GCompiled::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GCompiled_delete(cv::GCompiled* instance) {
			delete instance;
	}

	// GComputation(GMat, GMat)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:174
	// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in", "out"], ["cv::GMat", "cv::GMat"]), _)]),
	void cv_GComputation_GComputation_GMat_GMat(cv::GMat* in, cv::GMat* out, Result<cv::GComputation*>* ocvrs_return) {
		try {
			cv::GComputation* ret = new cv::GComputation(*in, *out);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GComputation(GMat, GScalar)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:183
	// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in", "out"], ["cv::GMat", "cv::GScalar"]), _)]),
	void cv_GComputation_GComputation_GMat_GScalar(cv::GMat* in, cv::GScalar* out, Result<cv::GComputation*>* ocvrs_return) {
		try {
			cv::GComputation* ret = new cv::GComputation(*in, *out);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GComputation(GMat, GMat, GMat)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:193
	// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::GMat", "cv::GMat", "cv::GMat"]), _)]),
	void cv_GComputation_GComputation_GMat_GMat_GMat(cv::GMat* in1, cv::GMat* in2, cv::GMat* out, Result<cv::GComputation*>* ocvrs_return) {
		try {
			cv::GComputation* ret = new cv::GComputation(*in1, *in2, *out);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GComputation(GMat, GMat, GScalar)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:203
	// ("cv::GComputation::GComputation", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::GMat", "cv::GMat", "cv::GScalar"]), _)]),
	void cv_GComputation_GComputation_GMat_GMat_GScalar(cv::GMat* in1, cv::GMat* in2, cv::GScalar* out, Result<cv::GComputation*>* ocvrs_return) {
		try {
			cv::GComputation* ret = new cv::GComputation(*in1, *in2, *out);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GComputation(const std::vector<GMat> &, const std::vector<GMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:219
	// ("cv::GComputation::GComputation", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::GMat>*", "const std::vector<cv::GMat>*"]), _)]),
	void cv_GComputation_GComputation_const_vectorLGMatGR_const_vectorLGMatGR(const std::vector<cv::GMat>* ins, const std::vector<cv::GMat>* outs, Result<cv::GComputation*>* ocvrs_return) {
		try {
			cv::GComputation* ret = new cv::GComputation(*ins, *outs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(const cv::detail::ExtractArgsCallback &, GCompileArgs &&)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:262
	// ("cv::GComputation::apply", vec![(pred!(mut, ["callback", "args"], ["const cv::detail::ExtractArgsCallback*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_const_ExtractArgsCallbackR_GCompileArgsRR(cv::GComputation* instance, const cv::detail::ExtractArgsCallback* callback, cv::GCompileArgs* args, Result<cv::GRunArgs*>* ocvrs_return) {
		try {
			cv::GRunArgs ret = instance->apply(*callback, std::move(*args));
			Ok(new cv::GRunArgs(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:262
	// ("cv::GComputation::apply", vec![(pred!(mut, ["callback"], ["const cv::detail::ExtractArgsCallback*"]), _)]),
	void cv_GComputation_apply_const_ExtractArgsCallbackR(cv::GComputation* instance, const cv::detail::ExtractArgsCallback* callback, Result<cv::GRunArgs*>* ocvrs_return) {
		try {
			cv::GRunArgs ret = instance->apply(*callback);
			Ok(new cv::GRunArgs(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(const std::vector<cv::Mat> &, const std::vector<cv::Mat> &, GCompileArgs &&)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:266
	// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs", "args"], ["const std::vector<cv::Mat>*", "const std::vector<cv::Mat>*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_const_vectorLMatGR_const_vectorLMatGR_GCompileArgsRR(cv::GComputation* instance, const std::vector<cv::Mat>* ins, const std::vector<cv::Mat>* outs, cv::GCompileArgs* args, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*ins, *outs, std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:266
	// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::Mat>*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_GComputation_apply_const_vectorLMatGR_const_vectorLMatGR(cv::GComputation* instance, const std::vector<cv::Mat>* ins, const std::vector<cv::Mat>* outs, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*ins, *outs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(cv::Mat, cv::Mat &, GCompileArgs &&)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:281
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out", "args"], ["cv::Mat", "cv::Mat*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_Mat_MatR_GCompileArgsRR(cv::GComputation* instance, cv::Mat* in, cv::Mat* out, cv::GCompileArgs* args, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in, *out, std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:281
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Mat*"]), _)]),
	void cv_GComputation_apply_Mat_MatR(cv::GComputation* instance, cv::Mat* in, cv::Mat* out, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(cv::Mat, cv::Scalar &, GCompileArgs &&)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:292
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out", "args"], ["cv::Mat", "cv::Scalar*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_Mat_ScalarR_GCompileArgsRR(cv::GComputation* instance, cv::Mat* in, cv::Scalar* out, cv::GCompileArgs* args, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in, *out, std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:292
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in", "out"], ["cv::Mat", "cv::Scalar*"]), _)]),
	void cv_GComputation_apply_Mat_ScalarR(cv::GComputation* instance, cv::Mat* in, cv::Scalar* out, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(cv::Mat, cv::Mat, cv::Mat &, GCompileArgs &&)(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:304
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out", "args"], ["cv::Mat", "cv::Mat", "cv::Mat*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_Mat_Mat_MatR_GCompileArgsRR(cv::GComputation* instance, cv::Mat* in1, cv::Mat* in2, cv::Mat* out, cv::GCompileArgs* args, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in1, *in2, *out, std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:304
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Mat*"]), _)]),
	void cv_GComputation_apply_Mat_Mat_MatR(cv::GComputation* instance, cv::Mat* in1, cv::Mat* in2, cv::Mat* out, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in1, *in2, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(cv::Mat, cv::Mat, cv::Scalar &, GCompileArgs &&)(TraitClass, TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:316
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out", "args"], ["cv::Mat", "cv::Mat", "cv::Scalar*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_Mat_Mat_ScalarR_GCompileArgsRR(cv::GComputation* instance, cv::Mat* in1, cv::Mat* in2, cv::Scalar* out, cv::GCompileArgs* args, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in1, *in2, *out, std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(TraitClass, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:316
	// ("cv::GComputation::apply", vec![(pred!(mut, ["in1", "in2", "out"], ["cv::Mat", "cv::Mat", "cv::Scalar*"]), _)]),
	void cv_GComputation_apply_Mat_Mat_ScalarR(cv::GComputation* instance, cv::Mat* in1, cv::Mat* in2, cv::Scalar* out, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*in1, *in2, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(const std::vector<cv::Mat> &, std::vector<cv::Mat> &, GCompileArgs &&)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:333
	// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs", "args"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_apply_const_vectorLMatGR_vectorLMatGR_GCompileArgsRR(cv::GComputation* instance, const std::vector<cv::Mat>* ins, std::vector<cv::Mat>* outs, cv::GCompileArgs* args, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*ins, *outs, std::move(*args));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::apply(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:333
	// ("cv::GComputation::apply", vec![(pred!(mut, ["ins", "outs"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_GComputation_apply_const_vectorLMatGR_vectorLMatGR(cv::GComputation* instance, const std::vector<cv::Mat>* ins, std::vector<cv::Mat>* outs, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*ins, *outs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compileStreaming(GCompileArgs &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:462
	// ("cv::GComputation::compileStreaming", vec![(pred!(mut, ["args"], ["cv::GCompileArgs*"]), _)]),
	void cv_GComputation_compileStreaming_GCompileArgsRR(cv::GComputation* instance, cv::GCompileArgs* args, Result<cv::GStreamingCompiled*>* ocvrs_return) {
		try {
			cv::GStreamingCompiled ret = instance->compileStreaming(std::move(*args));
			Ok(new cv::GStreamingCompiled(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::compileStreaming() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:462
	// ("cv::GComputation::compileStreaming", vec![(pred!(mut, [], []), _)]),
	void cv_GComputation_compileStreaming(cv::GComputation* instance, Result<cv::GStreamingCompiled*>* ocvrs_return) {
		try {
			cv::GStreamingCompiled ret = instance->compileStreaming();
			Ok(new cv::GStreamingCompiled(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compileStreaming(const cv::detail::ExtractMetaCallback &, GCompileArgs &&)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:465
	// ("cv::GComputation::compileStreaming", vec![(pred!(mut, ["callback", "args"], ["const cv::detail::ExtractMetaCallback*", "cv::GCompileArgs*"]), _)]),
	void cv_GComputation_compileStreaming_const_ExtractMetaCallbackR_GCompileArgsRR(cv::GComputation* instance, const cv::detail::ExtractMetaCallback* callback, cv::GCompileArgs* args, Result<cv::GStreamingCompiled*>* ocvrs_return) {
		try {
			cv::GStreamingCompiled ret = instance->compileStreaming(*callback, std::move(*args));
			Ok(new cv::GStreamingCompiled(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::compileStreaming(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcomputation.hpp:465
	// ("cv::GComputation::compileStreaming", vec![(pred!(mut, ["callback"], ["const cv::detail::ExtractMetaCallback*"]), _)]),
	void cv_GComputation_compileStreaming_const_ExtractMetaCallbackR(cv::GComputation* instance, const cv::detail::ExtractMetaCallback* callback, Result<cv::GStreamingCompiled*>* ocvrs_return) {
		try {
			cv::GStreamingCompiled ret = instance->compileStreaming(*callback);
			Ok(new cv::GStreamingCompiled(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GComputation::delete() generated
	// ("cv::GComputation::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GComputation_delete(cv::GComputation* instance) {
			delete instance;
	}

	// GFrame()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:71
	// ("cv::GFrame::GFrame", vec![(pred!(mut, [], []), _)]),
	void cv_GFrame_GFrame(Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame* ret = new cv::GFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GFrame::implicitClone() generated
	// ("cv::GFrame::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GFrame* cv_GFrame_implicitClone_const(const cv::GFrame* instance) {
			return new cv::GFrame(*instance);
	}

	// cv::GFrame::delete() generated
	// ("cv::GFrame::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GFrame_delete(cv::GFrame* instance) {
			delete instance;
	}

	// operator==(const GFrameDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:101
	// ("cv::GFrameDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GFrameDesc*"]), _)]),
	void cv_GFrameDesc_operatorEQ_const_const_GFrameDescR(const cv::GFrameDesc* instance, const cv::GFrameDesc* unnamed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GFrameDesc::defaultNew() generated
	// ("cv::GFrameDesc::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GFrameDesc* cv_GFrameDesc_defaultNew_const() {
			cv::GFrameDesc* ret = new cv::GFrameDesc();
			return ret;
	}

	// cv::GFrameDesc::fmt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:98
	// ("cv::GFrameDesc::fmt", vec![(pred!(const, [], []), _)]),
	void cv_GFrameDesc_propFmt_const(const cv::GFrameDesc* instance, cv::MediaFormat* ocvrs_return) {
			cv::MediaFormat ret = instance->fmt;
			*ocvrs_return = ret;
	}

	// cv::GFrameDesc::setFmt(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:98
	// ("cv::GFrameDesc::setFmt", vec![(pred!(mut, ["val"], ["const cv::MediaFormat"]), _)]),
	void cv_GFrameDesc_propFmt_const_MediaFormat(cv::GFrameDesc* instance, const cv::MediaFormat val) {
			instance->fmt = val;
	}

	// cv::GFrameDesc::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:99
	// ("cv::GFrameDesc::size", vec![(pred!(const, [], []), _)]),
	void cv_GFrameDesc_propSize_const(const cv::GFrameDesc* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->size;
			*ocvrs_return = ret;
	}

	// cv::GFrameDesc::setSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gframe.hpp:99
	// ("cv::GFrameDesc::setSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_GFrameDesc_propSize_const_Size(cv::GFrameDesc* instance, const cv::Size* val) {
			instance->size = *val;
	}

	// cv::GFrameDesc::delete() generated
	// ("cv::GFrameDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GFrameDesc_delete(cv::GFrameDesc* instance) {
			delete instance;
	}

	// cv::GKernel::defaultNew() generated
	// ("cv::GKernel::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GKernel* cv_GKernel_defaultNew_const() {
			cv::GKernel* ret = new cv::GKernel();
			return ret;
	}

	// cv::GKernel::name() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:48
	// ("cv::GKernel::name", vec![(pred!(const, [], []), _)]),
	void* cv_GKernel_propName_const(const cv::GKernel* instance) {
			std::string ret = instance->name;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::GKernel::setName(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:48
	// ("cv::GKernel::setName", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_GKernel_propName_const_string(cv::GKernel* instance, const char* val) {
			instance->name = std::string(val);
	}

	// cv::GKernel::tag() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:49
	// ("cv::GKernel::tag", vec![(pred!(const, [], []), _)]),
	void* cv_GKernel_propTag_const(const cv::GKernel* instance) {
			std::string ret = instance->tag;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::GKernel::setTag(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:49
	// ("cv::GKernel::setTag", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_GKernel_propTag_const_string(cv::GKernel* instance, const char* val) {
			instance->tag = std::string(val);
	}

	// cv::GKernel::outShapes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:51
	// ("cv::GKernel::outShapes", vec![(pred!(const, [], []), _)]),
	cv::GShapes* cv_GKernel_propOutShapes_const(const cv::GKernel* instance) {
			cv::GShapes ret = instance->outShapes;
			return new cv::GShapes(ret);
	}

	// cv::GKernel::setOutShapes(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:51
	// ("cv::GKernel::setOutShapes", vec![(pred!(mut, ["val"], ["const cv::GShapes"]), _)]),
	void cv_GKernel_propOutShapes_const_GShapes(cv::GKernel* instance, const cv::GShapes* val) {
			instance->outShapes = *val;
	}

	// cv::GKernel::inKinds() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:52
	// ("cv::GKernel::inKinds", vec![(pred!(const, [], []), _)]),
	cv::GKinds* cv_GKernel_propInKinds_const(const cv::GKernel* instance) {
			cv::GKinds ret = instance->inKinds;
			return new cv::GKinds(ret);
	}

	// cv::GKernel::setInKinds(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:52
	// ("cv::GKernel::setInKinds", vec![(pred!(mut, ["val"], ["const cv::GKinds"]), _)]),
	void cv_GKernel_propInKinds_const_GKinds(cv::GKernel* instance, const cv::GKinds* val) {
			instance->inKinds = *val;
	}

	// cv::GKernel::outKinds() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:54
	// ("cv::GKernel::outKinds", vec![(pred!(const, [], []), _)]),
	cv::GKinds* cv_GKernel_propOutKinds_const(const cv::GKernel* instance) {
			cv::GKinds ret = instance->outKinds;
			return new cv::GKinds(ret);
	}

	// cv::GKernel::setOutKinds(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:54
	// ("cv::GKernel::setOutKinds", vec![(pred!(mut, ["val"], ["const cv::GKinds"]), _)]),
	void cv_GKernel_propOutKinds_const_GKinds(cv::GKernel* instance, const cv::GKinds* val) {
			instance->outKinds = *val;
	}

	// cv::GKernel::delete() generated
	// ("cv::GKernel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GKernel_delete(cv::GKernel* instance) {
			delete instance;
	}

	// cv::GKernelImpl::defaultNew() generated
	// ("cv::GKernelImpl::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GKernelImpl* cv_GKernelImpl_defaultNew_const() {
			cv::GKernelImpl* ret = new cv::GKernelImpl();
			return ret;
	}

	// cv::GKernelImpl::opaque() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:62
	// ("cv::GKernelImpl::opaque", vec![(pred!(const, [], []), _)]),
	cv::util::any* cv_GKernelImpl_propOpaque_const(const cv::GKernelImpl* instance) {
			cv::util::any ret = instance->opaque;
			return new cv::util::any(ret);
	}

	// cv::GKernelImpl::setOpaque(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:62
	// ("cv::GKernelImpl::setOpaque", vec![(pred!(mut, ["val"], ["const cv::util::any"]), _)]),
	void cv_GKernelImpl_propOpaque_const_any(cv::GKernelImpl* instance, const cv::util::any* val) {
			instance->opaque = *val;
	}

	// cv::GKernelImpl::delete() generated
	// ("cv::GKernelImpl::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GKernelImpl_delete(cv::GKernelImpl* instance) {
			delete instance;
	}

	// include(const cv::gapi::GFunctor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:511
	// ("cv::GKernelPackage::include", vec![(pred!(mut, ["functor"], ["const cv::gapi::GFunctor*"]), _)]),
	void cv_GKernelPackage_include_const_GFunctorR(cv::GKernelPackage* instance, const cv::gapi::GFunctor* functor, ResultVoid* ocvrs_return) {
		try {
			instance->include(*functor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get_transformations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:526
	// ("cv::GKernelPackage::get_transformations", vec![(pred!(const, [], []), _)]),
	void cv_GKernelPackage_get_transformations_const(const cv::GKernelPackage* instance, Result<std::vector<cv::GTransform>*>* ocvrs_return) {
		try {
			const std::vector<cv::GTransform> ret = instance->get_transformations();
			Ok(new const std::vector<cv::GTransform>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get_kernel_ids()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:533
	// ("cv::GKernelPackage::get_kernel_ids", vec![(pred!(const, [], []), _)]),
	void cv_GKernelPackage_get_kernel_ids_const(const cv::GKernelPackage* instance, Result<std::vector<std::string>*>* ocvrs_return) {
		try {
			std::vector<std::string> ret = instance->get_kernel_ids();
			Ok(new std::vector<std::string>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// remove(const cv::gapi::GBackend &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:564
	// ("cv::GKernelPackage::remove", vec![(pred!(mut, ["backend"], ["const cv::gapi::GBackend*"]), _)]),
	void cv_GKernelPackage_remove_const_GBackendR(cv::GKernelPackage* instance, const cv::gapi::GBackend* backend, ResultVoid* ocvrs_return) {
		try {
			instance->remove(*backend);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// includesAPI(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:591
	// ("cv::GKernelPackage::includesAPI", vec![(pred!(const, ["id"], ["const std::string*"]), _)]),
	void cv_GKernelPackage_includesAPI_const_const_stringR(const cv::GKernelPackage* instance, const char* id, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->includesAPI(std::string(id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lookup(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:611
	// ("cv::GKernelPackage::lookup", vec![(pred!(const, ["id"], ["const std::string*"]), _)]),
	void cv_GKernelPackage_lookup_const_const_stringR(const cv::GKernelPackage* instance, const char* id, Result<std::pair<cv::gapi::GBackend, cv::GKernelImpl>*>* ocvrs_return) {
		try {
			std::pair<cv::gapi::GBackend, cv::GKernelImpl> ret = instance->lookup(std::string(id));
			Ok(new std::pair<cv::gapi::GBackend, cv::GKernelImpl>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// include(const cv::gapi::GBackend &, const std::string &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:630
	// ("cv::GKernelPackage::include", vec![(pred!(mut, ["backend", "kernel_id"], ["const cv::gapi::GBackend*", "const std::string*"]), _)]),
	void cv_GKernelPackage_include_const_GBackendR_const_stringR(cv::GKernelPackage* instance, const cv::gapi::GBackend* backend, const char* kernel_id, ResultVoid* ocvrs_return) {
		try {
			instance->include(*backend, std::string(kernel_id));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// backends()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:637
	// ("cv::GKernelPackage::backends", vec![(pred!(const, [], []), _)]),
	void cv_GKernelPackage_backends_const(const cv::GKernelPackage* instance, Result<std::vector<cv::gapi::GBackend>*>* ocvrs_return) {
		try {
			std::vector<cv::gapi::GBackend> ret = instance->backends();
			Ok(new std::vector<cv::gapi::GBackend>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GKernelPackage::implicitClone() generated
	// ("cv::GKernelPackage::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GKernelPackage* cv_GKernelPackage_implicitClone_const(const cv::GKernelPackage* instance) {
			return new cv::GKernelPackage(*instance);
	}

	// cv::GKernelPackage::defaultNew() generated
	// ("cv::GKernelPackage::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GKernelPackage* cv_GKernelPackage_defaultNew_const() {
			cv::GKernelPackage* ret = new cv::GKernelPackage();
			return ret;
	}

	// cv::GKernelPackage::delete() generated
	// ("cv::GKernelPackage::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GKernelPackage_delete(cv::GKernelPackage* instance) {
			delete instance;
	}

	// GMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:78
	// ("cv::GMat::GMat", vec![(pred!(mut, [], []), _)]),
	void cv_GMat_GMat(Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat* ret = new cv::GMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GMat(cv::Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:90
	// ("cv::GMat::GMat", vec![(pred!(mut, ["m"], ["cv::Mat"]), _)]),
	void cv_GMat_GMat_Mat(cv::Mat* m, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat* ret = new cv::GMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GMat::implicitClone() generated
	// ("cv::GMat::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GMat* cv_GMat_implicitClone_const(const cv::GMat* instance) {
			return new cv::GMat(*instance);
	}

	// cv::GMat::delete() generated
	// ("cv::GMat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GMat_delete(cv::GMat* instance) {
			delete instance;
	}

	// GMatDesc(int, int, cv::Size, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:126
	// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "c", "s", "p"], ["int", "int", "cv::Size", "bool"]), _)]),
	void cv_GMatDesc_GMatDesc_int_int_Size_bool(int d, int c, cv::Size* s, bool p, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc* ret = new cv::GMatDesc(d, c, *s, p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GMatDesc::GMatDesc(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:126
	// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "c", "s"], ["int", "int", "cv::Size"]), _)]),
	void cv_GMatDesc_GMatDesc_int_int_Size(int d, int c, cv::Size* s, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc* ret = new cv::GMatDesc(d, c, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GMatDesc(int, const std::vector<int> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:129
	// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "dd"], ["int", "const std::vector<int>*"]), _)]),
	void cv_GMatDesc_GMatDesc_int_const_vectorLintGR(int d, const std::vector<int>* dd, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc* ret = new cv::GMatDesc(d, *dd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GMatDesc(int, std::vector<int> &&)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:132
	// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, ["d", "dd"], ["int", "std::vector<int>*"]), _)]),
	void cv_GMatDesc_GMatDesc_int_vectorLintGRR(int d, std::vector<int>* dd, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc* ret = new cv::GMatDesc(d, std::move(*dd));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GMatDesc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:135
	// ("cv::GMatDesc::GMatDesc", vec![(pred!(mut, [], []), _)]),
	void cv_GMatDesc_GMatDesc(Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc* ret = new cv::GMatDesc();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const GMatDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:137
	// ("cv::GMatDesc::operator==", vec![(pred!(const, ["rhs"], ["const cv::GMatDesc*"]), _)]),
	void cv_GMatDesc_operatorEQ_const_const_GMatDescR(const cv::GMatDesc* instance, const cv::GMatDesc* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const GMatDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:146
	// ("cv::GMatDesc::operator!=", vec![(pred!(const, ["rhs"], ["const cv::GMatDesc*"]), _)]),
	void cv_GMatDesc_operatorNE_const_const_GMatDescR(const cv::GMatDesc* instance, const cv::GMatDesc* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator!=(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isND()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:151
	// ("cv::GMatDesc::isND", vec![(pred!(const, [], []), _)]),
	void cv_GMatDesc_isND_const(const cv::GMatDesc* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isND();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// canDescribe(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:157
	// ("cv::GMatDesc::canDescribe", vec![(pred!(const, ["mat"], ["const cv::Mat*"]), _)]),
	void cv_GMatDesc_canDescribe_const_const_MatR(const cv::GMatDesc* instance, const cv::Mat* mat, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->canDescribe(*mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// canDescribe(const cv::RMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:159
	// ("cv::GMatDesc::canDescribe", vec![(pred!(const, ["mat"], ["const cv::RMat*"]), _)]),
	void cv_GMatDesc_canDescribe_const_const_RMatR(const cv::GMatDesc* instance, const cv::RMat* mat, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->canDescribe(*mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// withSizeDelta(cv::Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:164
	// ("cv::GMatDesc::withSizeDelta", vec![(pred!(const, ["delta"], ["cv::Size"]), _)]),
	void cv_GMatDesc_withSizeDelta_const_Size(const cv::GMatDesc* instance, cv::Size* delta, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->withSizeDelta(*delta);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// withSizeDelta(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:174
	// ("cv::GMatDesc::withSizeDelta", vec![(pred!(const, ["dx", "dy"], ["int", "int"]), _)]),
	void cv_GMatDesc_withSizeDelta_const_int_int(const cv::GMatDesc* instance, int dx, int dy, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->withSizeDelta(dx, dy);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// withSize(cv::Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:179
	// ("cv::GMatDesc::withSize", vec![(pred!(const, ["sz"], ["cv::Size"]), _)]),
	void cv_GMatDesc_withSize_const_Size(const cv::GMatDesc* instance, cv::Size* sz, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->withSize(*sz);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// withDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:188
	// ("cv::GMatDesc::withDepth", vec![(pred!(const, ["ddepth"], ["int"]), _)]),
	void cv_GMatDesc_withDepth_const_int(const cv::GMatDesc* instance, int ddepth, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->withDepth(ddepth);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// withType(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:199
	// ("cv::GMatDesc::withType", vec![(pred!(const, ["ddepth", "dchan"], ["int", "int"]), _)]),
	void cv_GMatDesc_withType_const_int_int(const cv::GMatDesc* instance, int ddepth, int dchan, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->withType(ddepth, dchan);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// asPlanar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:210
	// ("cv::GMatDesc::asPlanar", vec![(pred!(const, [], []), _)]),
	void cv_GMatDesc_asPlanar_const(const cv::GMatDesc* instance, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->asPlanar();
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// asPlanar(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:221
	// ("cv::GMatDesc::asPlanar", vec![(pred!(const, ["planes"], ["int"]), _)]),
	void cv_GMatDesc_asPlanar_const_int(const cv::GMatDesc* instance, int planes, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->asPlanar(planes);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// asInterleaved()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:236
	// ("cv::GMatDesc::asInterleaved", vec![(pred!(const, [], []), _)]),
	void cv_GMatDesc_asInterleaved_const(const cv::GMatDesc* instance, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->asInterleaved();
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GMatDesc::implicitClone() generated
	// ("cv::GMatDesc::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GMatDesc* cv_GMatDesc_implicitClone_const(const cv::GMatDesc* instance) {
			return new cv::GMatDesc(*instance);
	}

	// cv::GMatDesc::depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:120
	// ("cv::GMatDesc::depth", vec![(pred!(const, [], []), _)]),
	int cv_GMatDesc_propDepth_const(const cv::GMatDesc* instance) {
			int ret = instance->depth;
			return ret;
	}

	// cv::GMatDesc::setDepth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:120
	// ("cv::GMatDesc::setDepth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_GMatDesc_propDepth_const_int(cv::GMatDesc* instance, const int val) {
			instance->depth = val;
	}

	// cv::GMatDesc::chan() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:121
	// ("cv::GMatDesc::chan", vec![(pred!(const, [], []), _)]),
	int cv_GMatDesc_propChan_const(const cv::GMatDesc* instance) {
			int ret = instance->chan;
			return ret;
	}

	// cv::GMatDesc::setChan(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:121
	// ("cv::GMatDesc::setChan", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_GMatDesc_propChan_const_int(cv::GMatDesc* instance, const int val) {
			instance->chan = val;
	}

	// cv::GMatDesc::size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:122
	// ("cv::GMatDesc::size", vec![(pred!(const, [], []), _)]),
	void cv_GMatDesc_propSize_const(const cv::GMatDesc* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->size;
			*ocvrs_return = ret;
	}

	// cv::GMatDesc::setSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:122
	// ("cv::GMatDesc::setSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_GMatDesc_propSize_const_Size(cv::GMatDesc* instance, const cv::Size* val) {
			instance->size = *val;
	}

	// cv::GMatDesc::planar() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:123
	// ("cv::GMatDesc::planar", vec![(pred!(const, [], []), _)]),
	bool cv_GMatDesc_propPlanar_const(const cv::GMatDesc* instance) {
			bool ret = instance->planar;
			return ret;
	}

	// cv::GMatDesc::setPlanar(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:123
	// ("cv::GMatDesc::setPlanar", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_GMatDesc_propPlanar_const_bool(cv::GMatDesc* instance, const bool val) {
			instance->planar = val;
	}

	// cv::GMatDesc::dims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:124
	// ("cv::GMatDesc::dims", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_GMatDesc_propDims_const(const cv::GMatDesc* instance) {
			std::vector<int> ret = instance->dims;
			return new std::vector<int>(ret);
	}

	// cv::GMatDesc::setDims(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gmat.hpp:124
	// ("cv::GMatDesc::setDims", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_GMatDesc_propDims_const_vectorLintG(cv::GMatDesc* instance, const std::vector<int>* val) {
			instance->dims = *val;
	}

	// cv::GMatDesc::delete() generated
	// ("cv::GMatDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GMatDesc_delete(cv::GMatDesc* instance) {
			delete instance;
	}

	// cv::GMatP::defaultNew() generated
	// ("cv::GMatP::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GMatP* cv_GMatP_defaultNew_const() {
			cv::GMatP* ret = new cv::GMatP();
			return ret;
	}

	// cv::GMatP::to_GMat() generated
	// ("cv::GMatP::to_GMat", vec![(pred!(mut, [], []), _)]),
	cv::GMat* cv_GMatP_to_GMat(cv::GMatP* instance) {
			return dynamic_cast<cv::GMat*>(instance);
	}

	// cv::GMatP::delete() generated
	// ("cv::GMatP::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GMatP_delete(cv::GMatP* instance) {
			delete instance;
	}

	// operator==(const GOpaqueDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gopaque.hpp:43
	// ("cv::GOpaqueDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GOpaqueDesc*"]), _)]),
	void cv_GOpaqueDesc_operatorEQ_const_const_GOpaqueDescR(const cv::GOpaqueDesc* instance, const cv::GOpaqueDesc* unnamed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GOpaqueDesc::implicitClone() generated
	// ("cv::GOpaqueDesc::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GOpaqueDesc* cv_GOpaqueDesc_implicitClone_const(const cv::GOpaqueDesc* instance) {
			return new cv::GOpaqueDesc(*instance);
	}

	// cv::GOpaqueDesc::defaultNew() generated
	// ("cv::GOpaqueDesc::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GOpaqueDesc* cv_GOpaqueDesc_defaultNew_const() {
			cv::GOpaqueDesc* ret = new cv::GOpaqueDesc();
			return ret;
	}

	// cv::GOpaqueDesc::delete() generated
	// ("cv::GOpaqueDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GOpaqueDesc_delete(cv::GOpaqueDesc* instance) {
			delete instance;
	}

	// GRunArg()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:129
	// ("cv::GRunArg::GRunArg", vec![(pred!(mut, [], []), _)]),
	void cv_GRunArg_GRunArg(Result<cv::GRunArg*>* ocvrs_return) {
		try {
			cv::GRunArg* ret = new cv::GRunArg();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GRunArg(const cv::GRunArg &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:130
	// ("cv::GRunArg::GRunArg", vec![(pred!(mut, ["arg"], ["const cv::GRunArg*"]), _)]),
	void cv_GRunArg_GRunArg_const_GRunArgR(const cv::GRunArg* arg, Result<cv::GRunArg*>* ocvrs_return) {
		try {
			cv::GRunArg* ret = new cv::GRunArg(*arg);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GRunArg(cv::GRunArg &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:131
	// ("cv::GRunArg::GRunArg", vec![(pred!(mut, ["arg"], ["cv::GRunArg*"]), _)]),
	void cv_GRunArg_GRunArg_GRunArgRR(cv::GRunArg* arg, Result<cv::GRunArg*>* ocvrs_return) {
		try {
			cv::GRunArg* ret = new cv::GRunArg(std::move(*arg));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const GRunArg &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:133
	// ("cv::GRunArg::operator=", vec![(pred!(mut, ["arg"], ["const cv::GRunArg*"]), _)]),
	void cv_GRunArg_operatorST_const_GRunArgR(cv::GRunArg* instance, const cv::GRunArg* arg, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*arg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(GRunArg &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:134
	// ("cv::GRunArg::operator=", vec![(pred!(mut, ["arg"], ["cv::GRunArg*"]), _)]),
	void cv_GRunArg_operatorST_GRunArgRR(cv::GRunArg* instance, cv::GRunArg* arg, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(std::move(*arg));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GRunArg::delete() generated
	// ("cv::GRunArg::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GRunArg_delete(cv::GRunArg* instance) {
			delete instance;
	}

	// GScalar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:52
	// ("cv::GScalar::GScalar", vec![(pred!(mut, [], []), _)]),
	void cv_GScalar_GScalar(Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar* ret = new cv::GScalar();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GScalar(const cv::Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:70
	// ("cv::GScalar::GScalar", vec![(pred!(mut, ["s"], ["const cv::Scalar*"]), _)]),
	void cv_GScalar_GScalar_const_ScalarR(const cv::Scalar* s, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar* ret = new cv::GScalar(*s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GScalar(cv::Scalar &&)(ByMove) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:78
	// ("cv::GScalar::GScalar", vec![(pred!(mut, ["s"], ["cv::Scalar*"]), _)]),
	void cv_GScalar_GScalar_ScalarRR(cv::Scalar* s, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar* ret = new cv::GScalar(std::move(*s));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GScalar(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:98
	// ("cv::GScalar::GScalar", vec![(pred!(mut, ["v0"], ["double"]), _)]),
	void cv_GScalar_GScalar_double(double v0, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar* ret = new cv::GScalar(v0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GScalar::implicitClone() generated
	// ("cv::GScalar::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GScalar* cv_GScalar_implicitClone_const(const cv::GScalar* instance) {
			return new cv::GScalar(*instance);
	}

	// cv::GScalar::delete() generated
	// ("cv::GScalar::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GScalar_delete(cv::GScalar* instance) {
			delete instance;
	}

	// operator==(const GScalarDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:121
	// ("cv::GScalarDesc::operator==", vec![(pred!(const, ["unnamed"], ["const cv::GScalarDesc*"]), _)]),
	void cv_GScalarDesc_operatorEQ_const_const_GScalarDescR(const cv::GScalarDesc* instance, const cv::GScalarDesc* unnamed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const GScalarDesc &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gscalar.hpp:126
	// ("cv::GScalarDesc::operator!=", vec![(pred!(const, ["rhs"], ["const cv::GScalarDesc*"]), _)]),
	void cv_GScalarDesc_operatorNE_const_const_GScalarDescR(const cv::GScalarDesc* instance, const cv::GScalarDesc* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator!=(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GScalarDesc::implicitClone() generated
	// ("cv::GScalarDesc::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GScalarDesc* cv_GScalarDesc_implicitClone_const(const cv::GScalarDesc* instance) {
			return new cv::GScalarDesc(*instance);
	}

	// cv::GScalarDesc::defaultNew() generated
	// ("cv::GScalarDesc::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GScalarDesc* cv_GScalarDesc_defaultNew_const() {
			cv::GScalarDesc* ret = new cv::GScalarDesc();
			return ret;
	}

	// cv::GScalarDesc::delete() generated
	// ("cv::GScalarDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GScalarDesc_delete(cv::GScalarDesc* instance) {
			delete instance;
	}

	// GStreamingCompiled()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:159
	// ("cv::GStreamingCompiled::GStreamingCompiled", vec![(pred!(mut, [], []), _)]),
	void cv_GStreamingCompiled_GStreamingCompiled(Result<cv::GStreamingCompiled*>* ocvrs_return) {
		try {
			cv::GStreamingCompiled* ret = new cv::GStreamingCompiled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSource(GRunArgs &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:202
	// ("cv::GStreamingCompiled::setSource", vec![(pred!(mut, ["ins"], ["cv::GRunArgs*"]), _)]),
	void cv_GStreamingCompiled_setSource_GRunArgsRR(cv::GStreamingCompiled* instance, cv::GRunArgs* ins, ResultVoid* ocvrs_return) {
		try {
			instance->setSource(std::move(*ins));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSource(const cv::detail::ExtractArgsCallback &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:205
	// ("cv::GStreamingCompiled::setSource", vec![(pred!(mut, ["callback"], ["const cv::detail::ExtractArgsCallback*"]), _)]),
	void cv_GStreamingCompiled_setSource_const_ExtractArgsCallbackR(cv::GStreamingCompiled* instance, const cv::detail::ExtractArgsCallback* callback, ResultVoid* ocvrs_return) {
		try {
			instance->setSource(*callback);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// start()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:251
	// ("cv::GStreamingCompiled::start", vec![(pred!(mut, [], []), _)]),
	void cv_GStreamingCompiled_start(cv::GStreamingCompiled* instance, ResultVoid* ocvrs_return) {
		try {
			instance->start();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:339
	// ("cv::GStreamingCompiled::stop", vec![(pred!(mut, [], []), _)]),
	void cv_GStreamingCompiled_stop(cv::GStreamingCompiled* instance, ResultVoid* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// running()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:351
	// ("cv::GStreamingCompiled::running", vec![(pred!(const, [], []), _)]),
	void cv_GStreamingCompiled_running_const(const cv::GStreamingCompiled* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->running();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator bool()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:361
	// ("cv::GStreamingCompiled::operator bool", vec![(pred!(const, [], []), _)]),
	void cv_GStreamingCompiled_operator_bool_const(const cv::GStreamingCompiled* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator bool();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GStreamingCompiled::implicitClone() generated
	// ("cv::GStreamingCompiled::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GStreamingCompiled* cv_GStreamingCompiled_implicitClone_const(const cv::GStreamingCompiled* instance) {
			return new cv::GStreamingCompiled(*instance);
	}

	// cv::GStreamingCompiled::delete() generated
	// ("cv::GStreamingCompiled::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GStreamingCompiled_delete(cv::GStreamingCompiled* instance) {
			delete instance;
	}

	// cv::GTransform::description() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gtransform.hpp:30
	// ("cv::GTransform::description", vec![(pred!(const, [], []), _)]),
	void* cv_GTransform_propDescription_const(const cv::GTransform* instance) {
			std::string ret = instance->description;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::GTransform::setDescription(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gtransform.hpp:30
	// ("cv::GTransform::setDescription", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_GTransform_propDescription_const_string(cv::GTransform* instance, const char* val) {
			instance->description = std::string(val);
	}

	// cv::GTransform::delete() generated
	// ("cv::GTransform::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GTransform_delete(cv::GTransform* instance) {
			delete instance;
	}

	// cv::GTypeInfo::implicitClone() generated
	// ("cv::GTypeInfo::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::GTypeInfo* cv_GTypeInfo_implicitClone_const(const cv::GTypeInfo* instance) {
			return new cv::GTypeInfo(*instance);
	}

	// cv::GTypeInfo::defaultNew() generated
	// ("cv::GTypeInfo::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::GTypeInfo* cv_GTypeInfo_defaultNew_const() {
			cv::GTypeInfo* ret = new cv::GTypeInfo();
			return ret;
	}

	// cv::GTypeInfo::shape() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:31
	// ("cv::GTypeInfo::shape", vec![(pred!(const, [], []), _)]),
	void cv_GTypeInfo_propShape_const(const cv::GTypeInfo* instance, cv::GShape* ocvrs_return) {
			cv::GShape ret = instance->shape;
			*ocvrs_return = ret;
	}

	// cv::GTypeInfo::setShape(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:31
	// ("cv::GTypeInfo::setShape", vec![(pred!(mut, ["val"], ["const cv::GShape"]), _)]),
	void cv_GTypeInfo_propShape_const_GShape(cv::GTypeInfo* instance, const cv::GShape val) {
			instance->shape = val;
	}

	// cv::GTypeInfo::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:32
	// ("cv::GTypeInfo::kind", vec![(pred!(const, [], []), _)]),
	void cv_GTypeInfo_propKind_const(const cv::GTypeInfo* instance, cv::detail::OpaqueKind* ocvrs_return) {
			cv::detail::OpaqueKind ret = instance->kind;
			*ocvrs_return = ret;
	}

	// cv::GTypeInfo::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:32
	// ("cv::GTypeInfo::setKind", vec![(pred!(mut, ["val"], ["const cv::detail::OpaqueKind"]), _)]),
	void cv_GTypeInfo_propKind_const_OpaqueKind(cv::GTypeInfo* instance, const cv::detail::OpaqueKind val) {
			instance->kind = val;
	}

	// cv::GTypeInfo::delete() generated
	// ("cv::GTypeInfo::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GTypeInfo_delete(cv::GTypeInfo* instance) {
			delete instance;
	}

	// MediaFrame()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:71
	// ("cv::MediaFrame::MediaFrame", vec![(pred!(mut, [], []), _)]),
	void cv_MediaFrame_MediaFrame(Result<cv::MediaFrame*>* ocvrs_return) {
		try {
			cv::MediaFrame* ret = new cv::MediaFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:111
	// ("cv::MediaFrame::desc", vec![(pred!(const, [], []), _)]),
	void cv_MediaFrame_desc_const(const cv::MediaFrame* instance, Result<cv::GFrameDesc*>* ocvrs_return) {
		try {
			cv::GFrameDesc ret = instance->desc();
			Ok(new cv::GFrameDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:116
	// ("cv::MediaFrame::blobParams", vec![(pred!(const, [], []), _)]),
	void cv_MediaFrame_blobParams_const(const cv::MediaFrame* instance, Result<cv::util::any*>* ocvrs_return) {
		try {
			cv::util::any ret = instance->blobParams();
			Ok(new cv::util::any(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MediaFrame::delete() generated
	// ("cv::MediaFrame::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MediaFrame_delete(cv::MediaFrame* instance) {
			delete instance;
	}

	// meta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:240
	// ("cv::MediaFrame::IAdapter::meta", vec![(pred!(const, [], []), _)]),
	void cv_MediaFrame_IAdapter_meta_const(const cv::MediaFrame::IAdapter* instance, Result<cv::GFrameDesc*>* ocvrs_return) {
		try {
			cv::GFrameDesc ret = instance->meta();
			Ok(new cv::GFrameDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:244
	// ("cv::MediaFrame::IAdapter::blobParams", vec![(pred!(const, [], []), _)]),
	void cv_MediaFrame_IAdapter_blobParams_const(const cv::MediaFrame::IAdapter* instance, Result<cv::util::any*>* ocvrs_return) {
		try {
			cv::util::any ret = instance->blobParams();
			Ok(new cv::util::any(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MediaFrame::IAdapter::delete() generated
	// ("cv::MediaFrame::IAdapter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MediaFrame_IAdapter_delete(cv::MediaFrame::IAdapter* instance) {
			delete instance;
	}

	// View(View &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/media.hpp:212
	// ("cv::MediaFrame::View::View", vec![(pred!(mut, ["unnamed"], ["cv::MediaFrame::View*"]), _)]),
	cv::MediaFrame::View* cv_MediaFrame_View_View_ViewRR(cv::MediaFrame::View* unnamed) {
			cv::MediaFrame::View* ret = new cv::MediaFrame::View(std::move(*unnamed));
			return ret;
	}

	// cv::MediaFrame::View::delete() generated
	// ("cv::MediaFrame::View::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MediaFrame_View_delete(cv::MediaFrame::View* instance) {
			delete instance;
	}

	// RMat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:128
	// ("cv::RMat::RMat", vec![(pred!(mut, [], []), _)]),
	cv::RMat* cv_RMat_RMat() {
			cv::RMat* ret = new cv::RMat();
			return ret;
	}

	// desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:130
	// ("cv::RMat::desc", vec![(pred!(const, [], []), _)]),
	void cv_RMat_desc_const(const cv::RMat* instance, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->desc();
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RMat::delete() generated
	// ("cv::RMat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RMat_delete(cv::RMat* instance) {
			delete instance;
	}

	// desc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:111
	// ("cv::RMat::IAdapter::desc", vec![(pred!(const, [], []), _)]),
	void cv_RMat_IAdapter_desc_const(const cv::RMat::IAdapter* instance, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->desc();
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RMat::IAdapter::delete() generated
	// ("cv::RMat::IAdapter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RMat_IAdapter_delete(cv::RMat::IAdapter* instance) {
			delete instance;
	}

	// View()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:62
	// ("cv::RMat::View::View", vec![(pred!(mut, [], []), _)]),
	cv::RMat::View* cv_RMat_View_View() {
			cv::RMat::View* ret = new cv::RMat::View();
			return ret;
	}

	// View(View &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:68
	// ("cv::RMat::View::View", vec![(pred!(mut, ["unnamed"], ["cv::RMat::View*"]), _)]),
	cv::RMat::View* cv_RMat_View_View_ViewRR(cv::RMat::View* unnamed) {
			cv::RMat::View* ret = new cv::RMat::View(std::move(*unnamed));
			return ret;
	}

	// operator=(View &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:69
	// ("cv::RMat::View::operator=", vec![(pred!(mut, ["v"], ["cv::RMat::View*"]), _)]),
	void cv_RMat_View_operatorST_ViewRR(cv::RMat::View* instance, cv::RMat::View* v, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(std::move(*v));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:72
	// ("cv::RMat::View::size", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_size_const(const cv::RMat::View* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dims()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:73
	// ("cv::RMat::View::dims", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_dims_const(const cv::RMat::View* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->dims();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:74
	// ("cv::RMat::View::cols", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_cols_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:75
	// ("cv::RMat::View::rows", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_rows_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:76
	// ("cv::RMat::View::type", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_type_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:77
	// ("cv::RMat::View::depth", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_depth_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// chan()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:78
	// ("cv::RMat::View::chan", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_chan_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->chan();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:79
	// ("cv::RMat::View::elemSize", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_elemSize_const(const cv::RMat::View* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// step(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:93
	// ("cv::RMat::View::step", vec![(pred!(const, ["i"], ["size_t"]), _)]),
	void cv_RMat_View_step_const_size_t(const cv::RMat::View* instance, size_t i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RMat::View::step() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:93
	// ("cv::RMat::View::step", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_step_const(const cv::RMat::View* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// steps()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/rmat.hpp:96
	// ("cv::RMat::View::steps", vec![(pred!(const, [], []), _)]),
	void cv_RMat_View_steps_const(const cv::RMat::View* instance, Result<cv::RMat::View::stepsT*>* ocvrs_return) {
		try {
			const cv::RMat::View::stepsT ret = instance->steps();
			Ok(new const cv::RMat::View::stepsT(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RMat::View::delete() generated
	// ("cv::RMat::View::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RMat_View_delete(cv::RMat::View* instance) {
			delete instance;
	}

	// operator()(const cv::GTypesInfo &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/garg.hpp:292
	// ("cv::detail::ExtractArgsCallback::operator()", vec![(pred!(const, ["info"], ["const cv::GTypesInfo*"]), _)]),
	void cv_detail_ExtractArgsCallback_operator___const_const_GTypesInfoR(const cv::detail::ExtractArgsCallback* instance, const cv::GTypesInfo* info, Result<cv::GRunArgs*>* ocvrs_return) {
		try {
			cv::GRunArgs ret = instance->operator()(*info);
			Ok(new cv::GRunArgs(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detail::ExtractArgsCallback::defaultNew() generated
	// ("cv::detail::ExtractArgsCallback::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::ExtractArgsCallback* cv_detail_ExtractArgsCallback_defaultNew_const() {
			cv::detail::ExtractArgsCallback* ret = new cv::detail::ExtractArgsCallback();
			return ret;
	}

	// cv::detail::ExtractArgsCallback::delete() generated
	// ("cv::detail::ExtractArgsCallback::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ExtractArgsCallback_delete(cv::detail::ExtractArgsCallback* instance) {
			delete instance;
	}

	// cv::detail::ExtractMetaCallback::defaultNew() generated
	// ("cv::detail::ExtractMetaCallback::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::ExtractMetaCallback* cv_detail_ExtractMetaCallback_defaultNew_const() {
			cv::detail::ExtractMetaCallback* ret = new cv::detail::ExtractMetaCallback();
			return ret;
	}

	// cv::detail::ExtractMetaCallback::delete() generated
	// ("cv::detail::ExtractMetaCallback::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_ExtractMetaCallback_delete(cv::detail::ExtractMetaCallback* instance) {
			delete instance;
	}

	// cv::detail::GArrayU::delete() generated
	// ("cv::detail::GArrayU::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GArrayU_delete(cv::detail::GArrayU* instance) {
			delete instance;
	}

	// cv::detail::GOpaqueU::delete() generated
	// ("cv::detail::GOpaqueU::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_GOpaqueU_delete(cv::detail::GOpaqueU* instance) {
			delete instance;
	}

	// GBackend()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:385
	// ("cv::gapi::GBackend::GBackend", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_GBackend_GBackend(Result<cv::gapi::GBackend*>* ocvrs_return) {
		try {
			cv::gapi::GBackend* ret = new cv::gapi::GBackend();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const GBackend &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:392
	// ("cv::gapi::GBackend::operator==", vec![(pred!(const, ["rhs"], ["const cv::gapi::GBackend*"]), _)]),
	void cv_gapi_GBackend_operatorEQ_const_const_GBackendR(const cv::gapi::GBackend* instance, const cv::gapi::GBackend* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::GBackend::delete() generated
	// ("cv::gapi::GBackend::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_GBackend_delete(cv::gapi::GBackend* instance) {
			delete instance;
	}

	// impl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:427
	// ("cv::gapi::GFunctor::impl", vec![(pred!(const, [], []), _)]),
	void cv_gapi_GFunctor_impl_const(const cv::gapi::GFunctor* instance, Result<cv::GKernelImpl*>* ocvrs_return) {
		try {
			cv::GKernelImpl ret = instance->impl();
			Ok(new cv::GKernelImpl(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// backend()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:428
	// ("cv::gapi::GFunctor::backend", vec![(pred!(const, [], []), _)]),
	void cv_gapi_GFunctor_backend_const(const cv::gapi::GFunctor* instance, Result<cv::gapi::GBackend*>* ocvrs_return) {
		try {
			cv::gapi::GBackend ret = instance->backend();
			Ok(new cv::gapi::GBackend(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// id()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:429
	// ("cv::gapi::GFunctor::id", vec![(pred!(const, [], []), _)]),
	void cv_gapi_GFunctor_id_const(const cv::gapi::GFunctor* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->id();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::GFunctor::delete() generated
	// ("cv::gapi::GFunctor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_GFunctor_delete(cv::gapi::GFunctor* instance) {
			delete instance;
	}

	// Scalar()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:23
	// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, [], []), _)]),
	cv::gapi::own::Scalar* cv_gapi_own_Scalar_Scalar() {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar();
			return ret;
	}

	// Scalar(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:24
	// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, ["v0"], ["double"]), _)]),
	void cv_gapi_own_Scalar_Scalar_double(double v0, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar(v0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Scalar(double, double, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:25
	// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, ["v0", "v1", "v2", "v3"], ["double", "double", "double", "double"]), _)]),
	void cv_gapi_own_Scalar_Scalar_double_double_double_double(double v0, double v1, double v2, double v3, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar(v0, v1, v2, v3);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::own::Scalar::Scalar(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:25
	// ("cv::gapi::own::Scalar::Scalar", vec![(pred!(mut, ["v0", "v1"], ["double", "double"]), _)]),
	void cv_gapi_own_Scalar_Scalar_double_double(double v0, double v1, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar(v0, v1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:30
	// ("cv::gapi::own::Scalar::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv_gapi_own_Scalar_operator___const_int(const cv::gapi::own::Scalar* instance, int i, Result<double>* ocvrs_return) {
		try {
			const double ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:31
	// ("cv::gapi::own::Scalar::operator[]", vec![(pred!(mut, ["i"], ["int"]), _)]),
	void cv_gapi_own_Scalar_operator___int(cv::gapi::own::Scalar* instance, int i, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// all(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:33
	// ("cv::gapi::own::Scalar::all", vec![(pred!(mut, ["v0"], ["double"]), _)]),
	void cv_gapi_own_Scalar_all_double(double v0, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar ret = cv::gapi::own::Scalar::all(v0);
			Ok(new cv::gapi::own::Scalar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::own::Scalar::val() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:35
	// ("cv::gapi::own::Scalar::val", vec![(pred!(const, [], []), _)]),
	const double** cv_gapi_own_Scalar_propVal_const(const cv::gapi::own::Scalar* instance) {
			const double(*ret)[4] = &instance->val;
			return (const double**)ret;
	}

	// cv::gapi::own::Scalar::valMut() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/own/scalar.hpp:35
	// ("cv::gapi::own::Scalar::valMut", vec![(pred!(mut, [], []), _)]),
	double** cv_gapi_own_Scalar_propVal(cv::gapi::own::Scalar* instance) {
			double(*ret)[4] = &instance->val;
			return (double**)ret;
	}

	// cv::gapi::own::Scalar::delete() generated
	// ("cv::gapi::own::Scalar::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_own_Scalar_delete(cv::gapi::own::Scalar* instance) {
			delete instance;
	}

	// queue_capacity(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:411
	// ("cv::gapi::streaming::queue_capacity::queue_capacity", vec![(pred!(mut, ["cap"], ["size_t"]), _)]),
	void cv_gapi_streaming_queue_capacity_queue_capacity_size_t(size_t cap, Result<cv::gapi::streaming::queue_capacity>* ocvrs_return) {
		try {
			cv::gapi::streaming::queue_capacity ret(cap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::streaming::queue_capacity::queue_capacity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gstreaming.hpp:411
	// ("cv::gapi::streaming::queue_capacity::queue_capacity", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_streaming_queue_capacity_queue_capacity(Result<cv::gapi::streaming::queue_capacity>* ocvrs_return) {
		try {
			cv::gapi::streaming::queue_capacity ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::use_only::defaultNew() generated
	// ("cv::gapi::use_only::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::gapi::use_only* cv_gapi_use_only_defaultNew_const() {
			cv::gapi::use_only* ret = new cv::gapi::use_only();
			return ret;
	}

	// cv::gapi::use_only::pkg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:736
	// ("cv::gapi::use_only::pkg", vec![(pred!(const, [], []), _)]),
	cv::gapi::GKernelPackage* cv_gapi_use_only_propPkg_const(const cv::gapi::use_only* instance) {
			cv::gapi::GKernelPackage ret = instance->pkg;
			return new cv::gapi::GKernelPackage(ret);
	}

	// cv::gapi::use_only::setPkg(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gkernel.hpp:736
	// ("cv::gapi::use_only::setPkg", vec![(pred!(mut, ["val"], ["const cv::gapi::GKernelPackage"]), _)]),
	void cv_gapi_use_only_propPkg_const_GKernelPackage(cv::gapi::use_only* instance, const cv::gapi::GKernelPackage* val) {
			instance->pkg = *val;
	}

	// cv::gapi::use_only::delete() generated
	// ("cv::gapi::use_only::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_use_only_delete(cv::gapi::use_only* instance) {
			delete instance;
	}

	// cv::gapi::wip::Data::defaultNew() generated
	// ("cv::gapi::wip::Data::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::gapi::wip::Data* cv_gapi_wip_Data_defaultNew_const() {
			cv::gapi::wip::Data* ret = new cv::gapi::wip::Data();
			return ret;
	}

	// cv::gapi::wip::Data::to_GRunArg() generated
	// ("cv::gapi::wip::Data::to_GRunArg", vec![(pred!(mut, [], []), _)]),
	cv::GRunArg* cv_gapi_wip_Data_to_GRunArg(cv::gapi::wip::Data* instance) {
			return dynamic_cast<cv::GRunArg*>(instance);
	}

	// cv::gapi::wip::Data::delete() generated
	// ("cv::gapi::wip::Data::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_Data_delete(cv::gapi::wip::Data* instance) {
			delete instance;
	}

	// Circle(const cv::Point &, int, const cv::Scalar &, int, int, int)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:177
	// ("cv::gapi::wip::draw::Circle::Circle", vec![(pred!(mut, ["center_", "radius_", "color_", "thick_", "lt_", "shift_"], ["const cv::Point*", "int", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_gapi_wip_draw_Circle_Circle_const_PointR_int_const_ScalarR_int_int_int(const cv::Point* center_, int radius_, const cv::Scalar* color_, int thick_, int lt_, int shift_, Result<cv::gapi::wip::draw::Circle>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Circle ret(*center_, radius_, *color_, thick_, lt_, shift_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::wip::draw::Circle::Circle(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:177
	// ("cv::gapi::wip::draw::Circle::Circle", vec![(pred!(mut, ["center_", "radius_", "color_"], ["const cv::Point*", "int", "const cv::Scalar*"]), _)]),
	void cv_gapi_wip_draw_Circle_Circle_const_PointR_int_const_ScalarR(const cv::Point* center_, int radius_, const cv::Scalar* color_, Result<cv::gapi::wip::draw::Circle>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Circle ret(*center_, radius_, *color_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Circle()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:188
	// ("cv::gapi::wip::draw::Circle::Circle", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Circle_Circle(cv::gapi::wip::draw::Circle* ocvrs_return) {
			cv::gapi::wip::draw::Circle ret;
			*ocvrs_return = ret;
	}

	// Image(const cv::Point &, const cv::Mat &, const cv::Mat &)(SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:287
	// ("cv::gapi::wip::draw::Image::Image", vec![(pred!(mut, ["org_", "img_", "alpha_"], ["const cv::Point*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_gapi_wip_draw_Image_Image_const_PointR_const_MatR_const_MatR(const cv::Point* org_, const cv::Mat* img_, const cv::Mat* alpha_, Result<cv::gapi::wip::draw::Image*>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Image* ret = new cv::gapi::wip::draw::Image(*org_, *img_, *alpha_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Image()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:295
	// ("cv::gapi::wip::draw::Image::Image", vec![(pred!(mut, [], []), _)]),
	cv::gapi::wip::draw::Image* cv_gapi_wip_draw_Image_Image() {
			cv::gapi::wip::draw::Image* ret = new cv::gapi::wip::draw::Image();
			return ret;
	}

	// cv::gapi::wip::draw::Image::implicitClone() generated
	// ("cv::gapi::wip::draw::Image::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::gapi::wip::draw::Image* cv_gapi_wip_draw_Image_implicitClone_const(const cv::gapi::wip::draw::Image* instance) {
			return new cv::gapi::wip::draw::Image(*instance);
	}

	// cv::gapi::wip::draw::Image::org() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:298
	// ("cv::gapi::wip::draw::Image::org", vec![(pred!(const, [], []), _)]),
	void cv_gapi_wip_draw_Image_propOrg_const(const cv::gapi::wip::draw::Image* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->org;
			*ocvrs_return = ret;
	}

	// cv::gapi::wip::draw::Image::setOrg(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:298
	// ("cv::gapi::wip::draw::Image::setOrg", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
	void cv_gapi_wip_draw_Image_propOrg_const_Point(cv::gapi::wip::draw::Image* instance, const cv::Point* val) {
			instance->org = *val;
	}

	// cv::gapi::wip::draw::Image::img() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:299
	// ("cv::gapi::wip::draw::Image::img", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_gapi_wip_draw_Image_propImg_const(const cv::gapi::wip::draw::Image* instance) {
			cv::Mat ret = instance->img;
			return new cv::Mat(ret);
	}

	// cv::gapi::wip::draw::Image::setImg(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:299
	// ("cv::gapi::wip::draw::Image::setImg", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_gapi_wip_draw_Image_propImg_const_Mat(cv::gapi::wip::draw::Image* instance, const cv::Mat* val) {
			instance->img = *val;
	}

	// cv::gapi::wip::draw::Image::alpha() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:300
	// ("cv::gapi::wip::draw::Image::alpha", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_gapi_wip_draw_Image_propAlpha_const(const cv::gapi::wip::draw::Image* instance) {
			cv::Mat ret = instance->alpha;
			return new cv::Mat(ret);
	}

	// cv::gapi::wip::draw::Image::setAlpha(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:300
	// ("cv::gapi::wip::draw::Image::setAlpha", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_gapi_wip_draw_Image_propAlpha_const_Mat(cv::gapi::wip::draw::Image* instance, const cv::Mat* val) {
			instance->alpha = *val;
	}

	// cv::gapi::wip::draw::Image::delete() generated
	// ("cv::gapi::wip::draw::Image::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Image_delete(cv::gapi::wip::draw::Image* instance) {
			delete instance;
	}

	// Line(const cv::Point &, const cv::Point &, const cv::Scalar &, int, int, int)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:218
	// ("cv::gapi::wip::draw::Line::Line", vec![(pred!(mut, ["pt1_", "pt2_", "color_", "thick_", "lt_", "shift_"], ["const cv::Point*", "const cv::Point*", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_gapi_wip_draw_Line_Line_const_PointR_const_PointR_const_ScalarR_int_int_int(const cv::Point* pt1_, const cv::Point* pt2_, const cv::Scalar* color_, int thick_, int lt_, int shift_, Result<cv::gapi::wip::draw::Line>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Line ret(*pt1_, *pt2_, *color_, thick_, lt_, shift_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::wip::draw::Line::Line(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:218
	// ("cv::gapi::wip::draw::Line::Line", vec![(pred!(mut, ["pt1_", "pt2_", "color_"], ["const cv::Point*", "const cv::Point*", "const cv::Scalar*"]), _)]),
	void cv_gapi_wip_draw_Line_Line_const_PointR_const_PointR_const_ScalarR(const cv::Point* pt1_, const cv::Point* pt2_, const cv::Scalar* color_, Result<cv::gapi::wip::draw::Line>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Line ret(*pt1_, *pt2_, *color_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Line()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:229
	// ("cv::gapi::wip::draw::Line::Line", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Line_Line(cv::gapi::wip::draw::Line* ocvrs_return) {
			cv::gapi::wip::draw::Line ret;
			*ocvrs_return = ret;
	}

	// Mosaic(const cv::Rect &, int, int)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:255
	// ("cv::gapi::wip::draw::Mosaic::Mosaic", vec![(pred!(mut, ["mos_", "cellSz_", "decim_"], ["const cv::Rect*", "int", "int"]), _)]),
	void cv_gapi_wip_draw_Mosaic_Mosaic_const_RectR_int_int(const cv::Rect* mos_, int cellSz_, int decim_, Result<cv::gapi::wip::draw::Mosaic>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Mosaic ret(*mos_, cellSz_, decim_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mosaic()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:263
	// ("cv::gapi::wip::draw::Mosaic::Mosaic", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Mosaic_Mosaic(Result<cv::gapi::wip::draw::Mosaic>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Mosaic ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Poly(const std::vector<cv::Point> &, const cv::Scalar &, int, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:319
	// ("cv::gapi::wip::draw::Poly::Poly", vec![(pred!(mut, ["points_", "color_", "thick_", "lt_", "shift_"], ["const std::vector<cv::Point>*", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_gapi_wip_draw_Poly_Poly_const_vectorLPointGR_const_ScalarR_int_int_int(const std::vector<cv::Point>* points_, const cv::Scalar* color_, int thick_, int lt_, int shift_, Result<cv::gapi::wip::draw::Poly*>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Poly* ret = new cv::gapi::wip::draw::Poly(*points_, *color_, thick_, lt_, shift_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::wip::draw::Poly::Poly(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:319
	// ("cv::gapi::wip::draw::Poly::Poly", vec![(pred!(mut, ["points_", "color_"], ["const std::vector<cv::Point>*", "const cv::Scalar*"]), _)]),
	void cv_gapi_wip_draw_Poly_Poly_const_vectorLPointGR_const_ScalarR(const std::vector<cv::Point>* points_, const cv::Scalar* color_, Result<cv::gapi::wip::draw::Poly*>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Poly* ret = new cv::gapi::wip::draw::Poly(*points_, *color_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Poly()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:329
	// ("cv::gapi::wip::draw::Poly::Poly", vec![(pred!(mut, [], []), _)]),
	cv::gapi::wip::draw::Poly* cv_gapi_wip_draw_Poly_Poly() {
			cv::gapi::wip::draw::Poly* ret = new cv::gapi::wip::draw::Poly();
			return ret;
	}

	// cv::gapi::wip::draw::Poly::implicitClone() generated
	// ("cv::gapi::wip::draw::Poly::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::gapi::wip::draw::Poly* cv_gapi_wip_draw_Poly_implicitClone_const(const cv::gapi::wip::draw::Poly* instance) {
			return new cv::gapi::wip::draw::Poly(*instance);
	}

	// cv::gapi::wip::draw::Poly::points() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:332
	// ("cv::gapi::wip::draw::Poly::points", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point>* cv_gapi_wip_draw_Poly_propPoints_const(const cv::gapi::wip::draw::Poly* instance) {
			std::vector<cv::Point> ret = instance->points;
			return new std::vector<cv::Point>(ret);
	}

	// cv::gapi::wip::draw::Poly::setPoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:332
	// ("cv::gapi::wip::draw::Poly::setPoints", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
	void cv_gapi_wip_draw_Poly_propPoints_const_vectorLPointG(cv::gapi::wip::draw::Poly* instance, const std::vector<cv::Point>* val) {
			instance->points = *val;
	}

	// cv::gapi::wip::draw::Poly::color() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:333
	// ("cv::gapi::wip::draw::Poly::color", vec![(pred!(const, [], []), _)]),
	void cv_gapi_wip_draw_Poly_propColor_const(const cv::gapi::wip::draw::Poly* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->color;
			*ocvrs_return = ret;
	}

	// cv::gapi::wip::draw::Poly::setColor(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:333
	// ("cv::gapi::wip::draw::Poly::setColor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_gapi_wip_draw_Poly_propColor_const_Scalar(cv::gapi::wip::draw::Poly* instance, const cv::Scalar* val) {
			instance->color = *val;
	}

	// cv::gapi::wip::draw::Poly::thick() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:334
	// ("cv::gapi::wip::draw::Poly::thick", vec![(pred!(const, [], []), _)]),
	int cv_gapi_wip_draw_Poly_propThick_const(const cv::gapi::wip::draw::Poly* instance) {
			int ret = instance->thick;
			return ret;
	}

	// cv::gapi::wip::draw::Poly::setThick(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:334
	// ("cv::gapi::wip::draw::Poly::setThick", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_gapi_wip_draw_Poly_propThick_const_int(cv::gapi::wip::draw::Poly* instance, const int val) {
			instance->thick = val;
	}

	// cv::gapi::wip::draw::Poly::lt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:335
	// ("cv::gapi::wip::draw::Poly::lt", vec![(pred!(const, [], []), _)]),
	int cv_gapi_wip_draw_Poly_propLt_const(const cv::gapi::wip::draw::Poly* instance) {
			int ret = instance->lt;
			return ret;
	}

	// cv::gapi::wip::draw::Poly::setLt(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:335
	// ("cv::gapi::wip::draw::Poly::setLt", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_gapi_wip_draw_Poly_propLt_const_int(cv::gapi::wip::draw::Poly* instance, const int val) {
			instance->lt = val;
	}

	// cv::gapi::wip::draw::Poly::shift() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:336
	// ("cv::gapi::wip::draw::Poly::shift", vec![(pred!(const, [], []), _)]),
	int cv_gapi_wip_draw_Poly_propShift_const(const cv::gapi::wip::draw::Poly* instance) {
			int ret = instance->shift;
			return ret;
	}

	// cv::gapi::wip::draw::Poly::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:336
	// ("cv::gapi::wip::draw::Poly::setShift", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_gapi_wip_draw_Poly_propShift_const_int(cv::gapi::wip::draw::Poly* instance, const int val) {
			instance->shift = val;
	}

	// cv::gapi::wip::draw::Poly::delete() generated
	// ("cv::gapi::wip::draw::Poly::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Poly_delete(cv::gapi::wip::draw::Poly* instance) {
			delete instance;
	}

	// Rect(const cv::Rect &, const cv::Scalar &, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:138
	// ("cv::gapi::wip::draw::Rect::Rect", vec![(pred!(mut, ["rect_", "color_", "thick_", "lt_", "shift_"], ["const cv::Rect*", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_gapi_wip_draw_Rect_Rect_const_RectR_const_ScalarR_int_int_int(const cv::Rect* rect_, const cv::Scalar* color_, int thick_, int lt_, int shift_, Result<cv::gapi::wip::draw::Rect>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Rect ret(*rect_, *color_, thick_, lt_, shift_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::wip::draw::Rect::Rect(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:138
	// ("cv::gapi::wip::draw::Rect::Rect", vec![(pred!(mut, ["rect_", "color_"], ["const cv::Rect*", "const cv::Scalar*"]), _)]),
	void cv_gapi_wip_draw_Rect_Rect_const_RectR_const_ScalarR(const cv::Rect* rect_, const cv::Scalar* color_, Result<cv::gapi::wip::draw::Rect>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Rect ret(*rect_, *color_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Rect()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:148
	// ("cv::gapi::wip::draw::Rect::Rect", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Rect_Rect(cv::gapi::wip::draw::Rect* ocvrs_return) {
			cv::gapi::wip::draw::Rect ret;
			*ocvrs_return = ret;
	}

	// Text(const std::string &, const cv::Point &, int, double, const cv::Scalar &, int, int, bool)(InString, SimpleClass, Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:59
	// ("cv::gapi::wip::draw::Text::Text", vec![(pred!(mut, ["text_", "org_", "ff_", "fs_", "color_", "thick_", "lt_", "bottom_left_origin_"], ["const std::string*", "const cv::Point*", "int", "double", "const cv::Scalar*", "int", "int", "bool"]), _)]),
	void cv_gapi_wip_draw_Text_Text_const_stringR_const_PointR_int_double_const_ScalarR_int_int_bool(const char* text_, const cv::Point* org_, int ff_, double fs_, const cv::Scalar* color_, int thick_, int lt_, bool bottom_left_origin_, Result<cv::gapi::wip::draw::Text*>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Text* ret = new cv::gapi::wip::draw::Text(std::string(text_), *org_, ff_, fs_, *color_, thick_, lt_, bottom_left_origin_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gapi::wip::draw::Text::Text(InString, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:59
	// ("cv::gapi::wip::draw::Text::Text", vec![(pred!(mut, ["text_", "org_", "ff_", "fs_", "color_"], ["const std::string*", "const cv::Point*", "int", "double", "const cv::Scalar*"]), _)]),
	void cv_gapi_wip_draw_Text_Text_const_stringR_const_PointR_int_double_const_ScalarR(const char* text_, const cv::Point* org_, int ff_, double fs_, const cv::Scalar* color_, Result<cv::gapi::wip::draw::Text*>* ocvrs_return) {
		try {
			cv::gapi::wip::draw::Text* ret = new cv::gapi::wip::draw::Text(std::string(text_), *org_, ff_, fs_, *color_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Text()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:73
	// ("cv::gapi::wip::draw::Text::Text", vec![(pred!(mut, [], []), _)]),
	cv::gapi::wip::draw::Text* cv_gapi_wip_draw_Text_Text() {
			cv::gapi::wip::draw::Text* ret = new cv::gapi::wip::draw::Text();
			return ret;
	}

	// cv::gapi::wip::draw::Text::implicitClone() generated
	// ("cv::gapi::wip::draw::Text::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::gapi::wip::draw::Text* cv_gapi_wip_draw_Text_implicitClone_const(const cv::gapi::wip::draw::Text* instance) {
			return new cv::gapi::wip::draw::Text(*instance);
	}

	// cv::gapi::wip::draw::Text::text() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:76
	// ("cv::gapi::wip::draw::Text::text", vec![(pred!(const, [], []), _)]),
	void* cv_gapi_wip_draw_Text_propText_const(const cv::gapi::wip::draw::Text* instance) {
			std::string ret = instance->text;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::gapi::wip::draw::Text::setText(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:76
	// ("cv::gapi::wip::draw::Text::setText", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_gapi_wip_draw_Text_propText_const_string(cv::gapi::wip::draw::Text* instance, const char* val) {
			instance->text = std::string(val);
	}

	// cv::gapi::wip::draw::Text::org() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:77
	// ("cv::gapi::wip::draw::Text::org", vec![(pred!(const, [], []), _)]),
	void cv_gapi_wip_draw_Text_propOrg_const(const cv::gapi::wip::draw::Text* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->org;
			*ocvrs_return = ret;
	}

	// cv::gapi::wip::draw::Text::setOrg(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:77
	// ("cv::gapi::wip::draw::Text::setOrg", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
	void cv_gapi_wip_draw_Text_propOrg_const_Point(cv::gapi::wip::draw::Text* instance, const cv::Point* val) {
			instance->org = *val;
	}

	// cv::gapi::wip::draw::Text::ff() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:78
	// ("cv::gapi::wip::draw::Text::ff", vec![(pred!(const, [], []), _)]),
	int cv_gapi_wip_draw_Text_propFf_const(const cv::gapi::wip::draw::Text* instance) {
			int ret = instance->ff;
			return ret;
	}

	// cv::gapi::wip::draw::Text::setFf(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:78
	// ("cv::gapi::wip::draw::Text::setFf", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_gapi_wip_draw_Text_propFf_const_int(cv::gapi::wip::draw::Text* instance, const int val) {
			instance->ff = val;
	}

	// cv::gapi::wip::draw::Text::fs() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:79
	// ("cv::gapi::wip::draw::Text::fs", vec![(pred!(const, [], []), _)]),
	double cv_gapi_wip_draw_Text_propFs_const(const cv::gapi::wip::draw::Text* instance) {
			double ret = instance->fs;
			return ret;
	}

	// cv::gapi::wip::draw::Text::setFs(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:79
	// ("cv::gapi::wip::draw::Text::setFs", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_gapi_wip_draw_Text_propFs_const_double(cv::gapi::wip::draw::Text* instance, const double val) {
			instance->fs = val;
	}

	// cv::gapi::wip::draw::Text::color() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:80
	// ("cv::gapi::wip::draw::Text::color", vec![(pred!(const, [], []), _)]),
	void cv_gapi_wip_draw_Text_propColor_const(const cv::gapi::wip::draw::Text* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->color;
			*ocvrs_return = ret;
	}

	// cv::gapi::wip::draw::Text::setColor(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:80
	// ("cv::gapi::wip::draw::Text::setColor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_gapi_wip_draw_Text_propColor_const_Scalar(cv::gapi::wip::draw::Text* instance, const cv::Scalar* val) {
			instance->color = *val;
	}

	// cv::gapi::wip::draw::Text::thick() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:81
	// ("cv::gapi::wip::draw::Text::thick", vec![(pred!(const, [], []), _)]),
	int cv_gapi_wip_draw_Text_propThick_const(const cv::gapi::wip::draw::Text* instance) {
			int ret = instance->thick;
			return ret;
	}

	// cv::gapi::wip::draw::Text::setThick(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:81
	// ("cv::gapi::wip::draw::Text::setThick", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_gapi_wip_draw_Text_propThick_const_int(cv::gapi::wip::draw::Text* instance, const int val) {
			instance->thick = val;
	}

	// cv::gapi::wip::draw::Text::lt() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:82
	// ("cv::gapi::wip::draw::Text::lt", vec![(pred!(const, [], []), _)]),
	int cv_gapi_wip_draw_Text_propLt_const(const cv::gapi::wip::draw::Text* instance) {
			int ret = instance->lt;
			return ret;
	}

	// cv::gapi::wip::draw::Text::setLt(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:82
	// ("cv::gapi::wip::draw::Text::setLt", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_gapi_wip_draw_Text_propLt_const_int(cv::gapi::wip::draw::Text* instance, const int val) {
			instance->lt = val;
	}

	// cv::gapi::wip::draw::Text::bottom_left_origin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:83
	// ("cv::gapi::wip::draw::Text::bottom_left_origin", vec![(pred!(const, [], []), _)]),
	bool cv_gapi_wip_draw_Text_propBottom_left_origin_const(const cv::gapi::wip::draw::Text* instance) {
			bool ret = instance->bottom_left_origin;
			return ret;
	}

	// cv::gapi::wip::draw::Text::setBottom_left_origin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/render/render_types.hpp:83
	// ("cv::gapi::wip::draw::Text::setBottom_left_origin", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_gapi_wip_draw_Text_propBottom_left_origin_const_bool(cv::gapi::wip::draw::Text* instance, const bool val) {
			instance->bottom_left_origin = val;
	}

	// cv::gapi::wip::draw::Text::delete() generated
	// ("cv::gapi::wip::draw::Text::delete", vec![(pred!(mut, [], []), _)]),
	void cv_gapi_wip_draw_Text_delete(cv::gapi::wip::draw::Text* instance) {
			delete instance;
	}

	// use_threaded_executor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:275
	// ("cv::use_threaded_executor::use_threaded_executor", vec![(pred!(mut, [], []), _)]),
	void cv_use_threaded_executor_use_threaded_executor(Result<cv::use_threaded_executor*>* ocvrs_return) {
		try {
			cv::use_threaded_executor* ret = new cv::use_threaded_executor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// use_threaded_executor(const uint32_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:276
	// ("cv::use_threaded_executor::use_threaded_executor", vec![(pred!(mut, ["nthreads"], ["const uint32_t"]), _)]),
	void cv_use_threaded_executor_use_threaded_executor_const_uint32_t(const uint32_t nthreads, Result<cv::use_threaded_executor*>* ocvrs_return) {
		try {
			cv::use_threaded_executor* ret = new cv::use_threaded_executor(nthreads);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::use_threaded_executor::num_threads() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:278
	// ("cv::use_threaded_executor::num_threads", vec![(pred!(const, [], []), _)]),
	uint32_t cv_use_threaded_executor_propNum_threads_const(const cv::use_threaded_executor* instance) {
			uint32_t ret = instance->num_threads;
			return ret;
	}

	// cv::use_threaded_executor::setNum_threads(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/gcommon.hpp:278
	// ("cv::use_threaded_executor::setNum_threads", vec![(pred!(mut, ["val"], ["const uint32_t"]), _)]),
	void cv_use_threaded_executor_propNum_threads_const_uint32_t(cv::use_threaded_executor* instance, const uint32_t val) {
			instance->num_threads = val;
	}

	// cv::use_threaded_executor::delete() generated
	// ("cv::use_threaded_executor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_use_threaded_executor_delete(cv::use_threaded_executor* instance) {
			delete instance;
	}

	// any(const any &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:82
	// ("cv::util::any::any", vec![(pred!(mut, ["src"], ["const cv::util::any*"]), _)]),
	void cv_util_any_any_const_anyR(const cv::util::any* src, Result<cv::util::any*>* ocvrs_return) {
		try {
			cv::util::any* ret = new cv::util::any(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// any(any &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:84
	// ("cv::util::any::any", vec![(pred!(mut, ["src"], ["cv::util::any*"]), _)]),
	void cv_util_any_any_anyR(cv::util::any* src, Result<cv::util::any*>* ocvrs_return) {
		try {
			cv::util::any* ret = new cv::util::any(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// any()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:86
	// ("cv::util::any::any", vec![(pred!(mut, [], []), _)]),
	cv::util::any* cv_util_any_any() {
			cv::util::any* ret = new cv::util::any();
			return ret;
	}

	// any(any &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:87
	// ("cv::util::any::any", vec![(pred!(mut, ["unnamed"], ["cv::util::any*"]), _)]),
	cv::util::any* cv_util_any_any_anyRR(cv::util::any* unnamed) {
			cv::util::any* ret = new cv::util::any(std::move(*unnamed));
			return ret;
	}

	// operator=(any &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:89
	// ("cv::util::any::operator=", vec![(pred!(mut, ["unnamed"], ["cv::util::any*"]), _)]),
	void cv_util_any_operatorST_anyRR(cv::util::any* instance, cv::util::any* unnamed) {
			instance->operator=(std::move(*unnamed));
	}

	// operator=(const any &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/gapi/util/any.hpp:91
	// ("cv::util::any::operator=", vec![(pred!(mut, ["src"], ["const cv::util::any*"]), _)]),
	void cv_util_any_operatorST_const_anyR(cv::util::any* instance, const cv::util::any* src, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::util::any::delete() generated
	// ("cv::util::any::delete", vec![(pred!(mut, [], []), _)]),
	void cv_util_any_delete(cv::util::any* instance) {
			delete instance;
	}

}
