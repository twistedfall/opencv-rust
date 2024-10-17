use std::ops::*;

use crate::core::*;
use crate::{Error, Result};

/// Intermediate result type that's produced by the [Mat] operations. Call [MatExprResult::into_result] to get the regular
/// [Result].
///
/// This type is needed because of Rust orphan rules.
pub enum MatExprResult<T> {
	Ok(T),
	Err(Error),
}

impl<T> From<Result<T, Error>> for MatExprResult<T> {
	#[inline]
	fn from(r: Result<T, Error>) -> Self {
		match r {
			Ok(val) => Self::Ok(val),
			Err(e) => Self::Err(e),
		}
	}
}

impl<T> MatExprResult<T> {
	#[inline]
	pub fn into_result(self) -> Result<T, Error> {
		match self {
			Self::Ok(val) => Ok(val),
			Self::Err(e) => Err(e),
		}
	}
}

/// Elementwise multiplication
pub trait ElemMul<Rhs = Self> {
	type Output;
	fn elem_mul(self, rhs: Rhs) -> Self::Output;
}

#[inline]
fn elemmul_mat_mat(a: &impl MatTraitConst, b: &impl ToInputArray) -> Result<MatExpr> {
	a.mul_def(b)
}

#[inline]
fn elemmul_mat_matexpr(a: &impl MatTraitConst, b: &impl ToInputArray) -> Result<MatExpr> {
	a.mul_def(b)
}

#[inline]
fn elemmul_matexpr_mat(a: &impl MatExprTraitConst, b: &impl MatTraitConst) -> Result<MatExpr> {
	a.mul_def(b)
}

#[inline]
fn elemmul_matexpr_matexpr(a: &impl MatExprTraitConst, b: &impl MatExprTraitConst) -> Result<MatExpr> {
	a.mul_matexpr_def(b)
}

/// Hints Rust about whether we need a borrow or a move for an argument passed to Mat operation function, only for internal usage
trait OpsArg: Sized {
	#[inline]
	fn ops_arg(self) -> Self {
		self
	}
}

impl OpsArg for &Mat {}
impl OpsArg for &MatExpr {}
impl OpsArg for Scalar {}
impl OpsArg for f64 {}

macro_rules! impl_ops_core {
	($func_name:ident, $op_type:ident, $lhs_type:ty, $rhs_type:ty, $op_func:ident) => {
		// Lhs op Rhs
		impl $op_type<$rhs_type> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			#[inline]
			fn $op_func(self, rhs: $rhs_type) -> Self::Output {
				let lhs = self;
				$func_name(lhs.ops_arg(), rhs.ops_arg()).into()
			}
		}

		// MatExprResult<Lhs> op Rhs
		impl $op_type<$rhs_type> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			#[inline]
			fn $op_func(self, rhs: $rhs_type) -> Self::Output {
				let lhs = self;
				match lhs {
					MatExprResult::Ok(lhs) => $func_name(lhs.ops_arg(), rhs.ops_arg()).into(),
					MatExprResult::Err(e) => MatExprResult::Err(e),
				}
			}
		}

		// Lhs op MatExprResult<Rhs>
		impl $op_type<MatExprResult<$rhs_type>> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			#[inline]
			fn $op_func(self, rhs: MatExprResult<$rhs_type>) -> Self::Output {
				let lhs = self;
				match rhs {
					MatExprResult::Ok(rhs) => $func_name(lhs.ops_arg(), rhs.ops_arg()).into(),
					MatExprResult::Err(e) => MatExprResult::Err(e),
				}
			}
		}

		// MatExprResult<Lhs> op MatExprResult<Rhs>
		impl $op_type<MatExprResult<$rhs_type>> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			#[inline]
			fn $op_func(self, rhs: MatExprResult<$rhs_type>) -> Self::Output {
				let lhs = self;
				match (lhs, rhs) {
					(MatExprResult::Ok(lhs), MatExprResult::Ok(rhs)) => $func_name(lhs.ops_arg(), rhs.ops_arg()).into(),
					(MatExprResult::Err(e), MatExprResult::Ok(_)) | (MatExprResult::Ok(_), MatExprResult::Err(e)) => {
						MatExprResult::Err(e)
					}
					(MatExprResult::Err(lhs_e), MatExprResult::Err(rhs_e)) => MatExprResult::Err(crate::Error::new(
						StsBadArg,
						format!("Both sides of operator have error: lhs-error={lhs_e} rhs-error={rhs_e}"),
					)),
				}
			}
		}
	};
}

