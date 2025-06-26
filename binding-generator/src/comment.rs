use crate::string_ext::Indent;
use crate::StrExt;

pub fn strip_doxygen_comment_markers(comment: &str) -> String {
	// todo, simplify/optimize this function, spec is here https://www.doxygen.nl/manual/docblocks.html
	const MULTILINE_PREFIX: &str = "/*";
	const MULTILINE_CONT: &str = "*";
	const MULTILINE_SUFFIX: &str = "*/";
	const SINGLELINE: &str = "//";
	const DETAIL: &str = "!";
	const SINGLELINE_DETAIL: &str = "/";
	const SINGLELINE_SIDE: &str = "<";

	fn trim_last_empty_lines(lines: &mut Vec<&str>) {
		while lines.last().is_some_and(|line| line.is_empty()) {
			lines.pop();
		}
	}

	let mut comment_type = CommentType::SingleLineDelimited;
	let mut lines = Vec::with_capacity(128);
	// first pass:
	// 1. checks whether a comment is single-line or multi-line delimited
	// 2. checks whether a multiline comment is asterisk prefixed
	// 3. strips comment delimiters for multiline and single line comments
	// 4. collects resulting stripped lines into `lines` Vec
	for (i, mut line) in comment.lines_with_nl().enumerate() {
		let mut line_trimmed = line.trim_start();
		if i == 0 {
			if let Some(new_line) = line_trimmed.strip_prefix(MULTILINE_PREFIX) {
				line = new_line;
				if let Some(new_line) = line.strip_prefix(MULTILINE_CONT) {
					line = new_line;
					comment_type = CommentType::MultilineAsteriskPrefixed;
				} else {
					comment_type = CommentType::MultilineWithoutAsteriskPrefix;
				}
				line_trimmed = line.trim_start();
			}
		} else if let Some(line_clean) = line_trimmed.strip_prefix(MULTILINE_PREFIX) {
			line = line_clean
				.trim_start_matches(DETAIL)
				.trim_start_matches(MULTILINE_CONT)
				.trim_start();
		}
		if comment_type == CommentType::SingleLineDelimited {
			if let Some(new_line) = line_trimmed.strip_prefix(SINGLELINE) {
				line = new_line
					.strip_prefix(SINGLELINE_DETAIL)
					.or_else(|| new_line.strip_prefix(DETAIL))
					.unwrap_or(new_line);
				line = line.strip_prefix(SINGLELINE_SIDE).unwrap_or(line);
			}
		} else if i == 1 && comment_type == CommentType::MultilineAsteriskPrefixed && !line_trimmed.starts_with(MULTILINE_CONT) {
			comment_type = CommentType::MultilineWithoutAsteriskPrefix;
		}
		lines.push(line);
	}
	trim_last_empty_lines(&mut lines);
	// trim ending multiline delimiter
	if let Some(last_line) = lines.last_mut() {
		if comment_type != CommentType::SingleLineDelimited {
			*last_line = last_line.trim_end();
			if let Some(new_line) = last_line.strip_suffix(MULTILINE_SUFFIX) {
				*last_line = new_line.trim_end();
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
		if comment_type == CommentType::MultilineAsteriskPrefixed {
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

#[derive(PartialEq)]
enum CommentType {
	/// Each line starts with `//`
	SingleLineDelimited,
	/// Comment starting with `/*` or `/**` and every remaining line within starts with `*`
	MultilineAsteriskPrefixed,
	/// Comment starting with `/*` or `/**`, but without any prefix for the remaining lines
	MultilineWithoutAsteriskPrefix,
}

#[cfg(test)]
mod test {
	use super::strip_doxygen_comment_markers;

	#[test]
	fn test_strip_comment_markers() {
		assert_eq!("test", &strip_doxygen_comment_markers("/** test */"));
		assert_eq!("test", &strip_doxygen_comment_markers("//   test"));
		assert_eq!("test", &strip_doxygen_comment_markers("/*test */"));

		// multiline with prefix
		{
			let comment = "\
/**
 * line1
 * line2
 */
";
			let expected = "\
line1
line2";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline without prefix
		{
			let comment = "\
/*
line1
 line2
*/
";
			let expected = "\
line1
 line2";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline with indent and no end marker
		{
			let comment = "\
/** line1
 *     line2
 *     line3
";
			let expected = "\
line1
line2
line3";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline with MD unordered list
		{
			let comment = "\
/* line1
      * line2
        line3
      * line4
*/
";
			let expected = "\
line1
* line2
  line3
* line4";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline with indent on a single line
		{
			let comment = "\
/**
 * line1
 * line2
                          line3
 */
";
			let expected = "\
line1
line2
                       line3";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline, end marker on the same line
		{
			let comment = "\
/** line1
    line2
    line3*/
";
			let expected = "\
line1
line2
line3";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// two multiline glued
		{
			let comment = "\
/** line1
  line2*/
/** line3 */
";
			let expected = "\
line1
  line2
line3";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline with empty lines inside and at the end
		{
			let comment = "\
/** line1

line2

*/
";
			let expected = "\
line1

line2";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline with empty lines and indent
		{
			let comment = "\
/**
 line1

 line2
*/
";
			let expected = "\
line1

line2";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// multiline with indent when the first line is on the same line as the start marker
		{
			let comment = "\
/** line1

 line2
 */
";
			let expected = "\
line1

line2";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// single-line with exclamation mark
		{
			let comment = "\
//!< line1
      //!< line2
";
			let expected = "\
line1
line2";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}

		// single-line
		{
			let comment = "// @overload
// @brief It's the same function as #calibrateCameraAruco but without calibration error estimation.
//";

			let expected = "@overload
@brief It's the same function as #calibrateCameraAruco but without calibration error estimation.";
			assert_eq!(expected, &strip_doxygen_comment_markers(comment));
		}
	}
}
