use libc::{c_void, c_char, size_t};
use crate::{core, types};

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAKAZE_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAKAZE(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAKAZE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAKAZE {
    #[doc(hidden)] pub fn as_raw_PtrOfAKAZE(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfAKAZE {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAKAZE(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfAKAZE {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAKAZE {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

impl crate::features2d::AKAZE for PtrOfAKAZE {
    #[doc(hidden)]
    fn as_raw_AKAZE(&self) -> *mut c_void {
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfANN_MLP_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfANN_MLP(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfANN_MLP {
    pub(crate) ptr: *mut c_void
}

impl PtrOfANN_MLP {
    #[doc(hidden)] pub fn as_raw_PtrOfANN_MLP(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfANN_MLP {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfANN_MLP(self.ptr) };
    }
}
impl crate::ml::ANN_MLP for PtrOfANN_MLP {
    #[doc(hidden)]
    fn as_raw_ANN_MLP(&self) -> *mut c_void {
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfANN_MLP {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfANN_MLP {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAbsLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAbsLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAbsLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAbsLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfAbsLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfAbsLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAbsLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfActivationLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfActivationLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfActivationLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfActivationLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfActivationLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfActivationLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfActivationLayer(self.ptr) };
    }
}
impl crate::dnn::Layer for PtrOfActivationLayer {
    #[doc(hidden)]
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfActivationLayer {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

impl crate::dnn::ActivationLayer for PtrOfActivationLayer {
    #[doc(hidden)]
    fn as_raw_ActivationLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAffineTransformer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAffineTransformer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAffineTransformer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAffineTransformer {
    #[doc(hidden)] pub fn as_raw_PtrOfAffineTransformer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfAffineTransformer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAffineTransformer(self.ptr) };
    }
}
impl core::Algorithm for PtrOfAffineTransformer {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
    #[doc(hidden)]
    fn as_raw_AffineTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
    #[doc(hidden)]
    fn as_raw_ShapeTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAgastFeatureDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAgastFeatureDetector(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAgastFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAgastFeatureDetector {
    #[doc(hidden)] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfAgastFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAgastFeatureDetector(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfAgastFeatureDetector {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAgastFeatureDetector {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
    #[doc(hidden)]
    fn as_raw_AgastFeatureDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAlignMTB_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAlignMTB(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAlignMTB {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAlignMTB {
    #[doc(hidden)] pub fn as_raw_PtrOfAlignMTB(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfAlignMTB {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAlignMTB(self.ptr) };
    }
}
impl crate::photo::AlignMTB for PtrOfAlignMTB {
    #[doc(hidden)]
    fn as_raw_AlignMTB(&self) -> *mut c_void {
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAlignMTB {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

impl crate::photo::AlignExposures for PtrOfAlignMTB {
    #[doc(hidden)]
    fn as_raw_AlignExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBFMatcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBFMatcher(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBFMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBFMatcher {
    #[doc(hidden)] pub fn as_raw_PtrOfBFMatcher(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBFMatcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBFMatcher(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBNLLLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBNLLLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBNLLLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBNLLLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfBNLLLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBNLLLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBNLLLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBRISK_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBRISK(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBRISK {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBRISK {
    #[doc(hidden)] pub fn as_raw_PtrOfBRISK(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBRISK {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBRISK(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackendNode_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackendNode(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackendNode {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackendNode {
    #[doc(hidden)] pub fn as_raw_PtrOfBackendNode(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBackendNode {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackendNode(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackendWrapper_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackendWrapper(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackendWrapper {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackendWrapper {
    #[doc(hidden)] pub fn as_raw_PtrOfBackendWrapper(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBackendWrapper {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackendWrapper(self.ptr) };
    }
}
impl crate::dnn::BackendWrapper for PtrOfBackendWrapper {
    #[doc(hidden)]
    fn as_raw_BackendWrapper(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackendWrapper_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorKNN_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackgroundSubtractorKNN(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackgroundSubtractorKNN {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBackgroundSubtractorKNN {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackgroundSubtractorKNN(self.ptr) };
    }
}
impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)]
    fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)]
    fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorMOG2_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackgroundSubtractorMOG2(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackgroundSubtractorMOG2 {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBackgroundSubtractorMOG2 {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackgroundSubtractorMOG2(self.ptr) };
    }
}
impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)]
    fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)]
    fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBaseConvolutionLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBaseConvolutionLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBaseConvolutionLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBaseConvolutionLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBaseConvolutionLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBaseConvolutionLayer(self.ptr) };
    }
}
impl crate::dnn::Layer for PtrOfBaseConvolutionLayer {
    #[doc(hidden)]
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBaseConvolutionLayer {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

impl crate::dnn::BaseConvolutionLayer for PtrOfBaseConvolutionLayer {
    #[doc(hidden)]
    fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBatchNormLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBatchNormLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBatchNormLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBatchNormLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfBatchNormLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBatchNormLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBatchNormLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBoost_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBoost(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBoost {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBoost {
    #[doc(hidden)] pub fn as_raw_PtrOfBoost(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfBoost {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBoost(self.ptr) };
    }
}
impl crate::ml::DTrees for PtrOfBoost {
    #[doc(hidden)]
    fn as_raw_DTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBoost {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfBoost {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl crate::ml::Boost for PtrOfBoost {
    #[doc(hidden)]
    fn as_raw_Boost(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfCLAHE_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCLAHE(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCLAHE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCLAHE {
    #[doc(hidden)] pub fn as_raw_PtrOfCLAHE(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfCLAHE {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCLAHE(self.ptr) };
    }
}
impl core::Algorithm for PtrOfCLAHE {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfCLAHE_get(self.ptr) }
    }
}

impl crate::imgproc::CLAHE for PtrOfCLAHE {
    #[doc(hidden)]
    fn as_raw_CLAHE(&self) -> *mut c_void {
        unsafe { cv_PtrOfCLAHE_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfCalibrateDebevec_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCalibrateDebevec(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCalibrateDebevec {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCalibrateDebevec {
    #[doc(hidden)] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfCalibrateDebevec {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCalibrateDebevec(self.ptr) };
    }
}
impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
    #[doc(hidden)]
    fn as_raw_CalibrateDebevec(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfCalibrateDebevec {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
    #[doc(hidden)]
    fn as_raw_CalibrateCRF(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfCalibrateRobertson_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCalibrateRobertson(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCalibrateRobertson {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCalibrateRobertson {
    #[doc(hidden)] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfCalibrateRobertson {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCalibrateRobertson(self.ptr) };
    }
}
impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
    #[doc(hidden)]
    fn as_raw_CalibrateRobertson(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfCalibrateRobertson {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
    #[doc(hidden)]
    fn as_raw_CalibrateCRF(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfConcatLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfConcatLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfConcatLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfConcatLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfConcatLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfConcatLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfConcatLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfConjGradSolver_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfConjGradSolver(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfConjGradSolver {
    pub(crate) ptr: *mut c_void
}

impl PtrOfConjGradSolver {
    #[doc(hidden)] pub fn as_raw_PtrOfConjGradSolver(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfConjGradSolver {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfConjGradSolver(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCropLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCropLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCropLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCropLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfCropLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfCropLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCropLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDTrees_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDTrees(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDTrees {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDTrees {
    #[doc(hidden)] pub fn as_raw_PtrOfDTrees(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfDTrees {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDTrees(self.ptr) };
    }
}
impl crate::ml::DTrees for PtrOfDTrees {
    #[doc(hidden)]
    fn as_raw_DTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfDTrees {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfDTrees {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDeblurerBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDeblurerBase(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDeblurerBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDeblurerBase {
    #[doc(hidden)] pub fn as_raw_PtrOfDeblurerBase(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfDeblurerBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDeblurerBase(self.ptr) };
    }
}
impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
    #[doc(hidden)]
    fn as_raw_DeblurerBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfDeblurerBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDescriptorMatcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDescriptorMatcher(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDescriptorMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDescriptorMatcher {
    #[doc(hidden)] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfDescriptorMatcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDescriptorMatcher(self.ptr) };
    }
}
impl core::Algorithm for PtrOfDescriptorMatcher {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDescriptorMatcher_get(self.ptr) }
    }
}

impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
    #[doc(hidden)]
    fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
        unsafe { cv_PtrOfDescriptorMatcher_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDetectionOutputLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDetectionOutputLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDetectionOutputLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDetectionOutputLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfDetectionOutputLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDetectionOutputLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDownhillSolver_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDownhillSolver(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDownhillSolver {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDownhillSolver {
    #[doc(hidden)] pub fn as_raw_PtrOfDownhillSolver(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfDownhillSolver {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDownhillSolver(self.ptr) };
    }
}
impl core::Algorithm for PtrOfDownhillSolver {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

impl core::DownhillSolver for PtrOfDownhillSolver {
    #[doc(hidden)]
    fn as_raw_DownhillSolver(&self) -> *mut c_void {
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

impl core::MinProblemSolver for PtrOfDownhillSolver {
    #[doc(hidden)]
    fn as_raw_MinProblemSolver(&self) -> *mut c_void {
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDualTVL1OpticalFlow_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDualTVL1OpticalFlow(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDualTVL1OpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfDualTVL1OpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDualTVL1OpticalFlow(self.ptr) };
    }
}
impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)]
    fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)]
    fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfELULayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfELULayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfELULayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfELULayer {
    #[doc(hidden)] pub fn as_raw_PtrOfELULayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfELULayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfELULayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfEM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfEM(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfEM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfEM {
    #[doc(hidden)] pub fn as_raw_PtrOfEM(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfEM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfEM(self.ptr) };
    }
}
impl crate::ml::EM for PtrOfEM {
    #[doc(hidden)]
    fn as_raw_EM(&self) -> *mut c_void {
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfEM {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfEM {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfEltwiseLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfEltwiseLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfEltwiseLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfEltwiseLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfEltwiseLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfEltwiseLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfEltwiseLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFarnebackOpticalFlow_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFarnebackOpticalFlow(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFarnebackOpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFarnebackOpticalFlow {
    #[doc(hidden)] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFarnebackOpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFarnebackOpticalFlow(self.ptr) };
    }
}
impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
    #[doc(hidden)]
    fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFarnebackOpticalFlow {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
    #[doc(hidden)]
    fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFastFeatureDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFastFeatureDetector(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFastFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFastFeatureDetector {
    #[doc(hidden)] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFastFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFastFeatureDetector(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfFastFeatureDetector {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFastFeatureDetector {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
    #[doc(hidden)]
    fn as_raw_FastFeatureDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFeature2D_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFeature2D(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFeature2D {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFeature2D {
    #[doc(hidden)] pub fn as_raw_PtrOfFeature2D(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFeature2D {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFeature2D(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfFeature2D {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfFeature2D_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFeature2D {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFeature2D_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFlannBasedMatcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFlannBasedMatcher(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFlannBasedMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFlannBasedMatcher {
    #[doc(hidden)] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFlannBasedMatcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFlannBasedMatcher(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFlattenLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFlattenLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFlattenLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFlattenLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfFlattenLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFlattenLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFlattenLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFormatted_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFormatted(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFormatted {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFormatted {
    #[doc(hidden)] pub fn as_raw_PtrOfFormatted(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFormatted {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFormatted(self.ptr) };
    }
}
impl core::Formatted for PtrOfFormatted {
    #[doc(hidden)]
    fn as_raw_Formatted(&self) -> *mut c_void {
        unsafe { cv_PtrOfFormatted_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFormatter_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFormatter(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFormatter {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFormatter {
    #[doc(hidden)] pub fn as_raw_PtrOfFormatter(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFormatter {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFormatter(self.ptr) };
    }
}
impl core::Formatter for PtrOfFormatter {
    #[doc(hidden)]
    fn as_raw_Formatter(&self) -> *mut c_void {
        unsafe { cv_PtrOfFormatter_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFrameSource_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFrameSource(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFrameSource {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFrameSource {
    #[doc(hidden)] pub fn as_raw_PtrOfFrameSource(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFrameSource {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFrameSource(self.ptr) };
    }
}
impl crate::superres::FrameSource for PtrOfFrameSource {
    #[doc(hidden)]
    fn as_raw_FrameSource(&self) -> *mut c_void {
        unsafe { cv_PtrOfFrameSource_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFunction_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFunction(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFunction {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFunction {
    #[doc(hidden)] pub fn as_raw_PtrOfFunction(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfFunction {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFunction(self.ptr) };
    }
}
impl core::MinProblemSolver_Function for PtrOfFunction {
    #[doc(hidden)]
    fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void {
        unsafe { cv_PtrOfFunction_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfGFTTDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfGFTTDetector(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfGFTTDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGFTTDetector {
    #[doc(hidden)] pub fn as_raw_PtrOfGFTTDetector(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfGFTTDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfGFTTDetector(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfGFTTDetector {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
    #[doc(hidden)]
    fn as_raw_GFTTDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfGFTTDetector {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughBallard_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfGeneralizedHoughBallard(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfGeneralizedHoughBallard {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGeneralizedHoughBallard {
    #[doc(hidden)] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfGeneralizedHoughBallard {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfGeneralizedHoughBallard(self.ptr) };
    }
}
impl core::Algorithm for PtrOfGeneralizedHoughBallard {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
    #[doc(hidden)]
    fn as_raw_GeneralizedHough(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
    #[doc(hidden)]
    fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughGuil_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfGeneralizedHoughGuil(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfGeneralizedHoughGuil {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGeneralizedHoughGuil {
    #[doc(hidden)] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfGeneralizedHoughGuil {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfGeneralizedHoughGuil(self.ptr) };
    }
}
impl core::Algorithm for PtrOfGeneralizedHoughGuil {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
    #[doc(hidden)]
    fn as_raw_GeneralizedHough(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
    #[doc(hidden)]
    fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfHausdorffDistanceExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfHausdorffDistanceExtractor(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfHausdorffDistanceExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfHausdorffDistanceExtractor {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfHausdorffDistanceExtractor(self.ptr) };
    }
}
impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)]
    fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)]
    fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfHistogramCostExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfHistogramCostExtractor(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfHistogramCostExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHistogramCostExtractor {
    #[doc(hidden)] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfHistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfHistogramCostExtractor(self.ptr) };
    }
}
impl core::Algorithm for PtrOfHistogramCostExtractor {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramCostExtractor_get(self.ptr) }
    }
}

impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
    #[doc(hidden)]
    fn as_raw_HistogramCostExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramCostExtractor_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfIFrameSource_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfIFrameSource(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfIFrameSource {
    pub(crate) ptr: *mut c_void
}

impl PtrOfIFrameSource {
    #[doc(hidden)] pub fn as_raw_PtrOfIFrameSource(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfIFrameSource {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfIFrameSource(self.ptr) };
    }
}
impl crate::videostab::IFrameSource for PtrOfIFrameSource {
    #[doc(hidden)]
    fn as_raw_IFrameSource(&self) -> *mut c_void {
        unsafe { cv_PtrOfIFrameSource_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfILog_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfILog(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfILog {
    pub(crate) ptr: *mut c_void
}

impl PtrOfILog {
    #[doc(hidden)] pub fn as_raw_PtrOfILog(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfILog {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfILog(self.ptr) };
    }
}
impl crate::videostab::ILog for PtrOfILog {
    #[doc(hidden)]
    fn as_raw_ILog(&self) -> *mut c_void {
        unsafe { cv_PtrOfILog_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfIMotionStabilizer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfIMotionStabilizer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfIMotionStabilizer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfIMotionStabilizer {
    #[doc(hidden)] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfIMotionStabilizer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfIMotionStabilizer(self.ptr) };
    }
}
impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
    #[doc(hidden)]
    fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
        unsafe { cv_PtrOfIMotionStabilizer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfImageMotionEstimatorBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfImageMotionEstimatorBase(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfImageMotionEstimatorBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfImageMotionEstimatorBase {
    #[doc(hidden)] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfImageMotionEstimatorBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfImageMotionEstimatorBase(self.ptr) };
    }
}
impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
    #[doc(hidden)]
    fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfImageMotionEstimatorBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfInnerProductLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfInnerProductLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfInnerProductLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfInnerProductLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfInnerProductLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfInnerProductLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfInnerProductLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfInpainterBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfInpainterBase(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfInpainterBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfInpainterBase {
    #[doc(hidden)] pub fn as_raw_PtrOfInpainterBase(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfInpainterBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfInpainterBase(self.ptr) };
    }
}
impl crate::videostab::InpainterBase for PtrOfInpainterBase {
    #[doc(hidden)]
    fn as_raw_InpainterBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfInpainterBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfKAZE_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfKAZE(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfKAZE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKAZE {
    #[doc(hidden)] pub fn as_raw_PtrOfKAZE(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfKAZE {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfKAZE(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfKAZE {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

impl crate::features2d::KAZE for PtrOfKAZE {
    #[doc(hidden)]
    fn as_raw_KAZE(&self) -> *mut c_void {
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfKAZE {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfKNearest_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfKNearest(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfKNearest {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKNearest {
    #[doc(hidden)] pub fn as_raw_PtrOfKNearest(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfKNearest {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfKNearest(self.ptr) };
    }
}
impl core::Algorithm for PtrOfKNearest {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfKNearest {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

impl crate::ml::KNearest for PtrOfKNearest {
    #[doc(hidden)]
    fn as_raw_KNearest(&self) -> *mut c_void {
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfKernel_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfKernel(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfKernel {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKernel {
    #[doc(hidden)] pub fn as_raw_PtrOfKernel(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfKernel {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfKernel(self.ptr) };
    }
}
impl crate::ml::SVM_Kernel for PtrOfKernel {
    #[doc(hidden)]
    fn as_raw_SVM_Kernel(&self) -> *mut c_void {
        unsafe { cv_PtrOfKernel_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfKernel {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfKernel_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfLRNLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLRNLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLRNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLRNLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfLRNLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfLRNLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLRNLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLSTMLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLSTMLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLSTMLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLSTMLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfLSTMLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfLSTMLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLSTMLayer(self.ptr) };
    }
}
impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
    #[doc(hidden)]
    fn as_raw_LSTMLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

impl crate::dnn::Layer for PtrOfLSTMLayer {
    #[doc(hidden)]
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfLSTMLayer {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfLineSegmentDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLineSegmentDetector(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLineSegmentDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLineSegmentDetector {
    #[doc(hidden)] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfLineSegmentDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLineSegmentDetector(self.ptr) };
    }
}
impl core::Algorithm for PtrOfLineSegmentDetector {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfLineSegmentDetector_get(self.ptr) }
    }
}

impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
    #[doc(hidden)]
    fn as_raw_LineSegmentDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfLineSegmentDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfLogisticRegression_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLogisticRegression(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLogisticRegression {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLogisticRegression {
    #[doc(hidden)] pub fn as_raw_PtrOfLogisticRegression(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfLogisticRegression {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLogisticRegression(self.ptr) };
    }
}
impl core::Algorithm for PtrOfLogisticRegression {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfLogisticRegression {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
    #[doc(hidden)]
    fn as_raw_LogisticRegression(&self) -> *mut c_void {
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMSER_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMSER(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMSER {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMSER {
    #[doc(hidden)] pub fn as_raw_PtrOfMSER(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMSER {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMSER(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfMSER {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfMSER {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

impl crate::features2d::MSER for PtrOfMSER {
    #[doc(hidden)]
    fn as_raw_MSER(&self) -> *mut c_void {
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMVNLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMVNLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMVNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMVNLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfMVNLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMVNLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMVNLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMaskGenerator_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMaskGenerator(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMaskGenerator {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMaskGenerator {
    #[doc(hidden)] pub fn as_raw_PtrOfMaskGenerator(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMaskGenerator {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMaskGenerator(self.ptr) };
    }
}
impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfMaskGenerator {
    #[doc(hidden)]
    fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void {
        unsafe { cv_PtrOfMaskGenerator_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMaxUnpoolLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMaxUnpoolLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMaxUnpoolLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMaxUnpoolLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMaxUnpoolLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMaxUnpoolLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeDebevec_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMergeDebevec(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMergeDebevec {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeDebevec {
    #[doc(hidden)] pub fn as_raw_PtrOfMergeDebevec(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMergeDebevec {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMergeDebevec(self.ptr) };
    }
}
impl core::Algorithm for PtrOfMergeDebevec {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
    #[doc(hidden)]
    fn as_raw_MergeDebevec(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

impl crate::photo::MergeExposures for PtrOfMergeDebevec {
    #[doc(hidden)]
    fn as_raw_MergeExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeMertens_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMergeMertens(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMergeMertens {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeMertens {
    #[doc(hidden)] pub fn as_raw_PtrOfMergeMertens(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMergeMertens {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMergeMertens(self.ptr) };
    }
}
impl core::Algorithm for PtrOfMergeMertens {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

impl crate::photo::MergeMertens for PtrOfMergeMertens {
    #[doc(hidden)]
    fn as_raw_MergeMertens(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

impl crate::photo::MergeExposures for PtrOfMergeMertens {
    #[doc(hidden)]
    fn as_raw_MergeExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeRobertson_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMergeRobertson(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMergeRobertson {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeRobertson {
    #[doc(hidden)] pub fn as_raw_PtrOfMergeRobertson(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMergeRobertson {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMergeRobertson(self.ptr) };
    }
}
impl core::Algorithm for PtrOfMergeRobertson {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
    #[doc(hidden)]
    fn as_raw_MergeRobertson(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

impl crate::photo::MergeExposures for PtrOfMergeRobertson {
    #[doc(hidden)]
    fn as_raw_MergeExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMotionEstimatorBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMotionEstimatorBase(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMotionEstimatorBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMotionEstimatorBase {
    #[doc(hidden)] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMotionEstimatorBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMotionEstimatorBase(self.ptr) };
    }
}
impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
    #[doc(hidden)]
    fn as_raw_MotionEstimatorBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfMotionEstimatorBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMotionFilterBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMotionFilterBase(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMotionFilterBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMotionFilterBase {
    #[doc(hidden)] pub fn as_raw_PtrOfMotionFilterBase(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfMotionFilterBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMotionFilterBase(self.ptr) };
    }
}
impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
    #[doc(hidden)]
    fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
        unsafe { cv_PtrOfMotionFilterBase_get(self.ptr) }
    }
}

impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
    #[doc(hidden)]
    fn as_raw_MotionFilterBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfMotionFilterBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfNormalBayesClassifier_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfNormalBayesClassifier(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfNormalBayesClassifier {
    pub(crate) ptr: *mut c_void
}

impl PtrOfNormalBayesClassifier {
    #[doc(hidden)] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfNormalBayesClassifier {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfNormalBayesClassifier(self.ptr) };
    }
}
impl core::Algorithm for PtrOfNormalBayesClassifier {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
    #[doc(hidden)]
    fn as_raw_NormalBayesClassifier(&self) -> *mut c_void {
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfNormalizeBBoxLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfNormalizeBBoxLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfNormalizeBBoxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfNormalizeBBoxLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfNormalizeBBoxLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfNormalizeBBoxLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfORB_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfORB(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfORB {
    pub(crate) ptr: *mut c_void
}

impl PtrOfORB {
    #[doc(hidden)] pub fn as_raw_PtrOfORB(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfORB {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfORB(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfORB {
    #[doc(hidden)]
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfORB {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

impl crate::features2d::ORB for PtrOfORB {
    #[doc(hidden)]
    fn as_raw_ORB(&self) -> *mut c_void {
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfPaddingLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPaddingLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPaddingLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPaddingLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfPaddingLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfPaddingLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPaddingLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfParamGrid_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfParamGrid(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfParamGrid {
    pub(crate) ptr: *mut c_void
}

impl PtrOfParamGrid {
    #[doc(hidden)] pub fn as_raw_PtrOfParamGrid(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfParamGrid {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfParamGrid(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPermuteLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPermuteLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPermuteLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPermuteLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfPermuteLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfPermuteLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPermuteLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPoolingLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPoolingLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPoolingLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPoolingLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfPoolingLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfPoolingLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPoolingLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPowerLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPowerLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPowerLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPowerLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfPowerLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfPowerLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPowerLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPriorBoxLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPriorBoxLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPriorBoxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPriorBoxLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfPriorBoxLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPriorBoxLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfProposalLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfProposalLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfProposalLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfProposalLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfProposalLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfProposalLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfProposalLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRNNLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfRNNLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRNNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRNNLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfRNNLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfRNNLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfRNNLayer(self.ptr) };
    }
}
impl crate::dnn::Layer for PtrOfRNNLayer {
    #[doc(hidden)]
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfRNNLayer {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

impl crate::dnn::RNNLayer for PtrOfRNNLayer {
    #[doc(hidden)]
    fn as_raw_RNNLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfRTrees_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfRTrees(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRTrees {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRTrees {
    #[doc(hidden)] pub fn as_raw_PtrOfRTrees(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfRTrees {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfRTrees(self.ptr) };
    }
}
impl crate::ml::DTrees for PtrOfRTrees {
    #[doc(hidden)]
    fn as_raw_DTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfRTrees {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfRTrees {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl crate::ml::RTrees for PtrOfRTrees {
    #[doc(hidden)]
    fn as_raw_RTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfReLU6Layer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReLU6Layer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReLU6Layer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReLU6Layer {
    #[doc(hidden)] pub fn as_raw_PtrOfReLU6Layer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfReLU6Layer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReLU6Layer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReLULayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReLULayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReLULayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReLULayer {
    #[doc(hidden)] pub fn as_raw_PtrOfReLULayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfReLULayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReLULayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRegionLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfRegionLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRegionLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRegionLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfRegionLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfRegionLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfRegionLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReorgLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReorgLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReorgLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReorgLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfReorgLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfReorgLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReorgLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReshapeLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReshapeLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReshapeLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReshapeLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfReshapeLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfReshapeLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReshapeLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfResizeLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfResizeLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfResizeLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfResizeLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfResizeLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfResizeLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfResizeLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSVM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSVM(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSVM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSVM {
    #[doc(hidden)] pub fn as_raw_PtrOfSVM(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSVM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSVM(self.ptr) };
    }
}
impl crate::ml::SVM for PtrOfSVM {
    #[doc(hidden)]
    fn as_raw_SVM(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSVM {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfSVM {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSVMSGD_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSVMSGD(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSVMSGD {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSVMSGD {
    #[doc(hidden)] pub fn as_raw_PtrOfSVMSGD(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSVMSGD {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSVMSGD(self.ptr) };
    }
}
impl crate::ml::SVMSGD for PtrOfSVMSGD {
    #[doc(hidden)]
    fn as_raw_SVMSGD(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSVMSGD {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfSVMSGD {
    #[doc(hidden)]
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfScaleLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfScaleLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfScaleLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfScaleLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfScaleLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfScaleLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfScaleLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfShapeContextDistanceExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfShapeContextDistanceExtractor(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfShapeContextDistanceExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfShapeContextDistanceExtractor {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfShapeContextDistanceExtractor(self.ptr) };
    }
}
impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)]
    fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)]
    fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSigmoidLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSigmoidLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSigmoidLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSigmoidLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfSigmoidLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSigmoidLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSigmoidLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSimpleBlobDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSimpleBlobDetector(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSimpleBlobDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSimpleBlobDetector {
    #[doc(hidden)] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSimpleBlobDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSimpleBlobDetector(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSliceLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSliceLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSliceLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSliceLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfSliceLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSliceLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSliceLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSoftmaxLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSoftmaxLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSoftmaxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSoftmaxLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSoftmaxLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSoftmaxLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSparsePyrLKOpticalFlow_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSparsePyrLKOpticalFlow(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSparsePyrLKOpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSparsePyrLKOpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSparsePyrLKOpticalFlow(self.ptr) };
    }
}
impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)]
    fn as_raw_SparseOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)]
    fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSplitLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSplitLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSplitLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSplitLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfSplitLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSplitLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSplitLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfStereoBM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfStereoBM(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStereoBM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStereoBM {
    #[doc(hidden)] pub fn as_raw_PtrOfStereoBM(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfStereoBM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfStereoBM(self.ptr) };
    }
}
impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
    #[doc(hidden)]
    fn as_raw_StereoMatcher(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfStereoBM {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

impl crate::calib3d::StereoBM for PtrOfStereoBM {
    #[doc(hidden)]
    fn as_raw_StereoBM(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfStereoSGBM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfStereoSGBM(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStereoSGBM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStereoSGBM {
    #[doc(hidden)] pub fn as_raw_PtrOfStereoSGBM(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfStereoSGBM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfStereoSGBM(self.ptr) };
    }
}
impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
    #[doc(hidden)]
    fn as_raw_StereoMatcher(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfStereoSGBM {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
    #[doc(hidden)]
    fn as_raw_StereoSGBM(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfStitcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfStitcher(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStitcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStitcher {
    #[doc(hidden)] pub fn as_raw_PtrOfStitcher(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfStitcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfStitcher(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSuperResolution_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSuperResolution(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSuperResolution {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSuperResolution {
    #[doc(hidden)] pub fn as_raw_PtrOfSuperResolution(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfSuperResolution {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSuperResolution(self.ptr) };
    }
}
impl crate::superres::SuperResolution for PtrOfSuperResolution {
    #[doc(hidden)]
    fn as_raw_SuperResolution(&self) -> *mut c_void {
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSuperResolution {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

impl crate::superres::FrameSource for PtrOfSuperResolution {
    #[doc(hidden)]
    fn as_raw_FrameSource(&self) -> *mut c_void {
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTanHLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTanHLayer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTanHLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTanHLayer {
    #[doc(hidden)] pub fn as_raw_PtrOfTanHLayer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfTanHLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTanHLayer(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfThinPlateSplineShapeTransformer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfThinPlateSplineShapeTransformer(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfThinPlateSplineShapeTransformer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfThinPlateSplineShapeTransformer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfThinPlateSplineShapeTransformer(self.ptr) };
    }
}
impl core::Algorithm for PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)]
    fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)]
    fn as_raw_ShapeTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemap_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemap(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemap {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemap {
    #[doc(hidden)] pub fn as_raw_PtrOfTonemap(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfTonemap {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemap(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTonemap {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemap_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemap {
    #[doc(hidden)]
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemap_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapDrago_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemapDrago(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemapDrago {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapDrago {
    #[doc(hidden)] pub fn as_raw_PtrOfTonemapDrago(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfTonemapDrago {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemapDrago(self.ptr) };
    }
}
impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
    #[doc(hidden)]
    fn as_raw_TonemapDrago(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfTonemapDrago {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemapDrago {
    #[doc(hidden)]
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapMantiuk_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemapMantiuk(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemapMantiuk {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapMantiuk {
    #[doc(hidden)] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfTonemapMantiuk {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemapMantiuk(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTonemapMantiuk {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
    #[doc(hidden)]
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
    #[doc(hidden)]
    fn as_raw_TonemapMantiuk(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapReinhard_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemapReinhard(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemapReinhard {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapReinhard {
    #[doc(hidden)] pub fn as_raw_PtrOfTonemapReinhard(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfTonemapReinhard {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemapReinhard(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTonemapReinhard {
    #[doc(hidden)]
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemapReinhard {
    #[doc(hidden)]
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
    #[doc(hidden)]
    fn as_raw_TonemapReinhard(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTrainData_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTrainData(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTrainData {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTrainData {
    #[doc(hidden)] pub fn as_raw_PtrOfTrainData(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOfTrainData {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTrainData(self.ptr) };
    }
}
impl crate::ml::TrainData for PtrOfTrainData {
    #[doc(hidden)]
    fn as_raw_TrainData(&self) -> *mut c_void {
        unsafe { cv_PtrOfTrainData_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOffloat_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOffloat(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOffloat {
    pub(crate) ptr: *mut c_void
}

impl PtrOffloat {
    #[doc(hidden)] pub fn as_raw_PtrOffloat(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for PtrOffloat {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOffloat(self.ptr) };
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfDMatch() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfDMatch(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfDMatch(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfDMatch_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfDMatch_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfDMatch {
    pub(crate) ptr: *mut c_void
}

impl VectorOfDMatch {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfDMatch() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfDMatch_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfDMatch(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfDMatch {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfDMatch(self.ptr) };
    }
}
impl VectorOfDMatch {
    pub fn push(&mut self, val: core::DMatch) {
        unsafe { cv_push_VectorOfDMatch(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::DMatch {
        unsafe { (cv_VectorOfDMatch_get(self.ptr, index) as *mut core::DMatch).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfDMatch_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfDMatch {
    type Target = [core::DMatch];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfDMatch_len(self.ptr) as usize;
            let data = cv_VectorOfDMatch_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfDetectionROI() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfDetectionROI(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfDetectionROI(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfDetectionROI_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfDetectionROI_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfDetectionROI {
    pub(crate) ptr: *mut c_void
}

impl VectorOfDetectionROI {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfDetectionROI() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfDetectionROI_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfDetectionROI(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfDetectionROI {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfDetectionROI(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfDetectionROI {
    pub fn push(&mut self, val: crate::objdetect::DetectionROI) {
        unsafe { cv_push_VectorOfDetectionROI(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> crate::objdetect::DetectionROI {
        crate::objdetect::DetectionROI { ptr: unsafe { cv_VectorOfDetectionROI_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::objdetect::DetectionROI> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfExtObject() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfExtObject(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfExtObject(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfExtObject_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfExtObject_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfExtObject {
    pub(crate) ptr: *mut c_void
}

impl VectorOfExtObject {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfExtObject() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfExtObject_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfExtObject(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfExtObject {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfExtObject(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfExtObject {
    pub fn push(&mut self, val: crate::objdetect::DetectionBasedTracker_ExtObject) {
        unsafe { cv_push_VectorOfExtObject(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> crate::objdetect::DetectionBasedTracker_ExtObject {
        crate::objdetect::DetectionBasedTracker_ExtObject { ptr: unsafe { cv_VectorOfExtObject_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::objdetect::DetectionBasedTracker_ExtObject> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfKeyPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfKeyPoint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfKeyPoint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfKeyPoint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfKeyPoint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfKeyPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfKeyPoint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfKeyPoint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfKeyPoint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfKeyPoint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfKeyPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfKeyPoint(self.ptr) };
    }
}
impl VectorOfKeyPoint {
    pub fn push(&mut self, val: core::KeyPoint) {
        unsafe { cv_push_VectorOfKeyPoint(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::KeyPoint {
        unsafe { (cv_VectorOfKeyPoint_get(self.ptr, index) as *mut core::KeyPoint).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfKeyPoint_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfKeyPoint {
    type Target = [core::KeyPoint];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfKeyPoint_len(self.ptr) as usize;
            let data = cv_VectorOfKeyPoint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfMat() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfMat(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfMat(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfMat_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfMat_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfMat {
    pub(crate) ptr: *mut c_void
}

impl VectorOfMat {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfMat() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfMat_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfMat(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfMat {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfMat(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfMat {
    pub fn push(&mut self, val: core::Mat) {
        unsafe { cv_push_VectorOfMat(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> core::Mat {
        core::Mat { ptr: unsafe { cv_VectorOfMat_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<core::Mat> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfNode() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfNode(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfNode(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfNode_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfNode_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfNode {
    pub(crate) ptr: *mut c_void
}

impl VectorOfNode {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfNode() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfNode_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfNode(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfNode {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfNode(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfNode {
    pub fn push(&mut self, val: crate::ml::DTrees_Node) {
        unsafe { cv_push_VectorOfNode(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> crate::ml::DTrees_Node {
        crate::ml::DTrees_Node { ptr: unsafe { cv_VectorOfNode_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::ml::DTrees_Node> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPoint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfPoint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPoint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPoint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPoint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfPoint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfPoint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfPoint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPoint(self.ptr) };
    }
}
impl VectorOfPoint {
    pub fn push(&mut self, val: core::Point) {
        unsafe { cv_push_VectorOfPoint(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Point {
        unsafe { (cv_VectorOfPoint_get(self.ptr, index) as *mut core::Point).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfPoint_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfPoint {
    type Target = [core::Point];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfPoint_len(self.ptr) as usize;
            let data = cv_VectorOfPoint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPoint2d() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPoint2d(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfPoint2d(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPoint2d_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPoint2d_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfPoint2d {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPoint2d {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfPoint2d() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfPoint2d_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfPoint2d(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfPoint2d {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPoint2d(self.ptr) };
    }
}
impl VectorOfPoint2d {
    pub fn push(&mut self, val: core::Point2d) {
        unsafe { cv_push_VectorOfPoint2d(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Point2d {
        unsafe { (cv_VectorOfPoint2d_get(self.ptr, index) as *mut core::Point2d).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfPoint2d_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfPoint2d {
    type Target = [core::Point2d];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfPoint2d_len(self.ptr) as usize;
            let data = cv_VectorOfPoint2d_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPoint2f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPoint2f(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfPoint2f(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPoint2f_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPoint2f_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfPoint2f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPoint2f {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfPoint2f() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfPoint2f_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfPoint2f(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfPoint2f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPoint2f(self.ptr) };
    }
}
impl VectorOfPoint2f {
    pub fn push(&mut self, val: core::Point2f) {
        unsafe { cv_push_VectorOfPoint2f(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Point2f {
        unsafe { (cv_VectorOfPoint2f_get(self.ptr, index) as *mut core::Point2f).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfPoint2f_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfPoint2f {
    type Target = [core::Point2f];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfPoint2f_len(self.ptr) as usize;
            let data = cv_VectorOfPoint2f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPtrOfBackendWrapper() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPtrOfBackendWrapper(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfPtrOfBackendWrapper(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfPtrOfBackendWrapper {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPtrOfBackendWrapper {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfPtrOfBackendWrapper() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfPtrOfBackendWrapper_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfPtrOfBackendWrapper {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPtrOfBackendWrapper(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfPtrOfBackendWrapper {
    pub fn push(&mut self, val: types::PtrOfBackendWrapper) {
        unsafe { cv_push_VectorOfPtrOfBackendWrapper(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::PtrOfBackendWrapper {
        types::PtrOfBackendWrapper { ptr: unsafe { cv_VectorOfPtrOfBackendWrapper_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::PtrOfBackendWrapper> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRange() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRange(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfRange(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRange_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRange_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfRange {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRange {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfRange() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfRange_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfRange(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfRange {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRange(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfRange {
    pub fn push(&mut self, val: core::Range) {
        unsafe { cv_push_VectorOfRange(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> core::Range {
        core::Range { ptr: unsafe { cv_VectorOfRange_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<core::Range> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRect() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRect(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfRect(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRect_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRect_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfRect {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRect {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfRect() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfRect_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfRect(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfRect {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRect(self.ptr) };
    }
}
impl VectorOfRect {
    pub fn push(&mut self, val: core::Rect) {
        unsafe { cv_push_VectorOfRect(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Rect {
        unsafe { (cv_VectorOfRect_get(self.ptr, index) as *mut core::Rect).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfRect_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfRect {
    type Target = [core::Rect];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfRect_len(self.ptr) as usize;
            let data = cv_VectorOfRect_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRect2d() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRect2d(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfRect2d(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRect2d_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRect2d_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfRect2d {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRect2d {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfRect2d() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfRect2d_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfRect2d(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfRect2d {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRect2d(self.ptr) };
    }
}
impl VectorOfRect2d {
    pub fn push(&mut self, val: core::Rect2d) {
        unsafe { cv_push_VectorOfRect2d(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Rect2d {
        unsafe { (cv_VectorOfRect2d_get(self.ptr, index) as *mut core::Rect2d).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfRect2d_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfRect2d {
    type Target = [core::Rect2d];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfRect2d_len(self.ptr) as usize;
            let data = cv_VectorOfRect2d_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRotatedRect() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRotatedRect(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfRotatedRect(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRotatedRect_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRotatedRect_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfRotatedRect {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRotatedRect {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfRotatedRect() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfRotatedRect_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfRotatedRect(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfRotatedRect {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRotatedRect(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfRotatedRect {
    pub fn push(&mut self, val: core::RotatedRect) {
        unsafe { cv_push_VectorOfRotatedRect(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> core::RotatedRect {
        core::RotatedRect { ptr: unsafe { cv_VectorOfRotatedRect_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<core::RotatedRect> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfSplit() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfSplit(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfSplit(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfSplit_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfSplit_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfSplit {
    pub(crate) ptr: *mut c_void
}

impl VectorOfSplit {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfSplit() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfSplit_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfSplit(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfSplit {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfSplit(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfSplit {
    pub fn push(&mut self, val: crate::ml::DTrees_Split) {
        unsafe { cv_push_VectorOfSplit(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> crate::ml::DTrees_Split {
        crate::ml::DTrees_Split { ptr: unsafe { cv_VectorOfSplit_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::ml::DTrees_Split> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfString() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfString(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfString(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfString_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfString_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfString {
    pub(crate) ptr: *mut c_void
}

impl VectorOfString {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfString() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfString_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfString(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfString {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfString(self.ptr) };
    }
}
impl VectorOfString {
    pub fn push(&mut self, val: String) {
        unsafe { cv_push_VectorOfString(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut String {
        unsafe { (cv_VectorOfString_get(self.ptr, index) as *mut String).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfString_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfString {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfString_len(self.ptr) as usize;
            let data = cv_VectorOfString_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVec4f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVec4f(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVec4f(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVec4f_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVec4f_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVec4f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVec4f {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVec4f() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVec4f_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVec4f(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVec4f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVec4f(self.ptr) };
    }
}
impl VectorOfVec4f {
    pub fn push(&mut self, val: core::Vec4f) {
        unsafe { cv_push_VectorOfVec4f(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Vec4f {
        unsafe { (cv_VectorOfVec4f_get(self.ptr, index) as *mut core::Vec4f).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfVec4f_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfVec4f {
    type Target = [core::Vec4f];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfVec4f_len(self.ptr) as usize;
            let data = cv_VectorOfVec4f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVec6f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVec6f(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVec6f(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVec6f_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVec6f_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVec6f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVec6f {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVec6f() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVec6f_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVec6f(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVec6f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVec6f(self.ptr) };
    }
}
impl VectorOfVec6f {
    pub fn push(&mut self, val: core::Vec6f) {
        unsafe { cv_push_VectorOfVec6f(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut core::Vec6f {
        unsafe { (cv_VectorOfVec6f_get(self.ptr, index) as *mut core::Vec6f).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfVec6f_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfVec6f {
    type Target = [core::Vec6f];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfVec6f_len(self.ptr) as usize;
            let data = cv_VectorOfVec6f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfDMatch() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfDMatch(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfDMatch(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfDMatch {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfDMatch {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfDMatch() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfDMatch_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfDMatch {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfDMatch(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfDMatch {
    pub fn push(&mut self, val: types::VectorOfDMatch) {
        unsafe { cv_push_VectorOfVectorOfDMatch(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfDMatch {
        types::VectorOfDMatch { ptr: unsafe { cv_VectorOfVectorOfDMatch_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfDMatch> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfKeyPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfKeyPoint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfKeyPoint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfKeyPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfKeyPoint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfKeyPoint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfKeyPoint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfKeyPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfKeyPoint(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfKeyPoint {
    pub fn push(&mut self, val: types::VectorOfKeyPoint) {
        unsafe { cv_push_VectorOfVectorOfKeyPoint(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfKeyPoint {
        types::VectorOfKeyPoint { ptr: unsafe { cv_VectorOfVectorOfKeyPoint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfKeyPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfMat() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfMat(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfMat(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfMat {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfMat {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfMat() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfMat_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfMat(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfMat {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfMat(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfMat {
    pub fn push(&mut self, val: types::VectorOfMat) {
        unsafe { cv_push_VectorOfVectorOfMat(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfMat {
        types::VectorOfMat { ptr: unsafe { cv_VectorOfVectorOfMat_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfMat> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfPoint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfPoint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfPoint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfPoint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfPoint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfPoint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfPoint(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfPoint {
    pub fn push(&mut self, val: types::VectorOfPoint) {
        unsafe { cv_push_VectorOfVectorOfPoint(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfPoint {
        types::VectorOfPoint { ptr: unsafe { cv_VectorOfVectorOfPoint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfPoint2f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfPoint2f(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfPoint2f(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfPoint2f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfPoint2f {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfPoint2f() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfPoint2f_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfPoint2f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfPoint2f(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfPoint2f {
    pub fn push(&mut self, val: types::VectorOfPoint2f) {
        unsafe { cv_push_VectorOfVectorOfPoint2f(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfPoint2f {
        types::VectorOfPoint2f { ptr: unsafe { cv_VectorOfVectorOfPoint2f_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfPoint2f> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfRect() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfRect(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfRect(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfRect {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfRect {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfRect() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfRect_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfRect(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfRect {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfRect(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfRect {
    pub fn push(&mut self, val: types::VectorOfRect) {
        unsafe { cv_push_VectorOfVectorOfRect(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfRect {
        types::VectorOfRect { ptr: unsafe { cv_VectorOfVectorOfRect_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfRect> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfVectorOfint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfVectorOfint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfVectorOfint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfVectorOfint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfVectorOfint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfVectorOfint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfVectorOfint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfVectorOfint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfVectorOfint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfVectorOfint(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfVectorOfint {
    pub fn push(&mut self, val: types::VectorOfVectorOfint) {
        unsafe { cv_push_VectorOfVectorOfVectorOfint(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfVectorOfint {
        types::VectorOfVectorOfint { ptr: unsafe { cv_VectorOfVectorOfVectorOfint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfVectorOfint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfbool() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfbool(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfbool(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfbool {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfbool {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfbool() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfbool_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfbool(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfbool {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfbool(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfbool {
    pub fn push(&mut self, val: types::VectorOfbool) {
        unsafe { cv_push_VectorOfVectorOfbool(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfbool {
        types::VectorOfbool { ptr: unsafe { cv_VectorOfVectorOfbool_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfbool> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfchar(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfchar(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfchar {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfchar() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfchar_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfchar(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfchar(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfchar {
    pub fn push(&mut self, val: types::VectorOfchar) {
        unsafe { cv_push_VectorOfVectorOfchar(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfchar {
        types::VectorOfchar { ptr: unsafe { cv_VectorOfVectorOfchar_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfchar> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfint(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfint {
    pub fn push(&mut self, val: types::VectorOfint) {
        unsafe { cv_push_VectorOfVectorOfint(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfint {
        types::VectorOfint { ptr: unsafe { cv_VectorOfVectorOfint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfuchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfuchar(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfVectorOfuchar(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfVectorOfuchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfuchar {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfVectorOfuchar() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfVectorOfuchar_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfVectorOfuchar(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfVectorOfuchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfuchar(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfuchar {
    pub fn push(&mut self, val: types::VectorOfuchar) {
        unsafe { cv_push_VectorOfVectorOfuchar(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: i32) -> types::VectorOfuchar {
        types::VectorOfuchar { ptr: unsafe { cv_VectorOfVectorOfuchar_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfuchar> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfbool() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfbool(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfbool(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfbool_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfbool_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfbool {
    pub(crate) ptr: *mut c_void
}

impl VectorOfbool {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfbool() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfbool_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfbool(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfbool {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfbool(self.ptr) };
    }
}
impl VectorOfbool {
    pub fn push(&mut self, val: bool) {
        unsafe { cv_push_VectorOfbool(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut bool {
        unsafe { (cv_VectorOfbool_get(self.ptr, index) as *mut bool).as_mut().unwrap() }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfchar(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfchar(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfchar_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfchar_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfchar {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfchar() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfchar_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfchar(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfchar(self.ptr) };
    }
}
impl VectorOfchar {
    pub fn push(&mut self, val: i8) {
        unsafe { cv_push_VectorOfchar(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut i8 {
        unsafe { (cv_VectorOfchar_get(self.ptr, index) as *mut i8).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfchar_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfchar {
    type Target = [i8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfchar_len(self.ptr) as usize;
            let data = cv_VectorOfchar_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfdouble() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfdouble(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfdouble(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfdouble_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfdouble_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfdouble {
    pub(crate) ptr: *mut c_void
}

impl VectorOfdouble {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfdouble() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfdouble_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfdouble(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfdouble {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfdouble(self.ptr) };
    }
}
impl VectorOfdouble {
    pub fn push(&mut self, val: f64) {
        unsafe { cv_push_VectorOfdouble(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut f64 {
        unsafe { (cv_VectorOfdouble_get(self.ptr, index) as *mut f64).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfdouble_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfdouble {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfdouble_len(self.ptr) as usize;
            let data = cv_VectorOfdouble_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOffloat() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOffloat(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOffloat(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOffloat_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOffloat_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOffloat {
    pub(crate) ptr: *mut c_void
}

impl VectorOffloat {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOffloat() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOffloat_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOffloat(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOffloat {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOffloat(self.ptr) };
    }
}
impl VectorOffloat {
    pub fn push(&mut self, val: f32) {
        unsafe { cv_push_VectorOffloat(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut f32 {
        unsafe { (cv_VectorOffloat_get(self.ptr, index) as *mut f32).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOffloat_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOffloat {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOffloat_len(self.ptr) as usize;
            let data = cv_VectorOffloat_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfint(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfint(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfint_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfint_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfint {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfint() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfint_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfint(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfint(self.ptr) };
    }
}
impl VectorOfint {
    pub fn push(&mut self, val: i32) {
        unsafe { cv_push_VectorOfint(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut i32 {
        unsafe { (cv_VectorOfint_get(self.ptr, index) as *mut i32).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfint_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfint {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfint_len(self.ptr) as usize;
            let data = cv_VectorOfint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfsize_t() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfsize_t(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfsize_t(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfsize_t_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfsize_t_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfsize_t {
    pub(crate) ptr: *mut c_void
}

impl VectorOfsize_t {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfsize_t() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfsize_t_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfsize_t(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfsize_t {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfsize_t(self.ptr) };
    }
}
impl VectorOfsize_t {
    pub fn push(&mut self, val: size_t) {
        unsafe { cv_push_VectorOfsize_t(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut size_t {
        unsafe { (cv_VectorOfsize_t_get(self.ptr, index) as *mut size_t).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfsize_t_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfsize_t {
    type Target = [size_t];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfsize_t_len(self.ptr) as usize;
            let data = cv_VectorOfsize_t_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfuchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfuchar(ptr: *mut c_void);
   #[doc(hidden)] fn cv_push_VectorOfuchar(ptr: *mut c_void, ptr2: *const c_void);
   #[doc(hidden)] fn cv_VectorOfuchar_len(ptr: *mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfuchar_get(ptr: *mut c_void, index: i32) -> *mut c_void;
}

#[allow(dead_code)]
pub struct VectorOfuchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfuchar {
    pub fn new() -> Self {
        unsafe { Self { ptr: cv_new_VectorOfuchar() } }
    }

    pub fn len(&self) -> i32 {
        unsafe { cv_VectorOfuchar_len(self.ptr) }
    }

    #[doc(hidden)]
    pub fn as_raw_VectorOfuchar(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for VectorOfuchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfuchar(self.ptr) };
    }
}
impl VectorOfuchar {
    pub fn push(&mut self, val: u8) {
        unsafe { cv_push_VectorOfuchar(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: i32) -> &mut u8 {
        unsafe { (cv_VectorOfuchar_get(self.ptr, index) as *mut u8).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfuchar_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfuchar {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfuchar_len(self.ptr) as usize;
            let data = cv_VectorOfuchar_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
pub use crate::hub::manual::types::*;
