extern "C" {
	// std::vector<cv::linemod::Template>::new() generated
	// ("std::vector<cv::linemod::Template>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Template>* std_vectorLcv_linemod_TemplateG_new_const() {
			std::vector<cv::linemod::Template>* ret = new std::vector<cv::linemod::Template>();
			return ret;
	}

	// std::vector<cv::linemod::Template>::delete() generated
	// ("std::vector<cv::linemod::Template>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_TemplateG_delete(std::vector<cv::linemod::Template>* instance) {
			delete instance;
	}

	// std::vector<cv::linemod::Template>::len() generated
	// ("std::vector<cv::linemod::Template>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_linemod_TemplateG_len_const(const std::vector<cv::linemod::Template>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::linemod::Template>::isEmpty() generated
	// ("std::vector<cv::linemod::Template>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_linemod_TemplateG_isEmpty_const(const std::vector<cv::linemod::Template>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::linemod::Template>::capacity() generated
	// ("std::vector<cv::linemod::Template>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_linemod_TemplateG_capacity_const(const std::vector<cv::linemod::Template>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::linemod::Template>::shrinkToFit() generated
	// ("std::vector<cv::linemod::Template>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_TemplateG_shrinkToFit(std::vector<cv::linemod::Template>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::linemod::Template>::reserve(Primitive) generated
	// ("std::vector<cv::linemod::Template>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_TemplateG_reserve_size_t(std::vector<cv::linemod::Template>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::linemod::Template>::remove(Primitive) generated
	// ("std::vector<cv::linemod::Template>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_TemplateG_remove_size_t(std::vector<cv::linemod::Template>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::linemod::Template>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::linemod::Template>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_linemod_TemplateG_swap_size_t_size_t(std::vector<cv::linemod::Template>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::linemod::Template>::clear() generated
	// ("std::vector<cv::linemod::Template>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_linemod_TemplateG_clear(std::vector<cv::linemod::Template>* instance) {
			instance->clear();
	}

	// std::vector<cv::linemod::Template>::push(TraitClass) generated
	// ("std::vector<cv::linemod::Template>::push", vec![(pred!(mut, ["val"], ["const cv::linemod::Template"]), _)]),
	void std_vectorLcv_linemod_TemplateG_push_const_Template(std::vector<cv::linemod::Template>* instance, const cv::linemod::Template* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::linemod::Template>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::linemod::Template>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Template"]), _)]),
	void std_vectorLcv_linemod_TemplateG_insert_size_t_const_Template(std::vector<cv::linemod::Template>* instance, size_t index, const cv::linemod::Template* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::linemod::Template>::get(Primitive) generated
	// ("std::vector<cv::linemod::Template>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_linemod_TemplateG_get_const_size_t(const std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template** ocvrs_return) {
			cv::linemod::Template ret = (*instance)[index];
			*ocvrs_return = new cv::linemod::Template(ret);
	}

	// std::vector<cv::linemod::Template>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::linemod::Template>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Template"]), _)]),
	void std_vectorLcv_linemod_TemplateG_set_size_t_const_Template(std::vector<cv::linemod::Template>* instance, size_t index, const cv::linemod::Template* val) {
			(*instance)[index] = *val;
	}

}


