ptr_extern! { crate::videostab::WeightingDeblurer,
	cv_PtrLcv_videostab_WeightingDeblurerG_new_null_const, cv_PtrLcv_videostab_WeightingDeblurerG_delete, cv_PtrLcv_videostab_WeightingDeblurerG_getInnerPtr_const, cv_PtrLcv_videostab_WeightingDeblurerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::WeightingDeblurer, cv_PtrLcv_videostab_WeightingDeblurerG_new_const_WeightingDeblurer }
impl core::Ptr<crate::videostab::WeightingDeblurer> {
	#[inline] pub fn as_raw_PtrOfWeightingDeblurer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWeightingDeblurer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::WeightingDeblurerTraitConst for core::Ptr<crate::videostab::WeightingDeblurer> {
	#[inline] fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WeightingDeblurerTrait for core::Ptr<crate::videostab::WeightingDeblurer> {
	#[inline] fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::DeblurerBaseTraitConst for core::Ptr<crate::videostab::WeightingDeblurer> {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DeblurerBaseTrait for core::Ptr<crate::videostab::WeightingDeblurer> {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::WeightingDeblurer>, core::Ptr<crate::videostab::DeblurerBase>, cv_PtrLcv_videostab_WeightingDeblurerG_to_PtrOfDeblurerBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::WeightingDeblurer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfWeightingDeblurer")
			.finish()
	}
}

