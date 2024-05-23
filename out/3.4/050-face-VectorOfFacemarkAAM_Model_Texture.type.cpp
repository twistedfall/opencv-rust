extern "C" {
	// std::vector<cv::face::FacemarkAAM::Model::Texture>::new() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::face::FacemarkAAM::Model::Texture>* std_vectorLcv_face_FacemarkAAM_Model_TextureG_new_const() {
			std::vector<cv::face::FacemarkAAM::Model::Texture>* ret = new std::vector<cv::face::FacemarkAAM::Model::Texture>();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::delete() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_delete(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			delete instance;
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::len() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_face_FacemarkAAM_Model_TextureG_len_const(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::isEmpty() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_face_FacemarkAAM_Model_TextureG_isEmpty_const(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::capacity() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_face_FacemarkAAM_Model_TextureG_capacity_const(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::shrinkToFit() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_shrinkToFit(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::reserve(Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_reserve_size_t(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::remove(Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_remove_size_t(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_swap_size_t_size_t(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::clear() generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_clear(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			instance->clear();
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::push(TraitClass) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::push", vec![(pred!(mut, ["val"], ["const cv::face::FacemarkAAM::Model::Texture"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_push_const_Texture(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, const cv::face::FacemarkAAM::Model::Texture* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::face::FacemarkAAM::Model::Texture"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_insert_size_t_const_Texture(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, const cv::face::FacemarkAAM::Model::Texture* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::get(Primitive) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_get_const_size_t(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture** ocvrs_return) {
			cv::face::FacemarkAAM::Model::Texture ret = (*instance)[index];
			*ocvrs_return = new cv::face::FacemarkAAM::Model::Texture(ret);
	}

	// std::vector<cv::face::FacemarkAAM::Model::Texture>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::face::FacemarkAAM::Model::Texture>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::face::FacemarkAAM::Model::Texture"]), _)]),
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_set_size_t_const_Texture(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, const cv::face::FacemarkAAM::Model::Texture* val) {
			(*instance)[index] = *val;
	}

}


