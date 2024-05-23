extern "C" {
	// std::vector<cv::Ptr<cv::linemod::Modality>>::new() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::linemod::Modality>>* std_vectorLcv_PtrLcv_linemod_ModalityGG_new_const() {
			std::vector<cv::Ptr<cv::linemod::Modality>>* ret = new std::vector<cv::Ptr<cv::linemod::Modality>>();
			return ret;
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::delete() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_delete(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			delete instance;
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::len() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_linemod_ModalityGG_len_const(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::isEmpty() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_PtrLcv_linemod_ModalityGG_isEmpty_const(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::capacity() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_PtrLcv_linemod_ModalityGG_capacity_const(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::shrinkToFit() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_shrinkToFit(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::reserve(Primitive) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_reserve_size_t(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::remove(Primitive) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_remove_size_t(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::clear() generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_clear(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			instance->clear();
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::push(CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::push", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::linemod::Modality>"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_push_const_PtrLModalityG(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, const cv::Ptr<cv::linemod::Modality>* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::insert(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::linemod::Modality>"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_insert_size_t_const_PtrLModalityG(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, const cv::Ptr<cv::linemod::Modality>* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::get(Primitive) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_get_const_size_t(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>** ocvrs_return) {
			cv::Ptr<cv::linemod::Modality> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::linemod::Modality>(ret);
	}

	// std::vector<cv::Ptr<cv::linemod::Modality>>::set(Primitive, CppPassByVoidPtr) generated
	// ("std::vector<cv::Ptr<cv::linemod::Modality>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Ptr<cv::linemod::Modality>"]), _)]),
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_set_size_t_const_PtrLModalityG(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, const cv::Ptr<cv::linemod::Modality>* val) {
			(*instance)[index] = *val;
	}

}


