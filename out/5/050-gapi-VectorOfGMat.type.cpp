extern "C" {
	// std::vector<cv::GMat>::new() generated
	// ("std::vector<cv::GMat>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GMat>* std_vectorLcv_GMatG_new_const() {
			std::vector<cv::GMat>* ret = new std::vector<cv::GMat>();
			return ret;
	}

	// std::vector<cv::GMat>::delete() generated
	// ("std::vector<cv::GMat>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GMatG_delete(std::vector<cv::GMat>* instance) {
			delete instance;
	}

	// std::vector<cv::GMat>::len() generated
	// ("std::vector<cv::GMat>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GMatG_len_const(const std::vector<cv::GMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GMat>::isEmpty() generated
	// ("std::vector<cv::GMat>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GMatG_isEmpty_const(const std::vector<cv::GMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GMat>::capacity() generated
	// ("std::vector<cv::GMat>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GMatG_capacity_const(const std::vector<cv::GMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GMat>::shrinkToFit() generated
	// ("std::vector<cv::GMat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GMatG_shrinkToFit(std::vector<cv::GMat>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GMat>::reserve(Primitive) generated
	// ("std::vector<cv::GMat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GMatG_reserve_size_t(std::vector<cv::GMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GMat>::remove(Primitive) generated
	// ("std::vector<cv::GMat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GMatG_remove_size_t(std::vector<cv::GMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GMat>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GMat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GMatG_swap_size_t_size_t(std::vector<cv::GMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GMat>::clear() generated
	// ("std::vector<cv::GMat>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GMatG_clear(std::vector<cv::GMat>* instance) {
			instance->clear();
	}

	// std::vector<cv::GMat>::push(TraitClass) generated
	// ("std::vector<cv::GMat>::push", vec![(pred!(mut, ["val"], ["const cv::GMat"]), _)]),
	void std_vectorLcv_GMatG_push_const_GMat(std::vector<cv::GMat>* instance, const cv::GMat* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::GMat>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::GMat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GMat"]), _)]),
	void std_vectorLcv_GMatG_insert_size_t_const_GMat(std::vector<cv::GMat>* instance, size_t index, const cv::GMat* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::GMat>::get(Primitive) generated
	// ("std::vector<cv::GMat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GMatG_get_const_size_t(const std::vector<cv::GMat>* instance, size_t index, cv::GMat** ocvrs_return) {
			cv::GMat ret = (*instance)[index];
			*ocvrs_return = new cv::GMat(ret);
	}

	// std::vector<cv::GMat>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::GMat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GMat"]), _)]),
	void std_vectorLcv_GMatG_set_size_t_const_GMat(std::vector<cv::GMat>* instance, size_t index, const cv::GMat* val) {
			(*instance)[index] = *val;
	}

}


