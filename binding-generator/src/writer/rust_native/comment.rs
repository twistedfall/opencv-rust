use std::borrow::Cow;
use std::fmt::Write;

use once_cell::sync::Lazy;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;

use crate::comment::strip_comment_markers;
use crate::field::Field;
use crate::type_ref::FishStyle;
use crate::writer::rust_native::element::RustElement;
use crate::{StrExt, StringExt};

fn preprocess_formula(formula: &str) -> String {
	const ARG_REGEX: &str = r"\s*\{([^}]*?)\}";
	static MACROS: Lazy<Vec<(Regex, &str)>> = Lazy::new(|| {
		vec![
		(
			Regex::new(&format!("\\\\matTT{}", ARG_REGEX.repeat(9))).unwrap(),
			"\\[ \\left|\\begin{array}{ccc} $1 & $2 & $3\\\\ $4 & $5 & $6\\\\ $7 & $8 & $9 \\end{array}\\right| \\]",
		),
		(
			Regex::new(&format!("\\\\fork{}", ARG_REGEX.repeat(4))).unwrap(),
			"\\left\\{ \\begin{array}{l l} $1 & \\mbox{$2}\\\\ $3 & \\mbox{$4}\\\\ \\end{array} \\right.",
		),
		(
			Regex::new(&format!("\\\\forkthree{}", ARG_REGEX.repeat(6))).unwrap(),
			"\\left\\{ \\begin{array}{l l} $1 & \\mbox{$2}\\\\ $3 & \\mbox{$4}\\\\ $5 & \\mbox{$6}\\\\ \\end{array} \\right.",
		),
		(
			Regex::new(&format!("\\\\forkfour{}", ARG_REGEX.repeat(8))).unwrap(),
			"\\left\\{ \\begin{array}{l l} $1 & \\mbox{$2}\\\\ $3 & \\mbox{$4}\\\\ $5 & \\mbox{$6}\\\\ $7 & \\mbox{$8}\\\\ \\end{array} \\right.",
		),
		(
			Regex::new(&format!("\\\\vecthree{}", ARG_REGEX.repeat(3))).unwrap(),
			"\\begin{bmatrix} $1\\\\ $2\\\\ $3 \\end{bmatrix}",
		),
		(
			Regex::new(&format!("\\\\vecthreethree{}", ARG_REGEX.repeat(9))).unwrap(),
			"\\begin{bmatrix} $1 & $2 & $3\\\\ $4 & $5 & $6\\\\ $7 & $8 & $9 \\end{bmatrix}",
		),
		(
			Regex::new(&format!("\\\\hdotsfor{ARG_REGEX}")).unwrap(),
			"\\dots",
		),
		(
			Regex::new(&format!("\\\\mathbbm{ARG_REGEX}")).unwrap(),
			"\\mathbb{$1}",
		),
		(
			Regex::new(&format!("\\\\bordermatrix{}", ARG_REGEX.repeat(9))).unwrap(),
			"\\matrix{$1}",
		),
	]
	});

	let mut out = formula.to_string();
	for (re, repl) in &*MACROS {
		out.replace_in_place_regex(re, repl);
	}
	out
}

pub fn render_doc_comment(doc_comment: &str, prefix: &str, opencv_version: &str) -> String {
	render_doc_comment_with_processor(doc_comment, prefix, opencv_version, |_| {})
}

