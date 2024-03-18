use crate::{
	core::{self, _OutputArrayTraitConst},
	error::Result,
	sys::{self, ResultVoid},
	traits::OpenCVType,
};
use std::ffi::c_void;

use windows::{
	core::Interface,
	Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D},
};

pub unsafe fn convert_from_d3d11_texture_2d(texture: &ID3D11Texture2D, dst: &mut impl core::ToOutputArray) -> Result<()> {
	return_send!(via ocvrs_return);
	output_array_arg!(dst);
	cv_directx_convertFromD3D11Texture2D_ID3D11Texture2DX_const__OutputArrayR(
		texture.as_raw(),
		dst.as_raw__OutputArray(),
		ocvrs_return.as_mut_ptr(),
	);
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub unsafe fn initialize_context_from_d3d11_device(device: &ID3D11Device) -> Result<core::Context> {
	return_send!(via ocvrs_return);
	cv_directx_ocl_initializeContextFromD3D11Device_ID3D11Device2DX(device.as_raw(), ocvrs_return.as_mut_ptr());
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Context::opencv_from_extern(ret) };
	Ok(ret)
}

extern "C" {
	fn cv_directx_convertFromD3D11Texture2D_ID3D11Texture2DX_const__OutputArrayR(
		ptr: *const c_void,
		dst: *const c_void,
		ret: *mut ResultVoid,
	);

	fn cv_directx_ocl_initializeContextFromD3D11Device_ID3D11Device2DX(ptr: *mut c_void, ret: *mut sys::Result<*mut c_void>);
}
