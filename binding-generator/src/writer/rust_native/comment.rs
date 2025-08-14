use std::borrow::Cow;
use std::fmt::Write;
use std::sync::LazyLock;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::bytes::Regex;

use crate::field::Field;
use crate::func::FuncKind;
use crate::type_ref::FishStyle;
use crate::writer::rust_native::class::ClassExt;
use crate::writer::rust_native::element::RustElement;
use crate::{CowMapBorrowedExt, CppNameStyle, Element, Func, NameStyle, StrExt, StringExt};

#[derive(Debug, Clone, PartialEq)]
pub struct RenderComment {
	pub doc_comment: String,
	pub attributes: Vec<String>,
}

impl RenderComment {
	pub fn new(mut doc_comment: String, opencv_version: &str) -> Self {
		// todo, simplify/optimize this function, spec is here https://www.doxygen.nl/manual/docblocks.html
		doc_comment.replace_in_place("\r\n", "\n");
		// module titles
		static MODULE_TITLE_1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)\s*@[}{].*$").unwrap());
		static MODULE_TITLE_2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@defgroup [^ ]+ (.*)").unwrap());
		static MODULE_TITLE_3: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^.*?@addtogroup\s+.+").unwrap());
		doc_comment.replace_in_place_regex(&MODULE_TITLE_1, "");
		doc_comment.replace_in_place_regex(&MODULE_TITLE_2, r#"# $1"#);
		doc_comment.replace_in_place_regex(&MODULE_TITLE_3, "");
		let trimmed = doc_comment.trim();
		if trimmed.len() != doc_comment.len() {
			doc_comment = trimmed.to_string()
		}

		// comment body markers
		static BRIEF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"@brief[ :]*"#).unwrap());
		doc_comment.replace_in_place_regex(&BRIEF, "");
		doc_comment.replace_in_place("@note", "\nNote:");

		// code blocks, don't run them during tests
		static CODE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@code(?: ?\{.+?})?").unwrap());
		doc_comment.replace_in_place_regex(&CODE, "```C++");
		doc_comment.replace_in_place("@endcode", "```\n");

		// name block
		doc_comment.replace_in_place("@name ", "");

		// snippets
		static SNIPPET: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@snippet\s+([\w/.]+)\s+([\w-]+)").unwrap());
		doc_comment.replace_in_place_regex_cb(&SNIPPET, |comment, caps| {
			let path = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
			let name = caps.get(2).map(|(s, e)| &comment[s..e]).expect("Impossible");
			Some(
				if path.starts_with("samples/") {
					// fixme: hack to detect hdf snippets
					format!("[{name}](https://github.com/opencv/opencv_contrib/blob/{opencv_version}/modules/hdf/{path}#L1)")
				} else {
					format!("[{name}](https://github.com/opencv/opencv/blob/{opencv_version}/samples/cpp/tutorial_code/{path}#L1)")
				}
				.into(),
			)
		});

		// some special casing for docs.rs build failures
		doc_comment.replace_in_place("'fps'", r#""fps""#);
		doc_comment.replace_in_place("'cv::Exception'", r#""cv::Exception""#);

		// see also block
		static SEE_ALSO_BLOCK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*@(sa|see)\s+").unwrap());
		static SEE_ALSO_INLINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@(sa|see)\s+").unwrap());
		if doc_comment.replacen_in_place_regex(&SEE_ALSO_BLOCK, 1, "## See also\n") {
			doc_comment.replace_in_place_regex(&SEE_ALSO_INLINE, "");
		} else {
			doc_comment.replace_in_place_regex(&SEE_ALSO_INLINE, "see also: ");
		}

		// citation links
		static CITE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@cite\s+([\w:]+)").unwrap());
		doc_comment.replace_in_place_regex(
			&CITE,
			&format!("[$1](https://docs.opencv.org/{opencv_version}/d0/de3/citelist.html#CITEREF_$1)"),
		);

		// references
		static REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"@ref\s+([\w:]+)").unwrap());
		doc_comment.replace_in_place_regex(&REF, "[$1]");

		static REF_2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"#(\w+)(\s+)").unwrap());
		doc_comment.replace_in_place_regex_cb(&REF_2, |comment, caps| {
			let name = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
			let space = caps.get(2).map(|(s, e)| &comment[s..e]).expect("Impossible");
			let name = if name.contains(char::is_lowercase) {
				name.cpp_name_to_rust_fn_case()
			} else {
				Cow::Borrowed(name)
			};
			Some(format!("[{name}]{space}").into())
		});

		// images
		static IMAGE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"!\[(.*?)]\((?:.*/)?(.+)?\)").unwrap());
		doc_comment.replace_in_place_regex(&IMAGE, &format!("![$1](https://docs.opencv.org/{opencv_version}/$2)"));

		// returns
		static RETURNS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".*?@returns?\s*").unwrap());
		doc_comment.replace_in_place_regex(&RETURNS, "## Returns\n");

		// parameter list
		static PARAM_HEADER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?m)^(.*?@param)"#).unwrap());
		static PARAM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".*?@param\s*(?:\[in]|(\[out]))?\s+(\w+) *(.*)").unwrap());
		doc_comment.replacen_in_place_regex(&PARAM_HEADER, 1, "## Parameters\n$1");
		doc_comment.replace_in_place_regex(&PARAM, "* $2:$1 $3");

		// deprecated
		static DEPRECATED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".*?@deprecated\s+(.+)").unwrap());
		let mut deprecated = None;
		doc_comment.replace_in_place_regex_cb(&DEPRECATED, |comment, caps| {
			let deprecated_msg = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible").to_string();
			let out = format!("\n**Deprecated**: {deprecated_msg}");
			deprecated = Some(deprecated_msg);
			Some(out.into())
		});

		// leading dashes
		static LEADING_DASH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^(\s*)-(\s{2,})").unwrap());
		doc_comment.replace_in_place_regex(&LEADING_DASH, "$1*$2");

		// math expressions
		static BLOCK_FORMULA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)\\f\[(.*?)\\f]").unwrap());
		static INLINE_FORMULA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)\\f\$(.*?)\\f\$").unwrap());
		doc_comment.replace_in_place_regex_cb(&BLOCK_FORMULA, |out, caps| {
			let formula = preprocess_formula(caps.get(1).map(|(s, e)| &out[s..e]).expect("Impossible"));
			let encoded = utf8_percent_encode(&formula, NON_ALPHANUMERIC);
			Some(format!("![block formula](https://latex.codecogs.com/png.latex?{encoded})").into())
		});
		doc_comment.replace_in_place_regex_cb(&INLINE_FORMULA, |out, caps| {
			let formula = preprocess_formula(caps.get(1).map(|(s, e)| &out[s..e]).expect("Impossible"));
			let encoded = utf8_percent_encode(&formula, NON_ALPHANUMERIC);
			Some(format!("![inline formula](https://latex.codecogs.com/png.latex?{encoded})").into())
		});

		// separate urls
		static URL: LazyLock<Regex> = LazyLock::new(|| {
			Regex::new(r#"([^<"/(]|[^]]\(|^)(https?://[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_+.~#?&/=]*[-a-zA-Z0-9@:%_+~#?&/=])?)"#).unwrap()
		});
		doc_comment.replace_in_place_regex(&URL, "$1<$2>");

		// escapes
		static ESCAPE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)\\n$").unwrap());
		doc_comment.replace_in_place_regex(&ESCAPE, "\n");

		// catch sequences of 4 indents and reduce them to avoid cargo test running them as code
		static INDENTS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^(\s{3}|\s{7}|\s{11}|\s{15}|\s{19})\s(\S)").unwrap());
		doc_comment.replace_in_place_regex(&INDENTS, "$1$2");

		let mut attributes = vec![];
		if let Some(deprecated) = deprecated {
			attributes.push(format!("#[deprecated = \"{deprecated}\"]"));
		}
		Self { doc_comment, attributes }
	}

	pub fn render_with_comment_marker(&self, comment_marker: &str) -> Cow<'_, str> {
		let mut out = add_comment_markers(&self.doc_comment, comment_marker);
		if !out.is_empty() && !self.attributes.is_empty() {
			out.to_mut().push('\n');
		}
		for attr in &self.attributes {
			out.to_mut().push_str(attr);
		}
		out
	}
}

