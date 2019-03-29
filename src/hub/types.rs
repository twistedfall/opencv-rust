use libc::{c_void, c_char, size_t};
use crate::{core, types};

#[allow(dead_code)]
pub struct PtrOfAKAZE {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfAKAZE_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAKAZE(ptr:*mut c_void);
}
impl types::PtrOfAKAZE {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfAKAZE(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfAKAZE {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAKAZE(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfAKAZE {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAKAZE {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

impl super::features2d::AKAZE for PtrOfAKAZE {
    #[doc(hidden)] fn as_raw_AKAZE(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfANN_MLP {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfANN_MLP_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfANN_MLP(ptr:*mut c_void);
}
impl types::PtrOfANN_MLP {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfANN_MLP(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfANN_MLP {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfANN_MLP(self.ptr) };
    }
}

impl super::ml::ANN_MLP for PtrOfANN_MLP {
    #[doc(hidden)] fn as_raw_ANN_MLP(&self) -> *mut c_void { 
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfANN_MLP {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfANN_MLP {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfAbsLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfAbsLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAbsLayer(ptr:*mut c_void);
}
impl types::PtrOfAbsLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfAbsLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfAbsLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAbsLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfActivationLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfActivationLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfActivationLayer(ptr:*mut c_void);
}
impl types::PtrOfActivationLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfActivationLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfActivationLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfActivationLayer(self.ptr) };
    }
}

impl super::dnn::Layer for PtrOfActivationLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfActivationLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

impl super::dnn::ActivationLayer for PtrOfActivationLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfAffineTransformer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfAffineTransformer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAffineTransformer(ptr:*mut c_void);
}
impl types::PtrOfAffineTransformer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfAffineTransformer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfAffineTransformer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAffineTransformer(self.ptr) };
    }
}

impl core::Algorithm for PtrOfAffineTransformer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

impl super::shape::AffineTransformer for PtrOfAffineTransformer {
    #[doc(hidden)] fn as_raw_AffineTransformer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

impl super::shape::ShapeTransformer for PtrOfAffineTransformer {
    #[doc(hidden)] fn as_raw_ShapeTransformer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfAgastFeatureDetector {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfAgastFeatureDetector_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAgastFeatureDetector(ptr:*mut c_void);
}
impl types::PtrOfAgastFeatureDetector {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfAgastFeatureDetector(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfAgastFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAgastFeatureDetector(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfAgastFeatureDetector {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAgastFeatureDetector {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

impl super::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
    #[doc(hidden)] fn as_raw_AgastFeatureDetector(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfAlignMTB {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfAlignMTB_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfAlignMTB(ptr:*mut c_void);
}
impl types::PtrOfAlignMTB {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfAlignMTB(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfAlignMTB {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfAlignMTB(self.ptr) };
    }
}

impl super::photo::AlignMTB for PtrOfAlignMTB {
    #[doc(hidden)] fn as_raw_AlignMTB(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAlignMTB {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

impl super::photo::AlignExposures for PtrOfAlignMTB {
    #[doc(hidden)] fn as_raw_AlignExposures(&self) -> *mut c_void { 
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfBFMatcher {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBFMatcher_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBFMatcher(ptr:*mut c_void);
}
impl types::PtrOfBFMatcher {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBFMatcher(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBFMatcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBFMatcher(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfBNLLLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBNLLLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBNLLLayer(ptr:*mut c_void);
}
impl types::PtrOfBNLLLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBNLLLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBNLLLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBNLLLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfBRISK {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBRISK_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBRISK(ptr:*mut c_void);
}
impl types::PtrOfBRISK {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBRISK(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBRISK {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBRISK(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfBackendNode {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackendNode_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackendNode(ptr:*mut c_void);
}
impl types::PtrOfBackendNode {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBackendNode(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBackendNode {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackendNode(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfBackendWrapper {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackendWrapper_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackendWrapper(ptr:*mut c_void);
}
impl types::PtrOfBackendWrapper {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBackendWrapper(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBackendWrapper {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackendWrapper(self.ptr) };
    }
}

impl super::dnn::BackendWrapper for PtrOfBackendWrapper {
    #[doc(hidden)] fn as_raw_BackendWrapper(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackendWrapper_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorKNN_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackgroundSubtractorKNN(ptr:*mut c_void);
}
impl types::PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBackgroundSubtractorKNN {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackgroundSubtractorKNN(self.ptr) };
    }
}

impl super::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

impl super::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)] fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBackgroundSubtractorKNN {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorMOG2_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBackgroundSubtractorMOG2(ptr:*mut c_void);
}
impl types::PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBackgroundSubtractorMOG2 {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBackgroundSubtractorMOG2(self.ptr) };
    }
}

impl super::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

impl super::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)] fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBackgroundSubtractorMOG2 {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfBaseConvolutionLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBaseConvolutionLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBaseConvolutionLayer(ptr:*mut c_void);
}
impl types::PtrOfBaseConvolutionLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBaseConvolutionLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBaseConvolutionLayer(self.ptr) };
    }
}

impl super::dnn::Layer for PtrOfBaseConvolutionLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBaseConvolutionLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

impl super::dnn::BaseConvolutionLayer for PtrOfBaseConvolutionLayer {
    #[doc(hidden)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfBatchNormLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBatchNormLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBatchNormLayer(ptr:*mut c_void);
}
impl types::PtrOfBatchNormLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBatchNormLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBatchNormLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBatchNormLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfBoost {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBoost_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfBoost(ptr:*mut c_void);
}
impl types::PtrOfBoost {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfBoost(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfBoost {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfBoost(self.ptr) };
    }
}

impl super::ml::DTrees for PtrOfBoost {
    #[doc(hidden)] fn as_raw_DTrees(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBoost {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfBoost {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl super::ml::Boost for PtrOfBoost {
    #[doc(hidden)] fn as_raw_Boost(&self) -> *mut c_void { 
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfCLAHE {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCLAHE_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCLAHE(ptr:*mut c_void);
}
impl types::PtrOfCLAHE {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfCLAHE(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfCLAHE {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCLAHE(self.ptr) };
    }
}

impl core::Algorithm for PtrOfCLAHE {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCLAHE_get(self.ptr) }
    }
}

impl super::imgproc::CLAHE for PtrOfCLAHE {
    #[doc(hidden)] fn as_raw_CLAHE(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCLAHE_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfCalibrateDebevec {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCalibrateDebevec_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCalibrateDebevec(ptr:*mut c_void);
}
impl types::PtrOfCalibrateDebevec {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfCalibrateDebevec(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfCalibrateDebevec {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCalibrateDebevec(self.ptr) };
    }
}

impl super::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
    #[doc(hidden)] fn as_raw_CalibrateDebevec(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfCalibrateDebevec {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

impl super::photo::CalibrateCRF for PtrOfCalibrateDebevec {
    #[doc(hidden)] fn as_raw_CalibrateCRF(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfCalibrateRobertson {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCalibrateRobertson_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCalibrateRobertson(ptr:*mut c_void);
}
impl types::PtrOfCalibrateRobertson {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfCalibrateRobertson(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfCalibrateRobertson {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCalibrateRobertson(self.ptr) };
    }
}

impl super::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
    #[doc(hidden)] fn as_raw_CalibrateRobertson(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfCalibrateRobertson {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

impl super::photo::CalibrateCRF for PtrOfCalibrateRobertson {
    #[doc(hidden)] fn as_raw_CalibrateCRF(&self) -> *mut c_void { 
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfConcatLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfConcatLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfConcatLayer(ptr:*mut c_void);
}
impl types::PtrOfConcatLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfConcatLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfConcatLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfConcatLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfConjGradSolver {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfConjGradSolver_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfConjGradSolver(ptr:*mut c_void);
}
impl types::PtrOfConjGradSolver {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfConjGradSolver(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfConjGradSolver {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfConjGradSolver(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfCropLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCropLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfCropLayer(ptr:*mut c_void);
}
impl types::PtrOfCropLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfCropLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfCropLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfCropLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfDTrees {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDTrees_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDTrees(ptr:*mut c_void);
}
impl types::PtrOfDTrees {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfDTrees(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfDTrees {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDTrees(self.ptr) };
    }
}

impl super::ml::DTrees for PtrOfDTrees {
    #[doc(hidden)] fn as_raw_DTrees(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfDTrees {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfDTrees {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfDeblurerBase {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDeblurerBase_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDeblurerBase(ptr:*mut c_void);
}
impl types::PtrOfDeblurerBase {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfDeblurerBase(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfDeblurerBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDeblurerBase(self.ptr) };
    }
}

impl super::videostab::DeblurerBase for PtrOfDeblurerBase {
    #[doc(hidden)] fn as_raw_DeblurerBase(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDeblurerBase_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfDescriptorMatcher {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDescriptorMatcher_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDescriptorMatcher(ptr:*mut c_void);
}
impl types::PtrOfDescriptorMatcher {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfDescriptorMatcher(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfDescriptorMatcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDescriptorMatcher(self.ptr) };
    }
}

impl core::Algorithm for PtrOfDescriptorMatcher {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDescriptorMatcher_get(self.ptr) }
    }
}

impl super::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
    #[doc(hidden)] fn as_raw_DescriptorMatcher(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDescriptorMatcher_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfDetectionOutputLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDetectionOutputLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDetectionOutputLayer(ptr:*mut c_void);
}
impl types::PtrOfDetectionOutputLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfDetectionOutputLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfDetectionOutputLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDetectionOutputLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfDownhillSolver {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDownhillSolver_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDownhillSolver(ptr:*mut c_void);
}
impl types::PtrOfDownhillSolver {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfDownhillSolver(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfDownhillSolver {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDownhillSolver(self.ptr) };
    }
}

impl core::Algorithm for PtrOfDownhillSolver {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

impl core::DownhillSolver for PtrOfDownhillSolver {
    #[doc(hidden)] fn as_raw_DownhillSolver(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

impl core::MinProblemSolver for PtrOfDownhillSolver {
    #[doc(hidden)] fn as_raw_MinProblemSolver(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDualTVL1OpticalFlow_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfDualTVL1OpticalFlow(ptr:*mut c_void);
}
impl types::PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfDualTVL1OpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfDualTVL1OpticalFlow(self.ptr) };
    }
}

impl super::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

impl super::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)] fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfDualTVL1OpticalFlow {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfELULayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfELULayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfELULayer(ptr:*mut c_void);
}
impl types::PtrOfELULayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfELULayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfELULayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfELULayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfEM {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfEM_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfEM(ptr:*mut c_void);
}
impl types::PtrOfEM {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfEM(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfEM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfEM(self.ptr) };
    }
}

