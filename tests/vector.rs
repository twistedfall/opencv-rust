use std::any::TypeId;
use std::os::raw::c_char;

use matches::assert_matches;

use opencv::core::{DMatch, Point2d, Point2f, Scalar, SparseMat_Hdr, Vec4i};
use opencv::prelude::*;
use opencv::types::{
	VectorOfDMatch, VectorOfKeyPoint, VectorOfMat, VectorOfPoint2d, VectorOfPoint2f, VectorOfRange, VectorOfString, VectorOfVec4i,
	VectorOfVectorOfPoint2f, VectorOfbool, VectorOfc_char, VectorOff64, VectorOfi32, VectorOfi8, VectorOfu8,
};
use opencv::{core, Error, Result};

#[test]
fn boxed() -> Result<()> {
	{
		let mut vec = VectorOfMat::new();
		vec.push(Mat::new_rows_cols_with_default(1, 3, u8::opencv_type(), Scalar::all(1.))?);
		vec.push(Mat::new_rows_cols_with_default(1, 3, u16::opencv_type(), Scalar::all(2.))?);
		vec.push(Mat::new_rows_cols_with_default(1, 3, i32::opencv_type(), Scalar::all(3.))?);
		assert_eq!(3, vec.len());
		assert_eq!(u8::opencv_type(), vec.get(0)?.typ());
		assert_eq!(1, *vec.get(0)?.at_2d::<u8>(0, 1)?);
		assert_eq!(u16::opencv_type(), vec.get(1)?.typ());
		assert_eq!(2, *vec.get(1)?.at_2d::<u16>(0, 1)?);
		assert_eq!(i32::opencv_type(), vec.get(2)?.typ());
		assert_eq!(3, *vec.get(2)?.at_2d::<i32>(0, 1)?);
		vec.set(0, Mat::new_rows_cols_with_default(1, 3, f32::opencv_type(), Scalar::all(3.))?)?;
		unsafe {
			vec.set_unchecked(1, Mat::new_rows_cols_with_default(1, 3, f64::opencv_type(), Scalar::all(4.))?);
		}
		vec.set(2, Mat::new_rows_cols_with_default(1, 3, i16::opencv_type(), Scalar::all(5.))?)?;
		assert_eq!(f32::opencv_type(), unsafe { vec.get_unchecked(0) }.typ());
		assert_eq!(3., *unsafe { vec.get_unchecked(0) }.at_2d::<f32>(0, 1)?);
		assert_eq!(f64::opencv_type(), unsafe { vec.get_unchecked(1) }.typ());
		assert_eq!(4., *unsafe { vec.get_unchecked(1) }.at_2d::<f64>(0, 1)?);
		assert_eq!(i16::opencv_type(), unsafe { vec.get_unchecked(2) }.typ());
		assert_eq!(5, *unsafe { vec.get_unchecked(2) }.at_2d::<i16>(0, 1)?);
	}

	#[cfg(ocvrs_has_module_imgproc)]
	{
		use opencv::{
			core::{Point, Vec3b},
			imgproc,
		};

		let mut m = Mat::new_rows_cols_with_default(10, 10, Vec3b::opencv_type(), Scalar::default())?;
		let mut ps = VectorOfMat::new();
		assert_eq!(ps.len(), 0);
		let mut p1 = unsafe { Mat::new_rows_cols(3, 2, i32::opencv_type()) }?;
		p1.at_row_mut::<i32>(0)?.copy_from_slice(&[0, 0]);
		p1.at_row_mut::<i32>(1)?.copy_from_slice(&[0, 9]);
		p1.at_row_mut::<i32>(2)?.copy_from_slice(&[9, 9]);
		ps.push(p1);
		assert_eq!(ps.len(), 1);
		#[cfg(ocvrs_opencv_branch_4)]
		use imgproc::LINE_8;
		#[cfg(not(ocvrs_opencv_branch_4))]
		use opencv::core::LINE_8;
		imgproc::fill_poly(&mut m, &ps, Scalar::new(127., 127., 127., 0.), LINE_8, 0, Point::default())?;
		assert_eq!(*m.at_2d::<Vec3b>(0, 0)?, Vec3b::from([127, 127, 127]));
		assert_eq!(*m.at_2d::<Vec3b>(0, 9)?, Vec3b::default());
		assert_eq!(*m.at_2d::<Vec3b>(9, 9)?, Vec3b::from([127, 127, 127]));
	}

	Ok(())
}

