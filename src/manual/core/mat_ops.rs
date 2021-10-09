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

// impl<T> From<MatExprResult<T>> for Result<T, crate::Error> {
// 	fn from(r: MatExprResult<T>) -> Self {
// 		match r {
// 			MatExprResult::Ok(val) => std::result::Result::Ok(val),
// 			MatExprResult::Err(e) => std::result::Result::Err(e),
// 		}
// 	}
// }

// impl<LInner: Sub<RInner, Output = MatExprResult<O>>, RInner, O> Sub<MatExprResult<RInner>> for LInner {
// 	type Output = MatExprResult<O>;
//
// 	fn sub(self, rhs: MatExprResult<RInner>) -> Self::Output {
// 		match rhs {
// 			MatExprResult::Ok(rhs) => self - rhs,
// 			MatExprResult::Err(err) => MatExprResult::Err(err),
// 		}
// 	}
// }
//
// impl<LInner: Sub<RInner, Output = MatExprResult<O>>, RInner, O> Sub<RInner> for MatExprResult<LInner> {
// 	type Output = MatExprResult<O>;
//
// 	fn sub(self, rhs: MatExprResult<RInner>) -> Self::Output {
// 		match self {
// 			MatExprResult::Ok(lhs) => lhs - rhs,
// 			MatExprResult::Err(err) => MatExprResult::Err(err),
// 		}
// 	}
// }
//
// impl Sub<MatExpr> for Scalar {
// 	type Output = MatExprResult<MatExpr>;
//
// 	fn sub(self, rhs: MatExpr) -> Self::Output {
// 		sub_scalar_matexpr(self, &rhs)
// 	}
// }

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

impl_ops!(sub_mat_mat, Sub, Mat, Mat);
impl_ops!(sub_matexpr_matexpr, Sub, MatExpr, MatExpr);

// impl Sub<Mat> for Mat {
// 	type Output = MatExprResult<MatExpr>;
//
// 	fn sub(self, rhs: Mat) -> Self::Output {
// 		sub_mat_mat(&self, &rhs).into()
// 	}
// }
//
// impl Sub<Mat> for MatExpr {
// 	type Output = MatExprResult<MatExpr>;
//
// 	fn sub(self, rhs: Mat) -> Self::Output {
// 		sub_matexpr_mat(&self, &rhs).into()
// 	}
// }
//
// impl Sub<Mat> for MatExprResult<MatExpr> {
// 	type Output = MatExprResult<MatExpr>;
//
// 	fn sub(self, rhs: Mat) -> Self::Output {
// 		todo!()
// 	}
// }
//
// impl Sub<MatExprResult<Mat>> for MatExpr {
// 	type Output = MatExprResult<MatExpr>;
//
// 	fn sub(self, rhs: MatExprResult<Mat>) -> Self::Output {
// 		todo!()
// 	}
// }
//
// impl Sub<MatExprResult<MatExpr>> for MatExprResult<MatExpr> {
// 	type Output = MatExprResult<MatExpr>;
//
// 	fn sub(self, rhs: MatExprResult<MatExpr>) -> Self::Output {
// 		todo!()
// 	}
// }

// impl Mul<MatExpr> for f64 {
// 	type Output = Result<MatExpr>;
//
// 	fn mul(self, rhs: MatExpr) -> Self::Output {
// 		mul_f64_matexpr(self, &rhs)
// 	}
// }
//
// impl Div<f64> for Mat {
// 	type Output = Result<MatExpr>;
//
// 	fn div(self, rhs: f64) -> Self::Output {
// 		div_mat_f64(&self, rhs)
// 	}
// }
