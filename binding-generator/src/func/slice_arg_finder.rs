use crate::field::{Field, SliceArgEligibility};

pub struct SliceArgFinder {
	slice_args: Vec<(Vec<usize>, usize)>,
	state: Option<SliceArgFinderState>,
}

impl SliceArgFinder {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			slice_args: Vec::with_capacity(capacity),
			state: None,
		}
	}

	pub fn feed(&mut self, idx: usize, arg: &Field) {
		self.state = match self.state.take() {
			None => match arg.slice_arg_eligibility() {
				SliceArgEligibility::None => None,
				SliceArgEligibility::SliceArgLength => Some(SliceArgFinderState::LenArgFound(idx)),
				SliceArgEligibility::SliceArgSingle => Some(SliceArgFinderState::SliceArgFound(idx)),
				SliceArgEligibility::SliceArgMultiple => Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(vec![idx])),
			},
			Some(SliceArgFinderState::SliceArgFound(slice_arg_idx)) => match arg.slice_arg_eligibility() {
				SliceArgEligibility::None => Some(SliceArgFinderState::SliceArgFound(slice_arg_idx)),
				SliceArgEligibility::SliceArgLength => {
					self.slice_args.push((vec![slice_arg_idx], idx));
					None
				}
				SliceArgEligibility::SliceArgSingle => Some(SliceArgFinderState::SliceArgFound(idx)),
				SliceArgEligibility::SliceArgMultiple => Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(vec![
					slice_arg_idx,
					idx,
				])),
			},
			Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(mut slice_arg_indices)) => {
				match arg.slice_arg_eligibility() {
					SliceArgEligibility::SliceArgLength => {
						self.slice_args.push((slice_arg_indices, idx));
						None
					}
					// multiple slice arguments are consecutive
					SliceArgEligibility::None | SliceArgEligibility::SliceArgSingle => {
						slice_arg_indices.into_iter().next().map(SliceArgFinderState::SliceArgFound)
					} // todo: try `idx`
					SliceArgEligibility::SliceArgMultiple => {
						slice_arg_indices.push(idx);
						Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(slice_arg_indices))
					}
				}
			}
			Some(SliceArgFinderState::SliceArgWithPotentialMultipleAndLenFound(mut slice_arg_indices, slice_len_arg_idx)) => {
				match arg.slice_arg_eligibility() {
					SliceArgEligibility::None => {
						self.slice_args.push((slice_arg_indices, slice_len_arg_idx));
						None
					}
					SliceArgEligibility::SliceArgLength => {
						self.slice_args.push((slice_arg_indices, slice_len_arg_idx));
						Some(SliceArgFinderState::LenArgFound(idx))
					}
					SliceArgEligibility::SliceArgSingle => {
						self.slice_args.push((slice_arg_indices, slice_len_arg_idx));
						Some(SliceArgFinderState::SliceArgFound(idx))
					}
					SliceArgEligibility::SliceArgMultiple => {
						slice_arg_indices.push(idx);
						Some(SliceArgFinderState::SliceArgWithPotentialMultipleAndLenFound(
							slice_arg_indices,
							slice_len_arg_idx,
						))
					}
				}
			}
			Some(SliceArgFinderState::LenArgFound(slice_len_arg_idx)) => match arg.slice_arg_eligibility() {
				SliceArgEligibility::None => Some(SliceArgFinderState::LenArgFound(slice_len_arg_idx)),
				SliceArgEligibility::SliceArgLength => Some(SliceArgFinderState::LenArgFound(idx)),
				SliceArgEligibility::SliceArgSingle => {
					self.slice_args.push((vec![idx], slice_len_arg_idx));
					None
				}
				SliceArgEligibility::SliceArgMultiple => Some(SliceArgFinderState::SliceArgWithPotentialMultipleAndLenFound(
					vec![idx],
					slice_len_arg_idx,
				)),
			},
		};
	}

	pub fn finish(mut self) -> Vec<(Vec<usize>, usize)> {
		match self.state {
			None
			| Some(SliceArgFinderState::LenArgFound(_))
			| Some(SliceArgFinderState::SliceArgFound(_))
			| Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(_)) => {}
			Some(SliceArgFinderState::SliceArgWithPotentialMultipleAndLenFound(slice_arg_indices, slice_len_arg_idx)) => {
				self.slice_args.push((slice_arg_indices, slice_len_arg_idx));
			}
		}
		self.slice_args
	}
}

