use std::collections::HashMap;

use maplit::hashmap;
use regex::Regex;

use crate::string_ext::Indent;
use crate::{StrExt, StringExt};

#[test]
fn replace_in_place() {
	{
		let mut s = "".to_string();
		assert!(!s.replace_in_place(".", "-"));
		assert_eq!(s, "");
	}

	{
		let mut s = "test string".to_string();
		let ptr_before = s.as_bytes().as_ptr();
		assert!(s.replace_in_place("t", "!"));
		let ptr_after = s.as_bytes().as_ptr();
		assert_eq!(s, "!es! s!ring");
		assert_eq!(ptr_before, ptr_after);
	}
	{
		let mut s = "test string".to_string();
		// allocate string after the one already allocated to force realloc to move
		let _s = "test string".to_string();
		let ptr_before = s.as_bytes().as_ptr();
		let big_text = " ".repeat(1024 * 1024);
		let pad_string = "padding string".to_string();
		assert!(s.replace_in_place("t", &big_text));
		let ptr_after = s.as_bytes().as_ptr();
		assert_ne!(ptr_before, ptr_after);
		assert_eq!("padding string", pad_string);
	}

	{
		let mut s = "test string".to_string();
		assert!(!s.replace_in_place("*", "!"));
		assert_eq!(s, "test string");
	}

	{
		let mut s = "t".to_string();
		assert!(s.replace_in_place("t", ""));
		assert_eq!(s, "");
	}

	{
		let mut s = "ttt".to_string();
		assert!(!s.replace_in_place("", "!"));
		assert_eq!(s, "ttt");
	}

	{
		let mut s = "ttt".to_string();
		assert!(!s.replace_in_place("tttbbb", "!"));
		assert_eq!(s, "ttt");
	}

	{
		let mut s = "ttt".to_string();
		assert!(s.replace_in_place("tt", "!"));
		assert_eq!(s, "!t");
	}

	{
		let mut s = "ttxxxtt".to_string();
		assert!(s.replace_in_place("tt", ""));
		assert_eq!(s, "xxx");
	}

	{
		let mut s = "АБВ".to_string();
		assert!(s.replace_in_place("Б", "Г"));
		assert_eq!(s, "АГВ");
	}
}

#[test]
fn replace_in_place_regex() {
	{
		let mut s = "".to_string();
		assert!(!s.replace_in_place_regex(&Regex::new(".").unwrap(), "-"));
		assert_eq!(s, "");
	}

	{
		let mut s = "test string".to_string();
		let ptr_before = s.as_bytes().as_ptr();
		assert!(s.replace_in_place_regex(&Regex::new("t").unwrap(), "!"));
		let ptr_after = s.as_bytes().as_ptr();
		assert_eq!(s, "!es! s!ring");
		assert_eq!(ptr_before, ptr_after);
	}
	{
		let mut s = "test string".to_string();
		// allocate string after the one already allocated to force realloc to move
		let _s = "test string".to_string();
		let ptr_before = s.as_bytes().as_ptr();
		assert!(s.replace_in_place_regex(&Regex::new("t").unwrap(), "some very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very big text"));
		let ptr_after = s.as_bytes().as_ptr();
		assert_ne!(ptr_before, ptr_after);
	}

	{
		let mut s = "test string".to_string();
		assert!(!s.replace_in_place_regex(&Regex::new("\\*").unwrap(), "!"));
		assert_eq!(s, "test string");
	}

	{
		let mut s = "test string".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("^t").unwrap(), "!"));
		assert_eq!(s, "!est string");
	}

	{
		let mut s = "ggg".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("g$").unwrap(), "!"));
		assert_eq!(s, "gg!");
	}

	{
		let mut s = "test string".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("^(test).+$").unwrap(), "this $1 is good"));
		assert_eq!(s, "this test is good");
	}

	{
		let mut s = "test string".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("(t)(.)").unwrap(), "T($2-$1)"));
		assert_eq!(s, "T(e-t)sT( -t)sT(r-t)ing");
	}

	{
		let mut s = "test string".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("[ts]").unwrap(), "!"));
		assert_eq!(s, "!e!! !!ring");
	}

	{
		let mut s = "t".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("t").unwrap(), ""));
		assert_eq!(s, "");
	}

	{
		let mut s = "ttt".to_string();
		assert!(s.replacen_in_place_regex(&Regex::new("t").unwrap(), 2, "!"));
		assert_eq!(s, "!!t");
	}

	{
		let mut s = "ttt".to_string();
		assert!(!s.replace_in_place_regex(&Regex::new("").unwrap(), "!"));
		assert_eq!(s, "ttt");
	}

	{
		let mut s = "ttt".to_string();
		assert!(!s.replace_in_place_regex(&Regex::new("tttbbb").unwrap(), "!"));
		assert_eq!(s, "ttt");
	}

	{
		let mut s = "ttt".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("tt").unwrap(), "!"));
		assert_eq!(s, "!t");
	}

	{
		let mut s = "ttxxxtt".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("tt").unwrap(), ""));
		assert_eq!(s, "xxx");
	}

	{
		let mut s = "АБВ".to_string();
		assert!(s.replace_in_place_regex(&Regex::new("Б").unwrap(), "Г"));
		assert_eq!(s, "АГВ");
	}
}

