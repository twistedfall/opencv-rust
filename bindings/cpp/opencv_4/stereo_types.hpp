template struct Result<cv::Mat*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*>;
template struct Result<cv::stereo::Match*>;
template struct Result<cv::stereo::PropagationParameters*>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::stereo::Match>*>;
extern "C" void cv_PtrOfQuasiDenseStereo_delete(cv::Ptr<cv::stereo::QuasiDenseStereo>* instance) {
	delete instance;
}

extern "C" cv::stereo::QuasiDenseStereo* cv_PtrOfQuasiDenseStereo_get_inner_ptr(cv::Ptr<cv::stereo::QuasiDenseStereo>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfMatch_delete(std::vector<cv::stereo::Match>* instance) {
		delete instance;
	}

	std::vector<cv::stereo::Match>* cv_VectorOfMatch_new() {
		return new std::vector<cv::stereo::Match>();
	}

	size_t cv_VectorOfMatch_len(const std::vector<cv::stereo::Match>* instance) {
		return instance->size();
	}

	bool cv_VectorOfMatch_is_empty(const std::vector<cv::stereo::Match>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfMatch_capacity(const std::vector<cv::stereo::Match>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfMatch_shrink_to_fit(std::vector<cv::stereo::Match>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfMatch_reserve(std::vector<cv::stereo::Match>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfMatch_remove(std::vector<cv::stereo::Match>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfMatch_swap(std::vector<cv::stereo::Match>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfMatch_clear(std::vector<cv::stereo::Match>* instance) {
		instance->clear();
	}

	void cv_VectorOfMatch_push(std::vector<cv::stereo::Match>* instance, cv::stereo::Match* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfMatch_insert(std::vector<cv::stereo::Match>* instance, size_t index, cv::stereo::Match* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::stereo::Match*> cv_VectorOfMatch_get(const std::vector<cv::stereo::Match>* instance, size_t index) {
		return Ok<cv::stereo::Match*>(new cv::stereo::Match((*instance)[index]));
	}

	void cv_VectorOfMatch_set(std::vector<cv::stereo::Match>* instance, size_t index, cv::stereo::Match* val) {
		(*instance)[index] = *val;
	}

}