#[test]
fn string() -> Result<()> {
	let mut vec = VectorOfString::new();
	vec.push("123");
	vec.push("456");
	vec.push("789");
	vec.push("888\0999");
	vec.push("\0999");
	assert_eq!(5, vec.len());
	assert_eq!("123", vec.get(0)?);
	assert_eq!("456", vec.get(1)?);
	assert_eq!("789", vec.get(2)?);
	assert_eq!("888", vec.get(3)?);
	assert_eq!("", vec.get(4)?);
	vec.set(0, "qqq")?;
	unsafe {
		vec.set_unchecked(1, "www");
	}
	vec.set(2, "eee")?;
	assert_eq!("qqq", unsafe { vec.get_unchecked(0) });
	assert_eq!("www", unsafe { vec.get_unchecked(1) });
	assert_eq!("eee", unsafe { vec.get_unchecked(2) });
	Ok(())
}

#[test]
fn boolean() -> Result<()> {
	#![allow(clippy::bool_assert_comparison)]
	let mut vec = VectorOfbool::new();
	vec.push(true);
	vec.push(true);
	vec.push(false);
	assert_eq!(true, vec.get(0)?);
	assert_eq!(true, vec.get(1)?);
	assert_eq!(false, vec.get(2)?);
	vec.set(0, false)?;
	unsafe {
		vec.set_unchecked(1, false);
	}
	vec.set(2, true)?;
	assert_eq!(false, unsafe { vec.get_unchecked(0) });
	assert_eq!(false, unsafe { vec.get_unchecked(1) });
	assert_eq!(true, unsafe { vec.get_unchecked(2) });
	Ok(())
}

#[test]
fn primitive() -> Result<()> {
	let mut vec = VectorOfi32::with_capacity(10);
	vec.push(1);
	vec.push(2);
	vec.push(3);
	assert_eq!(1, vec.get(0)?);
	assert_eq!(2, vec.get(1)?);
	assert_eq!(3, vec.get(2)?);
	vec.set(0, 4)?;
	vec.set(1, 5)?;
	unsafe { vec.set_unchecked(2, 6) };
	assert_eq!(4, unsafe { vec.get_unchecked(0) });
	assert_eq!(5, unsafe { vec.get_unchecked(1) });
	assert_eq!(6, unsafe { vec.get_unchecked(2) });
	Ok(())
}

#[test]
fn char() -> Result<()> {
	let mut vec_char = VectorOfc_char::new();
	vec_char.push(5);
	vec_char.push(10);
	vec_char.push(33);

	assert_eq!(&[5 as c_char, 10, 33], vec_char.as_slice());

	let mut vec_i8 = VectorOfi8::new();
	vec_i8.push(5);
	vec_i8.push(10);
	vec_i8.push(33);

	assert_eq!(&[5i8, 10, 33], vec_i8.as_slice());

	let mut vec_u8 = VectorOfu8::new();
	vec_u8.push(5);
	vec_u8.push(10);
	vec_u8.push(33);

	assert_eq!(&[5u8, 10, 33], vec_u8.as_slice());

	assert!(
		TypeId::of::<VectorOfc_char>() == TypeId::of::<VectorOfi8>()
			|| TypeId::of::<VectorOfc_char>() == TypeId::of::<VectorOfu8>()
	);

	Ok(())
}

