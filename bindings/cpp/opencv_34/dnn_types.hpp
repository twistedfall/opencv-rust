template struct Result<bool>;
template struct Result<const cv::dnn::DictValue*>;
template struct Result<const cv::dnn::DictValue**>;
template struct Result<const double>;
template struct Result<const long>;
template struct Result<cv::AsyncArray*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Mat**>;
template struct Result<cv::Ptr<cv::dnn::AbsLayer>*>;
template struct Result<cv::Ptr<cv::dnn::AccumLayer>*>;
template struct Result<cv::Ptr<cv::dnn::BNLLLayer>*>;
template struct Result<cv::Ptr<cv::dnn::BackendNode>*>;
template struct Result<cv::Ptr<cv::dnn::BackendWrapper>*>;
template struct Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>;
template struct Result<cv::Ptr<cv::dnn::BatchNormLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ConcatLayer>*>;
template struct Result<cv::Ptr<cv::dnn::CorrelationLayer>*>;
template struct Result<cv::Ptr<cv::dnn::DataAugmentationLayer>*>;
template struct Result<cv::Ptr<cv::dnn::DetectionOutputLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ELULayer>*>;
template struct Result<cv::Ptr<cv::dnn::EltwiseLayer>*>;
template struct Result<cv::Ptr<cv::dnn::FlattenLayer>*>;
template struct Result<cv::Ptr<cv::dnn::FlowWarpLayer>*>;
template struct Result<cv::Ptr<cv::dnn::InnerProductLayer>*>;
template struct Result<cv::Ptr<cv::dnn::LRNLayer>*>;
template struct Result<cv::Ptr<cv::dnn::LSTMLayer>*>;
template struct Result<cv::Ptr<cv::dnn::Layer>*>;
template struct Result<cv::Ptr<cv::dnn::MVNLayer>*>;
template struct Result<cv::Ptr<cv::dnn::MaxUnpoolLayer>*>;
template struct Result<cv::Ptr<cv::dnn::MishLayer>*>;
template struct Result<cv::Ptr<cv::dnn::NormalizeBBoxLayer>*>;
template struct Result<cv::Ptr<cv::dnn::PaddingLayer>*>;
template struct Result<cv::Ptr<cv::dnn::PermuteLayer>*>;
template struct Result<cv::Ptr<cv::dnn::PoolingLayer>*>;
template struct Result<cv::Ptr<cv::dnn::PowerLayer>*>;
template struct Result<cv::Ptr<cv::dnn::PriorBoxLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ProposalLayer>*>;
template struct Result<cv::Ptr<cv::dnn::RNNLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ReLU6Layer>*>;
template struct Result<cv::Ptr<cv::dnn::ReLULayer>*>;
template struct Result<cv::Ptr<cv::dnn::RegionLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ReorgLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ReshapeLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ResizeLayer>*>;
template struct Result<cv::Ptr<cv::dnn::ScaleLayer>*>;
template struct Result<cv::Ptr<cv::dnn::SigmoidLayer>*>;
template struct Result<cv::Ptr<cv::dnn::SliceLayer>*>;
template struct Result<cv::Ptr<cv::dnn::SoftmaxLayer>*>;
template struct Result<cv::Ptr<cv::dnn::SplitLayer>*>;
template struct Result<cv::Ptr<cv::dnn::SwishLayer>*>;
template struct Result<cv::Ptr<cv::dnn::TanHLayer>*>;
template struct Result<cv::Range*>;
template struct Result<cv::Rect_<double>>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::RotatedRect*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::_InputArray*>;
template struct Result<cv::_InputOutputArray*>;
template struct Result<cv::_OutputArray*>;
template struct Result<cv::dnn::DictValue*>;
template struct Result<cv::dnn::DictValue**>;
template struct Result<cv::dnn::Layer*>;
template struct Result<cv::dnn::Net*>;
template struct Result<cv::dnn::Target>;
template struct Result<cv::dnn::_Range*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<long>;
template struct Result<std::vector<bool>*>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Ptr<cv::dnn::Layer>>*>;
template struct Result<std::vector<cv::Range>*>;
template struct Result<std::vector<cv::String>*>;
template struct Result<std::vector<cv::dnn::Target>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::vector<cv::Range>>*>;
template struct Result<std::vector<std::vector<int>>*>;
template struct Result<std::vector<unsigned long>*>;
template struct Result<unsigned char>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" {
	void cv_PtrOfAbsLayer_delete(cv::Ptr<cv::dnn::AbsLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AbsLayer* cv_PtrOfAbsLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AbsLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AbsLayer* cv_PtrOfAbsLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AbsLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::AccumLayer>* cv_PtrOfAccumLayer_new(cv::dnn::AccumLayer* val) {
		return new cv::Ptr<cv::dnn::AccumLayer>(val);
	}
	
	void cv_PtrOfAccumLayer_delete(cv::Ptr<cv::dnn::AccumLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AccumLayer* cv_PtrOfAccumLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AccumLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AccumLayer* cv_PtrOfAccumLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AccumLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfActivationLayer_delete(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ActivationLayer* cv_PtrOfActivationLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ActivationLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ActivationLayer* cv_PtrOfActivationLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBNLLLayer_delete(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
		delete instance;
	}

	const cv::dnn::BNLLLayer* cv_PtrOfBNLLLayer_get_inner_ptr(const cv::Ptr<cv::dnn::BNLLLayer>* instance) {
		return instance->get();
	}

	cv::dnn::BNLLLayer* cv_PtrOfBNLLLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::BackendNode>* cv_PtrOfBackendNode_new(cv::dnn::BackendNode* val) {
		return new cv::Ptr<cv::dnn::BackendNode>(val);
	}
	
	void cv_PtrOfBackendNode_delete(cv::Ptr<cv::dnn::BackendNode>* instance) {
		delete instance;
	}

	const cv::dnn::BackendNode* cv_PtrOfBackendNode_get_inner_ptr(const cv::Ptr<cv::dnn::BackendNode>* instance) {
		return instance->get();
	}

	cv::dnn::BackendNode* cv_PtrOfBackendNode_get_inner_ptr_mut(cv::Ptr<cv::dnn::BackendNode>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackendWrapper_delete(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
		delete instance;
	}

	const cv::dnn::BackendWrapper* cv_PtrOfBackendWrapper_get_inner_ptr(const cv::Ptr<cv::dnn::BackendWrapper>* instance) {
		return instance->get();
	}

	cv::dnn::BackendWrapper* cv_PtrOfBackendWrapper_get_inner_ptr_mut(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::BaseConvolutionLayer>* cv_PtrOfBaseConvolutionLayer_new(cv::dnn::BaseConvolutionLayer* val) {
		return new cv::Ptr<cv::dnn::BaseConvolutionLayer>(val);
	}
	
	void cv_PtrOfBaseConvolutionLayer_delete(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
		delete instance;
	}

	const cv::dnn::BaseConvolutionLayer* cv_PtrOfBaseConvolutionLayer_get_inner_ptr(const cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
		return instance->get();
	}

	cv::dnn::BaseConvolutionLayer* cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBatchNormLayer_delete(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
		delete instance;
	}

	const cv::dnn::BatchNormLayer* cv_PtrOfBatchNormLayer_get_inner_ptr(const cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
		return instance->get();
	}

	cv::dnn::BatchNormLayer* cv_PtrOfBatchNormLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::ConcatLayer>* cv_PtrOfConcatLayer_new(cv::dnn::ConcatLayer* val) {
		return new cv::Ptr<cv::dnn::ConcatLayer>(val);
	}
	
	void cv_PtrOfConcatLayer_delete(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ConcatLayer* cv_PtrOfConcatLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ConcatLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ConcatLayer* cv_PtrOfConcatLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::CorrelationLayer>* cv_PtrOfCorrelationLayer_new(cv::dnn::CorrelationLayer* val) {
		return new cv::Ptr<cv::dnn::CorrelationLayer>(val);
	}
	
	void cv_PtrOfCorrelationLayer_delete(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CorrelationLayer* cv_PtrOfCorrelationLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CorrelationLayer* cv_PtrOfCorrelationLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::DataAugmentationLayer>* cv_PtrOfDataAugmentationLayer_new(cv::dnn::DataAugmentationLayer* val) {
		return new cv::Ptr<cv::dnn::DataAugmentationLayer>(val);
	}
	
	void cv_PtrOfDataAugmentationLayer_delete(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
		delete instance;
	}

	const cv::dnn::DataAugmentationLayer* cv_PtrOfDataAugmentationLayer_get_inner_ptr(const cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
		return instance->get();
	}

	cv::dnn::DataAugmentationLayer* cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::DetectionOutputLayer>* cv_PtrOfDetectionOutputLayer_new(cv::dnn::DetectionOutputLayer* val) {
		return new cv::Ptr<cv::dnn::DetectionOutputLayer>(val);
	}
	
	void cv_PtrOfDetectionOutputLayer_delete(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
		delete instance;
	}

	const cv::dnn::DetectionOutputLayer* cv_PtrOfDetectionOutputLayer_get_inner_ptr(const cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
		return instance->get();
	}

	cv::dnn::DetectionOutputLayer* cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfELULayer_delete(cv::Ptr<cv::dnn::ELULayer>* instance) {
		delete instance;
	}

	const cv::dnn::ELULayer* cv_PtrOfELULayer_get_inner_ptr(const cv::Ptr<cv::dnn::ELULayer>* instance) {
		return instance->get();
	}

	cv::dnn::ELULayer* cv_PtrOfELULayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ELULayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::EltwiseLayer>* cv_PtrOfEltwiseLayer_new(cv::dnn::EltwiseLayer* val) {
		return new cv::Ptr<cv::dnn::EltwiseLayer>(val);
	}
	
	void cv_PtrOfEltwiseLayer_delete(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
		delete instance;
	}

	const cv::dnn::EltwiseLayer* cv_PtrOfEltwiseLayer_get_inner_ptr(const cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
		return instance->get();
	}

	cv::dnn::EltwiseLayer* cv_PtrOfEltwiseLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::FlattenLayer>* cv_PtrOfFlattenLayer_new(cv::dnn::FlattenLayer* val) {
		return new cv::Ptr<cv::dnn::FlattenLayer>(val);
	}
	
	void cv_PtrOfFlattenLayer_delete(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
		delete instance;
	}

	const cv::dnn::FlattenLayer* cv_PtrOfFlattenLayer_get_inner_ptr(const cv::Ptr<cv::dnn::FlattenLayer>* instance) {
		return instance->get();
	}

	cv::dnn::FlattenLayer* cv_PtrOfFlattenLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::FlowWarpLayer>* cv_PtrOfFlowWarpLayer_new(cv::dnn::FlowWarpLayer* val) {
		return new cv::Ptr<cv::dnn::FlowWarpLayer>(val);
	}
	
	void cv_PtrOfFlowWarpLayer_delete(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
		delete instance;
	}

	const cv::dnn::FlowWarpLayer* cv_PtrOfFlowWarpLayer_get_inner_ptr(const cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
		return instance->get();
	}

	cv::dnn::FlowWarpLayer* cv_PtrOfFlowWarpLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::InnerProductLayer>* cv_PtrOfInnerProductLayer_new(cv::dnn::InnerProductLayer* val) {
		return new cv::Ptr<cv::dnn::InnerProductLayer>(val);
	}
	
	void cv_PtrOfInnerProductLayer_delete(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
		delete instance;
	}

	const cv::dnn::InnerProductLayer* cv_PtrOfInnerProductLayer_get_inner_ptr(const cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
		return instance->get();
	}

	cv::dnn::InnerProductLayer* cv_PtrOfInnerProductLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::LRNLayer>* cv_PtrOfLRNLayer_new(cv::dnn::LRNLayer* val) {
		return new cv::Ptr<cv::dnn::LRNLayer>(val);
	}
	
	void cv_PtrOfLRNLayer_delete(cv::Ptr<cv::dnn::LRNLayer>* instance) {
		delete instance;
	}

	const cv::dnn::LRNLayer* cv_PtrOfLRNLayer_get_inner_ptr(const cv::Ptr<cv::dnn::LRNLayer>* instance) {
		return instance->get();
	}

	cv::dnn::LRNLayer* cv_PtrOfLRNLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::LRNLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfLSTMLayer_delete(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
		delete instance;
	}

	const cv::dnn::LSTMLayer* cv_PtrOfLSTMLayer_get_inner_ptr(const cv::Ptr<cv::dnn::LSTMLayer>* instance) {
		return instance->get();
	}

	cv::dnn::LSTMLayer* cv_PtrOfLSTMLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::Layer>* cv_PtrOfLayer_new(cv::dnn::Layer* val) {
		return new cv::Ptr<cv::dnn::Layer>(val);
	}
	
	void cv_PtrOfLayer_delete(cv::Ptr<cv::dnn::Layer>* instance) {
		delete instance;
	}

	const cv::dnn::Layer* cv_PtrOfLayer_get_inner_ptr(const cv::Ptr<cv::dnn::Layer>* instance) {
		return instance->get();
	}

	cv::dnn::Layer* cv_PtrOfLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::Layer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::MVNLayer>* cv_PtrOfMVNLayer_new(cv::dnn::MVNLayer* val) {
		return new cv::Ptr<cv::dnn::MVNLayer>(val);
	}
	
	void cv_PtrOfMVNLayer_delete(cv::Ptr<cv::dnn::MVNLayer>* instance) {
		delete instance;
	}

	const cv::dnn::MVNLayer* cv_PtrOfMVNLayer_get_inner_ptr(const cv::Ptr<cv::dnn::MVNLayer>* instance) {
		return instance->get();
	}

	cv::dnn::MVNLayer* cv_PtrOfMVNLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::MVNLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::MaxUnpoolLayer>* cv_PtrOfMaxUnpoolLayer_new(cv::dnn::MaxUnpoolLayer* val) {
		return new cv::Ptr<cv::dnn::MaxUnpoolLayer>(val);
	}
	
	void cv_PtrOfMaxUnpoolLayer_delete(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
		delete instance;
	}

	const cv::dnn::MaxUnpoolLayer* cv_PtrOfMaxUnpoolLayer_get_inner_ptr(const cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
		return instance->get();
	}

	cv::dnn::MaxUnpoolLayer* cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMishLayer_delete(cv::Ptr<cv::dnn::MishLayer>* instance) {
		delete instance;
	}

	const cv::dnn::MishLayer* cv_PtrOfMishLayer_get_inner_ptr(const cv::Ptr<cv::dnn::MishLayer>* instance) {
		return instance->get();
	}

	cv::dnn::MishLayer* cv_PtrOfMishLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::MishLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::NormalizeBBoxLayer>* cv_PtrOfNormalizeBBoxLayer_new(cv::dnn::NormalizeBBoxLayer* val) {
		return new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(val);
	}
	
	void cv_PtrOfNormalizeBBoxLayer_delete(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
		delete instance;
	}

	const cv::dnn::NormalizeBBoxLayer* cv_PtrOfNormalizeBBoxLayer_get_inner_ptr(const cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
		return instance->get();
	}

	cv::dnn::NormalizeBBoxLayer* cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::NormalizeBBoxLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::PaddingLayer>* cv_PtrOfPaddingLayer_new(cv::dnn::PaddingLayer* val) {
		return new cv::Ptr<cv::dnn::PaddingLayer>(val);
	}
	
	void cv_PtrOfPaddingLayer_delete(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PaddingLayer* cv_PtrOfPaddingLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PaddingLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PaddingLayer* cv_PtrOfPaddingLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::PermuteLayer>* cv_PtrOfPermuteLayer_new(cv::dnn::PermuteLayer* val) {
		return new cv::Ptr<cv::dnn::PermuteLayer>(val);
	}
	
	void cv_PtrOfPermuteLayer_delete(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PermuteLayer* cv_PtrOfPermuteLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PermuteLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PermuteLayer* cv_PtrOfPermuteLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::PoolingLayer>* cv_PtrOfPoolingLayer_new(cv::dnn::PoolingLayer* val) {
		return new cv::Ptr<cv::dnn::PoolingLayer>(val);
	}
	
	void cv_PtrOfPoolingLayer_delete(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PoolingLayer* cv_PtrOfPoolingLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PoolingLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PoolingLayer* cv_PtrOfPoolingLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfPowerLayer_delete(cv::Ptr<cv::dnn::PowerLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PowerLayer* cv_PtrOfPowerLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PowerLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PowerLayer* cv_PtrOfPowerLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PowerLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::PriorBoxLayer>* cv_PtrOfPriorBoxLayer_new(cv::dnn::PriorBoxLayer* val) {
		return new cv::Ptr<cv::dnn::PriorBoxLayer>(val);
	}
	
	void cv_PtrOfPriorBoxLayer_delete(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PriorBoxLayer* cv_PtrOfPriorBoxLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PriorBoxLayer* cv_PtrOfPriorBoxLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::ProposalLayer>* cv_PtrOfProposalLayer_new(cv::dnn::ProposalLayer* val) {
		return new cv::Ptr<cv::dnn::ProposalLayer>(val);
	}
	
	void cv_PtrOfProposalLayer_delete(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ProposalLayer* cv_PtrOfProposalLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ProposalLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ProposalLayer* cv_PtrOfProposalLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfRNNLayer_delete(cv::Ptr<cv::dnn::RNNLayer>* instance) {
		delete instance;
	}

	const cv::dnn::RNNLayer* cv_PtrOfRNNLayer_get_inner_ptr(const cv::Ptr<cv::dnn::RNNLayer>* instance) {
		return instance->get();
	}

	cv::dnn::RNNLayer* cv_PtrOfRNNLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::RNNLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfReLU6Layer_delete(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
		delete instance;
	}

	const cv::dnn::ReLU6Layer* cv_PtrOfReLU6Layer_get_inner_ptr(const cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
		return instance->get();
	}

	cv::dnn::ReLU6Layer* cv_PtrOfReLU6Layer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfReLULayer_delete(cv::Ptr<cv::dnn::ReLULayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReLULayer* cv_PtrOfReLULayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReLULayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReLULayer* cv_PtrOfReLULayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReLULayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::RegionLayer>* cv_PtrOfRegionLayer_new(cv::dnn::RegionLayer* val) {
		return new cv::Ptr<cv::dnn::RegionLayer>(val);
	}
	
	void cv_PtrOfRegionLayer_delete(cv::Ptr<cv::dnn::RegionLayer>* instance) {
		delete instance;
	}

	const cv::dnn::RegionLayer* cv_PtrOfRegionLayer_get_inner_ptr(const cv::Ptr<cv::dnn::RegionLayer>* instance) {
		return instance->get();
	}

	cv::dnn::RegionLayer* cv_PtrOfRegionLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::RegionLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::ReorgLayer>* cv_PtrOfReorgLayer_new(cv::dnn::ReorgLayer* val) {
		return new cv::Ptr<cv::dnn::ReorgLayer>(val);
	}
	
	void cv_PtrOfReorgLayer_delete(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReorgLayer* cv_PtrOfReorgLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReorgLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReorgLayer* cv_PtrOfReorgLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::ReshapeLayer>* cv_PtrOfReshapeLayer_new(cv::dnn::ReshapeLayer* val) {
		return new cv::Ptr<cv::dnn::ReshapeLayer>(val);
	}
	
	void cv_PtrOfReshapeLayer_delete(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReshapeLayer* cv_PtrOfReshapeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReshapeLayer* cv_PtrOfReshapeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::ResizeLayer>* cv_PtrOfResizeLayer_new(cv::dnn::ResizeLayer* val) {
		return new cv::Ptr<cv::dnn::ResizeLayer>(val);
	}
	
	void cv_PtrOfResizeLayer_delete(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ResizeLayer* cv_PtrOfResizeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ResizeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ResizeLayer* cv_PtrOfResizeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ResizeLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::ScaleLayer>* cv_PtrOfScaleLayer_new(cv::dnn::ScaleLayer* val) {
		return new cv::Ptr<cv::dnn::ScaleLayer>(val);
	}
	
	void cv_PtrOfScaleLayer_delete(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ScaleLayer* cv_PtrOfScaleLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ScaleLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ScaleLayer* cv_PtrOfScaleLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfSigmoidLayer_delete(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SigmoidLayer* cv_PtrOfSigmoidLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SigmoidLayer* cv_PtrOfSigmoidLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::SliceLayer>* cv_PtrOfSliceLayer_new(cv::dnn::SliceLayer* val) {
		return new cv::Ptr<cv::dnn::SliceLayer>(val);
	}
	
	void cv_PtrOfSliceLayer_delete(cv::Ptr<cv::dnn::SliceLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SliceLayer* cv_PtrOfSliceLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SliceLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SliceLayer* cv_PtrOfSliceLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SliceLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::SoftmaxLayer>* cv_PtrOfSoftmaxLayer_new(cv::dnn::SoftmaxLayer* val) {
		return new cv::Ptr<cv::dnn::SoftmaxLayer>(val);
	}
	
	void cv_PtrOfSoftmaxLayer_delete(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SoftmaxLayer* cv_PtrOfSoftmaxLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SoftmaxLayer* cv_PtrOfSoftmaxLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::dnn::SplitLayer>* cv_PtrOfSplitLayer_new(cv::dnn::SplitLayer* val) {
		return new cv::Ptr<cv::dnn::SplitLayer>(val);
	}
	
	void cv_PtrOfSplitLayer_delete(cv::Ptr<cv::dnn::SplitLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SplitLayer* cv_PtrOfSplitLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SplitLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SplitLayer* cv_PtrOfSplitLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SplitLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfSwishLayer_delete(cv::Ptr<cv::dnn::SwishLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SwishLayer* cv_PtrOfSwishLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SwishLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SwishLayer* cv_PtrOfSwishLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SwishLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTanHLayer_delete(cv::Ptr<cv::dnn::TanHLayer>* instance) {
		delete instance;
	}

	const cv::dnn::TanHLayer* cv_PtrOfTanHLayer_get_inner_ptr(const cv::Ptr<cv::dnn::TanHLayer>* instance) {
		return instance->get();
	}

	cv::dnn::TanHLayer* cv_PtrOfTanHLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::TanHLayer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_VectorOfMatShape_delete(std::vector<cv::dnn::MatShape>* instance) {
		delete instance;
	}

	std::vector<cv::dnn::MatShape>* cv_VectorOfMatShape_new() {
		return new std::vector<cv::dnn::MatShape>();
	}

	size_t cv_VectorOfMatShape_len(const std::vector<cv::dnn::MatShape>* instance) {
		return instance->size();
	}

	bool cv_VectorOfMatShape_is_empty(const std::vector<cv::dnn::MatShape>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfMatShape_capacity(const std::vector<cv::dnn::MatShape>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfMatShape_shrink_to_fit(std::vector<cv::dnn::MatShape>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfMatShape_reserve(std::vector<cv::dnn::MatShape>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfMatShape_remove(std::vector<cv::dnn::MatShape>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfMatShape_swap(std::vector<cv::dnn::MatShape>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfMatShape_clear(std::vector<cv::dnn::MatShape>* instance) {
		instance->clear();
	}

	void cv_VectorOfMatShape_push(std::vector<cv::dnn::MatShape>* instance, cv::dnn::MatShape* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfMatShape_insert(std::vector<cv::dnn::MatShape>* instance, size_t index, cv::dnn::MatShape* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::dnn::MatShape*> cv_VectorOfMatShape_get(const std::vector<cv::dnn::MatShape>* instance, size_t index) {
		return Ok<cv::dnn::MatShape*>(new cv::dnn::MatShape((*instance)[index]));
	}

	void cv_VectorOfMatShape_set(std::vector<cv::dnn::MatShape>* instance, size_t index, cv::dnn::MatShape* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfMatShape_input_array(std::vector<cv::dnn::MatShape>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfMatShape_output_array(std::vector<cv::dnn::MatShape>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfMatShape_input_output_array(std::vector<cv::dnn::MatShape>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfPtrOfBackendNode_delete(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::dnn::BackendNode>>* cv_VectorOfPtrOfBackendNode_new() {
		return new std::vector<cv::Ptr<cv::dnn::BackendNode>>();
	}

	size_t cv_VectorOfPtrOfBackendNode_len(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfBackendNode_is_empty(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfBackendNode_capacity(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfBackendNode_shrink_to_fit(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfBackendNode_reserve(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfBackendNode_remove(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfBackendNode_swap(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfBackendNode_clear(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfBackendNode_push(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, cv::Ptr<cv::dnn::BackendNode>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfBackendNode_insert(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_VectorOfPtrOfBackendNode_get(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::dnn::BackendNode>*>(new cv::Ptr<cv::dnn::BackendNode>((*instance)[index]));
	}

	void cv_VectorOfPtrOfBackendNode_set(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfPtrOfBackendWrapper_delete(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* cv_VectorOfPtrOfBackendWrapper_new() {
		return new std::vector<cv::Ptr<cv::dnn::BackendWrapper>>();
	}

	size_t cv_VectorOfPtrOfBackendWrapper_len(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfBackendWrapper_is_empty(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfBackendWrapper_capacity(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfBackendWrapper_shrink_to_fit(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfBackendWrapper_reserve(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfBackendWrapper_remove(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfBackendWrapper_swap(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfBackendWrapper_clear(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfBackendWrapper_push(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, cv::Ptr<cv::dnn::BackendWrapper>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfBackendWrapper_insert(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::dnn::BackendWrapper>*> cv_VectorOfPtrOfBackendWrapper_get(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::dnn::BackendWrapper>*>(new cv::Ptr<cv::dnn::BackendWrapper>((*instance)[index]));
	}

	void cv_VectorOfPtrOfBackendWrapper_set(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfPtrOfLayer_delete(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::dnn::Layer>>* cv_VectorOfPtrOfLayer_new() {
		return new std::vector<cv::Ptr<cv::dnn::Layer>>();
	}

	size_t cv_VectorOfPtrOfLayer_len(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfLayer_is_empty(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfLayer_capacity(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfLayer_shrink_to_fit(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfLayer_reserve(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfLayer_remove(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfLayer_swap(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfLayer_clear(std::vector<cv::Ptr<cv::dnn::Layer>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfLayer_push(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, cv::Ptr<cv::dnn::Layer>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfLayer_insert(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, cv::Ptr<cv::dnn::Layer>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::dnn::Layer>*> cv_VectorOfPtrOfLayer_get(const std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::dnn::Layer>*>(new cv::Ptr<cv::dnn::Layer>((*instance)[index]));
	}

	void cv_VectorOfPtrOfLayer_set(std::vector<cv::Ptr<cv::dnn::Layer>>* instance, size_t index, cv::Ptr<cv::dnn::Layer>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfTarget_delete(std::vector<cv::dnn::Target>* instance) {
		delete instance;
	}

	std::vector<cv::dnn::Target>* cv_VectorOfTarget_new() {
		return new std::vector<cv::dnn::Target>();
	}

	size_t cv_VectorOfTarget_len(const std::vector<cv::dnn::Target>* instance) {
		return instance->size();
	}

	bool cv_VectorOfTarget_is_empty(const std::vector<cv::dnn::Target>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfTarget_capacity(const std::vector<cv::dnn::Target>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfTarget_shrink_to_fit(std::vector<cv::dnn::Target>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfTarget_reserve(std::vector<cv::dnn::Target>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfTarget_remove(std::vector<cv::dnn::Target>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfTarget_swap(std::vector<cv::dnn::Target>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfTarget_clear(std::vector<cv::dnn::Target>* instance) {
		instance->clear();
	}

	void cv_VectorOfTarget_push(std::vector<cv::dnn::Target>* instance, cv::dnn::Target val) {
		instance->push_back(val);
	}

	void cv_VectorOfTarget_insert(std::vector<cv::dnn::Target>* instance, size_t index, cv::dnn::Target val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<cv::dnn::Target> cv_VectorOfTarget_get(const std::vector<cv::dnn::Target>* instance, size_t index) {
		return Ok<cv::dnn::Target>((*instance)[index]);
	}

	void cv_VectorOfTarget_set(std::vector<cv::dnn::Target>* instance, size_t index, cv::dnn::Target val) {
		(*instance)[index] = val;
	}

	const cv::dnn::Target* cv_VectorOfTarget_data(const std::vector<cv::dnn::Target>* instance) {
		return instance->data();
	}
	
	cv::dnn::Target* cv_VectorOfTarget_data_mut(std::vector<cv::dnn::Target>* instance) {
		return instance->data();
	}
	
		std::vector<cv::dnn::Target>* cv_VectorOfTarget_clone(const std::vector<cv::dnn::Target>* instance) {
			return new std::vector<cv::dnn::Target>(*instance);
		}
	
}


extern "C" {
	void cv_VectorOfVectorOfMatShape_delete(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::dnn::MatShape>>* cv_VectorOfVectorOfMatShape_new() {
		return new std::vector<std::vector<cv::dnn::MatShape>>();
	}

	size_t cv_VectorOfVectorOfMatShape_len(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfMatShape_is_empty(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfMatShape_capacity(const std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfMatShape_shrink_to_fit(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfMatShape_reserve(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfMatShape_remove(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfMatShape_swap(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfMatShape_clear(std::vector<std::vector<cv::dnn::MatShape>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfMatShape_push(std::vector<std::vector<cv::dnn::MatShape>>* instance, std::vector<cv::dnn::MatShape>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfMatShape_insert(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, std::vector<cv::dnn::MatShape>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::dnn::MatShape>*> cv_VectorOfVectorOfMatShape_get(const std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index) {
		return Ok<std::vector<cv::dnn::MatShape>*>(new std::vector<cv::dnn::MatShape>((*instance)[index]));
	}

	void cv_VectorOfVectorOfMatShape_set(std::vector<std::vector<cv::dnn::MatShape>>* instance, size_t index, std::vector<cv::dnn::MatShape>* val) {
		(*instance)[index] = *val;
	}

}


