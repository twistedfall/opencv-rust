use std::borrow::Cow;
use std::collections::HashMap;
use std::iter;

use once_cell::sync::Lazy;
use regex::{CaptureLocations, Regex};

use crate::CppNameStyle;

pub trait StringExt {
	fn replacen_in_place(&mut self, from: &str, limit: usize, to: &str) -> bool;
	fn replace_in_place(&mut self, from: &str, to: &str) -> bool;
	fn replacen_in_place_regex(&mut self, from: &Regex, limit: usize, to: &str) -> bool;
	fn replace_in_place_regex(&mut self, from: &Regex, to: &str) -> bool;
	fn replacen_in_place_regex_cb<'a>(
		&mut self,
		from: &Regex,
		limit: usize,
		replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a,
	) -> bool;
	fn replace_in_place_regex_cb<'a>(
		&mut self,
		from: &Regex,
		replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a,
	) -> bool;
	fn extend_join(&mut self, it: impl Iterator<Item = impl AsRef<str>>, sep: &str);
	fn extend_sep(&mut self, sep: &str, s: &str);
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
							continue;
						}
						if let Some(mut num_end_idx) = rep[next_idx..]
							.char_indices()
							.take_while(|(_, c)| c.is_ascii_digit())
							.map(|(i, _)| i)
							.last()
						{
							num_end_idx += next_idx + 1;
							out.push(Elem::Literal(&rep[last_idx..idx]));
							out.push(Elem::CaptureGroup(
								rep[next_idx..num_end_idx].parse().expect("Can't parse as group number"),
							));
							last_idx = num_end_idx;
						}
					} else {
						break;
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
						}
						Elem::Literal(s) => s.len(),
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
						}
						Elem::Literal(s) => s,
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

	fn replacen_in_place_regex_cb<'a>(
		&mut self,
		from: &Regex,
		limit: usize,
		mut replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a,
	) -> bool {
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

	fn replace_in_place_regex_cb<'a>(
		&mut self,
		from: &Regex,
		replacer: impl FnMut(&str, &CaptureLocations) -> Option<Cow<'a, str>> + 'a,
	) -> bool {
		self.replacen_in_place_regex_cb(from, 0, replacer)
	}

	fn extend_join(&mut self, it: impl IntoIterator<Item = impl AsRef<str>>, sep: &str) {
		let mut it = it.into_iter();
		let first = it.find(|e| !e.as_ref().is_empty());
		if let Some(first) = first {
			let first = first.as_ref();
			if !first.is_empty() {
				let needed_cap = it.size_hint().1.unwrap_or(8) * (first.len() + sep.len());
				if needed_cap > self.capacity() {
					self.reserve(needed_cap - self.capacity());
				}
				self.push_str(first);
				it.for_each(|part| {
					let part = part.as_ref();
					if !part.is_empty() {
						self.push_str(sep);
						self.push_str(part.as_ref());
					}
				})
			}
		}
	}

	fn extend_sep(&mut self, sep: &str, s: &str) {
		if !self.is_empty() {
			self.reserve(s.len() + sep.len());
			self.push_str(sep);
		}
		self.push_str(s);
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
		let idx = self
			.rfind(|c: char| !c.is_ascii_digit())
			.map_or_else(|| self.len(), |idx| idx + 1);
		match self[idx..].parse::<u32>() {
			// parsing an empty string yields an error so that makes sure that [idx - 1] doesn't panic
			Ok(counter) if self.as_bytes()[idx - 1] == b'_' => self.replace_range(idx.., &(counter + 1).to_string()),
			_ => self.push_str("_1"),
		}
	}

	fn cleanup_name(&mut self) {
		// todo aho-corasick?
		self.replace_in_place(" ", "_");
		self.replace_in_place(">=", "GE");
		self.replace_in_place("<=", "LE");
		self.replace_in_place("<", "L");
		self.replace_in_place(">", "G");
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
		self.replace_in_place("!=", "NE");
		self.replace_in_place("|", "OR");
		self.replace_in_place("^", "XOR");
		self.replace_in_place("~", "NOTB");
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

		let result_len = self.elems.iter().fold(0, |len, elem| {
			len + match elem {
				Compiled::IntpLineStart(s) | Compiled::IntpLiteral(s) => s.len(),
				Compiled::IntpLineEnd(s) | Compiled::LiteralLine(s) => s.len() + 1,
				Compiled::Var(name) => params
					.get(name)
					.map_or_else(|| INVALID_PARAM_NAME.len(), |x| x.as_ref().len()),
			}
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
				Compiled::IntpLiteral(s) => out += s,
				Compiled::Var(name) => {
					out.push_indented_str(line_indent, params.get(name).map_or(INVALID_PARAM_NAME, |x| x.as_ref()))
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
		if let Some('\n') = out.chars().next_back() {
			out.pop();
		}
		out
	}
}

pub trait StrExt {
	fn cpp_name_to_rust_case(&self) -> String;
	fn lines_with_nl(&self) -> LinesWithNl;
	fn detect_indent(&self) -> Indent;
	fn compile_interpolation(&self) -> CompiledInterpolation;
	fn trim_start_idx(&self) -> usize;
	fn trim_end_idx(&self) -> usize;
	/// For `cv::rapid::Rapid` returns `Rapid`
	fn localname(&self) -> &str;
	/// For `cv::rapid::Rapid` returns `cv::rapid`
	fn namespace(&self) -> &str;
	/// For `crate::rapid::Rapid` and `rapid::Rapid` returns `rapid`
	fn module(&self) -> &str;
	fn cpp_name_from_fullname(&self, style: CppNameStyle) -> &str;
}

impl StrExt for str {
	fn cpp_name_to_rust_case(&self) -> String {
		let mut out = String::with_capacity(self.len() + 8);
		#[derive(Copy, Clone)]
		enum State {
			StartOrLastUnderscore,
			LastLowercase,
			LastUppercase,
		}
		let mut state = State::StartOrLastUnderscore;
		let mut chars = self.chars().peekable();
		while let Some(cur_c) = chars.next() {
			let (add_c, new_state) = match cur_c {
				_ if cur_c.is_ascii_uppercase() => {
					match state {
						State::StartOrLastUnderscore => {}
						State::LastLowercase => out.push('_'),
						State::LastUppercase => {
							// SVDValue => svd_value
							if chars.peek().map_or(false, |next_c| next_c.is_lowercase()) {
								out.push('_');
							}
						}
					}
					(cur_c.to_ascii_lowercase(), State::LastUppercase)
				}
				'_' => (cur_c, State::StartOrLastUnderscore),
				_ => (cur_c, State::LastLowercase),
			};
			out.push(add_c);
			state = new_state;
		}
		out.replacen_in_place("pn_p", 1, "pnp");
		out.replacen_in_place("p3_p", 1, "p3p");
		out.replacen_in_place("_u_mat", 1, "_umat");
		out.replacen_in_place("i_d3_d", 1, "id_3d_");
		out.replacen_in_place("2_d", 1, "_2d");
		out.replacen_in_place("3_d", 1, "_3d");
		out.replacen_in_place("open_gl", 1, "opengl");
		out.replacen_in_place("open_cl", 1, "opencl");
		out.replacen_in_place("open_vx", 1, "openvx");
		out.replacen_in_place("aruco_3detect", 1, "aruco3_detect");
		out
	}

	fn lines_with_nl(&self) -> LinesWithNl {
		LinesWithNl {
			string: self,
			len: self.len(),
			idx: 0,
		}
	}

	fn detect_indent(&self) -> Indent {
		self
			.char_indices()
			.take_while(|&(_, c)| c == ' ' || c == '\t')
			.last()
			.map_or_else(Indent::default, |(idx, chr)| Indent {
				len: idx + 1,
				symbol: chr,
			})
	}

	fn compile_interpolation(&self) -> CompiledInterpolation {
		static VARS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{\s*([^{}]+?)\s*}}").expect("Can't compile regex"));

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
		self
			.char_indices()
			.find(|(_, c)| !c.is_whitespace())
			.map_or_else(|| self.len(), |(i, _)| i)
	}

	fn trim_end_idx(&self) -> usize {
		self
			.char_indices()
			.rfind(|(_, c)| !c.is_whitespace())
			.map_or(0, |(i, _)| i + 1)
	}

	fn localname(&self) -> &str {
		self.rsplit("::").next().unwrap_or(self)
	}

	fn namespace(&self) -> &str {
		self.rsplit_once("::").map_or(self, |(left, _right)| left)
	}

	fn module(&self) -> &str {
		self
			.strip_prefix("crate::")
			.unwrap_or(self)
			.split("::")
			.next()
			.unwrap_or(self)
	}

	fn cpp_name_from_fullname(&self, style: CppNameStyle) -> &str {
		match style {
			CppNameStyle::Declaration => self.localname(),
			CppNameStyle::Reference => self,
		}
	}
}
