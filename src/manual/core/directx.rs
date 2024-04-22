#![allow(non_snake_case)]

use std::ffi::c_void;

pub trait ID3D11DeviceTrait {
	fn as_raw_mut_ID3D11Device(&mut self) -> *mut c_void;
}

pub trait ID3D11Texture2DTrait {
	fn as_raw_mut_ID3D11Texture2D(&mut self) -> *mut c_void;
}

pub trait ID3D10Texture2DTrait {
	fn as_raw_mut_ID3D10Texture2D(&mut self) -> *mut c_void;
}

pub trait ID3D10DeviceTrait {
	fn as_raw_mut_ID3D10Device(&mut self) -> *mut c_void;
}

pub trait IDirect3DSurface9Trait {
	fn as_raw_mut_IDirect3DSurface9(&mut self) -> *mut c_void;
}

pub trait IDirect3DDevice9ExTrait {
	fn as_raw_mut_IDirect3DDevice9Ex(&mut self) -> *mut c_void;
}

pub trait IDirect3DDevice9Trait {
	fn as_raw_mut_IDirect3DDevice9(&mut self) -> *mut c_void;
}

#[cfg(target_os = "windows")]
mod types {
	use windows::core::Interface;
	use windows::Win32::Graphics::Direct3D10::{ID3D10Device, ID3D10Texture2D};
	use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D};
	use windows::Win32::Graphics::Direct3D9::{IDirect3DDevice9, IDirect3DDevice9Ex, IDirect3DSurface9};

	use super::*;

	impl ID3D11DeviceTrait for ID3D11Device {
		fn as_raw_mut_ID3D11Device(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}

	impl ID3D11Texture2DTrait for ID3D11Texture2D {
		fn as_raw_mut_ID3D11Texture2D(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}

	impl ID3D10Texture2DTrait for ID3D10Texture2D {
		fn as_raw_mut_ID3D10Texture2D(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}

	impl ID3D10DeviceTrait for ID3D10Device {
		fn as_raw_mut_ID3D10Device(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}

	impl IDirect3DSurface9Trait for IDirect3DSurface9 {
		fn as_raw_mut_IDirect3DSurface9(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}

	impl IDirect3DDevice9ExTrait for IDirect3DDevice9Ex {
		fn as_raw_mut_IDirect3DDevice9Ex(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}

	impl IDirect3DDevice9Trait for IDirect3DDevice9 {
		fn as_raw_mut_IDirect3DDevice9(&mut self) -> *mut c_void {
			self.as_raw()
		}
	}
}
