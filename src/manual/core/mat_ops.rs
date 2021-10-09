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

macro_rules! impl_ops {
	($func_name:ident, $op_type: ident, $lhs_type:ident, $rhs_type: ident) => {
		// Lhs op Rhs
		impl $op_type<$rhs_type> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			fn sub(self, rhs: $rhs_type) -> Self::Output {
				$func_name(&self, &rhs).into()
			}
		}

		// MatExprResult<Lhs> op Rhs
		impl $op_type<$rhs_type> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			fn sub(self, rhs: $rhs_type) -> Self::Output {
				match self {
					MatExprResult::Ok(lhs) => $func_name(&lhs, &rhs).into(),
					MatExprResult::Err(e) => MatExprResult::Err(e),
				}
			}
		}

		// Lhs op MatExprResult<Rhs>
		impl $op_type<MatExprResult<$rhs_type>> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			fn sub(self, rhs: MatExprResult<$rhs_type>) -> Self::Output {
				match rhs {
					MatExprResult::Ok(rhs) => $func_name(&self, &rhs).into(),
					MatExprResult::Err(e) => MatExprResult::Err(e),
				}
			}
		}

		// MatExprResult<Lhs> op MatExprResult<Rhs>
		impl $op_type<MatExprResult<$rhs_type>> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			fn sub(self, rhs: MatExprResult<$rhs_type>) -> Self::Output {
				match (self, rhs) {
					(MatExprResult::Ok(lhs), MatExprResult::Ok(rhs)) => $func_name(&lhs, &rhs).into(),
					(MatExprResult::Err(e), MatExprResult::Ok(_)) => MatExprResult::Err(e),
					(MatExprResult::Ok(_), MatExprResult::Err(e)) => MatExprResult::Err(e),
					(MatExprResult::Err(lhs_e), MatExprResult::Err(rhs_e)) => {
						MatExprResult::Err(crate::Error::new(
							0,
							format!(
								"Both side of operator has error: lhs-error={} rhs-error={}",
								lhs_e, rhs_e
							),
						))
					}
				}
			}
		}
	};
}

impl_ops!(add_mat_mat, Add, Mat, Mat);
impl_ops!(add_mat_matexpr, Add, Mat, MatExpr);
impl_ops!(add_matexpr_mat, Add, MatExpr, Mat);
impl_ops!(add_matexpr_matexpr, Add, MatExpr, MatExpr);

impl_ops!(add_mat_scalar, Add, Mat, Scalar);
impl_ops!(add_matexpr_scalar, Add, MatExpr, Scalar);
impl_ops!(add_scalar_mat, Add, Scalar, Mat);
impl_ops!(add_scalar_matexpr, Add, Scalar, MatExpr);

impl_ops!(sub_mat_mat, Sub, Mat, Mat);
impl_ops!(sub_mat_matexpr, Sub, Mat, MatExpr);
impl_ops!(sub_matexpr_mat, Sub, MatExpr, Mat);
impl_ops!(sub_matexpr_matexpr, Sub, MatExpr, MatExpr);

impl_ops!(sub_mat_scalar, Sub, Mat, Scalar);
impl_ops!(sub_matexpr_scalar, Sub, MatExpr, Scalar);
impl_ops!(sub_scalar_mat, Sub, Scalar, Mat);
impl_ops!(sub_scalar_matexpr, Sub, Scalar, MatExpr);

impl_ops!(mul_mat_mat, Mul, Mat, Mat);
impl_ops!(mul_mat_matexpr, Mul, Mat, MatExpr);
impl_ops!(mul_matexpr_mat, Mul, MatExpr, Mat);
impl_ops!(mul_matexpr_matexpr, Mul, MatExpr, MatExpr);

impl_ops!(mul_mat_f64, Mul, Mat, f64);
impl_ops!(mul_matexpr_f64, Mul, MatExpr, f64);
impl_ops!(mul_f64_mat, Mul, f64, Mat);
impl_ops!(mul_f64_matexpr, Mul, f64, MatExpr);

impl_ops!(div_mat_mat, Div, Mat, Mat);
impl_ops!(div_mat_matexpr, Div, Mat, MatExpr);
impl_ops!(div_matexpr_mat, Div, MatExpr, Mat);
impl_ops!(div_matexpr_matexpr, Div, MatExpr, MatExpr);

impl_ops!(div_mat_f64, Div, Mat, f64);
impl_ops!(div_matexpr_f64, Div, MatExpr, f64);
impl_ops!(div_f64_mat, Div, f64, Mat);
impl_ops!(div_f64_matexpr, Div, f64, MatExpr);

// TODO
// fn sub_mat(Mat);
// fn sub_matexpr(MatExpr);
