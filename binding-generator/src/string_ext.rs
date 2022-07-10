use std::{
	borrow::Cow,
	collections::HashMap,
	iter,
};

use once_cell::sync::Lazy;
use regex::{CaptureLocations, Regex};

pub trait StringExt {
	fn replacen_in_place(&mut self, from: &str, limit: usize, to: &str) -> bool;
	fn replace_in_place(&mut self, from: &str, to: &str) -> bool;
	fn replacen_in_place_regex(&mut self, from: &Regex, limit: usize, to: &str) -> bool;
	fn replace_in_place_regex(&mut self, from: &Regex, to: &str) -> bool;
	fn replacen_in_place_regex_cb<'a>(&mut self, from: &Regex, limit: usize, replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a) -> bool;
	fn replace_in_place_regex_cb<'a>(&mut self, from: &Regex, replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a) -> bool;
	fn extend_join(&mut self, it: impl Iterator<Item=impl AsRef<str>>, sep: &str);
	fn push_indented_str(&mut self, indent: Indent, val: &str);
	fn bump_counter(&mut self);
	fn cleanup_name(&mut self);
}

impl StringExt for String {
	fn replacen_in_place(&mut self, from: &str, limit: usize, to: &str) -> bool {
		if from.is_empty() {
			return false;
		}
		let mut idx = 0;
		let mut count = 0;
		while let Some(start_idx) = self[idx..].find(from).map(|i| i + idx) {
			let end_idx = start_idx + from.len();
			self.replace_range(start_idx..end_idx, to);
			idx = start_idx + to.len();
			count += 1;
			if count == limit {
				break;
			}
		}
		count != 0
	}

	fn replace_in_place(&mut self, from: &str, to: &str) -> bool {
		self.replacen_in_place(from, 0, to)
	}

	fn replacen_in_place_regex(&mut self, from: &Regex, limit: usize, to: &str) -> bool {
		let mut idx = 0;
		if to.chars().any(|c| c == '$') {
			enum Elem<'a> {
				CaptureGroup(usize),
				Literal(&'a str),
			}

			#[inline(always)]
			fn compile_captures(rep: &str) -> Vec<Elem> {
				let mut out = Vec::with_capacity(10);
				let mut last_idx = 0;
				for (idx, _) in rep.match_indices('$') {
					if let Some((mut next_idx, next_char)) = rep[idx..].char_indices().nth(1) {
						next_idx += idx;
						if next_char == '$' {
							out.push(Elem::Literal(&rep[last_idx..next_idx]));
							last_idx = next_idx + 1;
							continue
						}
						if let Some(mut num_end_idx) = rep[next_idx..].char_indices().take_while(|(_, c)| c.is_ascii_digit()).map(|(i, _)| i).last() {
							num_end_idx += next_idx + 1;
							out.push(Elem::Literal(&rep[last_idx..idx]));
							out.push(Elem::CaptureGroup(rep[next_idx..num_end_idx].parse().expect("Can't parse as group number")));
							last_idx = num_end_idx;
						}
					} else {
						break
					}
				}
				out.push(Elem::Literal(&rep[last_idx..]));
				out
			}

			let rep = compile_captures(to);
			self.replacen_in_place_regex_cb(from, limit, |s, caps| {
				let cap_len = rep.iter().fold(0, |acc, x| {
					acc + match x {
						Elem::CaptureGroup(n) => {
							if let Some((start, end)) = caps.get(*n) {
								end - start
							} else {
								0
							}
						},
						Elem::Literal(s) => {
							s.len()
						},
					}
				});
				let out = rep.iter().fold(String::with_capacity(cap_len), |out, x| {
					out + match x {
						Elem::CaptureGroup(n) => {
							if let Some((start, end)) = caps.get(*n) {
								&s[start..end]
							} else {
								""
							}
						},
						Elem::Literal(s) => {
							s
						},
					}
				});
				Some(out.into())
			})
		} else {
			let mut count = 0;
			while let Some((start_idx, end_idx)) = from.find_at(self, idx).map(|m| (m.start(), m.end())) {
				if start_idx == end_idx {
					return false;
				}
				self.replace_range(start_idx..end_idx, to);
				idx = start_idx + to.len();
				count += 1;
				if count == limit {
					break;
				}
			}
			count != 0
		}
	}

	fn replace_in_place_regex(&mut self, from: &Regex, to: &str) -> bool {
		self.replacen_in_place_regex(from, 0, to)
	}

