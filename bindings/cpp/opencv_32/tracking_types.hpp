template struct Result<bool>;
template struct Result<const cv::Ptr<cv::TrackerTargetState>*>;
template struct Result<const std::vector<cv::CvHaarEvaluator::FeatureHaar>*>;
template struct Result<const std::vector<cv::Mat>*>;
template struct Result<const std::vector<cv::Rect_<int>>*>;
template struct Result<const std::vector<float>*>;
template struct Result<const std::vector<int>*>;
template struct Result<cv::ClfMilBoost*>;
template struct Result<cv::ClfMilBoost::Params*>;
template struct Result<cv::CvFeatureParams*>;
template struct Result<cv::CvHaarEvaluator::FeatureHaar*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Mat_<double>*>;
template struct Result<cv::MultiTracker*>;
template struct Result<cv::MultiTracker_Alt*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Ptr<cv::CvFeatureParams>*>;
template struct Result<cv::Ptr<cv::TrackerBoosting>*>;
template struct Result<cv::Ptr<cv::TrackerFeature>*>;
template struct Result<cv::Ptr<cv::TrackerGOTURN>*>;
template struct Result<cv::Ptr<cv::TrackerKCF>*>;
template struct Result<cv::Ptr<cv::TrackerMIL>*>;
template struct Result<cv::Ptr<cv::TrackerMedianFlow>*>;
template struct Result<cv::Ptr<cv::TrackerModel>*>;
template struct Result<cv::Ptr<cv::TrackerSamplerAlgorithm>*>;
template struct Result<cv::Ptr<cv::TrackerStateEstimator>*>;
template struct Result<cv::Ptr<cv::TrackerTLD>*>;
template struct Result<cv::Ptr<cv::TrackerTargetState>*>;
template struct Result<cv::Ptr<cv::Tracker>*>;
template struct Result<cv::Rect_<double>>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::TrackerBoosting::Params*>;
template struct Result<cv::TrackerFeatureFeature2d*>;
template struct Result<cv::TrackerFeatureHAAR*>;
template struct Result<cv::TrackerFeatureHAAR::Params*>;
template struct Result<cv::TrackerFeatureHOG*>;
template struct Result<cv::TrackerFeatureLBP*>;
template struct Result<cv::TrackerFeatureSet*>;
template struct Result<cv::TrackerGOTURN::Params*>;
template struct Result<cv::TrackerKCF::Params*>;
template struct Result<cv::TrackerMIL::Params*>;
template struct Result<cv::TrackerMedianFlow::Params*>;
template struct Result<cv::TrackerSamplerCSC*>;
template struct Result<cv::TrackerSamplerCSC::Params*>;
template struct Result<cv::TrackerSamplerCS*>;
template struct Result<cv::TrackerSamplerCS::Params*>;
template struct Result<cv::TrackerSamplerPF*>;
template struct Result<cv::TrackerSamplerPF::Params*>;
template struct Result<cv::TrackerSampler*>;
template struct Result<cv::TrackerStateEstimatorAdaBoosting*>;
template struct Result<cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState*>;
template struct Result<cv::TrackerStateEstimatorMILBoosting*>;
template struct Result<cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState*>;
template struct Result<cv::TrackerStateEstimatorSVM*>;
template struct Result<cv::TrackerTLD::Params*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Ptr<cv::Tracker>>*>;
template struct Result<std::vector<cv::Rect_<double>>*>;
template struct Result<std::vector<cv::Scalar_<double>>*>;
template struct Result<std::vector<float>*>;
template struct Result<std::vector<int>*>;
template struct Result<unsigned int>;
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::CvFeatureParams>* cv_PtrOfCvFeatureParams_new(cv::CvFeatureParams* val) {
		return new cv::Ptr<cv::CvFeatureParams>(val);
	}
	
	void cv_PtrOfCvFeatureParams_delete(cv::Ptr<cv::CvFeatureParams>* instance) {
		delete instance;
	}

	cv::CvFeatureParams* cv_PtrOfCvFeatureParams_get_inner_ptr(cv::Ptr<cv::CvFeatureParams>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTracker_delete(cv::Ptr<cv::Tracker>* instance) {
		delete instance;
	}

	cv::Tracker* cv_PtrOfTracker_get_inner_ptr(cv::Ptr<cv::Tracker>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerBoosting_delete(cv::Ptr<cv::TrackerBoosting>* instance) {
		delete instance;
	}

	cv::TrackerBoosting* cv_PtrOfTrackerBoosting_get_inner_ptr(cv::Ptr<cv::TrackerBoosting>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerFeature_delete(cv::Ptr<cv::TrackerFeature>* instance) {
		delete instance;
	}

	cv::TrackerFeature* cv_PtrOfTrackerFeature_get_inner_ptr(cv::Ptr<cv::TrackerFeature>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerGOTURN_delete(cv::Ptr<cv::TrackerGOTURN>* instance) {
		delete instance;
	}

	cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr(cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerKCF_delete(cv::Ptr<cv::TrackerKCF>* instance) {
		delete instance;
	}

	cv::TrackerKCF* cv_PtrOfTrackerKCF_get_inner_ptr(cv::Ptr<cv::TrackerKCF>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerMIL_delete(cv::Ptr<cv::TrackerMIL>* instance) {
		delete instance;
	}

	cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr(cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerMedianFlow_delete(cv::Ptr<cv::TrackerMedianFlow>* instance) {
		delete instance;
	}

	cv::TrackerMedianFlow* cv_PtrOfTrackerMedianFlow_get_inner_ptr(cv::Ptr<cv::TrackerMedianFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerModel_delete(cv::Ptr<cv::TrackerModel>* instance) {
		delete instance;
	}

	cv::TrackerModel* cv_PtrOfTrackerModel_get_inner_ptr(cv::Ptr<cv::TrackerModel>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerSamplerAlgorithm_delete(cv::Ptr<cv::TrackerSamplerAlgorithm>* instance) {
		delete instance;
	}

	cv::TrackerSamplerAlgorithm* cv_PtrOfTrackerSamplerAlgorithm_get_inner_ptr(cv::Ptr<cv::TrackerSamplerAlgorithm>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerStateEstimator_delete(cv::Ptr<cv::TrackerStateEstimator>* instance) {
		delete instance;
	}

	cv::TrackerStateEstimator* cv_PtrOfTrackerStateEstimator_get_inner_ptr(cv::Ptr<cv::TrackerStateEstimator>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerTLD_delete(cv::Ptr<cv::TrackerTLD>* instance) {
		delete instance;
	}

	cv::TrackerTLD* cv_PtrOfTrackerTLD_get_inner_ptr(cv::Ptr<cv::TrackerTLD>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::TrackerTargetState>* cv_PtrOfTrackerTargetState_new(cv::TrackerTargetState* val) {
		return new cv::Ptr<cv::TrackerTargetState>(val);
	}
	
	void cv_PtrOfTrackerTargetState_delete(cv::Ptr<cv::TrackerTargetState>* instance) {
		delete instance;
	}

	cv::TrackerTargetState* cv_PtrOfTrackerTargetState_get_inner_ptr(cv::Ptr<cv::TrackerTargetState>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_VectorOfCvHaarEvaluator_FeatureHaar_delete(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance) {
		delete instance;
	}

	std::vector<cv::CvHaarEvaluator::FeatureHaar>* cv_VectorOfCvHaarEvaluator_FeatureHaar_new() {
		return new std::vector<cv::CvHaarEvaluator::FeatureHaar>();
	}

	size_t cv_VectorOfCvHaarEvaluator_FeatureHaar_len(const std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance) {
		return instance->size();
	}

	bool cv_VectorOfCvHaarEvaluator_FeatureHaar_is_empty(const std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfCvHaarEvaluator_FeatureHaar_capacity(const std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_shrink_to_fit(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_reserve(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_remove(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_swap(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_clear(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance) {
		instance->clear();
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_push(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, cv::CvHaarEvaluator::FeatureHaar* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_insert(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, size_t index, cv::CvHaarEvaluator::FeatureHaar* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::CvHaarEvaluator::FeatureHaar*> cv_VectorOfCvHaarEvaluator_FeatureHaar_get(const std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, size_t index) {
		return Ok<cv::CvHaarEvaluator::FeatureHaar*>(new cv::CvHaarEvaluator::FeatureHaar((*instance)[index]));
	}

	void cv_VectorOfCvHaarEvaluator_FeatureHaar_set(std::vector<cv::CvHaarEvaluator::FeatureHaar>* instance, size_t index, cv::CvHaarEvaluator::FeatureHaar* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfPtrOfTracker_delete(std::vector<cv::Ptr<cv::Tracker>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::Tracker>>* cv_VectorOfPtrOfTracker_new() {
		return new std::vector<cv::Ptr<cv::Tracker>>();
	}

	size_t cv_VectorOfPtrOfTracker_len(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfTracker_is_empty(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfTracker_capacity(const std::vector<cv::Ptr<cv::Tracker>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfTracker_shrink_to_fit(std::vector<cv::Ptr<cv::Tracker>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfTracker_reserve(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfTracker_remove(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfTracker_swap(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfTracker_clear(std::vector<cv::Ptr<cv::Tracker>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfTracker_push(std::vector<cv::Ptr<cv::Tracker>>* instance, cv::Ptr<cv::Tracker>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfTracker_insert(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, cv::Ptr<cv::Tracker>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::Tracker>*> cv_VectorOfPtrOfTracker_get(const std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::Tracker>*>(new cv::Ptr<cv::Tracker>((*instance)[index]));
	}

	void cv_VectorOfPtrOfTracker_set(std::vector<cv::Ptr<cv::Tracker>>* instance, size_t index, cv::Ptr<cv::Tracker>* val) {
		(*instance)[index] = *val;
	}

}