#[test]
fn replace_in_place_regex_cb() {
	{
		let mut s = "ABCABCABCABABC".to_string();
		let mut i = 0;
		assert!(s.replace_in_place_regex_cb(&Regex::new("AB").unwrap(), |_, _| {
			i += 1;
			if i % 2 == 0 {
				Some("!".into())
			} else {
				None
			}
		}));
		assert_eq!(s, "ABC!CABC!ABC");
	}

	{
		let mut s = "ABCABCABCABABC".to_string();
		assert!(s.replace_in_place_regex_cb(&Regex::new("AB").unwrap(), |_, _| { Some("!!!".into()) }));
		assert_eq!(s, "!!!C!!!C!!!C!!!!!!C");
	}

	{
		let mut s = "ABCABCABCABABC".to_string();
		assert!(s.replace_in_place_regex_cb(&Regex::new("AB").unwrap(), |_, _| { Some("!".into()) }));
		assert_eq!(s, "!C!C!C!!C");
	}

	{
		let mut s = "ABCABCABCABAC".to_string();
		assert!(s.replacen_in_place_regex_cb(&Regex::new("A(.)").unwrap(), 3, |s, caps| {
			let (cap_start, cap_end) = caps.get(1).unwrap();
			let r = &s[cap_start..cap_end];
			Some(format!("!{}", r).into())
		}));
		assert_eq!(s, "!BC!BC!BCABAC");
	}

	{
		let mut s = "".to_string();
		assert!(!s.replace_in_place_regex_cb(&Regex::new("AB").unwrap(), |_, _| { Some("!".into()) }));
		assert_eq!(s, "");
	}

	{
		let mut s = "ABCABCABCABABC".to_string();
		assert!(!s.replace_in_place_regex_cb(&Regex::new("D").unwrap(), |_, _| { Some("!".into()) }));
		assert_eq!(s, "ABCABCABCABABC");
	}

	{
		let mut s = "ABC".to_string();
		assert!(!s.replace_in_place_regex_cb(&Regex::new("").unwrap(), |_, _| { Some("!".into()) }));
		assert_eq!(s, "ABC");
	}
}

#[test]
fn bump_counter() {
	{
		let mut s = "".to_string();
		s.bump_counter();
		assert_eq!("_1", s);
	}

	{
		let mut s = "func".to_string();
		s.bump_counter();
		assert_eq!("func_1", s);
	}

	{
		let mut s = "func_1".to_string();
		s.bump_counter();
		assert_eq!("func_2", s);
	}

	{
		let mut s = "func_999".to_string();
		s.bump_counter();
		assert_eq!("func_1000", s);
	}

	{
		let mut s = "func_0".to_string();
		s.bump_counter();
		assert_eq!("func_1", s);
	}

	{
		let mut s = "func1".to_string();
		s.bump_counter();
		assert_eq!("func1_1", s);
	}

	{
		let mut s = "func_3d".to_string();
		s.bump_counter();
		assert_eq!("func_3d_1", s);
	}

	{
		let mut s = "12345".to_string();
		s.bump_counter();
		assert_eq!("12345_1", s);
	}

	{
		let mut s = "func_-1".to_string();
		s.bump_counter();
		assert_eq!("func_-1_1", s);
	}

	{
		let mut s = "func_99999999999999999999999".to_string();
		s.bump_counter();
		assert_eq!("func_99999999999999999999999_1", s);
	}
}

