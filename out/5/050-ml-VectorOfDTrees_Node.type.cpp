extern "C" {
	// std::vector<cv::ml::DTrees::Node>::new() generated
	// ("std::vector<cv::ml::DTrees::Node>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ml::DTrees::Node>* std_vectorLcv_ml_DTrees_NodeG_new_const() {
			std::vector<cv::ml::DTrees::Node>* ret = new std::vector<cv::ml::DTrees::Node>();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Node>::delete() generated
	// ("std::vector<cv::ml::DTrees::Node>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_delete(std::vector<cv::ml::DTrees::Node>* instance) {
			delete instance;
	}

	// std::vector<cv::ml::DTrees::Node>::len() generated
	// ("std::vector<cv::ml::DTrees::Node>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ml_DTrees_NodeG_len_const(const std::vector<cv::ml::DTrees::Node>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Node>::isEmpty() generated
	// ("std::vector<cv::ml::DTrees::Node>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_ml_DTrees_NodeG_isEmpty_const(const std::vector<cv::ml::DTrees::Node>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Node>::capacity() generated
	// ("std::vector<cv::ml::DTrees::Node>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_ml_DTrees_NodeG_capacity_const(const std::vector<cv::ml::DTrees::Node>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::ml::DTrees::Node>::shrinkToFit() generated
	// ("std::vector<cv::ml::DTrees::Node>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_shrinkToFit(std::vector<cv::ml::DTrees::Node>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::ml::DTrees::Node>::reserve(Primitive) generated
	// ("std::vector<cv::ml::DTrees::Node>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_reserve_size_t(std::vector<cv::ml::DTrees::Node>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::ml::DTrees::Node>::remove(Primitive) generated
	// ("std::vector<cv::ml::DTrees::Node>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_remove_size_t(std::vector<cv::ml::DTrees::Node>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::ml::DTrees::Node>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::ml::DTrees::Node>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_swap_size_t_size_t(std::vector<cv::ml::DTrees::Node>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::ml::DTrees::Node>::clear() generated
	// ("std::vector<cv::ml::DTrees::Node>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_clear(std::vector<cv::ml::DTrees::Node>* instance) {
			instance->clear();
	}

	// std::vector<cv::ml::DTrees::Node>::push(TraitClass) generated
	// ("std::vector<cv::ml::DTrees::Node>::push", vec![(pred!(mut, ["val"], ["const cv::ml::DTrees::Node"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_push_const_Node(std::vector<cv::ml::DTrees::Node>* instance, const cv::ml::DTrees::Node* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::ml::DTrees::Node>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::ml::DTrees::Node>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ml::DTrees::Node"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_insert_size_t_const_Node(std::vector<cv::ml::DTrees::Node>* instance, size_t index, const cv::ml::DTrees::Node* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::ml::DTrees::Node>::get(Primitive) generated
	// ("std::vector<cv::ml::DTrees::Node>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_get_const_size_t(const std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node** ocvrs_return) {
			cv::ml::DTrees::Node ret = (*instance)[index];
			*ocvrs_return = new cv::ml::DTrees::Node(ret);
	}

	// std::vector<cv::ml::DTrees::Node>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::ml::DTrees::Node>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::ml::DTrees::Node"]), _)]),
	void std_vectorLcv_ml_DTrees_NodeG_set_size_t_const_Node(std::vector<cv::ml::DTrees::Node>* instance, size_t index, const cv::ml::DTrees::Node* val) {
			(*instance)[index] = *val;
	}

}


