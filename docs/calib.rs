pub mod calib {
	//! # Camera Calibration
	//!
	//! The functions in this section use a so-called pinhole camera model. The view of a scene
	//! is obtained by projecting a scene's 3D point ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw) into the image plane using a perspective
	//! transformation which forms the corresponding pixel ![inline formula](https://latex.codecogs.com/png.latex?p). Both ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw) and ![inline formula](https://latex.codecogs.com/png.latex?p) are
	//! represented in homogeneous coordinates, i.e. as 3D and 2D homogeneous vector respectively. You will
	//! find a brief introduction to projective geometry, homogeneous vectors and homogeneous
	//! transformations at the end of this section's introduction. For more succinct notation, we often drop
	//! the 'homogeneous' and say vector instead of homogeneous vector.
	//!
	//! The distortion-free projective transformation given by a  pinhole camera model is shown below.
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?s%20%5C%3B%20p%20%3D%20A%20%5Cbegin%7Bbmatrix%7D%20R%7Ct%20%5Cend%7Bbmatrix%7D%20P%5Fw%2C)
	//!
	//! where ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw) is a 3D point expressed with respect to the world coordinate system,
	//! ![inline formula](https://latex.codecogs.com/png.latex?p) is a 2D pixel in the image plane, ![inline formula](https://latex.codecogs.com/png.latex?A) is the camera intrinsic matrix,
	//! ![inline formula](https://latex.codecogs.com/png.latex?R) and ![inline formula](https://latex.codecogs.com/png.latex?t) are the rotation and translation that describe the change of coordinates from
	//! world to camera coordinate systems (or camera frame) and ![inline formula](https://latex.codecogs.com/png.latex?s) is the projective transformation's
	//! arbitrary scaling and not part of the camera model.
	//!
	//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?A) (notation used as in [Zhang2000](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Zhang2000) and also generally notated
	//! as ![inline formula](https://latex.codecogs.com/png.latex?K)) projects 3D points given in the camera coordinate system to 2D pixel coordinates, i.e.
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?p%20%3D%20A%20P%5Fc%2E)
	//!
	//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?A) is composed of the focal lengths ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy), which are
	//! expressed in pixel units, and the principal point ![inline formula](https://latex.codecogs.com/png.latex?%28c%5Fx%2C%20c%5Fy%29), that is usually close to the
	//! image center:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D%2C)
	//!
	//! and thus
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?s%20%5Cbegin%7Bbmatrix%7D%20u%5C%5C%20v%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%20X%5Fc%5C%5C%20Y%5Fc%5C%5C%20Z%5Fc%20%5Cend%7Bbmatrix%7D%2E)
	//!
	//! The matrix of intrinsic parameters does not depend on the scene viewed. So, once estimated, it can
	//! be re-used as long as the focal length is fixed (in case of a zoom lens). Thus, if an image from the
	//! camera is scaled by a factor, all of these parameters need to be scaled (multiplied/divided,
	//! respectively) by the same factor.
	//!
	//! The joint rotation-translation matrix ![inline formula](https://latex.codecogs.com/png.latex?%5BR%7Ct%5D) is the matrix product of a projective
	//! transformation and a homogeneous transformation. The 3-by-4 projective transformation maps 3D points
	//! represented in camera coordinates to 2D points in the image plane and represented in normalized
	//! camera coordinates ![inline formula](https://latex.codecogs.com/png.latex?x%27%20%3D%20X%5Fc%20%2F%20Z%5Fc) and ![inline formula](https://latex.codecogs.com/png.latex?y%27%20%3D%20Y%5Fc%20%2F%20Z%5Fc):
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cbegin%7Bbmatrix%7D%0Ax%27%20%5C%5C%0Ay%27%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A1%20%26%200%20%26%200%20%26%200%20%5C%5C%0A0%20%26%201%20%26%200%20%26%200%20%5C%5C%0A0%20%26%200%20%26%201%20%26%200%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fc%20%5C%5C%0AY%5Fc%20%5C%5C%0AZ%5Fc%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	//!
	//! The homogeneous transformation is encoded by the extrinsic parameters ![inline formula](https://latex.codecogs.com/png.latex?R) and ![inline formula](https://latex.codecogs.com/png.latex?t) and
	//! represents the change of basis from world coordinate system ![inline formula](https://latex.codecogs.com/png.latex?w) to the camera coordinate sytem
	//! ![inline formula](https://latex.codecogs.com/png.latex?c). Thus, given the representation of the point ![inline formula](https://latex.codecogs.com/png.latex?P) in world coordinates, ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw), we
	//! obtain ![inline formula](https://latex.codecogs.com/png.latex?P)'s representation in the camera coordinate system, ![inline formula](https://latex.codecogs.com/png.latex?P%5Fc), by
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?P%5Fc%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20P%5Fw%2C)
	//!
	//! This homogeneous transformation is composed out of ![inline formula](https://latex.codecogs.com/png.latex?R), a 3-by-3 rotation matrix, and ![inline formula](https://latex.codecogs.com/png.latex?t), a
	//! 3-by-1 translation vector:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%20%5C%5C%0A0%20%26%200%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D%2C%0A)
	//!
	//! and therefore
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5Fc%20%5C%5C%0AY%5Fc%20%5C%5C%0AZ%5Fc%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%20%5C%5C%0A0%20%26%200%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	//!
	//! Combining the projective transformation and the homogeneous transformation, we obtain the projective
	//! transformation that maps 3D points in world coordinates into 2D points in the image plane and in
	//! normalized camera coordinates:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cbegin%7Bbmatrix%7D%0Ax%27%20%5C%5C%0Ay%27%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%20R%7Ct%20%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2C)
	//!
	//! with ![inline formula](https://latex.codecogs.com/png.latex?x%27%20%3D%20X%5Fc%20%2F%20Z%5Fc) and ![inline formula](https://latex.codecogs.com/png.latex?y%27%20%3D%20Y%5Fc%20%2F%20Z%5Fc). Putting the equations for instrincs and extrinsics together, we can write out
	//! ![inline formula](https://latex.codecogs.com/png.latex?s%20%5C%3B%20p%20%3D%20A%20%5Cbegin%7Bbmatrix%7D%20R%7Ct%20%5Cend%7Bbmatrix%7D%20P%5Fw) as
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?s%20%5Cbegin%7Bbmatrix%7D%20u%5C%5C%20v%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	//!
	//! If ![inline formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cne%200), the transformation above is equivalent to the following,
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Au%20%5C%5C%0Av%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Af%5Fx%20X%5Fc%2FZ%5Fc%20%2B%20c%5Fx%20%5C%5C%0Af%5Fy%20Y%5Fc%2FZ%5Fc%20%2B%20c%5Fy%0A%5Cend%7Bbmatrix%7D)
	//!
	//! with
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20X%5Fc%5C%5C%20Y%5Fc%5C%5C%20Z%5Fc%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%7Ct%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	//!
	//! The following figure illustrates the pinhole camera model.
	//!
	//! ![Pinhole camera model](https://docs.opencv.org/5.0.0/pinhole_camera_model.png) { width=70% }
	//!
	//! Real lenses usually have some distortion, mostly radial distortion, and slight tangential distortion.
	//! So, the above model is extended as:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Au%20%5C%5C%0Av%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Af%5Fx%20x%27%27%20%2B%20c%5Fx%20%5C%5C%0Af%5Fy%20y%27%27%20%2B%20c%5Fy%0A%5Cend%7Bbmatrix%7D)
	//!
	//! where
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Ax%27%27%20%5C%5C%0Ay%27%27%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ax%27%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%20%2B%202%20p%5F1%20x%27%20y%27%20%2B%20p%5F2%28r%5E2%20%2B%202%20x%27%5E2%29%20%2B%20s%5F1%20r%5E2%20%2B%20s%5F2%20r%5E4%20%5C%5C%0Ay%27%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%20%2B%20p%5F1%20%28r%5E2%20%2B%202%20y%27%5E2%29%20%2B%202%20p%5F2%20x%27%20y%27%20%2B%20s%5F3%20r%5E2%20%2B%20s%5F4%20r%5E4%20%5C%5C%0A%5Cend%7Bbmatrix%7D)
	//!
	//! with
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?r%5E2%20%3D%20x%27%5E2%20%2B%20y%27%5E2)
	//!
	//! and
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Ax%27%5C%5C%0Ay%27%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AX%5Fc%2FZ%5Fc%20%5C%5C%0AY%5Fc%2FZ%5Fc%0A%5Cend%7Bbmatrix%7D%2C)
	//!
	//! if ![inline formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cne%200).
	//!
	//! The distortion parameters are the radial coefficients ![inline formula](https://latex.codecogs.com/png.latex?k%5F1), ![inline formula](https://latex.codecogs.com/png.latex?k%5F2), ![inline formula](https://latex.codecogs.com/png.latex?k%5F3), ![inline formula](https://latex.codecogs.com/png.latex?k%5F4), ![inline formula](https://latex.codecogs.com/png.latex?k%5F5), and ![inline formula](https://latex.codecogs.com/png.latex?k%5F6)
	//! ,![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are the tangential distortion coefficients, and ![inline formula](https://latex.codecogs.com/png.latex?s%5F1), ![inline formula](https://latex.codecogs.com/png.latex?s%5F2), ![inline formula](https://latex.codecogs.com/png.latex?s%5F3), and ![inline formula](https://latex.codecogs.com/png.latex?s%5F4),
	//! are the thin prism distortion coefficients. Higher-order coefficients are not considered in OpenCV.
	//!
	//! The next figures show two common types of radial distortion: barrel distortion
	//! (![inline formula](https://latex.codecogs.com/png.latex?%201%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%20) monotonically decreasing)
	//! and pincushion distortion (![inline formula](https://latex.codecogs.com/png.latex?%201%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%20) monotonically increasing).
	//! Radial distortion is always monotonic for real lenses,
	//! and if the estimator produces a non-monotonic result,
	//! this should be considered a calibration failure.
	//! More generally, radial distortion must be monotonic and the distortion function must be bijective.
	//! A failed estimation result may look deceptively good near the image center
	//! but will work poorly in e.g. AR/SFM applications.
	//! The optimization method used in OpenCV camera calibration does not include these constraints as
	//! the framework does not support the required integer programming and polynomial inequalities.
	//! See [issue #15992](https://github.com/opencv/opencv/issues/15992) for additional information.
	//!
	//! ![](https://docs.opencv.org/5.0.0/distortion_examples.png)
	//! ![](https://docs.opencv.org/5.0.0/distortion_examples2.png)
	//!
	//! In some cases, the image sensor may be tilted in order to focus an oblique plane in front of the
	//! camera (Scheimpflug principle). This can be useful for particle image velocimetry (PIV) or
	//! triangulation with a laser fan. The tilt causes a perspective distortion of ![inline formula](https://latex.codecogs.com/png.latex?x%27%27) and
	//! ![inline formula](https://latex.codecogs.com/png.latex?y%27%27). This distortion can be modeled in the following way, see e.g. [Louhichi07](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Louhichi07).
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Au%20%5C%5C%0Av%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Af%5Fx%20x%27%27%27%20%2B%20c%5Fx%20%5C%5C%0Af%5Fy%20y%27%27%27%20%2B%20c%5Fy%0A%5Cend%7Bbmatrix%7D%2C)
	//!
	//! where
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?s%5Cbegin%7Bbmatrix%7D%20x%27%27%27%5C%5C%20y%27%27%27%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%0A%5Cvecthreethree%7BR%5F%7B33%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%7B0%7D%7B%2DR%5F%7B13%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%0A%7B0%7D%7BR%5F%7B33%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%7B%2DR%5F%7B23%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%0A%7B0%7D%7B0%7D%7B1%7D%20R%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%20%5Cbegin%7Bbmatrix%7D%20x%27%27%5C%5C%20y%27%27%5C%5C%201%20%5Cend%7Bbmatrix%7D)
	//!
	//! and the matrix ![inline formula](https://latex.codecogs.com/png.latex?R%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) is defined by two rotations with angular parameter
	//! ![inline formula](https://latex.codecogs.com/png.latex?%5Ctau%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?%5Ctau%5Fy), respectively,
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%0AR%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%20%3D%0A%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctau%5Fy%29%20%26%200%20%26%20%2D%5Csin%28%5Ctau%5Fy%29%5C%5C%200%20%26%201%20%26%200%5C%5C%20%5Csin%28%5Ctau%5Fy%29%20%26%200%20%26%20%5Ccos%28%5Ctau%5Fy%29%20%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%201%20%26%200%20%26%200%5C%5C%200%20%26%20%5Ccos%28%5Ctau%5Fx%29%20%26%20%5Csin%28%5Ctau%5Fx%29%5C%5C%200%20%26%20%2D%5Csin%28%5Ctau%5Fx%29%20%26%20%5Ccos%28%5Ctau%5Fx%29%20%5Cend%7Bbmatrix%7D%20%3D%0A%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctau%5Fy%29%20%26%20%5Csin%28%5Ctau%5Fy%29%5Csin%28%5Ctau%5Fx%29%20%26%20%2D%5Csin%28%5Ctau%5Fy%29%5Ccos%28%5Ctau%5Fx%29%5C%5C%200%20%26%20%5Ccos%28%5Ctau%5Fx%29%20%26%20%5Csin%28%5Ctau%5Fx%29%5C%5C%20%5Csin%28%5Ctau%5Fy%29%20%26%20%2D%5Ccos%28%5Ctau%5Fy%29%5Csin%28%5Ctau%5Fx%29%20%26%20%5Ccos%28%5Ctau%5Fy%29%5Ccos%28%5Ctau%5Fx%29%20%5Cend%7Bbmatrix%7D%2E%0A)
	//!
	//! In the functions below the coefficients are passed or returned as
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
	//!
	//! vector. That is, if the vector contains four elements, it means that ![inline formula](https://latex.codecogs.com/png.latex?k%5F3%3D0) . The distortion
	//! coefficients do not depend on the scene viewed. Thus, they also belong to the intrinsic camera
	//! parameters. And they remain the same regardless of the captured image resolution. If, for example, a
	//! camera has been calibrated on images of 320 x 240 resolution, absolutely the same distortion
	//! coefficients can be used for 640 x 480 images from the same camera while ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx), ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy),
	//! ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx), and ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) need to be scaled appropriately.
	//!
	//! The functions below use the above model to do the following:
	//!
	//! *   Project 3D points to the image plane given intrinsic and extrinsic parameters.
	//! *   Compute extrinsic parameters given intrinsic parameters, a few 3D points, and their
	//! projections.
	//! *   Estimate intrinsic and extrinsic camera parameters from several views of a known calibration
	//! pattern (every view is described by several 3D-2D point correspondences).
	//! *   Estimate the relative position and orientation of the stereo camera "heads" and compute the
	//! *rectification* transformation that makes the camera optical axes parallel.
	//!
	//! <B> Homogeneous Coordinates </B><br>
	//! Homogeneous Coordinates are a system of coordinates that are used in projective geometry. Their use
	//! allows to represent points at infinity by finite coordinates and simplifies formulas when compared
	//! to the cartesian counterparts, e.g. they have the advantage that affine transformations can be
	//! expressed as linear homogeneous transformation.
	//!
	//! One obtains the homogeneous vector ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh) by appending a 1 along an n-dimensional cartesian
	//! vector ![inline formula](https://latex.codecogs.com/png.latex?P) e.g. for a 3D cartesian vector the mapping ![inline formula](https://latex.codecogs.com/png.latex?P%20%5Crightarrow%20P%5Fh) is:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%0A%5Cend%7Bbmatrix%7D%20%5Crightarrow%20%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	//!
	//! For the inverse mapping ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh%20%5Crightarrow%20P), one divides all elements of the homogeneous vector
	//! by its last element, e.g. for a 3D homogeneous vector one gets its 2D cartesian counterpart by:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AW%0A%5Cend%7Bbmatrix%7D%20%5Crightarrow%20%5Cbegin%7Bbmatrix%7D%0AX%20%2F%20W%20%5C%5C%0AY%20%2F%20W%0A%5Cend%7Bbmatrix%7D%2C)
	//!
	//! if ![inline formula](https://latex.codecogs.com/png.latex?W%20%5Cne%200).
	//!
	//! Due to this mapping, all multiples ![inline formula](https://latex.codecogs.com/png.latex?k%20P%5Fh), for ![inline formula](https://latex.codecogs.com/png.latex?k%20%5Cne%200), of a homogeneous point represent
	//! the same point ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh). An intuitive understanding of this property is that under a projective
	//! transformation, all multiples of ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh) are mapped to the same point. This is the physical
	//! observation one does for pinhole cameras, as all points along a ray through the camera's pinhole are
	//! projected to the same image point, e.g. all points along the red ray in the image of the pinhole
	//! camera model above would be mapped to the same image coordinate. This property is also the source
	//! for the scale ambiguity s in the equation of the pinhole camera model.
	//!
	//! As mentioned, by using homogeneous coordinates we can express any change of basis parameterized by
	//! ![inline formula](https://latex.codecogs.com/png.latex?R) and ![inline formula](https://latex.codecogs.com/png.latex?t) as a linear transformation, e.g. for the change of basis from coordinate system
	//! 0 to coordinate system 1 becomes:
	//!
	//! ![block formula](https://latex.codecogs.com/png.latex?P%5F1%20%3D%20R%20P%5F0%20%2B%20t%20%5Crightarrow%20P%5F%7Bh%5F1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20P%5F%7Bh%5F0%7D%2E)
	//!
	//! <B> Homogeneous Transformations, Object frame / Camera frame </B><br>
	//! Change of basis or computing the 3D coordinates from one frame to another frame can be achieved easily using
	//! the following notation:
	//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cmathbf%7BX%7D%5Fc%20%3D%20%5Chspace%7B0%2E2em%7D%0A%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fo%0A)
	//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fc%20%5C%5C%0AY%5Fc%20%5C%5C%0AZ%5Fc%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%0A%5Cbegin%7Bbmatrix%7D%0A%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5Fo%20%26%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20%5C%5C%0A0%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fo%20%5C%5C%0AY%5Fo%20%5C%5C%0AZ%5Fo%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%0A)
	//! For a 3D points (![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BX%7D%5Fo%20)) expressed in the object frame, the homogeneous transformation matrix
	//! ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) allows computing the corresponding coordinate (![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BX%7D%5Fc%20)) in the camera frame.
	//! This transformation matrix is composed of a 3x3 rotation matrix ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5Fo%20) and a 3x1 translation vector
	//! ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20).
	//! The 3x1 translation vector ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20) is the position of the object frame in the camera frame and the
	//! 3x3 rotation matrix ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5Fo%20) the orientation of the object frame in the camera frame.
	//! With this simple notation, it is easy to chain the transformations. For instance, to compute the 3D coordinates of a point
	//! expressed in the object frame in the world frame can be done with:
	//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cmathbf%7BX%7D%5Fw%20%3D%20%5Chspace%7B0%2E2em%7D%0A%7B%7D%5E%7Bw%7D%5Cmathbf%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Chspace%7B0%2E2em%7D%0A%5Cmathbf%7BX%7D%5Fo%20%3D%0A%7B%7D%5E%7Bw%7D%5Cmathbf%7BT%7D%5Fo%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fo%0A)
	//! Similarly, computing the inverse transformation can be done with:
	//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cmathbf%7BX%7D%5Fo%20%3D%20%5Chspace%7B0%2E2em%7D%0A%7B%7D%5E%7Bo%7D%5Cmathbf%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fc%20%3D%0A%5Cleft%28%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Cright%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fc%0A)
	//! The inverse of an homogeneous transformation matrix is then:
	//! ![block formula](https://latex.codecogs.com/png.latex?%0A%7B%7D%5E%7Bo%7D%5Cmathbf%7BT%7D%5Fc%20%3D%20%5Cleft%28%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Cright%29%5E%7B%2D1%7D%20%3D%0A%5Cbegin%7Bbmatrix%7D%0A%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%5Fo%20%26%20%2D%20%5Chspace%7B0%2E2em%7D%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%5Fo%20%5Chspace%7B0%2E2em%7D%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20%5C%5C%0A0%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%5Cend%7Bbmatrix%7D%0A)
	//! One can note that the inverse of a 3x3 rotation matrix is directly its matrix transpose.
	//! ![Perspective projection, from object to camera frame](https://docs.opencv.org/5.0.0/pinhole_homogeneous_transformation.png)
	//! This figure summarizes the whole process. The object pose returned for instance by the [solvePnP] function
	//! or pose from fiducial marker detection is this ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) transformation.
	//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BK%7D%20) allows projecting the 3D point expressed in the camera frame onto the image plane
	//! assuming a perspective projection model (pinhole camera model). Image coordinates extracted from classical image processing functions
	//! assume a (u,v) top-left coordinates frame.
	//! \note
	//! - for an online video course on this topic, see for instance:
	//!   - ["3.3.1. Homogeneous Transformation Matrices", Modern Robotics, Kevin M. Lynch and Frank C. Park](https://modernrobotics.northwestern.edu/nu-gm-book-resource/3-3-1-homogeneous-transformation-matrices/)
	//! - the 3x3 rotation matrix is composed of 9 values but describes a 3 dof transformation
	//! - some additional properties of the 3x3 rotation matrix are:
	//!   - ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathrm%7Bdet%7D%20%5Cleft%28%20%5Cmathbf%7BR%7D%20%5Cright%29%20%3D%201%20)
	//!   - ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BR%7D%20%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%20%3D%20%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%20%5Cmathbf%7BR%7D%20%3D%20%5Cmathrm%7BI%7D%5F%7B3%20%5Ctimes%203%7D%20)
	//!   - interpolating rotation can be done using the [Slerp (spherical linear interpolation)](https://en.wikipedia.org/wiki/Slerp) method
	//! - quick conversions between the different rotation formalisms can be done using this [online tool](https://www.andre-gaschler.com/rotationconverter/)
	//! <B> Intrinsic parameters from camera lens specifications </B><br>
	//! When dealing with industrial cameras, the camera intrinsic matrix or more precisely ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cleft%28f%5Fx%2C%20f%5Fy%20%5Cright%29%20)
	//! can be deduced, approximated from the camera specifications:
	//! ![block formula](https://latex.codecogs.com/png.latex?%0Af%5Fx%20%3D%20%5Cfrac%7Bf%5F%7B%5Ctext%7Bmm%7D%7D%7D%7B%5Ctext%7Bpixel%5Fsize%5Fin%5Fmm%7D%7D%20%3D%20%5Cfrac%7Bf%5F%7B%5Ctext%7Bmm%7D%7D%7D%7B%5Ctext%7Bsensor%5Fsize%5Fin%5Fmm%7D%20%2F%20%5Ctext%7Bnb%5Fpixels%7D%7D%0A)
	//! In a same way, the physical focal length can be deduced from the angular field of view:
	//! ![block formula](https://latex.codecogs.com/png.latex?%0Af%5F%7B%5Ctext%7Bmm%7D%7D%20%3D%20%5Cfrac%7B%5Ctext%7Bsensor%5Fsize%5Fin%5Fmm%7D%7D%7B2%20%5Ctimes%20%5Ctan%7B%5Cfrac%7B%5Ctext%7Bfov%7D%7D%7B2%7D%7D%7D%0A)
	//! This latter conversion can be useful when using a rendering software to mimic a physical camera device.
	//!
	//!
	//! Note:
	//!    *    See also [calibration_matrix_values]
	//!
	//! <B> Additional references, notes </B><br>
	//!
	//! Note:
	//!    *   Many functions in this module take a camera intrinsic matrix as an input parameter. Although all
	//!        functions assume the same structure of this parameter, they may name it differently. The
	//!        parameter's description, however, will be clear in that a camera intrinsic matrix with the structure
	//!        shown above is required.
	//!    *   A calibration sample for 3 cameras in a horizontal position can be found at
	//!        opencv_source_code/samples/cpp/3calibration.cpp
	//!    *   A calibration sample based on a sequence of images can be found at
	//!        opencv_source_code/samples/cpp/calibration.cpp
	//!    *   A calibration sample in order to do 3D reconstruction can be found at
	//!        opencv_source_code/samples/cpp/build3dmodel.cpp
	//!    *   A calibration example on stereo calibration can be found at
	//!        opencv_source_code/samples/cpp/stereo_calib.cpp
	//!    *   A calibration example on stereo matching can be found at
	//!        opencv_source_code/samples/cpp/stereo_match.cpp
	//!    *   (Python) A camera calibration sample can be found at
	//!        opencv_source_code/samples/python/calibrate.py
	//!    # Fisheye camera model
	//!
	//!    Definitions: Let P be a point in 3D of coordinates X in the world reference frame (stored in the
	//!    matrix X) The coordinate vector of P in the camera reference frame is:
	//!
	//!    ![block formula](https://latex.codecogs.com/png.latex?Xc%20%3D%20R%20X%20%2B%20T)
	//!
	//!    where R is the rotation matrix corresponding to the rotation vector om: R = rodrigues(om); call x, y
	//!    and z the 3 coordinates of Xc:
	//!
	//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20x%20%3D%20Xc%5F1%20%5C%5C%20y%20%3D%20Xc%5F2%20%5C%5C%20z%20%3D%20Xc%5F3%20%5Cend%7Barray%7D%20)
	//!
	//!    The pinhole projection coordinates of P is [a; b] where
	//!
	//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20a%20%3D%20x%20%2F%20z%20%5C%20and%20%5C%20b%20%3D%20y%20%2F%20z%20%5C%5C%20r%5E2%20%3D%20a%5E2%20%2B%20b%5E2%20%5C%5C%20%5Ctheta%20%3D%20atan%28r%29%20%5Cend%7Barray%7D%20)
	//!
	//!    Fisheye distortion:
	//!
	//!    ![block formula](https://latex.codecogs.com/png.latex?%5Ctheta%5Fd%20%3D%20%5Ctheta%20%281%20%2B%20k%5F1%20%5Ctheta%5E2%20%2B%20k%5F2%20%5Ctheta%5E4%20%2B%20k%5F3%20%5Ctheta%5E6%20%2B%20k%5F4%20%5Ctheta%5E8%29)
	//!
	//!    The distorted point coordinates are [x'; y'] where
	//!
	//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20x%27%20%3D%20%28%5Ctheta%5Fd%20%2F%20r%29%20a%20%5C%5C%20y%27%20%3D%20%28%5Ctheta%5Fd%20%2F%20r%29%20b%20%5Cend%7Barray%7D%20)
	//!
	//!    Finally, conversion into pixel coordinates: The final pixel coordinates vector [u; v] where:
	//!
	//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20u%20%3D%20f%5Fx%20%28x%27%20%2B%20%5Calpha%20y%27%29%20%2B%20c%5Fx%20%5C%5C%0A%20%20%20%20v%20%3D%20f%5Fy%20y%27%20%2B%20c%5Fy%20%5Cend%7Barray%7D%20)
	//!
	//!    Summary:
	//!    Generic camera model [Kannala2006](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Kannala2006) with perspective projection and without distortion correction
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
	}

	/// For fisheye model only. Check SVD decomposition quality for each frame during extrinsics estimation
	pub const CALIB_CHECK_COND: i32 = 16777216;
	/// disable Schur complement (use Bouguet calibration engine)
	pub const CALIB_DISABLE_SCHUR_COMPLEMENT: i32 = 262144;
	/// Use with CALIB_USE_INTRINSIC_GUESS. The ratio fx/fy stays the same as in the input cameraMatrix.
	pub const CALIB_FIX_ASPECT_RATIO: i32 = 2;
	/// Use with CALIB_USE_INTRINSIC_GUESS. The focal length (fx, fy) stays the same as in the input cameraMatrix.
	pub const CALIB_FIX_FOCAL_LENGTH: i32 = 16;
	/// For stereo and milti-camera calibration only. Do not optimize cameras intrinsics
	pub const CALIB_FIX_INTRINSIC: i32 = 256;
	/// The corresponding distortion coefficient is not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_K1: i32 = 32;
	/// The corresponding distortion coefficient is not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_K2: i32 = 64;
	/// The corresponding distortion coefficient is not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_K3: i32 = 128;
	/// The corresponding distortion coefficient is not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_K4: i32 = 2048;
	/// For pinhole model only. The corresponding distortion coefficient is not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_K5: i32 = 4096;
	/// For pinhole model only. The corresponding distortion coefficient is not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_K6: i32 = 8192;
	/// The principal point (cx, cy) stays the same as in the input camera matrix. Image center is used as principal point, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_PRINCIPAL_POINT: i32 = 4;
	/// For pinhole model only. The thin prism distortion coefficients are not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_S1_S2_S3_S4: i32 = 65536;
	/// For fisheye model only. Skew coefficient (alpha) is set to zero and stay zero.
	pub const CALIB_FIX_SKEW: i32 = 33554432;
	/// For pinhole model only. Tangential distortion coefficients (p1,p2) are set to zeros and stay zero.
	pub const CALIB_FIX_TANGENT_DIST: i32 = 2097152;
	/// For pinhole model only. The tauX and tauY coefficients are not changed during the optimization. 0 value is used, if CALIB_USE_INTRINSIC_GUESS is not set.
	pub const CALIB_FIX_TAUX_TAUY: i32 = 524288;
	/// On-line Hand-Eye Calibration [Andreff99](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Andreff99)
	pub const CALIB_HAND_EYE_ANDREFF: i32 = 3;
	/// Hand-Eye Calibration Using Dual Quaternions [Daniilidis98](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Daniilidis98)
	pub const CALIB_HAND_EYE_DANIILIDIS: i32 = 4;
	/// Hand-eye Calibration [Horaud95](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Horaud95)
	pub const CALIB_HAND_EYE_HORAUD: i32 = 2;
	/// Robot Sensor Calibration: Solving AX = XB on the Euclidean Group [Park94](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Park94)
	pub const CALIB_HAND_EYE_PARK: i32 = 1;
	/// A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/Eye Calibration [Tsai89](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Tsai89)
	pub const CALIB_HAND_EYE_TSAI: i32 = 0;
	/// Fisheye camera model
	pub const CALIB_MODEL_FISHEYE: i32 = 1;
	/// Pinhole camera model
	pub const CALIB_MODEL_PINHOLE: i32 = 0;
	pub const CALIB_NINTRINSIC: i32 = 18;
	/// For pinhole model only. Use rational distortion model with coefficients k4..k6.
	pub const CALIB_RATIONAL_MODEL: i32 = 16384;
	/// For fisheye model only. Recompute board position on each calibration iteration
	pub const CALIB_RECOMPUTE_EXTRINSIC: i32 = 8388608;
	/// Simultaneous robot-world and hand-eye calibration using dual-quaternions and kronecker product [Li2010SimultaneousRA](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Li2010SimultaneousRA)
	pub const CALIB_ROBOT_WORLD_HAND_EYE_LI: i32 = 1;
	/// Solving the robot-world/hand-eye calibration problem using the kronecker product [Shah2013SolvingTR](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Shah2013SolvingTR)
	pub const CALIB_ROBOT_WORLD_HAND_EYE_SHAH: i32 = 0;
	/// For stereo calibration only. Use the same focal length for cameras in pair.
	pub const CALIB_SAME_FOCAL_LENGTH: i32 = 512;
	/// For multiview calibration only. Use stereo correspondence approach for initial extrinsics guess. Limitation: all cameras should have the same type.
	pub const CALIB_STEREO_REGISTRATION: i32 = 67108864;
	/// For pinhole model only. Use thin prism distortion model with coefficients s1..s4.
	pub const CALIB_THIN_PRISM_MODEL: i32 = 32768;
	/// For pinhole model only. Coefficients tauX and tauY are enabled in camera matrix.
	pub const CALIB_TILTED_MODEL: i32 = 262144;
	/// For stereo and multi-view calibration. Use user provided extrinsics (R, T) as initial point for optimization
	pub const CALIB_USE_EXTRINSIC_GUESS: i32 = 4194304;
	/// Use user provided intrinsics as initial point for optimization.
	pub const CALIB_USE_INTRINSIC_GUESS: i32 = 1;
	/// use LU instead of SVD decomposition for solving. much faster but potentially less precise
	pub const CALIB_USE_LU: i32 = 131072;
	/// Use QR instead of SVD decomposition for solving. Faster but potentially less precise
	pub const CALIB_USE_QR: i32 = 1048576;
	/// Deprecated synonim of [STEREO_ZERO_DISPARITY]. See [stereoRectify].
	pub const CALIB_ZERO_DISPARITY: i32 = 1024;
	/// For pinhole model only. Tangential distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28p%5F1%2C%20p%5F2%29) are set to zeros and stay zero.
	pub const CALIB_ZERO_TANGENT_DIST: i32 = 8;
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CameraModel {
		/// Pinhole camera model
		CALIB_MODEL_PINHOLE = 0,
		/// Fisheye camera model
		CALIB_MODEL_FISHEYE = 1,
	}

	opencv_type_enum! { crate::calib::CameraModel { CALIB_MODEL_PINHOLE, CALIB_MODEL_FISHEYE } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum HandEyeCalibrationMethod {
		/// A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/Eye Calibration [Tsai89](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Tsai89)
		CALIB_HAND_EYE_TSAI = 0,
		/// Robot Sensor Calibration: Solving AX = XB on the Euclidean Group [Park94](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Park94)
		CALIB_HAND_EYE_PARK = 1,
		/// Hand-eye Calibration [Horaud95](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Horaud95)
		CALIB_HAND_EYE_HORAUD = 2,
		/// On-line Hand-Eye Calibration [Andreff99](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Andreff99)
		CALIB_HAND_EYE_ANDREFF = 3,
		/// Hand-Eye Calibration Using Dual Quaternions [Daniilidis98](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Daniilidis98)
		CALIB_HAND_EYE_DANIILIDIS = 4,
	}

	opencv_type_enum! { crate::calib::HandEyeCalibrationMethod { CALIB_HAND_EYE_TSAI, CALIB_HAND_EYE_PARK, CALIB_HAND_EYE_HORAUD, CALIB_HAND_EYE_ANDREFF, CALIB_HAND_EYE_DANIILIDIS } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RobotWorldHandEyeCalibrationMethod {
		/// Solving the robot-world/hand-eye calibration problem using the kronecker product [Shah2013SolvingTR](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Shah2013SolvingTR)
		CALIB_ROBOT_WORLD_HAND_EYE_SHAH = 0,
		/// Simultaneous robot-world and hand-eye calibration using dual-quaternions and kronecker product [Li2010SimultaneousRA](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Li2010SimultaneousRA)
		CALIB_ROBOT_WORLD_HAND_EYE_LI = 1,
	}

	opencv_type_enum! { crate::calib::RobotWorldHandEyeCalibrationMethod { CALIB_ROBOT_WORLD_HAND_EYE_SHAH, CALIB_ROBOT_WORLD_HAND_EYE_LI } }

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration pattern.
	///
	/// This function is an extension of [calibrate_camera] with the method of releasing object which was
	/// proposed in [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv). In many common cases with inaccurate, unmeasured, roughly planar
	/// targets (calibration plates), this method can dramatically improve the precision of the estimated
	/// camera parameters. Both the object-releasing method and standard method are supported by this
	/// function. Use the parameter **iFixedPoint** for method selection. In the internal implementation,
	/// [calibrate_camera] is a wrapper for this function.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of calibration pattern points in the calibration pattern
	/// coordinate space. See [calibrate_camera] for details. If the method of releasing object to be used,
	/// the identical calibration board must be used in each view and it must be fully visible, and all
	/// objectPoints[i] must be the same and all points should be roughly close to a plane. **The calibration
	/// target has to be rigid, or at least static if the camera (rather than the calibration target) is
	/// shifted for grabbing images.**
	/// * imagePoints: Vector of vectors of the projections of calibration pattern points. See
	/// [calibrate_camera] for details.
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * iFixedPoint: The index of the 3D object point in objectPoints[0] to be fixed. It also acts as
	/// a switch for calibration method selection. If object-releasing method to be used, pass in the
	/// parameter in the range of [1, objectPoints[0].size()-2], otherwise a value out of this range will
	/// make standard calibration method selected. Usually the top-right corner point of the calibration
	/// board grid is recommended to be fixed when object-releasing method being utilized. According to
	/// \cite strobl2011iccv, two other points are also fixed. In this implementation, objectPoints[0].front
	/// and objectPoints[0].back.z are used. With object-releasing method, accurate rvecs, tvecs and
	/// newObjPoints are only possible if coordinates of these three fixed points are accurate enough.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix. See [calibrate_camera] for details.
	/// * distCoeffs: Output vector of distortion coefficients. See [calibrate_camera] for details.
	/// * rvecs: Output vector of rotation vectors estimated for each pattern view. See [calibrate_camera]
	/// for details.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * newObjPoints: The updated output vector of calibration pattern points. The coordinates might
	/// be scaled based on three fixed points. The returned coordinates are accurate only if the above
	/// mentioned three fixed points are accurate. If not needed, noArray() can be passed in. This parameter
	/// is ignored with standard calibration method.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsObjPoints: Output vector of standard deviations estimated for refined coordinates
	/// of calibration pattern points. It has the same size and order as objectPoints[0] vector. This
	/// parameter is ignored with standard calibration method.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of some predefined values. See
	/// [calibrate_camera] for details. If the method of releasing object is used, the calibration time may
	/// be much longer. CALIB_USE_QR or CALIB_USE_LU could be used for faster calibration with potentially
	/// less precise and less stable in some rare cases.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. The object-releasing extension follows [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv) and uses the same optimization
	/// core as #calibrateCamera. See [calibrate_camera] for other detailed explanations.
	/// ## See also
	/// calibrateCamera, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [calibrate_camera_ro] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_ro_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, i_fixed_point: i32, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, new_obj_points: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(new_obj_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, i_fixed_point, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), new_obj_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration pattern.
	///
	/// This function is an extension of [calibrate_camera] with the method of releasing object which was
	/// proposed in [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv). In many common cases with inaccurate, unmeasured, roughly planar
	/// targets (calibration plates), this method can dramatically improve the precision of the estimated
	/// camera parameters. Both the object-releasing method and standard method are supported by this
	/// function. Use the parameter **iFixedPoint** for method selection. In the internal implementation,
	/// [calibrate_camera] is a wrapper for this function.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of calibration pattern points in the calibration pattern
	/// coordinate space. See [calibrate_camera] for details. If the method of releasing object to be used,
	/// the identical calibration board must be used in each view and it must be fully visible, and all
	/// objectPoints[i] must be the same and all points should be roughly close to a plane. **The calibration
	/// target has to be rigid, or at least static if the camera (rather than the calibration target) is
	/// shifted for grabbing images.**
	/// * imagePoints: Vector of vectors of the projections of calibration pattern points. See
	/// [calibrate_camera] for details.
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * iFixedPoint: The index of the 3D object point in objectPoints[0] to be fixed. It also acts as
	/// a switch for calibration method selection. If object-releasing method to be used, pass in the
	/// parameter in the range of [1, objectPoints[0].size()-2], otherwise a value out of this range will
	/// make standard calibration method selected. Usually the top-right corner point of the calibration
	/// board grid is recommended to be fixed when object-releasing method being utilized. According to
	/// \cite strobl2011iccv, two other points are also fixed. In this implementation, objectPoints[0].front
	/// and objectPoints[0].back.z are used. With object-releasing method, accurate rvecs, tvecs and
	/// newObjPoints are only possible if coordinates of these three fixed points are accurate enough.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix. See [calibrate_camera] for details.
	/// * distCoeffs: Output vector of distortion coefficients. See [calibrate_camera] for details.
	/// * rvecs: Output vector of rotation vectors estimated for each pattern view. See [calibrate_camera]
	/// for details.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * newObjPoints: The updated output vector of calibration pattern points. The coordinates might
	/// be scaled based on three fixed points. The returned coordinates are accurate only if the above
	/// mentioned three fixed points are accurate. If not needed, noArray() can be passed in. This parameter
	/// is ignored with standard calibration method.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsObjPoints: Output vector of standard deviations estimated for refined coordinates
	/// of calibration pattern points. It has the same size and order as objectPoints[0] vector. This
	/// parameter is ignored with standard calibration method.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of some predefined values. See
	/// [calibrate_camera] for details. If the method of releasing object is used, the calibration time may
	/// be much longer. CALIB_USE_QR or CALIB_USE_LU could be used for faster calibration with potentially
	/// less precise and less stable in some rare cases.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. The object-releasing extension follows [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv) and uses the same optimization
	/// core as #calibrateCamera. See [calibrate_camera] for other detailed explanations.
	/// ## See also
	/// calibrateCamera, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
	///
	/// ## Note
	/// This alternative version of [calibrate_camera_ro_extended] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_ro_extended_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, i_fixed_point: i32, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, new_obj_points: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, std_deviations_obj_points: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(new_obj_points);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(std_deviations_obj_points);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, i_fixed_point, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), new_obj_points.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), std_deviations_obj_points.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration pattern.
	///
	/// This function is an extension of [calibrate_camera] with the method of releasing object which was
	/// proposed in [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv). In many common cases with inaccurate, unmeasured, roughly planar
	/// targets (calibration plates), this method can dramatically improve the precision of the estimated
	/// camera parameters. Both the object-releasing method and standard method are supported by this
	/// function. Use the parameter **iFixedPoint** for method selection. In the internal implementation,
	/// [calibrate_camera] is a wrapper for this function.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of calibration pattern points in the calibration pattern
	/// coordinate space. See [calibrate_camera] for details. If the method of releasing object to be used,
	/// the identical calibration board must be used in each view and it must be fully visible, and all
	/// objectPoints[i] must be the same and all points should be roughly close to a plane. **The calibration
	/// target has to be rigid, or at least static if the camera (rather than the calibration target) is
	/// shifted for grabbing images.**
	/// * imagePoints: Vector of vectors of the projections of calibration pattern points. See
	/// [calibrate_camera] for details.
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * iFixedPoint: The index of the 3D object point in objectPoints[0] to be fixed. It also acts as
	/// a switch for calibration method selection. If object-releasing method to be used, pass in the
	/// parameter in the range of [1, objectPoints[0].size()-2], otherwise a value out of this range will
	/// make standard calibration method selected. Usually the top-right corner point of the calibration
	/// board grid is recommended to be fixed when object-releasing method being utilized. According to
	/// \cite strobl2011iccv, two other points are also fixed. In this implementation, objectPoints[0].front
	/// and objectPoints[0].back.z are used. With object-releasing method, accurate rvecs, tvecs and
	/// newObjPoints are only possible if coordinates of these three fixed points are accurate enough.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix. See [calibrate_camera] for details.
	/// * distCoeffs: Output vector of distortion coefficients. See [calibrate_camera] for details.
	/// * rvecs: Output vector of rotation vectors estimated for each pattern view. See [calibrate_camera]
	/// for details.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * newObjPoints: The updated output vector of calibration pattern points. The coordinates might
	/// be scaled based on three fixed points. The returned coordinates are accurate only if the above
	/// mentioned three fixed points are accurate. If not needed, noArray() can be passed in. This parameter
	/// is ignored with standard calibration method.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsObjPoints: Output vector of standard deviations estimated for refined coordinates
	/// of calibration pattern points. It has the same size and order as objectPoints[0] vector. This
	/// parameter is ignored with standard calibration method.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of some predefined values. See
	/// [calibrate_camera] for details. If the method of releasing object is used, the calibration time may
	/// be much longer. CALIB_USE_QR or CALIB_USE_LU could be used for faster calibration with potentially
	/// less precise and less stable in some rare cases.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. The object-releasing extension follows [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv) and uses the same optimization
	/// core as #calibrateCamera. See [calibrate_camera] for other detailed explanations.
	/// ## See also
	/// calibrateCamera, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_ro_extended(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, i_fixed_point: i32, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, new_obj_points: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, std_deviations_obj_points: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(new_obj_points);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(std_deviations_obj_points);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, i_fixed_point, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), new_obj_points.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), std_deviations_obj_points.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration pattern.
	///
	/// This function is an extension of [calibrate_camera] with the method of releasing object which was
	/// proposed in [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv). In many common cases with inaccurate, unmeasured, roughly planar
	/// targets (calibration plates), this method can dramatically improve the precision of the estimated
	/// camera parameters. Both the object-releasing method and standard method are supported by this
	/// function. Use the parameter **iFixedPoint** for method selection. In the internal implementation,
	/// [calibrate_camera] is a wrapper for this function.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of calibration pattern points in the calibration pattern
	/// coordinate space. See [calibrate_camera] for details. If the method of releasing object to be used,
	/// the identical calibration board must be used in each view and it must be fully visible, and all
	/// objectPoints[i] must be the same and all points should be roughly close to a plane. **The calibration
	/// target has to be rigid, or at least static if the camera (rather than the calibration target) is
	/// shifted for grabbing images.**
	/// * imagePoints: Vector of vectors of the projections of calibration pattern points. See
	/// [calibrate_camera] for details.
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * iFixedPoint: The index of the 3D object point in objectPoints[0] to be fixed. It also acts as
	/// a switch for calibration method selection. If object-releasing method to be used, pass in the
	/// parameter in the range of [1, objectPoints[0].size()-2], otherwise a value out of this range will
	/// make standard calibration method selected. Usually the top-right corner point of the calibration
	/// board grid is recommended to be fixed when object-releasing method being utilized. According to
	/// \cite strobl2011iccv, two other points are also fixed. In this implementation, objectPoints[0].front
	/// and objectPoints[0].back.z are used. With object-releasing method, accurate rvecs, tvecs and
	/// newObjPoints are only possible if coordinates of these three fixed points are accurate enough.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix. See [calibrate_camera] for details.
	/// * distCoeffs: Output vector of distortion coefficients. See [calibrate_camera] for details.
	/// * rvecs: Output vector of rotation vectors estimated for each pattern view. See [calibrate_camera]
	/// for details.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * newObjPoints: The updated output vector of calibration pattern points. The coordinates might
	/// be scaled based on three fixed points. The returned coordinates are accurate only if the above
	/// mentioned three fixed points are accurate. If not needed, noArray() can be passed in. This parameter
	/// is ignored with standard calibration method.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// See [calibrate_camera] for details.
	/// * stdDeviationsObjPoints: Output vector of standard deviations estimated for refined coordinates
	/// of calibration pattern points. It has the same size and order as objectPoints[0] vector. This
	/// parameter is ignored with standard calibration method.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of some predefined values. See
	/// [calibrate_camera] for details. If the method of releasing object is used, the calibration time may
	/// be much longer. CALIB_USE_QR or CALIB_USE_LU could be used for faster calibration with potentially
	/// less precise and less stable in some rare cases.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. The object-releasing extension follows [strobl2011iccv](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_strobl2011iccv) and uses the same optimization
	/// core as #calibrateCamera. See [calibrate_camera] for other detailed explanations.
	/// ## See also
	/// calibrateCamera, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_ro(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, i_fixed_point: i32, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, new_obj_points: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(new_obj_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCameraRO_const__InputArrayR_const__InputArrayR_Size_int_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, i_fixed_point, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), new_obj_points.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration
	/// pattern.
	///
	/// ## Parameters
	/// * objectPoints: In the new interface it is a vector of vectors of calibration pattern points in
	/// the calibration pattern coordinate space (e.g. std::vector<std::vector<cv::Vec3f>>). The outer
	/// vector contains as many elements as the number of pattern views. If the same calibration pattern
	/// is shown in each view and it is fully visible, all the vectors will be the same. Although, it is
	/// possible to use partially occluded patterns or even different patterns in different views. Then,
	/// the vectors will be different. Although the points are 3D, they all lie in the calibration pattern's
	/// XY coordinate plane (thus 0 in the Z-coordinate), if the used calibration pattern is a planar rig.
	/// In the old interface all the vectors of object points from different views are concatenated
	/// together.
	/// * imagePoints: In the new interface it is a vector of vectors of the projections of calibration
	/// pattern points (e.g. std::vector<std::vector<cv::Vec2f>>). imagePoints.size() and
	/// objectPoints.size(), and imagePoints[i].size() and objectPoints[i].size() for each i, must be equal,
	/// respectively. In the old interface all the vectors of object points from different views are
	/// concatenated together.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrix.
	/// * cameraMatrix: Input/output 3x3 floating-point camera intrinsic matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If [CALIB_USE_INTRINSIC_GUESS]
	/// and/or [CALIB_FIX_ASPECT_RATIO], [CALIB_FIX_PRINCIPAL_POINT] or [CALIB_FIX_FOCAL_LENGTH]
	/// are specified, some or all of fx, fy, cx, cy must be initialized before calling the function.
	/// * distCoeffs: Input/output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs).
	/// * rvecs: Output vector of rotation vectors ([Rodrigues] ) estimated for each pattern view
	/// (e.g. std::vector<cv::Mat>>). That is, each i-th rotation vector together with the corresponding
	/// i-th translation vector (see the next output parameter description) brings the calibration pattern
	/// from the object coordinate space (in which object points are specified) to the camera coordinate
	/// space. In more technical terms, the tuple of the i-th rotation and translation vector performs
	/// a change of basis from object coordinate space to camera coordinate space. Due to its duality, this
	/// tuple is equivalent to the position of the calibration pattern with respect to the camera coordinate
	/// space.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter
	/// description above.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic
	/// parameters. Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0A%20s%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic
	/// parameters. Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F0%2C%20T%5F0%2C%20%5Cdotsc%20%2C%20R%5F%7BM%20%2D%201%7D%2C%20T%5F%7BM%20%2D%201%7D%29) where M is
	/// the number of pattern views. ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_INTRINSIC_GUESS] cameraMatrix contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
	/// Note, that if intrinsic parameters are known, there is no need to use this function just to
	/// estimate extrinsic parameters. Use [solvePnP] instead.
	/// *   [CALIB_DISABLE_SCHUR_COMPLEMENT] Disable Schur complement and use the Bouguet calibration engine ([Zhang2000](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Zhang2000), [BouguetMCT](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BouguetMCT)).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] The principal point is not changed during the global
	/// optimization. It stays at the center or at a different location specified when
	///  [CALIB_USE_INTRINSIC_GUESS] is set too.
	/// *   [CALIB_FIX_ASPECT_RATIO] The functions consider only fy as a free parameter. The
	/// ratio fx/fy stays the same as in the input cameraMatrix . When
	///  [CALIB_USE_INTRINSIC_GUESS] is not set, the actual input values of fx and fy are
	/// ignored, only their ratio is computed and used further.
	/// *   [CALIB_ZERO_TANGENT_DIST] Tangential distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28p%5F1%2C%20p%5F2%29) are set
	/// to zeros and stay zero.
	/// *   [CALIB_FIX_FOCAL_LENGTH] The focal length is not changed during the global optimization if
	///  [CALIB_USE_INTRINSIC_GUESS] is set.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] The corresponding radial distortion
	/// coefficient is not changed during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is
	/// set, the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Coefficients k4, k5, and k6 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the rational model and return 8 coefficients or more.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients or more.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. By default, the optimization follows a sparse bundle adjustment formulation with Schur
	/// complement; see [Triggs2000_bundle_adjustment](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Triggs2000_bundle_adjustment) and [Lourakis2009_sba](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lourakis2009_sba) for background. Use
	/// [CALIB_DISABLE_SCHUR_COMPLEMENT] to switch to the Bouguet calibration engine. The coordinates of 3D object
	/// points and their corresponding 2D projections in each view must be specified. That may be achieved
	/// by using an object with known geometry and easily detectable feature points. Such an object is
	/// called a calibration rig or calibration pattern, and OpenCV has built-in support for a chessboard as
	/// a calibration rig (see [findChessboardCorners]). Currently, initialization of intrinsic
	/// parameters (when [CALIB_USE_INTRINSIC_GUESS] is not set) is only implemented for planar calibration
	/// patterns (where Z-coordinates of the object points must be all zeros). 3D calibration rigs can also
	/// be used as long as initial cameraMatrix is provided.
	///
	/// The algorithm performs the following steps:
	///
	/// *   Compute the initial intrinsic parameters (the option only available for planar calibration
	///    patterns) or read them from the input parameters. The distortion coefficients are all set to
	///    zeros initially unless some of CALIB_FIX_K? are specified.
	///
	/// *   Estimate the initial camera pose as if the intrinsic parameters have been already known. This is
	///    done using [solvePnP] .
	///
	/// *   Run the global Levenberg-Marquardt optimization algorithm to minimize the reprojection error,
	///    that is, the total sum of squared distances between the observed feature points imagePoints and
	///    the projected (using the current estimates for camera parameters and the poses) object points
	///    objectPoints. See [projectPoints] for details.
	///
	/// *   In practice, robust acquisition is essential for stable results: use multiple board poses with
	///    significant tilt, avoid collecting all views at a single working distance, span the expected
	///    working-distance range (a larger board with larger squares can help for longer distances).
	///
	///
	/// Note:
	///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
	///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
	///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
	///
	///
	/// Note:
	///    The function may throw exceptions, if unsupported combination of parameters is provided or
	///    the system is underconstrained.
	/// ## See also
	/// calibrateCameraRO, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate,
	///    undistort
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [calibrate_camera] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration
	/// pattern.
	///
	/// ## Parameters
	/// * objectPoints: In the new interface it is a vector of vectors of calibration pattern points in
	/// the calibration pattern coordinate space (e.g. std::vector<std::vector<cv::Vec3f>>). The outer
	/// vector contains as many elements as the number of pattern views. If the same calibration pattern
	/// is shown in each view and it is fully visible, all the vectors will be the same. Although, it is
	/// possible to use partially occluded patterns or even different patterns in different views. Then,
	/// the vectors will be different. Although the points are 3D, they all lie in the calibration pattern's
	/// XY coordinate plane (thus 0 in the Z-coordinate), if the used calibration pattern is a planar rig.
	/// In the old interface all the vectors of object points from different views are concatenated
	/// together.
	/// * imagePoints: In the new interface it is a vector of vectors of the projections of calibration
	/// pattern points (e.g. std::vector<std::vector<cv::Vec2f>>). imagePoints.size() and
	/// objectPoints.size(), and imagePoints[i].size() and objectPoints[i].size() for each i, must be equal,
	/// respectively. In the old interface all the vectors of object points from different views are
	/// concatenated together.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrix.
	/// * cameraMatrix: Input/output 3x3 floating-point camera intrinsic matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If [CALIB_USE_INTRINSIC_GUESS]
	/// and/or [CALIB_FIX_ASPECT_RATIO], [CALIB_FIX_PRINCIPAL_POINT] or [CALIB_FIX_FOCAL_LENGTH]
	/// are specified, some or all of fx, fy, cx, cy must be initialized before calling the function.
	/// * distCoeffs: Input/output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs).
	/// * rvecs: Output vector of rotation vectors ([Rodrigues] ) estimated for each pattern view
	/// (e.g. std::vector<cv::Mat>>). That is, each i-th rotation vector together with the corresponding
	/// i-th translation vector (see the next output parameter description) brings the calibration pattern
	/// from the object coordinate space (in which object points are specified) to the camera coordinate
	/// space. In more technical terms, the tuple of the i-th rotation and translation vector performs
	/// a change of basis from object coordinate space to camera coordinate space. Due to its duality, this
	/// tuple is equivalent to the position of the calibration pattern with respect to the camera coordinate
	/// space.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter
	/// description above.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic
	/// parameters. Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0A%20s%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic
	/// parameters. Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F0%2C%20T%5F0%2C%20%5Cdotsc%20%2C%20R%5F%7BM%20%2D%201%7D%2C%20T%5F%7BM%20%2D%201%7D%29) where M is
	/// the number of pattern views. ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_INTRINSIC_GUESS] cameraMatrix contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
	/// Note, that if intrinsic parameters are known, there is no need to use this function just to
	/// estimate extrinsic parameters. Use [solvePnP] instead.
	/// *   [CALIB_DISABLE_SCHUR_COMPLEMENT] Disable Schur complement and use the Bouguet calibration engine ([Zhang2000](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Zhang2000), [BouguetMCT](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BouguetMCT)).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] The principal point is not changed during the global
	/// optimization. It stays at the center or at a different location specified when
	///  [CALIB_USE_INTRINSIC_GUESS] is set too.
	/// *   [CALIB_FIX_ASPECT_RATIO] The functions consider only fy as a free parameter. The
	/// ratio fx/fy stays the same as in the input cameraMatrix . When
	///  [CALIB_USE_INTRINSIC_GUESS] is not set, the actual input values of fx and fy are
	/// ignored, only their ratio is computed and used further.
	/// *   [CALIB_ZERO_TANGENT_DIST] Tangential distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28p%5F1%2C%20p%5F2%29) are set
	/// to zeros and stay zero.
	/// *   [CALIB_FIX_FOCAL_LENGTH] The focal length is not changed during the global optimization if
	///  [CALIB_USE_INTRINSIC_GUESS] is set.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] The corresponding radial distortion
	/// coefficient is not changed during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is
	/// set, the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Coefficients k4, k5, and k6 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the rational model and return 8 coefficients or more.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients or more.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. By default, the optimization follows a sparse bundle adjustment formulation with Schur
	/// complement; see [Triggs2000_bundle_adjustment](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Triggs2000_bundle_adjustment) and [Lourakis2009_sba](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lourakis2009_sba) for background. Use
	/// [CALIB_DISABLE_SCHUR_COMPLEMENT] to switch to the Bouguet calibration engine. The coordinates of 3D object
	/// points and their corresponding 2D projections in each view must be specified. That may be achieved
	/// by using an object with known geometry and easily detectable feature points. Such an object is
	/// called a calibration rig or calibration pattern, and OpenCV has built-in support for a chessboard as
	/// a calibration rig (see [findChessboardCorners]). Currently, initialization of intrinsic
	/// parameters (when [CALIB_USE_INTRINSIC_GUESS] is not set) is only implemented for planar calibration
	/// patterns (where Z-coordinates of the object points must be all zeros). 3D calibration rigs can also
	/// be used as long as initial cameraMatrix is provided.
	///
	/// The algorithm performs the following steps:
	///
	/// *   Compute the initial intrinsic parameters (the option only available for planar calibration
	///    patterns) or read them from the input parameters. The distortion coefficients are all set to
	///    zeros initially unless some of CALIB_FIX_K? are specified.
	///
	/// *   Estimate the initial camera pose as if the intrinsic parameters have been already known. This is
	///    done using [solvePnP] .
	///
	/// *   Run the global Levenberg-Marquardt optimization algorithm to minimize the reprojection error,
	///    that is, the total sum of squared distances between the observed feature points imagePoints and
	///    the projected (using the current estimates for camera parameters and the poses) object points
	///    objectPoints. See [projectPoints] for details.
	///
	/// *   In practice, robust acquisition is essential for stable results: use multiple board poses with
	///    significant tilt, avoid collecting all views at a single working distance, span the expected
	///    working-distance range (a larger board with larger squares can help for longer distances).
	///
	///
	/// Note:
	///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
	///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
	///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
	///
	///
	/// Note:
	///    The function may throw exceptions, if unsupported combination of parameters is provided or
	///    the system is underconstrained.
	/// ## See also
	/// calibrateCameraRO, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate,
	///    undistort
	///
	/// ## Note
	/// This alternative version of [calibrate_camera_extended] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_extended_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration
	/// pattern.
	///
	/// ## Parameters
	/// * objectPoints: In the new interface it is a vector of vectors of calibration pattern points in
	/// the calibration pattern coordinate space (e.g. std::vector<std::vector<cv::Vec3f>>). The outer
	/// vector contains as many elements as the number of pattern views. If the same calibration pattern
	/// is shown in each view and it is fully visible, all the vectors will be the same. Although, it is
	/// possible to use partially occluded patterns or even different patterns in different views. Then,
	/// the vectors will be different. Although the points are 3D, they all lie in the calibration pattern's
	/// XY coordinate plane (thus 0 in the Z-coordinate), if the used calibration pattern is a planar rig.
	/// In the old interface all the vectors of object points from different views are concatenated
	/// together.
	/// * imagePoints: In the new interface it is a vector of vectors of the projections of calibration
	/// pattern points (e.g. std::vector<std::vector<cv::Vec2f>>). imagePoints.size() and
	/// objectPoints.size(), and imagePoints[i].size() and objectPoints[i].size() for each i, must be equal,
	/// respectively. In the old interface all the vectors of object points from different views are
	/// concatenated together.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrix.
	/// * cameraMatrix: Input/output 3x3 floating-point camera intrinsic matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If [CALIB_USE_INTRINSIC_GUESS]
	/// and/or [CALIB_FIX_ASPECT_RATIO], [CALIB_FIX_PRINCIPAL_POINT] or [CALIB_FIX_FOCAL_LENGTH]
	/// are specified, some or all of fx, fy, cx, cy must be initialized before calling the function.
	/// * distCoeffs: Input/output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs).
	/// * rvecs: Output vector of rotation vectors ([Rodrigues] ) estimated for each pattern view
	/// (e.g. std::vector<cv::Mat>>). That is, each i-th rotation vector together with the corresponding
	/// i-th translation vector (see the next output parameter description) brings the calibration pattern
	/// from the object coordinate space (in which object points are specified) to the camera coordinate
	/// space. In more technical terms, the tuple of the i-th rotation and translation vector performs
	/// a change of basis from object coordinate space to camera coordinate space. Due to its duality, this
	/// tuple is equivalent to the position of the calibration pattern with respect to the camera coordinate
	/// space.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter
	/// description above.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic
	/// parameters. Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0A%20s%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic
	/// parameters. Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F0%2C%20T%5F0%2C%20%5Cdotsc%20%2C%20R%5F%7BM%20%2D%201%7D%2C%20T%5F%7BM%20%2D%201%7D%29) where M is
	/// the number of pattern views. ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_INTRINSIC_GUESS] cameraMatrix contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
	/// Note, that if intrinsic parameters are known, there is no need to use this function just to
	/// estimate extrinsic parameters. Use [solvePnP] instead.
	/// *   [CALIB_DISABLE_SCHUR_COMPLEMENT] Disable Schur complement and use the Bouguet calibration engine ([Zhang2000](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Zhang2000), [BouguetMCT](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BouguetMCT)).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] The principal point is not changed during the global
	/// optimization. It stays at the center or at a different location specified when
	///  [CALIB_USE_INTRINSIC_GUESS] is set too.
	/// *   [CALIB_FIX_ASPECT_RATIO] The functions consider only fy as a free parameter. The
	/// ratio fx/fy stays the same as in the input cameraMatrix . When
	///  [CALIB_USE_INTRINSIC_GUESS] is not set, the actual input values of fx and fy are
	/// ignored, only their ratio is computed and used further.
	/// *   [CALIB_ZERO_TANGENT_DIST] Tangential distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28p%5F1%2C%20p%5F2%29) are set
	/// to zeros and stay zero.
	/// *   [CALIB_FIX_FOCAL_LENGTH] The focal length is not changed during the global optimization if
	///  [CALIB_USE_INTRINSIC_GUESS] is set.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] The corresponding radial distortion
	/// coefficient is not changed during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is
	/// set, the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Coefficients k4, k5, and k6 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the rational model and return 8 coefficients or more.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients or more.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. By default, the optimization follows a sparse bundle adjustment formulation with Schur
	/// complement; see [Triggs2000_bundle_adjustment](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Triggs2000_bundle_adjustment) and [Lourakis2009_sba](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lourakis2009_sba) for background. Use
	/// [CALIB_DISABLE_SCHUR_COMPLEMENT] to switch to the Bouguet calibration engine. The coordinates of 3D object
	/// points and their corresponding 2D projections in each view must be specified. That may be achieved
	/// by using an object with known geometry and easily detectable feature points. Such an object is
	/// called a calibration rig or calibration pattern, and OpenCV has built-in support for a chessboard as
	/// a calibration rig (see [findChessboardCorners]). Currently, initialization of intrinsic
	/// parameters (when [CALIB_USE_INTRINSIC_GUESS] is not set) is only implemented for planar calibration
	/// patterns (where Z-coordinates of the object points must be all zeros). 3D calibration rigs can also
	/// be used as long as initial cameraMatrix is provided.
	///
	/// The algorithm performs the following steps:
	///
	/// *   Compute the initial intrinsic parameters (the option only available for planar calibration
	///    patterns) or read them from the input parameters. The distortion coefficients are all set to
	///    zeros initially unless some of CALIB_FIX_K? are specified.
	///
	/// *   Estimate the initial camera pose as if the intrinsic parameters have been already known. This is
	///    done using [solvePnP] .
	///
	/// *   Run the global Levenberg-Marquardt optimization algorithm to minimize the reprojection error,
	///    that is, the total sum of squared distances between the observed feature points imagePoints and
	///    the projected (using the current estimates for camera parameters and the poses) object points
	///    objectPoints. See [projectPoints] for details.
	///
	/// *   In practice, robust acquisition is essential for stable results: use multiple board poses with
	///    significant tilt, avoid collecting all views at a single working distance, span the expected
	///    working-distance range (a larger board with larger squares can help for longer distances).
	///
	///
	/// Note:
	///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
	///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
	///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
	///
	///
	/// Note:
	///    The function may throw exceptions, if unsupported combination of parameters is provided or
	///    the system is underconstrained.
	/// ## See also
	/// calibrateCameraRO, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate,
	///    undistort
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_extended(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration
	/// pattern.
	///
	/// ## Parameters
	/// * objectPoints: In the new interface it is a vector of vectors of calibration pattern points in
	/// the calibration pattern coordinate space (e.g. std::vector<std::vector<cv::Vec3f>>). The outer
	/// vector contains as many elements as the number of pattern views. If the same calibration pattern
	/// is shown in each view and it is fully visible, all the vectors will be the same. Although, it is
	/// possible to use partially occluded patterns or even different patterns in different views. Then,
	/// the vectors will be different. Although the points are 3D, they all lie in the calibration pattern's
	/// XY coordinate plane (thus 0 in the Z-coordinate), if the used calibration pattern is a planar rig.
	/// In the old interface all the vectors of object points from different views are concatenated
	/// together.
	/// * imagePoints: In the new interface it is a vector of vectors of the projections of calibration
	/// pattern points (e.g. std::vector<std::vector<cv::Vec2f>>). imagePoints.size() and
	/// objectPoints.size(), and imagePoints[i].size() and objectPoints[i].size() for each i, must be equal,
	/// respectively. In the old interface all the vectors of object points from different views are
	/// concatenated together.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrix.
	/// * cameraMatrix: Input/output 3x3 floating-point camera intrinsic matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If [CALIB_USE_INTRINSIC_GUESS]
	/// and/or [CALIB_FIX_ASPECT_RATIO], [CALIB_FIX_PRINCIPAL_POINT] or [CALIB_FIX_FOCAL_LENGTH]
	/// are specified, some or all of fx, fy, cx, cy must be initialized before calling the function.
	/// * distCoeffs: Input/output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs).
	/// * rvecs: Output vector of rotation vectors ([Rodrigues] ) estimated for each pattern view
	/// (e.g. std::vector<cv::Mat>>). That is, each i-th rotation vector together with the corresponding
	/// i-th translation vector (see the next output parameter description) brings the calibration pattern
	/// from the object coordinate space (in which object points are specified) to the camera coordinate
	/// space. In more technical terms, the tuple of the i-th rotation and translation vector performs
	/// a change of basis from object coordinate space to camera coordinate space. Due to its duality, this
	/// tuple is equivalent to the position of the calibration pattern with respect to the camera coordinate
	/// space.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter
	/// description above.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic
	/// parameters. Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0A%20s%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic
	/// parameters. Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F0%2C%20T%5F0%2C%20%5Cdotsc%20%2C%20R%5F%7BM%20%2D%201%7D%2C%20T%5F%7BM%20%2D%201%7D%29) where M is
	/// the number of pattern views. ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_INTRINSIC_GUESS] cameraMatrix contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
	/// Note, that if intrinsic parameters are known, there is no need to use this function just to
	/// estimate extrinsic parameters. Use [solvePnP] instead.
	/// *   [CALIB_DISABLE_SCHUR_COMPLEMENT] Disable Schur complement and use the Bouguet calibration engine ([Zhang2000](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Zhang2000), [BouguetMCT](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BouguetMCT)).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] The principal point is not changed during the global
	/// optimization. It stays at the center or at a different location specified when
	///  [CALIB_USE_INTRINSIC_GUESS] is set too.
	/// *   [CALIB_FIX_ASPECT_RATIO] The functions consider only fy as a free parameter. The
	/// ratio fx/fy stays the same as in the input cameraMatrix . When
	///  [CALIB_USE_INTRINSIC_GUESS] is not set, the actual input values of fx and fy are
	/// ignored, only their ratio is computed and used further.
	/// *   [CALIB_ZERO_TANGENT_DIST] Tangential distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28p%5F1%2C%20p%5F2%29) are set
	/// to zeros and stay zero.
	/// *   [CALIB_FIX_FOCAL_LENGTH] The focal length is not changed during the global optimization if
	///  [CALIB_USE_INTRINSIC_GUESS] is set.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] The corresponding radial distortion
	/// coefficient is not changed during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is
	/// set, the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Coefficients k4, k5, and k6 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the rational model and return 8 coefficients or more.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients or more.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Returns
	/// the overall RMS re-projection error.
	///
	/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
	/// views. By default, the optimization follows a sparse bundle adjustment formulation with Schur
	/// complement; see [Triggs2000_bundle_adjustment](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Triggs2000_bundle_adjustment) and [Lourakis2009_sba](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lourakis2009_sba) for background. Use
	/// [CALIB_DISABLE_SCHUR_COMPLEMENT] to switch to the Bouguet calibration engine. The coordinates of 3D object
	/// points and their corresponding 2D projections in each view must be specified. That may be achieved
	/// by using an object with known geometry and easily detectable feature points. Such an object is
	/// called a calibration rig or calibration pattern, and OpenCV has built-in support for a chessboard as
	/// a calibration rig (see [findChessboardCorners]). Currently, initialization of intrinsic
	/// parameters (when [CALIB_USE_INTRINSIC_GUESS] is not set) is only implemented for planar calibration
	/// patterns (where Z-coordinates of the object points must be all zeros). 3D calibration rigs can also
	/// be used as long as initial cameraMatrix is provided.
	///
	/// The algorithm performs the following steps:
	///
	/// *   Compute the initial intrinsic parameters (the option only available for planar calibration
	///    patterns) or read them from the input parameters. The distortion coefficients are all set to
	///    zeros initially unless some of CALIB_FIX_K? are specified.
	///
	/// *   Estimate the initial camera pose as if the intrinsic parameters have been already known. This is
	///    done using [solvePnP] .
	///
	/// *   Run the global Levenberg-Marquardt optimization algorithm to minimize the reprojection error,
	///    that is, the total sum of squared distances between the observed feature points imagePoints and
	///    the projected (using the current estimates for camera parameters and the poses) object points
	///    objectPoints. See [projectPoints] for details.
	///
	/// *   In practice, robust acquisition is essential for stable results: use multiple board poses with
	///    significant tilt, avoid collecting all views at a single working distance, span the expected
	///    working-distance range (a larger board with larger squares can help for longer distances).
	///
	///
	/// Note:
	///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
	///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
	///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
	///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
	///
	///
	/// Note:
	///    The function may throw exceptions, if unsupported combination of parameters is provided or
	///    the system is underconstrained.
	/// ## See also
	/// calibrateCameraRO, findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate,
	///    undistort
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,500,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateCamera_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes Hand-Eye calibration: ![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc)
	///
	/// ## Parameters
	/// * R_gripper2base: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the robot base frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fg)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from gripper frame to robot base frame.
	/// * t_gripper2base: Translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the robot base frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fg)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from gripper frame to robot base frame.
	/// * R_target2cam: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the target frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Ft)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from calibration target frame to camera frame.
	/// * t_target2cam: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the target frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Ft)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from calibration target frame to camera frame.
	/// * R_cam2gripper:[out] Estimated `(3x3)` rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the camera frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc)).
	/// * t_cam2gripper:[out] Estimated `(3x1)` translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the camera frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc)).
	/// * method: One of the implemented Hand-Eye calibration method, see cv::HandEyeCalibrationMethod
	///
	/// The function performs the Hand-Eye calibration using various methods. One approach consists in estimating the
	/// rotation then the translation (separable solutions) and the following methods are implemented:
	///   - R. Tsai, R. Lenz A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/EyeCalibration \cite Tsai89
	///   - F. Park, B. Martin Robot Sensor Calibration: Solving AX = XB on the Euclidean Group \cite Park94
	///   - R. Horaud, F. Dornaika Hand-Eye Calibration \cite Horaud95
	///
	/// Another approach consists in estimating simultaneously the rotation and the translation (simultaneous solutions),
	/// with the following implemented methods:
	///   - N. Andreff, R. Horaud, B. Espiau On-line Hand-Eye Calibration \cite Andreff99
	///   - K. Daniilidis Hand-Eye Calibration Using Dual Quaternions \cite Daniilidis98
	///
	/// The following picture describes the Hand-Eye calibration problem where the transformation between a camera ("eye")
	/// mounted on a robot gripper ("hand") has to be estimated. This configuration is called eye-in-hand.
	///
	/// The eye-to-hand configuration consists in a static camera observing a calibration pattern mounted on the robot
	/// end-effector. The transformation from the camera to the robot base frame can then be estimated by inputting
	/// the suitable transformations to the function, see below.
	///
	/// ![](https://docs.opencv.org/5.0.0/hand-eye_figure.png)
	///
	/// The calibration procedure is the following:
	///   - a static calibration pattern is used to estimate the transformation between the target frame
	///   and the camera frame
	///   - the robot gripper is moved in order to acquire several poses
	///   - for each pose, the homogeneous transformation between the gripper frame and the robot base frame is recorded using for
	///   instance the robot kinematics
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fb%5C%5C%0A%20%20%20%20Y%5Fb%5C%5C%0A%20%20%20%20Z%5Fb%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7BR%7D%5Fg%20%26%20%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7Bt%7D%5Fg%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///   - for each pose, the homogeneous transformation between the calibration target frame and the camera frame is recorded using
	///   for instance a pose estimation method (PnP) from 2D-3D point correspondences
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BR%7D%5Ft%20%26%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7Bt%7D%5Ft%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Ft%5C%5C%0A%20%20%20%20Y%5Ft%5C%5C%0A%20%20%20%20Z%5Ft%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// The Hand-Eye calibration procedure returns the following homogeneous transformation
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BR%7D%5Fc%20%26%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7Bt%7D%5Fc%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// This problem is also known as solving the ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BA%7D%5Cmathbf%7BX%7D%3D%5Cmathbf%7BX%7D%5Cmathbf%7BB%7D) equation:
	///   - for an eye-in-hand configuration
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Balign%2A%7D%0A%20%20%20%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%5C%5C%0A%0A%20%20%20%20%28%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%28%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%29%5E%7B%2D1%7D%20%5C%5C%0A%0A%20%20%20%20%5Ctextrm%7BA%7D%5Fi%20%5Ctextrm%7BX%7D%20%26%3D%20%5Ctextrm%7BX%7D%20%5Ctextrm%7BB%7D%5Fi%20%5C%5C%0A%20%20%20%20%5Cend%7Balign%2A%7D%0A)
	///
	///   - for an eye-to-hand configuration
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Balign%2A%7D%0A%20%20%20%20%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%282%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%5C%5C%0A%0A%20%20%20%20%28%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%282%29%7D%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%28%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%29%5E%7B%2D1%7D%20%5C%5C%0A%0A%20%20%20%20%5Ctextrm%7BA%7D%5Fi%20%5Ctextrm%7BX%7D%20%26%3D%20%5Ctextrm%7BX%7D%20%5Ctextrm%7BB%7D%5Fi%20%5C%5C%0A%20%20%20%20%5Cend%7Balign%2A%7D%0A)
	///
	/// \note
	/// Additional information can be found on this [website](http://campar.in.tum.de/Chair/HandEyeCalibration).
	/// \note
	/// A minimum of 2 motions with non parallel rotation axes are necessary to determine the hand-eye transformation.
	/// So at least 3 different poses are required, but it is strongly recommended to use many more poses.
	///
	/// ## Note
	/// This alternative version of [calibrate_hand_eye] function uses the following default values for its arguments:
	/// * method: CALIB_HAND_EYE_TSAI
	#[inline]
	pub fn calibrate_hand_eye_def(r_gripper2base: &impl ToInputArray, t_gripper2base: &impl ToInputArray, r_target2cam: &impl ToInputArray, t_target2cam: &impl ToInputArray, r_cam2gripper: &mut impl ToOutputArray, t_cam2gripper: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(r_gripper2base);
		input_array_arg!(t_gripper2base);
		input_array_arg!(r_target2cam);
		input_array_arg!(t_target2cam);
		output_array_arg!(r_cam2gripper);
		output_array_arg!(t_cam2gripper);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(r_gripper2base.as_raw__InputArray(), t_gripper2base.as_raw__InputArray(), r_target2cam.as_raw__InputArray(), t_target2cam.as_raw__InputArray(), r_cam2gripper.as_raw__OutputArray(), t_cam2gripper.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes Hand-Eye calibration: ![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc)
	///
	/// ## Parameters
	/// * R_gripper2base: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the robot base frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fg)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from gripper frame to robot base frame.
	/// * t_gripper2base: Translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the robot base frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fg)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from gripper frame to robot base frame.
	/// * R_target2cam: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the target frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Ft)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from calibration target frame to camera frame.
	/// * t_target2cam: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the target frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Ft)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from calibration target frame to camera frame.
	/// * R_cam2gripper:[out] Estimated `(3x3)` rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the camera frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc)).
	/// * t_cam2gripper:[out] Estimated `(3x1)` translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the camera frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc)).
	/// * method: One of the implemented Hand-Eye calibration method, see cv::HandEyeCalibrationMethod
	///
	/// The function performs the Hand-Eye calibration using various methods. One approach consists in estimating the
	/// rotation then the translation (separable solutions) and the following methods are implemented:
	///   - R. Tsai, R. Lenz A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/EyeCalibration \cite Tsai89
	///   - F. Park, B. Martin Robot Sensor Calibration: Solving AX = XB on the Euclidean Group \cite Park94
	///   - R. Horaud, F. Dornaika Hand-Eye Calibration \cite Horaud95
	///
	/// Another approach consists in estimating simultaneously the rotation and the translation (simultaneous solutions),
	/// with the following implemented methods:
	///   - N. Andreff, R. Horaud, B. Espiau On-line Hand-Eye Calibration \cite Andreff99
	///   - K. Daniilidis Hand-Eye Calibration Using Dual Quaternions \cite Daniilidis98
	///
	/// The following picture describes the Hand-Eye calibration problem where the transformation between a camera ("eye")
	/// mounted on a robot gripper ("hand") has to be estimated. This configuration is called eye-in-hand.
	///
	/// The eye-to-hand configuration consists in a static camera observing a calibration pattern mounted on the robot
	/// end-effector. The transformation from the camera to the robot base frame can then be estimated by inputting
	/// the suitable transformations to the function, see below.
	///
	/// ![](https://docs.opencv.org/5.0.0/hand-eye_figure.png)
	///
	/// The calibration procedure is the following:
	///   - a static calibration pattern is used to estimate the transformation between the target frame
	///   and the camera frame
	///   - the robot gripper is moved in order to acquire several poses
	///   - for each pose, the homogeneous transformation between the gripper frame and the robot base frame is recorded using for
	///   instance the robot kinematics
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fb%5C%5C%0A%20%20%20%20Y%5Fb%5C%5C%0A%20%20%20%20Z%5Fb%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7BR%7D%5Fg%20%26%20%5F%7B%7D%5E%7Bb%7D%5Ctextrm%7Bt%7D%5Fg%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///   - for each pose, the homogeneous transformation between the calibration target frame and the camera frame is recorded using
	///   for instance a pose estimation method (PnP) from 2D-3D point correspondences
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BR%7D%5Ft%20%26%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7Bt%7D%5Ft%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Ft%5C%5C%0A%20%20%20%20Y%5Ft%5C%5C%0A%20%20%20%20Z%5Ft%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// The Hand-Eye calibration procedure returns the following homogeneous transformation
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BR%7D%5Fc%20%26%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7Bt%7D%5Fc%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// This problem is also known as solving the ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BA%7D%5Cmathbf%7BX%7D%3D%5Cmathbf%7BX%7D%5Cmathbf%7BB%7D) equation:
	///   - for an eye-in-hand configuration
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Balign%2A%7D%0A%20%20%20%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%5C%5C%0A%0A%20%20%20%20%28%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%28%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%29%5E%7B%2D1%7D%20%5C%5C%0A%0A%20%20%20%20%5Ctextrm%7BA%7D%5Fi%20%5Ctextrm%7BX%7D%20%26%3D%20%5Ctextrm%7BX%7D%20%5Ctextrm%7BB%7D%5Fi%20%5C%5C%0A%20%20%20%20%5Cend%7Balign%2A%7D%0A)
	///
	///   - for an eye-to-hand configuration
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Balign%2A%7D%0A%20%20%20%20%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%282%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%5C%5C%0A%0A%20%20%20%20%28%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%282%29%7D%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%7B%5Ctextrm%7BT%7D%5Fb%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bb%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%28%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%29%5E%7B%2D1%7D%20%5C%5C%0A%0A%20%20%20%20%5Ctextrm%7BA%7D%5Fi%20%5Ctextrm%7BX%7D%20%26%3D%20%5Ctextrm%7BX%7D%20%5Ctextrm%7BB%7D%5Fi%20%5C%5C%0A%20%20%20%20%5Cend%7Balign%2A%7D%0A)
	///
	/// \note
	/// Additional information can be found on this [website](http://campar.in.tum.de/Chair/HandEyeCalibration).
	/// \note
	/// A minimum of 2 motions with non parallel rotation axes are necessary to determine the hand-eye transformation.
	/// So at least 3 different poses are required, but it is strongly recommended to use many more poses.
	///
	/// ## C++ default parameters
	/// * method: CALIB_HAND_EYE_TSAI
	#[inline]
	pub fn calibrate_hand_eye(r_gripper2base: &impl ToInputArray, t_gripper2base: &impl ToInputArray, r_target2cam: &impl ToInputArray, t_target2cam: &impl ToInputArray, r_cam2gripper: &mut impl ToOutputArray, t_cam2gripper: &mut impl ToOutputArray, method: crate::calib::HandEyeCalibrationMethod) -> Result<()> {
		input_array_arg!(r_gripper2base);
		input_array_arg!(t_gripper2base);
		input_array_arg!(r_target2cam);
		input_array_arg!(t_target2cam);
		output_array_arg!(r_cam2gripper);
		output_array_arg!(t_cam2gripper);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_HandEyeCalibrationMethod(r_gripper2base.as_raw__InputArray(), t_gripper2base.as_raw__InputArray(), r_target2cam.as_raw__InputArray(), t_target2cam.as_raw__InputArray(), r_cam2gripper.as_raw__OutputArray(), t_cam2gripper.as_raw__OutputArray(), method, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates intrinsics and extrinsics (camera pose) for multi-camera system a.k.a multiview calibration.
	///
	/// ## Parameters
	/// * objPoints: Calibration pattern object points. Expected shape: NUM_FRAMES x NUM_POINTS x 3. Supported data type: CV_32F.
	/// * imagePoints: Detected pattern points on camera images. Expected shape: NUM_CAMERAS x NUM_FRAMES x NUM_POINTS x 2.
	/// This function supports partial observation of the calibration pattern.
	/// To enable this, set the unobserved image points to be invalid points (eg. (-1., -1.)).
	/// * imageSize: Images resolution array for each camera.
	/// * detectionMask: Pattern detection mask. Each value defines if i-camera observes the calibration pattern in j-th frame.
	/// Expected size: NUM_CAMERAS x NUM_FRAMES. Expected type: CV_8U.
	/// * models: indicates camera models for each camera: cv::CALIB_MODEL_PINHOLE or cv::CALIB_MODEL_PINHOLE.
	/// Current implementation does not support mix of different camera models. Expected type: CV_8U.
	/// * flagsForIntrinsics: Flags used for each camera intrinsics calibration.
	/// Use per-camera call and the `useIntrinsicsGuess` flag to get custom intrinsics calibration for each camera.
	/// * flags: Common multiview calibration flags. cv::CALIB_USE_INTRINSIC_GUESS and cv::CALIB_USE_EXTRINSIC_GUESS are supported.
	/// See [CALIB_USE_INTRINSIC_GUESS] and other `CALIB_` constants. Expected shape: NUM_CAMERAS x 1. Supported data type: CV_32S.
	/// * Rs:[out] Rotation vectors relative to camera 0, where Rs[0] = 0. Output size: NUM_CAMERAS x 3 x 3.
	/// * Ts:[out] Estimated translation vectors relative to camera 0, where Ts[0] = 0. Output size: NUM_CAMERAS x 3 x 1.
	/// * rvecs0:[out] Estimated rotation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1 (may contain null Mat, if the frame is not valid). See [Rodrigues].
	/// * tvecs0:[out] Translation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1. (may contain null Mat, if the frame is not valid).
	/// * Ks:[out] Estimated floating-point camera intrinsic matrix. Output size: NUM_CAMERAS x 3 x 3.
	/// * distortions:[out] Distortion coefficients. Output size: NUM_CAMERAS x NUM_PARAMS.
	/// * perFrameErrors:[out] RMSE value for each visible frame, (-1 for non-visible). Output size: NUM_CAMERAS x NUM_FRAMES.
	/// * initializationPairs:[out] Pairs with camera indices that were used for initial pairwise stereo calibration.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// Output size: (NUM_CAMERAS-1) x 2.
	///
	/// [tutorial_multiview_camera_calibration] provides a detailed tutorial of using this function. Please refer to it for more information.
	///
	/// Multiview calibration usually requires several cameras to observe the same calibration pattern simultaneously.
	/// The fundamental assumption is that relative camera poses are fixed,
	/// and then for each frame, only the absolute camera pose for a single camera is needed to fix the camera pose for the multiple cameras
	///
	/// ![multiview calibration](https://docs.opencv.org/5.0.0/multiview_calib.png)
	/// The above illustration shows an example setting for multiview camera calibration.
	///
	/// For each frame, suppose the absolute camera pose for camera ![inline formula](https://latex.codecogs.com/png.latex?i) is ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20t%5Fi),
	/// and the relative camera pose between camera ![inline formula](https://latex.codecogs.com/png.latex?i) and camera ![inline formula](https://latex.codecogs.com/png.latex?j) is ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20t%5F%7Bij%7D).
	/// Suppose ![inline formula](https://latex.codecogs.com/png.latex?R%5F1%2C%20t%5F1), and ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7B1i%7D) for any ![inline formula](https://latex.codecogs.com/png.latex?i%5Cnot%3D1) are known, then its pose can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5Fi%20%3D%20R%5F%7B1i%7D%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5Fi%20%3D%20R%5F%7B1i%7D%20t%5F1%20%2B%20t%5F%7B1i%7D)
	///
	/// Since the relative pose between two cameras can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5F%7Bij%7D%20%3D%20R%5Fj%20R%5Fi%5E%5Ctop%20)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5F%7Bij%7D%20%3D%20%2DR%5F%7Bij%7D%20t%5Fi%20%2B%20R%5Fj%20)
	///
	/// This implies that any other relative pose of the form ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20i%5Cnot%3D1) is redundant.
	/// Given this, the total number of poses to determine is (NUM_CAMERAS-1) and NUM_FRAMES.
	/// This serves as the foundation of this function.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from all cameras.
	///
	/// ## Returns
	/// Overall RMS re-projection error over detectionMask.
	/// ## See also
	/// findChessboardCorners, findCirclesGrid, calibrateCamera, fisheye::calibrate, registerCameras
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [calibrate_multiview] function uses the following default values for its arguments:
	/// * flags_for_intrinsics: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn calibrate_multiview_def(obj_points: &impl ToInputArray, image_points: &core::Vector<core::Vector<core::Mat>>, image_size: &core::Vector<core::Size>, detection_mask: &impl ToInputArray, models: &impl ToInputArray, ks: &mut impl ToInputOutputArray, distortions: &mut impl ToInputOutputArray, rs: &mut impl ToInputOutputArray, ts: &mut impl ToInputOutputArray) -> Result<f64> {
		input_array_arg!(obj_points);
		input_array_arg!(detection_mask);
		input_array_arg!(models);
		input_output_array_arg!(ks);
		input_output_array_arg!(distortions);
		input_output_array_arg!(rs);
		input_output_array_arg!(ts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(obj_points.as_raw__InputArray(), image_points.as_raw_VectorOfVectorOfMat(), image_size.as_raw_VectorOfSize(), detection_mask.as_raw__InputArray(), models.as_raw__InputArray(), ks.as_raw__InputOutputArray(), distortions.as_raw__InputOutputArray(), rs.as_raw__InputOutputArray(), ts.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates intrinsics and extrinsics (camera pose) for multi-camera system a.k.a multiview calibration.
	///
	/// ## Parameters
	/// * objPoints: Calibration pattern object points. Expected shape: NUM_FRAMES x NUM_POINTS x 3. Supported data type: CV_32F.
	/// * imagePoints: Detected pattern points on camera images. Expected shape: NUM_CAMERAS x NUM_FRAMES x NUM_POINTS x 2.
	/// This function supports partial observation of the calibration pattern.
	/// To enable this, set the unobserved image points to be invalid points (eg. (-1., -1.)).
	/// * imageSize: Images resolution array for each camera.
	/// * detectionMask: Pattern detection mask. Each value defines if i-camera observes the calibration pattern in j-th frame.
	/// Expected size: NUM_CAMERAS x NUM_FRAMES. Expected type: CV_8U.
	/// * models: indicates camera models for each camera: cv::CALIB_MODEL_PINHOLE or cv::CALIB_MODEL_PINHOLE.
	/// Current implementation does not support mix of different camera models. Expected type: CV_8U.
	/// * flagsForIntrinsics: Flags used for each camera intrinsics calibration.
	/// Use per-camera call and the `useIntrinsicsGuess` flag to get custom intrinsics calibration for each camera.
	/// * flags: Common multiview calibration flags. cv::CALIB_USE_INTRINSIC_GUESS and cv::CALIB_USE_EXTRINSIC_GUESS are supported.
	/// See [CALIB_USE_INTRINSIC_GUESS] and other `CALIB_` constants. Expected shape: NUM_CAMERAS x 1. Supported data type: CV_32S.
	/// * Rs:[out] Rotation vectors relative to camera 0, where Rs[0] = 0. Output size: NUM_CAMERAS x 3 x 3.
	/// * Ts:[out] Estimated translation vectors relative to camera 0, where Ts[0] = 0. Output size: NUM_CAMERAS x 3 x 1.
	/// * rvecs0:[out] Estimated rotation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1 (may contain null Mat, if the frame is not valid). See [Rodrigues].
	/// * tvecs0:[out] Translation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1. (may contain null Mat, if the frame is not valid).
	/// * Ks:[out] Estimated floating-point camera intrinsic matrix. Output size: NUM_CAMERAS x 3 x 3.
	/// * distortions:[out] Distortion coefficients. Output size: NUM_CAMERAS x NUM_PARAMS.
	/// * perFrameErrors:[out] RMSE value for each visible frame, (-1 for non-visible). Output size: NUM_CAMERAS x NUM_FRAMES.
	/// * initializationPairs:[out] Pairs with camera indices that were used for initial pairwise stereo calibration.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// Output size: (NUM_CAMERAS-1) x 2.
	///
	/// [tutorial_multiview_camera_calibration] provides a detailed tutorial of using this function. Please refer to it for more information.
	///
	/// Multiview calibration usually requires several cameras to observe the same calibration pattern simultaneously.
	/// The fundamental assumption is that relative camera poses are fixed,
	/// and then for each frame, only the absolute camera pose for a single camera is needed to fix the camera pose for the multiple cameras
	///
	/// ![multiview calibration](https://docs.opencv.org/5.0.0/multiview_calib.png)
	/// The above illustration shows an example setting for multiview camera calibration.
	///
	/// For each frame, suppose the absolute camera pose for camera ![inline formula](https://latex.codecogs.com/png.latex?i) is ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20t%5Fi),
	/// and the relative camera pose between camera ![inline formula](https://latex.codecogs.com/png.latex?i) and camera ![inline formula](https://latex.codecogs.com/png.latex?j) is ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20t%5F%7Bij%7D).
	/// Suppose ![inline formula](https://latex.codecogs.com/png.latex?R%5F1%2C%20t%5F1), and ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7B1i%7D) for any ![inline formula](https://latex.codecogs.com/png.latex?i%5Cnot%3D1) are known, then its pose can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5Fi%20%3D%20R%5F%7B1i%7D%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5Fi%20%3D%20R%5F%7B1i%7D%20t%5F1%20%2B%20t%5F%7B1i%7D)
	///
	/// Since the relative pose between two cameras can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5F%7Bij%7D%20%3D%20R%5Fj%20R%5Fi%5E%5Ctop%20)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5F%7Bij%7D%20%3D%20%2DR%5F%7Bij%7D%20t%5Fi%20%2B%20R%5Fj%20)
	///
	/// This implies that any other relative pose of the form ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20i%5Cnot%3D1) is redundant.
	/// Given this, the total number of poses to determine is (NUM_CAMERAS-1) and NUM_FRAMES.
	/// This serves as the foundation of this function.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from all cameras.
	///
	/// ## Returns
	/// Overall RMS re-projection error over detectionMask.
	/// ## See also
	/// findChessboardCorners, findCirclesGrid, calibrateCamera, fisheye::calibrate, registerCameras
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags_for_intrinsics: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn calibrate_multiview(obj_points: &impl ToInputArray, image_points: &core::Vector<core::Vector<core::Mat>>, image_size: &core::Vector<core::Size>, detection_mask: &impl ToInputArray, models: &impl ToInputArray, ks: &mut impl ToInputOutputArray, distortions: &mut impl ToInputOutputArray, rs: &mut impl ToInputOutputArray, ts: &mut impl ToInputOutputArray, flags_for_intrinsics: &impl ToInputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(obj_points);
		input_array_arg!(detection_mask);
		input_array_arg!(models);
		input_output_array_arg!(ks);
		input_output_array_arg!(distortions);
		input_output_array_arg!(rs);
		input_output_array_arg!(ts);
		input_array_arg!(flags_for_intrinsics);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_int_TermCriteria(obj_points.as_raw__InputArray(), image_points.as_raw_VectorOfVectorOfMat(), image_size.as_raw_VectorOfSize(), detection_mask.as_raw__InputArray(), models.as_raw__InputArray(), ks.as_raw__InputOutputArray(), distortions.as_raw__InputOutputArray(), rs.as_raw__InputOutputArray(), ts.as_raw__InputOutputArray(), flags_for_intrinsics.as_raw__InputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates intrinsics and extrinsics (camera pose) for multi-camera system a.k.a multiview calibration.
	///
	/// ## Parameters
	/// * objPoints: Calibration pattern object points. Expected shape: NUM_FRAMES x NUM_POINTS x 3. Supported data type: CV_32F.
	/// * imagePoints: Detected pattern points on camera images. Expected shape: NUM_CAMERAS x NUM_FRAMES x NUM_POINTS x 2.
	/// This function supports partial observation of the calibration pattern.
	/// To enable this, set the unobserved image points to be invalid points (eg. (-1., -1.)).
	/// * imageSize: Images resolution array for each camera.
	/// * detectionMask: Pattern detection mask. Each value defines if i-camera observes the calibration pattern in j-th frame.
	/// Expected size: NUM_CAMERAS x NUM_FRAMES. Expected type: CV_8U.
	/// * models: indicates camera models for each camera: cv::CALIB_MODEL_PINHOLE or cv::CALIB_MODEL_PINHOLE.
	/// Current implementation does not support mix of different camera models. Expected type: CV_8U.
	/// * flagsForIntrinsics: Flags used for each camera intrinsics calibration.
	/// Use per-camera call and the `useIntrinsicsGuess` flag to get custom intrinsics calibration for each camera.
	/// * flags: Common multiview calibration flags. cv::CALIB_USE_INTRINSIC_GUESS and cv::CALIB_USE_EXTRINSIC_GUESS are supported.
	/// See [CALIB_USE_INTRINSIC_GUESS] and other `CALIB_` constants. Expected shape: NUM_CAMERAS x 1. Supported data type: CV_32S.
	/// * Rs:[out] Rotation vectors relative to camera 0, where Rs[0] = 0. Output size: NUM_CAMERAS x 3 x 3.
	/// * Ts:[out] Estimated translation vectors relative to camera 0, where Ts[0] = 0. Output size: NUM_CAMERAS x 3 x 1.
	/// * rvecs0:[out] Estimated rotation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1 (may contain null Mat, if the frame is not valid). See [Rodrigues].
	/// * tvecs0:[out] Translation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1. (may contain null Mat, if the frame is not valid).
	/// * Ks:[out] Estimated floating-point camera intrinsic matrix. Output size: NUM_CAMERAS x 3 x 3.
	/// * distortions:[out] Distortion coefficients. Output size: NUM_CAMERAS x NUM_PARAMS.
	/// * perFrameErrors:[out] RMSE value for each visible frame, (-1 for non-visible). Output size: NUM_CAMERAS x NUM_FRAMES.
	/// * initializationPairs:[out] Pairs with camera indices that were used for initial pairwise stereo calibration.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// Output size: (NUM_CAMERAS-1) x 2.
	///
	/// [tutorial_multiview_camera_calibration] provides a detailed tutorial of using this function. Please refer to it for more information.
	///
	/// Multiview calibration usually requires several cameras to observe the same calibration pattern simultaneously.
	/// The fundamental assumption is that relative camera poses are fixed,
	/// and then for each frame, only the absolute camera pose for a single camera is needed to fix the camera pose for the multiple cameras
	///
	/// ![multiview calibration](https://docs.opencv.org/5.0.0/multiview_calib.png)
	/// The above illustration shows an example setting for multiview camera calibration.
	///
	/// For each frame, suppose the absolute camera pose for camera ![inline formula](https://latex.codecogs.com/png.latex?i) is ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20t%5Fi),
	/// and the relative camera pose between camera ![inline formula](https://latex.codecogs.com/png.latex?i) and camera ![inline formula](https://latex.codecogs.com/png.latex?j) is ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20t%5F%7Bij%7D).
	/// Suppose ![inline formula](https://latex.codecogs.com/png.latex?R%5F1%2C%20t%5F1), and ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7B1i%7D) for any ![inline formula](https://latex.codecogs.com/png.latex?i%5Cnot%3D1) are known, then its pose can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5Fi%20%3D%20R%5F%7B1i%7D%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5Fi%20%3D%20R%5F%7B1i%7D%20t%5F1%20%2B%20t%5F%7B1i%7D)
	///
	/// Since the relative pose between two cameras can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5F%7Bij%7D%20%3D%20R%5Fj%20R%5Fi%5E%5Ctop%20)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5F%7Bij%7D%20%3D%20%2DR%5F%7Bij%7D%20t%5Fi%20%2B%20R%5Fj%20)
	///
	/// This implies that any other relative pose of the form ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20i%5Cnot%3D1) is redundant.
	/// Given this, the total number of poses to determine is (NUM_CAMERAS-1) and NUM_FRAMES.
	/// This serves as the foundation of this function.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from all cameras.
	///
	/// ## Returns
	/// Overall RMS re-projection error over detectionMask.
	/// ## See also
	/// findChessboardCorners, findCirclesGrid, calibrateCamera, fisheye::calibrate, registerCameras
	///
	/// ## Note
	/// This alternative version of [calibrate_multiview_extended] function uses the following default values for its arguments:
	/// * flags_for_intrinsics: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn calibrate_multiview_extended_def(obj_points: &impl ToInputArray, image_points: &core::Vector<core::Vector<core::Mat>>, image_size: &core::Vector<core::Size>, detection_mask: &impl ToInputArray, models: &impl ToInputArray, ks: &mut impl ToInputOutputArray, distortions: &mut impl ToInputOutputArray, rs: &mut impl ToInputOutputArray, ts: &mut impl ToInputOutputArray, initialization_pairs: &mut impl ToOutputArray, rvecs0: &mut impl ToOutputArray, tvecs0: &mut impl ToOutputArray, per_frame_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(obj_points);
		input_array_arg!(detection_mask);
		input_array_arg!(models);
		input_output_array_arg!(ks);
		input_output_array_arg!(distortions);
		input_output_array_arg!(rs);
		input_output_array_arg!(ts);
		output_array_arg!(initialization_pairs);
		output_array_arg!(rvecs0);
		output_array_arg!(tvecs0);
		output_array_arg!(per_frame_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(obj_points.as_raw__InputArray(), image_points.as_raw_VectorOfVectorOfMat(), image_size.as_raw_VectorOfSize(), detection_mask.as_raw__InputArray(), models.as_raw__InputArray(), ks.as_raw__InputOutputArray(), distortions.as_raw__InputOutputArray(), rs.as_raw__InputOutputArray(), ts.as_raw__InputOutputArray(), initialization_pairs.as_raw__OutputArray(), rvecs0.as_raw__OutputArray(), tvecs0.as_raw__OutputArray(), per_frame_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates intrinsics and extrinsics (camera pose) for multi-camera system a.k.a multiview calibration.
	///
	/// ## Parameters
	/// * objPoints: Calibration pattern object points. Expected shape: NUM_FRAMES x NUM_POINTS x 3. Supported data type: CV_32F.
	/// * imagePoints: Detected pattern points on camera images. Expected shape: NUM_CAMERAS x NUM_FRAMES x NUM_POINTS x 2.
	/// This function supports partial observation of the calibration pattern.
	/// To enable this, set the unobserved image points to be invalid points (eg. (-1., -1.)).
	/// * imageSize: Images resolution array for each camera.
	/// * detectionMask: Pattern detection mask. Each value defines if i-camera observes the calibration pattern in j-th frame.
	/// Expected size: NUM_CAMERAS x NUM_FRAMES. Expected type: CV_8U.
	/// * models: indicates camera models for each camera: cv::CALIB_MODEL_PINHOLE or cv::CALIB_MODEL_PINHOLE.
	/// Current implementation does not support mix of different camera models. Expected type: CV_8U.
	/// * flagsForIntrinsics: Flags used for each camera intrinsics calibration.
	/// Use per-camera call and the `useIntrinsicsGuess` flag to get custom intrinsics calibration for each camera.
	/// * flags: Common multiview calibration flags. cv::CALIB_USE_INTRINSIC_GUESS and cv::CALIB_USE_EXTRINSIC_GUESS are supported.
	/// See [CALIB_USE_INTRINSIC_GUESS] and other `CALIB_` constants. Expected shape: NUM_CAMERAS x 1. Supported data type: CV_32S.
	/// * Rs:[out] Rotation vectors relative to camera 0, where Rs[0] = 0. Output size: NUM_CAMERAS x 3 x 3.
	/// * Ts:[out] Estimated translation vectors relative to camera 0, where Ts[0] = 0. Output size: NUM_CAMERAS x 3 x 1.
	/// * rvecs0:[out] Estimated rotation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1 (may contain null Mat, if the frame is not valid). See [Rodrigues].
	/// * tvecs0:[out] Translation vectors for camera 0. Output size: NUM_FRAMES x 3 x 1. (may contain null Mat, if the frame is not valid).
	/// * Ks:[out] Estimated floating-point camera intrinsic matrix. Output size: NUM_CAMERAS x 3 x 3.
	/// * distortions:[out] Distortion coefficients. Output size: NUM_CAMERAS x NUM_PARAMS.
	/// * perFrameErrors:[out] RMSE value for each visible frame, (-1 for non-visible). Output size: NUM_CAMERAS x NUM_FRAMES.
	/// * initializationPairs:[out] Pairs with camera indices that were used for initial pairwise stereo calibration.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// Output size: (NUM_CAMERAS-1) x 2.
	///
	/// [tutorial_multiview_camera_calibration] provides a detailed tutorial of using this function. Please refer to it for more information.
	///
	/// Multiview calibration usually requires several cameras to observe the same calibration pattern simultaneously.
	/// The fundamental assumption is that relative camera poses are fixed,
	/// and then for each frame, only the absolute camera pose for a single camera is needed to fix the camera pose for the multiple cameras
	///
	/// ![multiview calibration](https://docs.opencv.org/5.0.0/multiview_calib.png)
	/// The above illustration shows an example setting for multiview camera calibration.
	///
	/// For each frame, suppose the absolute camera pose for camera ![inline formula](https://latex.codecogs.com/png.latex?i) is ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20t%5Fi),
	/// and the relative camera pose between camera ![inline formula](https://latex.codecogs.com/png.latex?i) and camera ![inline formula](https://latex.codecogs.com/png.latex?j) is ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20t%5F%7Bij%7D).
	/// Suppose ![inline formula](https://latex.codecogs.com/png.latex?R%5F1%2C%20t%5F1), and ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7B1i%7D) for any ![inline formula](https://latex.codecogs.com/png.latex?i%5Cnot%3D1) are known, then its pose can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5Fi%20%3D%20R%5F%7B1i%7D%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5Fi%20%3D%20R%5F%7B1i%7D%20t%5F1%20%2B%20t%5F%7B1i%7D)
	///
	/// Since the relative pose between two cameras can be calculated by
	/// ![block formula](https://latex.codecogs.com/png.latex?%20R%5F%7Bij%7D%20%3D%20R%5Fj%20R%5Fi%5E%5Ctop%20)
	/// ![block formula](https://latex.codecogs.com/png.latex?%20t%5F%7Bij%7D%20%3D%20%2DR%5F%7Bij%7D%20t%5Fi%20%2B%20R%5Fj%20)
	///
	/// This implies that any other relative pose of the form ![inline formula](https://latex.codecogs.com/png.latex?R%5F%7Bij%7D%2C%20i%5Cnot%3D1) is redundant.
	/// Given this, the total number of poses to determine is (NUM_CAMERAS-1) and NUM_FRAMES.
	/// This serves as the foundation of this function.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from all cameras.
	///
	/// ## Returns
	/// Overall RMS re-projection error over detectionMask.
	/// ## See also
	/// findChessboardCorners, findCirclesGrid, calibrateCamera, fisheye::calibrate, registerCameras
	///
	/// ## C++ default parameters
	/// * flags_for_intrinsics: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn calibrate_multiview_extended(obj_points: &impl ToInputArray, image_points: &core::Vector<core::Vector<core::Mat>>, image_size: &core::Vector<core::Size>, detection_mask: &impl ToInputArray, models: &impl ToInputArray, ks: &mut impl ToInputOutputArray, distortions: &mut impl ToInputOutputArray, rs: &mut impl ToInputOutputArray, ts: &mut impl ToInputOutputArray, initialization_pairs: &mut impl ToOutputArray, rvecs0: &mut impl ToOutputArray, tvecs0: &mut impl ToOutputArray, per_frame_errors: &mut impl ToOutputArray, flags_for_intrinsics: &impl ToInputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(obj_points);
		input_array_arg!(detection_mask);
		input_array_arg!(models);
		input_output_array_arg!(ks);
		input_output_array_arg!(distortions);
		input_output_array_arg!(rs);
		input_output_array_arg!(ts);
		output_array_arg!(initialization_pairs);
		output_array_arg!(rvecs0);
		output_array_arg!(tvecs0);
		output_array_arg!(per_frame_errors);
		input_array_arg!(flags_for_intrinsics);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateMultiview_const__InputArrayR_const_vectorLvectorLMatGGR_const_vectorLSizeGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_int_TermCriteria(obj_points.as_raw__InputArray(), image_points.as_raw_VectorOfVectorOfMat(), image_size.as_raw_VectorOfSize(), detection_mask.as_raw__InputArray(), models.as_raw__InputArray(), ks.as_raw__InputOutputArray(), distortions.as_raw__InputOutputArray(), rs.as_raw__InputOutputArray(), ts.as_raw__InputOutputArray(), initialization_pairs.as_raw__OutputArray(), rvecs0.as_raw__OutputArray(), tvecs0.as_raw__OutputArray(), per_frame_errors.as_raw__OutputArray(), flags_for_intrinsics.as_raw__InputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes Robot-World/Hand-Eye calibration: ![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb) and ![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)
	///
	/// ## Parameters
	/// * R_world2cam: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the world frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fw)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from world frame to the camera frame.
	/// * t_world2cam: Translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the world frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fw)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from world frame to the camera frame.
	/// * R_base2gripper: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fb)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from robot base frame to the gripper frame.
	/// * t_base2gripper: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fb)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from robot base frame to the gripper frame.
	/// * R_base2world:[out] Estimated `(3x3)` rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the world frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb)).
	/// * t_base2world:[out] Estimated `(3x1)` translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the world frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb)).
	/// * R_gripper2cam:[out] Estimated `(3x3)` rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)).
	/// * t_gripper2cam:[out] Estimated `(3x1)` translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)).
	/// * method: One of the implemented Robot-World/Hand-Eye calibration method, see cv::RobotWorldHandEyeCalibrationMethod
	///
	/// The function performs the Robot-World/Hand-Eye calibration using various methods. One approach consists in estimating the
	/// rotation then the translation (separable solutions):
	///   - M. Shah, Solving the robot-world/hand-eye calibration problem using the kronecker product \cite Shah2013SolvingTR
	///
	/// Another approach consists in estimating simultaneously the rotation and the translation (simultaneous solutions),
	/// with the following implemented method:
	///   - A. Li, L. Wang, and D. Wu, Simultaneous robot-world and hand-eye calibration using dual-quaternions and kronecker product \cite Li2010SimultaneousRA
	///
	/// The following picture describes the Robot-World/Hand-Eye calibration problem where the transformations between a robot and a world frame
	/// and between a robot gripper ("hand") and a camera ("eye") mounted at the robot end-effector have to be estimated.
	///
	/// ![](https://docs.opencv.org/5.0.0/robot-world_hand-eye_figure.png)
	///
	/// The calibration procedure is the following:
	///   - a static calibration pattern is used to estimate the transformation between the target frame
	///   and the camera frame
	///   - the robot gripper is moved in order to acquire several poses
	///   - for each pose, the homogeneous transformation between the gripper frame and the robot base frame is recorded using for
	///   instance the robot kinematics
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BR%7D%5Fb%20%26%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7Bt%7D%5Fb%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fb%5C%5C%0A%20%20%20%20Y%5Fb%5C%5C%0A%20%20%20%20Z%5Fb%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///   - for each pose, the homogeneous transformation between the calibration target frame (the world frame) and the camera frame is recorded using
	///   for instance a pose estimation method (PnP) from 2D-3D point correspondences
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BR%7D%5Fw%20%26%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7Bt%7D%5Fw%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fw%5C%5C%0A%20%20%20%20Y%5Fw%5C%5C%0A%20%20%20%20Z%5Fw%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// The Robot-World/Hand-Eye calibration procedure returns the following homogeneous transformations
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fw%5C%5C%0A%20%20%20%20Y%5Fw%5C%5C%0A%20%20%20%20Z%5Fw%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BR%7D%5Fb%20%26%20%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7Bt%7D%5Fb%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fb%5C%5C%0A%20%20%20%20Y%5Fb%5C%5C%0A%20%20%20%20Z%5Fb%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BR%7D%5Fg%20%26%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7Bt%7D%5Fg%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// This problem is also known as solving the ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BA%7D%5Cmathbf%7BX%7D%3D%5Cmathbf%7BZ%7D%5Cmathbf%7BB%7D) equation, with:
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BA%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fw)
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BX%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb)
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BZ%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BB%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fb)
	///
	/// \note
	/// At least 3 measurements are required (input vectors size must be greater or equal to 3).
	///
	/// ## Note
	/// This alternative version of [calibrate_robot_world_hand_eye] function uses the following default values for its arguments:
	/// * method: CALIB_ROBOT_WORLD_HAND_EYE_SHAH
	#[inline]
	pub fn calibrate_robot_world_hand_eye_def(r_world2cam: &impl ToInputArray, t_world2cam: &impl ToInputArray, r_base2gripper: &impl ToInputArray, t_base2gripper: &impl ToInputArray, r_base2world: &mut impl ToOutputArray, t_base2world: &mut impl ToOutputArray, r_gripper2cam: &mut impl ToOutputArray, t_gripper2cam: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(r_world2cam);
		input_array_arg!(t_world2cam);
		input_array_arg!(r_base2gripper);
		input_array_arg!(t_base2gripper);
		output_array_arg!(r_base2world);
		output_array_arg!(t_base2world);
		output_array_arg!(r_gripper2cam);
		output_array_arg!(t_gripper2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateRobotWorldHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(r_world2cam.as_raw__InputArray(), t_world2cam.as_raw__InputArray(), r_base2gripper.as_raw__InputArray(), t_base2gripper.as_raw__InputArray(), r_base2world.as_raw__OutputArray(), t_base2world.as_raw__OutputArray(), r_gripper2cam.as_raw__OutputArray(), t_gripper2cam.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes Robot-World/Hand-Eye calibration: ![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb) and ![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)
	///
	/// ## Parameters
	/// * R_world2cam: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the world frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fw)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from world frame to the camera frame.
	/// * t_world2cam: Translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the world frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fw)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from world frame to the camera frame.
	/// * R_base2gripper: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fb)).
	/// This is a vector (`vector<Mat>`) that contains the rotation, `(3x3)` rotation matrices or `(3x1)` rotation vectors,
	/// for all the transformations from robot base frame to the gripper frame.
	/// * t_base2gripper: Rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the gripper frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fb)).
	/// This is a vector (`vector<Mat>`) that contains the `(3x1)` translation vectors for all the transformations
	/// from robot base frame to the gripper frame.
	/// * R_base2world:[out] Estimated `(3x3)` rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the world frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb)).
	/// * t_base2world:[out] Estimated `(3x1)` translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the robot base frame to the world frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb)).
	/// * R_gripper2cam:[out] Estimated `(3x3)` rotation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)).
	/// * t_gripper2cam:[out] Estimated `(3x1)` translation part extracted from the homogeneous matrix that transforms a point
	/// expressed in the gripper frame to the camera frame (![inline formula](https://latex.codecogs.com/png.latex?%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)).
	/// * method: One of the implemented Robot-World/Hand-Eye calibration method, see cv::RobotWorldHandEyeCalibrationMethod
	///
	/// The function performs the Robot-World/Hand-Eye calibration using various methods. One approach consists in estimating the
	/// rotation then the translation (separable solutions):
	///   - M. Shah, Solving the robot-world/hand-eye calibration problem using the kronecker product \cite Shah2013SolvingTR
	///
	/// Another approach consists in estimating simultaneously the rotation and the translation (simultaneous solutions),
	/// with the following implemented method:
	///   - A. Li, L. Wang, and D. Wu, Simultaneous robot-world and hand-eye calibration using dual-quaternions and kronecker product \cite Li2010SimultaneousRA
	///
	/// The following picture describes the Robot-World/Hand-Eye calibration problem where the transformations between a robot and a world frame
	/// and between a robot gripper ("hand") and a camera ("eye") mounted at the robot end-effector have to be estimated.
	///
	/// ![](https://docs.opencv.org/5.0.0/robot-world_hand-eye_figure.png)
	///
	/// The calibration procedure is the following:
	///   - a static calibration pattern is used to estimate the transformation between the target frame
	///   and the camera frame
	///   - the robot gripper is moved in order to acquire several poses
	///   - for each pose, the homogeneous transformation between the gripper frame and the robot base frame is recorded using for
	///   instance the robot kinematics
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BR%7D%5Fb%20%26%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7Bt%7D%5Fb%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fb%5C%5C%0A%20%20%20%20Y%5Fb%5C%5C%0A%20%20%20%20Z%5Fb%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///   - for each pose, the homogeneous transformation between the calibration target frame (the world frame) and the camera frame is recorded using
	///   for instance a pose estimation method (PnP) from 2D-3D point correspondences
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BR%7D%5Fw%20%26%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7Bt%7D%5Fw%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fw%5C%5C%0A%20%20%20%20Y%5Fw%5C%5C%0A%20%20%20%20Z%5Fw%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// The Robot-World/Hand-Eye calibration procedure returns the following homogeneous transformations
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fw%5C%5C%0A%20%20%20%20Y%5Fw%5C%5C%0A%20%20%20%20Z%5Fw%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BR%7D%5Fb%20%26%20%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7Bt%7D%5Fb%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fb%5C%5C%0A%20%20%20%20Y%5Fb%5C%5C%0A%20%20%20%20Z%5Fb%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fc%5C%5C%0A%20%20%20%20Y%5Fc%5C%5C%0A%20%20%20%20Z%5Fc%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%3D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BR%7D%5Fg%20%26%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7Bt%7D%5Fg%20%5C%5C%0A%20%20%20%200%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20X%5Fg%5C%5C%0A%20%20%20%20Y%5Fg%5C%5C%0A%20%20%20%20Z%5Fg%5C%5C%0A%20%20%20%201%0A%20%20%20%20%5Cend%7Bbmatrix%7D%0A)
	///
	/// This problem is also known as solving the ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BA%7D%5Cmathbf%7BX%7D%3D%5Cmathbf%7BZ%7D%5Cmathbf%7BB%7D) equation, with:
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BA%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fw)
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BX%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bw%7D%5Ctextrm%7BT%7D%5Fb)
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BZ%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bc%7D%5Ctextrm%7BT%7D%5Fg)
	///   - ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathbf%7BB%7D%20%5CLeftrightarrow%20%5Chspace%7B0%2E1em%7D%20%5F%7B%7D%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fb)
	///
	/// \note
	/// At least 3 measurements are required (input vectors size must be greater or equal to 3).
	///
	/// ## C++ default parameters
	/// * method: CALIB_ROBOT_WORLD_HAND_EYE_SHAH
	#[inline]
	pub fn calibrate_robot_world_hand_eye(r_world2cam: &impl ToInputArray, t_world2cam: &impl ToInputArray, r_base2gripper: &impl ToInputArray, t_base2gripper: &impl ToInputArray, r_base2world: &mut impl ToOutputArray, t_base2world: &mut impl ToOutputArray, r_gripper2cam: &mut impl ToOutputArray, t_gripper2cam: &mut impl ToOutputArray, method: crate::calib::RobotWorldHandEyeCalibrationMethod) -> Result<()> {
		input_array_arg!(r_world2cam);
		input_array_arg!(t_world2cam);
		input_array_arg!(r_base2gripper);
		input_array_arg!(t_base2gripper);
		output_array_arg!(r_base2world);
		output_array_arg!(t_base2world);
		output_array_arg!(r_gripper2cam);
		output_array_arg!(t_gripper2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_calibrateRobotWorldHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_RobotWorldHandEyeCalibrationMethod(r_world2cam.as_raw__InputArray(), t_world2cam.as_raw__InputArray(), r_base2gripper.as_raw__InputArray(), t_base2gripper.as_raw__InputArray(), r_base2world.as_raw__OutputArray(), t_base2world.as_raw__OutputArray(), r_gripper2cam.as_raw__OutputArray(), t_gripper2cam.as_raw__OutputArray(), method, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs camera calibration
	///
	/// ## Parameters
	/// * objectPoints: vector of vectors of calibration pattern points in the calibration pattern
	/// coordinate space.
	/// * imagePoints: vector of vectors of the projections of calibration pattern points.
	/// imagePoints.size() and objectPoints.size() and imagePoints[i].size() must be equal to
	/// objectPoints[i].size() for each i.
	/// * image_size: Size of the image used only to initialize the camera intrinsic matrix.
	/// * K: Output 3x3 floating-point camera intrinsic matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If
	/// [cv::CALIB_USE_INTRINSIC_GUESS] is specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * D: Output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view.
	/// That is, each k-th rotation vector together with the corresponding k-th translation vector (see
	/// the next output parameter description) brings the calibration pattern from the model coordinate
	/// space (in which object points are specified) to the world coordinate space, that is, a real
	/// position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [cv::CALIB_USE_INTRINSIC_GUESS]  cameraMatrix contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
	/// *   [cv::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
	/// of intrinsic optimization.
	/// *   [cv::CALIB_CHECK_COND]  The functions will check validity of condition number.
	/// *   [cv::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
	/// *   [cv::CALIB_FIX_K1],..., [cv::CALIB_FIX_K4] Selected distortion coefficients
	/// are set to zeros and stay zero.
	/// *   [cv::CALIB_FIX_PRINCIPAL_POINT]  The principal point is not changed during the global
	/// optimization. It stays at the center or at a different location specified when [cv::CALIB_USE_INTRINSIC_GUESS] is set too.
	/// *   [cv::CALIB_FIX_FOCAL_LENGTH] The focal length is not changed during the global
	/// optimization. It is the ![inline formula](https://latex.codecogs.com/png.latex?max%28width%2Cheight%29%2F%5Cpi) or the provided ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx), ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) when [cv::CALIB_USE_INTRINSIC_GUESS] is set too.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Note
	/// This alternative version of [calibrate] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn calibrate_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, k: &mut impl ToInputOutputArray, d: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(k);
		input_output_array_arg!(d);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, k.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs camera calibration
	///
	/// ## Parameters
	/// * objectPoints: vector of vectors of calibration pattern points in the calibration pattern
	/// coordinate space.
	/// * imagePoints: vector of vectors of the projections of calibration pattern points.
	/// imagePoints.size() and objectPoints.size() and imagePoints[i].size() must be equal to
	/// objectPoints[i].size() for each i.
	/// * image_size: Size of the image used only to initialize the camera intrinsic matrix.
	/// * K: Output 3x3 floating-point camera intrinsic matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If
	/// [cv::CALIB_USE_INTRINSIC_GUESS] is specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * D: Output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view.
	/// That is, each k-th rotation vector together with the corresponding k-th translation vector (see
	/// the next output parameter description) brings the calibration pattern from the model coordinate
	/// space (in which object points are specified) to the world coordinate space, that is, a real
	/// position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [cv::CALIB_USE_INTRINSIC_GUESS]  cameraMatrix contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
	/// *   [cv::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
	/// of intrinsic optimization.
	/// *   [cv::CALIB_CHECK_COND]  The functions will check validity of condition number.
	/// *   [cv::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
	/// *   [cv::CALIB_FIX_K1],..., [cv::CALIB_FIX_K4] Selected distortion coefficients
	/// are set to zeros and stay zero.
	/// *   [cv::CALIB_FIX_PRINCIPAL_POINT]  The principal point is not changed during the global
	/// optimization. It stays at the center or at a different location specified when [cv::CALIB_USE_INTRINSIC_GUESS] is set too.
	/// *   [cv::CALIB_FIX_FOCAL_LENGTH] The focal length is not changed during the global
	/// optimization. It is the ![inline formula](https://latex.codecogs.com/png.latex?max%28width%2Cheight%29%2F%5Cpi) or the provided ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx), ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) when [cv::CALIB_USE_INTRINSIC_GUESS] is set too.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn calibrate(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, k: &mut impl ToInputOutputArray, d: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(k);
		input_output_array_arg!(d);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_calibrate_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, k.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs stereo calibration
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera.
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera.
	/// * K1: Input/output first camera intrinsic matrix:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cvecthreethree%7Bf%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bc%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bf%5Fy%5E%7B%28j%29%7D%7D%7Bc%5Fy%5E%7B%28j%29%7D%7D%7B0%7D%7B0%7D%7B1%7D) , ![inline formula](https://latex.codecogs.com/png.latex?j%20%3D%200%2C%5C%2C%201) . If
	/// any of [cv::CALIB_USE_INTRINSIC_GUESS] , [cv::CALIB_FIX_INTRINSIC] are specified,
	/// some or all of the matrix components must be initialized.
	/// * D1: Input/output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) of 4 elements.
	/// * K2: Input/output second camera intrinsic matrix. The parameter is similar to K1 .
	/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
	/// similar to D1 .
	/// * imageSize: Size of the image used only to initialize camera intrinsic matrix.
	/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
	/// * T: Output translation vector between the coordinate systems of the cameras.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [cv::CALIB_FIX_INTRINSIC]  Fix K1, K2? and D1, D2? so that only R, T matrices
	/// are estimated.
	/// *   [cv::CALIB_USE_INTRINSIC_GUESS]  K1, K2 contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
	/// *   [cv::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
	/// of intrinsic optimization.
	/// *   [cv::CALIB_CHECK_COND]  The functions will check validity of condition number.
	/// *   [cv::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
	/// *   [cv::CALIB_FIX_K1],..., [cv::CALIB_FIX_K4] Selected distortion coefficients are set to zeros and stay
	/// zero.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [fisheye_stereo_calibrate] function uses the following default values for its arguments:
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn fisheye_stereo_calibrate_def(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, k1: &mut impl ToInputOutputArray, d1: &mut impl ToInputOutputArray, k2: &mut impl ToInputOutputArray, d2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(k1);
		input_output_array_arg!(d1);
		input_output_array_arg!(k2);
		input_output_array_arg!(d2);
		output_array_arg!(r);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), k1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), &image_size, r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs stereo calibration
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera.
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera.
	/// * K1: Input/output first camera intrinsic matrix:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cvecthreethree%7Bf%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bc%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bf%5Fy%5E%7B%28j%29%7D%7D%7Bc%5Fy%5E%7B%28j%29%7D%7D%7B0%7D%7B0%7D%7B1%7D) , ![inline formula](https://latex.codecogs.com/png.latex?j%20%3D%200%2C%5C%2C%201) . If
	/// any of [cv::CALIB_USE_INTRINSIC_GUESS] , [cv::CALIB_FIX_INTRINSIC] are specified,
	/// some or all of the matrix components must be initialized.
	/// * D1: Input/output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) of 4 elements.
	/// * K2: Input/output second camera intrinsic matrix. The parameter is similar to K1 .
	/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
	/// similar to D1 .
	/// * imageSize: Size of the image used only to initialize camera intrinsic matrix.
	/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
	/// * T: Output translation vector between the coordinate systems of the cameras.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [cv::CALIB_FIX_INTRINSIC]  Fix K1, K2? and D1, D2? so that only R, T matrices
	/// are estimated.
	/// *   [cv::CALIB_USE_INTRINSIC_GUESS]  K1, K2 contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
	/// *   [cv::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
	/// of intrinsic optimization.
	/// *   [cv::CALIB_CHECK_COND]  The functions will check validity of condition number.
	/// *   [cv::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
	/// *   [cv::CALIB_FIX_K1],..., [cv::CALIB_FIX_K4] Selected distortion coefficients are set to zeros and stay
	/// zero.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Note
	/// This alternative version of [stereo_calibrate_2] function uses the following default values for its arguments:
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn stereo_calibrate_2_def(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, k1: &mut impl ToInputOutputArray, d1: &mut impl ToInputOutputArray, k2: &mut impl ToInputOutputArray, d2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(k1);
		input_output_array_arg!(d1);
		input_output_array_arg!(k2);
		input_output_array_arg!(d2);
		output_array_arg!(r);
		output_array_arg!(t);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), k1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), &image_size, r.as_raw__OutputArray(), t.as_raw__OutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs stereo calibration
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera.
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera.
	/// * K1: Input/output first camera intrinsic matrix:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cvecthreethree%7Bf%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bc%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bf%5Fy%5E%7B%28j%29%7D%7D%7Bc%5Fy%5E%7B%28j%29%7D%7D%7B0%7D%7B0%7D%7B1%7D) , ![inline formula](https://latex.codecogs.com/png.latex?j%20%3D%200%2C%5C%2C%201) . If
	/// any of [cv::CALIB_USE_INTRINSIC_GUESS] , [cv::CALIB_FIX_INTRINSIC] are specified,
	/// some or all of the matrix components must be initialized.
	/// * D1: Input/output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) of 4 elements.
	/// * K2: Input/output second camera intrinsic matrix. The parameter is similar to K1 .
	/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
	/// similar to D1 .
	/// * imageSize: Size of the image used only to initialize camera intrinsic matrix.
	/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
	/// * T: Output translation vector between the coordinate systems of the cameras.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [cv::CALIB_FIX_INTRINSIC]  Fix K1, K2? and D1, D2? so that only R, T matrices
	/// are estimated.
	/// *   [cv::CALIB_USE_INTRINSIC_GUESS]  K1, K2 contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
	/// *   [cv::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
	/// of intrinsic optimization.
	/// *   [cv::CALIB_CHECK_COND]  The functions will check validity of condition number.
	/// *   [cv::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
	/// *   [cv::CALIB_FIX_K1],..., [cv::CALIB_FIX_K4] Selected distortion coefficients are set to zeros and stay
	/// zero.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## C++ default parameters
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn stereo_calibrate_2(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, k1: &mut impl ToInputOutputArray, d1: &mut impl ToInputOutputArray, k2: &mut impl ToInputOutputArray, d2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(k1);
		input_output_array_arg!(d1);
		input_output_array_arg!(k2);
		input_output_array_arg!(d2);
		output_array_arg!(r);
		output_array_arg!(t);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), k1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), &image_size, r.as_raw__OutputArray(), t.as_raw__OutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs stereo calibration
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera.
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera.
	/// * K1: Input/output first camera intrinsic matrix:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cvecthreethree%7Bf%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bc%5Fx%5E%7B%28j%29%7D%7D%7B0%7D%7Bf%5Fy%5E%7B%28j%29%7D%7D%7Bc%5Fy%5E%7B%28j%29%7D%7D%7B0%7D%7B0%7D%7B1%7D) , ![inline formula](https://latex.codecogs.com/png.latex?j%20%3D%200%2C%5C%2C%201) . If
	/// any of [cv::CALIB_USE_INTRINSIC_GUESS] , [cv::CALIB_FIX_INTRINSIC] are specified,
	/// some or all of the matrix components must be initialized.
	/// * D1: Input/output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) of 4 elements.
	/// * K2: Input/output second camera intrinsic matrix. The parameter is similar to K1 .
	/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
	/// similar to D1 .
	/// * imageSize: Size of the image used only to initialize camera intrinsic matrix.
	/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
	/// * T: Output translation vector between the coordinate systems of the cameras.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [cv::CALIB_FIX_INTRINSIC]  Fix K1, K2? and D1, D2? so that only R, T matrices
	/// are estimated.
	/// *   [cv::CALIB_USE_INTRINSIC_GUESS]  K1, K2 contains valid initial values of
	/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
	/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
	/// *   [cv::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
	/// of intrinsic optimization.
	/// *   [cv::CALIB_CHECK_COND]  The functions will check validity of condition number.
	/// *   [cv::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
	/// *   [cv::CALIB_FIX_K1],..., [cv::CALIB_FIX_K4] Selected distortion coefficients are set to zeros and stay
	/// zero.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
	#[inline]
	pub fn fisheye_stereo_calibrate(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, k1: &mut impl ToInputOutputArray, d1: &mut impl ToInputOutputArray, k2: &mut impl ToInputOutputArray, d2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(k1);
		input_output_array_arg!(d1);
		input_output_array_arg!(k2);
		input_output_array_arg!(d2);
		output_array_arg!(r);
		output_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fisheye_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), k1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), &image_size, r.as_raw__OutputArray(), t.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds an initial camera intrinsic matrix from 3D-2D point correspondences.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points in the calibration pattern
	/// coordinate space. In the old interface all the per-view vectors are concatenated. See
	/// [calibrate_camera] for details.
	/// * imagePoints: Vector of vectors of the projections of the calibration pattern points. In the
	/// old interface all the per-view vectors are concatenated.
	/// * imageSize: Image size in pixels used to initialize the principal point.
	/// * aspectRatio: If it is zero or negative, both ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) are estimated independently.
	/// Otherwise, ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%3D%20f%5Fy%20%5Ccdot%20%5Ctexttt%7BaspectRatio%7D) .
	///
	/// The function estimates and returns an initial camera intrinsic matrix for the camera calibration process.
	/// Currently, the function only supports planar calibration patterns, which are patterns where each
	/// object point has z-coordinate =0.
	///
	/// ## Note
	/// This alternative version of [init_camera_matrix_2d] function uses the following default values for its arguments:
	/// * aspect_ratio: 1.0
	#[inline]
	pub fn init_camera_matrix_2d_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size) -> Result<core::Mat> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Finds an initial camera intrinsic matrix from 3D-2D point correspondences.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points in the calibration pattern
	/// coordinate space. In the old interface all the per-view vectors are concatenated. See
	/// [calibrate_camera] for details.
	/// * imagePoints: Vector of vectors of the projections of the calibration pattern points. In the
	/// old interface all the per-view vectors are concatenated.
	/// * imageSize: Image size in pixels used to initialize the principal point.
	/// * aspectRatio: If it is zero or negative, both ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) are estimated independently.
	/// Otherwise, ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%3D%20f%5Fy%20%5Ccdot%20%5Ctexttt%7BaspectRatio%7D) .
	///
	/// The function estimates and returns an initial camera intrinsic matrix for the camera calibration process.
	/// Currently, the function only supports planar calibration patterns, which are patterns where each
	/// object point has z-coordinate =0.
	///
	/// ## C++ default parameters
	/// * aspect_ratio: 1.0
	#[inline]
	pub fn init_camera_matrix_2d(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, aspect_ratio: f64) -> Result<core::Mat> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size_double(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, aspect_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Calibrates a camera pair set up. This function finds the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints1: Vector of vectors of the calibration pattern points for camera 1.
	/// A similar structure as objectPoints in [calibrateCamera] and for each pattern view,
	/// both cameras do not need to see the same object points. objectPoints1.size(), imagePoints1.size()
	/// nees to be equal,as well as objectPoints1[i].size(), imagePoints1[i].size() need to be equal for each i.
	/// * objectPoints2: Vector of vectors of the calibration pattern points for camera 2.
	/// A similar structure as objectPoints1. objectPoints2.size(), and imagePoints2.size() nees to be equal,
	/// as well as objectPoints2[i].size(), imagePoints2[i].size() need to be equal for each i.
	/// However, objectPoints1[i].size() and objectPoints2[i].size() are not required to be equal.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraModel1: Flag reflecting the type of model for camera 1 (pinhole / fisheye):
	/// - [CALIB_MODEL_PINHOLE] pinhole camera model
	/// - [CALIB_MODEL_FISHEYE] fisheye camera model
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera.
	/// See description for cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * cameraModel2: Flag reflecting the type of model for camera 2 (pinhole / fisheye).
	/// See description for cameraModel1.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to the camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras similar to stereo pair calibration.
	/// The principle follows closely to [stereoCalibrate]. To understand the problem of estimating the
	/// relative pose between a camera pair, please refer to the description there. The difference for
	/// this function is that, camera intrinsics are not optimized and two cameras are not required
	/// to have overlapping fields of view as long as they are observing the same calibration target
	/// and the absolute positions of each object point are known.
	/// ![](https://docs.opencv.org/5.0.0/register_pair.png)
	/// The above illustration shows an example where such a case may become relevant.
	/// Additionally, it supports a camera pair with the mixed model (pinhole / fisheye).
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras.
	/// ## Returns
	/// the final value of the re-projection error.
	/// ## See also
	/// calibrateCamera, stereoCalibrate
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [register_cameras] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn register_cameras_def(object_points1: &impl ToInputArray, object_points2: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_model1: crate::calib::CameraModel, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, camera_model2: crate::calib::CameraModel, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points1);
		input_array_arg!(object_points2);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points1.as_raw__InputArray(), object_points2.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_model1, camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), camera_model2, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a camera pair set up. This function finds the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints1: Vector of vectors of the calibration pattern points for camera 1.
	/// A similar structure as objectPoints in [calibrateCamera] and for each pattern view,
	/// both cameras do not need to see the same object points. objectPoints1.size(), imagePoints1.size()
	/// nees to be equal,as well as objectPoints1[i].size(), imagePoints1[i].size() need to be equal for each i.
	/// * objectPoints2: Vector of vectors of the calibration pattern points for camera 2.
	/// A similar structure as objectPoints1. objectPoints2.size(), and imagePoints2.size() nees to be equal,
	/// as well as objectPoints2[i].size(), imagePoints2[i].size() need to be equal for each i.
	/// However, objectPoints1[i].size() and objectPoints2[i].size() are not required to be equal.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraModel1: Flag reflecting the type of model for camera 1 (pinhole / fisheye):
	/// - [CALIB_MODEL_PINHOLE] pinhole camera model
	/// - [CALIB_MODEL_FISHEYE] fisheye camera model
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera.
	/// See description for cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * cameraModel2: Flag reflecting the type of model for camera 2 (pinhole / fisheye).
	/// See description for cameraModel1.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to the camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras similar to stereo pair calibration.
	/// The principle follows closely to [stereoCalibrate]. To understand the problem of estimating the
	/// relative pose between a camera pair, please refer to the description there. The difference for
	/// this function is that, camera intrinsics are not optimized and two cameras are not required
	/// to have overlapping fields of view as long as they are observing the same calibration target
	/// and the absolute positions of each object point are known.
	/// ![](https://docs.opencv.org/5.0.0/register_pair.png)
	/// The above illustration shows an example where such a case may become relevant.
	/// Additionally, it supports a camera pair with the mixed model (pinhole / fisheye).
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras.
	/// ## Returns
	/// the final value of the re-projection error.
	/// ## See also
	/// calibrateCamera, stereoCalibrate
	///
	/// ## Note
	/// This alternative version of [register_cameras_extended] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn register_cameras_extended_def(object_points1: &impl ToInputArray, object_points2: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_model1: crate::calib::CameraModel, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, camera_model2: crate::calib::CameraModel, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points1);
		input_array_arg!(object_points2);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points1.as_raw__InputArray(), object_points2.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_model1, camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), camera_model2, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a camera pair set up. This function finds the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints1: Vector of vectors of the calibration pattern points for camera 1.
	/// A similar structure as objectPoints in [calibrateCamera] and for each pattern view,
	/// both cameras do not need to see the same object points. objectPoints1.size(), imagePoints1.size()
	/// nees to be equal,as well as objectPoints1[i].size(), imagePoints1[i].size() need to be equal for each i.
	/// * objectPoints2: Vector of vectors of the calibration pattern points for camera 2.
	/// A similar structure as objectPoints1. objectPoints2.size(), and imagePoints2.size() nees to be equal,
	/// as well as objectPoints2[i].size(), imagePoints2[i].size() need to be equal for each i.
	/// However, objectPoints1[i].size() and objectPoints2[i].size() are not required to be equal.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraModel1: Flag reflecting the type of model for camera 1 (pinhole / fisheye):
	/// - [CALIB_MODEL_PINHOLE] pinhole camera model
	/// - [CALIB_MODEL_FISHEYE] fisheye camera model
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera.
	/// See description for cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * cameraModel2: Flag reflecting the type of model for camera 2 (pinhole / fisheye).
	/// See description for cameraModel1.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to the camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras similar to stereo pair calibration.
	/// The principle follows closely to [stereoCalibrate]. To understand the problem of estimating the
	/// relative pose between a camera pair, please refer to the description there. The difference for
	/// this function is that, camera intrinsics are not optimized and two cameras are not required
	/// to have overlapping fields of view as long as they are observing the same calibration target
	/// and the absolute positions of each object point are known.
	/// ![](https://docs.opencv.org/5.0.0/register_pair.png)
	/// The above illustration shows an example where such a case may become relevant.
	/// Additionally, it supports a camera pair with the mixed model (pinhole / fisheye).
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras.
	/// ## Returns
	/// the final value of the re-projection error.
	/// ## See also
	/// calibrateCamera, stereoCalibrate
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn register_cameras_extended(object_points1: &impl ToInputArray, object_points2: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_model1: crate::calib::CameraModel, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, camera_model2: crate::calib::CameraModel, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points1);
		input_array_arg!(object_points2);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points1.as_raw__InputArray(), object_points2.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_model1, camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), camera_model2, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a camera pair set up. This function finds the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints1: Vector of vectors of the calibration pattern points for camera 1.
	/// A similar structure as objectPoints in [calibrateCamera] and for each pattern view,
	/// both cameras do not need to see the same object points. objectPoints1.size(), imagePoints1.size()
	/// nees to be equal,as well as objectPoints1[i].size(), imagePoints1[i].size() need to be equal for each i.
	/// * objectPoints2: Vector of vectors of the calibration pattern points for camera 2.
	/// A similar structure as objectPoints1. objectPoints2.size(), and imagePoints2.size() nees to be equal,
	/// as well as objectPoints2[i].size(), imagePoints2[i].size() need to be equal for each i.
	/// However, objectPoints1[i].size() and objectPoints2[i].size() are not required to be equal.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraModel1: Flag reflecting the type of model for camera 1 (pinhole / fisheye):
	/// - [CALIB_MODEL_PINHOLE] pinhole camera model
	/// - [CALIB_MODEL_FISHEYE] fisheye camera model
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera.
	/// See description for cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * cameraModel2: Flag reflecting the type of model for camera 2 (pinhole / fisheye).
	/// See description for cameraModel1.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to the camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras similar to stereo pair calibration.
	/// The principle follows closely to [stereoCalibrate]. To understand the problem of estimating the
	/// relative pose between a camera pair, please refer to the description there. The difference for
	/// this function is that, camera intrinsics are not optimized and two cameras are not required
	/// to have overlapping fields of view as long as they are observing the same calibration target
	/// and the absolute positions of each object point are known.
	/// ![](https://docs.opencv.org/5.0.0/register_pair.png)
	/// The above illustration shows an example where such a case may become relevant.
	/// Additionally, it supports a camera pair with the mixed model (pinhole / fisheye).
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras.
	/// ## Returns
	/// the final value of the re-projection error.
	/// ## See also
	/// calibrateCamera, stereoCalibrate
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn register_cameras(object_points1: &impl ToInputArray, object_points2: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_model1: crate::calib::CameraModel, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, camera_model2: crate::calib::CameraModel, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points1);
		input_array_arg!(object_points2);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_array_arg!(camera_matrix1);
		input_array_arg!(dist_coeffs1);
		input_array_arg!(camera_matrix2);
		input_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_registerCameras_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_CameraModel_const__InputArrayR_const__InputArrayR_CameraModel_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points1.as_raw__InputArray(), object_points2.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_model1, camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), camera_model2, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a stereo camera set up. This function finds the intrinsic parameters
	/// for each of the two cameras and the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points. The same structure as
	/// in [calibrateCamera]. For each pattern view, both cameras need to see the same object
	/// points. Therefore, objectPoints.size(), imagePoints1.size(), and imagePoints2.size() need to be
	/// equal as well as objectPoints[i].size(), imagePoints1[i].size(), and imagePoints2[i].size() need to
	/// be equal for each i.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera. See description for
	/// cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrices.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_FIX_INTRINSIC] Fix cameraMatrix? and distCoeffs? so that only R, T, E, and F
	/// matrices are estimated.
	/// *   [CALIB_USE_INTRINSIC_GUESS] Optimize some or all of the intrinsic parameters
	/// according to the specified flags. Initial values are provided by the user.
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// Otherwise R and T are initialized to the median value of the pattern views (each dimension separately).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] Fix the principal points during the optimization.
	/// *   [CALIB_FIX_FOCAL_LENGTH] Fix ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) .
	/// *   [CALIB_FIX_ASPECT_RATIO] Optimize ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) . Fix the ratio ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx%2Ff%5E%7B%28j%29%7D%5Fy)
	/// .
	/// *   [CALIB_SAME_FOCAL_LENGTH] Enforce ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fx%3Df%5E%7B%281%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fy%3Df%5E%7B%281%29%7D%5Fy) .
	/// *   [CALIB_ZERO_TANGENT_DIST] Set tangential distortion coefficients for each camera to
	/// zeros and fix there.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] Do not change the corresponding radial
	/// distortion coefficient during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set,
	/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Enable coefficients k4, k5, and k6. To provide the backward
	/// compatibility, this extra flag should be explicitly specified to make the calibration
	/// function use the rational model and return 8 coefficients. If the flag is not set, the
	/// function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras making a stereo pair. If one computes
	/// the poses of an object relative to the first camera and to the second camera,
	/// ( ![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1) ) and (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)), respectively, for a stereo camera where the
	/// relative position and orientation between the two cameras are fixed, then those poses definitely
	/// relate to each other. This means, if the relative position and orientation (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) of the
	/// two cameras is known, it is possible to compute (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)) when (![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1)) is
	/// given. This is what the described function does. It computes (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) such that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?R%5F2%3DR%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?T%5F2%3DR%20T%5F1%20%2B%20T%2E)
	///
	/// Therefore, one can compute the coordinate representation of a 3D point for the second camera's
	/// coordinate system when given the point's coordinate representation in the first camera's coordinate
	/// system:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5F2%20%5C%5C%0AY%5F2%20%5C%5C%0AZ%5F2%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20T%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5F1%20%5C%5C%0AY%5F1%20%5C%5C%0AZ%5F1%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	///
	///
	/// Optionally, it computes the essential matrix E:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?E%3D%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2DT%5F2%20%26%20T%5F1%5C%5C%20T%5F2%20%26%200%20%26%20%2DT%5F0%5C%5C%20%2DT%5F1%20%26%20T%5F0%20%26%200%20%5Cend%7Bbmatrix%7D%20R)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fi) are components of the translation vector ![inline formula](https://latex.codecogs.com/png.latex?T) : ![inline formula](https://latex.codecogs.com/png.latex?T%3D%5BT%5F0%2C%20T%5F1%2C%20T%5F2%5D%5ET) .
	/// And the function can also compute the fundamental matrix F:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?F%20%3D%20cameraMatrix2%5E%7B%2DT%7D%5Ccdot%20E%20%5Ccdot%20cameraMatrix1%5E%7B%2D1%7D)
	///
	/// Besides the stereo-related information, the function can also perform a full calibration of each of
	/// the two cameras. However, due to the high dimensionality of the parameter space and noise in the
	/// input data, the function can diverge from the correct solution. If the intrinsic parameters can be
	/// estimated with high accuracy for each of the cameras individually (for example, using
	/// [calibrate_camera] ), you are recommended to do so and then pass [CALIB_FIX_INTRINSIC] flag to the
	/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
	/// estimated at once, it makes sense to restrict some parameters, for example, pass
	///  [CALIB_SAME_FOCAL_LENGTH] and [CALIB_ZERO_TANGENT_DIST] flags, which is usually a
	/// reasonable assumption.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras. The function returns the final value of the
	/// re-projection error.
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [stereo_calibrate_1] function uses the following default values for its arguments:
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,1e-6)
	#[inline]
	pub fn stereo_calibrate_1_def(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(camera_matrix1);
		input_output_array_arg!(dist_coeffs1);
		input_output_array_arg!(camera_matrix2);
		input_output_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputOutputArray(), dist_coeffs1.as_raw__InputOutputArray(), camera_matrix2.as_raw__InputOutputArray(), dist_coeffs2.as_raw__InputOutputArray(), &image_size, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a stereo camera set up. This function finds the intrinsic parameters
	/// for each of the two cameras and the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points. The same structure as
	/// in [calibrateCamera]. For each pattern view, both cameras need to see the same object
	/// points. Therefore, objectPoints.size(), imagePoints1.size(), and imagePoints2.size() need to be
	/// equal as well as objectPoints[i].size(), imagePoints1[i].size(), and imagePoints2[i].size() need to
	/// be equal for each i.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera. See description for
	/// cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrices.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_FIX_INTRINSIC] Fix cameraMatrix? and distCoeffs? so that only R, T, E, and F
	/// matrices are estimated.
	/// *   [CALIB_USE_INTRINSIC_GUESS] Optimize some or all of the intrinsic parameters
	/// according to the specified flags. Initial values are provided by the user.
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// Otherwise R and T are initialized to the median value of the pattern views (each dimension separately).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] Fix the principal points during the optimization.
	/// *   [CALIB_FIX_FOCAL_LENGTH] Fix ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) .
	/// *   [CALIB_FIX_ASPECT_RATIO] Optimize ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) . Fix the ratio ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx%2Ff%5E%7B%28j%29%7D%5Fy)
	/// .
	/// *   [CALIB_SAME_FOCAL_LENGTH] Enforce ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fx%3Df%5E%7B%281%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fy%3Df%5E%7B%281%29%7D%5Fy) .
	/// *   [CALIB_ZERO_TANGENT_DIST] Set tangential distortion coefficients for each camera to
	/// zeros and fix there.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] Do not change the corresponding radial
	/// distortion coefficient during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set,
	/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Enable coefficients k4, k5, and k6. To provide the backward
	/// compatibility, this extra flag should be explicitly specified to make the calibration
	/// function use the rational model and return 8 coefficients. If the flag is not set, the
	/// function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras making a stereo pair. If one computes
	/// the poses of an object relative to the first camera and to the second camera,
	/// ( ![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1) ) and (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)), respectively, for a stereo camera where the
	/// relative position and orientation between the two cameras are fixed, then those poses definitely
	/// relate to each other. This means, if the relative position and orientation (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) of the
	/// two cameras is known, it is possible to compute (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)) when (![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1)) is
	/// given. This is what the described function does. It computes (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) such that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?R%5F2%3DR%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?T%5F2%3DR%20T%5F1%20%2B%20T%2E)
	///
	/// Therefore, one can compute the coordinate representation of a 3D point for the second camera's
	/// coordinate system when given the point's coordinate representation in the first camera's coordinate
	/// system:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5F2%20%5C%5C%0AY%5F2%20%5C%5C%0AZ%5F2%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20T%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5F1%20%5C%5C%0AY%5F1%20%5C%5C%0AZ%5F1%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	///
	///
	/// Optionally, it computes the essential matrix E:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?E%3D%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2DT%5F2%20%26%20T%5F1%5C%5C%20T%5F2%20%26%200%20%26%20%2DT%5F0%5C%5C%20%2DT%5F1%20%26%20T%5F0%20%26%200%20%5Cend%7Bbmatrix%7D%20R)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fi) are components of the translation vector ![inline formula](https://latex.codecogs.com/png.latex?T) : ![inline formula](https://latex.codecogs.com/png.latex?T%3D%5BT%5F0%2C%20T%5F1%2C%20T%5F2%5D%5ET) .
	/// And the function can also compute the fundamental matrix F:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?F%20%3D%20cameraMatrix2%5E%7B%2DT%7D%5Ccdot%20E%20%5Ccdot%20cameraMatrix1%5E%7B%2D1%7D)
	///
	/// Besides the stereo-related information, the function can also perform a full calibration of each of
	/// the two cameras. However, due to the high dimensionality of the parameter space and noise in the
	/// input data, the function can diverge from the correct solution. If the intrinsic parameters can be
	/// estimated with high accuracy for each of the cameras individually (for example, using
	/// [calibrate_camera] ), you are recommended to do so and then pass [CALIB_FIX_INTRINSIC] flag to the
	/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
	/// estimated at once, it makes sense to restrict some parameters, for example, pass
	///  [CALIB_SAME_FOCAL_LENGTH] and [CALIB_ZERO_TANGENT_DIST] flags, which is usually a
	/// reasonable assumption.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras. The function returns the final value of the
	/// re-projection error.
	///
	/// ## Note
	/// This alternative version of [stereo_calibrate_extended] function uses the following default values for its arguments:
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn stereo_calibrate_extended_def(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(camera_matrix1);
		input_output_array_arg!(dist_coeffs1);
		input_output_array_arg!(camera_matrix2);
		input_output_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputOutputArray(), dist_coeffs1.as_raw__InputOutputArray(), camera_matrix2.as_raw__InputOutputArray(), dist_coeffs2.as_raw__InputOutputArray(), &image_size, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a stereo camera set up. This function finds the intrinsic parameters
	/// for each of the two cameras and the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points. The same structure as
	/// in [calibrateCamera]. For each pattern view, both cameras need to see the same object
	/// points. Therefore, objectPoints.size(), imagePoints1.size(), and imagePoints2.size() need to be
	/// equal as well as objectPoints[i].size(), imagePoints1[i].size(), and imagePoints2[i].size() need to
	/// be equal for each i.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera. See description for
	/// cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrices.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_FIX_INTRINSIC] Fix cameraMatrix? and distCoeffs? so that only R, T, E, and F
	/// matrices are estimated.
	/// *   [CALIB_USE_INTRINSIC_GUESS] Optimize some or all of the intrinsic parameters
	/// according to the specified flags. Initial values are provided by the user.
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// Otherwise R and T are initialized to the median value of the pattern views (each dimension separately).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] Fix the principal points during the optimization.
	/// *   [CALIB_FIX_FOCAL_LENGTH] Fix ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) .
	/// *   [CALIB_FIX_ASPECT_RATIO] Optimize ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) . Fix the ratio ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx%2Ff%5E%7B%28j%29%7D%5Fy)
	/// .
	/// *   [CALIB_SAME_FOCAL_LENGTH] Enforce ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fx%3Df%5E%7B%281%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fy%3Df%5E%7B%281%29%7D%5Fy) .
	/// *   [CALIB_ZERO_TANGENT_DIST] Set tangential distortion coefficients for each camera to
	/// zeros and fix there.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] Do not change the corresponding radial
	/// distortion coefficient during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set,
	/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Enable coefficients k4, k5, and k6. To provide the backward
	/// compatibility, this extra flag should be explicitly specified to make the calibration
	/// function use the rational model and return 8 coefficients. If the flag is not set, the
	/// function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras making a stereo pair. If one computes
	/// the poses of an object relative to the first camera and to the second camera,
	/// ( ![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1) ) and (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)), respectively, for a stereo camera where the
	/// relative position and orientation between the two cameras are fixed, then those poses definitely
	/// relate to each other. This means, if the relative position and orientation (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) of the
	/// two cameras is known, it is possible to compute (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)) when (![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1)) is
	/// given. This is what the described function does. It computes (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) such that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?R%5F2%3DR%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?T%5F2%3DR%20T%5F1%20%2B%20T%2E)
	///
	/// Therefore, one can compute the coordinate representation of a 3D point for the second camera's
	/// coordinate system when given the point's coordinate representation in the first camera's coordinate
	/// system:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5F2%20%5C%5C%0AY%5F2%20%5C%5C%0AZ%5F2%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20T%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5F1%20%5C%5C%0AY%5F1%20%5C%5C%0AZ%5F1%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	///
	///
	/// Optionally, it computes the essential matrix E:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?E%3D%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2DT%5F2%20%26%20T%5F1%5C%5C%20T%5F2%20%26%200%20%26%20%2DT%5F0%5C%5C%20%2DT%5F1%20%26%20T%5F0%20%26%200%20%5Cend%7Bbmatrix%7D%20R)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fi) are components of the translation vector ![inline formula](https://latex.codecogs.com/png.latex?T) : ![inline formula](https://latex.codecogs.com/png.latex?T%3D%5BT%5F0%2C%20T%5F1%2C%20T%5F2%5D%5ET) .
	/// And the function can also compute the fundamental matrix F:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?F%20%3D%20cameraMatrix2%5E%7B%2DT%7D%5Ccdot%20E%20%5Ccdot%20cameraMatrix1%5E%7B%2D1%7D)
	///
	/// Besides the stereo-related information, the function can also perform a full calibration of each of
	/// the two cameras. However, due to the high dimensionality of the parameter space and noise in the
	/// input data, the function can diverge from the correct solution. If the intrinsic parameters can be
	/// estimated with high accuracy for each of the cameras individually (for example, using
	/// [calibrate_camera] ), you are recommended to do so and then pass [CALIB_FIX_INTRINSIC] flag to the
	/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
	/// estimated at once, it makes sense to restrict some parameters, for example, pass
	///  [CALIB_SAME_FOCAL_LENGTH] and [CALIB_ZERO_TANGENT_DIST] flags, which is usually a
	/// reasonable assumption.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras. The function returns the final value of the
	/// re-projection error.
	///
	/// ## C++ default parameters
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn stereo_calibrate_extended(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(camera_matrix1);
		input_output_array_arg!(dist_coeffs1);
		input_output_array_arg!(camera_matrix2);
		input_output_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputOutputArray(), dist_coeffs1.as_raw__InputOutputArray(), camera_matrix2.as_raw__InputOutputArray(), dist_coeffs2.as_raw__InputOutputArray(), &image_size, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a stereo camera set up. This function finds the intrinsic parameters
	/// for each of the two cameras and the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points. The same structure as
	/// in [calibrateCamera]. For each pattern view, both cameras need to see the same object
	/// points. Therefore, objectPoints.size(), imagePoints1.size(), and imagePoints2.size() need to be
	/// equal as well as objectPoints[i].size(), imagePoints1[i].size(), and imagePoints2[i].size() need to
	/// be equal for each i.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera. See description for
	/// cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrices.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_FIX_INTRINSIC] Fix cameraMatrix? and distCoeffs? so that only R, T, E, and F
	/// matrices are estimated.
	/// *   [CALIB_USE_INTRINSIC_GUESS] Optimize some or all of the intrinsic parameters
	/// according to the specified flags. Initial values are provided by the user.
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// Otherwise R and T are initialized to the median value of the pattern views (each dimension separately).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] Fix the principal points during the optimization.
	/// *   [CALIB_FIX_FOCAL_LENGTH] Fix ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) .
	/// *   [CALIB_FIX_ASPECT_RATIO] Optimize ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) . Fix the ratio ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx%2Ff%5E%7B%28j%29%7D%5Fy)
	/// .
	/// *   [CALIB_SAME_FOCAL_LENGTH] Enforce ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fx%3Df%5E%7B%281%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fy%3Df%5E%7B%281%29%7D%5Fy) .
	/// *   [CALIB_ZERO_TANGENT_DIST] Set tangential distortion coefficients for each camera to
	/// zeros and fix there.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] Do not change the corresponding radial
	/// distortion coefficient during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set,
	/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Enable coefficients k4, k5, and k6. To provide the backward
	/// compatibility, this extra flag should be explicitly specified to make the calibration
	/// function use the rational model and return 8 coefficients. If the flag is not set, the
	/// function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras making a stereo pair. If one computes
	/// the poses of an object relative to the first camera and to the second camera,
	/// ( ![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1) ) and (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)), respectively, for a stereo camera where the
	/// relative position and orientation between the two cameras are fixed, then those poses definitely
	/// relate to each other. This means, if the relative position and orientation (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) of the
	/// two cameras is known, it is possible to compute (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)) when (![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1)) is
	/// given. This is what the described function does. It computes (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) such that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?R%5F2%3DR%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?T%5F2%3DR%20T%5F1%20%2B%20T%2E)
	///
	/// Therefore, one can compute the coordinate representation of a 3D point for the second camera's
	/// coordinate system when given the point's coordinate representation in the first camera's coordinate
	/// system:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5F2%20%5C%5C%0AY%5F2%20%5C%5C%0AZ%5F2%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20T%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5F1%20%5C%5C%0AY%5F1%20%5C%5C%0AZ%5F1%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	///
	///
	/// Optionally, it computes the essential matrix E:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?E%3D%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2DT%5F2%20%26%20T%5F1%5C%5C%20T%5F2%20%26%200%20%26%20%2DT%5F0%5C%5C%20%2DT%5F1%20%26%20T%5F0%20%26%200%20%5Cend%7Bbmatrix%7D%20R)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fi) are components of the translation vector ![inline formula](https://latex.codecogs.com/png.latex?T) : ![inline formula](https://latex.codecogs.com/png.latex?T%3D%5BT%5F0%2C%20T%5F1%2C%20T%5F2%5D%5ET) .
	/// And the function can also compute the fundamental matrix F:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?F%20%3D%20cameraMatrix2%5E%7B%2DT%7D%5Ccdot%20E%20%5Ccdot%20cameraMatrix1%5E%7B%2D1%7D)
	///
	/// Besides the stereo-related information, the function can also perform a full calibration of each of
	/// the two cameras. However, due to the high dimensionality of the parameter space and noise in the
	/// input data, the function can diverge from the correct solution. If the intrinsic parameters can be
	/// estimated with high accuracy for each of the cameras individually (for example, using
	/// [calibrate_camera] ), you are recommended to do so and then pass [CALIB_FIX_INTRINSIC] flag to the
	/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
	/// estimated at once, it makes sense to restrict some parameters, for example, pass
	///  [CALIB_SAME_FOCAL_LENGTH] and [CALIB_ZERO_TANGENT_DIST] flags, which is usually a
	/// reasonable assumption.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras. The function returns the final value of the
	/// re-projection error.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,1e-6)
	#[inline]
	pub fn stereo_calibrate_1(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(camera_matrix1);
		input_output_array_arg!(dist_coeffs1);
		input_output_array_arg!(camera_matrix2);
		input_output_array_arg!(dist_coeffs2);
		input_output_array_arg!(r);
		input_output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputOutputArray(), dist_coeffs1.as_raw__InputOutputArray(), camera_matrix2.as_raw__InputOutputArray(), dist_coeffs2.as_raw__InputOutputArray(), &image_size, r.as_raw__InputOutputArray(), t.as_raw__InputOutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a stereo camera set up. This function finds the intrinsic parameters
	/// for each of the two cameras and the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points. The same structure as
	/// in [calibrateCamera]. For each pattern view, both cameras need to see the same object
	/// points. Therefore, objectPoints.size(), imagePoints1.size(), and imagePoints2.size() need to be
	/// equal as well as objectPoints[i].size(), imagePoints1[i].size(), and imagePoints2[i].size() need to
	/// be equal for each i.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera. See description for
	/// cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrices.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_FIX_INTRINSIC] Fix cameraMatrix? and distCoeffs? so that only R, T, E, and F
	/// matrices are estimated.
	/// *   [CALIB_USE_INTRINSIC_GUESS] Optimize some or all of the intrinsic parameters
	/// according to the specified flags. Initial values are provided by the user.
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// Otherwise R and T are initialized to the median value of the pattern views (each dimension separately).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] Fix the principal points during the optimization.
	/// *   [CALIB_FIX_FOCAL_LENGTH] Fix ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) .
	/// *   [CALIB_FIX_ASPECT_RATIO] Optimize ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) . Fix the ratio ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx%2Ff%5E%7B%28j%29%7D%5Fy)
	/// .
	/// *   [CALIB_SAME_FOCAL_LENGTH] Enforce ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fx%3Df%5E%7B%281%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fy%3Df%5E%7B%281%29%7D%5Fy) .
	/// *   [CALIB_ZERO_TANGENT_DIST] Set tangential distortion coefficients for each camera to
	/// zeros and fix there.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] Do not change the corresponding radial
	/// distortion coefficient during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set,
	/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Enable coefficients k4, k5, and k6. To provide the backward
	/// compatibility, this extra flag should be explicitly specified to make the calibration
	/// function use the rational model and return 8 coefficients. If the flag is not set, the
	/// function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras making a stereo pair. If one computes
	/// the poses of an object relative to the first camera and to the second camera,
	/// ( ![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1) ) and (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)), respectively, for a stereo camera where the
	/// relative position and orientation between the two cameras are fixed, then those poses definitely
	/// relate to each other. This means, if the relative position and orientation (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) of the
	/// two cameras is known, it is possible to compute (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)) when (![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1)) is
	/// given. This is what the described function does. It computes (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) such that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?R%5F2%3DR%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?T%5F2%3DR%20T%5F1%20%2B%20T%2E)
	///
	/// Therefore, one can compute the coordinate representation of a 3D point for the second camera's
	/// coordinate system when given the point's coordinate representation in the first camera's coordinate
	/// system:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5F2%20%5C%5C%0AY%5F2%20%5C%5C%0AZ%5F2%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20T%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5F1%20%5C%5C%0AY%5F1%20%5C%5C%0AZ%5F1%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	///
	///
	/// Optionally, it computes the essential matrix E:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?E%3D%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2DT%5F2%20%26%20T%5F1%5C%5C%20T%5F2%20%26%200%20%26%20%2DT%5F0%5C%5C%20%2DT%5F1%20%26%20T%5F0%20%26%200%20%5Cend%7Bbmatrix%7D%20R)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fi) are components of the translation vector ![inline formula](https://latex.codecogs.com/png.latex?T) : ![inline formula](https://latex.codecogs.com/png.latex?T%3D%5BT%5F0%2C%20T%5F1%2C%20T%5F2%5D%5ET) .
	/// And the function can also compute the fundamental matrix F:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?F%20%3D%20cameraMatrix2%5E%7B%2DT%7D%5Ccdot%20E%20%5Ccdot%20cameraMatrix1%5E%7B%2D1%7D)
	///
	/// Besides the stereo-related information, the function can also perform a full calibration of each of
	/// the two cameras. However, due to the high dimensionality of the parameter space and noise in the
	/// input data, the function can diverge from the correct solution. If the intrinsic parameters can be
	/// estimated with high accuracy for each of the cameras individually (for example, using
	/// [calibrate_camera] ), you are recommended to do so and then pass [CALIB_FIX_INTRINSIC] flag to the
	/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
	/// estimated at once, it makes sense to restrict some parameters, for example, pass
	///  [CALIB_SAME_FOCAL_LENGTH] and [CALIB_ZERO_TANGENT_DIST] flags, which is usually a
	/// reasonable assumption.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras. The function returns the final value of the
	/// re-projection error.
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [stereo_calibrate] function uses the following default values for its arguments:
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn stereo_calibrate_def(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(camera_matrix1);
		input_output_array_arg!(dist_coeffs1);
		input_output_array_arg!(camera_matrix2);
		input_output_array_arg!(dist_coeffs2);
		output_array_arg!(r);
		output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputOutputArray(), dist_coeffs1.as_raw__InputOutputArray(), camera_matrix2.as_raw__InputOutputArray(), dist_coeffs2.as_raw__InputOutputArray(), &image_size, r.as_raw__OutputArray(), t.as_raw__OutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calibrates a stereo camera set up. This function finds the intrinsic parameters
	/// for each of the two cameras and the extrinsic parameters between the two cameras.
	///
	/// ## Parameters
	/// * objectPoints: Vector of vectors of the calibration pattern points. The same structure as
	/// in [calibrateCamera]. For each pattern view, both cameras need to see the same object
	/// points. Therefore, objectPoints.size(), imagePoints1.size(), and imagePoints2.size() need to be
	/// equal as well as objectPoints[i].size(), imagePoints1[i].size(), and imagePoints2[i].size() need to
	/// be equal for each i.
	/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the first camera. The same structure as in [calibrateCamera].
	/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
	/// observed by the second camera. The same structure as in [calibrateCamera].
	/// * cameraMatrix1: Input/output camera intrinsic matrix for the first camera, the same as in
	/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
	/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
	/// [calibrateCamera].
	/// * cameraMatrix2: Input/output second camera intrinsic matrix for the second camera. See description for
	/// cameraMatrix1.
	/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. See
	/// description for distCoeffs1.
	/// * imageSize: Size of the image used only to initialize the camera intrinsic matrices.
	/// * R: Output rotation matrix. Together with the translation vector T, this matrix brings
	/// points given in the first camera's coordinate system to points in the second camera's
	/// coordinate system. In more technical terms, the tuple of R and T performs a change of basis
	/// from the first camera's coordinate system to the second camera's coordinate system. Due to its
	/// duality, this tuple is equivalent to the position of the first camera with respect to the
	/// second camera coordinate system.
	/// * T: Output translation vector, see description above.
	/// * E: Output essential matrix.
	/// * F: Output fundamental matrix.
	/// * rvecs: Output vector of rotation vectors ( [Rodrigues] ) estimated for each pattern view in the
	/// coordinate system of the first camera of the stereo pair (e.g. std::vector<cv::Mat>). More in detail, each
	/// i-th rotation vector together with the corresponding i-th translation vector (see the next output parameter
	/// description) brings the calibration pattern from the object coordinate space (in which object points are
	/// specified) to the camera coordinate space of the first camera of the stereo pair. In more technical terms,
	/// the tuple of the i-th rotation and translation vector performs a change of basis from object coordinate space
	/// to camera coordinate space of the first camera of the stereo pair.
	/// * tvecs: Output vector of translation vectors estimated for each pattern view, see parameter description
	/// of previous output parameter ( rvecs ).
	/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
	/// * flags: Different flags that may be zero or a combination of the following values:
	/// *   [CALIB_FIX_INTRINSIC] Fix cameraMatrix? and distCoeffs? so that only R, T, E, and F
	/// matrices are estimated.
	/// *   [CALIB_USE_INTRINSIC_GUESS] Optimize some or all of the intrinsic parameters
	/// according to the specified flags. Initial values are provided by the user.
	/// *   [CALIB_USE_EXTRINSIC_GUESS] R and T contain valid initial values that are optimized further.
	/// Otherwise R and T are initialized to the median value of the pattern views (each dimension separately).
	/// *   [CALIB_FIX_PRINCIPAL_POINT] Fix the principal points during the optimization.
	/// *   [CALIB_FIX_FOCAL_LENGTH] Fix ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) .
	/// *   [CALIB_FIX_ASPECT_RATIO] Optimize ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fy) . Fix the ratio ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%28j%29%7D%5Fx%2Ff%5E%7B%28j%29%7D%5Fy)
	/// .
	/// *   [CALIB_SAME_FOCAL_LENGTH] Enforce ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fx%3Df%5E%7B%281%29%7D%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5E%7B%280%29%7D%5Fy%3Df%5E%7B%281%29%7D%5Fy) .
	/// *   [CALIB_ZERO_TANGENT_DIST] Set tangential distortion coefficients for each camera to
	/// zeros and fix there.
	/// *   [CALIB_FIX_K1],..., [CALIB_FIX_K6] Do not change the corresponding radial
	/// distortion coefficient during the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set,
	/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_RATIONAL_MODEL] Enable coefficients k4, k5, and k6. To provide the backward
	/// compatibility, this extra flag should be explicitly specified to make the calibration
	/// function use the rational model and return 8 coefficients. If the flag is not set, the
	/// function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_THIN_PRISM_MODEL] Coefficients s1, s2, s3 and s4 are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_S1_S2_S3_S4] The thin prism distortion coefficients are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// *   [CALIB_TILTED_MODEL] Coefficients tauX and tauY are enabled. To provide the
	/// backward compatibility, this extra flag should be explicitly specified to make the
	/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
	/// set, the function computes and returns only 5 distortion coefficients.
	/// *   [CALIB_FIX_TAUX_TAUY] The coefficients of the tilted sensor model are not changed during
	/// the optimization. If [CALIB_USE_INTRINSIC_GUESS] is set, the coefficient from the
	/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	///
	/// The function estimates the transformation between two cameras making a stereo pair. If one computes
	/// the poses of an object relative to the first camera and to the second camera,
	/// ( ![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1) ) and (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)), respectively, for a stereo camera where the
	/// relative position and orientation between the two cameras are fixed, then those poses definitely
	/// relate to each other. This means, if the relative position and orientation (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) of the
	/// two cameras is known, it is possible to compute (![inline formula](https://latex.codecogs.com/png.latex?R%5F2),![inline formula](https://latex.codecogs.com/png.latex?T%5F2)) when (![inline formula](https://latex.codecogs.com/png.latex?R%5F1),![inline formula](https://latex.codecogs.com/png.latex?T%5F1)) is
	/// given. This is what the described function does. It computes (![inline formula](https://latex.codecogs.com/png.latex?R),![inline formula](https://latex.codecogs.com/png.latex?T)) such that:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?R%5F2%3DR%20R%5F1)
	/// ![block formula](https://latex.codecogs.com/png.latex?T%5F2%3DR%20T%5F1%20%2B%20T%2E)
	///
	/// Therefore, one can compute the coordinate representation of a 3D point for the second camera's
	/// coordinate system when given the point's coordinate representation in the first camera's coordinate
	/// system:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5F2%20%5C%5C%0AY%5F2%20%5C%5C%0AZ%5F2%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20T%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5F1%20%5C%5C%0AY%5F1%20%5C%5C%0AZ%5F1%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
	///
	///
	/// Optionally, it computes the essential matrix E:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?E%3D%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2DT%5F2%20%26%20T%5F1%5C%5C%20T%5F2%20%26%200%20%26%20%2DT%5F0%5C%5C%20%2DT%5F1%20%26%20T%5F0%20%26%200%20%5Cend%7Bbmatrix%7D%20R)
	///
	/// where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fi) are components of the translation vector ![inline formula](https://latex.codecogs.com/png.latex?T) : ![inline formula](https://latex.codecogs.com/png.latex?T%3D%5BT%5F0%2C%20T%5F1%2C%20T%5F2%5D%5ET) .
	/// And the function can also compute the fundamental matrix F:
	///
	/// ![block formula](https://latex.codecogs.com/png.latex?F%20%3D%20cameraMatrix2%5E%7B%2DT%7D%5Ccdot%20E%20%5Ccdot%20cameraMatrix1%5E%7B%2D1%7D)
	///
	/// Besides the stereo-related information, the function can also perform a full calibration of each of
	/// the two cameras. However, due to the high dimensionality of the parameter space and noise in the
	/// input data, the function can diverge from the correct solution. If the intrinsic parameters can be
	/// estimated with high accuracy for each of the cameras individually (for example, using
	/// [calibrate_camera] ), you are recommended to do so and then pass [CALIB_FIX_INTRINSIC] flag to the
	/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
	/// estimated at once, it makes sense to restrict some parameters, for example, pass
	///  [CALIB_SAME_FOCAL_LENGTH] and [CALIB_ZERO_TANGENT_DIST] flags, which is usually a
	/// reasonable assumption.
	///
	/// Similarly to #calibrateCamera, the function minimizes the total re-projection error for all the
	/// points in all the available views from both cameras. The function returns the final value of the
	/// re-projection error.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: CALIB_FIX_INTRINSIC
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	#[inline]
	pub fn stereo_calibrate(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points1);
		input_array_arg!(image_points2);
		input_output_array_arg!(camera_matrix1);
		input_output_array_arg!(dist_coeffs1);
		input_output_array_arg!(camera_matrix2);
		input_output_array_arg!(dist_coeffs2);
		output_array_arg!(r);
		output_array_arg!(t);
		output_array_arg!(e);
		output_array_arg!(f);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereoCalibrate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), camera_matrix1.as_raw__InputOutputArray(), dist_coeffs1.as_raw__InputOutputArray(), camera_matrix2.as_raw__InputOutputArray(), dist_coeffs2.as_raw__InputOutputArray(), &image_size, r.as_raw__OutputArray(), t.as_raw__OutputArray(), e.as_raw__OutputArray(), f.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}