	fn replacen_in_place_regex_cb<'a>(&mut self, from: &Regex, limit: usize, mut replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a) -> bool {
		let mut idx = 0;
		let mut caps = from.capture_locations();
		let mut count = 0;
		while let Some((start_idx, end_idx)) = from.captures_read_at(&mut caps, self, idx).map(|m| (m.start(), m.end())) {
			if start_idx == end_idx {
				return false;
			}
			if let Some(repl) = replacer(self, &caps) {
				self.replace_range(start_idx..end_idx, &repl);
				idx = start_idx + repl.len();
			} else {
				idx = end_idx;
			}
			count += 1;
			if count == limit {
				break;
			}
		}
		count != 0
	}

	fn replace_in_place_regex_cb<'a>(&mut self, from: &Regex, replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a) -> bool {
		self.replacen_in_place_regex_cb(from, 0, replacer)
	}

	fn extend_join(&mut self, it: impl IntoIterator<Item=impl AsRef<str>>, sep: &str) {
		let mut it = it.into_iter();
		if let Some(first) = it.next() {
			let first = first.as_ref();
			let needed_cap = it.size_hint().1.unwrap_or(16) + first.len();
			if needed_cap > self.capacity() {
				self.reserve(needed_cap - self.capacity());
			}
			self.push_str(first);
			it.for_each(|part| {
				self.push_str(sep);
				self.push_str(part.as_ref());
			})
		}
	}

	fn push_indented_str(&mut self, indent: Indent, val: &str) {
		let mut lines = val.lines_with_nl();
		if let Some(line) = lines.next() {
			self.push_str(line);
		}
		for line in lines {
			self.extend(iter::repeat(indent.symbol).take(indent.len));
			self.push_str(line);
		}
	}

	fn bump_counter(&mut self) {
		let idx = self.rfind(|c: char| !c.is_ascii_digit()).map_or_else(|| self.len(), |idx| idx + 1);
		let counter = &self[idx..];
		if counter.is_empty() || self.as_bytes()[idx - 1] != b'_' {
			self.push_str("_1")
		} else {
			let counter: u32 = counter.parse().unwrap_or(0) + 1;
			self.replace_range(idx.., &counter.to_string());
		}
	}

	fn cleanup_name(&mut self) {
		// todo aho-corasick?
		self.replace_in_place(" ", "_");
		self.replace_in_place("<", "_");
		self.replace_in_place(">", "_");
		self.replace_in_place("(", "_");
		self.replace_in_place(")", "_");
		self.replace_in_place("*", "X");
		self.replace_in_place("&", "R");
		self.replace_in_place(",", "_");
		self.replace_in_place("[", "_");
		self.replace_in_place("]", "_");
		self.replace_in_place("::", "_");
		self.replace_in_place("+", "A");
		self.replace_in_place("-", "S");
		self.replace_in_place("/", "D");
		self.replace_in_place("==", "EQ");
	}
}

pub struct LinesWithNl<'s> {
	string: &'s str,
	len: usize,
	idx: usize,
}

