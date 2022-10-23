use crate::core::*;
use crate::Result;
use std::ops::*;

pub enum MatExprResult<T> {
	Ok(T),
	Err(crate::Error),
}

impl<T> From<Result<T, crate::Error>> for MatExprResult<T> {
	fn from(r: Result<T, crate::Error>) -> Self {
		match r {
			Result::Ok(val) => MatExprResult::Ok(val),
			Result::Err(e) => MatExprResult::Err(e),
		}
	}
}

impl<T> MatExprResult<T> {
	pub fn into_result(self) -> Result<T, crate::Error> {
		match self {
			MatExprResult::Ok(val) => std::result::Result::Ok(val),
			MatExprResult::Err(e) => std::result::Result::Err(e),
		}
	}
}

/// elementwise multiplication
pub trait ElemMul<Rhs = Self> {
	type Output;
	fn elem_mul(self, rhs: Rhs) -> Self::Output;
}

// only for internal usage
trait ToUnderlyingArg<'a, T: 'a> {
	fn to_underlying_arg(&'a self) -> T;
}
impl<'a> ToUnderlyingArg<'a, &'a Mat> for Mat {
	fn to_underlying_arg(&'a self) -> &'a Mat {
		self
	}
}
impl<'a> ToUnderlyingArg<'a, &'a MatExpr> for MatExpr {
	fn to_underlying_arg(&'a self) -> &'a MatExpr {
		self
	}
}
impl<'a> ToUnderlyingArg<'a, Scalar> for Scalar {
	fn to_underlying_arg(&'a self) -> Scalar {
		*self
	}
}
impl<'a> ToUnderlyingArg<'a, f64> for f64 {
	fn to_underlying_arg(&'a self) -> f64 {
		*self
	}
}

macro_rules! impl_ops_core {
	($func_name:ident, $op_type:ident, $lhs_type:ty, $rhs_type:ty, $op_func:ident) => {
		// Lhs op Rhs
		impl $op_type<$rhs_type> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			fn $op_func(self, rhs: $rhs_type) -> Self::Output {
				let lhs = self;
				$func_name(lhs.to_underlying_arg(), rhs.to_underlying_arg()).into()
			}
		}

		// MatExprResult<Lhs> op Rhs
		impl $op_type<$rhs_type> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			fn $op_func(self, rhs: $rhs_type) -> Self::Output {
				let lhs = self;
				match lhs {
					MatExprResult::Ok(lhs) => $func_name(lhs.to_underlying_arg(), rhs.to_underlying_arg()).into(),
					MatExprResult::Err(e) => MatExprResult::Err(e),
				}
			}
		}

		// Lhs op MatExprResult<Rhs>
		impl $op_type<MatExprResult<$rhs_type>> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			fn $op_func(self, rhs: MatExprResult<$rhs_type>) -> Self::Output {
				let lhs = self;
				match rhs {
					MatExprResult::Ok(rhs) => $func_name(lhs.to_underlying_arg(), rhs.to_underlying_arg()).into(),
					MatExprResult::Err(e) => MatExprResult::Err(e),
				}
			}
		}

		// MatExprResult<Lhs> op MatExprResult<Rhs>
		impl $op_type<MatExprResult<$rhs_type>> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			fn $op_func(self, rhs: MatExprResult<$rhs_type>) -> Self::Output {
				let lhs = self;
				match (lhs, rhs) {
					(MatExprResult::Ok(lhs), MatExprResult::Ok(rhs)) => {
						$func_name(lhs.to_underlying_arg(), rhs.to_underlying_arg()).into()
					}
					(MatExprResult::Err(e), MatExprResult::Ok(_)) => MatExprResult::Err(e),
					(MatExprResult::Ok(_), MatExprResult::Err(e)) => MatExprResult::Err(e),
					(MatExprResult::Err(lhs_e), MatExprResult::Err(rhs_e)) => MatExprResult::Err(crate::Error::new(
						0,
						format!(
							"Both side of operator has error: lhs-error={} rhs-error={}",
							lhs_e, rhs_e
						),
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

fn elemmul_mat_mat(a: &Mat, b: &Mat) -> Result<MatExpr> {
	MatTraitConst::mul(a, b, 1.0)
}
fn elemmul_mat_matexpr(a: &Mat, b: &MatExpr) -> Result<MatExpr> {
	MatTraitConst::mul(a, b, 1.0)
}
fn elemmul_matexpr_mat(a: &MatExpr, b: &Mat) -> Result<MatExpr> {
	MatExprTraitConst::mul(a, b, 1.0)
}
fn elemmul_matexpr_matexpr(a: &MatExpr, b: &MatExpr) -> Result<MatExpr> {
	MatExprTraitConst::mul_matexpr(a, b, 1.0)
}

impl_ops!(elemmul_mat_mat, ElemMul, Mat, Mat, elem_mul);
impl_ops!(elemmul_mat_matexpr, ElemMul, Mat, MatExpr, elem_mul);
impl_ops!(elemmul_matexpr_mat, ElemMul, MatExpr, Mat, elem_mul);
impl_ops!(elemmul_matexpr_matexpr, ElemMul, MatExpr, MatExpr, elem_mul);

// not implemented yet, but can use `0 - mat`
// fn sub_mat(Mat);
// fn sub_matexpr(MatExpr);