pub fn render_doc_comment_with_processor(
	doc_comment: &str,
	prefix: &str,
	opencv_version: &str,
	mut post_processor: impl FnMut(&mut String),
) -> String {
	let mut out = strip_comment_markers(doc_comment);
	out.replace_in_place("\r\n", "\n");
	// module titles
	static MODULE_TITLE_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)\s*@[}{].*$").unwrap());
	static MODULE_TITLE_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"@defgroup [^ ]+ (.*)").unwrap());
	static MODULE_TITLE_3: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^.*?@addtogroup\s+.+").unwrap());
	out.replace_in_place_regex(&MODULE_TITLE_1, "");
	out.replace_in_place_regex(&MODULE_TITLE_2, r#"# $1"#);
	out.replace_in_place_regex(&MODULE_TITLE_3, "");
	let trimmed = out.trim();
	if trimmed.len() != out.len() {
		out = trimmed.to_string()
	}

	// comment body markers
	static BRIEF: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@brief[ :]*"#).unwrap());
	out.replace_in_place_regex(&BRIEF, "");
	out.replace_in_place("@note", "\nNote:");

	// code blocks, don't run them during tests
	static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@code(?: ?\{.+?})?").unwrap());
	out.replace_in_place_regex(&CODE, "```C++");
	out.replace_in_place("@endcode", "```\n");

	// name block
	out.replace_in_place("@name ", "");

	// snippets
	static SNIPPET: Lazy<Regex> = Lazy::new(|| Regex::new(r"@snippet\s+([\w/.]+)\s+([\w-]+)").unwrap());
	out.replace_in_place_regex_cb(&SNIPPET, |comment, caps| {
		let path = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
		let name = caps.get(2).map(|(s, e)| &comment[s..e]).expect("Impossible");
		if path.starts_with("samples/") {
			// fixme: hack to detect hdf snippets
			Some(format!("[{name}](https://github.com/opencv/opencv_contrib/blob/{opencv_version}/modules/hdf/{path}#L1)",).into())
		} else {
			Some(
				format!("[{name}](https://github.com/opencv/opencv/blob/{opencv_version}/samples/cpp/tutorial_code/{path}#L1)",)
					.into(),
			)
		}
	});

	// some special casing for docs.rs build failures
	out.replace_in_place("'fps'", r#""fps""#);
	out.replace_in_place("'cv::Exception'", r#""cv::Exception""#);

	// see also block
	static SEE_ALSO_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^\s*@(sa|see)\s+").unwrap());
	static SEE_ALSO_INLINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@(sa|see)\s+").unwrap());
	if out.replacen_in_place_regex(&SEE_ALSO_BLOCK, 1, "## See also\n") {
		out.replace_in_place_regex(&SEE_ALSO_INLINE, "");
	} else {
		out.replace_in_place_regex(&SEE_ALSO_INLINE, "see also: ");
	}

	// citation links
	static CITE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@cite\s+([\w:]+)").unwrap());
	out.replace_in_place_regex(
		&CITE,
		&format!("[$1](https://docs.opencv.org/{opencv_version}/d0/de3/citelist.html#CITEREF_$1)"),
	);

	// references
	static REF: Lazy<Regex> = Lazy::new(|| Regex::new(r"@ref\s+([\w:]+)").unwrap());
	out.replace_in_place_regex(&REF, "[$1]");

	static REF_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"#(\w+)(\s+)").unwrap());
	out.replace_in_place_regex_cb(&REF_2, |comment, caps| {
		let name = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
		let space = caps.get(2).map(|(s, e)| &comment[s..e]).expect("Impossible");
		let name = if name.contains(char::is_lowercase) {
			Cow::Owned(name.cpp_name_to_rust_case())
		} else {
			Cow::Borrowed(name)
		};
		Some(format!("[{name}]{space}").into())
	});

	// images
	static IMAGE: Lazy<Regex> = Lazy::new(|| Regex::new(r"!\[(.*?)]\((?:.*/)?(.+)?\)").unwrap());
	out.replace_in_place_regex(&IMAGE, &format!("![$1](https://docs.opencv.org/{opencv_version}/$2)"));

	// returns
	static RETURNS: Lazy<Regex> = Lazy::new(|| Regex::new(r".*?@returns?\s*").unwrap());
	out.replace_in_place_regex(&RETURNS, "## Returns\n");

	// parameter list
	static PARAM_HEADER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^(.*?@param)"#).unwrap());
	static PARAM: Lazy<Regex> = Lazy::new(|| Regex::new(r".*?@param\s*(?:\[in]|(\[out]))?\s+(\w+) *(.*)").unwrap());
	out.replacen_in_place_regex(&PARAM_HEADER, 1, "## Parameters\n$1");
	out.replace_in_place_regex(&PARAM, "* $2:$1 $3");

	// deprecated
	static DEPRECATED: Lazy<Regex> = Lazy::new(|| Regex::new(r".*?@deprecated\s+(.+)").unwrap());
	let mut deprecated = None;
	out.replace_in_place_regex_cb(&DEPRECATED, |comment, caps| {
		let deprecated_msg = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible").to_string();
		let out = format!("\n**Deprecated**: {deprecated_msg}");
		deprecated = Some(deprecated_msg);
		Some(out.into())
	});

	// leading dashes
	static LEADING_DASH: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^(\s*)-(\s{2,})").unwrap());
	out.replace_in_place_regex(&LEADING_DASH, "$1*$2");

	// math expressions
	static BLOCK_FORMULA: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)\\f\[(.*?)\\f]").unwrap());
	static INLINE_FORMULA: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)\\f\$(.*?)\\f\$").unwrap());
	out.replace_in_place_regex_cb(&BLOCK_FORMULA, |out, caps| {
		let formula = preprocess_formula(caps.get(1).map(|(s, e)| &out[s..e]).expect("Impossible"));
		let encoded = utf8_percent_encode(&formula, NON_ALPHANUMERIC);
		Some(format!("![block formula](https://latex.codecogs.com/png.latex?{encoded})").into())
	});
	out.replace_in_place_regex_cb(&INLINE_FORMULA, |out, caps| {
		let formula = preprocess_formula(caps.get(1).map(|(s, e)| &out[s..e]).expect("Impossible"));
		let encoded = utf8_percent_encode(&formula, NON_ALPHANUMERIC);
		Some(format!("![inline formula](https://latex.codecogs.com/png.latex?{encoded})").into())
	});

	// separate urls
	static URL: Lazy<Regex> = Lazy::new(|| {
		Regex::new(r#"([^<"/(]|[^]]\(|^)(https?://[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_+.~#?&/=]*[-a-zA-Z0-9@:%_+~#?&/=])?)"#).unwrap()
	});
	out.replace_in_place_regex(&URL, "$1<$2>");

	// escapes
	static ESCAPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)\\n$").unwrap());
	out.replace_in_place_regex(&ESCAPE, "\n");

	// catch sequences of 4 indents and reduce them to avoid cargo test running them as code
	static INDENTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^(\s{3}|\s{7}|\s{11}|\s{15}|\s{19})\s(\S)").unwrap());
	out.replace_in_place_regex(&INDENTS, "$1$2");

	post_processor(&mut out);

	let mut out = if out.is_empty() || prefix.is_empty() {
		out
	} else {
		out.lines_with_nl().fold(
			String::with_capacity(out.len() + (prefix.len() + 1) * 128),
			|mut out_prefixed, line| {
				out_prefixed.push_str(prefix);
				out_prefixed.push(' ');
				out_prefixed + line
			},
		)
	};
	if let Some(deprecated) = deprecated {
		write!(out, "\n#[deprecated = \"{deprecated}\"]").expect("write! to String shouldn't fail");
	}
	out
}