macro_rules! impl_ops {
	($func_name:ident, $op_type:ident, $lhs_type:ty, $rhs_type:ty, $op_func:ident) => {
		impl_ops_core!($func_name, $op_type, $lhs_type, $rhs_type, $op_func);
		impl_ops_core!($func_name, $op_type, $lhs_type, &$rhs_type, $op_func);
		impl_ops_core!($func_name, $op_type, &$lhs_type, $rhs_type, $op_func);
		impl_ops_core!($func_name, $op_type, &$lhs_type, &$rhs_type, $op_func);
	};
}

impl_ops!(add_mat_mat, Add, Mat, Mat, add);
impl_ops!(add_mat_matexpr, Add, Mat, MatExpr, add);
impl_ops!(add_matexpr_mat, Add, MatExpr, Mat, add);
impl_ops!(add_matexpr_matexpr, Add, MatExpr, MatExpr, add);

impl_ops!(add_mat_scalar, Add, Mat, Scalar, add);
impl_ops!(add_matexpr_scalar, Add, MatExpr, Scalar, add);
impl_ops!(add_scalar_mat, Add, Scalar, Mat, add);
impl_ops!(add_scalar_matexpr, Add, Scalar, MatExpr, add);

impl_ops!(sub_mat_mat, Sub, Mat, Mat, sub);
impl_ops!(sub_mat_matexpr, Sub, Mat, MatExpr, sub);
impl_ops!(sub_matexpr_mat, Sub, MatExpr, Mat, sub);
impl_ops!(sub_matexpr_matexpr, Sub, MatExpr, MatExpr, sub);

impl_ops!(sub_mat_scalar, Sub, Mat, Scalar, sub);
impl_ops!(sub_matexpr_scalar, Sub, MatExpr, Scalar, sub);
impl_ops!(sub_scalar_mat, Sub, Scalar, Mat, sub);
impl_ops!(sub_scalar_matexpr, Sub, Scalar, MatExpr, sub);

impl_ops!(mul_mat_mat, Mul, Mat, Mat, mul);
impl_ops!(mul_mat_matexpr, Mul, Mat, MatExpr, mul);
impl_ops!(mul_matexpr_mat, Mul, MatExpr, Mat, mul);
impl_ops!(mul_matexpr_matexpr, Mul, MatExpr, MatExpr, mul);

impl_ops!(mul_mat_f64, Mul, Mat, f64, mul);
impl_ops!(mul_matexpr_f64, Mul, MatExpr, f64, mul);
impl_ops!(mul_f64_mat, Mul, f64, Mat, mul);
impl_ops!(mul_f64_matexpr, Mul, f64, MatExpr, mul);

impl_ops!(div_mat_mat, Div, Mat, Mat, div);
impl_ops!(div_mat_matexpr, Div, Mat, MatExpr, div);
impl_ops!(div_matexpr_mat, Div, MatExpr, Mat, div);
impl_ops!(div_matexpr_matexpr, Div, MatExpr, MatExpr, div);

impl_ops!(div_mat_f64, Div, Mat, f64, div);
impl_ops!(div_matexpr_f64, Div, MatExpr, f64, div);
impl_ops!(div_f64_mat, Div, f64, Mat, div);
impl_ops!(div_f64_matexpr, Div, f64, MatExpr, div);

impl_ops!(elemmul_mat_mat, ElemMul, Mat, Mat, elem_mul);
impl_ops!(elemmul_mat_matexpr, ElemMul, Mat, MatExpr, elem_mul);
impl_ops!(elemmul_matexpr_mat, ElemMul, MatExpr, Mat, elem_mul);
impl_ops!(elemmul_matexpr_matexpr, ElemMul, MatExpr, MatExpr, elem_mul);

// not implemented yet, but can use `0 - mat`
// fn sub_mat(Mat);
// fn sub_matexpr(MatExpr);
