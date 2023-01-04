use crate::string_ext::Indent;
use crate::StrExt;

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
				*last_line = last_line[..last_line.len() - MULTILINE_SUFFIX.len()].trim_end();
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