#[allow(clippy::enum_variant_names)]
enum SliceArgFinderState {
	/// Found a slice argument which doesn't have a pair
	SliceArgFound(usize),
	/// Found a slice argument which might have an additional connected slice arg (e.g. `src`/`dst` which share the same slice length argument)
	SliceArgWithPotentialMultipleFound(Vec<usize>),
	/// Same as [SliceArgFinderState::SliceArgWithPotentialMultipleFound], but also a length for those arg has also been found
	SliceArgWithPotentialMultipleAndLenFound(Vec<usize>, usize),
	LenArgFound(usize),
}

#[cfg(test)]
mod test {
	use super::SliceArgFinder;
	use crate::field::{Field, FieldDesc};
	use crate::type_ref::{TypeRef, TypeRefDesc};

	#[test]
	fn test_slice_arg_finder() {
		// one arg, not slice
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(0, &Field::new_desc(FieldDesc::new("src", TypeRefDesc::int())));
			assert_eq!(finder.finish(), vec![]);
		}

		// slice len after
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(
				0,
				&Field::new_desc(FieldDesc::new("dims", TypeRef::new_pointer(TypeRefDesc::float()))),
			);
			finder.feed(1, &Field::new_desc(FieldDesc::new("ndims", TypeRefDesc::size_t())));
			assert_eq!(finder.finish(), vec![(vec![0], 1)]);
		}

		// slice len first
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(0, &Field::new_desc(FieldDesc::new("argc", TypeRefDesc::int())));
			finder.feed(
				1,
				&Field::new_desc(FieldDesc::new("argv", TypeRef::new_array(TypeRefDesc::char_ptr(), None))),
			);
			assert_eq!(finder.finish(), vec![(vec![1], 0)]);
		}

		// slice name prevents eligibility
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(
				0,
				&Field::new_desc(FieldDesc::new("rmsd", TypeRef::new_pointer(TypeRefDesc::float()))),
			);
			finder.feed(1, &Field::new_desc(FieldDesc::new("ndims", TypeRefDesc::size_t())));
			assert_eq!(finder.finish(), vec![]);
		}

		// 2 slices with 1 length, size first
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(0, &Field::new_desc(FieldDesc::new("abSize", TypeRefDesc::int())));
			finder.feed(
				1,
				&Field::new_desc(FieldDesc::new("a", TypeRef::new_pointer(TypeRefDesc::float()))),
			);
			finder.feed(
				2,
				&Field::new_desc(FieldDesc::new("b", TypeRef::new_pointer(TypeRefDesc::int()))),
			);
			assert_eq!(finder.finish(), vec![(vec![1, 2], 0)]);
		}

		// 2 slices with 1 length, size last
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(
				0,
				&Field::new_desc(FieldDesc::new("a", TypeRef::new_pointer(TypeRefDesc::float()))),
			);
			finder.feed(
				1,
				&Field::new_desc(FieldDesc::new("b", TypeRef::new_pointer(TypeRefDesc::int()))),
			);
			finder.feed(2, &Field::new_desc(FieldDesc::new("abSize", TypeRefDesc::int())));
			assert_eq!(finder.finish(), vec![(vec![0, 1], 2)]);
		}

		// 2 sets of slices with lengths
		{
			let mut finder = SliceArgFinder::with_capacity(0);
			finder.feed(
				0,
				&Field::new_desc(FieldDesc::new("a", TypeRef::new_pointer(TypeRefDesc::float()))),
			);
			finder.feed(1, &Field::new_desc(FieldDesc::new("aSize", TypeRefDesc::int())));
			finder.feed(
				2,
				&Field::new_desc(FieldDesc::new("b", TypeRef::new_pointer(TypeRefDesc::uint64_t()))),
			);
			finder.feed(3, &Field::new_desc(FieldDesc::new("len", TypeRefDesc::int())));
			assert_eq!(finder.finish(), vec![(vec![0], 1), (vec![2], 3)]);
		}
	}
}
