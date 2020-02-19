.and_then(|x| {{unsafety_call}}{ x.as_{{ptr_call}}() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
