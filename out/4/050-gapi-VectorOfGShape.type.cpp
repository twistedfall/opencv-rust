extern "C" {
	// std::vector<cv::GShape>::new() generated
	// ("std::vector<cv::GShape>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GShape>* std_vectorLcv_GShapeG_new_const() {
			std::vector<cv::GShape>* ret = new std::vector<cv::GShape>();
			return ret;
	}

	// std::vector<cv::GShape>::delete() generated
	// ("std::vector<cv::GShape>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GShapeG_delete(std::vector<cv::GShape>* instance) {
			delete instance;
	}

	// std::vector<cv::GShape>::len() generated
	// ("std::vector<cv::GShape>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GShapeG_len_const(const std::vector<cv::GShape>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::GShape>::isEmpty() generated
	// ("std::vector<cv::GShape>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_GShapeG_isEmpty_const(const std::vector<cv::GShape>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::GShape>::capacity() generated
	// ("std::vector<cv::GShape>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_GShapeG_capacity_const(const std::vector<cv::GShape>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::GShape>::shrinkToFit() generated
	// ("std::vector<cv::GShape>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GShapeG_shrinkToFit(std::vector<cv::GShape>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::GShape>::reserve(Primitive) generated
	// ("std::vector<cv::GShape>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_GShapeG_reserve_size_t(std::vector<cv::GShape>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::GShape>::remove(Primitive) generated
	// ("std::vector<cv::GShape>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GShapeG_remove_size_t(std::vector<cv::GShape>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::GShape>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::GShape>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_GShapeG_swap_size_t_size_t(std::vector<cv::GShape>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::GShape>::clear() generated
	// ("std::vector<cv::GShape>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_GShapeG_clear(std::vector<cv::GShape>* instance) {
			instance->clear();
	}

	// std::vector<cv::GShape>::push(Enum) generated
	// ("std::vector<cv::GShape>::push", vec![(pred!(mut, ["val"], ["const cv::GShape"]), _)]),
	void std_vectorLcv_GShapeG_push_const_GShape(std::vector<cv::GShape>* instance, const cv::GShape val) {
			instance->push_back(val);
	}

	// std::vector<cv::GShape>::insert(Primitive, Enum) generated
	// ("std::vector<cv::GShape>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GShape"]), _)]),
	void std_vectorLcv_GShapeG_insert_size_t_const_GShape(std::vector<cv::GShape>* instance, size_t index, const cv::GShape val) {
			instance->insert(instance->begin() + index, val);
	}

	// std::vector<cv::GShape>::get(Primitive) generated
	// ("std::vector<cv::GShape>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_GShapeG_get_const_size_t(const std::vector<cv::GShape>* instance, size_t index, cv::GShape* ocvrs_return) {
			cv::GShape ret = (*instance)[index];
			*ocvrs_return = ret;
	}

	// std::vector<cv::GShape>::set(Primitive, Enum) generated
	// ("std::vector<cv::GShape>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GShape"]), _)]),
	void std_vectorLcv_GShapeG_set_size_t_const_GShape(std::vector<cv::GShape>* instance, size_t index, const cv::GShape val) {
			(*instance)[index] = val;
	}

	// std::vector<cv::GShape>::clone() generated
	// ("std::vector<cv::GShape>::clone", vec![(pred!(const, [], []), _)]),
	std::vector<cv::GShape>* std_vectorLcv_GShapeG_clone_const(const std::vector<cv::GShape>* instance) {
			std::vector<cv::GShape> ret = std::vector<cv::GShape>(*instance);
			return new std::vector<cv::GShape>(ret);
	}

	// std::vector<cv::GShape>::data() generated
	// ("std::vector<cv::GShape>::data", vec![(pred!(const, [], []), _)]),
	const cv::GShape* std_vectorLcv_GShapeG_data_const(const std::vector<cv::GShape>* instance) {
			const cv::GShape* ret = instance->data();
			return ret;
	}

	// std::vector<cv::GShape>::dataMut() generated
	// ("std::vector<cv::GShape>::dataMut", vec![(pred!(mut, [], []), _)]),
	cv::GShape* std_vectorLcv_GShapeG_dataMut(std::vector<cv::GShape>* instance) {
			cv::GShape* ret = instance->data();
			return ret;
	}

	// cv::fromSlice(Enum, Primitive) generated
	// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::GShape*", "size_t"]), _)]),
	std::vector<cv::GShape>* cv_fromSlice_const_const_GShapeX_size_t(const cv::GShape* data, size_t len) {
			return new std::vector<cv::GShape>(data, data + len);
	}

}


