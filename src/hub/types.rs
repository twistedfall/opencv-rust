use libc::{c_void, c_char, size_t};
use crate::{core, types};

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAKAZE_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAKAZE_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAKAZE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAKAZE {
    pub fn as_raw_PtrOfAKAZE(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAKAZE {
            ptr
        }
    }
}

impl Drop for PtrOfAKAZE {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAKAZE_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfAKAZE {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAKAZE {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

impl crate::features2d::AKAZE for PtrOfAKAZE {
    fn as_raw_AKAZE(&self) -> *mut c_void {
        unsafe { cv_PtrOfAKAZE_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfANN_MLP_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfANN_MLP_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfANN_MLP {
    pub(crate) ptr: *mut c_void
}

impl PtrOfANN_MLP {
    pub fn as_raw_PtrOfANN_MLP(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfANN_MLP {
            ptr
        }
    }
}

impl Drop for PtrOfANN_MLP {
    fn drop(&mut self) {
        unsafe { cv_PtrOfANN_MLP_delete(self.ptr) };
    }
}
impl crate::ml::ANN_MLP for PtrOfANN_MLP {
    fn as_raw_ANN_MLP(&self) -> *mut c_void {
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfANN_MLP {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfANN_MLP {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfANN_MLP_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAbsLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAbsLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAbsLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAbsLayer {
    pub fn as_raw_PtrOfAbsLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAbsLayer {
            ptr
        }
    }
}

impl Drop for PtrOfAbsLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAbsLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfActivationLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfActivationLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfActivationLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfActivationLayer {
    pub fn as_raw_PtrOfActivationLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfActivationLayer {
            ptr
        }
    }
}

impl Drop for PtrOfActivationLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfActivationLayer_delete(self.ptr) };
    }
}
impl crate::dnn::Layer for PtrOfActivationLayer {
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfActivationLayer {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

impl crate::dnn::ActivationLayer for PtrOfActivationLayer {
    fn as_raw_ActivationLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfActivationLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAffineFeature2D_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAffineFeature2D_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAffineFeature2D {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAffineFeature2D {
    pub fn as_raw_PtrOfAffineFeature2D(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAffineFeature2D {
            ptr
        }
    }
}

impl Drop for PtrOfAffineFeature2D {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAffineFeature2D_delete(self.ptr) };
    }
}
impl crate::xfeatures2d::AffineFeature2D for PtrOfAffineFeature2D {
    fn as_raw_AffineFeature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineFeature2D_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAffineTransformer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAffineTransformer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAffineTransformer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAffineTransformer {
    pub fn as_raw_PtrOfAffineTransformer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAffineTransformer {
            ptr
        }
    }
}

impl Drop for PtrOfAffineTransformer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAffineTransformer_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfAffineTransformer {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
    fn as_raw_AffineTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
    fn as_raw_ShapeTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfAffineTransformer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAgastFeatureDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAgastFeatureDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAgastFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAgastFeatureDetector {
    pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAgastFeatureDetector {
            ptr
        }
    }
}

impl Drop for PtrOfAgastFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAgastFeatureDetector_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfAgastFeatureDetector {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAgastFeatureDetector {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
    fn as_raw_AgastFeatureDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfAgastFeatureDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAlignMTB_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAlignMTB_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAlignMTB {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAlignMTB {
    pub fn as_raw_PtrOfAlignMTB(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAlignMTB {
            ptr
        }
    }
}

impl Drop for PtrOfAlignMTB {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAlignMTB_delete(self.ptr) };
    }
}
impl crate::photo::AlignMTB for PtrOfAlignMTB {
    fn as_raw_AlignMTB(&self) -> *mut c_void {
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfAlignMTB {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

impl crate::photo::AlignExposures for PtrOfAlignMTB {
    fn as_raw_AlignExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfAlignMTB_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfAverageHash_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfAverageHash_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfAverageHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAverageHash {
    pub fn as_raw_PtrOfAverageHash(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfAverageHash {
            ptr
        }
    }
}

impl Drop for PtrOfAverageHash {
    fn drop(&mut self) {
        unsafe { cv_PtrOfAverageHash_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBFMatcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBFMatcher_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBFMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBFMatcher {
    pub fn as_raw_PtrOfBFMatcher(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBFMatcher {
            ptr
        }
    }
}

impl Drop for PtrOfBFMatcher {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBFMatcher_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBNLLLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBNLLLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBNLLLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBNLLLayer {
    pub fn as_raw_PtrOfBNLLLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBNLLLayer {
            ptr
        }
    }
}

impl Drop for PtrOfBNLLLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBNLLLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBRISK_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBRISK_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBRISK {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBRISK {
    pub fn as_raw_PtrOfBRISK(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBRISK {
            ptr
        }
    }
}

impl Drop for PtrOfBRISK {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBRISK_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackendNode_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBackendNode_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackendNode {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackendNode {
    pub fn as_raw_PtrOfBackendNode(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBackendNode {
            ptr
        }
    }
}

impl Drop for PtrOfBackendNode {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBackendNode_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackendWrapper_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBackendWrapper_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackendWrapper {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackendWrapper {
    pub fn as_raw_PtrOfBackendWrapper(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBackendWrapper {
            ptr
        }
    }
}

impl Drop for PtrOfBackendWrapper {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBackendWrapper_delete(self.ptr) };
    }
}
impl crate::dnn::BackendWrapper for PtrOfBackendWrapper {
    fn as_raw_BackendWrapper(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackendWrapper_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorKNN_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorKNN_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackgroundSubtractorKNN {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackgroundSubtractorKNN {
    pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBackgroundSubtractorKNN {
            ptr
        }
    }
}

impl Drop for PtrOfBackgroundSubtractorKNN {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_delete(self.ptr) };
    }
}
impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
    fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
    fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBackgroundSubtractorKNN {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorKNN_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorMOG2_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBackgroundSubtractorMOG2_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBackgroundSubtractorMOG2 {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackgroundSubtractorMOG2 {
    pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBackgroundSubtractorMOG2 {
            ptr
        }
    }
}

impl Drop for PtrOfBackgroundSubtractorMOG2 {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_delete(self.ptr) };
    }
}
impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
    fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
    fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBackgroundSubtractorMOG2 {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBackgroundSubtractorMOG2_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBaseConvolutionLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBaseConvolutionLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBaseConvolutionLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBaseConvolutionLayer {
    pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBaseConvolutionLayer {
            ptr
        }
    }
}

impl Drop for PtrOfBaseConvolutionLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBaseConvolutionLayer_delete(self.ptr) };
    }
}
impl crate::dnn::Layer for PtrOfBaseConvolutionLayer {
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBaseConvolutionLayer {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

impl crate::dnn::BaseConvolutionLayer for PtrOfBaseConvolutionLayer {
    fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfBaseConvolutionLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBatchNormLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBatchNormLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBatchNormLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBatchNormLayer {
    pub fn as_raw_PtrOfBatchNormLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBatchNormLayer {
            ptr
        }
    }
}

impl Drop for PtrOfBatchNormLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBatchNormLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBlockMeanHash_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBlockMeanHash_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBlockMeanHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBlockMeanHash {
    pub fn as_raw_PtrOfBlockMeanHash(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBlockMeanHash {
            ptr
        }
    }
}

impl Drop for PtrOfBlockMeanHash {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBlockMeanHash_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfBoost_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBoost_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBoost {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBoost {
    pub fn as_raw_PtrOfBoost(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBoost {
            ptr
        }
    }
}

impl Drop for PtrOfBoost {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBoost_delete(self.ptr) };
    }
}
impl crate::ml::DTrees for PtrOfBoost {
    fn as_raw_DTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfBoost {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfBoost {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

impl crate::ml::Boost for PtrOfBoost {
    fn as_raw_Boost(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoost_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBoostDesc_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBoostDesc_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBoostDesc {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBoostDesc {
    pub fn as_raw_PtrOfBoostDesc(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBoostDesc {
            ptr
        }
    }
}

impl Drop for PtrOfBoostDesc {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBoostDesc_delete(self.ptr) };
    }
}
impl crate::xfeatures2d::BoostDesc for PtrOfBoostDesc {
    fn as_raw_BoostDesc(&self) -> *mut c_void {
        unsafe { cv_PtrOfBoostDesc_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfBriefDescriptorExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfBriefDescriptorExtractor_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfBriefDescriptorExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBriefDescriptorExtractor {
    pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfBriefDescriptorExtractor {
            ptr
        }
    }
}

impl Drop for PtrOfBriefDescriptorExtractor {
    fn drop(&mut self) {
        unsafe { cv_PtrOfBriefDescriptorExtractor_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCLAHE_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfCLAHE_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCLAHE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCLAHE {
    pub fn as_raw_PtrOfCLAHE(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfCLAHE {
            ptr
        }
    }
}

impl Drop for PtrOfCLAHE {
    fn drop(&mut self) {
        unsafe { cv_PtrOfCLAHE_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfCLAHE {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfCLAHE_get(self.ptr) }
    }
}

impl crate::imgproc::CLAHE for PtrOfCLAHE {
    fn as_raw_CLAHE(&self) -> *mut c_void {
        unsafe { cv_PtrOfCLAHE_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfCalibrateDebevec_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfCalibrateDebevec_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCalibrateDebevec {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCalibrateDebevec {
    pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfCalibrateDebevec {
            ptr
        }
    }
}

impl Drop for PtrOfCalibrateDebevec {
    fn drop(&mut self) {
        unsafe { cv_PtrOfCalibrateDebevec_delete(self.ptr) };
    }
}
impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
    fn as_raw_CalibrateDebevec(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfCalibrateDebevec {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
    fn as_raw_CalibrateCRF(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateDebevec_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfCalibrateRobertson_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfCalibrateRobertson_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCalibrateRobertson {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCalibrateRobertson {
    pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfCalibrateRobertson {
            ptr
        }
    }
}

impl Drop for PtrOfCalibrateRobertson {
    fn drop(&mut self) {
        unsafe { cv_PtrOfCalibrateRobertson_delete(self.ptr) };
    }
}
impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
    fn as_raw_CalibrateRobertson(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfCalibrateRobertson {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
    fn as_raw_CalibrateCRF(&self) -> *mut c_void {
        unsafe { cv_PtrOfCalibrateRobertson_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfColorMomentHash_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfColorMomentHash_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfColorMomentHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfColorMomentHash {
    pub fn as_raw_PtrOfColorMomentHash(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfColorMomentHash {
            ptr
        }
    }
}

impl Drop for PtrOfColorMomentHash {
    fn drop(&mut self) {
        unsafe { cv_PtrOfColorMomentHash_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfConcatLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfConcatLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfConcatLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfConcatLayer {
    pub fn as_raw_PtrOfConcatLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfConcatLayer {
            ptr
        }
    }
}

impl Drop for PtrOfConcatLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfConcatLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfConjGradSolver_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfConjGradSolver_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfConjGradSolver {
    pub(crate) ptr: *mut c_void
}

impl PtrOfConjGradSolver {
    pub fn as_raw_PtrOfConjGradSolver(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfConjGradSolver {
            ptr
        }
    }
}

impl Drop for PtrOfConjGradSolver {
    fn drop(&mut self) {
        unsafe { cv_PtrOfConjGradSolver_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfCropLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfCropLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfCropLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCropLayer {
    pub fn as_raw_PtrOfCropLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfCropLayer {
            ptr
        }
    }
}

impl Drop for PtrOfCropLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfCropLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDAISY_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDAISY_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDAISY {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDAISY {
    pub fn as_raw_PtrOfDAISY(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDAISY {
            ptr
        }
    }
}

impl Drop for PtrOfDAISY {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDAISY_delete(self.ptr) };
    }
}
impl crate::xfeatures2d::DAISY for PtrOfDAISY {
    fn as_raw_DAISY(&self) -> *mut c_void {
        unsafe { cv_PtrOfDAISY_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDTrees_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDTrees_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDTrees {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDTrees {
    pub fn as_raw_PtrOfDTrees(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDTrees {
            ptr
        }
    }
}

impl Drop for PtrOfDTrees {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDTrees_delete(self.ptr) };
    }
}
impl crate::ml::DTrees for PtrOfDTrees {
    fn as_raw_DTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfDTrees {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfDTrees {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfDTrees_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDeblurerBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDeblurerBase_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDeblurerBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDeblurerBase {
    pub fn as_raw_PtrOfDeblurerBase(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDeblurerBase {
            ptr
        }
    }
}

impl Drop for PtrOfDeblurerBase {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDeblurerBase_delete(self.ptr) };
    }
}
impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
    fn as_raw_DeblurerBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfDeblurerBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDescriptorMatcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDescriptorMatcher_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDescriptorMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDescriptorMatcher {
    pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDescriptorMatcher {
            ptr
        }
    }
}

impl Drop for PtrOfDescriptorMatcher {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDescriptorMatcher_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfDescriptorMatcher {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDescriptorMatcher_get(self.ptr) }
    }
}

impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
    fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
        unsafe { cv_PtrOfDescriptorMatcher_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDetectionOutputLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDetectionOutputLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDetectionOutputLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDetectionOutputLayer {
    pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDetectionOutputLayer {
            ptr
        }
    }
}

impl Drop for PtrOfDetectionOutputLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDetectionOutputLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfDownhillSolver_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDownhillSolver_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDownhillSolver {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDownhillSolver {
    pub fn as_raw_PtrOfDownhillSolver(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDownhillSolver {
            ptr
        }
    }
}

impl Drop for PtrOfDownhillSolver {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDownhillSolver_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfDownhillSolver {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

impl core::DownhillSolver for PtrOfDownhillSolver {
    fn as_raw_DownhillSolver(&self) -> *mut c_void {
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

impl core::MinProblemSolver for PtrOfDownhillSolver {
    fn as_raw_MinProblemSolver(&self) -> *mut c_void {
        unsafe { cv_PtrOfDownhillSolver_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfDualTVL1OpticalFlow_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfDualTVL1OpticalFlow_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfDualTVL1OpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDualTVL1OpticalFlow {
    pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfDualTVL1OpticalFlow {
            ptr
        }
    }
}

impl Drop for PtrOfDualTVL1OpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_delete(self.ptr) };
    }
}
impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
    fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
    fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfDualTVL1OpticalFlow {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfDualTVL1OpticalFlow_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfELULayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfELULayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfELULayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfELULayer {
    pub fn as_raw_PtrOfELULayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfELULayer {
            ptr
        }
    }
}

impl Drop for PtrOfELULayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfELULayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfEM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfEM_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfEM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfEM {
    pub fn as_raw_PtrOfEM(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfEM {
            ptr
        }
    }
}

impl Drop for PtrOfEM {
    fn drop(&mut self) {
        unsafe { cv_PtrOfEM_delete(self.ptr) };
    }
}
impl crate::ml::EM for PtrOfEM {
    fn as_raw_EM(&self) -> *mut c_void {
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfEM {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfEM {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfEM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfEltwiseLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfEltwiseLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfEltwiseLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfEltwiseLayer {
    pub fn as_raw_PtrOfEltwiseLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfEltwiseLayer {
            ptr
        }
    }
}

impl Drop for PtrOfEltwiseLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfEltwiseLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFREAK_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFREAK_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFREAK {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFREAK {
    pub fn as_raw_PtrOfFREAK(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFREAK {
            ptr
        }
    }
}

impl Drop for PtrOfFREAK {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFREAK_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFarnebackOpticalFlow_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFarnebackOpticalFlow_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFarnebackOpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFarnebackOpticalFlow {
    pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFarnebackOpticalFlow {
            ptr
        }
    }
}

impl Drop for PtrOfFarnebackOpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFarnebackOpticalFlow_delete(self.ptr) };
    }
}
impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
    fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFarnebackOpticalFlow {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
    fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfFarnebackOpticalFlow_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFastFeatureDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFastFeatureDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFastFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFastFeatureDetector {
    pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFastFeatureDetector {
            ptr
        }
    }
}

impl Drop for PtrOfFastFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFastFeatureDetector_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfFastFeatureDetector {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFastFeatureDetector {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
    fn as_raw_FastFeatureDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfFastFeatureDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFeature2D_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFeature2D_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFeature2D {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFeature2D {
    pub fn as_raw_PtrOfFeature2D(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFeature2D {
            ptr
        }
    }
}

impl Drop for PtrOfFeature2D {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFeature2D_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfFeature2D {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfFeature2D_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfFeature2D {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFeature2D_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFlannBasedMatcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFlannBasedMatcher_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFlannBasedMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFlannBasedMatcher {
    pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFlannBasedMatcher {
            ptr
        }
    }
}

impl Drop for PtrOfFlannBasedMatcher {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFlannBasedMatcher_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFlattenLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFlattenLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFlattenLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFlattenLayer {
    pub fn as_raw_PtrOfFlattenLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFlattenLayer {
            ptr
        }
    }
}

impl Drop for PtrOfFlattenLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFlattenLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfFormatted_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFormatted_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFormatted {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFormatted {
    pub fn as_raw_PtrOfFormatted(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFormatted {
            ptr
        }
    }
}

impl Drop for PtrOfFormatted {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFormatted_delete(self.ptr) };
    }
}
impl core::Formatted for PtrOfFormatted {
    fn as_raw_Formatted(&self) -> *mut c_void {
        unsafe { cv_PtrOfFormatted_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFormatter_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFormatter_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFormatter {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFormatter {
    pub fn as_raw_PtrOfFormatter(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFormatter {
            ptr
        }
    }
}

impl Drop for PtrOfFormatter {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFormatter_delete(self.ptr) };
    }
}
impl core::Formatter for PtrOfFormatter {
    fn as_raw_Formatter(&self) -> *mut c_void {
        unsafe { cv_PtrOfFormatter_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFrameSource_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFrameSource_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFrameSource {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFrameSource {
    pub fn as_raw_PtrOfFrameSource(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFrameSource {
            ptr
        }
    }
}

impl Drop for PtrOfFrameSource {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFrameSource_delete(self.ptr) };
    }
}
impl crate::superres::FrameSource for PtrOfFrameSource {
    fn as_raw_FrameSource(&self) -> *mut c_void {
        unsafe { cv_PtrOfFrameSource_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFreeType2_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFreeType2_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFreeType2 {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFreeType2 {
    pub fn as_raw_PtrOfFreeType2(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFreeType2 {
            ptr
        }
    }
}

impl Drop for PtrOfFreeType2 {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFreeType2_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfFreeType2 {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfFreeType2_get(self.ptr) }
    }
}

impl crate::freetype::FreeType2 for PtrOfFreeType2 {
    fn as_raw_FreeType2(&self) -> *mut c_void {
        unsafe { cv_PtrOfFreeType2_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfFunction_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfFunction_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfFunction {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFunction {
    pub fn as_raw_PtrOfFunction(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfFunction {
            ptr
        }
    }
}

impl Drop for PtrOfFunction {
    fn drop(&mut self) {
        unsafe { cv_PtrOfFunction_delete(self.ptr) };
    }
}
impl core::MinProblemSolver_Function for PtrOfFunction {
    fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void {
        unsafe { cv_PtrOfFunction_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfGFTTDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfGFTTDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfGFTTDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGFTTDetector {
    pub fn as_raw_PtrOfGFTTDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfGFTTDetector {
            ptr
        }
    }
}

impl Drop for PtrOfGFTTDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfGFTTDetector_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfGFTTDetector {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
    fn as_raw_GFTTDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfGFTTDetector {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfGFTTDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughBallard_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughBallard_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfGeneralizedHoughBallard {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGeneralizedHoughBallard {
    pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfGeneralizedHoughBallard {
            ptr
        }
    }
}

impl Drop for PtrOfGeneralizedHoughBallard {
    fn drop(&mut self) {
        unsafe { cv_PtrOfGeneralizedHoughBallard_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfGeneralizedHoughBallard {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
    fn as_raw_GeneralizedHough(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
    fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughBallard_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughGuil_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfGeneralizedHoughGuil_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfGeneralizedHoughGuil {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGeneralizedHoughGuil {
    pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfGeneralizedHoughGuil {
            ptr
        }
    }
}

impl Drop for PtrOfGeneralizedHoughGuil {
    fn drop(&mut self) {
        unsafe { cv_PtrOfGeneralizedHoughGuil_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfGeneralizedHoughGuil {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
    fn as_raw_GeneralizedHough(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
    fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void {
        unsafe { cv_PtrOfGeneralizedHoughGuil_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfHarrisLaplaceFeatureDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfHarrisLaplaceFeatureDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfHarrisLaplaceFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHarrisLaplaceFeatureDetector {
    pub fn as_raw_PtrOfHarrisLaplaceFeatureDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfHarrisLaplaceFeatureDetector {
            ptr
        }
    }
}

impl Drop for PtrOfHarrisLaplaceFeatureDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfHarrisLaplaceFeatureDetector_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfHausdorffDistanceExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfHausdorffDistanceExtractor_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfHausdorffDistanceExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHausdorffDistanceExtractor {
    pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfHausdorffDistanceExtractor {
            ptr
        }
    }
}

impl Drop for PtrOfHausdorffDistanceExtractor {
    fn drop(&mut self) {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_delete(self.ptr) };
    }
}
impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfHausdorffDistanceExtractor {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfHausdorffDistanceExtractor_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfHistogramCostExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfHistogramCostExtractor_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfHistogramCostExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHistogramCostExtractor {
    pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfHistogramCostExtractor {
            ptr
        }
    }
}

impl Drop for PtrOfHistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { cv_PtrOfHistogramCostExtractor_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfHistogramCostExtractor {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramCostExtractor_get(self.ptr) }
    }
}

impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
    fn as_raw_HistogramCostExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramCostExtractor_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfHistogramPhaseUnwrapping_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfHistogramPhaseUnwrapping_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfHistogramPhaseUnwrapping {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHistogramPhaseUnwrapping {
    pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfHistogramPhaseUnwrapping {
            ptr
        }
    }
}

impl Drop for PtrOfHistogramPhaseUnwrapping {
    fn drop(&mut self) {
        unsafe { cv_PtrOfHistogramPhaseUnwrapping_delete(self.ptr) };
    }
}
impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
    fn as_raw_HistogramPhaseUnwrapping(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramPhaseUnwrapping_get(self.ptr) }
    }
}

impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
    fn as_raw_PhaseUnwrapping(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramPhaseUnwrapping_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfHistogramPhaseUnwrapping {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfHistogramPhaseUnwrapping_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfIFrameSource_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfIFrameSource_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfIFrameSource {
    pub(crate) ptr: *mut c_void
}

impl PtrOfIFrameSource {
    pub fn as_raw_PtrOfIFrameSource(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfIFrameSource {
            ptr
        }
    }
}

impl Drop for PtrOfIFrameSource {
    fn drop(&mut self) {
        unsafe { cv_PtrOfIFrameSource_delete(self.ptr) };
    }
}
impl crate::videostab::IFrameSource for PtrOfIFrameSource {
    fn as_raw_IFrameSource(&self) -> *mut c_void {
        unsafe { cv_PtrOfIFrameSource_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfILog_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfILog_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfILog {
    pub(crate) ptr: *mut c_void
}

impl PtrOfILog {
    pub fn as_raw_PtrOfILog(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfILog {
            ptr
        }
    }
}

impl Drop for PtrOfILog {
    fn drop(&mut self) {
        unsafe { cv_PtrOfILog_delete(self.ptr) };
    }
}
impl crate::videostab::ILog for PtrOfILog {
    fn as_raw_ILog(&self) -> *mut c_void {
        unsafe { cv_PtrOfILog_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfIMotionStabilizer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfIMotionStabilizer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfIMotionStabilizer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfIMotionStabilizer {
    pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfIMotionStabilizer {
            ptr
        }
    }
}

impl Drop for PtrOfIMotionStabilizer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfIMotionStabilizer_delete(self.ptr) };
    }
}
impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
    fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
        unsafe { cv_PtrOfIMotionStabilizer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfImageMotionEstimatorBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfImageMotionEstimatorBase_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfImageMotionEstimatorBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfImageMotionEstimatorBase {
    pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfImageMotionEstimatorBase {
            ptr
        }
    }
}

impl Drop for PtrOfImageMotionEstimatorBase {
    fn drop(&mut self) {
        unsafe { cv_PtrOfImageMotionEstimatorBase_delete(self.ptr) };
    }
}
impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
    fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfImageMotionEstimatorBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfInnerProductLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfInnerProductLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfInnerProductLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfInnerProductLayer {
    pub fn as_raw_PtrOfInnerProductLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfInnerProductLayer {
            ptr
        }
    }
}

impl Drop for PtrOfInnerProductLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfInnerProductLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfInpainterBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfInpainterBase_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfInpainterBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfInpainterBase {
    pub fn as_raw_PtrOfInpainterBase(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfInpainterBase {
            ptr
        }
    }
}

impl Drop for PtrOfInpainterBase {
    fn drop(&mut self) {
        unsafe { cv_PtrOfInpainterBase_delete(self.ptr) };
    }
}
impl crate::videostab::InpainterBase for PtrOfInpainterBase {
    fn as_raw_InpainterBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfInpainterBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfKAZE_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfKAZE_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfKAZE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKAZE {
    pub fn as_raw_PtrOfKAZE(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfKAZE {
            ptr
        }
    }
}

impl Drop for PtrOfKAZE {
    fn drop(&mut self) {
        unsafe { cv_PtrOfKAZE_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfKAZE {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

impl crate::features2d::KAZE for PtrOfKAZE {
    fn as_raw_KAZE(&self) -> *mut c_void {
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfKAZE {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfKAZE_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfKNearest_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfKNearest_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfKNearest {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKNearest {
    pub fn as_raw_PtrOfKNearest(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfKNearest {
            ptr
        }
    }
}

impl Drop for PtrOfKNearest {
    fn drop(&mut self) {
        unsafe { cv_PtrOfKNearest_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfKNearest {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfKNearest {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

impl crate::ml::KNearest for PtrOfKNearest {
    fn as_raw_KNearest(&self) -> *mut c_void {
        unsafe { cv_PtrOfKNearest_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfKernel_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfKernel_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfKernel {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKernel {
    pub fn as_raw_PtrOfKernel(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfKernel {
            ptr
        }
    }
}

impl Drop for PtrOfKernel {
    fn drop(&mut self) {
        unsafe { cv_PtrOfKernel_delete(self.ptr) };
    }
}
impl crate::ml::SVM_Kernel for PtrOfKernel {
    fn as_raw_SVM_Kernel(&self) -> *mut c_void {
        unsafe { cv_PtrOfKernel_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfKernel {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfKernel_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfLATCH_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfLATCH_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLATCH {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLATCH {
    pub fn as_raw_PtrOfLATCH(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfLATCH {
            ptr
        }
    }
}

impl Drop for PtrOfLATCH {
    fn drop(&mut self) {
        unsafe { cv_PtrOfLATCH_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLRNLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfLRNLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLRNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLRNLayer {
    pub fn as_raw_PtrOfLRNLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfLRNLayer {
            ptr
        }
    }
}

impl Drop for PtrOfLRNLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfLRNLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLSTMLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfLSTMLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLSTMLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLSTMLayer {
    pub fn as_raw_PtrOfLSTMLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfLSTMLayer {
            ptr
        }
    }
}

impl Drop for PtrOfLSTMLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfLSTMLayer_delete(self.ptr) };
    }
}
impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
    fn as_raw_LSTMLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

impl crate::dnn::Layer for PtrOfLSTMLayer {
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfLSTMLayer {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfLSTMLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfLUCID_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfLUCID_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLUCID {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLUCID {
    pub fn as_raw_PtrOfLUCID(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfLUCID {
            ptr
        }
    }
}

impl Drop for PtrOfLUCID {
    fn drop(&mut self) {
        unsafe { cv_PtrOfLUCID_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfLineSegmentDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfLineSegmentDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLineSegmentDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLineSegmentDetector {
    pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfLineSegmentDetector {
            ptr
        }
    }
}

impl Drop for PtrOfLineSegmentDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfLineSegmentDetector_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfLineSegmentDetector {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfLineSegmentDetector_get(self.ptr) }
    }
}

impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
    fn as_raw_LineSegmentDetector(&self) -> *mut c_void {
        unsafe { cv_PtrOfLineSegmentDetector_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfLogisticRegression_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfLogisticRegression_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfLogisticRegression {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLogisticRegression {
    pub fn as_raw_PtrOfLogisticRegression(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfLogisticRegression {
            ptr
        }
    }
}

impl Drop for PtrOfLogisticRegression {
    fn drop(&mut self) {
        unsafe { cv_PtrOfLogisticRegression_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfLogisticRegression {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfLogisticRegression {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
    fn as_raw_LogisticRegression(&self) -> *mut c_void {
        unsafe { cv_PtrOfLogisticRegression_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMSDDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMSDDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMSDDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMSDDetector {
    pub fn as_raw_PtrOfMSDDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMSDDetector {
            ptr
        }
    }
}

impl Drop for PtrOfMSDDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMSDDetector_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMSER_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMSER_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMSER {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMSER {
    pub fn as_raw_PtrOfMSER(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMSER {
            ptr
        }
    }
}

impl Drop for PtrOfMSER {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMSER_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfMSER {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfMSER {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

impl crate::features2d::MSER for PtrOfMSER {
    fn as_raw_MSER(&self) -> *mut c_void {
        unsafe { cv_PtrOfMSER_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMVNLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMVNLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMVNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMVNLayer {
    pub fn as_raw_PtrOfMVNLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMVNLayer {
            ptr
        }
    }
}

impl Drop for PtrOfMVNLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMVNLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMarrHildrethHash_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMarrHildrethHash_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMarrHildrethHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMarrHildrethHash {
    pub fn as_raw_PtrOfMarrHildrethHash(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMarrHildrethHash {
            ptr
        }
    }
}

impl Drop for PtrOfMarrHildrethHash {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMarrHildrethHash_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMaskGenerator_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMaskGenerator_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMaskGenerator {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMaskGenerator {
    pub fn as_raw_PtrOfMaskGenerator(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMaskGenerator {
            ptr
        }
    }
}

impl Drop for PtrOfMaskGenerator {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMaskGenerator_delete(self.ptr) };
    }
}
impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfMaskGenerator {
    fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void {
        unsafe { cv_PtrOfMaskGenerator_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMaxUnpoolLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMaxUnpoolLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMaxUnpoolLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMaxUnpoolLayer {
    pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMaxUnpoolLayer {
            ptr
        }
    }
}

impl Drop for PtrOfMaxUnpoolLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMaxUnpoolLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeDebevec_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMergeDebevec_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMergeDebevec {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeDebevec {
    pub fn as_raw_PtrOfMergeDebevec(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMergeDebevec {
            ptr
        }
    }
}

impl Drop for PtrOfMergeDebevec {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMergeDebevec_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfMergeDebevec {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
    fn as_raw_MergeDebevec(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

impl crate::photo::MergeExposures for PtrOfMergeDebevec {
    fn as_raw_MergeExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeDebevec_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeMertens_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMergeMertens_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMergeMertens {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeMertens {
    pub fn as_raw_PtrOfMergeMertens(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMergeMertens {
            ptr
        }
    }
}

impl Drop for PtrOfMergeMertens {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMergeMertens_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfMergeMertens {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

impl crate::photo::MergeMertens for PtrOfMergeMertens {
    fn as_raw_MergeMertens(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

impl crate::photo::MergeExposures for PtrOfMergeMertens {
    fn as_raw_MergeExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeMertens_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMergeRobertson_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMergeRobertson_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMergeRobertson {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeRobertson {
    pub fn as_raw_PtrOfMergeRobertson(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMergeRobertson {
            ptr
        }
    }
}

impl Drop for PtrOfMergeRobertson {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMergeRobertson_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfMergeRobertson {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
    fn as_raw_MergeRobertson(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

impl crate::photo::MergeExposures for PtrOfMergeRobertson {
    fn as_raw_MergeExposures(&self) -> *mut c_void {
        unsafe { cv_PtrOfMergeRobertson_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMotionEstimatorBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMotionEstimatorBase_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMotionEstimatorBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMotionEstimatorBase {
    pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMotionEstimatorBase {
            ptr
        }
    }
}

impl Drop for PtrOfMotionEstimatorBase {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMotionEstimatorBase_delete(self.ptr) };
    }
}
impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
    fn as_raw_MotionEstimatorBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfMotionEstimatorBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfMotionFilterBase_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfMotionFilterBase_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfMotionFilterBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMotionFilterBase {
    pub fn as_raw_PtrOfMotionFilterBase(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfMotionFilterBase {
            ptr
        }
    }
}

impl Drop for PtrOfMotionFilterBase {
    fn drop(&mut self) {
        unsafe { cv_PtrOfMotionFilterBase_delete(self.ptr) };
    }
}
impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
    fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
        unsafe { cv_PtrOfMotionFilterBase_get(self.ptr) }
    }
}

impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
    fn as_raw_MotionFilterBase(&self) -> *mut c_void {
        unsafe { cv_PtrOfMotionFilterBase_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfNormalBayesClassifier_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfNormalBayesClassifier_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfNormalBayesClassifier {
    pub(crate) ptr: *mut c_void
}

impl PtrOfNormalBayesClassifier {
    pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfNormalBayesClassifier {
            ptr
        }
    }
}

impl Drop for PtrOfNormalBayesClassifier {
    fn drop(&mut self) {
        unsafe { cv_PtrOfNormalBayesClassifier_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfNormalBayesClassifier {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
    fn as_raw_NormalBayesClassifier(&self) -> *mut c_void {
        unsafe { cv_PtrOfNormalBayesClassifier_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfNormalizeBBoxLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfNormalizeBBoxLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfNormalizeBBoxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfNormalizeBBoxLayer {
    pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfNormalizeBBoxLayer {
            ptr
        }
    }
}

impl Drop for PtrOfNormalizeBBoxLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfNormalizeBBoxLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfORB_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfORB_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfORB {
    pub(crate) ptr: *mut c_void
}

impl PtrOfORB {
    pub fn as_raw_PtrOfORB(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfORB {
            ptr
        }
    }
}

impl Drop for PtrOfORB {
    fn drop(&mut self) {
        unsafe { cv_PtrOfORB_delete(self.ptr) };
    }
}
impl crate::features2d::Feature2D for PtrOfORB {
    fn as_raw_Feature2D(&self) -> *mut c_void {
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfORB {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

impl crate::features2d::ORB for PtrOfORB {
    fn as_raw_ORB(&self) -> *mut c_void {
        unsafe { cv_PtrOfORB_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfPCTSignatures_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPCTSignatures_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPCTSignatures {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPCTSignatures {
    pub fn as_raw_PtrOfPCTSignatures(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPCTSignatures {
            ptr
        }
    }
}

impl Drop for PtrOfPCTSignatures {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPCTSignatures_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfPCTSignatures {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfPCTSignatures_get(self.ptr) }
    }
}

impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
    fn as_raw_PCTSignatures(&self) -> *mut c_void {
        unsafe { cv_PtrOfPCTSignatures_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfPCTSignaturesSQFD_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPCTSignaturesSQFD_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPCTSignaturesSQFD {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPCTSignaturesSQFD {
    pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPCTSignaturesSQFD {
            ptr
        }
    }
}

impl Drop for PtrOfPCTSignaturesSQFD {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPCTSignaturesSQFD_delete(self.ptr) };
    }
}
impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
    fn as_raw_PCTSignaturesSQFD(&self) -> *mut c_void {
        unsafe { cv_PtrOfPCTSignaturesSQFD_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfPCTSignaturesSQFD {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfPCTSignaturesSQFD_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfPHash_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPHash_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPHash {
    pub fn as_raw_PtrOfPHash(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPHash {
            ptr
        }
    }
}

impl Drop for PtrOfPHash {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPHash_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPaddingLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPaddingLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPaddingLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPaddingLayer {
    pub fn as_raw_PtrOfPaddingLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPaddingLayer {
            ptr
        }
    }
}

impl Drop for PtrOfPaddingLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPaddingLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfParamGrid_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfParamGrid_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfParamGrid {
    pub(crate) ptr: *mut c_void
}

impl PtrOfParamGrid {
    pub fn as_raw_PtrOfParamGrid(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfParamGrid {
            ptr
        }
    }
}

impl Drop for PtrOfParamGrid {
    fn drop(&mut self) {
        unsafe { cv_PtrOfParamGrid_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPermuteLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPermuteLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPermuteLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPermuteLayer {
    pub fn as_raw_PtrOfPermuteLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPermuteLayer {
            ptr
        }
    }
}

impl Drop for PtrOfPermuteLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPermuteLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPlot2d_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPlot2d_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPlot2d {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPlot2d {
    pub fn as_raw_PtrOfPlot2d(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPlot2d {
            ptr
        }
    }
}

impl Drop for PtrOfPlot2d {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPlot2d_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfPlot2d {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfPlot2d_get(self.ptr) }
    }
}

impl crate::plot::Plot2d for PtrOfPlot2d {
    fn as_raw_Plot2d(&self) -> *mut c_void {
        unsafe { cv_PtrOfPlot2d_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfPoolingLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPoolingLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPoolingLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPoolingLayer {
    pub fn as_raw_PtrOfPoolingLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPoolingLayer {
            ptr
        }
    }
}

impl Drop for PtrOfPoolingLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPoolingLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPowerLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPowerLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPowerLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPowerLayer {
    pub fn as_raw_PtrOfPowerLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPowerLayer {
            ptr
        }
    }
}

impl Drop for PtrOfPowerLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPowerLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfPriorBoxLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfPriorBoxLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfPriorBoxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPriorBoxLayer {
    pub fn as_raw_PtrOfPriorBoxLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfPriorBoxLayer {
            ptr
        }
    }
}

impl Drop for PtrOfPriorBoxLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfPriorBoxLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfProposalLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfProposalLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfProposalLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfProposalLayer {
    pub fn as_raw_PtrOfProposalLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfProposalLayer {
            ptr
        }
    }
}

impl Drop for PtrOfProposalLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfProposalLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRNNLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfRNNLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRNNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRNNLayer {
    pub fn as_raw_PtrOfRNNLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfRNNLayer {
            ptr
        }
    }
}

impl Drop for PtrOfRNNLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfRNNLayer_delete(self.ptr) };
    }
}
impl crate::dnn::Layer for PtrOfRNNLayer {
    fn as_raw_Layer(&self) -> *mut c_void {
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfRNNLayer {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

impl crate::dnn::RNNLayer for PtrOfRNNLayer {
    fn as_raw_RNNLayer(&self) -> *mut c_void {
        unsafe { cv_PtrOfRNNLayer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfRTrees_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfRTrees_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRTrees {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRTrees {
    pub fn as_raw_PtrOfRTrees(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfRTrees {
            ptr
        }
    }
}

impl Drop for PtrOfRTrees {
    fn drop(&mut self) {
        unsafe { cv_PtrOfRTrees_delete(self.ptr) };
    }
}
impl crate::ml::DTrees for PtrOfRTrees {
    fn as_raw_DTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfRTrees {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfRTrees {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

impl crate::ml::RTrees for PtrOfRTrees {
    fn as_raw_RTrees(&self) -> *mut c_void {
        unsafe { cv_PtrOfRTrees_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfRadialVarianceHash_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfRadialVarianceHash_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRadialVarianceHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRadialVarianceHash {
    pub fn as_raw_PtrOfRadialVarianceHash(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfRadialVarianceHash {
            ptr
        }
    }
}

impl Drop for PtrOfRadialVarianceHash {
    fn drop(&mut self) {
        unsafe { cv_PtrOfRadialVarianceHash_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReLU6Layer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfReLU6Layer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReLU6Layer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReLU6Layer {
    pub fn as_raw_PtrOfReLU6Layer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfReLU6Layer {
            ptr
        }
    }
}

impl Drop for PtrOfReLU6Layer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfReLU6Layer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReLULayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfReLULayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReLULayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReLULayer {
    pub fn as_raw_PtrOfReLULayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfReLULayer {
            ptr
        }
    }
}

impl Drop for PtrOfReLULayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfReLULayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRegionLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfRegionLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRegionLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRegionLayer {
    pub fn as_raw_PtrOfRegionLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfRegionLayer {
            ptr
        }
    }
}

impl Drop for PtrOfRegionLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfRegionLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReorgLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfReorgLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReorgLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReorgLayer {
    pub fn as_raw_PtrOfReorgLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfReorgLayer {
            ptr
        }
    }
}

impl Drop for PtrOfReorgLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfReorgLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfReshapeLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfReshapeLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfReshapeLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReshapeLayer {
    pub fn as_raw_PtrOfReshapeLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfReshapeLayer {
            ptr
        }
    }
}

impl Drop for PtrOfReshapeLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfReshapeLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfResizeLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfResizeLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfResizeLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfResizeLayer {
    pub fn as_raw_PtrOfResizeLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfResizeLayer {
            ptr
        }
    }
}

impl Drop for PtrOfResizeLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfResizeLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfRetina_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfRetina_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRetina {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRetina {
    pub fn as_raw_PtrOfRetina(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfRetina {
            ptr
        }
    }
}

impl Drop for PtrOfRetina {
    fn drop(&mut self) {
        unsafe { cv_PtrOfRetina_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfRetina {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfRetina_get(self.ptr) }
    }
}

impl crate::bioinspired::Retina for PtrOfRetina {
    fn as_raw_Retina(&self) -> *mut c_void {
        unsafe { cv_PtrOfRetina_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfRetinaFastToneMapping_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfRetinaFastToneMapping_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfRetinaFastToneMapping {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRetinaFastToneMapping {
    pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfRetinaFastToneMapping {
            ptr
        }
    }
}

impl Drop for PtrOfRetinaFastToneMapping {
    fn drop(&mut self) {
        unsafe { cv_PtrOfRetinaFastToneMapping_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfRetinaFastToneMapping {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfRetinaFastToneMapping_get(self.ptr) }
    }
}

impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
    fn as_raw_RetinaFastToneMapping(&self) -> *mut c_void {
        unsafe { cv_PtrOfRetinaFastToneMapping_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSIFT_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSIFT_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSIFT {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSIFT {
    pub fn as_raw_PtrOfSIFT(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSIFT {
            ptr
        }
    }
}

impl Drop for PtrOfSIFT {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSIFT_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSURF_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSURF_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSURF {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSURF {
    pub fn as_raw_PtrOfSURF(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSURF {
            ptr
        }
    }
}

impl Drop for PtrOfSURF {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSURF_delete(self.ptr) };
    }
}
impl crate::xfeatures2d::SURF for PtrOfSURF {
    fn as_raw_SURF(&self) -> *mut c_void {
        unsafe { cv_PtrOfSURF_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSVM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSVM_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSVM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSVM {
    pub fn as_raw_PtrOfSVM(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSVM {
            ptr
        }
    }
}

impl Drop for PtrOfSVM {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSVM_delete(self.ptr) };
    }
}
impl crate::ml::SVM for PtrOfSVM {
    fn as_raw_SVM(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSVM {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfSVM {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSVMSGD_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSVMSGD_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSVMSGD {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSVMSGD {
    pub fn as_raw_PtrOfSVMSGD(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSVMSGD {
            ptr
        }
    }
}

impl Drop for PtrOfSVMSGD {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSVMSGD_delete(self.ptr) };
    }
}
impl crate::ml::SVMSGD for PtrOfSVMSGD {
    fn as_raw_SVMSGD(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSVMSGD {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

impl crate::ml::StatModel for PtrOfSVMSGD {
    fn as_raw_StatModel(&self) -> *mut c_void {
        unsafe { cv_PtrOfSVMSGD_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfScaleLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfScaleLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfScaleLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfScaleLayer {
    pub fn as_raw_PtrOfScaleLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfScaleLayer {
            ptr
        }
    }
}

impl Drop for PtrOfScaleLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfScaleLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfShapeContextDistanceExtractor_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfShapeContextDistanceExtractor_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfShapeContextDistanceExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfShapeContextDistanceExtractor {
    pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfShapeContextDistanceExtractor {
            ptr
        }
    }
}

impl Drop for PtrOfShapeContextDistanceExtractor {
    fn drop(&mut self) {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_delete(self.ptr) };
    }
}
impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfShapeContextDistanceExtractor {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void {
        unsafe { cv_PtrOfShapeContextDistanceExtractor_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSigmoidLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSigmoidLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSigmoidLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSigmoidLayer {
    pub fn as_raw_PtrOfSigmoidLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSigmoidLayer {
            ptr
        }
    }
}

impl Drop for PtrOfSigmoidLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSigmoidLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSimpleBlobDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSimpleBlobDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSimpleBlobDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSimpleBlobDetector {
    pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSimpleBlobDetector {
            ptr
        }
    }
}

impl Drop for PtrOfSimpleBlobDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSimpleBlobDetector_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSliceLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSliceLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSliceLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSliceLayer {
    pub fn as_raw_PtrOfSliceLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSliceLayer {
            ptr
        }
    }
}

impl Drop for PtrOfSliceLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSliceLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSoftmaxLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSoftmaxLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSoftmaxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSoftmaxLayer {
    pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSoftmaxLayer {
            ptr
        }
    }
}

impl Drop for PtrOfSoftmaxLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSoftmaxLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSparsePyrLKOpticalFlow_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSparsePyrLKOpticalFlow_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSparsePyrLKOpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSparsePyrLKOpticalFlow {
    pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSparsePyrLKOpticalFlow {
            ptr
        }
    }
}

impl Drop for PtrOfSparsePyrLKOpticalFlow {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_delete(self.ptr) };
    }
}
impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    fn as_raw_SparseOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSparsePyrLKOpticalFlow {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void {
        unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfSplitLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSplitLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSplitLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSplitLayer {
    pub fn as_raw_PtrOfSplitLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSplitLayer {
            ptr
        }
    }
}

impl Drop for PtrOfSplitLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSplitLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfStarDetector_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfStarDetector_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStarDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStarDetector {
    pub fn as_raw_PtrOfStarDetector(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfStarDetector {
            ptr
        }
    }
}

impl Drop for PtrOfStarDetector {
    fn drop(&mut self) {
        unsafe { cv_PtrOfStarDetector_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfStereoBM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfStereoBM_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStereoBM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStereoBM {
    pub fn as_raw_PtrOfStereoBM(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfStereoBM {
            ptr
        }
    }
}

impl Drop for PtrOfStereoBM {
    fn drop(&mut self) {
        unsafe { cv_PtrOfStereoBM_delete(self.ptr) };
    }
}
impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
    fn as_raw_StereoMatcher(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfStereoBM {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

impl crate::calib3d::StereoBM for PtrOfStereoBM {
    fn as_raw_StereoBM(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoBM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfStereoSGBM_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfStereoSGBM_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStereoSGBM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStereoSGBM {
    pub fn as_raw_PtrOfStereoSGBM(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfStereoSGBM {
            ptr
        }
    }
}

impl Drop for PtrOfStereoSGBM {
    fn drop(&mut self) {
        unsafe { cv_PtrOfStereoSGBM_delete(self.ptr) };
    }
}
impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
    fn as_raw_StereoMatcher(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfStereoSGBM {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
    fn as_raw_StereoSGBM(&self) -> *mut c_void {
        unsafe { cv_PtrOfStereoSGBM_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfStitcher_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfStitcher_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfStitcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStitcher {
    pub fn as_raw_PtrOfStitcher(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfStitcher {
            ptr
        }
    }
}

impl Drop for PtrOfStitcher {
    fn drop(&mut self) {
        unsafe { cv_PtrOfStitcher_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfSuperResolution_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfSuperResolution_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfSuperResolution {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSuperResolution {
    pub fn as_raw_PtrOfSuperResolution(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfSuperResolution {
            ptr
        }
    }
}

impl Drop for PtrOfSuperResolution {
    fn drop(&mut self) {
        unsafe { cv_PtrOfSuperResolution_delete(self.ptr) };
    }
}
impl crate::superres::SuperResolution for PtrOfSuperResolution {
    fn as_raw_SuperResolution(&self) -> *mut c_void {
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfSuperResolution {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

impl crate::superres::FrameSource for PtrOfSuperResolution {
    fn as_raw_FrameSource(&self) -> *mut c_void {
        unsafe { cv_PtrOfSuperResolution_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTanHLayer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTanHLayer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTanHLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTanHLayer {
    pub fn as_raw_PtrOfTanHLayer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTanHLayer {
            ptr
        }
    }
}

impl Drop for PtrOfTanHLayer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTanHLayer_delete(self.ptr) };
    }
}
extern "C" {
    #[doc(hidden)] fn cv_PtrOfThinPlateSplineShapeTransformer_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfThinPlateSplineShapeTransformer_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfThinPlateSplineShapeTransformer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfThinPlateSplineShapeTransformer {
    pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfThinPlateSplineShapeTransformer {
            ptr
        }
    }
}

impl Drop for PtrOfThinPlateSplineShapeTransformer {
    fn drop(&mut self) {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfThinPlateSplineShapeTransformer {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    fn as_raw_ShapeTransformer(&self) -> *mut c_void {
        unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemap_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTonemap_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemap {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemap {
    pub fn as_raw_PtrOfTonemap(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTonemap {
            ptr
        }
    }
}

impl Drop for PtrOfTonemap {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTonemap_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTonemap {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemap_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemap {
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemap_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapDrago_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTonemapDrago_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemapDrago {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapDrago {
    pub fn as_raw_PtrOfTonemapDrago(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTonemapDrago {
            ptr
        }
    }
}

impl Drop for PtrOfTonemapDrago {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTonemapDrago_delete(self.ptr) };
    }
}
impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
    fn as_raw_TonemapDrago(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

impl core::Algorithm for PtrOfTonemapDrago {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemapDrago {
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapDrago_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapMantiuk_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTonemapMantiuk_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemapMantiuk {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapMantiuk {
    pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTonemapMantiuk {
            ptr
        }
    }
}

impl Drop for PtrOfTonemapMantiuk {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTonemapMantiuk_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTonemapMantiuk {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
    fn as_raw_TonemapMantiuk(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapMantiuk_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTonemapReinhard_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTonemapReinhard_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTonemapReinhard {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapReinhard {
    pub fn as_raw_PtrOfTonemapReinhard(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTonemapReinhard {
            ptr
        }
    }
}

impl Drop for PtrOfTonemapReinhard {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTonemapReinhard_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTonemapReinhard {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

impl crate::photo::Tonemap for PtrOfTonemapReinhard {
    fn as_raw_Tonemap(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
    fn as_raw_TonemapReinhard(&self) -> *mut c_void {
        unsafe { cv_PtrOfTonemapReinhard_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTrainData_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTrainData_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTrainData {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTrainData {
    pub fn as_raw_PtrOfTrainData(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTrainData {
            ptr
        }
    }
}

impl Drop for PtrOfTrainData {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTrainData_delete(self.ptr) };
    }
}
impl crate::ml::TrainData for PtrOfTrainData {
    fn as_raw_TrainData(&self) -> *mut c_void {
        unsafe { cv_PtrOfTrainData_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfTransientAreasSegmentationModule_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfTransientAreasSegmentationModule_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfTransientAreasSegmentationModule {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTransientAreasSegmentationModule {
    pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfTransientAreasSegmentationModule {
            ptr
        }
    }
}

impl Drop for PtrOfTransientAreasSegmentationModule {
    fn drop(&mut self) {
        unsafe { cv_PtrOfTransientAreasSegmentationModule_delete(self.ptr) };
    }
}
impl core::Algorithm for PtrOfTransientAreasSegmentationModule {
    fn as_raw_Algorithm(&self) -> *mut c_void {
        unsafe { cv_PtrOfTransientAreasSegmentationModule_get(self.ptr) }
    }
}

impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
    fn as_raw_TransientAreasSegmentationModule(&self) -> *mut c_void {
        unsafe { cv_PtrOfTransientAreasSegmentationModule_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOfVGG_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOfVGG_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOfVGG {
    pub(crate) ptr: *mut c_void
}

impl PtrOfVGG {
    pub fn as_raw_PtrOfVGG(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOfVGG {
            ptr
        }
    }
}

impl Drop for PtrOfVGG {
    fn drop(&mut self) {
        unsafe { cv_PtrOfVGG_delete(self.ptr) };
    }
}
impl crate::xfeatures2d::VGG for PtrOfVGG {
    fn as_raw_VGG(&self) -> *mut c_void {
        unsafe { cv_PtrOfVGG_get(self.ptr) }
    }
}

extern "C" {
    #[doc(hidden)] fn cv_PtrOffloat_get(ptr: *mut c_void) -> *mut c_void;
    #[doc(hidden)] fn cv_PtrOffloat_delete(ptr: *mut c_void);
}

#[allow(dead_code)]
pub struct PtrOffloat {
    pub(crate) ptr: *mut c_void
}

impl PtrOffloat {
    pub fn as_raw_PtrOffloat(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PtrOffloat {
            ptr
        }
    }
}

impl Drop for PtrOffloat {
    fn drop(&mut self) {
        unsafe { cv_PtrOffloat_delete(self.ptr) };
    }
}
#[allow(dead_code)]
pub struct VectorOfDMatch {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfDMatch_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfDMatch_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfDMatch_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfDMatch_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfDMatch_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfDMatch_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfDMatch_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfDMatch_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfDMatch {
    pub fn as_raw_VectorOfDMatch(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfDMatch_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfDMatch_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfDMatch_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfDMatch_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfDMatch {
    fn drop(&mut self) {
        unsafe { cv_VectorOfDMatch_delete(self.ptr) };
    }
}
impl VectorOfDMatch {
    pub fn push(&mut self, val: core::DMatch) {
        unsafe { cv_VectorOfDMatch_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::DMatch {
        unsafe { (cv_VectorOfDMatch_get(self.ptr, index) as *mut core::DMatch).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::DMatch {
        unsafe { (cv_VectorOfDMatch_get(self.ptr, index) as *mut core::DMatch).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfDMatch_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfDMatch {
    type Target = [core::DMatch];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfDMatch_size(self.ptr);
            let data = cv_VectorOfDMatch_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfDetectionROI {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfDetectionROI_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfDetectionROI_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfDetectionROI_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfDetectionROI_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfDetectionROI_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfDetectionROI_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfDetectionROI_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfDetectionROI_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfDetectionROI {
    pub fn as_raw_VectorOfDetectionROI(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfDetectionROI_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfDetectionROI_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfDetectionROI_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfDetectionROI_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfDetectionROI {
    fn drop(&mut self) {
        unsafe { cv_VectorOfDetectionROI_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfDetectionROI {
    pub fn push(&mut self, val: crate::objdetect::DetectionROI) {
        unsafe { cv_VectorOfDetectionROI_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> crate::objdetect::DetectionROI {
        crate::objdetect::DetectionROI { ptr: unsafe { cv_VectorOfDetectionROI_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> crate::objdetect::DetectionROI {
        crate::objdetect::DetectionROI { ptr: unsafe { cv_VectorOfDetectionROI_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::objdetect::DetectionROI> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfElliptic_KeyPoint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfElliptic_KeyPoint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfElliptic_KeyPoint {
    pub fn as_raw_VectorOfElliptic_KeyPoint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfElliptic_KeyPoint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfElliptic_KeyPoint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfElliptic_KeyPoint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfElliptic_KeyPoint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfElliptic_KeyPoint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfElliptic_KeyPoint_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfElliptic_KeyPoint {
    pub fn push(&mut self, val: crate::xfeatures2d::Elliptic_KeyPoint) {
        unsafe { cv_VectorOfElliptic_KeyPoint_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> crate::xfeatures2d::Elliptic_KeyPoint {
        crate::xfeatures2d::Elliptic_KeyPoint { ptr: unsafe { cv_VectorOfElliptic_KeyPoint_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> crate::xfeatures2d::Elliptic_KeyPoint {
        crate::xfeatures2d::Elliptic_KeyPoint { ptr: unsafe { cv_VectorOfElliptic_KeyPoint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::xfeatures2d::Elliptic_KeyPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfExtObject {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfExtObject_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfExtObject_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfExtObject_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfExtObject_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfExtObject_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfExtObject_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfExtObject_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfExtObject_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfExtObject {
    pub fn as_raw_VectorOfExtObject(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfExtObject_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfExtObject_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfExtObject_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfExtObject_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfExtObject {
    fn drop(&mut self) {
        unsafe { cv_VectorOfExtObject_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfExtObject {
    pub fn push(&mut self, val: crate::objdetect::DetectionBasedTracker_ExtObject) {
        unsafe { cv_VectorOfExtObject_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> crate::objdetect::DetectionBasedTracker_ExtObject {
        crate::objdetect::DetectionBasedTracker_ExtObject { ptr: unsafe { cv_VectorOfExtObject_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> crate::objdetect::DetectionBasedTracker_ExtObject {
        crate::objdetect::DetectionBasedTracker_ExtObject { ptr: unsafe { cv_VectorOfExtObject_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::objdetect::DetectionBasedTracker_ExtObject> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfKeyPoint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfKeyPoint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfKeyPoint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfKeyPoint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfKeyPoint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfKeyPoint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfKeyPoint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfKeyPoint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfKeyPoint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfKeyPoint {
    pub fn as_raw_VectorOfKeyPoint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfKeyPoint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfKeyPoint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfKeyPoint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfKeyPoint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfKeyPoint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfKeyPoint_delete(self.ptr) };
    }
}
impl VectorOfKeyPoint {
    pub fn push(&mut self, val: core::KeyPoint) {
        unsafe { cv_VectorOfKeyPoint_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::KeyPoint {
        unsafe { (cv_VectorOfKeyPoint_get(self.ptr, index) as *mut core::KeyPoint).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::KeyPoint {
        unsafe { (cv_VectorOfKeyPoint_get(self.ptr, index) as *mut core::KeyPoint).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfKeyPoint_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfKeyPoint {
    type Target = [core::KeyPoint];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfKeyPoint_size(self.ptr);
            let data = cv_VectorOfKeyPoint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfMat {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfMat_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfMat_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfMat_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfMat_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfMat_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfMat_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfMat_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfMat_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfMat {
    pub fn as_raw_VectorOfMat(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfMat_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfMat_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfMat_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfMat_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfMat {
    fn drop(&mut self) {
        unsafe { cv_VectorOfMat_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfMat {
    pub fn push(&mut self, val: core::Mat) {
        unsafe { cv_VectorOfMat_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> core::Mat {
        core::Mat { ptr: unsafe { cv_VectorOfMat_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> core::Mat {
        core::Mat { ptr: unsafe { cv_VectorOfMat_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<core::Mat> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfNode {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfNode_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfNode_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfNode_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfNode_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfNode_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfNode_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfNode_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfNode_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfNode {
    pub fn as_raw_VectorOfNode(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfNode_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfNode_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfNode_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfNode_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfNode {
    fn drop(&mut self) {
        unsafe { cv_VectorOfNode_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfNode {
    pub fn push(&mut self, val: crate::ml::DTrees_Node) {
        unsafe { cv_VectorOfNode_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> crate::ml::DTrees_Node {
        crate::ml::DTrees_Node { ptr: unsafe { cv_VectorOfNode_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> crate::ml::DTrees_Node {
        crate::ml::DTrees_Node { ptr: unsafe { cv_VectorOfNode_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::ml::DTrees_Node> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfObjectDetection {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfObjectDetection_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfObjectDetection_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfObjectDetection_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfObjectDetection_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfObjectDetection_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfObjectDetection_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfObjectDetection_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfObjectDetection_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfObjectDetection {
    pub fn as_raw_VectorOfObjectDetection(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfObjectDetection_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfObjectDetection_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfObjectDetection_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfObjectDetection_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfObjectDetection {
    fn drop(&mut self) {
        unsafe { cv_VectorOfObjectDetection_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfObjectDetection {
    pub fn push(&mut self, val: crate::dpm::DPMDetector_ObjectDetection) {
        unsafe { cv_VectorOfObjectDetection_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> crate::dpm::DPMDetector_ObjectDetection {
        crate::dpm::DPMDetector_ObjectDetection { ptr: unsafe { cv_VectorOfObjectDetection_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> crate::dpm::DPMDetector_ObjectDetection {
        crate::dpm::DPMDetector_ObjectDetection { ptr: unsafe { cv_VectorOfObjectDetection_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::dpm::DPMDetector_ObjectDetection> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfPoint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfPoint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPoint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPoint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfPoint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfPoint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPoint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPoint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPoint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfPoint {
    pub fn as_raw_VectorOfPoint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfPoint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfPoint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfPoint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfPoint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfPoint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfPoint_delete(self.ptr) };
    }
}
impl VectorOfPoint {
    pub fn push(&mut self, val: core::Point) {
        unsafe { cv_VectorOfPoint_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Point {
        unsafe { (cv_VectorOfPoint_get(self.ptr, index) as *mut core::Point).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Point {
        unsafe { (cv_VectorOfPoint_get(self.ptr, index) as *mut core::Point).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfPoint_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfPoint {
    type Target = [core::Point];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfPoint_size(self.ptr);
            let data = cv_VectorOfPoint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfPoint2d {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfPoint2d_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPoint2d_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPoint2d_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfPoint2d_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfPoint2d_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPoint2d_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPoint2d_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPoint2d_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfPoint2d {
    pub fn as_raw_VectorOfPoint2d(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfPoint2d_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfPoint2d_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfPoint2d_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfPoint2d_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfPoint2d {
    fn drop(&mut self) {
        unsafe { cv_VectorOfPoint2d_delete(self.ptr) };
    }
}
impl VectorOfPoint2d {
    pub fn push(&mut self, val: core::Point2d) {
        unsafe { cv_VectorOfPoint2d_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Point2d {
        unsafe { (cv_VectorOfPoint2d_get(self.ptr, index) as *mut core::Point2d).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Point2d {
        unsafe { (cv_VectorOfPoint2d_get(self.ptr, index) as *mut core::Point2d).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfPoint2d_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfPoint2d {
    type Target = [core::Point2d];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfPoint2d_size(self.ptr);
            let data = cv_VectorOfPoint2d_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfPoint2f {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfPoint2f_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPoint2f_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPoint2f_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfPoint2f_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfPoint2f_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPoint2f_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPoint2f_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPoint2f_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfPoint2f {
    pub fn as_raw_VectorOfPoint2f(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfPoint2f_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfPoint2f_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfPoint2f_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfPoint2f_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfPoint2f {
    fn drop(&mut self) {
        unsafe { cv_VectorOfPoint2f_delete(self.ptr) };
    }
}
impl VectorOfPoint2f {
    pub fn push(&mut self, val: core::Point2f) {
        unsafe { cv_VectorOfPoint2f_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Point2f {
        unsafe { (cv_VectorOfPoint2f_get(self.ptr, index) as *mut core::Point2f).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Point2f {
        unsafe { (cv_VectorOfPoint2f_get(self.ptr, index) as *mut core::Point2f).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfPoint2f_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfPoint2f {
    type Target = [core::Point2f];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfPoint2f_size(self.ptr);
            let data = cv_VectorOfPoint2f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfPtrOfBackendWrapper {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfPtrOfBackendWrapper_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfPtrOfBackendWrapper {
    pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfPtrOfBackendWrapper_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfPtrOfBackendWrapper_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfPtrOfBackendWrapper_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfPtrOfBackendWrapper_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfPtrOfBackendWrapper {
    fn drop(&mut self) {
        unsafe { cv_VectorOfPtrOfBackendWrapper_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfPtrOfBackendWrapper {
    pub fn push(&mut self, val: types::PtrOfBackendWrapper) {
        unsafe { cv_VectorOfPtrOfBackendWrapper_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::PtrOfBackendWrapper {
        types::PtrOfBackendWrapper { ptr: unsafe { cv_VectorOfPtrOfBackendWrapper_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::PtrOfBackendWrapper {
        types::PtrOfBackendWrapper { ptr: unsafe { cv_VectorOfPtrOfBackendWrapper_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::PtrOfBackendWrapper> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfRange {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfRange_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRange_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRange_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfRange_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfRange_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRange_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRange_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRange_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfRange {
    pub fn as_raw_VectorOfRange(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfRange_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfRange_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfRange_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfRange_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfRange {
    fn drop(&mut self) {
        unsafe { cv_VectorOfRange_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfRange {
    pub fn push(&mut self, val: core::Range) {
        unsafe { cv_VectorOfRange_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> core::Range {
        core::Range { ptr: unsafe { cv_VectorOfRange_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> core::Range {
        core::Range { ptr: unsafe { cv_VectorOfRange_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<core::Range> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfRect {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfRect_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRect_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRect_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfRect_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfRect_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRect_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRect_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRect_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfRect {
    pub fn as_raw_VectorOfRect(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfRect_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfRect_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfRect_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfRect_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfRect {
    fn drop(&mut self) {
        unsafe { cv_VectorOfRect_delete(self.ptr) };
    }
}
impl VectorOfRect {
    pub fn push(&mut self, val: core::Rect) {
        unsafe { cv_VectorOfRect_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Rect {
        unsafe { (cv_VectorOfRect_get(self.ptr, index) as *mut core::Rect).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Rect {
        unsafe { (cv_VectorOfRect_get(self.ptr, index) as *mut core::Rect).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfRect_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfRect {
    type Target = [core::Rect];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfRect_size(self.ptr);
            let data = cv_VectorOfRect_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfRect2d {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfRect2d_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRect2d_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRect2d_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfRect2d_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfRect2d_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRect2d_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRect2d_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRect2d_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfRect2d {
    pub fn as_raw_VectorOfRect2d(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfRect2d_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfRect2d_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfRect2d_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfRect2d_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfRect2d {
    fn drop(&mut self) {
        unsafe { cv_VectorOfRect2d_delete(self.ptr) };
    }
}
impl VectorOfRect2d {
    pub fn push(&mut self, val: core::Rect2d) {
        unsafe { cv_VectorOfRect2d_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Rect2d {
        unsafe { (cv_VectorOfRect2d_get(self.ptr, index) as *mut core::Rect2d).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Rect2d {
        unsafe { (cv_VectorOfRect2d_get(self.ptr, index) as *mut core::Rect2d).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfRect2d_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfRect2d {
    type Target = [core::Rect2d];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfRect2d_size(self.ptr);
            let data = cv_VectorOfRect2d_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfRotatedRect {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfRotatedRect_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRotatedRect_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfRotatedRect_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfRotatedRect_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfRotatedRect_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRotatedRect_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfRotatedRect_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfRotatedRect_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfRotatedRect {
    pub fn as_raw_VectorOfRotatedRect(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfRotatedRect_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfRotatedRect_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfRotatedRect_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfRotatedRect_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfRotatedRect {
    fn drop(&mut self) {
        unsafe { cv_VectorOfRotatedRect_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfRotatedRect {
    pub fn push(&mut self, val: core::RotatedRect) {
        unsafe { cv_VectorOfRotatedRect_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> core::RotatedRect {
        core::RotatedRect { ptr: unsafe { cv_VectorOfRotatedRect_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> core::RotatedRect {
        core::RotatedRect { ptr: unsafe { cv_VectorOfRotatedRect_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<core::RotatedRect> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfSplit {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfSplit_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfSplit_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfSplit_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfSplit_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfSplit_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfSplit_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfSplit_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfSplit_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfSplit {
    pub fn as_raw_VectorOfSplit(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfSplit_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfSplit_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfSplit_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfSplit_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfSplit {
    fn drop(&mut self) {
        unsafe { cv_VectorOfSplit_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfSplit {
    pub fn push(&mut self, val: crate::ml::DTrees_Split) {
        unsafe { cv_VectorOfSplit_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> crate::ml::DTrees_Split {
        crate::ml::DTrees_Split { ptr: unsafe { cv_VectorOfSplit_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> crate::ml::DTrees_Split {
        crate::ml::DTrees_Split { ptr: unsafe { cv_VectorOfSplit_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<crate::ml::DTrees_Split> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfString {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfString_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfString_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfString_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfString_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfString_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfString_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfString_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfString_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfString {
    pub fn as_raw_VectorOfString(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfString_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfString_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfString_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfString_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfString {
    fn drop(&mut self) {
        unsafe { cv_VectorOfString_delete(self.ptr) };
    }
}
impl VectorOfString {
    pub fn push(&mut self, val: String) {
        unsafe { cv_VectorOfString_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &String {
        unsafe { (cv_VectorOfString_get(self.ptr, index) as *mut String).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut String {
        unsafe { (cv_VectorOfString_get(self.ptr, index) as *mut String).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfString_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfString {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfString_size(self.ptr);
            let data = cv_VectorOfString_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfVec4f {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVec4f_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVec4f_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVec4f_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVec4f_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVec4f_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVec4f_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVec4f_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVec4f_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVec4f {
    pub fn as_raw_VectorOfVec4f(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVec4f_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVec4f_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVec4f_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVec4f_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVec4f {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVec4f_delete(self.ptr) };
    }
}
impl VectorOfVec4f {
    pub fn push(&mut self, val: core::Vec4f) {
        unsafe { cv_VectorOfVec4f_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Vec4f {
        unsafe { (cv_VectorOfVec4f_get(self.ptr, index) as *mut core::Vec4f).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Vec4f {
        unsafe { (cv_VectorOfVec4f_get(self.ptr, index) as *mut core::Vec4f).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfVec4f_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfVec4f {
    type Target = [core::Vec4f];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfVec4f_size(self.ptr);
            let data = cv_VectorOfVec4f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfVec6f {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVec6f_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVec6f_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVec6f_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVec6f_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVec6f_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVec6f_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVec6f_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVec6f_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVec6f {
    pub fn as_raw_VectorOfVec6f(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVec6f_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVec6f_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVec6f_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVec6f_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVec6f {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVec6f_delete(self.ptr) };
    }
}
impl VectorOfVec6f {
    pub fn push(&mut self, val: core::Vec6f) {
        unsafe { cv_VectorOfVec6f_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &core::Vec6f {
        unsafe { (cv_VectorOfVec6f_get(self.ptr, index) as *mut core::Vec6f).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut core::Vec6f {
        unsafe { (cv_VectorOfVec6f_get(self.ptr, index) as *mut core::Vec6f).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfVec6f_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfVec6f {
    type Target = [core::Vec6f];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfVec6f_size(self.ptr);
            let data = cv_VectorOfVec6f_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfDMatch {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfDMatch_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfDMatch {
    pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfDMatch_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfDMatch_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfDMatch_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfDMatch_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfDMatch {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfDMatch_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfDMatch {
    pub fn push(&mut self, val: types::VectorOfDMatch) {
        unsafe { cv_VectorOfVectorOfDMatch_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfDMatch {
        types::VectorOfDMatch { ptr: unsafe { cv_VectorOfVectorOfDMatch_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfDMatch {
        types::VectorOfDMatch { ptr: unsafe { cv_VectorOfVectorOfDMatch_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfDMatch> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfKeyPoint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfKeyPoint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfKeyPoint {
    pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfKeyPoint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfKeyPoint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfKeyPoint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfKeyPoint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfKeyPoint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfKeyPoint_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfKeyPoint {
    pub fn push(&mut self, val: types::VectorOfKeyPoint) {
        unsafe { cv_VectorOfVectorOfKeyPoint_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfKeyPoint {
        types::VectorOfKeyPoint { ptr: unsafe { cv_VectorOfVectorOfKeyPoint_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfKeyPoint {
        types::VectorOfKeyPoint { ptr: unsafe { cv_VectorOfVectorOfKeyPoint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfKeyPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfMat {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfMat_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfMat {
    pub fn as_raw_VectorOfVectorOfMat(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfMat_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfMat_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfMat_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfMat_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfMat {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfMat_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfMat {
    pub fn push(&mut self, val: types::VectorOfMat) {
        unsafe { cv_VectorOfVectorOfMat_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfMat {
        types::VectorOfMat { ptr: unsafe { cv_VectorOfVectorOfMat_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfMat {
        types::VectorOfMat { ptr: unsafe { cv_VectorOfVectorOfMat_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfMat> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfPoint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfPoint {
    pub fn as_raw_VectorOfVectorOfPoint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfPoint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfPoint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfPoint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfPoint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfPoint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfPoint_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfPoint {
    pub fn push(&mut self, val: types::VectorOfPoint) {
        unsafe { cv_VectorOfVectorOfPoint_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfPoint {
        types::VectorOfPoint { ptr: unsafe { cv_VectorOfVectorOfPoint_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfPoint {
        types::VectorOfPoint { ptr: unsafe { cv_VectorOfVectorOfPoint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfPoint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfPoint2f {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfPoint2f_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfPoint2f {
    pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfPoint2f_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfPoint2f_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfPoint2f_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfPoint2f_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfPoint2f {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfPoint2f_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfPoint2f {
    pub fn push(&mut self, val: types::VectorOfPoint2f) {
        unsafe { cv_VectorOfVectorOfPoint2f_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfPoint2f {
        types::VectorOfPoint2f { ptr: unsafe { cv_VectorOfVectorOfPoint2f_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfPoint2f {
        types::VectorOfPoint2f { ptr: unsafe { cv_VectorOfVectorOfPoint2f_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfPoint2f> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfRect {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfRect_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfRect {
    pub fn as_raw_VectorOfVectorOfRect(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfRect_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfRect_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfRect_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfRect_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfRect {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfRect_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfRect {
    pub fn push(&mut self, val: types::VectorOfRect) {
        unsafe { cv_VectorOfVectorOfRect_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfRect {
        types::VectorOfRect { ptr: unsafe { cv_VectorOfVectorOfRect_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfRect {
        types::VectorOfRect { ptr: unsafe { cv_VectorOfVectorOfRect_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfRect> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfVectorOfint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfVectorOfint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfVectorOfint {
    pub fn as_raw_VectorOfVectorOfVectorOfint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfVectorOfint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfVectorOfint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfVectorOfint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfVectorOfint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfVectorOfint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfVectorOfint_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfVectorOfint {
    pub fn push(&mut self, val: types::VectorOfVectorOfint) {
        unsafe { cv_VectorOfVectorOfVectorOfint_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfVectorOfint {
        types::VectorOfVectorOfint { ptr: unsafe { cv_VectorOfVectorOfVectorOfint_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfVectorOfint {
        types::VectorOfVectorOfint { ptr: unsafe { cv_VectorOfVectorOfVectorOfint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfVectorOfint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfbool {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfbool_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfbool {
    pub fn as_raw_VectorOfVectorOfbool(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfbool_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfbool_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfbool_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfbool_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfbool {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfbool_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfbool {
    pub fn push(&mut self, val: types::VectorOfbool) {
        unsafe { cv_VectorOfVectorOfbool_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfbool {
        types::VectorOfbool { ptr: unsafe { cv_VectorOfVectorOfbool_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfbool {
        types::VectorOfbool { ptr: unsafe { cv_VectorOfVectorOfbool_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfbool> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfchar {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfchar_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfchar {
    pub fn as_raw_VectorOfVectorOfchar(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfchar_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfchar_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfchar_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfchar_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfchar {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfchar_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfchar {
    pub fn push(&mut self, val: types::VectorOfchar) {
        unsafe { cv_VectorOfVectorOfchar_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfchar {
        types::VectorOfchar { ptr: unsafe { cv_VectorOfVectorOfchar_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfchar {
        types::VectorOfchar { ptr: unsafe { cv_VectorOfVectorOfchar_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfchar> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfint {
    pub fn as_raw_VectorOfVectorOfint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfint_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfint {
    pub fn push(&mut self, val: types::VectorOfint) {
        unsafe { cv_VectorOfVectorOfint_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfint {
        types::VectorOfint { ptr: unsafe { cv_VectorOfVectorOfint_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfint {
        types::VectorOfint { ptr: unsafe { cv_VectorOfVectorOfint_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfint> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfVectorOfuchar {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfVectorOfuchar_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfVectorOfuchar {
    pub fn as_raw_VectorOfVectorOfuchar(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfVectorOfuchar_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfuchar_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfVectorOfuchar_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfVectorOfuchar_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfVectorOfuchar {
    fn drop(&mut self) {
        unsafe { cv_VectorOfVectorOfuchar_delete(self.ptr) };
    }
}
// BoxedClassTypeInfo
impl VectorOfVectorOfuchar {
    pub fn push(&mut self, val: types::VectorOfuchar) {
        unsafe { cv_VectorOfVectorOfuchar_push_back(self.ptr, val.ptr) }
    }

    pub fn get(&self, index: size_t) -> types::VectorOfuchar {
        types::VectorOfuchar { ptr: unsafe { cv_VectorOfVectorOfuchar_get(self.ptr, index) } }
    }

    pub fn get_mut(&mut self, index: size_t) -> types::VectorOfuchar {
        types::VectorOfuchar { ptr: unsafe { cv_VectorOfVectorOfuchar_get(self.ptr, index) } }
    }

    pub fn to_vec(&self) -> Vec<types::VectorOfuchar> {
        (0..self.len()).map(|x| self.get(x)).collect()
    }
}
#[allow(dead_code)]
pub struct VectorOfbool {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfbool_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfbool_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfbool_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfbool_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfbool_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfbool_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfbool_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfbool_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfbool {
    pub fn as_raw_VectorOfbool(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfbool_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfbool_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfbool_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfbool_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfbool {
    fn drop(&mut self) {
        unsafe { cv_VectorOfbool_delete(self.ptr) };
    }
}
impl VectorOfbool {
    pub fn push(&mut self, val: bool) {
        unsafe { cv_VectorOfbool_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &bool {
        unsafe { (cv_VectorOfbool_get(self.ptr, index) as *mut bool).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut bool {
        unsafe { (cv_VectorOfbool_get(self.ptr, index) as *mut bool).as_mut().unwrap() }
    }
}
#[allow(dead_code)]
pub struct VectorOfchar {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfchar_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfchar_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfchar_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfchar_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfchar_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfchar_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfchar_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfchar_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfchar {
    pub fn as_raw_VectorOfchar(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfchar_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfchar_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfchar_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfchar_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfchar {
    fn drop(&mut self) {
        unsafe { cv_VectorOfchar_delete(self.ptr) };
    }
}
impl VectorOfchar {
    pub fn push(&mut self, val: i8) {
        unsafe { cv_VectorOfchar_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &i8 {
        unsafe { (cv_VectorOfchar_get(self.ptr, index) as *mut i8).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut i8 {
        unsafe { (cv_VectorOfchar_get(self.ptr, index) as *mut i8).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfchar_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfchar {
    type Target = [i8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfchar_size(self.ptr);
            let data = cv_VectorOfchar_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfdouble {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfdouble_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfdouble_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfdouble_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfdouble_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfdouble_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfdouble_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfdouble_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfdouble_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfdouble {
    pub fn as_raw_VectorOfdouble(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfdouble_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfdouble_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfdouble_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfdouble_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfdouble {
    fn drop(&mut self) {
        unsafe { cv_VectorOfdouble_delete(self.ptr) };
    }
}
impl VectorOfdouble {
    pub fn push(&mut self, val: f64) {
        unsafe { cv_VectorOfdouble_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &f64 {
        unsafe { (cv_VectorOfdouble_get(self.ptr, index) as *mut f64).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut f64 {
        unsafe { (cv_VectorOfdouble_get(self.ptr, index) as *mut f64).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfdouble_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfdouble {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfdouble_size(self.ptr);
            let data = cv_VectorOfdouble_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOffloat {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOffloat_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOffloat_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOffloat_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOffloat_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOffloat_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOffloat_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOffloat_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOffloat_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOffloat {
    pub fn as_raw_VectorOffloat(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOffloat_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOffloat_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOffloat_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOffloat_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOffloat {
    fn drop(&mut self) {
        unsafe { cv_VectorOffloat_delete(self.ptr) };
    }
}
impl VectorOffloat {
    pub fn push(&mut self, val: f32) {
        unsafe { cv_VectorOffloat_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &f32 {
        unsafe { (cv_VectorOffloat_get(self.ptr, index) as *mut f32).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut f32 {
        unsafe { (cv_VectorOffloat_get(self.ptr, index) as *mut f32).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOffloat_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOffloat {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOffloat_size(self.ptr);
            let data = cv_VectorOffloat_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfint {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfint_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfint_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfint_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfint_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfint_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfint_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfint_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfint_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfint {
    pub fn as_raw_VectorOfint(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfint_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfint_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfint_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfint_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfint {
    fn drop(&mut self) {
        unsafe { cv_VectorOfint_delete(self.ptr) };
    }
}
impl VectorOfint {
    pub fn push(&mut self, val: i32) {
        unsafe { cv_VectorOfint_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &i32 {
        unsafe { (cv_VectorOfint_get(self.ptr, index) as *mut i32).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut i32 {
        unsafe { (cv_VectorOfint_get(self.ptr, index) as *mut i32).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfint_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfint {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfint_size(self.ptr);
            let data = cv_VectorOfint_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfsize_t {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfsize_t_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfsize_t_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfsize_t_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfsize_t_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfsize_t_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfsize_t_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfsize_t_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfsize_t_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfsize_t {
    pub fn as_raw_VectorOfsize_t(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfsize_t_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfsize_t_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfsize_t_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfsize_t_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfsize_t {
    fn drop(&mut self) {
        unsafe { cv_VectorOfsize_t_delete(self.ptr) };
    }
}
impl VectorOfsize_t {
    pub fn push(&mut self, val: size_t) {
        unsafe { cv_VectorOfsize_t_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &size_t {
        unsafe { (cv_VectorOfsize_t_get(self.ptr, index) as *mut size_t).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut size_t {
        unsafe { (cv_VectorOfsize_t_get(self.ptr, index) as *mut size_t).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfsize_t_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfsize_t {
    type Target = [size_t];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfsize_t_size(self.ptr);
            let data = cv_VectorOfsize_t_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
#[allow(dead_code)]
pub struct VectorOfuchar {
    pub(crate) ptr: *mut c_void
}

extern "C" {
   #[doc(hidden)] fn cv_VectorOfuchar_new() -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfuchar_clone(src: *mut c_void) -> *mut c_void;
   #[doc(hidden)] fn cv_VectorOfuchar_delete(vec: *mut c_void);
   #[doc(hidden)] fn cv_VectorOfuchar_reserve(vec: *mut c_void, n: size_t);
   #[doc(hidden)] fn cv_VectorOfuchar_capacity(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfuchar_push_back(vec: *mut c_void, val_ref: *const c_void);
   #[doc(hidden)] fn cv_VectorOfuchar_size(vec: *mut c_void) -> size_t;
   #[doc(hidden)] fn cv_VectorOfuchar_get(vec: *mut c_void, index: size_t) -> *mut c_void;
}

impl VectorOfuchar {
    pub fn as_raw_VectorOfuchar(&self) -> *mut c_void { self.ptr }

    pub fn new() -> Self {
        unsafe { Self { ptr: cv_VectorOfuchar_new() } }
    }

    pub fn with_capacity(capacity: size_t) -> Self {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    } 

    pub fn len(&self) -> size_t {
        unsafe { cv_VectorOfuchar_size(self.ptr) }
    }

    pub fn capacity(&self) -> size_t {
        unsafe { cv_VectorOfuchar_capacity(self.ptr) }
    }

    pub fn reserve(&mut self, additional: size_t) {
        unsafe { cv_VectorOfuchar_reserve(self.ptr, self.len() + additional) }
    }
}

impl Drop for VectorOfuchar {
    fn drop(&mut self) {
        unsafe { cv_VectorOfuchar_delete(self.ptr) };
    }
}
impl VectorOfuchar {
    pub fn push(&mut self, val: u8) {
        unsafe { cv_VectorOfuchar_push_back(self.ptr, &val as *const _ as _) }
    }

    pub fn get(&self, index: size_t) -> &u8 {
        unsafe { (cv_VectorOfuchar_get(self.ptr, index) as *mut u8).as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, index: size_t) -> &mut u8 {
        unsafe { (cv_VectorOfuchar_get(self.ptr, index) as *mut u8).as_mut().unwrap() }
    }
}
extern "C" { #[doc(hidden)] fn cv_VectorOfuchar_data(ptr: *mut c_void) -> *mut c_void; }

impl ::std::ops::Deref for VectorOfuchar {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let length = cv_VectorOfuchar_size(self.ptr);
            let data = cv_VectorOfuchar_data(self.ptr);
            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
        }
    }
}
pub use crate::hub::manual::types::*;
