extern "C" {
	// std::vector<cv::String>::new() generated
	// ("std::vector<cv::String>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::String>* std_vectorLcv_StringG_new_const() {
			std::vector<cv::String>* ret = new std::vector<cv::String>();
			return ret;
	}

	// std::vector<cv::String>::delete() generated
	// ("std::vector<cv::String>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_StringG_delete(std::vector<cv::String>* instance) {
			delete instance;
	}

	// std::vector<cv::String>::len() generated
	// ("std::vector<cv::String>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_StringG_len_const(const std::vector<cv::String>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::String>::isEmpty() generated
	// ("std::vector<cv::String>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_StringG_isEmpty_const(const std::vector<cv::String>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::String>::capacity() generated
	// ("std::vector<cv::String>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_StringG_capacity_const(const std::vector<cv::String>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::String>::shrinkToFit() generated
	// ("std::vector<cv::String>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_StringG_shrinkToFit(std::vector<cv::String>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::String>::reserve(Primitive) generated
	// ("std::vector<cv::String>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_StringG_reserve_size_t(std::vector<cv::String>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::String>::remove(Primitive) generated
	// ("std::vector<cv::String>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_StringG_remove_size_t(std::vector<cv::String>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::String>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::String>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_StringG_swap_size_t_size_t(std::vector<cv::String>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::String>::clear() generated
	// ("std::vector<cv::String>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_StringG_clear(std::vector<cv::String>* instance) {
			instance->clear();
	}

	// std::vector<cv::String>::push(InString) generated
	// ("std::vector<cv::String>::push", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void std_vectorLcv_StringG_push_const_String(std::vector<cv::String>* instance, const char* val) {
			instance->push_back(cv::String(val));
	}

	// std::vector<cv::String>::insert(Primitive, InString) generated
	// ("std::vector<cv::String>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::String"]), _)]),
	void std_vectorLcv_StringG_insert_size_t_const_String(std::vector<cv::String>* instance, size_t index, const char* val) {
			instance->insert(instance->begin() + index, cv::String(val));
	}

	// std::vector<cv::String>::get(Primitive) generated
	// ("std::vector<cv::String>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_StringG_get_const_size_t(const std::vector<cv::String>* instance, size_t index, void** ocvrs_return) {
			cv::String ret = (*instance)[index];
			*ocvrs_return = ocvrs_create_string(ret.c_str());
	}

	// std::vector<cv::String>::set(Primitive, InString) generated
	// ("std::vector<cv::String>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::String"]), _)]),
	void std_vectorLcv_StringG_set_size_t_const_String(std::vector<cv::String>* instance, size_t index, const char* val) {
			(*instance)[index] = cv::String(val);
	}

}