impl super::ml::EM for PtrOfEM {
    #[doc(hidden)] fn as_raw_EM(&self) -> *mut c_void { 
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfEM {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfEM {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfEltwiseLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfEltwiseLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfEltwiseLayer(ptr:*mut c_void);
}
impl types::PtrOfEltwiseLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfEltwiseLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfEltwiseLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfEltwiseLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfFarnebackOpticalFlow {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFarnebackOpticalFlow_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFarnebackOpticalFlow(ptr:*mut c_void);
}
impl types::PtrOfFarnebackOpticalFlow {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFarnebackOpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFarnebackOpticalFlow(self.ptr) };
    }
}

impl super::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
    #[doc(hidden)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFarnebackOpticalFlow {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

impl super::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
    #[doc(hidden)] fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfFastFeatureDetector {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFastFeatureDetector_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFastFeatureDetector(ptr:*mut c_void);
}
impl types::PtrOfFastFeatureDetector {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFastFeatureDetector(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFastFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFastFeatureDetector(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfFastFeatureDetector {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFastFeatureDetector {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

impl super::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
    #[doc(hidden)] fn as_raw_FastFeatureDetector(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfFlannBasedMatcher {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFlannBasedMatcher_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFlannBasedMatcher(ptr:*mut c_void);
}
impl types::PtrOfFlannBasedMatcher {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFlannBasedMatcher(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFlannBasedMatcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFlannBasedMatcher(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfFlattenLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFlattenLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFlattenLayer(ptr:*mut c_void);
}
impl types::PtrOfFlattenLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFlattenLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFlattenLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFlattenLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfFormatted {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFormatted_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFormatted(ptr:*mut c_void);
}
impl types::PtrOfFormatted {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFormatted(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFormatted {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFormatted(self.ptr) };
    }
}

impl core::Formatted for PtrOfFormatted {
    #[doc(hidden)] fn as_raw_Formatted(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFormatted_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfFormatter {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFormatter_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFormatter(ptr:*mut c_void);
}
impl types::PtrOfFormatter {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFormatter(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFormatter {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFormatter(self.ptr) };
    }
}

impl core::Formatter for PtrOfFormatter {
    #[doc(hidden)] fn as_raw_Formatter(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFormatter_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfFrameSource {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFrameSource_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFrameSource(ptr:*mut c_void);
}
impl types::PtrOfFrameSource {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFrameSource(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFrameSource {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFrameSource(self.ptr) };
    }
}

impl super::superres::FrameSource for PtrOfFrameSource {
    #[doc(hidden)] fn as_raw_FrameSource(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFrameSource_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfFunction {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFunction_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfFunction(ptr:*mut c_void);
}
impl types::PtrOfFunction {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfFunction(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfFunction {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfFunction(self.ptr) };
    }
}

impl core::MinProblemSolver_Function for PtrOfFunction {
    #[doc(hidden)] fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void { 
        unsafe { cv_PtrOfFunction_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfGFTTDetector {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfGFTTDetector_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfGFTTDetector(ptr:*mut c_void);
}
impl types::PtrOfGFTTDetector {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfGFTTDetector(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfGFTTDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfGFTTDetector(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfGFTTDetector {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

impl super::features2d::GFTTDetector for PtrOfGFTTDetector {
    #[doc(hidden)] fn as_raw_GFTTDetector(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfGFTTDetector {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfGeneralizedHoughBallard {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughBallard_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfGeneralizedHoughBallard(ptr:*mut c_void);
}
impl types::PtrOfGeneralizedHoughBallard {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfGeneralizedHoughBallard {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfGeneralizedHoughBallard(self.ptr) };
    }
}

impl core::Algorithm for PtrOfGeneralizedHoughBallard {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

impl super::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
    #[doc(hidden)] fn as_raw_GeneralizedHough(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

impl super::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
    #[doc(hidden)] fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfGeneralizedHoughGuil {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughGuil_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfGeneralizedHoughGuil(ptr:*mut c_void);
}
impl types::PtrOfGeneralizedHoughGuil {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfGeneralizedHoughGuil {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfGeneralizedHoughGuil(self.ptr) };
    }
}

impl core::Algorithm for PtrOfGeneralizedHoughGuil {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

impl super::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
    #[doc(hidden)] fn as_raw_GeneralizedHough(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

impl super::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
    #[doc(hidden)] fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void { 
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfHausdorffDistanceExtractor_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfHausdorffDistanceExtractor(ptr:*mut c_void);
}
impl types::PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfHausdorffDistanceExtractor {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfHausdorffDistanceExtractor(self.ptr) };
    }
}

impl super::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

impl super::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    #[doc(hidden)] fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfHistogramCostExtractor {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfHistogramCostExtractor_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfHistogramCostExtractor(ptr:*mut c_void);
}
impl types::PtrOfHistogramCostExtractor {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfHistogramCostExtractor(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfHistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfHistogramCostExtractor(self.ptr) };
    }
}

impl core::Algorithm for PtrOfHistogramCostExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfHistogramCostExtractor_get(self.ptr) }
    }
}

impl super::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
    #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfHistogramCostExtractor_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfIFrameSource {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfIFrameSource_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfIFrameSource(ptr:*mut c_void);
}
impl types::PtrOfIFrameSource {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfIFrameSource(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfIFrameSource {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfIFrameSource(self.ptr) };
    }
}

impl super::videostab::IFrameSource for PtrOfIFrameSource {
    #[doc(hidden)] fn as_raw_IFrameSource(&self) -> *mut c_void { 
        unsafe { cv_PtrOfIFrameSource_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfILog {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfILog_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfILog(ptr:*mut c_void);
}
impl types::PtrOfILog {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfILog(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfILog {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfILog(self.ptr) };
    }
}

impl super::videostab::ILog for PtrOfILog {
    #[doc(hidden)] fn as_raw_ILog(&self) -> *mut c_void { 
        unsafe { cv_PtrOfILog_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfIMotionStabilizer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfIMotionStabilizer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfIMotionStabilizer(ptr:*mut c_void);
}
impl types::PtrOfIMotionStabilizer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfIMotionStabilizer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfIMotionStabilizer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfIMotionStabilizer(self.ptr) };
    }
}

impl super::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
    #[doc(hidden)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfIMotionStabilizer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfImageMotionEstimatorBase {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfImageMotionEstimatorBase_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfImageMotionEstimatorBase(ptr:*mut c_void);
}
impl types::PtrOfImageMotionEstimatorBase {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfImageMotionEstimatorBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfImageMotionEstimatorBase(self.ptr) };
    }
}

