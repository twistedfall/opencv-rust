extern "C" {
	// std::vector<cv::MatShape>::new() generated
	// ("std::vector<cv::MatShape>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::MatShape>* std_vectorLcv_MatShapeG_new_const() {
			std::vector<cv::MatShape>* ret = new std::vector<cv::MatShape>();
			return ret;
	}

	// std::vector<cv::MatShape>::delete() generated
	// ("std::vector<cv::MatShape>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_MatShapeG_delete(std::vector<cv::MatShape>* instance) {
			delete instance;
	}

	// std::vector<cv::MatShape>::len() generated
	// ("std::vector<cv::MatShape>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_MatShapeG_len_const(const std::vector<cv::MatShape>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::MatShape>::isEmpty() generated
	// ("std::vector<cv::MatShape>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_MatShapeG_isEmpty_const(const std::vector<cv::MatShape>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::MatShape>::capacity() generated
	// ("std::vector<cv::MatShape>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_MatShapeG_capacity_const(const std::vector<cv::MatShape>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::MatShape>::shrinkToFit() generated
	// ("std::vector<cv::MatShape>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_MatShapeG_shrinkToFit(std::vector<cv::MatShape>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::MatShape>::reserve(Primitive) generated
	// ("std::vector<cv::MatShape>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_MatShapeG_reserve_size_t(std::vector<cv::MatShape>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::MatShape>::remove(Primitive) generated
	// ("std::vector<cv::MatShape>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_MatShapeG_remove_size_t(std::vector<cv::MatShape>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::MatShape>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::MatShape>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_MatShapeG_swap_size_t_size_t(std::vector<cv::MatShape>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::MatShape>::clear() generated
	// ("std::vector<cv::MatShape>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_MatShapeG_clear(std::vector<cv::MatShape>* instance) {
			instance->clear();
	}

	// std::vector<cv::MatShape>::push(TraitClass) generated
	// ("std::vector<cv::MatShape>::push", vec![(pred!(mut, ["val"], ["const cv::MatShape"]), _)]),
	void std_vectorLcv_MatShapeG_push_const_MatShape(std::vector<cv::MatShape>* instance, const cv::MatShape* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::MatShape>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::MatShape>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::MatShape"]), _)]),
	void std_vectorLcv_MatShapeG_insert_size_t_const_MatShape(std::vector<cv::MatShape>* instance, size_t index, const cv::MatShape* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::MatShape>::get(Primitive) generated
	// ("std::vector<cv::MatShape>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_MatShapeG_get_const_size_t(const std::vector<cv::MatShape>* instance, size_t index, cv::MatShape** ocvrs_return) {
			cv::MatShape ret = (*instance)[index];
			*ocvrs_return = new cv::MatShape(ret);
	}

	// std::vector<cv::MatShape>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::MatShape>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::MatShape"]), _)]),
	void std_vectorLcv_MatShapeG_set_size_t_const_MatShape(std::vector<cv::MatShape>* instance, size_t index, const cv::MatShape* val) {
			(*instance)[index] = *val;
	}

}