#[test]
fn interpolate() {
	{
		let tpl = "";
		let res = tpl.compile_interpolation().interpolate(&hashmap! {"test" => "test"});
		assert_eq!("", res);
	}

	{
		let tpl = "";
		let res = tpl.compile_interpolation().interpolate(&HashMap::<_, &str>::new());
		assert_eq!("", res);
	}

	{
		let tpl = "{{ name1 }}, {{name2}}, {{name3
		}}, {{{name4}}}";
		let res = tpl.compile_interpolation().interpolate(&hashmap! {
			"name1" => "test1",
			"name2" => "test2",
			"name3" => "test3",
		});
		assert_eq!(
			"test1, test2, {{name3
		}}, {<parameter not found>}",
			res
		);
	}

	{
		let tpl = "
			{{name1}} tt
			line
				start {{ name2}} end
			{{name3 }} end

		";
		let res = tpl.compile_interpolation().interpolate(&hashmap! {
			"name1" => "test1",
			"name2" => "test21\ntest22",
			"name3" => "test3",
		});
		assert_eq!(
			"test1 tt
line
	start test21
	test22 end
test3 end

",
			res
		);
	}

	{
		let tpl = "
         {{name1}}
            {{ name2}}
         {{name3 }}";
		let res = tpl.compile_interpolation().interpolate(&hashmap! {
			"name1" => "test1",
			"name2" => "test21\ntest22",
			"name3" => "",
		});
		assert_eq!(
			"test1
   test21
   test22",
			res
		);
	}

	{
		let tpl = "
			{{name1}}
				{{ name2}} 			{{name3 }}";
		let res = tpl.compile_interpolation().interpolate(&hashmap! {
			"name1" => "",
			"name2" => "test2",
			"name3" => "test3",
		});
		assert_eq!("	test2 			test3", res);
	}
}

#[test]
fn detect_indent() {
	assert_eq!(Indent { len: 0, symbol: '\t' }, "".detect_indent());
	assert_eq!(Indent { len: 3, symbol: ' ' }, "   ".detect_indent());
	assert_eq!(Indent { len: 3, symbol: '\t' }, "  \ttest".detect_indent());
	assert_eq!(Indent { len: 1, symbol: '\t' }, "\ttest".detect_indent());
}

#[test]
fn to_snake_case() {
	assert_eq!("get_path_to_application", "getPathToApplication".to_snake_case());
	assert_eq!("solve_p3p", "solveP3P".to_snake_case());
	assert_eq!("solve_pnp", "solvePnP".to_snake_case());
	assert_eq!("reproject_image_to_3d", "reprojectImageTo3D".to_snake_case());
	assert_eq!("filter_2d", "filter2D".to_snake_case());
	assert_eq!("is_umat", "isUMat".to_snake_case());
}

#[test]
fn lines_with_nl() {
	assert_eq!(vec![""], "".lines_with_nl().collect::<Vec<_>>());
	assert_eq!(vec!["\n", ""], "\n".lines_with_nl().collect::<Vec<_>>());

	let s = "this
is
test";
	assert_eq!(vec!["this\n", "is\n", "test"], s.lines_with_nl().collect::<Vec<_>>());

	let s = "this
is
test
";
	assert_eq!(vec!["this\n", "is\n", "test\n", ""], s.lines_with_nl().collect::<Vec<_>>());
}

#[test]
fn trim_index() {
	let s = "test";
	assert_eq!(0, s.trim_start_idx());
	assert_eq!(4, s.trim_end_idx());

	let s = "";
	assert_eq!(0, s.trim_start_idx());
	assert_eq!(0, s.trim_end_idx());

	let s = "   test   ";
	assert_eq!(3, s.trim_start_idx());
	assert_eq!(7, s.trim_end_idx());

	let s = "  	 test \n   ";
	assert_eq!(4, s.trim_start_idx());
	assert_eq!(8, s.trim_end_idx());

	let s = "  	 test \n   ";
	let start = s.trim_start_idx();
	let end = s.trim_end_idx();
	assert_eq!("test", &s[start..end]);
}

#[test]
fn localname() {
	assert_eq!("test", "namespace::test".localname());
	assert_eq!("test", "test".localname());
	assert_eq!("", "".localname());
	assert_eq!("", "::".localname());
}

#[test]
fn namespace() {
	assert_eq!("namespace", "namespace::test".namespace());
	assert_eq!("test", "test".namespace());
	assert_eq!("", "".namespace());
	assert_eq!("", "::".namespace());
}
