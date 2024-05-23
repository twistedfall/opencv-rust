extern "C" {
	// std::vector<std::pair<cv::UMat, unsigned char>>::new() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<std::pair<cv::UMat, unsigned char>>* std_vectorLstd_pairLcv_UMat__unsigned_charGG_new_const() {
			std::vector<std::pair<cv::UMat, unsigned char>>* ret = new std::vector<std::pair<cv::UMat, unsigned char>>();
			return ret;
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::delete() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_delete(std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			delete instance;
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::len() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_UMat__unsigned_charGG_len_const(const std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::isEmpty() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLstd_pairLcv_UMat__unsigned_charGG_isEmpty_const(const std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::capacity() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLstd_pairLcv_UMat__unsigned_charGG_capacity_const(const std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::shrinkToFit() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_shrinkToFit(std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::reserve(Primitive) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_reserve_size_t(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::remove(Primitive) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_remove_size_t(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::swap(Primitive, Primitive) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_swap_size_t_size_t(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::clear() generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_clear(std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			instance->clear();
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::push(CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::push", vec![(pred!(mut, ["val"], ["const std::pair<cv::UMat, unsigned char>"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_push_const_pairLcv_UMat__unsigned_charG(std::vector<std::pair<cv::UMat, unsigned char>>* instance, const std::pair<cv::UMat, unsigned char>* val) {
			instance->push_back(*val);
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::UMat, unsigned char>"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_insert_size_t_const_pairLcv_UMat__unsigned_charG(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index, const std::pair<cv::UMat, unsigned char>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::get(Primitive) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_get_const_size_t(const std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index, std::pair<cv::UMat, unsigned char>** ocvrs_return) {
			std::pair<cv::UMat, unsigned char> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::UMat, unsigned char>(ret);
	}

	// std::vector<std::pair<cv::UMat, unsigned char>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<std::pair<cv::UMat, unsigned char>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::pair<cv::UMat, unsigned char>"]), _)]),
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_set_size_t_const_pairLcv_UMat__unsigned_charG(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index, const std::pair<cv::UMat, unsigned char>* val) {
			(*instance)[index] = *val;
	}

}


