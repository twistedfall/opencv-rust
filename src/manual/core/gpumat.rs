use crate::core::{GpuMat, HostMem};
use crate::input_output_array;

input_output_array! { GpuMat, from_gpumat, from_gpumat_mut }
input_output_array! { HostMem, from_hostmem, from_hostmem_mut }