#[test]
fn simple_struct() -> Result<()> {
	let mut vec = VectorOfPoint2d::new();
	vec.push(Point2d::new(10., 10.));
	vec.push(Point2d::new(20., 20.));
	vec.push(Point2d::new(30., 30.));
	assert_eq!(Point2d::new(10., 10.), vec.get(0)?);
	assert_eq!(Point2d::new(20., 20.), vec.get(1)?);
	assert_eq!(Point2d::new(30., 30.), vec.get(2)?);
	unsafe { vec.set_unchecked(0, Point2d::new(40., 50.)) };
	vec.set(1, Point2d::new(50., 60.))?;
	vec.set(2, Point2d::new(60., 70.))?;
	assert_eq!(Point2d::new(40., 50.), unsafe { vec.get_unchecked(0) });
	assert_eq!(Point2d::new(50., 60.), unsafe { vec.get_unchecked(1) });
	assert_eq!(Point2d::new(60., 70.), unsafe { vec.get_unchecked(2) });
	Ok(())
}

#[test]
fn simple() -> Result<()> {
	let mut matches = VectorOfDMatch::new();
	let m = DMatch::new_index(0, 1, 10, 12.4)?;
	matches.push(m);
	assert_eq!(m.distance, matches.get(0)?.distance);
	assert_eq!(m.query_idx, matches.get(0)?.query_idx);
	assert_eq!(m.img_idx, matches.get(0)?.img_idx);
	assert_eq!(m.train_idx, matches.get(0)?.train_idx);

	let m = DMatch::new_index(2, 13, 9, 98.1)?;
	matches.set(0, m)?;
	assert_eq!(m.distance, unsafe { matches.get_unchecked(0) }.distance);
	assert_eq!(m.query_idx, unsafe { matches.get_unchecked(0) }.query_idx);
	assert_eq!(m.img_idx, unsafe { matches.get_unchecked(0) }.img_idx);
	assert_eq!(m.train_idx, unsafe { matches.get_unchecked(0) }.train_idx);

	let m = DMatch::new_index(3, 7, 19, 76.)?;
	unsafe { matches.set_unchecked(0, m) };
	assert_eq!(m.distance, unsafe { matches.get_unchecked(0) }.distance);
	assert_eq!(m.query_idx, unsafe { matches.get_unchecked(0) }.query_idx);
	assert_eq!(m.img_idx, unsafe { matches.get_unchecked(0) }.img_idx);
	assert_eq!(m.train_idx, unsafe { matches.get_unchecked(0) }.train_idx);
	Ok(())
}

#[test]
fn vector_of_vector_simple_struct() -> Result<()> {
	#[inline(never)]
	fn make_vec() -> VectorOfVectorOfPoint2f {
		let mut outer = VectorOfVectorOfPoint2f::new();
		outer.push({
			let mut inner = VectorOfPoint2f::new();
			inner.push(Point2f::new(1., 1.));
			inner.push(Point2f::new(2., 2.));
			inner.push(Point2f::new(3., 3.));
			inner
		});
		outer.push({
			let mut inner = VectorOfPoint2f::new();
			inner.push(Point2f::new(4., 4.));
			inner.push(Point2f::new(5., 5.));
			inner.push(Point2f::new(6., 6.));
			inner
		});
		outer.push({
			let mut inner = VectorOfPoint2f::new();
			inner.push(Point2f::new(7., 7.));
			inner.push(Point2f::new(8., 8.));
			inner.push(Point2f::new(9., 9.));
			inner
		});
		outer
	}

	let mut outer = make_vec();
	assert_eq!(6., outer.get(1)?.get(2)?.x);
	outer.remove(1)?;
	assert_eq!(9., outer.get(1)?.get(2)?.x);
	Ok(())
}

#[test]
fn from_elem() {
	let v = VectorOfi32::from_elem(5, 10);
	assert_eq!(10, v.len());
	assert!(v.iter().all(|v| v == 5));

	let v = VectorOfMat::from_elem(Mat::default(), 0);
	assert_eq!(0, v.len());

	let v = VectorOfString::from_elem("test", 3);
	assert_eq!(3, v.len());
	assert!(v.iter().all(|v| v == "test"));
}

