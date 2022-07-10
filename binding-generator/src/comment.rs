use std::fmt::Write;

use once_cell::sync::Lazy;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use regex::Regex;

use crate::{StrExt, string_ext::Indent, StringExt};

pub fn strip_comment_markers(comment: &str) -> String {
	const MULTILINE_PREFIX: &str = "/*";
	const MULTILINE_CONT: &str = "*";
	const MULTILINE_SUFFIX: &str = "*/";
	const SINGLELINE: &str = "//";
	const DETAIL: &str = "!";
	const SINGLELINE_DETAIL: &str = "/";
	const SINGLELINE_SIDE: &str = "<";
	let mut singleline_delimited = true;
	let mut asterisk_indented = false;
	let mut lines = Vec::with_capacity(128);
	// first pass:
	// 1. checks whether the comment is single-line or multi-line delimited
	// 2. checks whether multiline comment is asterisk prefixed
	// 3. strips comment delimiters for multiline and single line comments
	// 4. collects resulting stripped lines into `lines` Vec
	for (i, mut line) in comment.lines_with_nl().enumerate() {
		let mut line_clean = line.trim_start();
		if i == 0 {
			if let Some(new_line) = line_clean.strip_prefix(MULTILINE_PREFIX) {
				line = new_line;
				singleline_delimited = false;
				if let Some(new_line) = line.strip_prefix(MULTILINE_CONT) {
					line = new_line;
					asterisk_indented = true;
				} else {
					asterisk_indented = false;
				}
				line_clean = line.trim_start();
			} else {
				singleline_delimited = true;
				asterisk_indented = false;
			}
		} else if let Some(line_clean) = line_clean.strip_prefix(MULTILINE_PREFIX) {
			line = line_clean
				.trim_start_matches(DETAIL)
				.trim_start_matches(MULTILINE_CONT)
				.trim_start();
		}
		if singleline_delimited && line_clean.starts_with(SINGLELINE) {
			line = &line_clean[SINGLELINE.len()..];
			if let Some(new_line) = line.strip_prefix(SINGLELINE_DETAIL) {
				line = new_line;
			} else if let Some(new_line) = line.strip_prefix(DETAIL) {
				line = new_line;
			}
			if let Some(new_line) = line.strip_prefix(SINGLELINE_SIDE) {
				line = new_line;
			}
		} else if asterisk_indented && i == 1 && !line_clean.starts_with(MULTILINE_CONT) {
			asterisk_indented = false;
		}
		lines.push(line);
	}
	let trim_last_empty_lines = |lines: &mut Vec<&str>| {
		while let Some(last_line) = lines.last() {
			if last_line.is_empty() {
				lines.pop();
			} else {
				break;
			}
		}
	};
	trim_last_empty_lines(&mut lines);
	// trim ending multiline delimiter
	if let Some(last_line) = lines.last_mut() {
		if !singleline_delimited {
			*last_line = last_line.trim_end();
			if last_line.ends_with(MULTILINE_SUFFIX) {
				*last_line = last_line[..last_line.len() - MULTILINE_SUFFIX.len()]
					.trim_end();
			}
		}
	}
	trim_last_empty_lines(&mut lines);
	// second pass:
	// 1. calculates common indent
	// 2. for multiline asterisk prefixed comments, strips this prefix modifying `lines`
	let mut first_line_indent = None;
	let mut common_indent: Option<Indent> = None;
	for line in &mut lines {
		if !singleline_delimited && asterisk_indented {
			let line_trimmed = line.trim_start();
			if let Some(line_trimmed) = line_trimmed.strip_prefix(MULTILINE_CONT) {
				*line = line_trimmed;
			} else {
				let trim_start = line.trim_start_idx().min(2);
				*line = &line[trim_start..];
			}
		}
		if first_line_indent.is_none() {
			first_line_indent = Some(line.detect_indent());
		} else {
			let detected_indent = line.detect_indent();
			if !line[detected_indent.len..].trim_start().is_empty() {
				if let Some(common_indent) = common_indent.as_mut() {
					*common_indent = (*common_indent).min(detected_indent);
				} else {
					common_indent = Some(line.detect_indent());
				}
			}
		}
	}
	let mut out = String::with_capacity(comment.len());
	for (i, mut line) in lines.into_iter().enumerate() {
		if i == 0 {
			line = &line[first_line_indent.unwrap_or_default().len..];
			if line.trim().is_empty() {
				continue;
			}
		} else {
			let indent_len = common_indent.unwrap_or_default().len;
			if line.len() > indent_len {
				line = &line[indent_len..];
			} else {
				line = "\n";
			}
		}
		let line_clean_end = line.trim_end();
		if let Some(suffix) = line_clean_end.strip_suffix(MULTILINE_SUFFIX) {
			out += suffix.trim_end();
			out.push('\n');
		} else {
			out += line;
		}
	}

	let out_trim_end = out.trim_end_idx();
	out.drain(out_trim_end..);
	out
}