fn preprocess_formula(formula: &str) -> String {
	const ARG_REGEX: &str = r"\s*\{([^}]*?)\}";
	static MACROS: LazyLock<[(Regex, &str); 9]> = LazyLock::new(|| {
		[
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

fn add_comment_markers<'c>(comment: &'c str, marker: &str) -> Cow<'c, str> {
	if comment.is_empty() || marker.is_empty() {
		comment.into()
	} else {
		comment
			.lines_with_nl()
			.fold(
				String::with_capacity(comment.len() + (marker.len() + 1) * 128),
				|mut out_prefixed, line| {
					out_prefixed.push_str(marker);
					// there is more than just a newline in the buffer
					if line.len() > 1 {
						out_prefixed.push(' ');
					}
					out_prefixed + line
				},
			)
			.into()
	}
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

pub fn render_ref<'r>(referenced: &'r Func, force_cpp_name: Option<&'r str>) -> Cow<'r, str> {
	match referenced.kind().as_ref() {
		FuncKind::Function | FuncKind::GenericFunction => force_cpp_name
			.map_or_else(|| referenced.cpp_name(CppNameStyle::Declaration), Cow::Borrowed)
			.map_borrowed(|s| s.cpp_name_to_rust_fn_case()),
		FuncKind::FunctionOperator(_) | FuncKind::Constructor(_) => referenced.rust_leafname(FishStyle::No),
		FuncKind::InstanceMethod(cls) | FuncKind::GenericInstanceMethod(cls) => format!(
			"{}::{}",
			cls.rust_trait_name(NameStyle::Declaration, referenced.constness()),
			force_cpp_name
				.map_or_else(|| referenced.cpp_name(CppNameStyle::Declaration), Cow::Borrowed)
				.cpp_name_to_rust_fn_case()
		)
		.into(),
		FuncKind::StaticMethod(cls) => format!(
			"{}::{}",
			cls.rust_name(NameStyle::Declaration),
			force_cpp_name
				.map_or_else(|| referenced.cpp_name(CppNameStyle::Declaration), Cow::Borrowed)
				.cpp_name_to_rust_fn_case()
		)
		.into(),
		FuncKind::ConversionMethod(cls) | FuncKind::InstanceOperator(cls, _) | FuncKind::FieldAccessor(cls, _) => format!(
			"{}::{}",
			cls.rust_trait_name(NameStyle::Declaration, referenced.constness()),
			referenced.rust_leafname(FishStyle::No)
		)
		.into(),
	}
}

#[cfg(test)]
mod test {
	use super::RenderComment;
	use crate::comment::strip_doxygen_comment_markers;

	#[test]
	fn test_render_doc_comment() {
		{
			let comment = "@deprecated test\r\ntest\r\n";
			assert!(comment.contains('\r'));
			let res = RenderComment {
				doc_comment: "\
\n**Deprecated**: test
test"
					.to_string(),
				attributes: vec!["#[deprecated = \"test\"]".to_string()],
			};
			assert_eq!(res, RenderComment::new(strip_doxygen_comment_markers(comment), "master"));
		}

		{
			let comment = "";
			let res = RenderComment {
				doc_comment: "".to_string(),
				attributes: vec![],
			};
			assert_eq!(res, RenderComment::new(comment.to_string(), "master"))
		}
	}

	#[test]
	fn render_with_prefix() {
		{
			assert_eq!(
				"",
				RenderComment {
					doc_comment: "".to_string(),
					attributes: vec![]
				}
				.render_with_comment_marker("")
			);
		}

		{
			assert_eq!(
				"/// test1\n/// test2",
				RenderComment {
					doc_comment: "test1\ntest2".to_string(),
					attributes: vec![]
				}
				.render_with_comment_marker("///")
			);
		}

		{
			assert_eq!(
				"/// test1\n/// test2\n#[deprecated]",
				RenderComment {
					doc_comment: "test1\ntest2".to_string(),
					attributes: vec!["#[deprecated]".to_string()]
				}
				.render_with_comment_marker("///")
			);
		}
	}
}
