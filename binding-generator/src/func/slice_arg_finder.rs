use crate::field::{Field, SliceArgEligibility};

pub struct SliceArgFinder {
	slice_args: Vec<(Vec<usize>, usize)>,
	state: Option<SliceArgFinderState>,
}

impl SliceArgFinder {
	pub fn new() -> Self {
		Self {
			slice_args: vec![],
			state: None,
		}
	}

	pub fn feed(&mut self, idx: usize, arg: &Field) {
		self.state = match self.state.take() {
			None => match arg.slice_arg_eligibility() {
				SliceArgEligibility::NotEligible => {
					if arg.can_be_slice_arg_len() {
						Some(SliceArgFinderState::LenArgFound(idx))
					} else {
						None
					}
				}
				SliceArgEligibility::Eligible => Some(SliceArgFinderState::SliceArgFound(idx)),
				SliceArgEligibility::EligibleWithMultiple => Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(vec![idx])),
			},
			Some(SliceArgFinderState::SliceArgFound(slice_arg_idx)) => {
				if arg.can_be_slice_arg_len() {
					self.slice_args.push((vec![slice_arg_idx], idx));
					None
				} else {
					Some(SliceArgFinderState::SliceArgFound(slice_arg_idx))
				}
			}
			Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(mut slice_arg_indices)) => {
				match arg.slice_arg_eligibility() {
					SliceArgEligibility::NotEligible if arg.can_be_slice_arg_len() => {
						self.slice_args.push((slice_arg_indices, idx));
						None
					}
					// multiple slice arguments are consecutive
					SliceArgEligibility::NotEligible | SliceArgEligibility::Eligible => {
						slice_arg_indices.into_iter().next().map(SliceArgFinderState::SliceArgFound)
					} // todo: try `idx`
					SliceArgEligibility::EligibleWithMultiple => {
						slice_arg_indices.push(idx);
						Some(SliceArgFinderState::SliceArgWithPotentialMultipleFound(slice_arg_indices))
					}
				}
			}
			Some(SliceArgFinderState::LenArgFound(slice_len_arg_idx)) => match arg.slice_arg_eligibility() {
				SliceArgEligibility::NotEligible => Some(SliceArgFinderState::LenArgFound(slice_len_arg_idx)),
				SliceArgEligibility::Eligible => {
					self.slice_args.push((vec![idx], slice_len_arg_idx));
					None
				}
				SliceArgEligibility::EligibleWithMultiple => {
					// fixme: case where slice len argument comes before 2 slice arguments is not handled
					self.slice_args.push((vec![idx], slice_len_arg_idx));
					None
				}
			},
		};
	}

	pub fn finish(self) -> Vec<(Vec<usize>, usize)> {
		self.slice_args
	}
}

#[allow(clippy::enum_variant_names)]
enum SliceArgFinderState {
	/// Found a slice argument which doesn't have a pair
	SliceArgFound(usize),
	/// Found a slice argument which might have an additional connected slice arg (e.g. `src`/`dst` which share the same slice length argument)
	SliceArgWithPotentialMultipleFound(Vec<usize>),
	LenArgFound(usize),
}
