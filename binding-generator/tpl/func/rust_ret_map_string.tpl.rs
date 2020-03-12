.map(|s| unsafe { crate::templ::receive_string(s as *mut String) })