impl super::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
    #[doc(hidden)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { 
        unsafe { cv_PtrOfImageMotionEstimatorBase_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfInnerProductLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfInnerProductLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfInnerProductLayer(ptr:*mut c_void);
}
impl types::PtrOfInnerProductLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfInnerProductLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfInnerProductLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfInnerProductLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfInpainterBase {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfInpainterBase_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfInpainterBase(ptr:*mut c_void);
}
impl types::PtrOfInpainterBase {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfInpainterBase(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfInpainterBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfInpainterBase(self.ptr) };
    }
}

impl super::videostab::InpainterBase for PtrOfInpainterBase {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { 
        unsafe { cv_PtrOfInpainterBase_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfKAZE {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfKAZE_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfKAZE(ptr:*mut c_void);
}
impl types::PtrOfKAZE {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfKAZE(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfKAZE {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfKAZE(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfKAZE {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

impl super::features2d::KAZE for PtrOfKAZE {
    #[doc(hidden)] fn as_raw_KAZE(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfKAZE {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfKNearest {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfKNearest_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfKNearest(ptr:*mut c_void);
}
impl types::PtrOfKNearest {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfKNearest(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfKNearest {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfKNearest(self.ptr) };
    }
}

impl core::Algorithm for PtrOfKNearest {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfKNearest {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

impl super::ml::KNearest for PtrOfKNearest {
    #[doc(hidden)] fn as_raw_KNearest(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfKernel {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfKernel_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfKernel(ptr:*mut c_void);
}
impl types::PtrOfKernel {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfKernel(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfKernel {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfKernel(self.ptr) };
    }
}

impl super::ml::SVM_Kernel for PtrOfKernel {
    #[doc(hidden)] fn as_raw_SVM_Kernel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKernel_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfKernel {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfKernel_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfLRNLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLRNLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLRNLayer(ptr:*mut c_void);
}
impl types::PtrOfLRNLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfLRNLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfLRNLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLRNLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfLSTMLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLSTMLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLSTMLayer(ptr:*mut c_void);
}
impl types::PtrOfLSTMLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfLSTMLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfLSTMLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLSTMLayer(self.ptr) };
    }
}

impl super::dnn::LSTMLayer for PtrOfLSTMLayer {
    #[doc(hidden)] fn as_raw_LSTMLayer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

impl super::dnn::Layer for PtrOfLSTMLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfLSTMLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfLineSegmentDetector {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLineSegmentDetector_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLineSegmentDetector(ptr:*mut c_void);
}
impl types::PtrOfLineSegmentDetector {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfLineSegmentDetector(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfLineSegmentDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLineSegmentDetector(self.ptr) };
    }
}

impl core::Algorithm for PtrOfLineSegmentDetector {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLineSegmentDetector_get(self.ptr) }
    }
}

impl super::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
    #[doc(hidden)] fn as_raw_LineSegmentDetector(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLineSegmentDetector_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfLogisticRegression {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLogisticRegression_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfLogisticRegression(ptr:*mut c_void);
}
impl types::PtrOfLogisticRegression {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfLogisticRegression(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfLogisticRegression {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfLogisticRegression(self.ptr) };
    }
}

impl core::Algorithm for PtrOfLogisticRegression {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfLogisticRegression {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

impl super::ml::LogisticRegression for PtrOfLogisticRegression {
    #[doc(hidden)] fn as_raw_LogisticRegression(&self) -> *mut c_void { 
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMSER {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMSER_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMSER(ptr:*mut c_void);
}
impl types::PtrOfMSER {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMSER(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMSER {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMSER(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfMSER {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfMSER {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

impl super::features2d::MSER for PtrOfMSER {
    #[doc(hidden)] fn as_raw_MSER(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMVNLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMVNLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMVNLayer(ptr:*mut c_void);
}
impl types::PtrOfMVNLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMVNLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMVNLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMVNLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfMaskGenerator {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMaskGenerator_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMaskGenerator(ptr:*mut c_void);
}
impl types::PtrOfMaskGenerator {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMaskGenerator(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMaskGenerator {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMaskGenerator(self.ptr) };
    }
}

impl super::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfMaskGenerator {
    #[doc(hidden)] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMaskGenerator_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMaxUnpoolLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMaxUnpoolLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMaxUnpoolLayer(ptr:*mut c_void);
}
impl types::PtrOfMaxUnpoolLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMaxUnpoolLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMaxUnpoolLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfMergeDebevec {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeDebevec_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMergeDebevec(ptr:*mut c_void);
}
impl types::PtrOfMergeDebevec {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMergeDebevec(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMergeDebevec {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMergeDebevec(self.ptr) };
    }
}

impl core::Algorithm for PtrOfMergeDebevec {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

impl super::photo::MergeDebevec for PtrOfMergeDebevec {
    #[doc(hidden)] fn as_raw_MergeDebevec(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

impl super::photo::MergeExposures for PtrOfMergeDebevec {
    #[doc(hidden)] fn as_raw_MergeExposures(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMergeMertens {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeMertens_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMergeMertens(ptr:*mut c_void);
}
impl types::PtrOfMergeMertens {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMergeMertens(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMergeMertens {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMergeMertens(self.ptr) };
    }
}

impl core::Algorithm for PtrOfMergeMertens {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

impl super::photo::MergeMertens for PtrOfMergeMertens {
    #[doc(hidden)] fn as_raw_MergeMertens(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

impl super::photo::MergeExposures for PtrOfMergeMertens {
    #[doc(hidden)] fn as_raw_MergeExposures(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMergeRobertson {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeRobertson_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMergeRobertson(ptr:*mut c_void);
}
impl types::PtrOfMergeRobertson {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMergeRobertson(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMergeRobertson {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMergeRobertson(self.ptr) };
    }
}

impl core::Algorithm for PtrOfMergeRobertson {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

impl super::photo::MergeRobertson for PtrOfMergeRobertson {
    #[doc(hidden)] fn as_raw_MergeRobertson(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

impl super::photo::MergeExposures for PtrOfMergeRobertson {
    #[doc(hidden)] fn as_raw_MergeExposures(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMotionEstimatorBase {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMotionEstimatorBase_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMotionEstimatorBase(ptr:*mut c_void);
}
impl types::PtrOfMotionEstimatorBase {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMotionEstimatorBase(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMotionEstimatorBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMotionEstimatorBase(self.ptr) };
    }
}

impl super::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
    #[doc(hidden)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMotionEstimatorBase_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfMotionFilterBase {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMotionFilterBase_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfMotionFilterBase(ptr:*mut c_void);
}
impl types::PtrOfMotionFilterBase {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfMotionFilterBase(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfMotionFilterBase {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfMotionFilterBase(self.ptr) };
    }
}

impl super::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
    #[doc(hidden)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMotionFilterBase_get(self.ptr) }
    }
}

impl super::videostab::MotionFilterBase for PtrOfMotionFilterBase {
    #[doc(hidden)] fn as_raw_MotionFilterBase(&self) -> *mut c_void { 
        unsafe { cv_PtrOfMotionFilterBase_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfNormalBayesClassifier {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfNormalBayesClassifier_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfNormalBayesClassifier(ptr:*mut c_void);
}
impl types::PtrOfNormalBayesClassifier {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfNormalBayesClassifier(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfNormalBayesClassifier {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfNormalBayesClassifier(self.ptr) };
    }
}

impl core::Algorithm for PtrOfNormalBayesClassifier {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfNormalBayesClassifier {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

impl super::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
    #[doc(hidden)] fn as_raw_NormalBayesClassifier(&self) -> *mut c_void { 
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfNormalizeBBoxLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfNormalizeBBoxLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfNormalizeBBoxLayer(ptr:*mut c_void);
}
impl types::PtrOfNormalizeBBoxLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfNormalizeBBoxLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfNormalizeBBoxLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfORB {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfORB_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfORB(ptr:*mut c_void);
}
impl types::PtrOfORB {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfORB(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfORB {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfORB(self.ptr) };
    }
}

impl super::features2d::Feature2D for PtrOfORB {
    #[doc(hidden)] fn as_raw_Feature2D(&self) -> *mut c_void { 
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfORB {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

impl super::features2d::ORB for PtrOfORB {
    #[doc(hidden)] fn as_raw_ORB(&self) -> *mut c_void { 
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfPaddingLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPaddingLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPaddingLayer(ptr:*mut c_void);
}
impl types::PtrOfPaddingLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfPaddingLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfPaddingLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPaddingLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfParamGrid {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfParamGrid_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfParamGrid(ptr:*mut c_void);
}
impl types::PtrOfParamGrid {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfParamGrid(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfParamGrid {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfParamGrid(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfPermuteLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPermuteLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPermuteLayer(ptr:*mut c_void);
}
impl types::PtrOfPermuteLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfPermuteLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfPermuteLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPermuteLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfPoolingLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPoolingLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPoolingLayer(ptr:*mut c_void);
}
impl types::PtrOfPoolingLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfPoolingLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfPoolingLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPoolingLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfPowerLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPowerLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPowerLayer(ptr:*mut c_void);
}
impl types::PtrOfPowerLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfPowerLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfPowerLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPowerLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfPriorBoxLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPriorBoxLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfPriorBoxLayer(ptr:*mut c_void);
}
impl types::PtrOfPriorBoxLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfPriorBoxLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfPriorBoxLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfPriorBoxLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfProposalLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfProposalLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfProposalLayer(ptr:*mut c_void);
}
impl types::PtrOfProposalLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfProposalLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfProposalLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfProposalLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfRNNLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRNNLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfRNNLayer(ptr:*mut c_void);
}
impl types::PtrOfRNNLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfRNNLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfRNNLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfRNNLayer(self.ptr) };
    }
}

impl super::dnn::Layer for PtrOfRNNLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfRNNLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

impl super::dnn::RNNLayer for PtrOfRNNLayer {
    #[doc(hidden)] fn as_raw_RNNLayer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfRTrees {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRTrees_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfRTrees(ptr:*mut c_void);
}
impl types::PtrOfRTrees {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfRTrees(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfRTrees {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfRTrees(self.ptr) };
    }
}

impl super::ml::DTrees for PtrOfRTrees {
    #[doc(hidden)] fn as_raw_DTrees(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfRTrees {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfRTrees {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl super::ml::RTrees for PtrOfRTrees {
    #[doc(hidden)] fn as_raw_RTrees(&self) -> *mut c_void { 
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfReLU6Layer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReLU6Layer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReLU6Layer(ptr:*mut c_void);
}
impl types::PtrOfReLU6Layer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfReLU6Layer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfReLU6Layer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReLU6Layer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfReLULayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReLULayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReLULayer(ptr:*mut c_void);
}
impl types::PtrOfReLULayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfReLULayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfReLULayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReLULayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfRegionLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRegionLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfRegionLayer(ptr:*mut c_void);
}
impl types::PtrOfRegionLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfRegionLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfRegionLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfRegionLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfReorgLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReorgLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReorgLayer(ptr:*mut c_void);
}
impl types::PtrOfReorgLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfReorgLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfReorgLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReorgLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfReshapeLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReshapeLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfReshapeLayer(ptr:*mut c_void);
}
impl types::PtrOfReshapeLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfReshapeLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfReshapeLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfReshapeLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfResizeLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfResizeLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfResizeLayer(ptr:*mut c_void);
}
impl types::PtrOfResizeLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfResizeLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfResizeLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfResizeLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfSVM {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSVM_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSVM(ptr:*mut c_void);
}
impl types::PtrOfSVM {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSVM(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSVM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSVM(self.ptr) };
    }
}

impl super::ml::SVM for PtrOfSVM {
    #[doc(hidden)] fn as_raw_SVM(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSVM {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfSVM {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfSVMSGD {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSVMSGD_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSVMSGD(ptr:*mut c_void);
}
impl types::PtrOfSVMSGD {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSVMSGD(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSVMSGD {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSVMSGD(self.ptr) };
    }
}

impl super::ml::SVMSGD for PtrOfSVMSGD {
    #[doc(hidden)] fn as_raw_SVMSGD(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSVMSGD {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

impl super::ml::StatModel for PtrOfSVMSGD {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfScaleLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfScaleLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfScaleLayer(ptr:*mut c_void);
}
impl types::PtrOfScaleLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfScaleLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfScaleLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfScaleLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfShapeContextDistanceExtractor_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfShapeContextDistanceExtractor(ptr:*mut c_void);
}
impl types::PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfShapeContextDistanceExtractor {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfShapeContextDistanceExtractor(self.ptr) };
    }
}

impl super::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

impl super::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    #[doc(hidden)] fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void { 
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfSigmoidLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSigmoidLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSigmoidLayer(ptr:*mut c_void);
}
impl types::PtrOfSigmoidLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSigmoidLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSigmoidLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSigmoidLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfSimpleBlobDetector {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSimpleBlobDetector_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSimpleBlobDetector(ptr:*mut c_void);
}
impl types::PtrOfSimpleBlobDetector {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSimpleBlobDetector(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSimpleBlobDetector {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSimpleBlobDetector(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfSliceLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSliceLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSliceLayer(ptr:*mut c_void);
}
impl types::PtrOfSliceLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSliceLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSliceLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSliceLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfSoftmaxLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSoftmaxLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSoftmaxLayer(ptr:*mut c_void);
}
impl types::PtrOfSoftmaxLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSoftmaxLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSoftmaxLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSoftmaxLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSparsePyrLKOpticalFlow_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSparsePyrLKOpticalFlow(ptr:*mut c_void);
}
impl types::PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSparsePyrLKOpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSparsePyrLKOpticalFlow(self.ptr) };
    }
}

impl super::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)] fn as_raw_SparseOpticalFlow(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

impl super::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    #[doc(hidden)] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfSplitLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSplitLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSplitLayer(ptr:*mut c_void);
}
impl types::PtrOfSplitLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSplitLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSplitLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSplitLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfStereoBM {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfStereoBM_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfStereoBM(ptr:*mut c_void);
}
impl types::PtrOfStereoBM {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfStereoBM(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfStereoBM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfStereoBM(self.ptr) };
    }
}

