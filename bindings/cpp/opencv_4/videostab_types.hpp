template struct Result<bool>;
template struct Result<const std::vector<cv::Mat>*>;
template struct Result<const std::vector<float>*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::Feature2D>*>;
template struct Result<cv::Ptr<cv::videostab::DeblurerBase>*>;
template struct Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>;
template struct Result<cv::Ptr<cv::videostab::IFrameSource>*>;
template struct Result<cv::Ptr<cv::videostab::ILog>*>;
template struct Result<cv::Ptr<cv::videostab::IMotionStabilizer>*>;
template struct Result<cv::Ptr<cv::videostab::IOutlierRejector>*>;
template struct Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>;
template struct Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>;
template struct Result<cv::Ptr<cv::videostab::InpainterBase>*>;
template struct Result<cv::Ptr<cv::videostab::MotionFilterBase>*>;
template struct Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::videostab::ColorInpainter*>;
template struct Result<cv::videostab::ConsistentMosaicInpainter*>;
template struct Result<cv::videostab::DensePyrLkOptFlowEstimatorGpu*>;
template struct Result<cv::videostab::FastMarchingMethod*>;
template struct Result<cv::videostab::FromFileMotionReader*>;
template struct Result<cv::videostab::GaussianMotionFilter*>;
template struct Result<cv::videostab::KeypointBasedMotionEstimatorGpu*>;
template struct Result<cv::videostab::KeypointBasedMotionEstimator*>;
template struct Result<cv::videostab::LpMotionStabilizer*>;
template struct Result<cv::videostab::MaskFrameSource*>;
template struct Result<cv::videostab::MotionEstimatorL1*>;
template struct Result<cv::videostab::MotionEstimatorRansacL2*>;
template struct Result<cv::videostab::MotionInpainter*>;
template struct Result<cv::videostab::MotionModel>;
template struct Result<cv::videostab::OnePassStabilizer*>;
template struct Result<cv::videostab::PyrLkOptFlowEstimatorBase*>;
template struct Result<cv::videostab::RansacParams*>;
template struct Result<cv::videostab::SparsePyrLkOptFlowEstimatorGpu*>;
template struct Result<cv::videostab::ToFileMotionWriter*>;
template struct Result<cv::videostab::TranslationBasedLocalOutlierRejector*>;
template struct Result<cv::videostab::TwoPassStabilizer*>;
template struct Result<cv::videostab::VideoFileSource*>;
template struct Result<cv::videostab::WeightingDeblurer*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
extern "C" {
	cv::Ptr<cv::videostab::ColorAverageInpainter>* cv_PtrOfColorAverageInpainter_new(cv::videostab::ColorAverageInpainter* val) {
		return new cv::Ptr<cv::videostab::ColorAverageInpainter>(val);
	}
	
	void cv_PtrOfColorAverageInpainter_delete(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::ColorAverageInpainter* cv_PtrOfColorAverageInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::ColorAverageInpainter* cv_PtrOfColorAverageInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfColorAverageInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::ColorInpainter>* cv_PtrOfColorInpainter_new(cv::videostab::ColorInpainter* val) {
		return new cv::Ptr<cv::videostab::ColorInpainter>(val);
	}
	
	void cv_PtrOfColorInpainter_delete(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::ColorInpainter* cv_PtrOfColorInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::ColorInpainter* cv_PtrOfColorInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfColorInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* cv_PtrOfConsistentMosaicInpainter_new(cv::videostab::ConsistentMosaicInpainter* val) {
		return new cv::Ptr<cv::videostab::ConsistentMosaicInpainter>(val);
	}
	
	void cv_PtrOfConsistentMosaicInpainter_delete(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::ConsistentMosaicInpainter* cv_PtrOfConsistentMosaicInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::ConsistentMosaicInpainter* cv_PtrOfConsistentMosaicInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfConsistentMosaicInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

extern "C" {
	void cv_PtrOfDeblurerBase_delete(cv::Ptr<cv::videostab::DeblurerBase>* instance) {
		delete instance;
	}

	const cv::videostab::DeblurerBase* cv_PtrOfDeblurerBase_get_inner_ptr(const cv::Ptr<cv::videostab::DeblurerBase>* instance) {
		return instance->get();
	}

	cv::videostab::DeblurerBase* cv_PtrOfDeblurerBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::DeblurerBase>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* cv_PtrOfDensePyrLkOptFlowEstimatorGpu_new(cv::videostab::DensePyrLkOptFlowEstimatorGpu* val) {
		return new cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>(val);
	}
	
	void cv_PtrOfDensePyrLkOptFlowEstimatorGpu_delete(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
		delete instance;
	}

	const cv::videostab::DensePyrLkOptFlowEstimatorGpu* cv_PtrOfDensePyrLkOptFlowEstimatorGpu_get_inner_ptr(const cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
		return instance->get();
	}

	cv::videostab::DensePyrLkOptFlowEstimatorGpu* cv_PtrOfDensePyrLkOptFlowEstimatorGpu_get_inner_ptr_mut(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* cv_PtrOfDensePyrLkOptFlowEstimatorGpu_to_PtrOfIDenseOptFlowEstimator(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
		return new cv::Ptr<cv::videostab::IDenseOptFlowEstimator>(instance->dynamicCast<cv::videostab::IDenseOptFlowEstimator>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::FromFileMotionReader>* cv_PtrOfFromFileMotionReader_new(cv::videostab::FromFileMotionReader* val) {
		return new cv::Ptr<cv::videostab::FromFileMotionReader>(val);
	}
	
	void cv_PtrOfFromFileMotionReader_delete(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		delete instance;
	}

	const cv::videostab::FromFileMotionReader* cv_PtrOfFromFileMotionReader_get_inner_ptr(const cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		return instance->get();
	}

	cv::videostab::FromFileMotionReader* cv_PtrOfFromFileMotionReader_get_inner_ptr_mut(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfFromFileMotionReader_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::GaussianMotionFilter>* cv_PtrOfGaussianMotionFilter_new(cv::videostab::GaussianMotionFilter* val) {
		return new cv::Ptr<cv::videostab::GaussianMotionFilter>(val);
	}
	
	void cv_PtrOfGaussianMotionFilter_delete(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		delete instance;
	}

	const cv::videostab::GaussianMotionFilter* cv_PtrOfGaussianMotionFilter_get_inner_ptr(const cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return instance->get();
	}

	cv::videostab::GaussianMotionFilter* cv_PtrOfGaussianMotionFilter_get_inner_ptr_mut(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfGaussianMotionFilter_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
	
	cv::Ptr<cv::videostab::MotionFilterBase>* cv_PtrOfGaussianMotionFilter_to_PtrOfMotionFilterBase(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return new cv::Ptr<cv::videostab::MotionFilterBase>(instance->dynamicCast<cv::videostab::MotionFilterBase>());
	}
}

extern "C" {
	void cv_PtrOfIDenseOptFlowEstimator_delete(cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::IDenseOptFlowEstimator* cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::IDenseOptFlowEstimator* cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfIFrameSource_delete(cv::Ptr<cv::videostab::IFrameSource>* instance) {
		delete instance;
	}

	const cv::videostab::IFrameSource* cv_PtrOfIFrameSource_get_inner_ptr(const cv::Ptr<cv::videostab::IFrameSource>* instance) {
		return instance->get();
	}

	cv::videostab::IFrameSource* cv_PtrOfIFrameSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::IFrameSource>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfILog_delete(cv::Ptr<cv::videostab::ILog>* instance) {
		delete instance;
	}

	const cv::videostab::ILog* cv_PtrOfILog_get_inner_ptr(const cv::Ptr<cv::videostab::ILog>* instance) {
		return instance->get();
	}

	cv::videostab::ILog* cv_PtrOfILog_get_inner_ptr_mut(cv::Ptr<cv::videostab::ILog>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfIMotionStabilizer_delete(cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::IMotionStabilizer* cv_PtrOfIMotionStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::IMotionStabilizer* cv_PtrOfIMotionStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfIOutlierRejector_delete(cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
		delete instance;
	}

	const cv::videostab::IOutlierRejector* cv_PtrOfIOutlierRejector_get_inner_ptr(const cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
		return instance->get();
	}

	cv::videostab::IOutlierRejector* cv_PtrOfIOutlierRejector_get_inner_ptr_mut(cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfISparseOptFlowEstimator_delete(cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::ISparseOptFlowEstimator* cv_PtrOfISparseOptFlowEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::ISparseOptFlowEstimator* cv_PtrOfISparseOptFlowEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfImageMotionEstimatorBase_delete(cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
		delete instance;
	}

	const cv::videostab::ImageMotionEstimatorBase* cv_PtrOfImageMotionEstimatorBase_get_inner_ptr(const cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
		return instance->get();
	}

	cv::videostab::ImageMotionEstimatorBase* cv_PtrOfImageMotionEstimatorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfInpainterBase_delete(cv::Ptr<cv::videostab::InpainterBase>* instance) {
		delete instance;
	}

	const cv::videostab::InpainterBase* cv_PtrOfInpainterBase_get_inner_ptr(const cv::Ptr<cv::videostab::InpainterBase>* instance) {
		return instance->get();
	}

	cv::videostab::InpainterBase* cv_PtrOfInpainterBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::InpainterBase>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::videostab::InpaintingPipeline>* cv_PtrOfInpaintingPipeline_new(cv::videostab::InpaintingPipeline* val) {
		return new cv::Ptr<cv::videostab::InpaintingPipeline>(val);
	}
	
	void cv_PtrOfInpaintingPipeline_delete(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		delete instance;
	}

	const cv::videostab::InpaintingPipeline* cv_PtrOfInpaintingPipeline_get_inner_ptr(const cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		return instance->get();
	}

	cv::videostab::InpaintingPipeline* cv_PtrOfInpaintingPipeline_get_inner_ptr_mut(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfInpaintingPipeline_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* cv_PtrOfKeypointBasedMotionEstimator_new(cv::videostab::KeypointBasedMotionEstimator* val) {
		return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>(val);
	}
	
	void cv_PtrOfKeypointBasedMotionEstimator_delete(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::KeypointBasedMotionEstimator* cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::KeypointBasedMotionEstimator* cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfKeypointBasedMotionEstimator_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* cv_PtrOfKeypointBasedMotionEstimatorGpu_new(cv::videostab::KeypointBasedMotionEstimatorGpu* val) {
		return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>(val);
	}
	
	void cv_PtrOfKeypointBasedMotionEstimatorGpu_delete(cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
		delete instance;
	}

	const cv::videostab::KeypointBasedMotionEstimatorGpu* cv_PtrOfKeypointBasedMotionEstimatorGpu_get_inner_ptr(const cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
		return instance->get();
	}

	cv::videostab::KeypointBasedMotionEstimatorGpu* cv_PtrOfKeypointBasedMotionEstimatorGpu_get_inner_ptr_mut(cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfKeypointBasedMotionEstimatorGpu_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::LogToStdout>* cv_PtrOfLogToStdout_new(cv::videostab::LogToStdout* val) {
		return new cv::Ptr<cv::videostab::LogToStdout>(val);
	}
	
	void cv_PtrOfLogToStdout_delete(cv::Ptr<cv::videostab::LogToStdout>* instance) {
		delete instance;
	}

	const cv::videostab::LogToStdout* cv_PtrOfLogToStdout_get_inner_ptr(const cv::Ptr<cv::videostab::LogToStdout>* instance) {
		return instance->get();
	}

	cv::videostab::LogToStdout* cv_PtrOfLogToStdout_get_inner_ptr_mut(cv::Ptr<cv::videostab::LogToStdout>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ILog>* cv_PtrOfLogToStdout_to_PtrOfILog(cv::Ptr<cv::videostab::LogToStdout>* instance) {
		return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::LpMotionStabilizer>* cv_PtrOfLpMotionStabilizer_new(cv::videostab::LpMotionStabilizer* val) {
		return new cv::Ptr<cv::videostab::LpMotionStabilizer>(val);
	}
	
	void cv_PtrOfLpMotionStabilizer_delete(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::LpMotionStabilizer* cv_PtrOfLpMotionStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::LpMotionStabilizer* cv_PtrOfLpMotionStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfLpMotionStabilizer_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::MaskFrameSource>* cv_PtrOfMaskFrameSource_new(cv::videostab::MaskFrameSource* val) {
		return new cv::Ptr<cv::videostab::MaskFrameSource>(val);
	}
	
	void cv_PtrOfMaskFrameSource_delete(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		delete instance;
	}

	const cv::videostab::MaskFrameSource* cv_PtrOfMaskFrameSource_get_inner_ptr(const cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		return instance->get();
	}

	cv::videostab::MaskFrameSource* cv_PtrOfMaskFrameSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfMaskFrameSource_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

extern "C" {
	void cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_delete(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		delete instance;
	}

	const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr(const cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		return instance->get();
	}

	cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
}

extern "C" {
	void cv_PtrOfMotionEstimatorBase_delete(cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
		delete instance;
	}

	const cv::videostab::MotionEstimatorBase* cv_PtrOfMotionEstimatorBase_get_inner_ptr(const cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
		return instance->get();
	}

	cv::videostab::MotionEstimatorBase* cv_PtrOfMotionEstimatorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::videostab::MotionEstimatorL1>* cv_PtrOfMotionEstimatorL1_new(cv::videostab::MotionEstimatorL1* val) {
		return new cv::Ptr<cv::videostab::MotionEstimatorL1>(val);
	}
	
	void cv_PtrOfMotionEstimatorL1_delete(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		delete instance;
	}

	const cv::videostab::MotionEstimatorL1* cv_PtrOfMotionEstimatorL1_get_inner_ptr(const cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		return instance->get();
	}

	cv::videostab::MotionEstimatorL1* cv_PtrOfMotionEstimatorL1_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrOfMotionEstimatorL1_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* cv_PtrOfMotionEstimatorRansacL2_new(cv::videostab::MotionEstimatorRansacL2* val) {
		return new cv::Ptr<cv::videostab::MotionEstimatorRansacL2>(val);
	}
	
	void cv_PtrOfMotionEstimatorRansacL2_delete(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		delete instance;
	}

	const cv::videostab::MotionEstimatorRansacL2* cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr(const cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		return instance->get();
	}

	cv::videostab::MotionEstimatorRansacL2* cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrOfMotionEstimatorRansacL2_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}
}

extern "C" {
	void cv_PtrOfMotionFilterBase_delete(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		delete instance;
	}

	const cv::videostab::MotionFilterBase* cv_PtrOfMotionFilterBase_get_inner_ptr(const cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		return instance->get();
	}

	cv::videostab::MotionFilterBase* cv_PtrOfMotionFilterBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfMotionFilterBase_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::MotionInpainter>* cv_PtrOfMotionInpainter_new(cv::videostab::MotionInpainter* val) {
		return new cv::Ptr<cv::videostab::MotionInpainter>(val);
	}
	
	void cv_PtrOfMotionInpainter_delete(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::MotionInpainter* cv_PtrOfMotionInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::MotionInpainter* cv_PtrOfMotionInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfMotionInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::MotionStabilizationPipeline>* cv_PtrOfMotionStabilizationPipeline_new(cv::videostab::MotionStabilizationPipeline* val) {
		return new cv::Ptr<cv::videostab::MotionStabilizationPipeline>(val);
	}
	
	void cv_PtrOfMotionStabilizationPipeline_delete(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		delete instance;
	}

	const cv::videostab::MotionStabilizationPipeline* cv_PtrOfMotionStabilizationPipeline_get_inner_ptr(const cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		return instance->get();
	}

	cv::videostab::MotionStabilizationPipeline* cv_PtrOfMotionStabilizationPipeline_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfMotionStabilizationPipeline_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::NullDeblurer>* cv_PtrOfNullDeblurer_new(cv::videostab::NullDeblurer* val) {
		return new cv::Ptr<cv::videostab::NullDeblurer>(val);
	}
	
	void cv_PtrOfNullDeblurer_delete(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		delete instance;
	}

	const cv::videostab::NullDeblurer* cv_PtrOfNullDeblurer_get_inner_ptr(const cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		return instance->get();
	}

	cv::videostab::NullDeblurer* cv_PtrOfNullDeblurer_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrOfNullDeblurer_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::NullFrameSource>* cv_PtrOfNullFrameSource_new(cv::videostab::NullFrameSource* val) {
		return new cv::Ptr<cv::videostab::NullFrameSource>(val);
	}
	
	void cv_PtrOfNullFrameSource_delete(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		delete instance;
	}

	const cv::videostab::NullFrameSource* cv_PtrOfNullFrameSource_get_inner_ptr(const cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		return instance->get();
	}

	cv::videostab::NullFrameSource* cv_PtrOfNullFrameSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfNullFrameSource_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::NullInpainter>* cv_PtrOfNullInpainter_new(cv::videostab::NullInpainter* val) {
		return new cv::Ptr<cv::videostab::NullInpainter>(val);
	}
	
	void cv_PtrOfNullInpainter_delete(cv::Ptr<cv::videostab::NullInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::NullInpainter* cv_PtrOfNullInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::NullInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::NullInpainter* cv_PtrOfNullInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfNullInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::NullInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::NullLog>* cv_PtrOfNullLog_new(cv::videostab::NullLog* val) {
		return new cv::Ptr<cv::videostab::NullLog>(val);
	}
	
	void cv_PtrOfNullLog_delete(cv::Ptr<cv::videostab::NullLog>* instance) {
		delete instance;
	}

	const cv::videostab::NullLog* cv_PtrOfNullLog_get_inner_ptr(const cv::Ptr<cv::videostab::NullLog>* instance) {
		return instance->get();
	}

	cv::videostab::NullLog* cv_PtrOfNullLog_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullLog>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ILog>* cv_PtrOfNullLog_to_PtrOfILog(cv::Ptr<cv::videostab::NullLog>* instance) {
		return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::NullOutlierRejector>* cv_PtrOfNullOutlierRejector_new(cv::videostab::NullOutlierRejector* val) {
		return new cv::Ptr<cv::videostab::NullOutlierRejector>(val);
	}
	
	void cv_PtrOfNullOutlierRejector_delete(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		delete instance;
	}

	const cv::videostab::NullOutlierRejector* cv_PtrOfNullOutlierRejector_get_inner_ptr(const cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		return instance->get();
	}

	cv::videostab::NullOutlierRejector* cv_PtrOfNullOutlierRejector_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrOfNullOutlierRejector_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::NullWobbleSuppressor>* cv_PtrOfNullWobbleSuppressor_new(cv::videostab::NullWobbleSuppressor* val) {
		return new cv::Ptr<cv::videostab::NullWobbleSuppressor>(val);
	}
	
	void cv_PtrOfNullWobbleSuppressor_delete(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		delete instance;
	}

	const cv::videostab::NullWobbleSuppressor* cv_PtrOfNullWobbleSuppressor_get_inner_ptr(const cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		return instance->get();
	}

	cv::videostab::NullWobbleSuppressor* cv_PtrOfNullWobbleSuppressor_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrOfNullWobbleSuppressor_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::OnePassStabilizer>* cv_PtrOfOnePassStabilizer_new(cv::videostab::OnePassStabilizer* val) {
		return new cv::Ptr<cv::videostab::OnePassStabilizer>(val);
	}
	
	void cv_PtrOfOnePassStabilizer_delete(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::OnePassStabilizer* cv_PtrOfOnePassStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::OnePassStabilizer* cv_PtrOfOnePassStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfOnePassStabilizer_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* cv_PtrOfSparsePyrLkOptFlowEstimator_new(cv::videostab::SparsePyrLkOptFlowEstimator* val) {
		return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>(val);
	}
	
	void cv_PtrOfSparsePyrLkOptFlowEstimator_delete(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* cv_PtrOfSparsePyrLkOptFlowEstimator_to_PtrOfISparseOptFlowEstimator(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		return new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(instance->dynamicCast<cv::videostab::ISparseOptFlowEstimator>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_new(cv::videostab::SparsePyrLkOptFlowEstimatorGpu* val) {
		return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>(val);
	}
	
	void cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_delete(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
		delete instance;
	}

	const cv::videostab::SparsePyrLkOptFlowEstimatorGpu* cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_get_inner_ptr(const cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
		return instance->get();
	}

	cv::videostab::SparsePyrLkOptFlowEstimatorGpu* cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_get_inner_ptr_mut(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_to_PtrOfISparseOptFlowEstimator(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
		return new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(instance->dynamicCast<cv::videostab::ISparseOptFlowEstimator>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::ToFileMotionWriter>* cv_PtrOfToFileMotionWriter_new(cv::videostab::ToFileMotionWriter* val) {
		return new cv::Ptr<cv::videostab::ToFileMotionWriter>(val);
	}
	
	void cv_PtrOfToFileMotionWriter_delete(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		delete instance;
	}

	const cv::videostab::ToFileMotionWriter* cv_PtrOfToFileMotionWriter_get_inner_ptr(const cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		return instance->get();
	}

	cv::videostab::ToFileMotionWriter* cv_PtrOfToFileMotionWriter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfToFileMotionWriter_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* cv_PtrOfTranslationBasedLocalOutlierRejector_new(cv::videostab::TranslationBasedLocalOutlierRejector* val) {
		return new cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>(val);
	}
	
	void cv_PtrOfTranslationBasedLocalOutlierRejector_delete(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		delete instance;
	}

	const cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr(const cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		return instance->get();
	}

	cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr_mut(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrOfTranslationBasedLocalOutlierRejector_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::TwoPassStabilizer>* cv_PtrOfTwoPassStabilizer_new(cv::videostab::TwoPassStabilizer* val) {
		return new cv::Ptr<cv::videostab::TwoPassStabilizer>(val);
	}
	
	void cv_PtrOfTwoPassStabilizer_delete(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::TwoPassStabilizer* cv_PtrOfTwoPassStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::TwoPassStabilizer* cv_PtrOfTwoPassStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfTwoPassStabilizer_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::VideoFileSource>* cv_PtrOfVideoFileSource_new(cv::videostab::VideoFileSource* val) {
		return new cv::Ptr<cv::videostab::VideoFileSource>(val);
	}
	
	void cv_PtrOfVideoFileSource_delete(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		delete instance;
	}

	const cv::videostab::VideoFileSource* cv_PtrOfVideoFileSource_get_inner_ptr(const cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		return instance->get();
	}

	cv::videostab::VideoFileSource* cv_PtrOfVideoFileSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfVideoFileSource_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

extern "C" {
	cv::Ptr<cv::videostab::WeightingDeblurer>* cv_PtrOfWeightingDeblurer_new(cv::videostab::WeightingDeblurer* val) {
		return new cv::Ptr<cv::videostab::WeightingDeblurer>(val);
	}
	
	void cv_PtrOfWeightingDeblurer_delete(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		delete instance;
	}

	const cv::videostab::WeightingDeblurer* cv_PtrOfWeightingDeblurer_get_inner_ptr(const cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		return instance->get();
	}

	cv::videostab::WeightingDeblurer* cv_PtrOfWeightingDeblurer_get_inner_ptr_mut(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrOfWeightingDeblurer_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}
}

extern "C" {
	void cv_PtrOfWobbleSuppressorBase_delete(cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
		delete instance;
	}

	const cv::videostab::WobbleSuppressorBase* cv_PtrOfWobbleSuppressorBase_get_inner_ptr(const cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
		return instance->get();
	}

	cv::videostab::WobbleSuppressorBase* cv_PtrOfWobbleSuppressorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
		return instance->get();
	}
}