#[test]
fn capacity() {
	{
		let mut vec = VectorOfi32::with_capacity(0);
		assert_eq!(0, vec.len());
		assert!(vec.is_empty());
		assert_eq!(0, vec.capacity());
		vec.shrink_to_fit();
		assert_eq!(0, vec.capacity());
	}

	{
		let mut vec = VectorOfi32::with_capacity(10);
		assert_eq!(0, vec.len());
		assert!(vec.is_empty());
		assert_eq!(10, vec.capacity());
		vec.push(1);
		assert_eq!(1, vec.len());
		assert!(!vec.is_empty());
		assert_eq!(10, vec.capacity());
		vec.reserve(10);
		assert_eq!(1, vec.len());
		assert_eq!(11, vec.capacity());
		vec.reserve(10);
		assert!(!vec.is_empty());
		assert_eq!(11, vec.capacity());
		vec.shrink_to_fit();
		assert_eq!(1, vec.capacity());
	}

	{
		let mut vec = VectorOfbool::new();
		assert_eq!(0, vec.len());
		assert!(vec.is_empty());
		assert_eq!(0, vec.capacity());
		vec.push(true);
		assert_eq!(1, vec.len());
		assert!(!vec.is_empty());
		assert!(vec.capacity() > 0);
	}
}

#[test]
fn insert() -> Result<()> {
	let mut vec = VectorOfi32::from_iter(vec![1, 2, 3]);
	vec.insert(1, 4)?;
	assert_eq!(4, vec.len());
	assert_eq!(4, vec.get(1)?);
	vec.insert(0, 8)?;
	assert_eq!(5, vec.len());
	assert_eq!(8, vec.get(0)?);
	assert_eq!(1, vec.get(1)?);
	assert_matches!(
		vec.insert(10, 10),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	vec.insert(5, 10)?;
	assert_eq!(6, vec.len());
	assert_eq!(10, vec.get(5)?);
	Ok(())
}

#[test]
fn remove() -> Result<()> {
	let mut vec = VectorOfi32::with_capacity(10);
	vec.push(10);
	vec.push(20);
	vec.push(30);
	assert_eq!(3, vec.len());
	assert_eq!(10, vec.capacity());
	vec.remove(1)?;
	assert_eq!(2, vec.len());
	assert_eq!(10, vec.get(0)?);
	assert_eq!(30, vec.get(1)?);
	vec.remove(0)?;
	assert_eq!(1, vec.len());
	assert_eq!(30, vec.get(0)?);
	vec.clear();
	assert_eq!(0, vec.len());
	assert_eq!(10, vec.capacity());
	assert_matches!(
		vec.remove(0),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	Ok(())
}

#[test]
fn swap() -> Result<()> {
	{
		let mut vec = VectorOfi32::from_iter(vec![1, 2, 3]);
		vec.swap(0, 0)?;
		vec.swap(0, 1)?;
		assert_eq!(2, vec.get(0)?);
		assert_eq!(1, vec.get(1)?);

		vec.swap(2, 0)?;
		assert_eq!(3, vec.get(0)?);
		assert_eq!(1, vec.get(1)?);

		assert_matches!(
			vec.swap(0, 4),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			vec.swap(6, 1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
	}

	{
		let mut vec = VectorOfString::new();
		vec.push("123");
		vec.push("456");
		vec.push("789");
		vec.swap(0, 2)?;
		assert_eq!("789", vec.get(0)?);
		assert_eq!("456", vec.get(1)?);
		assert_eq!("123", vec.get(2)?);
	}

	#[allow(clippy::bool_assert_comparison)]
	{
		let mut vec = VectorOfbool::new();
		vec.push(true);
		vec.push(false);
		vec.push(true);
		vec.swap(0, 1)?;
		assert_eq!(false, vec.get(0)?);
		assert_eq!(true, vec.get(1)?);
		vec.swap(1, 2)?;
		assert_eq!(true, vec.get(2)?);
	}

	Ok(())
}

#[test]
fn nth() -> Result<()> {
	#[allow(clippy::iter_nth_zero)]
	{
		let mut vec = VectorOfi32::new();
		assert_eq!(None, vec.iter().nth(0));
		vec.push(1);
		vec.push(2);
		vec.push(3);
		assert_eq!(Some(1), vec.iter().nth(0));
		assert_eq!(Some(2), vec.iter().nth(1));
		assert_eq!(Some(3), vec.iter().nth(2));
		assert_eq!(None, vec.iter().nth(3));

		let mut iter = vec.iter();
		assert_eq!(Some(1), iter.nth(0));
		assert_eq!(Some(3), iter.nth(1));
		assert_eq!(None, iter.nth(0));
	}

	Ok(())
}

#[test]
fn out_of_bounds() -> Result<()> {
	let mut vec = VectorOff64::new();
	assert_matches!(
		vec.get(0),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	vec.push(1.);
	vec.push(2.);
	assert_matches!(
		vec.get(3),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	assert_matches!(
		vec.set(3, 5.),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	Ok(())
}

#[test]
fn from_iter() -> Result<()> {
	{
		let vec = VectorOfc_char::from_iter(vec![1, 2, 3]);
		assert_eq!(3, vec.len());
		assert_eq!(1, vec.get(0)?);
		assert_eq!(2, vec.get(1)?);
		assert_eq!(3, vec.get(2)?);
	}

	{
		let vec = VectorOfc_char::from_iter([1, 2, 3].iter().copied());
		assert_eq!(3, vec.len());
		assert_eq!(1, vec.get(0)?);
		assert_eq!(2, vec.get(1)?);
		assert_eq!(3, vec.get(2)?);
	}

	{
		let vec = VectorOfc_char::from_iter(1..=3);
		assert_eq!(3, vec.len());
		assert_eq!(1, vec.get(0)?);
		assert_eq!(2, vec.get(1)?);
		assert_eq!(3, vec.get(2)?);
	}

	Ok(())
}

#[test]
fn iter() -> Result<()> {
	{
		let vec = VectorOfi32::from_iter(vec![1, 2, 3, 4]);
		let mut sum = 0;
		for i in &vec {
			sum += i;
		}
		assert_eq!(10, sum);
		for i in vec {
			sum += i;
		}
		assert_eq!(20, sum);
	}

	{
		let vec = VectorOfMat::from_iter(vec![]);
		for _ in vec {
			panic!("iterator must not yield any elements")
		}
	}

	{
		let vec = VectorOfi32::from_iter(vec![1, 2, 3, 4]);
		let mut vec_iter = vec.into_iter();
		let mut len = vec_iter.len();
		assert_eq!(4, len);
		while let Some(n) = vec_iter.next() {
			len -= 1;
			assert_eq!(n, 4 - len as i32);
			assert_eq!(len, vec_iter.len());
			assert_eq!((len, Some(len)), vec_iter.size_hint());
		}
		assert_eq!((0, Some(0)), vec_iter.size_hint());
	}

	{
		let vec = VectorOfi32::from_iter(vec![1, 2, 3, 4]);
		let mut vec_iter = vec.iter();
		let mut len = vec_iter.len();
		assert_eq!(4, len);
		while let Some(n) = vec_iter.next() {
			len -= 1;
			assert_eq!(n, 4 - len as i32);
			assert_eq!(len, vec_iter.len());
			assert_eq!((len, Some(len)), vec_iter.size_hint());
		}
		assert_eq!((0, Some(0)), vec_iter.size_hint());
	}

	{
		let vec = VectorOfi32::from_iter(vec![1, 2, 3, 4]);
		let mut vec_iter = vec.iter();
		assert_eq!(Some(1), vec_iter.next());
		let mut len = vec_iter.len();
		assert_eq!(3, len);
		let mut vec_iter_clone = vec_iter.clone();
		while let Some(n) = vec_iter.next() {
			len -= 1;
			assert_eq!(n, 4 - len as i32);
			assert_eq!(len, vec_iter.len());
			assert_eq!((len, Some(len)), vec_iter.size_hint());
		}
		let mut len = vec_iter_clone.len();
		assert_eq!(3, len);
		while let Some(n) = vec_iter_clone.next() {
			len -= 1;
			assert_eq!(n, 4 - len as i32);
			assert_eq!(len, vec_iter_clone.len());
			assert_eq!((len, Some(len)), vec_iter_clone.size_hint());
		}
	}

	// clone works for iterators for &Vectors of non-Clone elements
	{
		let _v = VectorOfRange::new().iter().clone();
	}

	Ok(())
}

#[test]
fn as_slice() -> Result<()> {
	{
		let src = vec![1, 2, 3, 4, 5];
		let mut vec = VectorOfu8::from_iter(src.clone());
		assert_eq!(vec.as_slice(), src.as_slice());
		assert_eq!(vec.as_mut_slice(), src.as_slice());
		vec.as_mut_slice()[2] = 10;
		vec.as_mut_slice()[4] = 15;
		assert_eq!(vec.as_mut_slice(), &[1, 2, 10, 4, 15]);
	}
	{
		let mut vec = VectorOfi32::new();
		vec.push(5);
		vec.push(10);
		assert_eq!(vec.as_slice(), &[5, 10]);
		vec.as_mut_slice().swap(0, 1);
		assert_eq!(vec.as_slice(), &[10, 5]);
	}
	{
		let src = vec![Point2d::new(10., 20.), Point2d::new(60.5, 90.3), Point2d::new(-40.333, 89.)];
		let mut vec = VectorOfPoint2d::from_iter(src);
		let slice = vec.as_slice();
		assert_eq!(20., slice[0].y);
		assert_eq!(60.5, slice[1].x);
		assert_eq!(Point2d::new(-40.333, 89.), slice[2]);
		let slice = vec.as_mut_slice();
		slice[0].x = 15.;
		slice[0].y = 90.;
		slice[1] = Point2d::new(91.5, 92.6);
		slice.swap(0, 1);
		assert_eq!(
			&[Point2d::new(91.5, 92.6), Point2d::new(15., 90.), Point2d::new(-40.333, 89.)],
			vec.as_slice()
		);
	}
	{
		let default = DMatch::default()?;
		let src = vec![DMatch::default()?, DMatch::new(1, 2, 9.)?];
		let mut vec = VectorOfDMatch::from_iter(src);
		let slice = vec.as_slice();
		assert_eq!(slice[0].distance, default.distance);
		assert_eq!(slice[0].img_idx, default.img_idx);
		assert_eq!(slice[1], DMatch::new(1, 2, 9.)?);
		let slice = vec.as_mut_slice();
		slice[0].distance = 15.;
		slice[0].img_idx = 8;
		slice[1] = DMatch::new_index(1, 2, 3, 98.)?;
		slice.swap(0, 1);
		assert_eq!(
			&[
				DMatch::new_index(1, 2, 3, 98.)?,
				DMatch::new_index(default.query_idx, default.train_idx, 8, 15.)?,
			],
			vec.as_slice()
		);
	}
	Ok(())
}

#[test]
fn to_vec() -> Result<()> {
	{
		let src_vec = vec![1, 2, 3, 4, 5];
		let vec: VectorOfu8 = VectorOfu8::from_iter(src_vec.clone());
		assert_eq!(vec.to_vec(), src_vec);
		assert_eq!(Vec::from(vec), src_vec);
		let vec_new: Vec<u8> = VectorOfu8::from_iter(src_vec.clone()).into();
		assert_eq!(vec_new, src_vec);
	}
	{
		let src_vec = vec![Vec4i::from([1, 2, 3, 4])];
		let mut vec = VectorOfVec4i::new();
		vec.push(Vec4i::from([1, 2, 3, 4]));
		assert_eq!(vec.to_vec(), src_vec);
		assert_eq!(Vec::from(vec), src_vec);
		let vec_new: Vec<Vec4i> = VectorOfVec4i::new().into();
		assert_eq!(vec_new, vec![]);
	}
	{
		let mut vec = VectorOfMat::new();
		vec.push(Mat::new_rows_cols_with_default(5, 8, u16::opencv_type(), Scalar::all(8.))?);
		vec.push(Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), Scalar::all(19.))?);
		let mat_vec = vec.to_vec();
		assert_eq!(vec.get(0)?.total(), mat_vec[0].total());
		assert_eq!(vec.get(0)?.typ(), mat_vec[0].typ());
		assert_eq!(vec.get(0)?.at_2d::<u16>(2, 2)?, mat_vec[0].at_2d::<u16>(2, 2)?);
		assert_eq!(vec.get(1)?.total(), mat_vec[1].total());
		assert_eq!(vec.get(1)?.typ(), mat_vec[1].typ());
		assert_eq!(vec.get(1)?.at_2d::<u8>(2, 2)?, mat_vec[1].at_2d::<u8>(2, 2)?);
	}

	Ok(())
}

#[test]
fn property() -> Result<()> {
	let mut hdr = SparseMat_Hdr::new(&[4, 2], i32::opencv_type())?;
	#[inline(never)]
	fn f(hdr: &mut SparseMat_Hdr, pool: VectorOfu8) {
		hdr.set_pool(pool);
	}
	let pool = VectorOfu8::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9].iter().copied());
	f(&mut hdr, pool);
	let pool = VectorOfu8::from_iter([11, 12, 13, 14, 15, 16, 17, 18, 19].iter().copied());
	let pool_out = hdr.pool();
	assert_eq!(pool_out.get(0)?, pool.get(0)? - 10);
	assert_eq!(pool_out.get(2)?, pool.get(2)? - 10);
	assert_eq!(pool_out.get(4)?, pool.get(4)? - 10);
	assert_eq!(pool_out.get(8)?, pool.get(8)? - 10);
	assert_eq!(pool_out.len(), pool.len());
	Ok(())
}

#[test]
fn clone() -> Result<()> {
	{
		let src = vec![1, 2, 3, 4, 5, 6];
		let mut primitive = VectorOfi32::from_iter(src.clone());
		let primitive_clone = primitive.clone();
		primitive.remove(2)?;
		assert_eq!(primitive.len() + 1, primitive_clone.len());
		primitive.set(1, 10)?;
		assert_eq!(10, primitive.get(1)?);
		primitive.as_mut_slice()[2] = 80;
		assert_eq!(80, primitive.get(2)?);
		drop(primitive);
		assert_eq!(primitive_clone.as_slice(), src.as_slice())
	}
	{
		let src = vec![Point2d::new(1., 2.), Point2d::new(3., 4.), Point2d::new(5., 6.)];
		let mut simple = VectorOfPoint2d::from_iter(src.clone());
		let simple_clone = simple.clone();
		simple.remove(2)?;
		assert_eq!(simple.len() + 1, simple_clone.len());
		simple.set(0, Point2d::new(10., 11.))?;
		assert_eq!(10., simple.get(0)?.x);
		assert_eq!(11., simple.get(0)?.y);
		simple.as_mut_slice()[1] = Point2d::new(20., 21.);
		assert_eq!(20., simple.get(1)?.x);
		assert_eq!(21., simple.get(1)?.y);
		drop(simple);
		assert_eq!(simple_clone.as_slice(), src.as_slice())
	}
	{
		let mut src = vec![
			Mat::new_rows_cols_with_default(10, 20, f64::opencv_type(), Scalar::from(10.))?,
			Mat::new_rows_cols_with_default(5, 8, i32::opencv_type(), Scalar::from(20.))?,
		];
		let src_clone = src.clone();
		assert_eq!(20, *src[1].at_2d::<i32>(2, 2)?);
		assert_eq!(20, *src_clone[1].at_2d::<i32>(2, 2)?);
		*src[1].at_2d_mut::<i32>(2, 2)? = 30;
		assert_eq!(30, *src[1].at_2d::<i32>(2, 2)?);
		assert_eq!(20, *src_clone[1].at_2d::<i32>(2, 2)?);
		let mut boxed = VectorOfMat::from_iter(src_clone);
		let boxed_clone = boxed.clone();
		assert_eq!(20, *boxed.get(1)?.at_2d::<i32>(2, 2)?);
		assert_eq!(20, *boxed_clone.get(1)?.at_2d::<i32>(2, 2)?);
		*boxed.get(1)?.at_2d_mut::<i32>(2, 2)? = 30;
		assert_eq!(30, *boxed.get(1)?.at_2d::<i32>(2, 2)?);
		assert_eq!(20, *boxed_clone.get(1)?.at_2d::<i32>(2, 2)?);
		boxed.remove(1)?;
		assert_eq!(boxed.len() + 1, boxed_clone.len());
		boxed.set(
			0,
			Mat::new_rows_cols_with_default(40, 50, f64::opencv_type(), Scalar::from(40.))?,
		)?;
		assert_eq!(40., *boxed.get(0)?.at_2d::<f64>(2, 2)?);
		assert_eq!(10., *boxed_clone.get(0)?.at_2d::<f64>(2, 2)?);
		drop(boxed);
		assert_eq!(10., *boxed_clone.get(0)?.at_2d::<f64>(2, 2)?);
	}
	{
		let src = vec![
			VectorOfPoint2f::from_iter(vec![Point2f::new(10., 11.), Point2f::new(12., 13.)]),
			VectorOfPoint2f::from_iter(vec![Point2f::new(20., 21.), Point2f::new(22., 23.)]),
			VectorOfPoint2f::from_iter(vec![Point2f::new(30., 31.), Point2f::new(32., 33.)]),
		];
		let mut vec_of_vec = VectorOfVectorOfPoint2f::from_iter(src);
		let vec_of_vec_clone = vec_of_vec.clone();
		assert_eq!(21., vec_of_vec.get(1)?.get(0)?.y);
		assert_eq!(21., vec_of_vec_clone.get(1)?.get(0)?.y);
		vec_of_vec.set(
			1,
			VectorOfPoint2f::from_iter(vec![Point2f::new(40., 41.), Point2f::new(42., 43.)]),
		)?;
		assert_eq!(41., vec_of_vec.get(1)?.get(0)?.y);
		assert_eq!(21., vec_of_vec_clone.get(1)?.get(0)?.y);
	}

	Ok(())
}

#[test]
fn from_slice() -> Result<()> {
	{
		let bytes: &[u8] = &[1, 2, 3, 4, 5];
		let v = VectorOfu8::from_slice(bytes);
		assert_eq!(bytes.len(), v.len());
		assert_eq!(bytes, v.as_slice());
	}

	{
		let ints: &[i32] = &[5, 10, 15, 20];
		let v = VectorOfi32::from_slice(ints);
		assert_eq!(ints.len(), v.len());
		assert_eq!(ints, v.as_slice());
	}

	{
		let points: &[Point2d] = &[Point2d::new(10., 40.), Point2d::new(0., -55.), Point2d::new(100., -1.)];
		let v = VectorOfPoint2d::from_slice(points);
		assert_eq!(points.len(), v.len());
		assert_eq!(points, v.as_slice());
	}

	{
		let bytes: &[u8] = &[1, 2, 3, 4, 5];
		let mut v = VectorOfu8::from_slice(bytes);
		v.set(2, 10)?;
		assert_eq!(v.get(2)?, 10);
		assert_eq!(bytes[2], 3);
	}

	Ok(())
}

#[test]
fn must_be_clone() {
	fn must_be_clone(_: impl Clone) {}

	must_be_clone(VectorOfi32::new());
	must_be_clone(VectorOfKeyPoint::new());
}

#[test]
fn send() {
	fn must_be_send(_: impl Send) {}

	must_be_send(VectorOfi32::new());
	must_be_send(VectorOfMat::new());
	must_be_send(VectorOfString::new());
	must_be_send(VectorOfPoint2d::new());
}

#[test]
fn sync() {
	fn must_be_sync(_: impl Sync) {}

	must_be_sync(VectorOfi32::new());
	must_be_sync(VectorOfString::new());
	must_be_sync(VectorOfPoint2d::new());
}