impl super::calib3d::StereoMatcher for PtrOfStereoBM {
    #[doc(hidden)] fn as_raw_StereoMatcher(&self) -> *mut c_void { 
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfStereoBM {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

impl super::calib3d::StereoBM for PtrOfStereoBM {
    #[doc(hidden)] fn as_raw_StereoBM(&self) -> *mut c_void { 
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfStereoSGBM {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfStereoSGBM_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfStereoSGBM(ptr:*mut c_void);
}
impl types::PtrOfStereoSGBM {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfStereoSGBM(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfStereoSGBM {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfStereoSGBM(self.ptr) };
    }
}

impl super::calib3d::StereoMatcher for PtrOfStereoSGBM {
    #[doc(hidden)] fn as_raw_StereoMatcher(&self) -> *mut c_void { 
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfStereoSGBM {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

impl super::calib3d::StereoSGBM for PtrOfStereoSGBM {
    #[doc(hidden)] fn as_raw_StereoSGBM(&self) -> *mut c_void { 
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfStitcher {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfStitcher_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfStitcher(ptr:*mut c_void);
}
impl types::PtrOfStitcher {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfStitcher(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfStitcher {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfStitcher(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfSuperResolution {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSuperResolution_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfSuperResolution(ptr:*mut c_void);
}
impl types::PtrOfSuperResolution {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfSuperResolution(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfSuperResolution {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfSuperResolution(self.ptr) };
    }
}

impl super::superres::SuperResolution for PtrOfSuperResolution {
    #[doc(hidden)] fn as_raw_SuperResolution(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSuperResolution {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

impl super::superres::FrameSource for PtrOfSuperResolution {
    #[doc(hidden)] fn as_raw_FrameSource(&self) -> *mut c_void { 
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfTanHLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfTanHLayer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTanHLayer(ptr:*mut c_void);
}
impl types::PtrOfTanHLayer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfTanHLayer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfTanHLayer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTanHLayer(self.ptr) };
    }
}

#[allow(dead_code)]
pub struct PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfThinPlateSplineShapeTransformer_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfThinPlateSplineShapeTransformer(ptr:*mut c_void);
}
impl types::PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfThinPlateSplineShapeTransformer {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfThinPlateSplineShapeTransformer(self.ptr) };
    }
}

impl core::Algorithm for PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

impl super::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

impl super::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    #[doc(hidden)] fn as_raw_ShapeTransformer(&self) -> *mut c_void { 
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfTonemap {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemap_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemap(ptr:*mut c_void);
}
impl types::PtrOfTonemap {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfTonemap(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfTonemap {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemap(self.ptr) };
    }
}

impl core::Algorithm for PtrOfTonemap {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemap_get(self.ptr) }
    }
}

impl super::photo::Tonemap for PtrOfTonemap {
    #[doc(hidden)] fn as_raw_Tonemap(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemap_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfTonemapDrago {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapDrago_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemapDrago(ptr:*mut c_void);
}
impl types::PtrOfTonemapDrago {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfTonemapDrago(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfTonemapDrago {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemapDrago(self.ptr) };
    }
}

impl super::photo::TonemapDrago for PtrOfTonemapDrago {
    #[doc(hidden)] fn as_raw_TonemapDrago(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfTonemapDrago {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

impl super::photo::Tonemap for PtrOfTonemapDrago {
    #[doc(hidden)] fn as_raw_Tonemap(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfTonemapMantiuk {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapMantiuk_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemapMantiuk(ptr:*mut c_void);
}
impl types::PtrOfTonemapMantiuk {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfTonemapMantiuk(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfTonemapMantiuk {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemapMantiuk(self.ptr) };
    }
}

impl core::Algorithm for PtrOfTonemapMantiuk {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

impl super::photo::Tonemap for PtrOfTonemapMantiuk {
    #[doc(hidden)] fn as_raw_Tonemap(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

impl super::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
    #[doc(hidden)] fn as_raw_TonemapMantiuk(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfTonemapReinhard {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapReinhard_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTonemapReinhard(ptr:*mut c_void);
}
impl types::PtrOfTonemapReinhard {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfTonemapReinhard(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfTonemapReinhard {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTonemapReinhard(self.ptr) };
    }
}

impl core::Algorithm for PtrOfTonemapReinhard {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

impl super::photo::Tonemap for PtrOfTonemapReinhard {
    #[doc(hidden)] fn as_raw_Tonemap(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

impl super::photo::TonemapReinhard for PtrOfTonemapReinhard {
    #[doc(hidden)] fn as_raw_TonemapReinhard(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOfTrainData {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfTrainData_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOfTrainData(ptr:*mut c_void);
}
impl types::PtrOfTrainData {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOfTrainData(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOfTrainData {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOfTrainData(self.ptr) };
    }
}

impl super::ml::TrainData for PtrOfTrainData {
    #[doc(hidden)] fn as_raw_TrainData(&self) -> *mut c_void { 
        unsafe { cv_PtrOfTrainData_get(self.ptr) }
    }
}

#[allow(dead_code)]
pub struct PtrOffloat {
    #[doc(hidden)] pub ptr: *mut c_void
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOffloat_get(ptr:*mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_delete_PtrOffloat(ptr:*mut c_void);
}
impl types::PtrOffloat {
    #[doc(hidden)] pub unsafe fn as_raw_PtrOffloat(&self) -> *mut c_void {
        self.ptr
    }
}
impl Drop for types::PtrOffloat {
    fn drop(&mut self) {
        unsafe { cv_delete_PtrOffloat(self.ptr) };
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfDMatch_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfDMatch() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfDMatch(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfDMatch(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfDMatch_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfDMatch_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfDMatch {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfDMatch {
    pub fn new() -> VectorOfDMatch {
        unsafe { return VectorOfDMatch { ptr: cv_new_VectorOfDMatch() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfDMatch_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfDMatch(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfDMatch {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfDMatch(self.ptr) };
    }
}

impl types::VectorOfDMatch {
    pub fn push(&mut self, val: core::DMatch) {
        unsafe { cv_push_VectorOfDMatch(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::DMatch {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfDMatch_get(self.ptr, index)};
        let mut_data: &mut core::DMatch = unsafe { &mut *(raw_pointer as *mut core::DMatch )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfDMatch {
    type Target = [core::DMatch];
    fn deref(&self) -> &[core::DMatch] {
        unsafe {
            let length = cv_VectorOfDMatch_len(self.ptr) as usize;
            let data = cv_VectorOfDMatch_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfDetectionROI_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfDetectionROI() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfDetectionROI(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfDetectionROI(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfDetectionROI_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfDetectionROI_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfDetectionROI {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfDetectionROI {
    pub fn new() -> VectorOfDetectionROI {
        unsafe { return VectorOfDetectionROI { ptr: cv_new_VectorOfDetectionROI() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfDetectionROI_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfDetectionROI(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfDetectionROI {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfDetectionROI(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfDetectionROI {
    pub fn push(&mut self, val: super::objdetect::DetectionROI) {
        unsafe { cv_push_VectorOfDetectionROI(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> super::objdetect::DetectionROI {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfDetectionROI_get(self.ptr, index)};
        return super::objdetect::DetectionROI{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<super::objdetect::DetectionROI> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfExtObject_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfExtObject() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfExtObject(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfExtObject(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfExtObject_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfExtObject_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfExtObject {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfExtObject {
    pub fn new() -> VectorOfExtObject {
        unsafe { return VectorOfExtObject { ptr: cv_new_VectorOfExtObject() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfExtObject_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfExtObject(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfExtObject {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfExtObject(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfExtObject {
    pub fn push(&mut self, val: super::objdetect::DetectionBasedTracker_ExtObject) {
        unsafe { cv_push_VectorOfExtObject(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> super::objdetect::DetectionBasedTracker_ExtObject {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfExtObject_get(self.ptr, index)};
        return super::objdetect::DetectionBasedTracker_ExtObject{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<super::objdetect::DetectionBasedTracker_ExtObject> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfKeyPoint_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfKeyPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfKeyPoint(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfKeyPoint(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfKeyPoint_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfKeyPoint_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfKeyPoint {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfKeyPoint {
    pub fn new() -> VectorOfKeyPoint {
        unsafe { return VectorOfKeyPoint { ptr: cv_new_VectorOfKeyPoint() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfKeyPoint_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfKeyPoint(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfKeyPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfKeyPoint(self.ptr) };
    }
}

impl types::VectorOfKeyPoint {
    pub fn push(&mut self, val: core::KeyPoint) {
        unsafe { cv_push_VectorOfKeyPoint(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::KeyPoint {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfKeyPoint_get(self.ptr, index)};
        let mut_data: &mut core::KeyPoint = unsafe { &mut *(raw_pointer as *mut core::KeyPoint )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfKeyPoint {
    type Target = [core::KeyPoint];
    fn deref(&self) -> &[core::KeyPoint] {
        unsafe {
            let length = cv_VectorOfKeyPoint_len(self.ptr) as usize;
            let data = cv_VectorOfKeyPoint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfMat_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfMat() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfMat(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfMat(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfMat_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfMat_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfMat {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfMat {
    pub fn new() -> VectorOfMat {
        unsafe { return VectorOfMat { ptr: cv_new_VectorOfMat() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfMat_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfMat(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfMat {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfMat(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfMat {
    pub fn push(&mut self, val: core::Mat) {
        unsafe { cv_push_VectorOfMat(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> core::Mat {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfMat_get(self.ptr, index)};
        return core::Mat{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<core::Mat> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfNode_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfNode() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfNode(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfNode(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfNode_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfNode_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfNode {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfNode {
    pub fn new() -> VectorOfNode {
        unsafe { return VectorOfNode { ptr: cv_new_VectorOfNode() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfNode_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfNode(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfNode {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfNode(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfNode {
    pub fn push(&mut self, val: super::ml::DTrees_Node) {
        unsafe { cv_push_VectorOfNode(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> super::ml::DTrees_Node {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfNode_get(self.ptr, index)};
        return super::ml::DTrees_Node{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<super::ml::DTrees_Node> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfPoint_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPoint(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfPoint(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfPoint_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPoint_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfPoint {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfPoint {
    pub fn new() -> VectorOfPoint {
        unsafe { return VectorOfPoint { ptr: cv_new_VectorOfPoint() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfPoint_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfPoint(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPoint(self.ptr) };
    }
}

impl types::VectorOfPoint {
    pub fn push(&mut self, val: core::Point) {
        unsafe { cv_push_VectorOfPoint(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Point {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfPoint_get(self.ptr, index)};
        let mut_data: &mut core::Point = unsafe { &mut *(raw_pointer as *mut core::Point )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfPoint {
    type Target = [core::Point];
    fn deref(&self) -> &[core::Point] {
        unsafe {
            let length = cv_VectorOfPoint_len(self.ptr) as usize;
            let data = cv_VectorOfPoint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfPoint2d_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPoint2d() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPoint2d(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfPoint2d(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfPoint2d_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPoint2d_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfPoint2d {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfPoint2d {
    pub fn new() -> VectorOfPoint2d {
        unsafe { return VectorOfPoint2d { ptr: cv_new_VectorOfPoint2d() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfPoint2d_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfPoint2d(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfPoint2d {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPoint2d(self.ptr) };
    }
}

impl types::VectorOfPoint2d {
    pub fn push(&mut self, val: core::Point2d) {
        unsafe { cv_push_VectorOfPoint2d(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Point2d {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfPoint2d_get(self.ptr, index)};
        let mut_data: &mut core::Point2d = unsafe { &mut *(raw_pointer as *mut core::Point2d )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfPoint2d {
    type Target = [core::Point2d];
    fn deref(&self) -> &[core::Point2d] {
        unsafe {
            let length = cv_VectorOfPoint2d_len(self.ptr) as usize;
            let data = cv_VectorOfPoint2d_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfPoint2f_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPoint2f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPoint2f(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfPoint2f(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfPoint2f_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPoint2f_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfPoint2f {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfPoint2f {
    pub fn new() -> VectorOfPoint2f {
        unsafe { return VectorOfPoint2f { ptr: cv_new_VectorOfPoint2f() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfPoint2f_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfPoint2f(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfPoint2f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPoint2f(self.ptr) };
    }
}

impl types::VectorOfPoint2f {
    pub fn push(&mut self, val: core::Point2f) {
        unsafe { cv_push_VectorOfPoint2f(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Point2f {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfPoint2f_get(self.ptr, index)};
        let mut_data: &mut core::Point2f = unsafe { &mut *(raw_pointer as *mut core::Point2f )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfPoint2f {
    type Target = [core::Point2f];
    fn deref(&self) -> &[core::Point2f] {
        unsafe {
            let length = cv_VectorOfPoint2f_len(self.ptr) as usize;
            let data = cv_VectorOfPoint2f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfPtrOfBackendWrapper() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfPtrOfBackendWrapper(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfPtrOfBackendWrapper(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfPtrOfBackendWrapper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfPtrOfBackendWrapper {
    pub fn new() -> VectorOfPtrOfBackendWrapper {
        unsafe { return VectorOfPtrOfBackendWrapper { ptr: cv_new_VectorOfPtrOfBackendWrapper() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfPtrOfBackendWrapper_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfPtrOfBackendWrapper {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfPtrOfBackendWrapper(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfPtrOfBackendWrapper {
    pub fn push(&mut self, val: types::PtrOfBackendWrapper) {
        unsafe { cv_push_VectorOfPtrOfBackendWrapper(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::PtrOfBackendWrapper {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfPtrOfBackendWrapper_get(self.ptr, index)};
        return types::PtrOfBackendWrapper{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::PtrOfBackendWrapper> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfRange_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRange() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRange(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfRange(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfRange_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRange_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfRange {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfRange {
    pub fn new() -> VectorOfRange {
        unsafe { return VectorOfRange { ptr: cv_new_VectorOfRange() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfRange_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfRange(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfRange {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRange(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfRange {
    pub fn push(&mut self, val: core::Range) {
        unsafe { cv_push_VectorOfRange(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> core::Range {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfRange_get(self.ptr, index)};
        return core::Range{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<core::Range> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfRect_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRect() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRect(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfRect(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfRect_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRect_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfRect {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfRect {
    pub fn new() -> VectorOfRect {
        unsafe { return VectorOfRect { ptr: cv_new_VectorOfRect() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfRect_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfRect(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfRect {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRect(self.ptr) };
    }
}

impl types::VectorOfRect {
    pub fn push(&mut self, val: core::Rect) {
        unsafe { cv_push_VectorOfRect(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Rect {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfRect_get(self.ptr, index)};
        let mut_data: &mut core::Rect = unsafe { &mut *(raw_pointer as *mut core::Rect )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfRect {
    type Target = [core::Rect];
    fn deref(&self) -> &[core::Rect] {
        unsafe {
            let length = cv_VectorOfRect_len(self.ptr) as usize;
            let data = cv_VectorOfRect_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfRect2d_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRect2d() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRect2d(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfRect2d(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfRect2d_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRect2d_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfRect2d {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfRect2d {
    pub fn new() -> VectorOfRect2d {
        unsafe { return VectorOfRect2d { ptr: cv_new_VectorOfRect2d() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfRect2d_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfRect2d(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfRect2d {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRect2d(self.ptr) };
    }
}

impl types::VectorOfRect2d {
    pub fn push(&mut self, val: core::Rect2d) {
        unsafe { cv_push_VectorOfRect2d(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Rect2d {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfRect2d_get(self.ptr, index)};
        let mut_data: &mut core::Rect2d = unsafe { &mut *(raw_pointer as *mut core::Rect2d )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfRect2d {
    type Target = [core::Rect2d];
    fn deref(&self) -> &[core::Rect2d] {
        unsafe {
            let length = cv_VectorOfRect2d_len(self.ptr) as usize;
            let data = cv_VectorOfRect2d_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfRotatedRect_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfRotatedRect() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfRotatedRect(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfRotatedRect(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfRotatedRect_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfRotatedRect_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfRotatedRect {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfRotatedRect {
    pub fn new() -> VectorOfRotatedRect {
        unsafe { return VectorOfRotatedRect { ptr: cv_new_VectorOfRotatedRect() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfRotatedRect_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfRotatedRect(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfRotatedRect {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfRotatedRect(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfRotatedRect {
    pub fn push(&mut self, val: core::RotatedRect) {
        unsafe { cv_push_VectorOfRotatedRect(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> core::RotatedRect {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfRotatedRect_get(self.ptr, index)};
        return core::RotatedRect{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<core::RotatedRect> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfSplit_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfSplit() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfSplit(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfSplit(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfSplit_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfSplit_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfSplit {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfSplit {
    pub fn new() -> VectorOfSplit {
        unsafe { return VectorOfSplit { ptr: cv_new_VectorOfSplit() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfSplit_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfSplit(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfSplit {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfSplit(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfSplit {
    pub fn push(&mut self, val: super::ml::DTrees_Split) {
        unsafe { cv_push_VectorOfSplit(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> super::ml::DTrees_Split {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfSplit_get(self.ptr, index)};
        return super::ml::DTrees_Split{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<super::ml::DTrees_Split> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfString_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfString() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfString(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfString(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfString_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfString_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfString {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfString {
    pub fn new() -> VectorOfString {
        unsafe { return VectorOfString { ptr: cv_new_VectorOfString() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfString_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfString(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfString {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfString(self.ptr) };
    }
}

impl types::VectorOfString {
    pub fn push(&mut self, val: String) {
        unsafe { cv_push_VectorOfString(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut String {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfString_get(self.ptr, index)};
        let mut_data: &mut String = unsafe { &mut *(raw_pointer as *mut String )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfString {
    type Target = [String];
    fn deref(&self) -> &[String] {
        unsafe {
            let length = cv_VectorOfString_len(self.ptr) as usize;
            let data = cv_VectorOfString_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVec4f_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVec4f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVec4f(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVec4f(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVec4f_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVec4f_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVec4f {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVec4f {
    pub fn new() -> VectorOfVec4f {
        unsafe { return VectorOfVec4f { ptr: cv_new_VectorOfVec4f() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVec4f_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVec4f(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVec4f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVec4f(self.ptr) };
    }
}

impl types::VectorOfVec4f {
    pub fn push(&mut self, val: core::Vec4f) {
        unsafe { cv_push_VectorOfVec4f(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Vec4f {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVec4f_get(self.ptr, index)};
        let mut_data: &mut core::Vec4f = unsafe { &mut *(raw_pointer as *mut core::Vec4f )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfVec4f {
    type Target = [core::Vec4f];
    fn deref(&self) -> &[core::Vec4f] {
        unsafe {
            let length = cv_VectorOfVec4f_len(self.ptr) as usize;
            let data = cv_VectorOfVec4f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVec6f_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVec6f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVec6f(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVec6f(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVec6f_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVec6f_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVec6f {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVec6f {
    pub fn new() -> VectorOfVec6f {
        unsafe { return VectorOfVec6f { ptr: cv_new_VectorOfVec6f() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVec6f_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVec6f(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVec6f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVec6f(self.ptr) };
    }
}

impl types::VectorOfVec6f {
    pub fn push(&mut self, val: core::Vec6f) {
        unsafe { cv_push_VectorOfVec6f(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut core::Vec6f {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVec6f_get(self.ptr, index)};
        let mut_data: &mut core::Vec6f = unsafe { &mut *(raw_pointer as *mut core::Vec6f )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfVec6f {
    type Target = [core::Vec6f];
    fn deref(&self) -> &[core::Vec6f] {
        unsafe {
            let length = cv_VectorOfVec6f_len(self.ptr) as usize;
            let data = cv_VectorOfVec6f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfDMatch() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfDMatch(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfDMatch(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfDMatch {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfDMatch {
    pub fn new() -> VectorOfVectorOfDMatch {
        unsafe { return VectorOfVectorOfDMatch { ptr: cv_new_VectorOfVectorOfDMatch() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfDMatch_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfDMatch(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfDMatch {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfDMatch(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfDMatch {
    pub fn push(&mut self, val: types::VectorOfDMatch) {
        unsafe { cv_push_VectorOfVectorOfDMatch(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfDMatch {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfDMatch_get(self.ptr, index)};
        return types::VectorOfDMatch{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfDMatch> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfKeyPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfKeyPoint(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfKeyPoint(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfKeyPoint {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfKeyPoint {
    pub fn new() -> VectorOfVectorOfKeyPoint {
        unsafe { return VectorOfVectorOfKeyPoint { ptr: cv_new_VectorOfVectorOfKeyPoint() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfKeyPoint_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfKeyPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfKeyPoint(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfKeyPoint {
    pub fn push(&mut self, val: types::VectorOfKeyPoint) {
        unsafe { cv_push_VectorOfVectorOfKeyPoint(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfKeyPoint {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfKeyPoint_get(self.ptr, index)};
        return types::VectorOfKeyPoint{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfKeyPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfMat_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfMat() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfMat(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfMat(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfMat {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfMat {
    pub fn new() -> VectorOfVectorOfMat {
        unsafe { return VectorOfVectorOfMat { ptr: cv_new_VectorOfVectorOfMat() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfMat_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfMat(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfMat {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfMat(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfMat {
    pub fn push(&mut self, val: types::VectorOfMat) {
        unsafe { cv_push_VectorOfVectorOfMat(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfMat {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfMat_get(self.ptr, index)};
        return types::VectorOfMat{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfMat> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfPoint_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfPoint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfPoint(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfPoint(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfPoint {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfPoint {
    pub fn new() -> VectorOfVectorOfPoint {
        unsafe { return VectorOfVectorOfPoint { ptr: cv_new_VectorOfVectorOfPoint() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfPoint_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfPoint(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfPoint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfPoint(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfPoint {
    pub fn push(&mut self, val: types::VectorOfPoint) {
        unsafe { cv_push_VectorOfVectorOfPoint(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfPoint {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfPoint_get(self.ptr, index)};
        return types::VectorOfPoint{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfPoint2f() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfPoint2f(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfPoint2f(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfPoint2f {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfPoint2f {
    pub fn new() -> VectorOfVectorOfPoint2f {
        unsafe { return VectorOfVectorOfPoint2f { ptr: cv_new_VectorOfVectorOfPoint2f() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfPoint2f_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfPoint2f(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfPoint2f {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfPoint2f(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfPoint2f {
    pub fn push(&mut self, val: types::VectorOfPoint2f) {
        unsafe { cv_push_VectorOfVectorOfPoint2f(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfPoint2f {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfPoint2f_get(self.ptr, index)};
        return types::VectorOfPoint2f{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfPoint2f> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfRect_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfRect() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfRect(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfRect(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfRect {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfRect {
    pub fn new() -> VectorOfVectorOfRect {
        unsafe { return VectorOfVectorOfRect { ptr: cv_new_VectorOfVectorOfRect() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfRect_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfRect(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfRect {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfRect(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfRect {
    pub fn push(&mut self, val: types::VectorOfRect) {
        unsafe { cv_push_VectorOfVectorOfRect(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfRect {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfRect_get(self.ptr, index)};
        return types::VectorOfRect{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfRect> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfbool_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfbool() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfbool(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfbool(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfbool {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfbool {
    pub fn new() -> VectorOfVectorOfbool {
        unsafe { return VectorOfVectorOfbool { ptr: cv_new_VectorOfVectorOfbool() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfbool_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfbool(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfbool {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfbool(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfbool {
    pub fn push(&mut self, val: types::VectorOfbool) {
        unsafe { cv_push_VectorOfVectorOfbool(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfbool {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfbool_get(self.ptr, index)};
        return types::VectorOfbool{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfbool> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfchar_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfchar(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfchar(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfchar {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfchar {
    pub fn new() -> VectorOfVectorOfchar {
        unsafe { return VectorOfVectorOfchar { ptr: cv_new_VectorOfVectorOfchar() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfchar_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfchar(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfchar(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfchar {
    pub fn push(&mut self, val: types::VectorOfchar) {
        unsafe { cv_push_VectorOfVectorOfchar(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfchar {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfchar_get(self.ptr, index)};
        return types::VectorOfchar{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfchar> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfint_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfint(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfint(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfint_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfint_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfint {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfint {
    pub fn new() -> VectorOfVectorOfint {
        unsafe { return VectorOfVectorOfint { ptr: cv_new_VectorOfVectorOfint() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfint_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfint(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfint(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfint {
    pub fn push(&mut self, val: types::VectorOfint) {
        unsafe { cv_push_VectorOfVectorOfint(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfint {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfint_get(self.ptr, index)};
        return types::VectorOfint{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfVectorOfuchar_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfVectorOfuchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfVectorOfuchar(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfVectorOfuchar(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfVectorOfuchar {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfVectorOfuchar {
    pub fn new() -> VectorOfVectorOfuchar {
        unsafe { return VectorOfVectorOfuchar { ptr: cv_new_VectorOfVectorOfuchar() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfVectorOfuchar_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfVectorOfuchar(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfVectorOfuchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfVectorOfuchar(self.ptr) };
    }
}

// BoxedClassTypeInfo
impl types::VectorOfVectorOfuchar {
    pub fn push(&mut self, val: types::VectorOfuchar) {
        unsafe { cv_push_VectorOfVectorOfuchar(self.ptr, val.ptr) }
    }
    pub fn get(&self, index: i32) -> types::VectorOfuchar {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfVectorOfuchar_get(self.ptr, index)};
        return types::VectorOfuchar{ ptr: raw_pointer}
    }
    pub fn to_vec(&self) -> Vec<types::VectorOfuchar> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfbool() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfbool(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfbool(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfbool_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfbool_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfbool {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfbool {
    pub fn new() -> VectorOfbool {
        unsafe { return VectorOfbool { ptr: cv_new_VectorOfbool() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfbool_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfbool(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfbool {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfbool(self.ptr) };
    }
}

impl types::VectorOfbool {
    pub fn push(&mut self, val: bool) {
        unsafe { cv_push_VectorOfbool(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut bool {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfbool_get(self.ptr, index)};
        let mut_data: &mut bool = unsafe { &mut *(raw_pointer as *mut bool )};
        return mut_data
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfchar_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfchar(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfchar(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfchar_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfchar_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfchar {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfchar {
    pub fn new() -> VectorOfchar {
        unsafe { return VectorOfchar { ptr: cv_new_VectorOfchar() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfchar_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfchar(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfchar(self.ptr) };
    }
}

impl types::VectorOfchar {
    pub fn push(&mut self, val: i8) {
        unsafe { cv_push_VectorOfchar(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut i8 {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfchar_get(self.ptr, index)};
        let mut_data: &mut i8 = unsafe { &mut *(raw_pointer as *mut i8 )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfchar {
    type Target = [i8];
    fn deref(&self) -> &[i8] {
        unsafe {
            let length = cv_VectorOfchar_len(self.ptr) as usize;
            let data = cv_VectorOfchar_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfdouble_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfdouble() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfdouble(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfdouble(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfdouble_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfdouble_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfdouble {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfdouble {
    pub fn new() -> VectorOfdouble {
        unsafe { return VectorOfdouble { ptr: cv_new_VectorOfdouble() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfdouble_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfdouble(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfdouble {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfdouble(self.ptr) };
    }
}

impl types::VectorOfdouble {
    pub fn push(&mut self, val: f64) {
        unsafe { cv_push_VectorOfdouble(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut f64 {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfdouble_get(self.ptr, index)};
        let mut_data: &mut f64 = unsafe { &mut *(raw_pointer as *mut f64 )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfdouble {
    type Target = [f64];
    fn deref(&self) -> &[f64] {
        unsafe {
            let length = cv_VectorOfdouble_len(self.ptr) as usize;
            let data = cv_VectorOfdouble_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOffloat_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOffloat() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOffloat(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOffloat(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOffloat_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOffloat_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOffloat {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOffloat {
    pub fn new() -> VectorOffloat {
        unsafe { return VectorOffloat { ptr: cv_new_VectorOffloat() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOffloat_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOffloat(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOffloat {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOffloat(self.ptr) };
    }
}

impl types::VectorOffloat {
    pub fn push(&mut self, val: f32) {
        unsafe { cv_push_VectorOffloat(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut f32 {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOffloat_get(self.ptr, index)};
        let mut_data: &mut f32 = unsafe { &mut *(raw_pointer as *mut f32 )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOffloat {
    type Target = [f32];
    fn deref(&self) -> &[f32] {
        unsafe {
            let length = cv_VectorOffloat_len(self.ptr) as usize;
            let data = cv_VectorOffloat_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfint_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfint() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfint(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfint(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfint_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfint_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfint {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfint {
    pub fn new() -> VectorOfint {
        unsafe { return VectorOfint { ptr: cv_new_VectorOfint() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfint_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfint(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfint {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfint(self.ptr) };
    }
}

impl types::VectorOfint {
    pub fn push(&mut self, val: i32) {
        unsafe { cv_push_VectorOfint(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut i32 {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfint_get(self.ptr, index)};
        let mut_data: &mut i32 = unsafe { &mut *(raw_pointer as *mut i32 )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfint {
    type Target = [i32];
    fn deref(&self) -> &[i32] {
        unsafe {
            let length = cv_VectorOfint_len(self.ptr) as usize;
            let data = cv_VectorOfint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfsize_t_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfsize_t() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfsize_t(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfsize_t(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfsize_t_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfsize_t_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfsize_t {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfsize_t {
    pub fn new() -> VectorOfsize_t {
        unsafe { return VectorOfsize_t { ptr: cv_new_VectorOfsize_t() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfsize_t_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfsize_t(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfsize_t {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfsize_t(self.ptr) };
    }
}

impl types::VectorOfsize_t {
    pub fn push(&mut self, val: size_t) {
        unsafe { cv_push_VectorOfsize_t(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut size_t {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfsize_t_get(self.ptr, index)};
        let mut_data: &mut size_t = unsafe { &mut *(raw_pointer as *mut size_t )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfsize_t {
    type Target = [size_t];
    fn deref(&self) -> &[size_t] {
        unsafe {
            let length = cv_VectorOfsize_t_len(self.ptr) as usize;
            let data = cv_VectorOfsize_t_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_VectorOfuchar_data(ptr:*mut c_void) -> *mut c_void;
}

extern "C" {
   #[doc(hidden)] fn cv_new_VectorOfuchar() -> *mut c_void;
   #[doc(hidden)] fn cv_delete_VectorOfuchar(ptr:*mut c_void) -> ();
   #[doc(hidden)] fn cv_push_VectorOfuchar(ptr:*mut c_void, ptr2: *const c_void) -> ();
   #[doc(hidden)] fn cv_VectorOfuchar_len(ptr:*mut c_void) -> i32;
   #[doc(hidden)] fn cv_VectorOfuchar_get(ptr:*mut c_void, index: i32) -> *mut c_void;
}
#[allow(dead_code)] pub struct VectorOfuchar {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl types::VectorOfuchar {
    pub fn new() -> VectorOfuchar {
        unsafe { return VectorOfuchar { ptr: cv_new_VectorOfuchar() } };
    }
    pub fn len(&self) -> i32 {
        unsafe { return cv_VectorOfuchar_len(self.ptr); }
    }
    #[doc(hidden)] pub unsafe fn as_raw_VectorOfuchar(&self) -> *mut c_void {
        self.ptr
    }

}
impl Drop for VectorOfuchar {
    fn drop(&mut self) {
        unsafe { cv_delete_VectorOfuchar(self.ptr) };
    }
}

impl types::VectorOfuchar {
    pub fn push(&mut self, val: u8) {
        unsafe { cv_push_VectorOfuchar(self.ptr, &val as *const _ as *const _) }
    }
    pub fn get(&self, index: i32) -> &mut u8 {
        let raw_pointer: *mut c_void = unsafe {cv_VectorOfuchar_get(self.ptr, index)};
        let mut_data: &mut u8 = unsafe { &mut *(raw_pointer as *mut u8 )};
        return mut_data
    }
}

impl ::std::ops::Deref for VectorOfuchar {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            let length = cv_VectorOfuchar_len(self.ptr) as usize;
            let data = cv_VectorOfuchar_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