fn preprocess_formula(formula: &str) -> String {
	const ARG_REGEX: &str = r"\s*\{([^}]*?)\}";
	static MACROS: Lazy<Vec<(Regex, &str)>> = Lazy::new(|| vec![
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
			Regex::new(&format!("\\\\hdotsfor{}", ARG_REGEX)).unwrap(),
			"\\dots",
		),
		(
			Regex::new(&format!("\\\\mathbbm{}", ARG_REGEX)).unwrap(),
			"\\mathbb{$1}",
		),
		(
			Regex::new(&format!("\\\\bordermatrix{}", ARG_REGEX.repeat(9))).unwrap(),
			"\\matrix{$1}",
		),
	]);

	let mut out = formula.to_string();
	for (re, repl) in &*MACROS {
		out.replace_in_place_regex(re, *repl);
	}
	out
}

pub fn render_doc_comment(doc_comment: &str, prefix: &str, opencv_version: &str) -> String {
	render_doc_comment_with_processor(doc_comment, prefix, opencv_version, |_| {})
}

pub fn render_doc_comment_with_processor(doc_comment: &str, prefix: &str, opencv_version: &str, mut post_processor: impl FnMut(&mut String)) -> String {
	let mut out = strip_comment_markers(doc_comment);
	out.replace_in_place("\r\n", "\n");
	// module titles
	static MODULE_TITLE_1: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)\s*@[}{].*$"#).unwrap());
	static MODULE_TITLE_2: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@defgroup [^ ]+ (.*)"#).unwrap());
	static MODULE_TITLE_3: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^.*?@addtogroup\s+.+"#).unwrap());
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
	static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@code(?: ?\{.+?})?"#).unwrap());
	out.replace_in_place_regex(&CODE, "```ignore");
	out.replace_in_place("@endcode", "```\n");

	// snippets
	static SNIPPET: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@snippet\s+([\w/.]+)\s+([\w-]+)"#).unwrap());
	out.replace_in_place_regex_cb(&SNIPPET, |s, caps| {
		let (path_start, path_end) = caps.get(1).expect("Impossible");
		let path = &s[path_start..path_end];
		let (name_start, name_end) = caps.get(2).expect("Impossible");
		if path.starts_with("samples/") { // fixme: hack to detect hdf snippets
			Some(format!(
				"[{name}](https://github.com/opencv/opencv_contrib/blob/{version}/modules/hdf/{path}#L1)",
				name=&s[name_start..name_end],
				version=opencv_version,
				path=path,
			).into())
		} else {
			Some(format!(
				"[{name}](https://github.com/opencv/opencv/blob/{version}/samples/cpp/tutorial_code/{path}#L1)",
				name=&s[name_start..name_end],
				version=opencv_version,
				path=path,
			).into())
		}
	});

	// some special casing for docs.rs build failures
	out.replace_in_place("'fps'", r#""fps""#);
	out.replace_in_place("'cv::Exception'", r#""cv::Exception""#);

	// see also block
	static SEE_ALSO_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^\s*@(sa|see)\s+"#).unwrap());
	static SEE_ALSO_INLINE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@(sa|see)\s+"#).unwrap());
	if out.replacen_in_place_regex(&SEE_ALSO_BLOCK, 1, "## See also\n") {
		out.replace_in_place_regex(&SEE_ALSO_INLINE, "");
	} else {
		out.replace_in_place_regex(&SEE_ALSO_INLINE, "see also: ");
	}

	// citation links
	static CITE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@cite\s+([\w:]+)"#).unwrap());
	out.replace_in_place_regex(&CITE, &format!("[$1](https://docs.opencv.org/{}/d0/de3/citelist.html#CITEREF_$1)", opencv_version));

	// images
	static IMAGE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"!\[(.*?)]\((?:.*/)?(.+)?\)"#).unwrap());
	out.replace_in_place_regex(&IMAGE, &format!("![$1](https://docs.opencv.org/{}/$2)", opencv_version));

	// returns
	static RETURNS: Lazy<Regex> = Lazy::new(|| Regex::new(r#".*?@returns?\s*"#).unwrap());
	out.replace_in_place_regex(&RETURNS, "## Returns\n");

	// parameter list
	static PARAM_HEADER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^(.*?@param)"#).unwrap());
	static PARAM: Lazy<Regex> = Lazy::new(|| Regex::new(r#".*?@param\s*(?:\[in]|(\[out]))?\s+(\w+) *(.*)"#).unwrap());
	out.replacen_in_place_regex(&PARAM_HEADER, 1, "## Parameters\n$1");
	out.replace_in_place_regex(&PARAM, "* $2:$1 $3");
	// deprecated
	static DEPRECATED: Lazy<Regex> = Lazy::new(|| Regex::new(r#".*?@deprecated\s+(.+)"#).unwrap());
	let mut deprecated = None;
	out.replace_in_place_regex_cb(&DEPRECATED, |out, caps| {
		let (cap_start, cap_end) = caps.get(1).expect("Impossible");
		let deprecated_msg = out[cap_start..cap_end].to_string();
		let out = format!("\n**Deprecated**: {}", deprecated_msg);
		deprecated = Some(deprecated_msg);
		Some(out.into())
	});

	// leading dashes
	static LEADING_DASH: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^(\s*)-(\s{2,})"#).unwrap());
	out.replace_in_place_regex(&LEADING_DASH, "$1*$2");

	// math expressions
	static BLOCK_FORMULA: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?s)\\f\[(.*?)\\f]"#).unwrap());
	static INLINE_FORMULA: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?s)\\f\$(.*?)\\f\$"#).unwrap());
	out.replace_in_place_regex_cb(&BLOCK_FORMULA, |out, caps| {
		let (cap_start, cap_end) = caps.get(1).expect("Impossible");
		let formula = preprocess_formula(&out[cap_start..cap_end]);
		let encoded = utf8_percent_encode(&formula, NON_ALPHANUMERIC);
		Some(format!("![block formula](https://latex.codecogs.com/png.latex?{})", encoded).into())
	});
	out.replace_in_place_regex_cb(&INLINE_FORMULA, |out, caps| {
		let (cap_start, cap_end) = caps.get(1).expect("Impossible");
		let formula = preprocess_formula(&out[cap_start..cap_end]);
		let encoded = utf8_percent_encode(&formula, NON_ALPHANUMERIC);
		Some(format!("![inline formula](https://latex.codecogs.com/png.latex?{})", encoded).into())
	});

	// escapes
	static ESCAPE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)\\n$"#).unwrap());
	out.replace_in_place_regex(&ESCAPE, "\n");

	// catch sequences of 4 indents and reduce them to avoid cargo test running them as code
	static INDENTS: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)^(\s{3}|\s{7}|\s{11}|\s{15}|\s{19})\s(\S)"#).unwrap());
	out.replace_in_place_regex(&INDENTS, "$1$2");

	post_processor(&mut out);

	let mut out = if out.is_empty() || prefix.is_empty() {
		out
	} else {
		out.lines_with_nl()
			.fold(
				String::with_capacity(out.len() + (prefix.len() + 1) * 128),
				|mut out_prefixed, line| {
					out_prefixed.push_str(prefix);
					out_prefixed.push(' ');
					out_prefixed + line
				},
			)
	};
	if let Some(deprecated) = deprecated {
		write!(&mut out, "\n#[deprecated = \"{}\"]", deprecated).expect("write! to String shouldn't fail");
	}
	out
}
