ptr_extern! { crate::hdf::HDF5,
	cv_PtrLcv_hdf_HDF5G_new_null_const, cv_PtrLcv_hdf_HDF5G_delete, cv_PtrLcv_hdf_HDF5G_getInnerPtr_const, cv_PtrLcv_hdf_HDF5G_getInnerPtrMut
}

impl core::Ptr<crate::hdf::HDF5> {
	#[inline] pub fn as_raw_PtrOfHDF5(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHDF5(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::hdf::HDF5TraitConst for core::Ptr<crate::hdf::HDF5> {
	#[inline] fn as_raw_HDF5(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::hdf::HDF5Trait for core::Ptr<crate::hdf::HDF5> {
	#[inline] fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::hdf::HDF5> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHDF5")
			.finish()
	}
}

