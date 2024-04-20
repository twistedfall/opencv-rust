


impl ToOutputArray for {{rust_full}} {
	{{output_array_impl}}
}

impl ToInputOutputArray for {{rust_full}} {
	{{input_output_array_impl}}
}

output_array_ref_forward! { {{rust_full}} }