pub fn render_cpp_default_args(args: &[Field]) -> String {
	let mut out = String::with_capacity(args.len() * 64);
	for arg in args {
		if let Some(def_val) = arg.default_value() {
			writeln!(&mut out, "* {name}: {def_val}", name = arg.rust_leafname(FishStyle::No))
				.expect("write! to String shouldn't fail");
		}
	}
	out
}

#[test]
fn test_render_doc_comment() {
	assert_eq!("test", &render_doc_comment("/** test */", "", "master"));
	assert_eq!("/// test", &render_doc_comment("//   test", "///", "master"));
	assert_eq!("//! test", &render_doc_comment("/*test */", "//!", "master"));

	{
		let comment = "\
/**
 * line1
 * line2
 */
";
		let res = "\
/// line1
/// line2";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/*
line1
 line2
*/
";
		let res = "\
/// line1
///  line2";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/** line1
 *     line2
 *     line3
";
		let res = "\
/// line1
/// line2
/// line3";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/* line1
      * line2
        line3
      * line4
*/
";
		let res = "\
/// line1
/// * line2
///   line3
/// * line4";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/**
 * line1
 * line2
                          line3
 */
";
		let res = "\
/// line1
/// line2
///                        line3";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/** line1
    line2
    line3*/
";
		let res = "\
/// line1
/// line2
/// line3";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
//!< line1
      //!< line2
";
		let res = "\
/// line1
/// line2";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/** line1
  line2*/
/** line3 */
";
		let res = "\
/// line1
///   line2
/// line3";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/** line1

line2

*/
";
		let res = "\
/// line1
///\x20
/// line2";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/**
  line1
\x20
  line2
*/
";
		let res = "\
/// line1
///\x20
/// line2";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "\
/** line1

 line2
 */
";
		let res = "\
/// line1
///\x20
/// line2";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}

	{
		let comment = "@deprecated test\r\ntest\r\n";
		assert!(comment.contains('\r'));
		let res = "\
///\x20
/// **Deprecated**: test
/// test
#[deprecated = \"test\"]";
		assert_eq!(res, &render_doc_comment(comment, "///", "master"));
	}
}
