use crate::core::*;
use crate::Result;
use std::ops::*;

impl Sub<MatExpr> for Scalar {
	type Output = Result<MatExpr>;

	fn sub(self, rhs: MatExpr) -> Self::Output {
		sub_scalar_matexpr(self, &rhs)
	}
}

impl Sub<Mat> for Mat {
	type Output = Result<MatExpr>;

	fn sub(self, rhs: Mat) -> Self::Output {
		sub_mat_mat(&self, &rhs)
	}
}

impl Sub<Mat> for MatExpr {
	type Output = Result<MatExpr>;

	fn sub(self, rhs: Mat) -> Self::Output {
		sub_matexpr_mat(&self, &rhs)
	}
}

impl Mul<MatExpr> for f64 {
	type Output = Result<MatExpr>;

	fn mul(self, rhs: MatExpr) -> Self::Output {
		mul_f64_matexpr(self, &rhs)
	}
}

impl Div<f64> for Mat {
	type Output = Result<MatExpr>;

	fn div(self, rhs: f64) -> Self::Output {
		div_mat_f64(&self, rhs)
	}
}
