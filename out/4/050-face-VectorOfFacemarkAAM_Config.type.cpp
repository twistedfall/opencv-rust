extern "C" {
	// std::vector<cv::face::FacemarkAAM::Config>::new() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::face::FacemarkAAM::Config>* std_vectorLcv_face_FacemarkAAM_ConfigG_new_const() {
			std::vector<cv::face::FacemarkAAM::Config>* ret = new std::vector<cv::face::FacemarkAAM::Config>();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Config>::delete() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_delete(std::vector<cv::face::FacemarkAAM::Config>* instance) {
			delete instance;
	}

	// std::vector<cv::face::FacemarkAAM::Config>::len() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_face_FacemarkAAM_ConfigG_len_const(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Config>::isEmpty() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_face_FacemarkAAM_ConfigG_isEmpty_const(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Config>::capacity() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_face_FacemarkAAM_ConfigG_capacity_const(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Config>::shrinkToFit() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_shrinkToFit(std::vector<cv::face::FacemarkAAM::Config>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::face::FacemarkAAM::Config>::reserve(Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_reserve_size_t(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::face::FacemarkAAM::Config>::remove(Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_remove_size_t(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::face::FacemarkAAM::Config>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_swap_size_t_size_t(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::face::FacemarkAAM::Config>::clear() generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_clear(std::vector<cv::face::FacemarkAAM::Config>* instance) {
			instance->clear();
	}

	// std::vector<cv::face::FacemarkAAM::Config>::push(TraitClass) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::push", vec![(pred!(mut, ["val"], ["const cv::face::FacemarkAAM::Config"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_push_const_Config(std::vector<cv::face::FacemarkAAM::Config>* instance, const cv::face::FacemarkAAM::Config* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::face::FacemarkAAM::Config>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::face::FacemarkAAM::Config"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_insert_size_t_const_Config(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, const cv::face::FacemarkAAM::Config* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::face::FacemarkAAM::Config>::get(Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_get_const_size_t(const std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config** ocvrs_return) {
			cv::face::FacemarkAAM::Config ret = (*instance)[index];
			*ocvrs_return = new cv::face::FacemarkAAM::Config(ret);
	}

	// std::vector<cv::face::FacemarkAAM::Config>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::face::FacemarkAAM::Config>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::face::FacemarkAAM::Config"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_ConfigG_set_size_t_const_Config(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, const cv::face::FacemarkAAM::Config* val) {
			(*instance)[index] = *val;
	}

}


