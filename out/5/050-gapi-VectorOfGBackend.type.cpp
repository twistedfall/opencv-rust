extern "C" {
	// std::vector<cv::gapi::GBackend>::new() generated
	// ("std::vector<cv::gapi::GBackend>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::gapi::GBackend>* std_vectorLcv_gapi_GBackendG_new_const() {
			std::vector<cv::gapi::GBackend>* ret = new std::vector<cv::gapi::GBackend>();
			return ret;
	}

	// std::vector<cv::gapi::GBackend>::delete() generated
	// ("std::vector<cv::gapi::GBackend>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_gapi_GBackendG_delete(std::vector<cv::gapi::GBackend>* instance) {
			delete instance;
	}

	// std::vector<cv::gapi::GBackend>::len() generated
	// ("std::vector<cv::gapi::GBackend>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_gapi_GBackendG_len_const(const std::vector<cv::gapi::GBackend>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::gapi::GBackend>::isEmpty() generated
	// ("std::vector<cv::gapi::GBackend>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_gapi_GBackendG_isEmpty_const(const std::vector<cv::gapi::GBackend>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::gapi::GBackend>::capacity() generated
	// ("std::vector<cv::gapi::GBackend>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_gapi_GBackendG_capacity_const(const std::vector<cv::gapi::GBackend>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::gapi::GBackend>::shrinkToFit() generated
	// ("std::vector<cv::gapi::GBackend>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_gapi_GBackendG_shrinkToFit(std::vector<cv::gapi::GBackend>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::gapi::GBackend>::reserve(Primitive) generated
	// ("std::vector<cv::gapi::GBackend>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_gapi_GBackendG_reserve_size_t(std::vector<cv::gapi::GBackend>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::gapi::GBackend>::remove(Primitive) generated
	// ("std::vector<cv::gapi::GBackend>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_gapi_GBackendG_remove_size_t(std::vector<cv::gapi::GBackend>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::gapi::GBackend>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::gapi::GBackend>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_gapi_GBackendG_swap_size_t_size_t(std::vector<cv::gapi::GBackend>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::gapi::GBackend>::clear() generated
	// ("std::vector<cv::gapi::GBackend>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_gapi_GBackendG_clear(std::vector<cv::gapi::GBackend>* instance) {
			instance->clear();
	}

	// std::vector<cv::gapi::GBackend>::push(TraitClass) generated
	// ("std::vector<cv::gapi::GBackend>::push", vec![(pred!(mut, ["val"], ["const cv::gapi::GBackend"]), _)]),
	void std_vectorLcv_gapi_GBackendG_push_const_GBackend(std::vector<cv::gapi::GBackend>* instance, const cv::gapi::GBackend* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::gapi::GBackend>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::gapi::GBackend>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::gapi::GBackend"]), _)]),
	void std_vectorLcv_gapi_GBackendG_insert_size_t_const_GBackend(std::vector<cv::gapi::GBackend>* instance, size_t index, const cv::gapi::GBackend* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::gapi::GBackend>::get(Primitive) generated
	// ("std::vector<cv::gapi::GBackend>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_gapi_GBackendG_get_const_size_t(const std::vector<cv::gapi::GBackend>* instance, size_t index, cv::gapi::GBackend** ocvrs_return) {
			cv::gapi::GBackend ret = (*instance)[index];
			*ocvrs_return = new cv::gapi::GBackend(ret);
	}

	// std::vector<cv::gapi::GBackend>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::gapi::GBackend>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::gapi::GBackend"]), _)]),
	void std_vectorLcv_gapi_GBackendG_set_size_t_const_GBackend(std::vector<cv::gapi::GBackend>* instance, size_t index, const cv::gapi::GBackend* val) {
			(*instance)[index] = *val;
	}

}