impl<'s> Iterator for LinesWithNl<'s> {
	type Item = &'s str;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx > self.len {
			None
		} else {
			let slice = &self.string[self.idx..];
			Some(if let Some(new_line_idx) = slice.find(|c| c == '\n') {
				self.idx += new_line_idx + 1;
				&slice[..=new_line_idx]
			} else {
				self.idx = self.len + 1;
				slice
			})
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Indent {
	pub len: usize,
	pub symbol: char,
}

impl Default for Indent {
	fn default() -> Self {
		Self { len: 0, symbol: '\t' }
	}
}

#[derive(Clone, Copy, Debug)]
enum Compiled<'s> {
	IntpLineStart(&'s str),
	IntpLiteral(&'s str),
	IntpLineEnd(&'s str),
	LiteralLine(&'s str),
	Var(&'s str),
}

#[derive(Clone, Debug)]
pub struct CompiledInterpolation<'s> {
	elems: Vec<Compiled<'s>>,
}

impl CompiledInterpolation<'_> {
	pub fn interpolate(&self, params: &HashMap<&str, impl AsRef<str>>) -> String {
		#[inline(always)]
		fn remove_trailing_empty_line(out: &mut String) -> bool {
			let last_line_start = out.rfind('\n').map_or(0, |i| i + 1);
			if out[last_line_start..].chars().all(char::is_whitespace) {
				out.drain(last_line_start..);
				true
			} else {
				false
			}
		}

		const INVALID_PARAM_NAME: &str = "<parameter not found>";

		let result_len = self.elems.iter()
			.fold(0, |len, elem| len + match elem {
				Compiled::IntpLineStart(s) | Compiled::IntpLiteral(s) => s.len(),
				Compiled::IntpLineEnd(s) | Compiled::LiteralLine(s) => s.len() + 1,
				Compiled::Var(name) => params.get(name)
					.map_or_else(|| INVALID_PARAM_NAME.len(), |x| x.as_ref().len())
			});
		let mut out = String::with_capacity(result_len);
		let mut line_indent = Indent::default();
		// interpolate vars keeping indent
		for elem in &self.elems {
			match elem {
				Compiled::IntpLineStart(s) => {
					line_indent = s.detect_indent();
					out += s;
				}
				Compiled::IntpLiteral(s) =>
					out += s,
				Compiled::Var(name) => {
					out.push_indented_str(
						line_indent,
						params.get(name)
							.map_or(INVALID_PARAM_NAME, |x| x.as_ref()),
					)
				}
				Compiled::IntpLineEnd(s) => {
					out += s;
					if !remove_trailing_empty_line(&mut out) {
						out.push('\n');
					}
				}
				Compiled::LiteralLine(s) => {
					line_indent = s.detect_indent();
					out += s;
					out.push('\n');
				}
			}
		}
		if let Some('\n') = out.chars().rev().next() {
			out.pop();
		}
		out
	}
}

pub trait StrExt {
	fn to_snake_case(&self) -> String;
	fn lines_with_nl(&self) -> LinesWithNl;
	fn detect_indent(&self) -> Indent;
	fn compile_interpolation(&self) -> CompiledInterpolation;
	fn trim_start_idx(&self) -> usize;
	fn trim_end_idx(&self) -> usize;
	fn localname(&self) -> &str;
	fn namespace(&self) -> &str;
}

impl StrExt for str {
	fn to_snake_case(&self) -> String {
		static R1: Lazy<Regex> = Lazy::new(|| Regex::new(r#"([^_])([A-Z][a-z]+)"#).expect("Can't compile regex"));
		let out = R1.replace_all(self, "${1}_$2");

		static R2: Lazy<Regex> = Lazy::new(|| Regex::new(r#"([a-z0-9])([A-Z])"#).expect("Can't compile regex"));
		let out = R2.replace_all(&out, "${1}_$2");

		static R3: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\B([23])_(D)\b"#).expect("Can't compile regex"));
		let out = R3.replace_all(&out, "_$1$2");

		static R4: Lazy<Regex> = Lazy::new(|| Regex::new(r#"_(P[n3])_(P)"#).expect("Can't compile regex"));
		let out = R4.replace_all(&out, "_$1$2");

		static R5: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Open_(CL|Gl|VX)"#).expect("Can't compile regex"));
		let out = R5.replace_all(&out, "Open$1");

		#[allow(clippy::trivial_regex)]
		static R6: Lazy<Regex> = Lazy::new(|| Regex::new(r#"U_Mat"#).expect("Can't compile regex"));
		let out = R6.replace_all(&out, "UMat");

		out.to_lowercase()
	}

	fn lines_with_nl(&self) -> LinesWithNl {
		LinesWithNl { string: self, len: self.len(), idx: 0 }
	}

	fn detect_indent(&self) -> Indent {
		self.char_indices()
			.take_while(|&(_, c)| c == ' ' || c == '\t')
			.last()
			.map(|(idx, chr)| Indent { len: idx + 1, symbol: chr })
			.unwrap_or_default()
	}

	fn compile_interpolation(&self) -> CompiledInterpolation {
		static VARS: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\{\{\s*([^{}]+?)\s*}}"#).expect("Can't compile regex"));

		// trim leading newline
		let tpl = self.strip_prefix('\n').unwrap_or(self);

		// find minimum common indent
		let mut common_indent_len: Option<usize> = None;
		for line in tpl.lines_with_nl() {
			let Indent { len: new_indent, .. } = if let Some(len) = common_indent_len {
				line[..len.min(line.len())].detect_indent()
			} else {
				line.detect_indent()
			};
			// only take lines with something else than only whitespace into account
			if !line[new_indent..].trim_start().is_empty() {
				common_indent_len = Some(new_indent);
			}
		}

		let mut elems = vec![];
		// interpolate vars keeping indent
		if let Some(common_indent_len) = common_indent_len {
			for line in tpl.lines() {
				let line = &line[common_indent_len.min(line.len())..];
				let mut last_idx = 0;
				for cap in VARS.captures_iter(line) {
					if let (Some(whole), Some(var)) = (cap.get(0), cap.get(1)) {
						if last_idx == 0 {
							elems.push(Compiled::IntpLineStart(&line[last_idx..whole.start()]));
						} else {
							elems.push(Compiled::IntpLiteral(&line[last_idx..whole.start()]));
						}
						last_idx = whole.end();
						elems.push(Compiled::Var(var.as_str()));
					}
				}
				if last_idx == 0 {
					elems.push(Compiled::LiteralLine(&line[last_idx..]));
				} else {
					elems.push(Compiled::IntpLineEnd(&line[last_idx..]));
				}
			}
		} else {
			elems.push(Compiled::LiteralLine(""));
		}

		CompiledInterpolation { elems }
	}

	fn trim_start_idx(&self) -> usize {
		self.char_indices()
			.find(|(_, c)| !c.is_whitespace())
			.map_or_else(|| self.len(), |(i, _)| i)
	}

	fn trim_end_idx(&self) -> usize {
		self.char_indices()
			.rfind(|(_, c)| !c.is_whitespace())
			.map_or(0, |(i, _)| i + 1)
	}

	fn localname(&self) -> &str {
		const SEP: &str = "::";
		if let Some(idx) = self.rfind(SEP) {
			&self[idx + SEP.len()..]
		} else {
			self
		}
	}

	fn namespace(&self) -> &str {
		if let Some(idx) = self.rfind("::") {
			&self[..idx]
		} else {
			self
		}
	}
}
