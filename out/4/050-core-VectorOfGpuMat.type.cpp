extern "C" {
	// std::vector<cv::cuda::GpuMat>::new() generated
	// ("std::vector<cv::cuda::GpuMat>::new", vec![(pred!(const, [], []), _)]),
	std::vector<cv::cuda::GpuMat>* std_vectorLcv_cuda_GpuMatG_new_const() {
			std::vector<cv::cuda::GpuMat>* ret = new std::vector<cv::cuda::GpuMat>();
			return ret;
	}

	// std::vector<cv::cuda::GpuMat>::delete() generated
	// ("std::vector<cv::cuda::GpuMat>::delete", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_cuda_GpuMatG_delete(std::vector<cv::cuda::GpuMat>* instance) {
			delete instance;
	}

	// std::vector<cv::cuda::GpuMat>::len() generated
	// ("std::vector<cv::cuda::GpuMat>::len", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_cuda_GpuMatG_len_const(const std::vector<cv::cuda::GpuMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}

	// std::vector<cv::cuda::GpuMat>::isEmpty() generated
	// ("std::vector<cv::cuda::GpuMat>::isEmpty", vec![(pred!(const, [], []), _)]),
	bool std_vectorLcv_cuda_GpuMatG_isEmpty_const(const std::vector<cv::cuda::GpuMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// std::vector<cv::cuda::GpuMat>::capacity() generated
	// ("std::vector<cv::cuda::GpuMat>::capacity", vec![(pred!(const, [], []), _)]),
	size_t std_vectorLcv_cuda_GpuMatG_capacity_const(const std::vector<cv::cuda::GpuMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}

	// std::vector<cv::cuda::GpuMat>::shrinkToFit() generated
	// ("std::vector<cv::cuda::GpuMat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_cuda_GpuMatG_shrinkToFit(std::vector<cv::cuda::GpuMat>* instance) {
			instance->shrink_to_fit();
	}

	// std::vector<cv::cuda::GpuMat>::reserve(Primitive) generated
	// ("std::vector<cv::cuda::GpuMat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_reserve_size_t(std::vector<cv::cuda::GpuMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}

	// std::vector<cv::cuda::GpuMat>::remove(Primitive) generated
	// ("std::vector<cv::cuda::GpuMat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_remove_size_t(std::vector<cv::cuda::GpuMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}

	// std::vector<cv::cuda::GpuMat>::swap(Primitive, Primitive) generated
	// ("std::vector<cv::cuda::GpuMat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_swap_size_t_size_t(std::vector<cv::cuda::GpuMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}

	// std::vector<cv::cuda::GpuMat>::clear() generated
	// ("std::vector<cv::cuda::GpuMat>::clear", vec![(pred!(mut, [], []), _)]),
	void std_vectorLcv_cuda_GpuMatG_clear(std::vector<cv::cuda::GpuMat>* instance) {
			instance->clear();
	}

	// std::vector<cv::cuda::GpuMat>::push(TraitClass) generated
	// ("std::vector<cv::cuda::GpuMat>::push", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_push_const_GpuMat(std::vector<cv::cuda::GpuMat>* instance, const cv::cuda::GpuMat* val) {
			instance->push_back(*val);
	}

	// std::vector<cv::cuda::GpuMat>::insert(Primitive, TraitClass) generated
	// ("std::vector<cv::cuda::GpuMat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::cuda::GpuMat"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_insert_size_t_const_GpuMat(std::vector<cv::cuda::GpuMat>* instance, size_t index, const cv::cuda::GpuMat* val) {
			instance->insert(instance->begin() + index, *val);
	}

	// std::vector<cv::cuda::GpuMat>::get(Primitive) generated
	// ("std::vector<cv::cuda::GpuMat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_get_const_size_t(const std::vector<cv::cuda::GpuMat>* instance, size_t index, cv::cuda::GpuMat** ocvrs_return) {
			cv::cuda::GpuMat ret = (*instance)[index];
			*ocvrs_return = new cv::cuda::GpuMat(ret);
	}

	// std::vector<cv::cuda::GpuMat>::set(Primitive, TraitClass) generated
	// ("std::vector<cv::cuda::GpuMat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::cuda::GpuMat"]), _)]),
	void std_vectorLcv_cuda_GpuMatG_set_size_t_const_GpuMat(std::vector<cv::cuda::GpuMat>* instance, size_t index, const cv::cuda::GpuMat* val) {
			(*instance)[index] = *val;
	}

}


