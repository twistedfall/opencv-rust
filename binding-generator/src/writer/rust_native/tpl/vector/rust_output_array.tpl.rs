


{{cfg_attr}}
impl ToOutputArray for {{rust_full}} {
	{{output_array_impl}}
}

{{cfg_attr}}
impl ToInputOutputArray for {{rust_full}} {
	{{input_output_array_impl}}
}

{{cfg_attr}}
output_array_ref_forward! { {{rust_full}} }
