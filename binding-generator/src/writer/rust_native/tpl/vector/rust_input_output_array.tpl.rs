

impl core::ToInputArray for {{rust_full}} {
	{{input_array_impl}}
}

input_array_ref_forward! { {{rust_full}} }

impl core::ToOutputArray for {{rust_full}} {
	{{output_array_impl}}
}

impl core::ToInputOutputArray for {{rust_full}} {
	{{input_output_array_impl}}
}

output_array_ref_forward! { {{rust_full}} }
