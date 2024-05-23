extern "C" {
	// std::vector<cv::dnn::Arg>::new() generated
	// ("std::vector<cv::dnn::Arg>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::dnn::Arg>* std_vectorLcv_dnn_ArgG_new_const() {
			std::vector<cv::dnn::Arg>* ret = new std::vector<cv::dnn::Arg>();
			return ret;
	}

	// std::vector<cv::dnn::Arg>::delete() generated
	// ("std::vector<cv::dnn::Arg>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_ArgG_delete(std::vector<cv::dnn::Arg>* instance) {
			delete instance;
	}

	// std::vector<cv::dnn::Arg>::len() generated
	// ("std::vector<cv::dnn::Arg>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_ArgG_len_const(const std::vector<cv::dnn::Arg>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::dnn::Arg>::isEmpty() generated
	// ("std::vector<cv::dnn::Arg>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_dnn_ArgG_isEmpty_const(const std::vector<cv::dnn::Arg>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::dnn::Arg>::capacity() generated
	// ("std::vector<cv::dnn::Arg>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_dnn_ArgG_capacity_const(const std::vector<cv::dnn::Arg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::dnn::Arg>::shrinkToFit() generated
	// ("std::vector<cv::dnn::Arg>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_ArgG_shrinkToFit(std::vector<cv::dnn::Arg>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::dnn::Arg>::reserve(Primitive) generated
	// ("std::vector<cv::dnn::Arg>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_ArgG_reserve_size_t(std::vector<cv::dnn::Arg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::dnn::Arg>::remove(Primitive) generated
	// ("std::vector<cv::dnn::Arg>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_ArgG_remove_size_t(std::vector<cv::dnn::Arg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::dnn::Arg>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::dnn::Arg>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_dnn_ArgG_swap_size_t_size_t(std::vector<cv::dnn::Arg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::dnn::Arg>::clear() generated
	// ("std::vector<cv::dnn::Arg>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_dnn_ArgG_clear(std::vector<cv::dnn::Arg>* instance) {
			instance->clear();
	}

	// std::vector<cv::dnn::Arg>::push(TraitClass) generated
	// ("std::vector<cv::dnn::Arg>::push", vec![(pred!(mut, ["val"], ["const cv::dnn::Arg"]), _)]),
	void std_vectorLcv_dnn_ArgG_push_const_Arg(std::vector<cv::dnn::Arg>* instance, const cv::dnn::Arg* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::dnn::Arg>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::dnn::Arg>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::Arg"]), _)]),
	void std_vectorLcv_dnn_ArgG_insert_size_t_const_Arg(std::vector<cv::dnn::Arg>* instance, size_t index, const cv::dnn::Arg* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::dnn::Arg>::get(Primitive) generated
	// ("std::vector<cv::dnn::Arg>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_dnn_ArgG_get_const_size_t(const std::vector<cv::dnn::Arg>* instance, size_t index, cv::dnn::Arg** ocvrs_return) {
			cv::dnn::Arg ret = (*instance)[index];
			*ocvrs_return = new cv::dnn::Arg(ret);
	}

	// std::vector<cv::dnn::Arg>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::dnn::Arg>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::Arg"]), _)]),
	void std_vectorLcv_dnn_ArgG_set_size_t_const_Arg(std::vector<cv::dnn::Arg>* instance, size_t index, const cv::dnn::Arg* val) {
			(*instance)[index] = *val;
	}

}


