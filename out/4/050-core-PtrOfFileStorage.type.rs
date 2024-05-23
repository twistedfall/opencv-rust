ptr_extern! { core::FileStorage,
	cv_PtrLcv_FileStorageG_new_null_const, cv_PtrLcv_FileStorageG_delete, cv_PtrLcv_FileStorageG_getInnerPtr_const, cv_PtrLcv_FileStorageG_getInnerPtrMut
}

ptr_extern_ctor! { core::FileStorage, cv_PtrLcv_FileStorageG_new_const_FileStorage }
impl core::Ptr<core::FileStorage> {
	#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::FileStorageTraitConst for core::Ptr<core::FileStorage> {
	#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::FileStorageTrait for core::Ptr<core::FileStorage> {
	#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::FileStorage> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFileStorage")
			.field("state", &core::FileStorageTraitConst::state(self))
			.field("elname", &core::FileStorageTraitConst::elname(self))
			.finish()
	}
}

