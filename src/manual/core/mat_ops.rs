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
	($func_name:ident, $op_type: ident, $lhs_type:ident, $rhs_type: ident, $op_func: ident, $lhs_usage:expr, $rhs_usage:expr) => {
		// Lhs op Rhs
		impl $op_type<$rhs_type> for $lhs_type {
			type Output = MatExprResult<MatExpr>;

			fn $op_func(self, rhs: $rhs_type) -> Self::Output {
				let lhs = self;
				$func_name($lhs_usage, $rhs_usage).into()
			}
		}

		// MatExprResult<Lhs> op Rhs
		impl $op_type<$rhs_type> for MatExprResult<$lhs_type> {
			type Output = MatExprResult<MatExpr>;

			fn $op_func(self, rhs: $rhs_type) -> Self::Output {
				let lhs = self;
				match lhs {
					MatExprResult::Ok(lhs) => $func_name($lhs_usage, $rhs_usage).into(),
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
					MatExprResult::Ok(rhs) => $func_name($lhs_usage, $rhs_usage).into(),
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
					(MatExprResult::Ok(lhs), MatExprResult::Ok(rhs)) => $func_name($lhs_usage, $rhs_usage).into(),
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

fn lhs() {}

fn rhs() {}

impl_ops!(add_mat_mat, Add, Mat, Mat, add, &lhs, &rhs);
impl_ops!(add_mat_matexpr, Add, Mat, MatExpr, add, &lhs, &rhs);
impl_ops!(add_matexpr_mat, Add, MatExpr, Mat, add, &lhs, &rhs);
impl_ops!(add_matexpr_matexpr, Add, MatExpr, MatExpr, add, &lhs, &rhs);

// impl_ops!(add_mat_scalar, Add, Mat, Scalar, add, &Mat, Scalar);
// impl_ops!(add_matexpr_scalar, Add, MatExpr, Scalar, add, &MatExpr, Scalar);
// impl_ops!(add_scalar_mat, Add, Scalar, Mat, add, Scalar, &Mat);
// impl_ops!(add_scalar_matexpr, Add, Scalar, MatExpr, add, Scalar, &MatExpr);
//
// impl_ops!(sub_mat_mat, Sub, Mat, Mat, sub, &Mat, &Mat);
// impl_ops!(sub_mat_matexpr, Sub, Mat, MatExpr, sub, &Mat, &MatExpr);
// impl_ops!(sub_matexpr_mat, Sub, MatExpr, Mat, sub, &MatExpr, &Mat);
// impl_ops!(sub_matexpr_matexpr, Sub, MatExpr, MatExpr, sub, &MatExpr, &MatExpr);
//
// impl_ops!(sub_mat_scalar, Sub, Mat, Scalar, sub, &Mat, Scalar);
// impl_ops!(sub_matexpr_scalar, Sub, MatExpr, Scalar, sub, &MatExpr, Scalar);
// impl_ops!(sub_scalar_mat, Sub, Scalar, Mat, sub, Scalar, &Mat);
// impl_ops!(sub_scalar_matexpr, Sub, Scalar, MatExpr, sub, Scalar, &MatExpr);
//
// impl_ops!(mul_mat_mat, Mul, Mat, Mat, mul, &Mat, &Mat);
// impl_ops!(mul_mat_matexpr, Mul, Mat, MatExpr, mul, &Mat, &MatExpr);
// impl_ops!(mul_matexpr_mat, Mul, MatExpr, Mat, mul, &MatExpr, &Mat);
// impl_ops!(mul_matexpr_matexpr, Mul, MatExpr, MatExpr, mul, &MatExpr, &MatExpr);
//
// impl_ops!(mul_mat_f64, Mul, Mat, f64, mul, &Mat, f64);
// impl_ops!(mul_matexpr_f64, Mul, MatExpr, f64, mul, &MatExpr, f64);
// impl_ops!(mul_f64_mat, Mul, f64, Mat, mul, f64, &Mat);
// impl_ops!(mul_f64_matexpr, Mul, f64, MatExpr, mul, f64, &MatExpr);
//
// impl_ops!(div_mat_mat, Div, Mat, Mat, div, &Mat, &Mat);
// impl_ops!(div_mat_matexpr, Div, Mat, MatExpr, div, &Mat, &MatExpr);
// impl_ops!(div_matexpr_mat, Div, MatExpr, Mat, div, &MatExpr, &Mat);
// impl_ops!(div_matexpr_matexpr, Div, MatExpr, MatExpr, div, &MatExpr, &MatExpr);
//
// impl_ops!(div_mat_f64, Div, Mat, f64, div, &Mat, f64);
// impl_ops!(div_matexpr_f64, Div, MatExpr, f64, div, &MatExpr, f64);
// impl_ops!(div_f64_mat, Div, f64, Mat, div, f64, &Mat);
// impl_ops!(div_f64_matexpr, Div, f64, MatExpr, div, f64, &MatExpr);

// TODO
// fn sub_mat(Mat);
// fn sub_matexpr(MatExpr);
