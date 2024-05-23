//! # 3D vision functionality
//!
//! Most of the functions in this section use a so-called pinhole camera model. The view of a scene
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
//! ![Pinhole camera model](https://docs.opencv.org/5.0.0/pinhole_camera_model.png)
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
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{LevMarqTrait, LevMarqTraitConst, LevMarq_ReportTrait, LevMarq_ReportTraitConst, LevMarq_SettingsTrait, LevMarq_SettingsTraitConst, OctreeTrait, OctreeTraitConst, OdometryFrameTrait, OdometryFrameTraitConst, OdometrySettingsTrait, OdometrySettingsTraitConst, OdometryTrait, OdometryTraitConst, RegionGrowing3DTrait, RegionGrowing3DTraitConst, RgbdNormalsTrait, RgbdNormalsTraitConst, SACSegmentationTrait, SACSegmentationTraitConst, VolumeSettingsTrait, VolumeSettingsTraitConst, VolumeTrait, VolumeTraitConst};
}

// COV_POLISHER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:428
pub const COV_POLISHER: i32 = 3;
/// 7-point algorithm
// FM_7POINT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:416
pub const FM_7POINT: i32 = 1;
/// 8-point algorithm
// FM_8POINT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:417
pub const FM_8POINT: i32 = 2;
/// least-median algorithm. 7-point algorithm is used.
// FM_LMEDS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:418
pub const FM_LMEDS: i32 = 4;
/// RANSAC algorithm. It needs at least 15 points. 7-point algorithm is used.
// FM_RANSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:419
pub const FM_RANSAC: i32 = 8;
/// least-median of squares algorithm
// LMEDS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:377
pub const LMEDS: i32 = 4;
// LOCAL_OPTIM_GC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:425
pub const LOCAL_OPTIM_GC: i32 = 3;
// LOCAL_OPTIM_INNER_AND_ITER_LO /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:424
pub const LOCAL_OPTIM_INNER_AND_ITER_LO: i32 = 2;
// LOCAL_OPTIM_INNER_LO /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:424
pub const LOCAL_OPTIM_INNER_LO: i32 = 1;
// LOCAL_OPTIM_NULL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:424
pub const LOCAL_OPTIM_NULL: i32 = 0;
// LOCAL_OPTIM_SIGMA /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:425
pub const LOCAL_OPTIM_SIGMA: i32 = 4;
// LSQ_POLISHER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:428
pub const LSQ_POLISHER: i32 = 1;
// MAGSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:428
pub const MAGSAC: i32 = 2;
// AUTO /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:485
pub const MatrixType_AUTO: i32 = 0;
// DENSE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:486
pub const MatrixType_DENSE: i32 = 1;
// SPARSE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:487
pub const MatrixType_SPARSE: i32 = 2;
// NEIGH_FLANN_KNN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:427
pub const NEIGH_FLANN_KNN: i32 = 0;
// NEIGH_FLANN_RADIUS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:427
pub const NEIGH_FLANN_RADIUS: i32 = 2;
// NEIGH_GRID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:427
pub const NEIGH_GRID: i32 = 1;
// NONE_POLISHER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:428
pub const NONE_POLISHER: i32 = 0;
// N_PYRAMIDS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:26
pub const N_PYRAMIDS: i32 = 9;
// COMMON /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:34
pub const OdometryAlgoType_COMMON: i32 = 0;
// FAST /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:35
pub const OdometryAlgoType_FAST: i32 = 1;
// DEPTH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:23
pub const OdometryType_DEPTH: i32 = 0;
// RGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:24
pub const OdometryType_RGB: i32 = 1;
// RGB_DEPTH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:25
pub const OdometryType_RGB_DEPTH: i32 = 2;
// PROJ_SPHERICAL_EQRECT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2143
pub const PROJ_SPHERICAL_EQRECT: i32 = 1;
// PROJ_SPHERICAL_ORTHO /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2142
pub const PROJ_SPHERICAL_ORTHO: i32 = 0;
/// The pyramid of point clouds, produced from the pyramid of depths
// PYR_CLOUD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:20
pub const PYR_CLOUD: i32 = 3;
/// The pyramid of depth images
// PYR_DEPTH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:18
pub const PYR_DEPTH: i32 = 1;
/// The pyramid of dI/dx derivative images
// PYR_DIX /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:21
pub const PYR_DIX: i32 = 4;
/// The pyramid of dI/dy derivative images
// PYR_DIY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:22
pub const PYR_DIY: i32 = 5;
/// The pyramid of grayscale images
// PYR_IMAGE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:17
pub const PYR_IMAGE: i32 = 0;
/// The pyramid of masks
// PYR_MASK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:19
pub const PYR_MASK: i32 = 2;
/// The pyramid of normals
// PYR_NORM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:24
pub const PYR_NORM: i32 = 7;
/// The pyramid of normals masks
// PYR_NORMMASK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:25
pub const PYR_NORMMASK: i32 = 8;
/// The pyramid of "textured" masks (i.e. additional masks for normals or grayscale images)
// PYR_TEXMASK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:23
pub const PYR_TEXMASK: i32 = 6;
/// RANSAC algorithm
// RANSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:378
pub const RANSAC: i32 = 8;
/// Color and depth have their natural values and converted to internal formats if needed
// RASTERIZE_COMPAT_DISABLED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2933
pub const RASTERIZE_COMPAT_DISABLED: i32 = 0;
/// Color is natural, Depth is transformed from [-zNear; -zFar] to [0; 1]
/// by the following formula: ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7Bz%5F%7Bfar%7D%20%5Cleft%28z%20%2B%20z%5F%7Bnear%7D%5Cright%29%7D%7Bz%20%5Cleft%28z%5F%7Bfar%7D%20%2D%20z%5F%7Bnear%7D%5Cright%29%7D%20) 
///
/// In this mode the input/output depthBuf is considered to be in this format,
/// therefore it's faster since there're no conversions performed
// RASTERIZE_COMPAT_INVDEPTH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2934
pub const RASTERIZE_COMPAT_INVDEPTH: i32 = 1;
/// triangles which vertices are given in counterclockwork order are drawn
// RASTERIZE_CULLING_CCW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2927
pub const RASTERIZE_CULLING_CCW: i32 = 2;
/// triangles which vertices are given in clockwork order are drawn
// RASTERIZE_CULLING_CW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2926
pub const RASTERIZE_CULLING_CW: i32 = 1;
/// all faces are drawn, no culling is actually performed
// RASTERIZE_CULLING_NONE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2925
pub const RASTERIZE_CULLING_NONE: i32 = 0;
/// a color of 1st vertex of each triangle is used
// RASTERIZE_SHADING_FLAT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2918
pub const RASTERIZE_SHADING_FLAT: i32 = 1;
/// a color is interpolated between 3 vertices with perspective correction
// RASTERIZE_SHADING_SHADED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2919
pub const RASTERIZE_SHADING_SHADED: i32 = 2;
/// a white color is used for the whole triangle
// RASTERIZE_SHADING_WHITE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2917
pub const RASTERIZE_SHADING_WHITE: i32 = 0;
// RGBD_PLANE_METHOD_DEFAULT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:147
pub const RGBD_PLANE_METHOD_DEFAULT: i32 = 0;
/// RHO algorithm
// RHO /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:379
pub const RHO: i32 = 16;
// RGBD_NORMALS_METHOD_CROSS_PRODUCT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:34
pub const RgbdNormals_RGBD_NORMALS_METHOD_CROSS_PRODUCT: i32 = 3;
// RGBD_NORMALS_METHOD_FALS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:31
pub const RgbdNormals_RGBD_NORMALS_METHOD_FALS: i32 = 0;
// RGBD_NORMALS_METHOD_LINEMOD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:32
pub const RgbdNormals_RGBD_NORMALS_METHOD_LINEMOD: i32 = 1;
// RGBD_NORMALS_METHOD_SRI /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:33
pub const RgbdNormals_RGBD_NORMALS_METHOD_SRI: i32 = 2;
/// The RANSAC algorithm described in [fischler1981random](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_fischler1981random).
// SAC_METHOD_RANSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:22
pub const SAC_METHOD_RANSAC: i32 = 0;
/// The 3D PLANE model coefficients in list **[a, b, c, d]**,
/// corresponding to the coefficients of equation
/// ![inline formula](https://latex.codecogs.com/png.latex?%20ax%20%2B%20by%20%2B%20cz%20%2B%20d%20%3D%200%20).
// SAC_MODEL_PLANE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:37
pub const SAC_MODEL_PLANE: i32 = 0;
/// The 3D SPHERE model coefficients in list **[center_x, center_y, center_z, radius]**,
/// corresponding to the coefficients of equation
/// ![inline formula](https://latex.codecogs.com/png.latex?%20%28x%20%2D%20center%5C%5Fx%29%5E2%20%2B%20%28y%20%2D%20center%5C%5Fy%29%5E2%20%2B%20%28z%20%2D%20center%5C%5Fz%29%5E2%20%3D%20radius%5E2%20).
// SAC_MODEL_SPHERE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:41
pub const SAC_MODEL_SPHERE: i32 = 1;
// SAMPLING_NAPSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:422
pub const SAMPLING_NAPSAC: i32 = 2;
// SAMPLING_PROGRESSIVE_NAPSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:422
pub const SAMPLING_PROGRESSIVE_NAPSAC: i32 = 1;
// SAMPLING_PROSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:423
pub const SAMPLING_PROSAC: i32 = 3;
// SAMPLING_UNIFORM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:422
pub const SAMPLING_UNIFORM: i32 = 0;
// SCORE_METHOD_LMEDS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:426
pub const SCORE_METHOD_LMEDS: i32 = 3;
// SCORE_METHOD_MAGSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:426
pub const SCORE_METHOD_MAGSAC: i32 = 2;
// SCORE_METHOD_MSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:426
pub const SCORE_METHOD_MSAC: i32 = 1;
// SCORE_METHOD_RANSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:426
pub const SCORE_METHOD_RANSAC: i32 = 0;
/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Ke17)
// SOLVEPNP_AP3P /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:399
pub const SOLVEPNP_AP3P: i32 = 5;
/// **Broken implementation. Using this flag will fallback to EPnP.** 
///
/// A Direct Least-Squares (DLS) Method for PnP [hesch2011direct](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_hesch2011direct)
// SOLVEPNP_DLS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:395
pub const SOLVEPNP_DLS: i32 = 3;
/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
// SOLVEPNP_EPNP /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:393
pub const SOLVEPNP_EPNP: i32 = 1;
/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
///
/// Object points must be coplanar.
// SOLVEPNP_IPPE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:400
pub const SOLVEPNP_IPPE: i32 = 6;
/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
///
/// This is a special case suitable for marker pose estimation.
///
/// 4 coplanar object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
// SOLVEPNP_IPPE_SQUARE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:402
pub const SOLVEPNP_IPPE_SQUARE: i32 = 7;
/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) 
///
/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
///
/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
// SOLVEPNP_ITERATIVE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:390
pub const SOLVEPNP_ITERATIVE: i32 = 0;
/// Used for count
// SOLVEPNP_MAX_COUNT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:411
pub const SOLVEPNP_MAX_COUNT: i32 = 9;
/// Complete Solution Classification for the Perspective-Three-Point Problem [gao2003complete](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_gao2003complete)
// SOLVEPNP_P3P /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:394
pub const SOLVEPNP_P3P: i32 = 2;
/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
// SOLVEPNP_SQPNP /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:409
pub const SOLVEPNP_SQPNP: i32 = 8;
/// **Broken implementation. Using this flag will fallback to EPnP.** 
///
/// Exhaustive Linearization for Robust Camera Pose and Focal Length Estimation [penate2013exhaustive](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_penate2013exhaustive)
// SOLVEPNP_UPNP /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:397
pub const SOLVEPNP_UPNP: i32 = 4;
/// USAC, accurate settings
// USAC_ACCURATE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:384
pub const USAC_ACCURATE: i32 = 36;
/// USAC algorithm, default settings
// USAC_DEFAULT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:380
pub const USAC_DEFAULT: i32 = 32;
/// USAC, fast settings
// USAC_FAST /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:383
pub const USAC_FAST: i32 = 35;
/// USAC, fundamental matrix 8 points
// USAC_FM_8PTS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:382
pub const USAC_FM_8PTS: i32 = 34;
/// USAC, runs MAGSAC++
// USAC_MAGSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:386
pub const USAC_MAGSAC: i32 = 38;
/// USAC, parallel version
// USAC_PARALLEL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:381
pub const USAC_PARALLEL: i32 = 33;
/// USAC, sorted points, runs PROSAC
// USAC_PROSAC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:385
pub const USAC_PROSAC: i32 = 37;
// LINEAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:498
pub const VariableType_LINEAR: i32 = 0;
// SE3 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:500
pub const VariableType_SE3: i32 = 2;
// SO3 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:499
pub const VariableType_SO3: i32 = 1;
// ColorTSDF /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:19
pub const VolumeType_ColorTSDF: i32 = 2;
// HashTSDF /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:18
pub const VolumeType_HashTSDF: i32 = 1;
// TSDF /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:17
pub const VolumeType_TSDF: i32 = 0;
// VOLUME_UNIT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:141
pub const Volume_VOLUME_UNIT: i32 = 0;
// VOXEL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:142
pub const Volume_VOXEL: i32 = 1;
// LocalOptimMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:424
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LocalOptimMethod {
	LOCAL_OPTIM_NULL = 0,
	LOCAL_OPTIM_INNER_LO = 1,
	LOCAL_OPTIM_INNER_AND_ITER_LO = 2,
	LOCAL_OPTIM_GC = 3,
	LOCAL_OPTIM_SIGMA = 4,
}

impl TryFrom<i32> for LocalOptimMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::LOCAL_OPTIM_NULL),
			1 => Ok(Self::LOCAL_OPTIM_INNER_LO),
			2 => Ok(Self::LOCAL_OPTIM_INNER_AND_ITER_LO),
			3 => Ok(Self::LOCAL_OPTIM_GC),
			4 => Ok(Self::LOCAL_OPTIM_SIGMA),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::LocalOptimMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::LocalOptimMethod }

/// Type of matrix used in LevMarq solver
///
/// Matrix type can be dense, sparse or chosen automatically based on a matrix size, performance considerations or backend availability.
///
/// Note: only dense matrix is now supported
// MatrixType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:483
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MatrixType {
	AUTO = 0,
	DENSE = 1,
	SPARSE = 2,
}

impl TryFrom<i32> for MatrixType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::AUTO),
			1 => Ok(Self::DENSE),
			2 => Ok(Self::SPARSE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::MatrixType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::MatrixType }

// NeighborSearchMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:427
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NeighborSearchMethod {
	NEIGH_FLANN_KNN = 0,
	NEIGH_GRID = 1,
	NEIGH_FLANN_RADIUS = 2,
}

impl TryFrom<i32> for NeighborSearchMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NEIGH_FLANN_KNN),
			1 => Ok(Self::NEIGH_GRID),
			2 => Ok(Self::NEIGH_FLANN_RADIUS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::NeighborSearchMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::NeighborSearchMethod }

/// These constants are used to set the speed and accuracy of odometry
/// ## Parameters
/// * COMMON: accurate but not so fast
/// * FAST: less accurate but faster
// OdometryAlgoType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:32
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OdometryAlgoType {
	COMMON = 0,
	FAST = 1,
}

impl TryFrom<i32> for OdometryAlgoType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::COMMON),
			1 => Ok(Self::FAST),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::OdometryAlgoType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::OdometryAlgoType }

/// Indicates what pyramid is to access using getPyramidAt() method:
// OdometryFramePyramidType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:15
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OdometryFramePyramidType {
	/// The pyramid of grayscale images
	PYR_IMAGE = 0,
	/// The pyramid of depth images
	PYR_DEPTH = 1,
	/// The pyramid of masks
	PYR_MASK = 2,
	/// The pyramid of point clouds, produced from the pyramid of depths
	PYR_CLOUD = 3,
	/// The pyramid of dI/dx derivative images
	PYR_DIX = 4,
	/// The pyramid of dI/dy derivative images
	PYR_DIY = 5,
	/// The pyramid of "textured" masks (i.e. additional masks for normals or grayscale images)
	PYR_TEXMASK = 6,
	/// The pyramid of normals
	PYR_NORM = 7,
	/// The pyramid of normals masks
	PYR_NORMMASK = 8,
	N_PYRAMIDS = 9,
}

impl TryFrom<i32> for OdometryFramePyramidType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::PYR_IMAGE),
			1 => Ok(Self::PYR_DEPTH),
			2 => Ok(Self::PYR_MASK),
			3 => Ok(Self::PYR_CLOUD),
			4 => Ok(Self::PYR_DIX),
			5 => Ok(Self::PYR_DIY),
			6 => Ok(Self::PYR_TEXMASK),
			7 => Ok(Self::PYR_NORM),
			8 => Ok(Self::PYR_NORMMASK),
			9 => Ok(Self::N_PYRAMIDS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::OdometryFramePyramidType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::OdometryFramePyramidType }

/// These constants are used to set a type of data which odometry will use
/// ## Parameters
/// * DEPTH: only depth data
/// * RGB: only rgb image
/// * RGB_DEPTH: depth and rgb data simultaneously
// OdometryType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:21
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OdometryType {
	DEPTH = 0,
	RGB = 1,
	RGB_DEPTH = 2,
}

impl TryFrom<i32> for OdometryType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DEPTH),
			1 => Ok(Self::RGB),
			2 => Ok(Self::RGB_DEPTH),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::OdometryType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::OdometryType }

// PolishingMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:428
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PolishingMethod {
	NONE_POLISHER = 0,
	LSQ_POLISHER = 1,
	MAGSAC = 2,
	COV_POLISHER = 3,
}

impl TryFrom<i32> for PolishingMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::NONE_POLISHER),
			1 => Ok(Self::LSQ_POLISHER),
			2 => Ok(Self::MAGSAC),
			3 => Ok(Self::COV_POLISHER),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::PolishingMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::PolishingMethod }

// RgbdNormalsMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:29
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RgbdNormals_RgbdNormalsMethod {
	RGBD_NORMALS_METHOD_FALS = 0,
	RGBD_NORMALS_METHOD_LINEMOD = 1,
	RGBD_NORMALS_METHOD_SRI = 2,
	RGBD_NORMALS_METHOD_CROSS_PRODUCT = 3,
}

impl TryFrom<i32> for RgbdNormals_RgbdNormalsMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RGBD_NORMALS_METHOD_FALS),
			1 => Ok(Self::RGBD_NORMALS_METHOD_LINEMOD),
			2 => Ok(Self::RGBD_NORMALS_METHOD_SRI),
			3 => Ok(Self::RGBD_NORMALS_METHOD_CROSS_PRODUCT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::RgbdNormals_RgbdNormalsMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::RgbdNormals_RgbdNormalsMethod }

// RgbdPlaneMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:145
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RgbdPlaneMethod {
	RGBD_PLANE_METHOD_DEFAULT = 0,
}

impl TryFrom<i32> for RgbdPlaneMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RGBD_PLANE_METHOD_DEFAULT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::RgbdPlaneMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::RgbdPlaneMethod }

/// type of the robust estimation algorithm
// SacMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:18
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SacMethod {
	/// The RANSAC algorithm described in [fischler1981random](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_fischler1981random).
	SAC_METHOD_RANSAC = 0,
}

impl TryFrom<i32> for SacMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SAC_METHOD_RANSAC),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::SacMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::SacMethod }

// SacModelType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:32
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SacModelType {
	/// The 3D PLANE model coefficients in list **[a, b, c, d]**,
	/// corresponding to the coefficients of equation
	/// ![inline formula](https://latex.codecogs.com/png.latex?%20ax%20%2B%20by%20%2B%20cz%20%2B%20d%20%3D%200%20).
	SAC_MODEL_PLANE = 0,
	/// The 3D SPHERE model coefficients in list **[center_x, center_y, center_z, radius]**,
	/// corresponding to the coefficients of equation
	/// ![inline formula](https://latex.codecogs.com/png.latex?%20%28x%20%2D%20center%5C%5Fx%29%5E2%20%2B%20%28y%20%2D%20center%5C%5Fy%29%5E2%20%2B%20%28z%20%2D%20center%5C%5Fz%29%5E2%20%3D%20radius%5E2%20).
	SAC_MODEL_SPHERE = 1,
}

impl TryFrom<i32> for SacModelType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SAC_MODEL_PLANE),
			1 => Ok(Self::SAC_MODEL_SPHERE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::SacModelType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::SacModelType }

// SamplingMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:422
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SamplingMethod {
	SAMPLING_UNIFORM = 0,
	SAMPLING_PROGRESSIVE_NAPSAC = 1,
	SAMPLING_NAPSAC = 2,
	SAMPLING_PROSAC = 3,
}

impl TryFrom<i32> for SamplingMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SAMPLING_UNIFORM),
			1 => Ok(Self::SAMPLING_PROGRESSIVE_NAPSAC),
			2 => Ok(Self::SAMPLING_NAPSAC),
			3 => Ok(Self::SAMPLING_PROSAC),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::SamplingMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::SamplingMethod }

// ScoreMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:426
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScoreMethod {
	SCORE_METHOD_RANSAC = 0,
	SCORE_METHOD_MSAC = 1,
	SCORE_METHOD_MAGSAC = 2,
	SCORE_METHOD_LMEDS = 3,
}

impl TryFrom<i32> for ScoreMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SCORE_METHOD_RANSAC),
			1 => Ok(Self::SCORE_METHOD_MSAC),
			2 => Ok(Self::SCORE_METHOD_MAGSAC),
			3 => Ok(Self::SCORE_METHOD_LMEDS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::ScoreMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::ScoreMethod }

// SolvePnPMethod /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:389
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SolvePnPMethod {
	/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) 
	///
	/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
	///
	/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
	SOLVEPNP_ITERATIVE = 0,
	/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
	SOLVEPNP_EPNP = 1,
	/// Complete Solution Classification for the Perspective-Three-Point Problem [gao2003complete](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_gao2003complete)
	SOLVEPNP_P3P = 2,
	/// **Broken implementation. Using this flag will fallback to EPnP.** 
	///
	/// A Direct Least-Squares (DLS) Method for PnP [hesch2011direct](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_hesch2011direct)
	SOLVEPNP_DLS = 3,
	/// **Broken implementation. Using this flag will fallback to EPnP.** 
	///
	/// Exhaustive Linearization for Robust Camera Pose and Focal Length Estimation [penate2013exhaustive](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_penate2013exhaustive)
	SOLVEPNP_UPNP = 4,
	/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Ke17)
	SOLVEPNP_AP3P = 5,
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
	///
	/// Object points must be coplanar.
	SOLVEPNP_IPPE = 6,
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Collins14) 
	///
	/// This is a special case suitable for marker pose estimation.
	///
	/// 4 coplanar object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	SOLVEPNP_IPPE_SQUARE = 7,
	/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
	SOLVEPNP_SQPNP = 8,
	/// Used for count
	SOLVEPNP_MAX_COUNT = 9,
}

impl TryFrom<i32> for SolvePnPMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SOLVEPNP_ITERATIVE),
			1 => Ok(Self::SOLVEPNP_EPNP),
			2 => Ok(Self::SOLVEPNP_P3P),
			3 => Ok(Self::SOLVEPNP_DLS),
			4 => Ok(Self::SOLVEPNP_UPNP),
			5 => Ok(Self::SOLVEPNP_AP3P),
			6 => Ok(Self::SOLVEPNP_IPPE),
			7 => Ok(Self::SOLVEPNP_IPPE_SQUARE),
			8 => Ok(Self::SOLVEPNP_SQPNP),
			9 => Ok(Self::SOLVEPNP_MAX_COUNT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::SolvePnPMethod"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::SolvePnPMethod }

/// Face culling settings: what faces are drawn after face culling
// TriangleCullingMode /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2923
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriangleCullingMode {
	/// all faces are drawn, no culling is actually performed
	RASTERIZE_CULLING_NONE = 0,
	/// triangles which vertices are given in clockwork order are drawn
	RASTERIZE_CULLING_CW = 1,
	/// triangles which vertices are given in counterclockwork order are drawn
	RASTERIZE_CULLING_CCW = 2,
}

impl TryFrom<i32> for TriangleCullingMode {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RASTERIZE_CULLING_NONE),
			1 => Ok(Self::RASTERIZE_CULLING_CW),
			2 => Ok(Self::RASTERIZE_CULLING_CCW),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::TriangleCullingMode"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::TriangleCullingMode }

/// GL compatibility settings
// TriangleGlCompatibleMode /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2931
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriangleGlCompatibleMode {
	/// Color and depth have their natural values and converted to internal formats if needed
	RASTERIZE_COMPAT_DISABLED = 0,
	/// Color is natural, Depth is transformed from [-zNear; -zFar] to [0; 1]
	/// by the following formula: ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7Bz%5F%7Bfar%7D%20%5Cleft%28z%20%2B%20z%5F%7Bnear%7D%5Cright%29%7D%7Bz%20%5Cleft%28z%5F%7Bfar%7D%20%2D%20z%5F%7Bnear%7D%5Cright%29%7D%20) 
	///
	/// In this mode the input/output depthBuf is considered to be in this format,
	/// therefore it's faster since there're no conversions performed
	RASTERIZE_COMPAT_INVDEPTH = 1,
}

impl TryFrom<i32> for TriangleGlCompatibleMode {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RASTERIZE_COMPAT_DISABLED),
			1 => Ok(Self::RASTERIZE_COMPAT_INVDEPTH),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::TriangleGlCompatibleMode"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::TriangleGlCompatibleMode }

/// Triangle fill settings
// TriangleShadingType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2915
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriangleShadingType {
	/// a white color is used for the whole triangle
	RASTERIZE_SHADING_WHITE = 0,
	/// a color of 1st vertex of each triangle is used
	RASTERIZE_SHADING_FLAT = 1,
	/// a color is interpolated between 3 vertices with perspective correction
	RASTERIZE_SHADING_SHADED = 2,
}

impl TryFrom<i32> for TriangleShadingType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RASTERIZE_SHADING_WHITE),
			1 => Ok(Self::RASTERIZE_SHADING_FLAT),
			2 => Ok(Self::RASTERIZE_SHADING_SHADED),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::TriangleShadingType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::TriangleShadingType }

/// cv::undistort mode
// UndistortTypes /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2140
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UndistortTypes {
	PROJ_SPHERICAL_ORTHO = 0,
	PROJ_SPHERICAL_EQRECT = 1,
}

impl TryFrom<i32> for UndistortTypes {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::PROJ_SPHERICAL_ORTHO),
			1 => Ok(Self::PROJ_SPHERICAL_EQRECT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::UndistortTypes"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::UndistortTypes }

/// Type of variables used in LevMarq solver
///
/// Variables can be linear, rotation (SO(3) group) or rigid transformation (SE(3) group) with corresponding jacobians and exponential updates.
///
/// Note: only linear variables are now supported
// VariableType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:496
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VariableType {
	LINEAR = 0,
	SO3 = 1,
	SE3 = 2,
}

impl TryFrom<i32> for VariableType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::LINEAR),
			1 => Ok(Self::SO3),
			2 => Ok(Self::SE3),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::VariableType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::VariableType }

// VolumeType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:15
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VolumeType {
	TSDF = 0,
	HashTSDF = 1,
	ColorTSDF = 2,
}

impl TryFrom<i32> for VolumeType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::TSDF),
			1 => Ok(Self::HashTSDF),
			2 => Ok(Self::ColorTSDF),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::VolumeType"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::VolumeType }

// BoundingBoxPrecision /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:139
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Volume_BoundingBoxPrecision {
	VOLUME_UNIT = 0,
	VOXEL = 1,
}

impl TryFrom<i32> for Volume_BoundingBoxPrecision {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::VOLUME_UNIT),
			1 => Ok(Self::VOXEL),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::mod_3d::Volume_BoundingBoxPrecision"))),
		}
	}
}

opencv_type_enum! { crate::mod_3d::Volume_BoundingBoxPrecision }

/// Computes an RQ decomposition of 3x3 matrices.
///
/// ## Parameters
/// * src: 3x3 input matrix.
/// * mtxR: Output 3x3 upper-triangular matrix.
/// * mtxQ: Output 3x3 orthogonal matrix.
/// * Qx: Optional output 3x3 rotation matrix around x-axis.
/// * Qy: Optional output 3x3 rotation matrix around y-axis.
/// * Qz: Optional output 3x3 rotation matrix around z-axis.
///
/// The function computes a RQ decomposition using the given rotations. This function is used in
/// [decompose_projection_matrix] to decompose the left 3x3 submatrix of a projection matrix into a camera
/// and a rotation matrix.
///
/// It optionally returns three rotation matrices, one for each axis, and the three Euler angles in
/// degrees (as the return value) that could be used in OpenGL. Note, there is always more than one
/// sequence of rotations about the three principal axes that results in the same orientation of an
/// object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned three rotation matrices and corresponding three Euler angles
/// are only one of the possible solutions.
///
/// ## Note
/// This alternative version of [rq_decomp3x3] function uses the following default values for its arguments:
/// * qx: noArray()
/// * qy: noArray()
/// * qz: noArray()
// cv::RQDecomp3x3(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:834
// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rq_decomp3x3_def(src: &impl ToInputArray, mtx_r: &mut impl ToOutputArray, mtx_q: &mut impl ToOutputArray) -> Result<core::Vec3d> {
	input_array_arg!(src);
	output_array_arg!(mtx_r);
	output_array_arg!(mtx_q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), mtx_r.as_raw__OutputArray(), mtx_q.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes an RQ decomposition of 3x3 matrices.
///
/// ## Parameters
/// * src: 3x3 input matrix.
/// * mtxR: Output 3x3 upper-triangular matrix.
/// * mtxQ: Output 3x3 orthogonal matrix.
/// * Qx: Optional output 3x3 rotation matrix around x-axis.
/// * Qy: Optional output 3x3 rotation matrix around y-axis.
/// * Qz: Optional output 3x3 rotation matrix around z-axis.
///
/// The function computes a RQ decomposition using the given rotations. This function is used in
/// [decompose_projection_matrix] to decompose the left 3x3 submatrix of a projection matrix into a camera
/// and a rotation matrix.
///
/// It optionally returns three rotation matrices, one for each axis, and the three Euler angles in
/// degrees (as the return value) that could be used in OpenGL. Note, there is always more than one
/// sequence of rotations about the three principal axes that results in the same orientation of an
/// object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned three rotation matrices and corresponding three Euler angles
/// are only one of the possible solutions.
///
/// ## C++ default parameters
/// * qx: noArray()
/// * qy: noArray()
/// * qz: noArray()
// RQDecomp3x3(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:834
// ("cv::RQDecomp3x3", vec![(pred!(mut, ["src", "mtxR", "mtxQ", "Qx", "Qy", "Qz"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rq_decomp3x3(src: &impl ToInputArray, mtx_r: &mut impl ToOutputArray, mtx_q: &mut impl ToOutputArray, qx: &mut impl ToOutputArray, qy: &mut impl ToOutputArray, qz: &mut impl ToOutputArray) -> Result<core::Vec3d> {
	input_array_arg!(src);
	output_array_arg!(mtx_r);
	output_array_arg!(mtx_q);
	output_array_arg!(qx);
	output_array_arg!(qy);
	output_array_arg!(qz);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_RQDecomp3x3_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), mtx_r.as_raw__OutputArray(), mtx_q.as_raw__OutputArray(), qx.as_raw__OutputArray(), qy.as_raw__OutputArray(), qz.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts a rotation matrix to a rotation vector or vice versa.
///
/// ## Parameters
/// * src: Input rotation vector (3x1 or 1x3) or rotation matrix (3x3).
/// * dst: Output rotation matrix (3x3) or rotation vector (3x1 or 1x3), respectively.
/// * jacobian: Optional output Jacobian matrix, 3x9 or 9x3, which is a matrix of partial
/// derivatives of the output array components with respect to the input array components.
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctheta%20%5Cleftarrow%20norm%28r%29%20%5C%5C%20r%20%20%5Cleftarrow%20r%2F%20%5Ctheta%20%5C%5C%20R%20%3D%20%20%5Ccos%28%5Ctheta%29%20I%20%2B%20%281%2D%20%5Ccos%7B%5Ctheta%7D%20%29%20r%20r%5ET%20%2B%20%20%5Csin%28%5Ctheta%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%5Cend%7Barray%7D)
///
/// Inverse transformation can be also done easily, since
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csin%20%28%20%5Ctheta%20%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cfrac%7BR%20%2D%20R%5ET%7D%7B2%7D)
///
/// A rotation vector is a convenient and most compact representation of a rotation matrix (since any
/// rotation matrix has just 3 degrees of freedom). The representation is used in the global 3D geometry
/// optimization procedures like [calibrateCamera], [stereoCalibrate], or [solvePnP] .
///
///
/// Note: More information about the computation of the derivative of a 3D rotation matrix with respect to its exponential coordinate
/// can be found in:
///    - A Compact Formula for the Derivative of a 3-D Rotation in Exponential Coordinates, Guillermo Gallego, Anthony J. Yezzi [Gallego2014ACF](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Gallego2014ACF)
///
///
/// Note: Useful information on SE(3) and Lie Groups can be found in:
///    - A tutorial on SE(3) transformation parameterizations and on-manifold optimization, Jose-Luis Blanco [blanco2010tutorial](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_blanco2010tutorial)
///    - Lie Groups for 2D and 3D Transformation, Ethan Eade [Eade17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade17)
///    - A micro Lie theory for state estimation in robotics, Joan Solà, Jérémie Deray, Dinesh Atchuthan [Sol2018AML](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Sol2018AML)
///
/// ## Note
/// This alternative version of [rodrigues] function uses the following default values for its arguments:
/// * jacobian: noArray()
// cv::Rodrigues(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:474
// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rodrigues_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_Rodrigues_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts a rotation matrix to a rotation vector or vice versa.
///
/// ## Parameters
/// * src: Input rotation vector (3x1 or 1x3) or rotation matrix (3x3).
/// * dst: Output rotation matrix (3x3) or rotation vector (3x1 or 1x3), respectively.
/// * jacobian: Optional output Jacobian matrix, 3x9 or 9x3, which is a matrix of partial
/// derivatives of the output array components with respect to the input array components.
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctheta%20%5Cleftarrow%20norm%28r%29%20%5C%5C%20r%20%20%5Cleftarrow%20r%2F%20%5Ctheta%20%5C%5C%20R%20%3D%20%20%5Ccos%28%5Ctheta%29%20I%20%2B%20%281%2D%20%5Ccos%7B%5Ctheta%7D%20%29%20r%20r%5ET%20%2B%20%20%5Csin%28%5Ctheta%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%5Cend%7Barray%7D)
///
/// Inverse transformation can be also done easily, since
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csin%20%28%20%5Ctheta%20%29%20%5Cbegin%7Bbmatrix%7D%200%20%26%20%2Dr%5Fz%20%26%20r%5Fy%5C%5C%20r%5Fz%20%26%200%20%26%20%2Dr%5Fx%5C%5C%20%2Dr%5Fy%20%26%20r%5Fx%20%26%200%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cfrac%7BR%20%2D%20R%5ET%7D%7B2%7D)
///
/// A rotation vector is a convenient and most compact representation of a rotation matrix (since any
/// rotation matrix has just 3 degrees of freedom). The representation is used in the global 3D geometry
/// optimization procedures like [calibrateCamera], [stereoCalibrate], or [solvePnP] .
///
///
/// Note: More information about the computation of the derivative of a 3D rotation matrix with respect to its exponential coordinate
/// can be found in:
///    - A Compact Formula for the Derivative of a 3-D Rotation in Exponential Coordinates, Guillermo Gallego, Anthony J. Yezzi [Gallego2014ACF](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Gallego2014ACF)
///
///
/// Note: Useful information on SE(3) and Lie Groups can be found in:
///    - A tutorial on SE(3) transformation parameterizations and on-manifold optimization, Jose-Luis Blanco [blanco2010tutorial](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_blanco2010tutorial)
///    - Lie Groups for 2D and 3D Transformation, Ethan Eade [Eade17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade17)
///    - A micro Lie theory for state estimation in robotics, Joan Solà, Jérémie Deray, Dinesh Atchuthan [Sol2018AML](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Sol2018AML)
///
/// ## C++ default parameters
/// * jacobian: noArray()
// Rodrigues(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:474
// ("cv::Rodrigues", vec![(pred!(mut, ["src", "dst", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rodrigues(src: &impl ToInputArray, dst: &mut impl ToOutputArray, jacobian: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_Rodrigues_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Combines two rotation-and-shift transformations.
///
/// ## Parameters
/// * rvec1: First rotation vector.
/// * tvec1: First translation vector.
/// * rvec2: Second rotation vector.
/// * tvec2: Second translation vector.
/// * rvec3: Output rotation vector of the superposition.
/// * tvec3: Output translation vector of the superposition.
/// * dr3dr1: Optional output derivative of rvec3 with regard to rvec1
/// * dr3dt1: Optional output derivative of rvec3 with regard to tvec1
/// * dr3dr2: Optional output derivative of rvec3 with regard to rvec2
/// * dr3dt2: Optional output derivative of rvec3 with regard to tvec2
/// * dt3dr1: Optional output derivative of tvec3 with regard to rvec1
/// * dt3dt1: Optional output derivative of tvec3 with regard to tvec1
/// * dt3dr2: Optional output derivative of tvec3 with regard to rvec2
/// * dt3dt2: Optional output derivative of tvec3 with regard to tvec2
///
/// The functions compute:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Brvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%5E%7B%2D1%7D%20%5Cleft%20%28%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec1%7D%20%29%20%5Cright%20%29%20%20%5C%5C%20%5Ctexttt%7Btvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Ctexttt%7Btvec1%7D%20%2B%20%20%5Ctexttt%7Btvec2%7D%20%5Cend%7Barray%7D%20%2C)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D) denotes a rotation vector to a rotation matrix transformation, and
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D%5E%7B%2D1%7D) denotes the inverse transformation. See [rodrigues] for details.
///
/// Also, the functions can compute the derivatives of the output vectors with regards to the input
/// vectors (see [mat_mul_deriv] ). The functions are used inside [stereo_calibrate] but can also be used in
/// your own code where Levenberg-Marquardt or another gradient-based solver is used to optimize a
/// function that contains a matrix multiplication.
///
/// ## Note
/// This alternative version of [compose_rt] function uses the following default values for its arguments:
/// * dr3dr1: noArray()
/// * dr3dt1: noArray()
/// * dr3dr2: noArray()
/// * dr3dt2: noArray()
/// * dt3dr1: noArray()
/// * dt3dt1: noArray()
/// * dt3dr2: noArray()
/// * dt3dt2: noArray()
// cv::composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:912
// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn compose_rt_def(rvec1: &impl ToInputArray, tvec1: &impl ToInputArray, rvec2: &impl ToInputArray, tvec2: &impl ToInputArray, rvec3: &mut impl ToOutputArray, tvec3: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(rvec1);
	input_array_arg!(tvec1);
	input_array_arg!(rvec2);
	input_array_arg!(tvec2);
	output_array_arg!(rvec3);
	output_array_arg!(tvec3);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(rvec1.as_raw__InputArray(), tvec1.as_raw__InputArray(), rvec2.as_raw__InputArray(), tvec2.as_raw__InputArray(), rvec3.as_raw__OutputArray(), tvec3.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Combines two rotation-and-shift transformations.
///
/// ## Parameters
/// * rvec1: First rotation vector.
/// * tvec1: First translation vector.
/// * rvec2: Second rotation vector.
/// * tvec2: Second translation vector.
/// * rvec3: Output rotation vector of the superposition.
/// * tvec3: Output translation vector of the superposition.
/// * dr3dr1: Optional output derivative of rvec3 with regard to rvec1
/// * dr3dt1: Optional output derivative of rvec3 with regard to tvec1
/// * dr3dr2: Optional output derivative of rvec3 with regard to rvec2
/// * dr3dt2: Optional output derivative of rvec3 with regard to tvec2
/// * dt3dr1: Optional output derivative of tvec3 with regard to rvec1
/// * dt3dt1: Optional output derivative of tvec3 with regard to tvec1
/// * dt3dr2: Optional output derivative of tvec3 with regard to rvec2
/// * dt3dt2: Optional output derivative of tvec3 with regard to tvec2
///
/// The functions compute:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Brvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%5E%7B%2D1%7D%20%5Cleft%20%28%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec1%7D%20%29%20%5Cright%20%29%20%20%5C%5C%20%5Ctexttt%7Btvec3%7D%20%3D%20%20%5Cmathrm%7Brodrigues%7D%20%28%20%5Ctexttt%7Brvec2%7D%20%29%20%20%5Ccdot%20%5Ctexttt%7Btvec1%7D%20%2B%20%20%5Ctexttt%7Btvec2%7D%20%5Cend%7Barray%7D%20%2C)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D) denotes a rotation vector to a rotation matrix transformation, and
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cmathrm%7Brodrigues%7D%5E%7B%2D1%7D) denotes the inverse transformation. See [rodrigues] for details.
///
/// Also, the functions can compute the derivatives of the output vectors with regards to the input
/// vectors (see [mat_mul_deriv] ). The functions are used inside [stereo_calibrate] but can also be used in
/// your own code where Levenberg-Marquardt or another gradient-based solver is used to optimize a
/// function that contains a matrix multiplication.
///
/// ## C++ default parameters
/// * dr3dr1: noArray()
/// * dr3dt1: noArray()
/// * dr3dr2: noArray()
/// * dr3dt2: noArray()
/// * dt3dr1: noArray()
/// * dt3dt1: noArray()
/// * dt3dr2: noArray()
/// * dt3dt2: noArray()
// composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:912
// ("cv::composeRT", vec![(pred!(mut, ["rvec1", "tvec1", "rvec2", "tvec2", "rvec3", "tvec3", "dr3dr1", "dr3dt1", "dr3dr2", "dr3dt2", "dt3dr1", "dt3dt1", "dt3dr2", "dt3dt2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn compose_rt(rvec1: &impl ToInputArray, tvec1: &impl ToInputArray, rvec2: &impl ToInputArray, tvec2: &impl ToInputArray, rvec3: &mut impl ToOutputArray, tvec3: &mut impl ToOutputArray, dr3dr1: &mut impl ToOutputArray, dr3dt1: &mut impl ToOutputArray, dr3dr2: &mut impl ToOutputArray, dr3dt2: &mut impl ToOutputArray, dt3dr1: &mut impl ToOutputArray, dt3dt1: &mut impl ToOutputArray, dt3dr2: &mut impl ToOutputArray, dt3dt2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(rvec1);
	input_array_arg!(tvec1);
	input_array_arg!(rvec2);
	input_array_arg!(tvec2);
	output_array_arg!(rvec3);
	output_array_arg!(tvec3);
	output_array_arg!(dr3dr1);
	output_array_arg!(dr3dt1);
	output_array_arg!(dr3dr2);
	output_array_arg!(dr3dt2);
	output_array_arg!(dt3dr1);
	output_array_arg!(dt3dt1);
	output_array_arg!(dt3dr2);
	output_array_arg!(dt3dt2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_composeRT_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(rvec1.as_raw__InputArray(), tvec1.as_raw__InputArray(), rvec2.as_raw__InputArray(), tvec2.as_raw__InputArray(), rvec3.as_raw__OutputArray(), tvec3.as_raw__OutputArray(), dr3dr1.as_raw__OutputArray(), dr3dt1.as_raw__OutputArray(), dr3dr2.as_raw__OutputArray(), dr3dt2.as_raw__OutputArray(), dt3dr1.as_raw__OutputArray(), dt3dt1.as_raw__OutputArray(), dt3dr2.as_raw__OutputArray(), dt3dt2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// For points in an image of a stereo pair, computes the corresponding epilines in the other image.
///
/// ## Parameters
/// * points: Input points. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Ctimes%201) or ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20N) matrix of type CV_32FC2 or
/// vector\<Point2f\> .
/// * whichImage: Index of the image (1 or 2) that contains the points .
/// * F: Fundamental matrix that can be estimated using [find_fundamental_mat] or [stereo_rectify] .
/// * lines: Output vector of the epipolar lines corresponding to the points in the other image.
/// Each line ![inline formula](https://latex.codecogs.com/png.latex?ax%20%2B%20by%20%2B%20c%3D0) is encoded by 3 numbers ![inline formula](https://latex.codecogs.com/png.latex?%28a%2C%20b%2C%20c%29) .
///
/// For every point in one of the two images of a stereo pair, the function finds the equation of the
/// corresponding epipolar line in the other image.
///
/// From the fundamental matrix definition (see [find_fundamental_mat] ), line ![inline formula](https://latex.codecogs.com/png.latex?l%5E%7B%282%29%7D%5Fi) in the second
/// image for the point ![inline formula](https://latex.codecogs.com/png.latex?p%5E%7B%281%29%7D%5Fi) in the first image (when whichImage=1 ) is computed as:
///
/// ![block formula](https://latex.codecogs.com/png.latex?l%5E%7B%282%29%7D%5Fi%20%3D%20F%20p%5E%7B%281%29%7D%5Fi)
///
/// And vice versa, when whichImage=2, ![inline formula](https://latex.codecogs.com/png.latex?l%5E%7B%281%29%7D%5Fi) is computed from ![inline formula](https://latex.codecogs.com/png.latex?p%5E%7B%282%29%7D%5Fi) as:
///
/// ![block formula](https://latex.codecogs.com/png.latex?l%5E%7B%281%29%7D%5Fi%20%3D%20F%5ET%20p%5E%7B%282%29%7D%5Fi)
///
/// Line coefficients are defined up to a scale. They are normalized so that ![inline formula](https://latex.codecogs.com/png.latex?a%5Fi%5E2%2Bb%5Fi%5E2%3D1) .
// computeCorrespondEpilines(InputArray, int, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1767
// ("cv::computeCorrespondEpilines", vec![(pred!(mut, ["points", "whichImage", "F", "lines"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn compute_correspond_epilines(points: &impl ToInputArray, which_image: i32, f: &impl ToInputArray, lines: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points);
	input_array_arg!(f);
	output_array_arg!(lines);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_computeCorrespondEpilines_const__InputArrayR_int_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), which_image, f.as_raw__InputArray(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts points from homogeneous to Euclidean space.
///
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N-1-dimensional points.
/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
///
/// The function converts points homogeneous to Euclidean space using perspective projection. That is,
/// each point (x1, x2, ... x(n-1), xn) is converted to (x1/xn, x2/xn, ..., x(n-1)/xn). When xn=0, the
/// output point coordinates will be (0,0,0,...).
///
/// ## Note
/// This alternative version of [convert_points_from_homogeneous] function uses the following default values for its arguments:
/// * dtype: -1
// cv::convertPointsFromHomogeneous(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1322
// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn convert_points_from_homogeneous_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts points from homogeneous to Euclidean space.
///
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N-1-dimensional points.
/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
///
/// The function converts points homogeneous to Euclidean space using perspective projection. That is,
/// each point (x1, x2, ... x(n-1), xn) is converted to (x1/xn, x2/xn, ..., x(n-1)/xn). When xn=0, the
/// output point coordinates will be (0,0,0,...).
///
/// ## C++ default parameters
/// * dtype: -1
// convertPointsFromHomogeneous(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1322
// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn convert_points_from_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts points to/from homogeneous coordinates.
///
/// ## Parameters
/// * src: Input array or vector of 2D, 3D, or 4D points.
/// * dst: Output vector of 2D, 3D, or 4D points.
///
/// The function converts 2D or 3D points from/to homogeneous coordinates by calling either
/// [convert_points_to_homogeneous] or #convertPointsFromHomogeneous.
///
///
/// Note: The function is obsolete. Use one of the previous two functions instead.
// convertPointsHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1334
// ("cv::convertPointsHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn convert_points_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts points from Euclidean to homogeneous space.
///
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N+1-dimensional points.
/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
///
/// The function converts points from Euclidean to homogeneous space by appending 1's to the tuple of
/// point coordinates. That is, each point (x1, x2, ..., xn) is converted to (x1, x2, ..., xn, 1).
///
/// ## Note
/// This alternative version of [convert_points_to_homogeneous] function uses the following default values for its arguments:
/// * dtype: -1
// cv::convertPointsToHomogeneous(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1309
// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn convert_points_to_homogeneous_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts points from Euclidean to homogeneous space.
///
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N+1-dimensional points.
/// * dtype: The desired output array depth (either CV_32F or CV_64F are currently supported).
///    If it's -1, then it's set automatically to CV_32F or CV_64F, depending on the input depth.
///
/// The function converts points from Euclidean to homogeneous space by appending 1's to the tuple of
/// point coordinates. That is, each point (x1, x2, ..., xn) is converted to (x1, x2, ..., xn, 1).
///
/// ## C++ default parameters
/// * dtype: -1
// convertPointsToHomogeneous(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1309
// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn convert_points_to_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray, dtype: i32) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dtype, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Refines coordinates of corresponding points.
///
/// ## Parameters
/// * F: 3x3 fundamental matrix.
/// * points1: 1xN array containing the first set of points.
/// * points2: 1xN array containing the second set of points.
/// * newPoints1: The optimized points1.
/// * newPoints2: The optimized points2.
///
/// The function implements the Optimal Triangulation Method (see Multiple View Geometry [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) for details).
/// For each given point correspondence points1[i] \<-\> points2[i], and a fundamental matrix F, it
/// computes the corrected correspondences newPoints1[i] \<-\> newPoints2[i] that minimize the geometric
/// error ![inline formula](https://latex.codecogs.com/png.latex?d%28points1%5Bi%5D%2C%20newPoints1%5Bi%5D%29%5E2%20%2B%20d%28points2%5Bi%5D%2CnewPoints2%5Bi%5D%29%5E2) (where ![inline formula](https://latex.codecogs.com/png.latex?d%28a%2Cb%29) is the
/// geometric distance between points ![inline formula](https://latex.codecogs.com/png.latex?a) and ![inline formula](https://latex.codecogs.com/png.latex?b) ) subject to the epipolar constraint
/// ![inline formula](https://latex.codecogs.com/png.latex?newPoints2%5ET%20%5Ccdot%20F%20%5Ccdot%20newPoints1%20%3D%200) .
// correctMatches(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1813
// ("cv::correctMatches", vec![(pred!(mut, ["F", "points1", "points2", "newPoints1", "newPoints2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn correct_matches(f: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, new_points1: &mut impl ToOutputArray, new_points2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(f);
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(new_points1);
	output_array_arg!(new_points2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_correctMatches_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(f.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), new_points1.as_raw__OutputArray(), new_points2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Decompose an essential matrix to possible rotations and translation.
///
/// ## Parameters
/// * E: The input essential matrix.
/// * R1: One possible rotation matrix.
/// * R2: Another possible rotation matrix.
/// * t: One possible translation.
///
/// This function decomposes the essential matrix E using svd decomposition [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00). In
/// general, four possible poses exist for the decomposition of E. They are ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20t%5D),
/// ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20%2Dt%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20%2Dt%5D).
///
/// If E gives the epipolar constraint ![inline formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20A%5E%7B%2DT%7D%20E%20A%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200) between the image
/// points ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) in the first image and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) in second image, then any of the tuples
/// ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20%2Dt%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20%2Dt%5D) is a change of basis from the first
/// camera's coordinate system to the second camera's coordinate system. However, by decomposing E, one
/// can only get the direction of the translation. For this reason, the translation t is returned with
/// unit length.
// decomposeEssentialMat(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1554
// ("cv::decomposeEssentialMat", vec![(pred!(mut, ["E", "R1", "R2", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn decompose_essential_mat(e: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(e);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decomposeEssentialMat_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Decompose a homography matrix to rotation(s), translation(s) and plane normal(s).
///
/// ## Parameters
/// * H: The input homography matrix between two images.
/// * K: The input camera intrinsic matrix.
/// * rotations: Array of rotation matrices.
/// * translations: Array of translation matrices.
/// * normals: Array of plane normal matrices.
///
/// This function extracts relative camera motion between two views of a planar object and returns up to
/// four mathematical solution tuples of rotation, translation, and plane normal. The decomposition of
/// the homography matrix H is described in detail in [Malis2007](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Malis2007).
///
/// If the homography H, induced by the plane, gives the constraint
/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D) on the source image points
/// ![inline formula](https://latex.codecogs.com/png.latex?p%5Fi) and the destination image points ![inline formula](https://latex.codecogs.com/png.latex?p%27%5Fi), then the tuple of rotations[k] and
/// translations[k] is a change of basis from the source camera's coordinate system to the destination
/// camera's coordinate system. However, by decomposing H, one can only get the translation normalized
/// by the (typically unknown) depth of the scene, i.e. its direction but with normalized length.
///
/// If point correspondences are available, at least two solutions may further be invalidated, by
/// applying positive depth constraint, i.e. all points must be in front of the camera.
// decomposeHomographyMat(InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2108
// ("cv::decomposeHomographyMat", vec![(pred!(mut, ["H", "K", "rotations", "translations", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn decompose_homography_mat(h: &impl ToInputArray, k: &impl ToInputArray, rotations: &mut impl ToOutputArray, translations: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(h);
	input_array_arg!(k);
	output_array_arg!(rotations);
	output_array_arg!(translations);
	output_array_arg!(normals);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decomposeHomographyMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(h.as_raw__InputArray(), k.as_raw__InputArray(), rotations.as_raw__OutputArray(), translations.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Decomposes a projection matrix into a rotation matrix and a camera intrinsic matrix.
///
/// ## Parameters
/// * projMatrix: 3x4 input projection matrix P.
/// * cameraMatrix: Output 3x3 camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D).
/// * rotMatrix: Output 3x3 external rotation matrix R.
/// * transVect: Output 4x1 translation vector T.
/// * rotMatrixX: Optional 3x3 rotation matrix around x-axis.
/// * rotMatrixY: Optional 3x3 rotation matrix around y-axis.
/// * rotMatrixZ: Optional 3x3 rotation matrix around z-axis.
/// * eulerAngles: Optional three-element vector containing three Euler angles of rotation in
/// degrees.
///
/// The function computes a decomposition of a projection matrix into a calibration and a rotation
/// matrix and the position of a camera.
///
/// It optionally returns three rotation matrices, one for each axis, and three Euler angles that could
/// be used in OpenGL. Note, there is always more than one sequence of rotations about the three
/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
/// three rotation matrices and corresponding three Euler angles are only one of the possible solutions.
///
/// The function is based on [rq_decomp3x3] .
///
/// ## Note
/// This alternative version of [decompose_projection_matrix] function uses the following default values for its arguments:
/// * rot_matrix_x: noArray()
/// * rot_matrix_y: noArray()
/// * rot_matrix_z: noArray()
/// * euler_angles: noArray()
// cv::decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:861
// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn decompose_projection_matrix_def(proj_matrix: &impl ToInputArray, camera_matrix: &mut impl ToOutputArray, rot_matrix: &mut impl ToOutputArray, trans_vect: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(proj_matrix);
	output_array_arg!(camera_matrix);
	output_array_arg!(rot_matrix);
	output_array_arg!(trans_vect);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(proj_matrix.as_raw__InputArray(), camera_matrix.as_raw__OutputArray(), rot_matrix.as_raw__OutputArray(), trans_vect.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Decomposes a projection matrix into a rotation matrix and a camera intrinsic matrix.
///
/// ## Parameters
/// * projMatrix: 3x4 input projection matrix P.
/// * cameraMatrix: Output 3x3 camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D).
/// * rotMatrix: Output 3x3 external rotation matrix R.
/// * transVect: Output 4x1 translation vector T.
/// * rotMatrixX: Optional 3x3 rotation matrix around x-axis.
/// * rotMatrixY: Optional 3x3 rotation matrix around y-axis.
/// * rotMatrixZ: Optional 3x3 rotation matrix around z-axis.
/// * eulerAngles: Optional three-element vector containing three Euler angles of rotation in
/// degrees.
///
/// The function computes a decomposition of a projection matrix into a calibration and a rotation
/// matrix and the position of a camera.
///
/// It optionally returns three rotation matrices, one for each axis, and three Euler angles that could
/// be used in OpenGL. Note, there is always more than one sequence of rotations about the three
/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
/// three rotation matrices and corresponding three Euler angles are only one of the possible solutions.
///
/// The function is based on [rq_decomp3x3] .
///
/// ## C++ default parameters
/// * rot_matrix_x: noArray()
/// * rot_matrix_y: noArray()
/// * rot_matrix_z: noArray()
/// * euler_angles: noArray()
// decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:861
// ("cv::decomposeProjectionMatrix", vec![(pred!(mut, ["projMatrix", "cameraMatrix", "rotMatrix", "transVect", "rotMatrixX", "rotMatrixY", "rotMatrixZ", "eulerAngles"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn decompose_projection_matrix(proj_matrix: &impl ToInputArray, camera_matrix: &mut impl ToOutputArray, rot_matrix: &mut impl ToOutputArray, trans_vect: &mut impl ToOutputArray, rot_matrix_x: &mut impl ToOutputArray, rot_matrix_y: &mut impl ToOutputArray, rot_matrix_z: &mut impl ToOutputArray, euler_angles: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(proj_matrix);
	output_array_arg!(camera_matrix);
	output_array_arg!(rot_matrix);
	output_array_arg!(trans_vect);
	output_array_arg!(rot_matrix_x);
	output_array_arg!(rot_matrix_y);
	output_array_arg!(rot_matrix_z);
	output_array_arg!(euler_angles);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decomposeProjectionMatrix_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(proj_matrix.as_raw__InputArray(), camera_matrix.as_raw__OutputArray(), rot_matrix.as_raw__OutputArray(), trans_vect.as_raw__OutputArray(), rot_matrix_x.as_raw__OutputArray(), rot_matrix_y.as_raw__OutputArray(), rot_matrix_z.as_raw__OutputArray(), euler_angles.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Parameters
/// * depth: the depth image
/// * in_K: 
/// * in_points: the list of xy coordinates
/// * points3d: the resulting 3d points (point is represented by 4 chanels value [x, y, z, 0])
// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:105
// ("cv::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn depth_to3d_sparse(depth: &impl ToInputArray, in_k: &impl ToInputArray, in_points: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(in_k);
	input_array_arg!(in_points);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), in_k.as_raw__InputArray(), in_points.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts a depth image to 3d points. If the mask is empty then the resulting array has the same dimensions as `depth`,
/// otherwise it is 1d vector containing mask-enabled values only.
/// The coordinate system is x pointing left, y down and z away from the camera
/// ## Parameters
/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
/// * K: The calibration matrix
/// * points3d: the resulting 3d points (point is represented by 4 channels value [x, y, z, 0]). They are of the same depth as `depth` if it is CV_32F or CV_64F, and the
///        depth of `K` if `depth` is of depth CV_16U or CV_16S
/// * mask: the mask of the points to consider (can be empty)
///
/// ## Note
/// This alternative version of [depth_to3d] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:117
// ("cv::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn depth_to3d_def(depth: &impl ToInputArray, k: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(k);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts a depth image to 3d points. If the mask is empty then the resulting array has the same dimensions as `depth`,
/// otherwise it is 1d vector containing mask-enabled values only.
/// The coordinate system is x pointing left, y down and z away from the camera
/// ## Parameters
/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
/// * K: The calibration matrix
/// * points3d: the resulting 3d points (point is represented by 4 channels value [x, y, z, 0]). They are of the same depth as `depth` if it is CV_32F or CV_64F, and the
///        depth of `K` if `depth` is of depth CV_16U or CV_16S
/// * mask: the mask of the points to consider (can be empty)
///
/// ## C++ default parameters
/// * mask: noArray()
// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:117
// ("cv::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn depth_to3d(depth: &impl ToInputArray, k: &impl ToInputArray, points3d: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(k);
	output_array_arg!(points3d);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Draw axes of the world/object coordinate system from pose estimation. see also: solvePnP
///
/// ## Parameters
/// * image: Input/output image. It must have 1 or 3 channels. The number of channels is not altered.
/// * cameraMatrix: Input 3x3 floating-point matrix of camera intrinsic parameters.
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D)
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is empty, the zero distortion coefficients are assumed.
/// * rvec: Rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Translation vector.
/// * length: Length of the painted axes in the same unit than tvec (usually in meters).
/// * thickness: Line thickness of the painted axes.
///
/// This function draws the axes of the world/object coordinate system w.r.t. to the camera frame.
/// OX is drawn in red, OY in green and OZ in blue.
///
/// ## Note
/// This alternative version of [draw_frame_axes] function uses the following default values for its arguments:
/// * thickness: 3
// cv::drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1296
// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float"]), _)]),
#[inline]
pub fn draw_frame_axes_def(image: &mut impl ToInputOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, length: f32) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float(image.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), length, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Draw axes of the world/object coordinate system from pose estimation. see also: solvePnP
///
/// ## Parameters
/// * image: Input/output image. It must have 1 or 3 channels. The number of channels is not altered.
/// * cameraMatrix: Input 3x3 floating-point matrix of camera intrinsic parameters.
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D)
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is empty, the zero distortion coefficients are assumed.
/// * rvec: Rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Translation vector.
/// * length: Length of the painted axes in the same unit than tvec (usually in meters).
/// * thickness: Line thickness of the painted axes.
///
/// This function draws the axes of the world/object coordinate system w.r.t. to the camera frame.
/// OX is drawn in red, OY in green and OZ in blue.
///
/// ## C++ default parameters
/// * thickness: 3
// drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, float, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1296
// ("cv::drawFrameAxes", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "length", "thickness"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "int"]), _)]),
#[inline]
pub fn draw_frame_axes(image: &mut impl ToInputOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, length: f32, thickness: i32) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawFrameAxes_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_int(image.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), length, thickness, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes an optimal affine transformation between two 2D point sets.
///
/// It computes
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Parameters
/// * from: First input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%29).
/// * to: Second input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29).
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * method: Robust method used to compute transformation. The following methods are possible:
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// RANSAC is the default method.
/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
/// a point as an inlier. Applies only to RANSAC.
/// * maxIters: The maximum number of robust method iterations.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
/// Passing 0 will disable refining, so the output matrix will be output of robust method.
///
/// ## Returns
/// Output 2D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or empty matrix if transformation
/// could not be estimated. The returned matrix has the following form:
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20b%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// The function estimates an optimal 2D affine transformation between two 2D point sets using the
/// selected robust algorithm.
///
/// The computed transformation is then refined further (using only inliers) with the
/// Levenberg-Marquardt method to reduce the re-projection error even more.
///
///
/// Note:
/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers.
/// ## See also
/// estimateAffinePartial2D, getAffineTransform
///
/// ## Note
/// This alternative version of [estimate_affine_2d] function uses the following default values for its arguments:
/// * inliers: noArray()
/// * method: RANSAC
/// * ransac_reproj_threshold: 3
/// * max_iters: 2000
/// * confidence: 0.99
/// * refine_iters: 10
// cv::estimateAffine2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2023
// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn estimate_affine_2d_def(from: &impl ToInputArray, to: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(from);
	input_array_arg!(to);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine2D_const__InputArrayR_const__InputArrayR(from.as_raw__InputArray(), to.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// estimateAffine2D(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2029
// ("cv::estimateAffine2D", vec![(pred!(mut, ["pts1", "pts2", "inliers", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
#[inline]
pub fn estimate_affine_2d_1(pts1: &impl ToInputArray, pts2: &impl ToInputArray, inliers: &mut impl ToOutputArray, params: crate::mod_3d::UsacParams) -> Result<core::Mat> {
	input_array_arg!(pts1);
	input_array_arg!(pts2);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(pts1.as_raw__InputArray(), pts2.as_raw__InputArray(), inliers.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal affine transformation between two 2D point sets.
///
/// It computes
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Parameters
/// * from: First input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%29).
/// * to: Second input 2D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29).
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * method: Robust method used to compute transformation. The following methods are possible:
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// RANSAC is the default method.
/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
/// a point as an inlier. Applies only to RANSAC.
/// * maxIters: The maximum number of robust method iterations.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
/// Passing 0 will disable refining, so the output matrix will be output of robust method.
///
/// ## Returns
/// Output 2D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or empty matrix if transformation
/// could not be estimated. The returned matrix has the following form:
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20b%5F2%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// The function estimates an optimal 2D affine transformation between two 2D point sets using the
/// selected robust algorithm.
///
/// The computed transformation is then refined further (using only inliers) with the
/// Levenberg-Marquardt method to reduce the re-projection error even more.
///
///
/// Note:
/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers.
/// ## See also
/// estimateAffinePartial2D, getAffineTransform
///
/// ## C++ default parameters
/// * inliers: noArray()
/// * method: RANSAC
/// * ransac_reproj_threshold: 3
/// * max_iters: 2000
/// * confidence: 0.99
/// * refine_iters: 10
// estimateAffine2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2023
// ("cv::estimateAffine2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
#[inline]
pub fn estimate_affine_2d(from: &impl ToInputArray, to: &impl ToInputArray, inliers: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Mat> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from.as_raw__InputArray(), to.as_raw__InputArray(), inliers.as_raw__OutputArray(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal affine transformation between two 3D point sets.
///
/// It computes ![inline formula](https://latex.codecogs.com/png.latex?R%2Cs%2Ct) minimizing ![inline formula](https://latex.codecogs.com/png.latex?%5Csum%7Bi%7D%20dst%5Fi%20%2D%20c%20%5Ccdot%20R%20%5Ccdot%20src%5Fi%20)
/// where ![inline formula](https://latex.codecogs.com/png.latex?R) is a 3x3 rotation matrix, ![inline formula](https://latex.codecogs.com/png.latex?t) is a 3x1 translation vector and ![inline formula](https://latex.codecogs.com/png.latex?s) is a
/// scalar size value. This is an implementation of the algorithm by Umeyama \cite umeyama1991least .
/// The estimated affine transform has a homogeneous scale which is a subclass of affine
/// transformations with 7 degrees of freedom. The paired point sets need to comprise at least 3
/// points each.
///
/// ## Parameters
/// * src: First input 3D point set.
/// * dst: Second input 3D point set.
/// * scale: If null is passed, the scale parameter c will be assumed to be 1.0.
/// Else the pointed-to variable will be set to the optimal scale.
/// * force_rotation: If true, the returned rotation will never be a reflection.
/// This might be unwanted, e.g. when optimizing a transform between a right- and a
/// left-handed coordinate system.
/// ## Returns
/// 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
/// ![block formula](https://latex.codecogs.com/png.latex?T%20%3D%0A%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Note
/// This alternative version of [estimate_affine_3d_1] function uses the following default values for its arguments:
/// * scale: nullptr
/// * force_rotation: true
// cv::estimateAffine3D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1911
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn estimate_affine_3d_1_def(src: &impl ToInputArray, dst: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal affine transformation between two 3D point sets.
///
/// It computes
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Parameters
/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
/// * out: Output 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%20%26%20b%5F2%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%20%26%20b%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
/// an inlier.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
///
/// The function estimates an optimal 3D affine transformation between two 3D point sets using the
/// RANSAC algorithm.
///
/// ## Note
/// This alternative version of [estimate_affine_3d] function uses the following default values for its arguments:
/// * ransac_threshold: 3
/// * confidence: 0.99
// cv::estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1883
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn estimate_affine_3d_def(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(src);
	input_array_arg!(dst);
	output_array_arg!(out);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes an optimal affine transformation between two 3D point sets.
///
/// It computes
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Parameters
/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
/// * out: Output 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Aa%5F%7B11%7D%20%26%20a%5F%7B12%7D%20%26%20a%5F%7B13%7D%20%26%20b%5F1%5C%5C%0Aa%5F%7B21%7D%20%26%20a%5F%7B22%7D%20%26%20a%5F%7B23%7D%20%26%20b%5F2%5C%5C%0Aa%5F%7B31%7D%20%26%20a%5F%7B32%7D%20%26%20a%5F%7B33%7D%20%26%20b%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
/// an inlier.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
///
/// The function estimates an optimal 3D affine transformation between two 3D point sets using the
/// RANSAC algorithm.
///
/// ## C++ default parameters
/// * ransac_threshold: 3
/// * confidence: 0.99
// estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1883
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
#[inline]
pub fn estimate_affine_3d(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, ransac_threshold: f64, confidence: f64) -> Result<i32> {
	input_array_arg!(src);
	input_array_arg!(dst);
	output_array_arg!(out);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ransac_threshold, confidence, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes an optimal affine transformation between two 3D point sets.
///
/// It computes ![inline formula](https://latex.codecogs.com/png.latex?R%2Cs%2Ct) minimizing ![inline formula](https://latex.codecogs.com/png.latex?%5Csum%7Bi%7D%20dst%5Fi%20%2D%20c%20%5Ccdot%20R%20%5Ccdot%20src%5Fi%20)
/// where ![inline formula](https://latex.codecogs.com/png.latex?R) is a 3x3 rotation matrix, ![inline formula](https://latex.codecogs.com/png.latex?t) is a 3x1 translation vector and ![inline formula](https://latex.codecogs.com/png.latex?s) is a
/// scalar size value. This is an implementation of the algorithm by Umeyama \cite umeyama1991least .
/// The estimated affine transform has a homogeneous scale which is a subclass of affine
/// transformations with 7 degrees of freedom. The paired point sets need to comprise at least 3
/// points each.
///
/// ## Parameters
/// * src: First input 3D point set.
/// * dst: Second input 3D point set.
/// * scale: If null is passed, the scale parameter c will be assumed to be 1.0.
/// Else the pointed-to variable will be set to the optimal scale.
/// * force_rotation: If true, the returned rotation will never be a reflection.
/// This might be unwanted, e.g. when optimizing a transform between a right- and a
/// left-handed coordinate system.
/// ## Returns
/// 3D affine transformation matrix ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%204) of the form
/// ![block formula](https://latex.codecogs.com/png.latex?T%20%3D%0A%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## C++ default parameters
/// * scale: nullptr
/// * force_rotation: true
// estimateAffine3D(InputArray, InputArray, double *, bool)(InputArray, InputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1911
// ("cv::estimateAffine3D", vec![(pred!(mut, ["src", "dst", "scale", "force_rotation"], ["const cv::_InputArray*", "const cv::_InputArray*", "double*", "bool"]), _)]),
#[inline]
pub fn estimate_affine_3d_1(src: &impl ToInputArray, dst: &impl ToInputArray, scale: &mut f64, force_rotation: bool) -> Result<core::Mat> {
	input_array_arg!(src);
	input_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffine3D_const__InputArrayR_const__InputArrayR_doubleX_bool(src.as_raw__InputArray(), dst.as_raw__InputArray(), scale, force_rotation, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal limited affine transformation with 4 degrees of freedom between
/// two 2D point sets.
///
/// ## Parameters
/// * from: First input 2D point set.
/// * to: Second input 2D point set.
/// * inliers: Output vector indicating which points are inliers.
/// * method: Robust method used to compute transformation. The following methods are possible:
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// RANSAC is the default method.
/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
/// a point as an inlier. Applies only to RANSAC.
/// * maxIters: The maximum number of robust method iterations.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
/// Passing 0 will disable refining, so the output matrix will be output of robust method.
///
/// ## Returns
/// Output 2D affine transformation (4 degrees of freedom) matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or
/// empty matrix if transformation could not be estimated.
///
/// The function estimates an optimal 2D affine transformation with 4 degrees of freedom limited to
/// combinations of translation, rotation, and uniform scaling. Uses the selected algorithm for robust
/// estimation.
///
/// The computed transformation is then refined further (using only inliers) with the
/// Levenberg-Marquardt method to reduce the re-projection error even more.
///
/// Estimated transformation matrix is:
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%2D%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fy%0A%5Cend%7Bbmatrix%7D%20)
/// Where ![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctheta%20) is the rotation angle, ![inline formula](https://latex.codecogs.com/png.latex?%20s%20) the scaling factor and ![inline formula](https://latex.codecogs.com/png.latex?%20t%5Fx%2C%20t%5Fy%20) are
/// translations in ![inline formula](https://latex.codecogs.com/png.latex?%20x%2C%20y%20) axes respectively.
///
///
/// Note:
/// The RANSAC method can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers.
/// ## See also
/// estimateAffine2D, getAffineTransform
///
/// ## Note
/// This alternative version of [estimate_affine_partial_2d] function uses the following default values for its arguments:
/// * inliers: noArray()
/// * method: RANSAC
/// * ransac_reproj_threshold: 3
/// * max_iters: 2000
/// * confidence: 0.99
/// * refine_iters: 10
// cv::estimateAffinePartial2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2075
// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn estimate_affine_partial_2d_def(from: &impl ToInputArray, to: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(from);
	input_array_arg!(to);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR(from.as_raw__InputArray(), to.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal limited affine transformation with 4 degrees of freedom between
/// two 2D point sets.
///
/// ## Parameters
/// * from: First input 2D point set.
/// * to: Second input 2D point set.
/// * inliers: Output vector indicating which points are inliers.
/// * method: Robust method used to compute transformation. The following methods are possible:
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// RANSAC is the default method.
/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
/// a point as an inlier. Applies only to RANSAC.
/// * maxIters: The maximum number of robust method iterations.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
/// Passing 0 will disable refining, so the output matrix will be output of robust method.
///
/// ## Returns
/// Output 2D affine transformation (4 degrees of freedom) matrix ![inline formula](https://latex.codecogs.com/png.latex?2%20%5Ctimes%203) or
/// empty matrix if transformation could not be estimated.
///
/// The function estimates an optimal 2D affine transformation with 4 degrees of freedom limited to
/// combinations of translation, rotation, and uniform scaling. Uses the selected algorithm for robust
/// estimation.
///
/// The computed transformation is then refined further (using only inliers) with the
/// Levenberg-Marquardt method to reduce the re-projection error even more.
///
/// Estimated transformation matrix is:
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%2D%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Csin%28%5Ctheta%29%20%5Ccdot%20s%20%26%20%5Ccos%28%5Ctheta%29%20%5Ccdot%20s%20%26%20t%5Fy%0A%5Cend%7Bbmatrix%7D%20)
/// Where ![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctheta%20) is the rotation angle, ![inline formula](https://latex.codecogs.com/png.latex?%20s%20) the scaling factor and ![inline formula](https://latex.codecogs.com/png.latex?%20t%5Fx%2C%20t%5Fy%20) are
/// translations in ![inline formula](https://latex.codecogs.com/png.latex?%20x%2C%20y%20) axes respectively.
///
///
/// Note:
/// The RANSAC method can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers.
/// ## See also
/// estimateAffine2D, getAffineTransform
///
/// ## C++ default parameters
/// * inliers: noArray()
/// * method: RANSAC
/// * ransac_reproj_threshold: 3
/// * max_iters: 2000
/// * confidence: 0.99
/// * refine_iters: 10
// estimateAffinePartial2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2075
// ("cv::estimateAffinePartial2D", vec![(pred!(mut, ["from", "to", "inliers", "method", "ransacReprojThreshold", "maxIters", "confidence", "refineIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "size_t", "double", "size_t"]), _)]),
#[inline]
pub fn estimate_affine_partial_2d(from: &impl ToInputArray, to: &impl ToInputArray, inliers: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Mat> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateAffinePartial2D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_size_t_double_size_t(from.as_raw__InputArray(), to.as_raw__InputArray(), inliers.as_raw__OutputArray(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Computes an optimal translation between two 3D point sets.
///
/// It computes
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Parameters
/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
/// * out: Output 3D translation vector ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) of the form
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%20%5C%5C%0Ab%5F2%20%5C%5C%0Ab%5F3%20%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
/// an inlier.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
///
/// The function estimates an optimal 3D translation between two 3D point sets using the
/// RANSAC algorithm.
///
/// ## Note
/// This alternative version of [estimate_translation_3d] function uses the following default values for its arguments:
/// * ransac_threshold: 3
/// * confidence: 0.99
// cv::estimateTranslation3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1957
// ("cv::estimateTranslation3D", vec![(pred!(mut, ["src", "dst", "out", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn estimate_translation_3d_def(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(src);
	input_array_arg!(dst);
	output_array_arg!(out);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes an optimal translation between two 3D point sets.
///
/// It computes
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ax%5C%5C%0Ay%5C%5C%0Az%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%3D%0A%5Cbegin%7Bbmatrix%7D%0AX%5C%5C%0AY%5C%5C%0AZ%5C%5C%0A%5Cend%7Bbmatrix%7D%0A%2B%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%5C%5C%0Ab%5F2%5C%5C%0Ab%5F3%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
///
/// ## Parameters
/// * src: First input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28X%2CY%2CZ%29).
/// * dst: Second input 3D point set containing ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%2Cz%29).
/// * out: Output 3D translation vector ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) of the form
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0Ab%5F1%20%5C%5C%0Ab%5F2%20%5C%5C%0Ab%5F3%20%5C%5C%0A%5Cend%7Bbmatrix%7D%0A)
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
/// an inlier.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
///
/// The function estimates an optimal 3D translation between two 3D point sets using the
/// RANSAC algorithm.
///
/// ## C++ default parameters
/// * ransac_threshold: 3
/// * confidence: 0.99
// estimateTranslation3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1957
// ("cv::estimateTranslation3D", vec![(pred!(mut, ["src", "dst", "out", "inliers", "ransacThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
#[inline]
pub fn estimate_translation_3d(src: &impl ToInputArray, dst: &impl ToInputArray, out: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, ransac_threshold: f64, confidence: f64) -> Result<i32> {
	input_array_arg!(src);
	input_array_arg!(dst);
	output_array_arg!(out);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_estimateTranslation3D_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(src.as_raw__InputArray(), dst.as_raw__InputArray(), out.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ransac_threshold, confidence, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Parameters
/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
///                      that is, sampled size = original size * sampled_scale.
/// * dist_lower_limit: Sampling is terminated early if the distance from
///                  the farthest point to S is less than dist_lower_limit, default 0.
/// * rng: Optional random number generator used for selecting seed point for FPS;
///                  if it is nullptr, theRNG () is used instead.
/// ## Returns
/// The number of points actually sampled.
///
/// ## Note
/// This alternative version of [farthest_point_sampling_1] function uses the following default values for its arguments:
/// * dist_lower_limit: 0
/// * rng: nullptr
// cv::farthestPointSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:259
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_scale"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float"]), _)]),
#[inline]
pub fn farthest_point_sampling_1_def(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32) -> Result<i32> {
	output_array_arg!(sampled_point_flags);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by Farthest Point Sampling(FPS).
///
/// FPS Algorithm:
/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
/// + Initialize: Set sampled point cloud S to the empty set
/// + Step:
///    1. Randomly take a seed point from C and take it from C to S;
///    2. Find a point in C that is the farthest away from S and take it from C to S;
///       (The distance from point to set S is the smallest distance from point to all points in S)
///    3. Repeat *step 2* until the farthest distance of the point in C from S
///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
/// + Output: Sampled point cloud S
///
/// ## Parameters
/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_pts_size: The desired point cloud size after sampling.
/// * dist_lower_limit: Sampling is terminated early if the distance from
///                  the farthest point to S is less than dist_lower_limit, default 0.
/// * rng: Optional random number generator used for selecting seed point for FPS;
///                  if it is nullptr, theRNG () is used instead.
/// ## Returns
/// The number of points actually sampled.
///
/// ## Overloaded parameters
///
///
/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
///                      that is, sampled size = original size * sampled_scale.
/// * dist_lower_limit: Sampling is terminated early if the distance from
///                  the farthest point to S is less than dist_lower_limit, default 0.
/// * rng: Optional random number generator used for selecting seed point for FPS;
///                  if it is nullptr, theRNG () is used instead.
/// ## Returns
/// The number of points actually sampled.
///
/// ## C++ default parameters
/// * dist_lower_limit: 0
/// * rng: nullptr
// farthestPointSampling(OutputArray, InputArray, float, float, RNG *)(OutputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:259
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_scale", "dist_lower_limit", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "float", "cv::RNG*"]), _)]),
#[inline]
pub fn farthest_point_sampling_1(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32, dist_lower_limit: f32, rng: &mut impl core::RNGTrait) -> Result<i32> {
	output_array_arg!(sampled_point_flags);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_float_float_RNGX(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, dist_lower_limit, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by Farthest Point Sampling(FPS).
///
/// FPS Algorithm:
/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
/// + Initialize: Set sampled point cloud S to the empty set
/// + Step:
///    1. Randomly take a seed point from C and take it from C to S;
///    2. Find a point in C that is the farthest away from S and take it from C to S;
///       (The distance from point to set S is the smallest distance from point to all points in S)
///    3. Repeat *step 2* until the farthest distance of the point in C from S
///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
/// + Output: Sampled point cloud S
///
/// ## Parameters
/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_pts_size: The desired point cloud size after sampling.
/// * dist_lower_limit: Sampling is terminated early if the distance from
///                  the farthest point to S is less than dist_lower_limit, default 0.
/// * rng: Optional random number generator used for selecting seed point for FPS;
///                  if it is nullptr, theRNG () is used instead.
/// ## Returns
/// The number of points actually sampled.
///
/// ## Note
/// This alternative version of [farthest_point_sampling] function uses the following default values for its arguments:
/// * dist_lower_limit: 0
/// * rng: nullptr
// cv::farthestPointSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:242
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_pts_size"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn farthest_point_sampling_def(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32) -> Result<i32> {
	output_array_arg!(sampled_point_flags);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by Farthest Point Sampling(FPS).
///
/// FPS Algorithm:
/// + Input: Point cloud *C*, *sampled_pts_size*, *dist_lower_limit*
/// + Initialize: Set sampled point cloud S to the empty set
/// + Step:
///    1. Randomly take a seed point from C and take it from C to S;
///    2. Find a point in C that is the farthest away from S and take it from C to S;
///       (The distance from point to set S is the smallest distance from point to all points in S)
///    3. Repeat *step 2* until the farthest distance of the point in C from S
///       is less than *dist_lower_limit*, or the size of S is equal to *sampled_pts_size*.
/// + Output: Sampled point cloud S
///
/// ## Parameters
/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_pts_size: The desired point cloud size after sampling.
/// * dist_lower_limit: Sampling is terminated early if the distance from
///                  the farthest point to S is less than dist_lower_limit, default 0.
/// * rng: Optional random number generator used for selecting seed point for FPS;
///                  if it is nullptr, theRNG () is used instead.
/// ## Returns
/// The number of points actually sampled.
///
/// ## C++ default parameters
/// * dist_lower_limit: 0
/// * rng: nullptr
// farthestPointSampling(OutputArray, InputArray, int, float, RNG *)(OutputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:242
// ("cv::farthestPointSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "sampled_pts_size", "dist_lower_limit", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int", "float", "cv::RNG*"]), _)]),
#[inline]
pub fn farthest_point_sampling(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32, dist_lower_limit: f32, rng: &mut impl core::RNGTrait) -> Result<i32> {
	output_array_arg!(sampled_point_flags);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_farthestPointSampling_const__OutputArrayR_const__InputArrayR_int_float_RNGX(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, dist_lower_limit, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Filters homography decompositions based on additional information.
///
/// ## Parameters
/// * rotations: Vector of rotation matrices.
/// * normals: Vector of plane normal matrices.
/// * beforePoints: Vector of (rectified) visible reference points before the homography is applied
/// * afterPoints: Vector of (rectified) visible reference points after the homography is applied
/// * possibleSolutions: Vector of int indices representing the viable solution set after filtering
/// * pointsMask: optional Mat/Vector of 8u type representing the mask for the inliers as given by the [find_homography] function
///
/// This function is intended to filter the output of the [decompose_homography_mat] based on additional
/// information as described in [Malis2007](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Malis2007) . The summary of the method: the [decompose_homography_mat] function
/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
/// sets of points visible in the camera frame before and after the homography transformation is applied,
/// we can determine which are the true potential solutions and which are the opposites by verifying which
/// homographies are consistent with all visible reference points being in front of the camera. The inputs
/// are left unchanged; the filtered solution set is returned as indices into the existing one.
///
/// ## Note
/// This alternative version of [filter_homography_decomp_by_visible_refpoints] function uses the following default values for its arguments:
/// * points_mask: noArray()
// cv::filterHomographyDecompByVisibleRefpoints(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2132
// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn filter_homography_decomp_by_visible_refpoints_def(rotations: &impl ToInputArray, normals: &impl ToInputArray, before_points: &impl ToInputArray, after_points: &impl ToInputArray, possible_solutions: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(rotations);
	input_array_arg!(normals);
	input_array_arg!(before_points);
	input_array_arg!(after_points);
	output_array_arg!(possible_solutions);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(rotations.as_raw__InputArray(), normals.as_raw__InputArray(), before_points.as_raw__InputArray(), after_points.as_raw__InputArray(), possible_solutions.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Filters homography decompositions based on additional information.
///
/// ## Parameters
/// * rotations: Vector of rotation matrices.
/// * normals: Vector of plane normal matrices.
/// * beforePoints: Vector of (rectified) visible reference points before the homography is applied
/// * afterPoints: Vector of (rectified) visible reference points after the homography is applied
/// * possibleSolutions: Vector of int indices representing the viable solution set after filtering
/// * pointsMask: optional Mat/Vector of 8u type representing the mask for the inliers as given by the [find_homography] function
///
/// This function is intended to filter the output of the [decompose_homography_mat] based on additional
/// information as described in [Malis2007](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Malis2007) . The summary of the method: the [decompose_homography_mat] function
/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
/// sets of points visible in the camera frame before and after the homography transformation is applied,
/// we can determine which are the true potential solutions and which are the opposites by verifying which
/// homographies are consistent with all visible reference points being in front of the camera. The inputs
/// are left unchanged; the filtered solution set is returned as indices into the existing one.
///
/// ## C++ default parameters
/// * points_mask: noArray()
// filterHomographyDecompByVisibleRefpoints(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2132
// ("cv::filterHomographyDecompByVisibleRefpoints", vec![(pred!(mut, ["rotations", "normals", "beforePoints", "afterPoints", "possibleSolutions", "pointsMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn filter_homography_decomp_by_visible_refpoints(rotations: &impl ToInputArray, normals: &impl ToInputArray, before_points: &impl ToInputArray, after_points: &impl ToInputArray, possible_solutions: &mut impl ToOutputArray, points_mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(rotations);
	input_array_arg!(normals);
	input_array_arg!(before_points);
	input_array_arg!(after_points);
	output_array_arg!(possible_solutions);
	input_array_arg!(points_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_filterHomographyDecompByVisibleRefpoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(rotations.as_raw__InputArray(), normals.as_raw__InputArray(), before_points.as_raw__InputArray(), after_points.as_raw__InputArray(), possible_solutions.as_raw__OutputArray(), points_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * focal: focal length of the camera. Note that this function assumes that points1 and points2
/// are feature points from cameras with same focal length and principal point.
/// * pp: principal point of the camera.
/// * method: Method for computing a fundamental matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// * maxIters: The maximum number of robust method iterations.
///
/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
/// principal point:
///
/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
///
/// ## Note
/// This alternative version of [find_essential_mat_1] function uses the following default values for its arguments:
/// * focal: 1.0
/// * pp: Point2d(0,0)
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * max_iters: 1000
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1481
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_1_def(points1: &impl ToInputArray, points2: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Calculates an essential matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
/// When passing these coordinates, pass the identity matrix for this parameter.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// * maxIters: The maximum number of robust method iterations.
///
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## Note
/// This alternative version of [find_essential_mat] function uses the following default values for its arguments:
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * max_iters: 1000
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1443
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Calculates an essential matrix from the corresponding points in two images from potentially two different cameras.
///
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix1: Camera matrix for the first camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * cameraMatrix2: Camera matrix for the second camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs1: Input vector of distortion coefficients for the first camera
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * distCoeffs2: Input vector of distortion coefficients for the second camera
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
///
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or  [recover_pose] to recover the relative pose between cameras.
///
/// ## Note
/// This alternative version of [find_essential_mat_2] function uses the following default values for its arguments:
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1523
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_2_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1531
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "cameraMatrix2", "dist_coeff1", "dist_coeff2", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
#[inline]
pub fn find_essential_mat_3(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeff1: &impl ToInputArray, dist_coeff2: &impl ToInputArray, mask: &mut impl ToOutputArray, params: crate::mod_3d::UsacParams) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeff1);
	input_array_arg!(dist_coeff2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeff1.as_raw__InputArray(), dist_coeff2.as_raw__InputArray(), mask.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Calculates an essential matrix from the corresponding points in two images from potentially two different cameras.
///
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix1: Camera matrix for the first camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * cameraMatrix2: Camera matrix for the second camera ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs1: Input vector of distortion coefficients for the first camera
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * distCoeffs2: Input vector of distortion coefficients for the second camera
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
///
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or  [recover_pose] to recover the relative pose between cameras.
///
/// ## C++ default parameters
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1523
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_2(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, method: i32, prob: f64, threshold: f64, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), method, prob, threshold, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Calculates an essential matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
/// When passing these coordinates, pass the identity matrix for this parameter.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// * maxIters: The maximum number of robust method iterations.
///
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## C++ default parameters
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * max_iters: 1000
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1443
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_essential_mat(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, method: i32, prob: f64, threshold: f64, max_iters: i32, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), method, prob, threshold, max_iters, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Calculates an essential matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use another
/// function overload or [undistort_points] with `P = cv::NoArray()` for both cameras to transform image
/// points to normalized image coordinates, which are valid for the identity camera intrinsic matrix.
/// When passing these coordinates, pass the identity matrix for this parameter.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// * maxIters: The maximum number of robust method iterations.
///
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## Overloaded parameters
///
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * focal: focal length of the camera. Note that this function assumes that points1 and points2
/// are feature points from cameras with same focal length and principal point.
/// * pp: principal point of the camera.
/// * method: Method for computing a fundamental matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// * maxIters: The maximum number of robust method iterations.
///
/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
/// principal point:
///
/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
///
/// ## C++ default parameters
/// * focal: 1.0
/// * pp: Point2d(0,0)
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * max_iters: 1000
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1481
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_1(points1: &impl ToInputArray, points2: &impl ToInputArray, focal: f64, pp: core::Point2d, method: i32, prob: f64, threshold: f64, max_iters: i32, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), focal, &pp, method, prob, threshold, max_iters, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_fundamental_mat_1] function uses the following default values for its arguments:
/// * method: FM_RANSAC
/// * ransac_reproj_threshold: 3.
/// * confidence: 0.99
/// * mask: noArray()
// cv::findFundamentalMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1395
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_fundamental_mat_1_def(points1: &impl ToInputArray, points2: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_fundamental_mat_mask] function uses the following default values for its arguments:
/// * method: FM_RANSAC
/// * ransac_reproj_threshold: 3.
/// * confidence: 0.99
// cv::findFundamentalMat(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1401
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_fundamental_mat_mask_def(points1: &impl ToInputArray, points2: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// findFundamentalMat(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1406
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
#[inline]
pub fn find_fundamental_mat_2(points1: &impl ToInputArray, points2: &impl ToInputArray, mask: &mut impl ToOutputArray, params: crate::mod_3d::UsacParams) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), mask.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @example samples/cpp/snippets/epipolar_lines.cpp
/// An example using the findFundamentalMat function
///
/// Calculates a fundamental matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * method: Method for computing a fundamental matrix.
/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
/// of confidence (probability) that the estimated matrix is correct.
/// * mask:[out] optional output mask
/// * maxIters: The maximum number of robust method iterations.
///
/// The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively.
///
/// The function calculates the fundamental matrix using one of four methods listed above and returns
/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
/// matrices sequentially).
///
/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
/// epipolar lines corresponding to the specified points. It can also be passed to
/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    Mat fundamental_matrix =
///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
/// ```
///
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: FM_RANSAC
/// * ransac_reproj_threshold: 3.
/// * confidence: 0.99
// findFundamentalMat(InputArray, InputArray, OutputArray, int, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1401
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "mask", "method", "ransacReprojThreshold", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
#[inline]
pub fn find_fundamental_mat_mask(points1: &impl ToInputArray, points2: &impl ToInputArray, mask: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(points1.as_raw__InputArray(), points2.as_raw__InputArray(), mask.as_raw__OutputArray(), method, ransac_reproj_threshold, confidence, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @example samples/cpp/snippets/epipolar_lines.cpp
/// An example using the findFundamentalMat function
///
/// Calculates a fundamental matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * method: Method for computing a fundamental matrix.
/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
/// of confidence (probability) that the estimated matrix is correct.
/// * mask:[out] optional output mask
/// * maxIters: The maximum number of robust method iterations.
///
/// The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively.
///
/// The function calculates the fundamental matrix using one of four methods listed above and returns
/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
/// matrices sequentially).
///
/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
/// epipolar lines corresponding to the specified points. It can also be passed to
/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    Mat fundamental_matrix =
///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
/// ```
///
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: FM_RANSAC
/// * ransac_reproj_threshold: 3.
/// * confidence: 0.99
/// * mask: noArray()
// findFundamentalMat(InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1395
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_fundamental_mat_1(points1: &impl ToInputArray, points2: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), method, ransac_reproj_threshold, confidence, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @example samples/cpp/snippets/epipolar_lines.cpp
/// An example using the findFundamentalMat function
///
/// Calculates a fundamental matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * method: Method for computing a fundamental matrix.
/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
/// of confidence (probability) that the estimated matrix is correct.
/// * mask:[out] optional output mask
/// * maxIters: The maximum number of robust method iterations.
///
/// The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively.
///
/// The function calculates the fundamental matrix using one of four methods listed above and returns
/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
/// matrices sequentially).
///
/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
/// epipolar lines corresponding to the specified points. It can also be passed to
/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    Mat fundamental_matrix =
///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
/// ```
///
///
/// ## Note
/// This alternative version of [find_fundamental_mat] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::findFundamentalMat(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1390
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int"]), _)]),
#[inline]
pub fn find_fundamental_mat_def(points1: &impl ToInputArray, points2: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64, max_iters: i32) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int(points1.as_raw__InputArray(), points2.as_raw__InputArray(), method, ransac_reproj_threshold, confidence, max_iters, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @example samples/cpp/snippets/epipolar_lines.cpp
/// An example using the findFundamentalMat function
///
/// Calculates a fundamental matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * method: Method for computing a fundamental matrix.
/// *   [FM_7POINT] for a 7-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%3D%207)
/// *   [FM_8POINT] for an 8-point algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_RANSAC] for the RANSAC algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// *   [FM_LMEDS] for the LMedS algorithm. ![inline formula](https://latex.codecogs.com/png.latex?N%20%5Cge%208)
/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
/// of confidence (probability) that the estimated matrix is correct.
/// * mask:[out] optional output mask
/// * maxIters: The maximum number of robust method iterations.
///
/// The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20F%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?F) is a fundamental matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively.
///
/// The function calculates the fundamental matrix using one of four methods listed above and returns
/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
/// algorithm, the function may return up to 3 solutions ( ![inline formula](https://latex.codecogs.com/png.latex?9%20%5Ctimes%203) matrix that stores all 3
/// matrices sequentially).
///
/// The calculated fundamental matrix may be passed further to [compute_correspond_epilines] that finds the
/// epipolar lines corresponding to the specified points. It can also be passed to
/// [stereo_rectify_uncalibrated] to compute the rectification transformation. :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    Mat fundamental_matrix =
///      findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
/// ```
///
///
/// ## C++ default parameters
/// * mask: noArray()
// findFundamentalMat(InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1390
// ("cv::findFundamentalMat", vec![(pred!(mut, ["points1", "points2", "method", "ransacReprojThreshold", "confidence", "maxIters", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_fundamental_mat(points1: &impl ToInputArray, points2: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, confidence: f64, max_iters: i32, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findFundamentalMat_const__InputArrayR_const__InputArrayR_int_double_double_int_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), method, ransac_reproj_threshold, confidence, max_iters, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Finds a perspective transformation between two planes.
///
/// ## Parameters
/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
/// or vector\<Point2f\> .
/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
/// a vector\<Point2f\> .
/// * method: Method used to compute a homography matrix. The following methods are possible:
/// *   **0** - a regular method using all the points, i.e., the least squares method
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// *   [RHO] - PROSAC-based robust method
/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
/// (used in the RANSAC and RHO methods only). That is, if
/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
/// mask values are ignored.
/// * maxIters: The maximum number of RANSAC iterations.
/// * confidence: Confidence level, between 0 and 1.
///
/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
/// destination planes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
///
/// so that the back-projection error
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
///
/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
/// pairs to compute an initial homography estimate with a simple least-squares scheme.
///
/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
/// the mask of inliers/outliers.
///
/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
/// re-projection error even more.
///
/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
/// noise is rather small, use the default method (method=0).
///
/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
///
/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
/// ## See also
/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
/// perspectiveTransform
///
/// ## Note
/// This alternative version of [find_homography_ext] function uses the following default values for its arguments:
/// * method: 0
/// * ransac_reproj_threshold: 3
/// * mask: noArray()
/// * max_iters: 2000
/// * confidence: 0.995
// cv::findHomography(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:802
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_homography_ext_def(src_points: &impl ToInputArray, dst_points: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(src_points);
	input_array_arg!(dst_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_homography] function uses the following default values for its arguments:
/// * method: 0
/// * ransac_reproj_threshold: 3
// cv::findHomography(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:808
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_homography_def(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(src_points);
	input_array_arg!(dst_points);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// findHomography(InputArray, InputArray, OutputArray, const UsacParams &)(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:812
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
#[inline]
pub fn find_homography_1(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, mask: &mut impl ToOutputArray, params: crate::mod_3d::UsacParams) -> Result<core::Mat> {
	input_array_arg!(src_points);
	input_array_arg!(dst_points);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_UsacParamsR(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), mask.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Finds a perspective transformation between two planes.
///
/// ## Parameters
/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
/// or vector\<Point2f\> .
/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
/// a vector\<Point2f\> .
/// * method: Method used to compute a homography matrix. The following methods are possible:
/// *   **0** - a regular method using all the points, i.e., the least squares method
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// *   [RHO] - PROSAC-based robust method
/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
/// (used in the RANSAC and RHO methods only). That is, if
/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
/// mask values are ignored.
/// * maxIters: The maximum number of RANSAC iterations.
/// * confidence: Confidence level, between 0 and 1.
///
/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
/// destination planes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
///
/// so that the back-projection error
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
///
/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
/// pairs to compute an initial homography estimate with a simple least-squares scheme.
///
/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
/// the mask of inliers/outliers.
///
/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
/// re-projection error even more.
///
/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
/// noise is rather small, use the default method (method=0).
///
/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
///
/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
/// ## See also
/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
/// perspectiveTransform
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: 0
/// * ransac_reproj_threshold: 3
// findHomography(InputArray, InputArray, OutputArray, int, double)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:808
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "mask", "method", "ransacReprojThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
#[inline]
pub fn find_homography(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, mask: &mut impl ToOutputArray, method: i32, ransac_reproj_threshold: f64) -> Result<core::Mat> {
	input_array_arg!(src_points);
	input_array_arg!(dst_points);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), mask.as_raw__OutputArray(), method, ransac_reproj_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Finds a perspective transformation between two planes.
///
/// ## Parameters
/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
/// or vector\<Point2f\> .
/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
/// a vector\<Point2f\> .
/// * method: Method used to compute a homography matrix. The following methods are possible:
/// *   **0** - a regular method using all the points, i.e., the least squares method
/// *   [RANSAC] - RANSAC-based robust method
/// *   [LMEDS] - Least-Median robust method
/// *   [RHO] - PROSAC-based robust method
/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
/// (used in the RANSAC and RHO methods only). That is, if
/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7BdstPoints%7D%20%5Fi%20%2D%20%20%5Ctexttt%7BconvertPointsHomogeneous%7D%20%28%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BsrcPoints%7D%20%5Fi%29%20%5C%7C%5F2%20%20%3E%20%20%5Ctexttt%7BransacReprojThreshold%7D)
/// then the point ![inline formula](https://latex.codecogs.com/png.latex?i) is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
/// * mask: Optional output mask set by a robust method ( RANSAC or LMeDS ). Note that the input
/// mask values are ignored.
/// * maxIters: The maximum number of RANSAC iterations.
/// * confidence: Confidence level, between 0 and 1.
///
/// The function finds and returns the perspective transformation ![inline formula](https://latex.codecogs.com/png.latex?H) between the source and the
/// destination planes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?s%5Fi%20%20%5Cbegin%7Bbmatrix%7D%20x%27%5Fi%5C%5C%20y%27%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%5Csim%20H%20%20%5Cbegin%7Bbmatrix%7D%20x%5Fi%5C%5C%20y%5Fi%5C%5C%201%20%5Cend%7Bbmatrix%7D)
///
/// so that the back-projection error
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Csum%20%5Fi%20%5Cleft%20%28%20x%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B11%7D%20x%5Fi%20%2B%20h%5F%7B12%7D%20y%5Fi%20%2B%20h%5F%7B13%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2%2B%20%5Cleft%20%28%20y%27%5Fi%2D%20%5Cfrac%7Bh%5F%7B21%7D%20x%5Fi%20%2B%20h%5F%7B22%7D%20y%5Fi%20%2B%20h%5F%7B23%7D%7D%7Bh%5F%7B31%7D%20x%5Fi%20%2B%20h%5F%7B32%7D%20y%5Fi%20%2B%20h%5F%7B33%7D%7D%20%5Cright%20%29%5E2)
///
/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
/// pairs to compute an initial homography estimate with a simple least-squares scheme.
///
/// However, if not all of the point pairs ( ![inline formula](https://latex.codecogs.com/png.latex?srcPoints%5Fi), ![inline formula](https://latex.codecogs.com/png.latex?dstPoints%5Fi) ) fit the rigid perspective
/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
/// the mask of inliers/outliers.
///
/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
/// re-projection error even more.
///
/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
/// noise is rather small, use the default method (method=0).
///
/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
/// determined up to a scale. If ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D) is non-zero, the matrix is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1).
///
/// Note: Whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix cannot be estimated, an empty one will be returned.
/// ## See also
/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
/// perspectiveTransform
///
/// ## C++ default parameters
/// * method: 0
/// * ransac_reproj_threshold: 3
/// * mask: noArray()
/// * max_iters: 2000
/// * confidence: 0.995
// findHomography(InputArray, InputArray, int, double, OutputArray, const int, const double)(InputArray, InputArray, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:802
// ("cv::findHomography", vec![(pred!(mut, ["srcPoints", "dstPoints", "method", "ransacReprojThreshold", "mask", "maxIters", "confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "const cv::_OutputArray*", "const int", "const double"]), _)]),
#[inline]
pub fn find_homography_ext(src_points: &impl ToInputArray, dst_points: &impl ToInputArray, method: i32, ransac_reproj_threshold: f64, mask: &mut impl ToOutputArray, max_iters: i32, confidence: f64) -> Result<core::Mat> {
	input_array_arg!(src_points);
	input_array_arg!(dst_points);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findHomography_const__InputArrayR_const__InputArrayR_int_double_const__OutputArrayR_const_int_const_double(src_points.as_raw__InputArray(), dst_points.as_raw__InputArray(), method, ransac_reproj_threshold, mask.as_raw__OutputArray(), max_iters, confidence, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Find the planes in a depth image
/// ## Parameters
/// * points3d: the 3d points organized like the depth image: rows x cols with 3 channels
/// * normals: the normals for every point in the depth image; optional, can be empty
/// * mask: An image where each pixel is labeled with the plane it belongs to
///        and 255 if it does not belong to any plane
/// * plane_coefficients: the coefficients of the corresponding planes (a,b,c,d) such that ax+by+cz+d=0, norm(a,b,c)=1
///        and c < 0 (so that the normal points towards the camera)
/// * block_size: The size of the blocks to look at for a stable MSE
/// * min_size: The minimum size of a cluster to be considered a plane
/// * threshold: The maximum distance of a point from a plane to belong to it (in meters)
/// * sensor_error_a: coefficient of the sensor error. 0 by default, use 0.0075 for a Kinect
/// * sensor_error_b: coefficient of the sensor error. 0 by default
/// * sensor_error_c: coefficient of the sensor error. 0 by default
/// * method: The method to use to compute the planes.
///
/// ## Note
/// This alternative version of [find_planes] function uses the following default values for its arguments:
/// * block_size: 40
/// * min_size: 40*40
/// * threshold: 0.01
/// * sensor_error_a: 0
/// * sensor_error_b: 0
/// * sensor_error_c: 0
/// * method: RGBD_PLANE_METHOD_DEFAULT
// cv::findPlanes(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:165
// ("cv::findPlanes", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_planes_def(points3d: &impl ToInputArray, normals: &impl ToInputArray, mask: &mut impl ToOutputArray, plane_coefficients: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points3d);
	input_array_arg!(normals);
	output_array_arg!(mask);
	output_array_arg!(plane_coefficients);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points3d.as_raw__InputArray(), normals.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Find the planes in a depth image
/// ## Parameters
/// * points3d: the 3d points organized like the depth image: rows x cols with 3 channels
/// * normals: the normals for every point in the depth image; optional, can be empty
/// * mask: An image where each pixel is labeled with the plane it belongs to
///        and 255 if it does not belong to any plane
/// * plane_coefficients: the coefficients of the corresponding planes (a,b,c,d) such that ax+by+cz+d=0, norm(a,b,c)=1
///        and c < 0 (so that the normal points towards the camera)
/// * block_size: The size of the blocks to look at for a stable MSE
/// * min_size: The minimum size of a cluster to be considered a plane
/// * threshold: The maximum distance of a point from a plane to belong to it (in meters)
/// * sensor_error_a: coefficient of the sensor error. 0 by default, use 0.0075 for a Kinect
/// * sensor_error_b: coefficient of the sensor error. 0 by default
/// * sensor_error_c: coefficient of the sensor error. 0 by default
/// * method: The method to use to compute the planes.
///
/// ## C++ default parameters
/// * block_size: 40
/// * min_size: 40*40
/// * threshold: 0.01
/// * sensor_error_a: 0
/// * sensor_error_b: 0
/// * sensor_error_c: 0
/// * method: RGBD_PLANE_METHOD_DEFAULT
// findPlanes(InputArray, InputArray, OutputArray, OutputArray, int, int, double, double, double, double, RgbdPlaneMethod)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:165
// ("cv::findPlanes", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "double", "double", "cv::RgbdPlaneMethod"]), _)]),
#[inline]
pub fn find_planes(points3d: &impl ToInputArray, normals: &impl ToInputArray, mask: &mut impl ToOutputArray, plane_coefficients: &mut impl ToOutputArray, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, method: crate::mod_3d::RgbdPlaneMethod) -> Result<()> {
	input_array_arg!(points3d);
	input_array_arg!(normals);
	output_array_arg!(mask);
	output_array_arg!(plane_coefficients);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_double_double_double_double_RgbdPlaneMethod(points3d.as_raw__InputArray(), normals.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, method, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Distorts 2D points using fisheye model.
///
/// ## Parameters
/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
/// the number of points in the view.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
/// use another function overload.
///
/// ## Note
/// This alternative version of [fisheye_distort_points] function uses the following default values for its arguments:
/// * alpha: 0
// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2518
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn fisheye_distort_points_def(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(undistorted);
	output_array_arg!(distorted);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
/// Overload of distortPoints function to handle cases when undistorted points are got with non-identity
/// camera matrix, e.g. output of #estimateNewCameraMatrixForUndistortRectify.
/// ## Parameters
/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
/// the number of points in the view.
/// * Kundistorted: Camera intrinsic matrix used as new camera matrix for undistortion.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
/// ## See also
/// estimateNewCameraMatrixForUndistortRectify
///
/// ## Note
/// This alternative version of [distort_points] function uses the following default values for its arguments:
/// * alpha: 0
// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2532
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "Kundistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn distort_points_def(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, kundistorted: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(undistorted);
	output_array_arg!(distorted);
	input_array_arg!(kundistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), kundistorted.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Distorts 2D points using fisheye model.
///
/// ## Parameters
/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
/// the number of points in the view.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
/// use another function overload.
///
/// ## Overloaded parameters
///
/// Overload of distortPoints function to handle cases when undistorted points are got with non-identity
/// camera matrix, e.g. output of #estimateNewCameraMatrixForUndistortRectify.
/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
/// the number of points in the view.
/// * Kundistorted: Camera intrinsic matrix used as new camera matrix for undistortion.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
/// ## See also
/// estimateNewCameraMatrixForUndistortRectify
///
/// ## C++ default parameters
/// * alpha: 0
// distortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2532
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "Kundistorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
#[inline]
pub fn distort_points(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, kundistorted: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64) -> Result<()> {
	input_array_arg!(undistorted);
	output_array_arg!(distorted);
	input_array_arg!(kundistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), kundistorted.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Distorts 2D points using fisheye model.
///
/// ## Parameters
/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
/// the number of points in the view.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// Note that the function assumes the camera intrinsic matrix of the undistorted points to be identity.
/// This means if you want to distort image points you have to multiply them with ![inline formula](https://latex.codecogs.com/png.latex?K%5E%7B%2D1%7D) or
/// use another function overload.
///
/// ## C++ default parameters
/// * alpha: 0
// distortPoints(InputArray, OutputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2518
// ("cv::fisheye::distortPoints", vec![(pred!(mut, ["undistorted", "distorted", "K", "D", "alpha"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double"]), _)]),
#[inline]
pub fn fisheye_distort_points(undistorted: &impl ToInputArray, distorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64) -> Result<()> {
	input_array_arg!(undistorted);
	output_array_arg!(distorted);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_distortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_double(undistorted.as_raw__InputArray(), distorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimates new camera intrinsic matrix for undistortion or rectification.
///
/// ## Parameters
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * image_size: Size of the image
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * new_size: the new size
/// * fov_scale: Divisor for new focal length.
///
/// ## Note
/// This alternative version of [estimate_new_camera_matrix_for_undistort_rectify] function uses the following default values for its arguments:
/// * balance: 0.0
/// * new_size: Size()
/// * fov_scale: 1.0
// cv::fisheye::estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, SimpleClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2563
// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn estimate_new_camera_matrix_for_undistort_rectify_def(k: &impl ToInputArray, d: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, p: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(r);
	output_array_arg!(p);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR(k.as_raw__InputArray(), d.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), p.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimates new camera intrinsic matrix for undistortion or rectification.
///
/// ## Parameters
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * image_size: Size of the image
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * new_size: the new size
/// * fov_scale: Divisor for new focal length.
///
/// ## C++ default parameters
/// * balance: 0.0
/// * new_size: Size()
/// * fov_scale: 1.0
// estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, const Size &, InputArray, OutputArray, double, const Size &, double)(InputArray, InputArray, SimpleClass, InputArray, OutputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2563
// ("cv::fisheye::estimateNewCameraMatrixForUndistortRectify", vec![(pred!(mut, ["K", "D", "image_size", "R", "P", "balance", "new_size", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "double"]), _)]),
#[inline]
pub fn estimate_new_camera_matrix_for_undistort_rectify(k: &impl ToInputArray, d: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, p: &mut impl ToOutputArray, balance: f64, new_size: core::Size, fov_scale: f64) -> Result<()> {
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(r);
	output_array_arg!(p);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_estimateNewCameraMatrixForUndistortRectify_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_double_const_SizeR_double(k.as_raw__InputArray(), d.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), p.as_raw__OutputArray(), balance, &new_size, fov_scale, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes undistortion and rectification maps for image transform by cv::remap(). If D is empty zero
/// distortion is used, if R or P is empty identity matrixes are used.
///
/// ## Parameters
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
/// * size: Undistorted image size.
/// * m1type: Type of the first output map that can be CV_32FC1 or CV_16SC2 . See convertMaps()
/// for details.
/// * map1: The first output map.
/// * map2: The second output map.
// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2580
// ("cv::fisheye::initUndistortRectifyMap", vec![(pred!(mut, ["K", "D", "R", "P", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fisheye_init_undistort_rectify_map(k: &impl ToInputArray, d: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray, size: core::Size, m1type: i32, map1: &mut impl ToOutputArray, map2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(r);
	input_array_arg!(p);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(k.as_raw__InputArray(), d.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &size, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Projects points using fisheye model
///
/// ## Parameters
/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
/// the number of points in the view.
/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
/// vector\<Point2f\>.
/// * affine: 
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
/// rotation vector, translation vector, and the skew. In the old interface different components of
/// the jacobian are returned via different output parameters.
///
/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
/// image points coordinates (as functions of all the input parameters) with respect to the particular
/// parameters, intrinsic and/or extrinsic.
///
/// ## Note
/// This alternative version of [fisheye_project_points] function uses the following default values for its arguments:
/// * alpha: 0
/// * jacobian: noArray()
// cv::fisheye::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2498
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn fisheye_project_points_def(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, affine: core::Affine3d, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Projects points using fisheye model
///
/// ## Parameters
/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
/// the number of points in the view.
/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
/// vector\<Point2f\>.
/// * affine: 
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
/// rotation vector, translation vector, and the skew. In the old interface different components of
/// the jacobian are returned via different output parameters.
///
/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
/// image points coordinates (as functions of all the input parameters) with respect to the particular
/// parameters, intrinsic and/or extrinsic.
///
/// ## C++ default parameters
/// * alpha: 0
/// * jacobian: noArray()
// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2498
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fisheye_project_points(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, affine: core::Affine3d, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64, jacobian: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(k);
	input_array_arg!(d);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [fisheye_project_points_vec] function uses the following default values for its arguments:
/// * alpha: 0
/// * jacobian: noArray()
// cv::fisheye::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2502
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn fisheye_project_points_vec_def(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Projects points using fisheye model
///
/// ## Parameters
/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
/// the number of points in the view.
/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
/// vector\<Point2f\>.
/// * affine: 
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * alpha: The skew coefficient.
/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
/// rotation vector, translation vector, and the skew. In the old interface different components of
/// the jacobian are returned via different output parameters.
///
/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
/// image points coordinates (as functions of all the input parameters) with respect to the particular
/// parameters, intrinsic and/or extrinsic.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * alpha: 0
/// * jacobian: noArray()
// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2502
// ("cv::fisheye::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "D", "alpha", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fisheye_project_points_vec(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, k: &impl ToInputArray, d: &impl ToInputArray, alpha: f64, jacobian: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(k);
	input_array_arg!(d);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), alpha, jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences for fisheye camera moodel.
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients (4x1/1x4).
/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
/// coordinate frame to the camera coordinate frame, using different methods:
/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
/// Number of input points must be 4. Object points must be defined in the following order:
/// - point 0: [-squareLength / 2,  squareLength / 2, 0]
/// - point 1: [ squareLength / 2,  squareLength / 2, 0]
/// - point 2: [ squareLength / 2, -squareLength / 2, 0]
/// - point 3: [-squareLength / 2, -squareLength / 2, 0]
/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
/// * criteria: Termination criteria for internal undistortPoints call.
/// The function interally undistorts points with [undistortPoints] and call [cv::solvePnP],
/// thus the input are very similar. Check there and Perspective-n-Points is described in [calib3d_solvePnP]
/// for more information.
///
/// ## Note
/// This alternative version of [solve_pnp_1] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
// cv::fisheye::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2646
// ("cv::fisheye::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_1_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences for fisheye camera moodel.
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients (4x1/1x4).
/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
/// coordinate frame to the camera coordinate frame, using different methods:
/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
/// Number of input points must be 4. Object points must be defined in the following order:
/// - point 0: [-squareLength / 2,  squareLength / 2, 0]
/// - point 1: [ squareLength / 2,  squareLength / 2, 0]
/// - point 2: [ squareLength / 2, -squareLength / 2, 0]
/// - point 3: [-squareLength / 2, -squareLength / 2, 0]
/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
/// * criteria: Termination criteria for internal undistortPoints call.
/// The function interally undistorts points with [undistortPoints] and call [cv::solvePnP],
/// thus the input are very similar. Check there and Perspective-n-Points is described in [calib3d_solvePnP]
/// for more information.
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, TermCriteria)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2646
// ("cv::fisheye::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn solve_pnp_1(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: i32, criteria: core::TermCriteria) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, flags, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Transforms an image to compensate for fisheye lens distortion.
///
/// ## Parameters
/// * distorted: image with fisheye lens distortion.
/// * undistorted: Output image with compensated fisheye lens distortion.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * Knew: Camera intrinsic matrix of the distorted image. By default, it is the identity matrix but you
/// may additionally scale and shift the result by using a different matrix.
/// * new_size: the new size
///
/// The function transforms an image to compensate radial and tangential lens distortion.
///
/// The function is simply a combination of fisheye::initUndistortRectifyMap (with unity R ) and remap
/// (with bilinear interpolation). See the former function for details of the transformation being
/// performed.
///
/// See below the results of undistortImage.
///    *   a\) result of undistort of perspective camera model (all possible coefficients (k_1, k_2, k_3,
///        k_4, k_5, k_6) of distortion were optimized under calibration)
///    *   b\) result of fisheye::undistortImage of fisheye camera model (all possible coefficients (k_1, k_2,
///        k_3, k_4) of fisheye distortion were optimized under calibration)
///    *   c\) original image was captured with fisheye lens
///
/// Pictures a) and b) almost the same. But if we consider points of image located far from the center
/// of image, we can notice that on image a) these points are distorted.
///
/// ![image](https://docs.opencv.org/5.0.0/fisheye_undistorted.jpg)
///
/// ## Note
/// This alternative version of [fisheye_undistort_image] function uses the following default values for its arguments:
/// * knew: cv::noArray()
/// * new_size: Size()
// cv::fisheye::undistortImage(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2611
// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn fisheye_undistort_image_def(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Transforms an image to compensate for fisheye lens distortion.
///
/// ## Parameters
/// * distorted: image with fisheye lens distortion.
/// * undistorted: Output image with compensated fisheye lens distortion.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * Knew: Camera intrinsic matrix of the distorted image. By default, it is the identity matrix but you
/// may additionally scale and shift the result by using a different matrix.
/// * new_size: the new size
///
/// The function transforms an image to compensate radial and tangential lens distortion.
///
/// The function is simply a combination of fisheye::initUndistortRectifyMap (with unity R ) and remap
/// (with bilinear interpolation). See the former function for details of the transformation being
/// performed.
///
/// See below the results of undistortImage.
///    *   a\) result of undistort of perspective camera model (all possible coefficients (k_1, k_2, k_3,
///        k_4, k_5, k_6) of distortion were optimized under calibration)
///    *   b\) result of fisheye::undistortImage of fisheye camera model (all possible coefficients (k_1, k_2,
///        k_3, k_4) of fisheye distortion were optimized under calibration)
///    *   c\) original image was captured with fisheye lens
///
/// Pictures a) and b) almost the same. But if we consider points of image located far from the center
/// of image, we can notice that on image a) these points are distorted.
///
/// ![image](https://docs.opencv.org/5.0.0/fisheye_undistorted.jpg)
///
/// ## C++ default parameters
/// * knew: cv::noArray()
/// * new_size: Size()
// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, const Size &)(InputArray, OutputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2611
// ("cv::fisheye::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "Knew", "new_size"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*"]), _)]),
#[inline]
pub fn fisheye_undistort_image(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, knew: &impl ToInputArray, new_size: core::Size) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(knew);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), knew.as_raw__InputArray(), &new_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Undistorts 2D points using fisheye model
///
/// ## Parameters
/// * distorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is the
/// number of points in the view.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
/// * criteria: Termination criteria
/// * undistorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// ## Note
/// This alternative version of [fisheye_undistort_points] function uses the following default values for its arguments:
/// * r: noArray()
/// * p: noArray()
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
// cv::fisheye::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2546
// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn fisheye_undistort_points_def(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Undistorts 2D points using fisheye model
///
/// ## Parameters
/// * distorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is the
/// number of points in the view.
/// * K: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?cameramatrix%7BK%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera intrinsic matrix (3x3) or new projection matrix (3x4)
/// * criteria: Termination criteria
/// * undistorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// ## C++ default parameters
/// * r: noArray()
/// * p: noArray()
/// * criteria: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,10,1e-8)
// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2546
// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "R", "P", "criteria"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
#[inline]
pub fn fisheye_undistort_points(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray, criteria: core::TermCriteria) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(r);
	input_array_arg!(p);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Returns the default new camera matrix.
///
/// The function returns the camera matrix that is either an exact copy of the input cameraMatrix (when
/// centerPrinicipalPoint=false ), or the modified one (when centerPrincipalPoint=true).
///
/// In the latter case, the new camera matrix will be:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%26%200%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Ewidth%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%20f%5Fy%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Eheight%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%200%20%26%26%201%20%5Cend%7Bbmatrix%7D%20%2C)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) are ![inline formula](https://latex.codecogs.com/png.latex?%280%2C0%29) and ![inline formula](https://latex.codecogs.com/png.latex?%281%2C1%29) elements of cameraMatrix, respectively.
///
/// By default, the undistortion functions in OpenCV (see #initUndistortRectifyMap, #undistort) do not
/// move the principal point. However, when you work with stereo, it is important to move the principal
/// points in both views to the same y-coordinate (which is required by most of stereo correspondence
/// algorithms), and may be to the same x-coordinate too. So, you can form the new camera matrix for
/// each view where the principal points are located at the center.
///
/// ## Parameters
/// * cameraMatrix: Input camera matrix.
/// * imgsize: Camera view image size in pixels.
/// * centerPrincipalPoint: Location of the principal point in the new camera matrix. The
/// parameter indicates whether this location should be at the image center or not.
///
/// ## Note
/// This alternative version of [get_default_new_camera_matrix] function uses the following default values for its arguments:
/// * imgsize: Size()
/// * center_principal_point: false
// cv::getDefaultNewCameraMatrix(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2357
// ("cv::getDefaultNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix"], ["const cv::_InputArray*"]), _)]),
#[inline]
pub fn get_default_new_camera_matrix_def(camera_matrix: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getDefaultNewCameraMatrix_const__InputArrayR(camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns the default new camera matrix.
///
/// The function returns the camera matrix that is either an exact copy of the input cameraMatrix (when
/// centerPrinicipalPoint=false ), or the modified one (when centerPrincipalPoint=true).
///
/// In the latter case, the new camera matrix will be:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%26%200%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Ewidth%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%20f%5Fy%20%26%26%20%28%20%5Ctexttt%7BimgSize%2Eheight%7D%20%2D1%29%2A0%2E5%20%20%5C%5C%200%20%26%26%200%20%26%26%201%20%5Cend%7Bbmatrix%7D%20%2C)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) are ![inline formula](https://latex.codecogs.com/png.latex?%280%2C0%29) and ![inline formula](https://latex.codecogs.com/png.latex?%281%2C1%29) elements of cameraMatrix, respectively.
///
/// By default, the undistortion functions in OpenCV (see #initUndistortRectifyMap, #undistort) do not
/// move the principal point. However, when you work with stereo, it is important to move the principal
/// points in both views to the same y-coordinate (which is required by most of stereo correspondence
/// algorithms), and may be to the same x-coordinate too. So, you can form the new camera matrix for
/// each view where the principal points are located at the center.
///
/// ## Parameters
/// * cameraMatrix: Input camera matrix.
/// * imgsize: Camera view image size in pixels.
/// * centerPrincipalPoint: Location of the principal point in the new camera matrix. The
/// parameter indicates whether this location should be at the image center or not.
///
/// ## C++ default parameters
/// * imgsize: Size()
/// * center_principal_point: false
// getDefaultNewCameraMatrix(InputArray, Size, bool)(InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2357
// ("cv::getDefaultNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "imgsize", "centerPrincipalPoint"], ["const cv::_InputArray*", "cv::Size", "bool"]), _)]),
#[inline]
pub fn get_default_new_camera_matrix(camera_matrix: &impl ToInputArray, imgsize: core::Size, center_principal_point: bool) -> Result<core::Mat> {
	input_array_arg!(camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getDefaultNewCameraMatrix_const__InputArrayR_Size_bool(camera_matrix.as_raw__InputArray(), &imgsize, center_principal_point, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns the new camera intrinsic matrix based on the free scaling parameter.
///
/// ## Parameters
/// * cameraMatrix: Input camera intrinsic matrix.
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * imageSize: Original image size.
/// * alpha: Free scaling parameter between 0 (when all the pixels in the undistorted image are
/// valid) and 1 (when all the source image pixels are retained in the undistorted image). See
/// [stereo_rectify] for details.
/// * newImgSize: Image size after rectification. By default, it is set to imageSize .
/// * validPixROI: Optional output rectangle that outlines all-good-pixels region in the
/// undistorted image. See roi1, roi2 description in [stereo_rectify] .
/// * centerPrincipalPoint: Optional flag that indicates whether in the new camera intrinsic matrix the
/// principal point should be at the image center or not. By default, the principal point is chosen to
/// best fit a subset of the source image (determined by alpha) to the corrected image.
/// ## Returns
/// new_camera_matrix Output new camera intrinsic matrix.
///
/// The function computes and returns the optimal new camera intrinsic matrix based on the free scaling parameter.
/// By varying this parameter, you may retrieve only sensible pixels alpha=0 , keep all the original
/// image pixels if there is valuable information in the corners alpha=1 , or get something in between.
/// When alpha\>0 , the undistorted result is likely to have some black pixels corresponding to
/// "virtual" pixels outside of the captured distorted image. The original camera intrinsic matrix, distortion
/// coefficients, the computed new camera intrinsic matrix, and newImageSize should be passed to
/// [init_undistort_rectify_map] to produce the maps for [remap] .
///
/// ## Note
/// This alternative version of [get_optimal_new_camera_matrix] function uses the following default values for its arguments:
/// * new_img_size: Size()
/// * valid_pix_roi: 0
/// * center_principal_point: false
// cv::getOptimalNewCameraMatrix(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2410
// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
#[inline]
pub fn get_optimal_new_camera_matrix_def(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_size: core::Size, alpha: f64) -> Result<core::Mat> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &image_size, alpha, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns the new camera intrinsic matrix based on the free scaling parameter.
///
/// ## Parameters
/// * cameraMatrix: Input camera intrinsic matrix.
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * imageSize: Original image size.
/// * alpha: Free scaling parameter between 0 (when all the pixels in the undistorted image are
/// valid) and 1 (when all the source image pixels are retained in the undistorted image). See
/// [stereo_rectify] for details.
/// * newImgSize: Image size after rectification. By default, it is set to imageSize .
/// * validPixROI: Optional output rectangle that outlines all-good-pixels region in the
/// undistorted image. See roi1, roi2 description in [stereo_rectify] .
/// * centerPrincipalPoint: Optional flag that indicates whether in the new camera intrinsic matrix the
/// principal point should be at the image center or not. By default, the principal point is chosen to
/// best fit a subset of the source image (determined by alpha) to the corrected image.
/// ## Returns
/// new_camera_matrix Output new camera intrinsic matrix.
///
/// The function computes and returns the optimal new camera intrinsic matrix based on the free scaling parameter.
/// By varying this parameter, you may retrieve only sensible pixels alpha=0 , keep all the original
/// image pixels if there is valuable information in the corners alpha=1 , or get something in between.
/// When alpha\>0 , the undistorted result is likely to have some black pixels corresponding to
/// "virtual" pixels outside of the captured distorted image. The original camera intrinsic matrix, distortion
/// coefficients, the computed new camera intrinsic matrix, and newImageSize should be passed to
/// [init_undistort_rectify_map] to produce the maps for [remap] .
///
/// ## C++ default parameters
/// * new_img_size: Size()
/// * valid_pix_roi: 0
/// * center_principal_point: false
// getOptimalNewCameraMatrix(InputArray, InputArray, Size, double, Size, Rect *, bool)(InputArray, InputArray, SimpleClass, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2410
// ("cv::getOptimalNewCameraMatrix", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "alpha", "newImgSize", "validPixROI", "centerPrincipalPoint"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double", "cv::Size", "cv::Rect*", "bool"]), _)]),
#[inline]
pub fn get_optimal_new_camera_matrix(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_size: core::Size, alpha: f64, new_img_size: core::Size, valid_pix_roi: Option<&mut core::Rect>, center_principal_point: bool) -> Result<core::Mat> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getOptimalNewCameraMatrix_const__InputArrayR_const__InputArrayR_Size_double_Size_RectX_bool(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &image_size, alpha, &new_img_size, valid_pix_roi.map_or(::core::ptr::null_mut(), |valid_pix_roi| valid_pix_roi), center_principal_point, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns the inscribed and bounding rectangles for the "undisorted" image plane.
///
/// The functions emulates undistortion of the image plane using the specified camera matrix,
/// distortion coefficients, the optional 3D rotation and the "new" camera matrix. In the case of
/// noticeable radial (or maybe pinclusion) distortion the rectangular image plane is distorted and
/// turns into some convex or concave shape. The function computes approximate inscribed (inner) and
/// bounding (outer) rectangles after such undistortion. The rectangles can be used to adjust
/// the newCameraMatrix so that the result image, for example, fits all the data from the original image
/// (at the expense of possibly big "black" areas) or, for another example, gets rid of black areas at the expense
/// some lost data near the original image edge. The function [get_optimal_new_camera_matrix] uses this function
/// to compute the optimal new camera matrix.
///
/// ## Parameters
/// * cameraMatrix: the original camera matrix.
/// * distCoeffs: distortion coefficients.
/// * R: the optional 3D rotation, applied before projection (see stereoRectify etc.)
/// * newCameraMatrix: the new camera matrix after undistortion. Usually it matches the original cameraMatrix.
/// * imgSize: the size of the image plane.
/// * inner: the output maximal inscribed rectangle of the undistorted image plane.
/// * outer: the output minimal bounding rectangle of the undistorted image plane.
// getUndistortRectangles(InputArray, InputArray, InputArray, InputArray, Size, Rect_<double> &, Rect_<double> &)(InputArray, InputArray, InputArray, InputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2380
// ("cv::getUndistortRectangles", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "imgSize", "inner", "outer"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "cv::Rect_<double>*", "cv::Rect_<double>*"]), _)]),
#[inline]
pub fn get_undistort_rectangles(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, r: &impl ToInputArray, new_camera_matrix: &impl ToInputArray, img_size: core::Size, inner: &mut core::Rect_<f64>, outer: &mut core::Rect_<f64>) -> Result<()> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(r);
	input_array_arg!(new_camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getUndistortRectangles_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_Rect_LdoubleGR_Rect_LdoubleGR(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), r.as_raw__InputArray(), new_camera_matrix.as_raw__InputArray(), &img_size, inner, outer, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the projection and inverse-rectification transformation map. In essense, this is the inverse of
/// [init_undistort_rectify_map] to accomodate stereo-rectification of projectors ('inverse-cameras') in projector-camera pairs.
///
/// The function computes the joint projection and inverse rectification transformation and represents the
/// result in the form of maps for #remap. The projected image looks like a distorted version of the original which,
/// once projected by a projector, should visually match the original. In case of a monocular camera, newCameraMatrix
/// is usually equal to cameraMatrix, or it can be computed by
/// [get_optimal_new_camera_matrix] for a better control over scaling. In case of a projector-camera pair,
/// newCameraMatrix is normally set to P1 or P2 computed by [stereo_rectify] .
///
/// The projector is oriented differently in the coordinate space, according to R. In case of projector-camera pairs,
/// this helps align the projector (in the same manner as [init_undistort_rectify_map] for the camera) to create a stereo-rectified pair. This
/// allows epipolar lines on both images to become horizontal and have the same y-coordinate (in case of a horizontally aligned projector-camera pair).
///
/// The function builds the maps for the inverse mapping algorithm that is used by #remap. That
/// is, for each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) in the destination (projected and inverse-rectified) image, the function
/// computes the corresponding coordinates in the source image (that is, in the original digital image). The following process is applied:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Barray%7D%7Bl%7D%0A%5Ctext%7BnewCameraMatrix%7D%5C%5C%0Ax%20%20%5Cleftarrow%20%28u%20%2D%20%7Bc%27%7D%5Fx%29%2F%7Bf%27%7D%5Fx%20%20%5C%5C%0Ay%20%20%5Cleftarrow%20%28v%20%2D%20%7Bc%27%7D%5Fy%29%2F%7Bf%27%7D%5Fy%20%20%5C%5C%0A%0A%5C%5C%5Ctext%7BUndistortion%7D%0A%5C%5C%5Cscriptsize%7B%5Ctextit%7Bthough%20equation%20shown%20is%20for%20radial%20undistortion%2C%20function%20implements%20cv%3A%3AundistortPoints%28%29%7D%7D%5C%5C%0Ar%5E2%20%20%5Cleftarrow%20x%5E2%20%2B%20y%5E2%20%5C%5C%0A%5Ctheta%20%5Cleftarrow%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%5C%5C%0Ax%27%20%5Cleftarrow%20%5Cfrac%7Bx%7D%7B%5Ctheta%7D%20%5C%5C%0Ay%27%20%20%5Cleftarrow%20%5Cfrac%7By%7D%7B%5Ctheta%7D%20%5C%5C%0A%0A%5C%5C%5Ctext%7BRectification%7D%5C%5C%0A%7B%5BX%5C%2CY%5C%2CW%5D%7D%20%5ET%20%20%5Cleftarrow%20R%2A%5Bx%27%20%5C%2C%20y%27%20%5C%2C%201%5D%5ET%20%20%5C%5C%0Ax%27%27%20%20%5Cleftarrow%20X%2FW%20%20%5C%5C%0Ay%27%27%20%20%5Cleftarrow%20Y%2FW%20%20%5C%5C%0A%0A%5C%5C%5Ctext%7BcameraMatrix%7D%5C%5C%0Amap%5Fx%28u%2Cv%29%20%20%5Cleftarrow%20x%27%27%20f%5Fx%20%2B%20c%5Fx%20%20%5C%5C%0Amap%5Fy%28u%2Cv%29%20%20%5Cleftarrow%20y%27%27%20f%5Fy%20%2B%20c%5Fy%0A%5Cend%7Barray%7D%0A)
/// where ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// are the distortion coefficients vector distCoeffs.
///
/// In case of a stereo-rectified projector-camera pair, this function is called for the projector while [init_undistort_rectify_map] is called for the camera head.
/// This is done after #stereoRectify, which in turn is called after #stereoCalibrate. If the projector-camera pair
/// is not calibrated, it is still possible to compute the rectification transformations directly from
/// the fundamental matrix using #stereoRectifyUncalibrated. For the projector and camera, the function computes
/// homography H as the rectification transformation in a pixel domain, not a rotation matrix R in 3D
/// space. R can be computed from H as
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BR%7D%20%3D%20%5Ctexttt%7BcameraMatrix%7D%20%5E%7B%2D1%7D%20%5Ccdot%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BcameraMatrix%7D)
/// where cameraMatrix can be chosen arbitrarily.
///
/// ## Parameters
/// * cameraMatrix: Input camera matrix ![inline formula](https://latex.codecogs.com/png.latex?A%3D%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * R: Optional rectification transformation in the object space (3x3 matrix). R1 or R2,
/// computed by [stereo_rectify] can be passed here. If the matrix is empty, the identity transformation
/// is assumed.
/// * newCameraMatrix: New camera matrix ![inline formula](https://latex.codecogs.com/png.latex?A%27%3D%5Cbegin%7Bbmatrix%7D%20f%5Fx%27%20%26%200%20%26%20c%5Fx%27%5C%5C%200%20%26%20f%5Fy%27%20%26%20c%5Fy%27%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * size: Distorted image size.
/// * m1type: Type of the first output map. Can be CV_32FC1, CV_32FC2 or CV_16SC2, see [convert_maps]
/// * map1: The first output map for #remap.
/// * map2: The second output map for #remap.
// initInverseRectificationMap(InputArray, InputArray, InputArray, InputArray, const Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2314
// ("cv::initInverseRectificationMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn init_inverse_rectification_map(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, r: &impl ToInputArray, new_camera_matrix: &impl ToInputArray, size: core::Size, m1type: i32, map1: &mut impl ToOutputArray, map2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(r);
	input_array_arg!(new_camera_matrix);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_initInverseRectificationMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), r.as_raw__InputArray(), new_camera_matrix.as_raw__InputArray(), &size, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the undistortion and rectification transformation map.
///
/// The function computes the joint undistortion and rectification transformation and represents the
/// result in the form of maps for #remap. The undistorted image looks like original, as if it is
/// captured with a camera using the camera matrix =newCameraMatrix and zero distortion. In case of a
/// monocular camera, newCameraMatrix is usually equal to cameraMatrix, or it can be computed by
/// [get_optimal_new_camera_matrix] for a better control over scaling. In case of a stereo camera,
/// newCameraMatrix is normally set to P1 or P2 computed by [stereo_rectify] .
///
/// Also, this new camera is oriented differently in the coordinate space, according to R. That, for
/// example, helps to align two heads of a stereo camera so that the epipolar lines on both images
/// become horizontal and have the same y- coordinate (in case of a horizontally aligned stereo camera).
///
/// The function actually builds the maps for the inverse mapping algorithm that is used by #remap. That
/// is, for each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) in the destination (corrected and rectified) image, the function
/// computes the corresponding coordinates in the source image (that is, in the original image from
/// camera). The following process is applied:
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Barray%7D%7Bl%7D%0Ax%20%20%5Cleftarrow%20%28u%20%2D%20%7Bc%27%7D%5Fx%29%2F%7Bf%27%7D%5Fx%20%20%5C%5C%0Ay%20%20%5Cleftarrow%20%28v%20%2D%20%7Bc%27%7D%5Fy%29%2F%7Bf%27%7D%5Fy%20%20%5C%5C%0A%7B%5BX%5C%2CY%5C%2CW%5D%7D%20%5ET%20%20%5Cleftarrow%20R%5E%7B%2D1%7D%2A%5Bx%20%5C%2C%20y%20%5C%2C%201%5D%5ET%20%20%5C%5C%0Ax%27%20%20%5Cleftarrow%20X%2FW%20%20%5C%5C%0Ay%27%20%20%5Cleftarrow%20Y%2FW%20%20%5C%5C%0Ar%5E2%20%20%5Cleftarrow%20x%27%5E2%20%2B%20y%27%5E2%20%5C%5C%0Ax%27%27%20%20%5Cleftarrow%20x%27%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%0A%2B%202p%5F1%20x%27%20y%27%20%2B%20p%5F2%28r%5E2%20%2B%202%20x%27%5E2%29%20%20%2B%20s%5F1%20r%5E2%20%2B%20s%5F2%20r%5E4%5C%5C%0Ay%27%27%20%20%5Cleftarrow%20y%27%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%0A%2B%20p%5F1%20%28r%5E2%20%2B%202%20y%27%5E2%29%20%2B%202%20p%5F2%20x%27%20y%27%20%2B%20s%5F3%20r%5E2%20%2B%20s%5F4%20r%5E4%20%5C%5C%0As%5Cbegin%7Bbmatrix%7D%20x%27%27%27%5C%5C%20y%27%27%27%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%0A%5Cvecthreethree%7BR%5F%7B33%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%7B0%7D%7B%2DR%5F%7B13%7D%28%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%0A%7B0%7D%7BR%5F%7B33%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%7B%2DR%5F%7B23%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%0A%7B0%7D%7B0%7D%7B1%7D%20R%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%20%5Cbegin%7Bbmatrix%7D%20x%27%27%5C%5C%20y%27%27%5C%5C%201%20%5Cend%7Bbmatrix%7D%5C%5C%0Amap%5Fx%28u%2Cv%29%20%20%5Cleftarrow%20x%27%27%27%20f%5Fx%20%2B%20c%5Fx%20%20%5C%5C%0Amap%5Fy%28u%2Cv%29%20%20%5Cleftarrow%20y%27%27%27%20f%5Fy%20%2B%20c%5Fy%0A%5Cend%7Barray%7D%0A)
/// where ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// are the distortion coefficients.
///
/// In case of a stereo camera, this function is called twice: once for each camera head, after
/// #stereoRectify, which in its turn is called after #stereoCalibrate. But if the stereo camera
/// was not calibrated, it is still possible to compute the rectification transformations directly from
/// the fundamental matrix using #stereoRectifyUncalibrated. For each camera, the function computes
/// homography H as the rectification transformation in a pixel domain, not a rotation matrix R in 3D
/// space. R can be computed from H as
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BR%7D%20%3D%20%5Ctexttt%7BcameraMatrix%7D%20%5E%7B%2D1%7D%20%5Ccdot%20%5Ctexttt%7BH%7D%20%5Ccdot%20%5Ctexttt%7BcameraMatrix%7D)
/// where cameraMatrix can be chosen arbitrarily.
///
/// ## Parameters
/// * cameraMatrix: Input camera matrix ![inline formula](https://latex.codecogs.com/png.latex?A%3D%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * R: Optional rectification transformation in the object space (3x3 matrix). R1 or R2 ,
/// computed by [stereo_rectify] can be passed here. If the matrix is empty, the identity transformation
/// is assumed. In [init_undistort_rectify_map] R assumed to be an identity matrix.
/// * newCameraMatrix: New camera matrix ![inline formula](https://latex.codecogs.com/png.latex?A%27%3D%5Cbegin%7Bbmatrix%7D%20f%5Fx%27%20%26%200%20%26%20c%5Fx%27%5C%5C%200%20%26%20f%5Fy%27%20%26%20c%5Fy%27%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * size: Undistorted image size.
/// * m1type: Type of the first output map that can be CV_32FC1, CV_32FC2 or CV_16SC2, see [convert_maps]
/// * map1: The first output map.
/// * map2: The second output map.
// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, Size, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2243
// ("cv::initUndistortRectifyMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "R", "newCameraMatrix", "size", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn init_undistort_rectify_map(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, r: &impl ToInputArray, new_camera_matrix: &impl ToInputArray, size: core::Size, m1type: i32, map1: &mut impl ToOutputArray, map2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(r);
	input_array_arg!(new_camera_matrix);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_int_const__OutputArrayR_const__OutputArrayR(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), r.as_raw__InputArray(), new_camera_matrix.as_raw__InputArray(), &size, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// initializes maps for [remap] for wide-angle
///
/// ## Note
/// This alternative version of [init_wide_angle_proj_map] function uses the following default values for its arguments:
/// * proj_type: PROJ_SPHERICAL_EQRECT
/// * alpha: 0
// cv::initWideAngleProjMap(InputArray, InputArray, SimpleClass, Primitive, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2320
// ("cv::initWideAngleProjMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "destImageWidth", "m1type", "map1", "map2"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn init_wide_angle_proj_map_def(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_size: core::Size, dest_image_width: i32, m1type: i32, map1: &mut impl ToOutputArray, map2: &mut impl ToOutputArray) -> Result<f32> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &image_size, dest_image_width, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// initializes maps for [remap] for wide-angle
///
/// ## C++ default parameters
/// * proj_type: PROJ_SPHERICAL_EQRECT
/// * alpha: 0
// initWideAngleProjMap(InputArray, InputArray, Size, int, int, OutputArray, OutputArray, enum UndistortTypes, double)(InputArray, InputArray, SimpleClass, Primitive, Primitive, OutputArray, OutputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2320
// ("cv::initWideAngleProjMap", vec![(pred!(mut, ["cameraMatrix", "distCoeffs", "imageSize", "destImageWidth", "m1type", "map1", "map2", "projType", "alpha"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::UndistortTypes", "double"]), _)]),
#[inline]
pub fn init_wide_angle_proj_map(camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_size: core::Size, dest_image_width: i32, m1type: i32, map1: &mut impl ToOutputArray, map2: &mut impl ToOutputArray, proj_type: crate::mod_3d::UndistortTypes, alpha: f64) -> Result<f32> {
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_initWideAngleProjMap_const__InputArrayR_const__InputArrayR_Size_int_int_const__OutputArrayR_const__OutputArrayR_UndistortTypes_double(camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &image_size, dest_image_width, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), proj_type, alpha, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Loads a mesh from a file.
///
/// The function loads mesh from the specified file and returns it.
/// If the mesh cannot be read, throws an error
/// Vertex attributes (i.e. space and texture coodinates, normals and colors) are returned in same-sized
/// arrays with corresponding elements having the same indices.
/// This means that if a face uses a vertex with a normal or a texture coordinate with different indices
/// (which is typical for OBJ files for example), this vertex will be duplicated for each face it uses.
///
/// Currently, the following file formats are supported:
/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file) (ONLY TRIANGULATED FACES)
/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
/// ## Parameters
/// * filename: Name of the file
/// * vertices: vertex coordinates, each value contains 3 floats
/// * indices: per-face list of vertices, each value is a vector of ints
/// * normals: per-vertex normals, each value contains 3 floats
/// * colors: per-vertex colors, each value contains 3 floats
/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
///
/// ## Note
/// This alternative version of [load_mesh] function uses the following default values for its arguments:
/// * normals: noArray()
/// * colors: noArray()
/// * tex_coords: noArray()
// cv::loadMesh(InString, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2894
// ("cv::loadMesh", vec![(pred!(mut, ["filename", "vertices", "indices"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn load_mesh_def(filename: &str, vertices: &mut impl ToOutputArray, indices: &mut impl ToOutputArray) -> Result<()> {
	extern_container_arg!(filename);
	output_array_arg!(vertices);
	output_array_arg!(indices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), indices.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Loads a mesh from a file.
///
/// The function loads mesh from the specified file and returns it.
/// If the mesh cannot be read, throws an error
/// Vertex attributes (i.e. space and texture coodinates, normals and colors) are returned in same-sized
/// arrays with corresponding elements having the same indices.
/// This means that if a face uses a vertex with a normal or a texture coordinate with different indices
/// (which is typical for OBJ files for example), this vertex will be duplicated for each face it uses.
///
/// Currently, the following file formats are supported:
/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file) (ONLY TRIANGULATED FACES)
/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
/// ## Parameters
/// * filename: Name of the file
/// * vertices: vertex coordinates, each value contains 3 floats
/// * indices: per-face list of vertices, each value is a vector of ints
/// * normals: per-vertex normals, each value contains 3 floats
/// * colors: per-vertex colors, each value contains 3 floats
/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
///
/// ## C++ default parameters
/// * normals: noArray()
/// * colors: noArray()
/// * tex_coords: noArray()
// loadMesh(const String &, OutputArray, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray)(InString, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2894
// ("cv::loadMesh", vec![(pred!(mut, ["filename", "vertices", "indices", "normals", "colors", "texCoords"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn load_mesh(filename: &str, vertices: &mut impl ToOutputArray, indices: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray, tex_coords: &mut impl ToOutputArray) -> Result<()> {
	extern_container_arg!(filename);
	output_array_arg!(vertices);
	output_array_arg!(indices);
	output_array_arg!(normals);
	output_array_arg!(colors);
	output_array_arg!(tex_coords);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), indices.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), tex_coords.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Loads a point cloud from a file.
///
/// The function loads point cloud from the specified file and returns it.
/// If the cloud cannot be read, throws an error.
/// Vertex coordinates, normals and colors are returned as they are saved in the file
/// even if these arrays have different sizes and their elements do not correspond to each other
/// (which is typical for OBJ files for example)
///
/// Currently, the following file formats are supported:
/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
///
/// ## Parameters
/// * filename: Name of the file
/// * vertices: vertex coordinates, each value contains 3 floats
/// * normals: per-vertex normals, each value contains 3 floats
/// * rgb: per-vertex colors, each value contains 3 floats
///
/// ## Note
/// This alternative version of [load_point_cloud] function uses the following default values for its arguments:
/// * normals: noArray()
/// * rgb: noArray()
// cv::loadPointCloud(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2861
// ("cv::loadPointCloud", vec![(pred!(mut, ["filename", "vertices"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn load_point_cloud_def(filename: &str, vertices: &mut impl ToOutputArray) -> Result<()> {
	extern_container_arg!(filename);
	output_array_arg!(vertices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_loadPointCloud_const_StringR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Loads a point cloud from a file.
///
/// The function loads point cloud from the specified file and returns it.
/// If the cloud cannot be read, throws an error.
/// Vertex coordinates, normals and colors are returned as they are saved in the file
/// even if these arrays have different sizes and their elements do not correspond to each other
/// (which is typical for OBJ files for example)
///
/// Currently, the following file formats are supported:
/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
///
/// ## Parameters
/// * filename: Name of the file
/// * vertices: vertex coordinates, each value contains 3 floats
/// * normals: per-vertex normals, each value contains 3 floats
/// * rgb: per-vertex colors, each value contains 3 floats
///
/// ## C++ default parameters
/// * normals: noArray()
/// * rgb: noArray()
// loadPointCloud(const String &, OutputArray, OutputArray, OutputArray)(InString, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2861
// ("cv::loadPointCloud", vec![(pred!(mut, ["filename", "vertices", "normals", "rgb"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn load_point_cloud(filename: &str, vertices: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, rgb: &mut impl ToOutputArray) -> Result<()> {
	extern_container_arg!(filename);
	output_array_arg!(vertices);
	output_array_arg!(normals);
	output_array_arg!(rgb);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_loadPointCloud_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), normals.as_raw__OutputArray(), rgb.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes partial derivatives of the matrix product for each multiplied matrix.
///
/// ## Parameters
/// * A: First multiplied matrix.
/// * B: Second multiplied matrix.
/// * dABdA: First output derivative matrix d(A\*B)/dA of size
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BA%2Erows%2AB%2Ecols%7D%20%5Ctimes%20%7BA%2Erows%2AA%2Ecols%7D) .
/// * dABdB: Second output derivative matrix d(A\*B)/dB of size
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BA%2Erows%2AB%2Ecols%7D%20%5Ctimes%20%7BB%2Erows%2AB%2Ecols%7D) .
///
/// The function computes partial derivatives of the elements of the matrix product ![inline formula](https://latex.codecogs.com/png.latex?A%2AB) with regard to
/// the elements of each of the two input matrices. The function is used to compute the Jacobian
/// matrices in [stereo_calibrate] but can also be used in any other similar optimization function.
// matMulDeriv(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:881
// ("cv::matMulDeriv", vec![(pred!(mut, ["A", "B", "dABdA", "dABdB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn mat_mul_deriv(a: &impl ToInputArray, b: &impl ToInputArray, d_a_bd_a: &mut impl ToOutputArray, d_a_bd_b: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(a);
	input_array_arg!(b);
	output_array_arg!(d_a_bd_a);
	output_array_arg!(d_a_bd_b);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_matMulDeriv_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(a.as_raw__InputArray(), b.as_raw__InputArray(), d_a_bd_a.as_raw__OutputArray(), d_a_bd_b.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate the normal and curvature of each point in point cloud from NN results.
///
/// Normal estimation by PCA:
/// + Input: Nearest neighbor points of a specific point: ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20)
/// + Step:
///    1. Calculate the ![inline formula](https://latex.codecogs.com/png.latex?%20mean%28%5Cbar%7Bx%7D%2C%5Cbar%7By%7D%2C%5Cbar%7Bz%7D%29%20) of ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20);
///    2. A 3x3 covariance matrix ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20) is obtained by ![inline formula](https://latex.codecogs.com/png.latex?%20mean%5ET%20%5Ccdot%20mean%20);
///    3. Calculate the eigenvalues(![inline formula](https://latex.codecogs.com/png.latex?%20%CE%BB%5F2%20%5Cge%20%CE%BB%5F1%20%5Cge%20%CE%BB%5F0%20)) and corresponding
///        eigenvectors(![inline formula](https://latex.codecogs.com/png.latex?%20v%5F2%2C%20v%5F1%2C%20v%5F0%20)) of ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20);
///    4. ![inline formula](https://latex.codecogs.com/png.latex?%20v0%20) is the normal of the specific point,
///        ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B%CE%BB%5F0%7D%7B%CE%BB%5F0%20%2B%20%CE%BB%5F1%20%2B%20%CE%BB%5F2%7D%20) is the curvature of the specific point;
/// + Output: Normal and curvature of the specific point.
///
/// ## Parameters
/// * normals:[out] Normal of each point, support vector<Point3f> and Mat of size Nx3.
/// * curvatures:[out] Curvature of each point, support vector<float> and Mat.
/// * input_pts: Original point cloud, support vector<Point3f> and Mat of size Nx3/3xN.
/// * nn_idx: Index information of nearest neighbors of all points. The first nearest neighbor of
///               each point is itself. Support vector<vector<int>>, vector<Mat> and Mat of size NxK.
///               If the information in a row is [0, 2, 1, -5, -1, 4, 7 ... negative number], it will
///               use only non-negative indexes until it meets a negative number or bound of this row
///               i.e. [0, 2, 1].
/// * max_neighbor_num: The maximum number of neighbors want to use including itself. Setting to
///               a non-positive number or default will use the information from nn_idx.
///
/// ## Note
/// This alternative version of [normal_estimate] function uses the following default values for its arguments:
/// * max_neighbor_num: 0
// cv::normalEstimate(OutputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:288
// ("cv::normalEstimate", vec![(pred!(mut, ["normals", "curvatures", "input_pts", "nn_idx"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn normal_estimate_def(normals: &mut impl ToOutputArray, curvatures: &mut impl ToOutputArray, input_pts: &impl ToInputArray, nn_idx: &impl ToInputArray) -> Result<()> {
	output_array_arg!(normals);
	output_array_arg!(curvatures);
	input_array_arg!(input_pts);
	input_array_arg!(nn_idx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(normals.as_raw__OutputArray(), curvatures.as_raw__OutputArray(), input_pts.as_raw__InputArray(), nn_idx.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate the normal and curvature of each point in point cloud from NN results.
///
/// Normal estimation by PCA:
/// + Input: Nearest neighbor points of a specific point: ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20)
/// + Step:
///    1. Calculate the ![inline formula](https://latex.codecogs.com/png.latex?%20mean%28%5Cbar%7Bx%7D%2C%5Cbar%7By%7D%2C%5Cbar%7Bz%7D%29%20) of ![inline formula](https://latex.codecogs.com/png.latex?%20pt%5C%5Fset%20);
///    2. A 3x3 covariance matrix ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20) is obtained by ![inline formula](https://latex.codecogs.com/png.latex?%20mean%5ET%20%5Ccdot%20mean%20);
///    3. Calculate the eigenvalues(![inline formula](https://latex.codecogs.com/png.latex?%20%CE%BB%5F2%20%5Cge%20%CE%BB%5F1%20%5Cge%20%CE%BB%5F0%20)) and corresponding
///        eigenvectors(![inline formula](https://latex.codecogs.com/png.latex?%20v%5F2%2C%20v%5F1%2C%20v%5F0%20)) of ![inline formula](https://latex.codecogs.com/png.latex?%20cov%20);
///    4. ![inline formula](https://latex.codecogs.com/png.latex?%20v0%20) is the normal of the specific point,
///        ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B%CE%BB%5F0%7D%7B%CE%BB%5F0%20%2B%20%CE%BB%5F1%20%2B%20%CE%BB%5F2%7D%20) is the curvature of the specific point;
/// + Output: Normal and curvature of the specific point.
///
/// ## Parameters
/// * normals:[out] Normal of each point, support vector<Point3f> and Mat of size Nx3.
/// * curvatures:[out] Curvature of each point, support vector<float> and Mat.
/// * input_pts: Original point cloud, support vector<Point3f> and Mat of size Nx3/3xN.
/// * nn_idx: Index information of nearest neighbors of all points. The first nearest neighbor of
///               each point is itself. Support vector<vector<int>>, vector<Mat> and Mat of size NxK.
///               If the information in a row is [0, 2, 1, -5, -1, 4, 7 ... negative number], it will
///               use only non-negative indexes until it meets a negative number or bound of this row
///               i.e. [0, 2, 1].
/// * max_neighbor_num: The maximum number of neighbors want to use including itself. Setting to
///               a non-positive number or default will use the information from nn_idx.
///
/// ## C++ default parameters
/// * max_neighbor_num: 0
// normalEstimate(OutputArray, OutputArray, InputArray, InputArrayOfArrays, int)(OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:288
// ("cv::normalEstimate", vec![(pred!(mut, ["normals", "curvatures", "input_pts", "nn_idx", "max_neighbor_num"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn normal_estimate(normals: &mut impl ToOutputArray, curvatures: &mut impl ToOutputArray, input_pts: &impl ToInputArray, nn_idx: &impl ToInputArray, max_neighbor_num: i32) -> Result<()> {
	output_array_arg!(normals);
	output_array_arg!(curvatures);
	input_array_arg!(input_pts);
	input_array_arg!(nn_idx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_normalEstimate_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(normals.as_raw__OutputArray(), curvatures.as_raw__OutputArray(), input_pts.as_raw__InputArray(), nn_idx.as_raw__InputArray(), max_neighbor_num, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Projects 3D points to an image plane.
///
/// ## Parameters
/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
/// basis from world to camera coordinate system, see [calibrateCamera] for details.
/// * tvec: The translation vector, see parameter description above.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
/// * imagePoints: Output array of image points, 1xN/Nx1 2-channel, or
/// vector\<Point2f\> .
/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
/// points with respect to components of the rotation vector, translation vector, focal lengths,
/// coordinates of the principal point and the distortion coefficients. In the old interface different
/// components of the jacobian are returned via different output parameters.
/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
/// jacobian matrix.
///
/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
/// parameters.
///
///
/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
/// or by passing zero distortion coefficients, one can get various useful partial cases of the
/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
///
/// ## Note
/// This alternative version of [project_points] function uses the following default values for its arguments:
/// * jacobian: noArray()
/// * aspect_ratio: 0
// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:953
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn project_points_def(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(image_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [project_points_sep_j] function uses the following default values for its arguments:
/// * dpdf: noArray()
/// * dpdc: noArray()
/// * dpdk: noArray()
/// * dpdo: noArray()
/// * aspect_ratio: 0.
// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:961
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "dpdr", "dpdt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn project_points_sep_j_def(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray, dpdr: &mut impl ToOutputArray, dpdt: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(image_points);
	output_array_arg!(dpdr);
	output_array_arg!(dpdt);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), dpdr.as_raw__OutputArray(), dpdt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Projects 3D points to an image plane.
///
/// ## Parameters
/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
/// basis from world to camera coordinate system, see [calibrateCamera] for details.
/// * tvec: The translation vector, see parameter description above.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
/// * imagePoints: Output array of image points, 1xN/Nx1 2-channel, or
/// vector\<Point2f\> .
/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
/// points with respect to components of the rotation vector, translation vector, focal lengths,
/// coordinates of the principal point and the distortion coefficients. In the old interface different
/// components of the jacobian are returned via different output parameters.
/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
/// jacobian matrix.
///
/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
/// parameters.
///
///
/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
/// or by passing zero distortion coefficients, one can get various useful partial cases of the
/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * dpdf: noArray()
/// * dpdc: noArray()
/// * dpdk: noArray()
/// * dpdo: noArray()
/// * aspect_ratio: 0.
// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:961
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "dpdr", "dpdt", "dpdf", "dpdc", "dpdk", "dpdo", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn project_points_sep_j(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray, dpdr: &mut impl ToOutputArray, dpdt: &mut impl ToOutputArray, dpdf: &mut impl ToOutputArray, dpdc: &mut impl ToOutputArray, dpdk: &mut impl ToOutputArray, dpdo: &mut impl ToOutputArray, aspect_ratio: f64) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(image_points);
	output_array_arg!(dpdr);
	output_array_arg!(dpdt);
	output_array_arg!(dpdf);
	output_array_arg!(dpdc);
	output_array_arg!(dpdk);
	output_array_arg!(dpdo);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), dpdr.as_raw__OutputArray(), dpdt.as_raw__OutputArray(), dpdf.as_raw__OutputArray(), dpdc.as_raw__OutputArray(), dpdk.as_raw__OutputArray(), dpdo.as_raw__OutputArray(), aspect_ratio, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Projects 3D points to an image plane.
///
/// ## Parameters
/// * objectPoints: Array of object points expressed wrt. the world coordinate frame. A 3xN/Nx3
/// 1-channel or 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is the number of points in the view.
/// * rvec: The rotation vector ([Rodrigues]) that, together with tvec, performs a change of
/// basis from world to camera coordinate system, see [calibrateCamera] for details.
/// * tvec: The translation vector, see parameter description above.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs) . If the vector is empty, the zero distortion coefficients are assumed.
/// * imagePoints: Output array of image points, 1xN/Nx1 2-channel, or
/// vector\<Point2f\> .
/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
/// points with respect to components of the rotation vector, translation vector, focal lengths,
/// coordinates of the principal point and the distortion coefficients. In the old interface different
/// components of the jacobian are returned via different output parameters.
/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
/// function assumes that the aspect ratio (![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%20%2F%20f%5Fy)) is fixed and correspondingly adjusts the
/// jacobian matrix.
///
/// The function computes the 2D projections of 3D points to the image plane, given intrinsic and
/// extrinsic camera parameters. Optionally, the function computes Jacobians -matrices of partial
/// derivatives of image points coordinates (as functions of all the input parameters) with respect to
/// the particular parameters, intrinsic and/or extrinsic. The Jacobians are used during the global
/// optimization in [calibrateCamera], [solvePnP], and [stereoCalibrate]. The function itself
/// can also be used to compute a re-projection error, given the current intrinsic and extrinsic
/// parameters.
///
///
/// Note: By setting rvec = tvec = ![inline formula](https://latex.codecogs.com/png.latex?%5B0%2C%200%2C%200%5D), or by setting cameraMatrix to a 3x3 identity matrix,
/// or by passing zero distortion coefficients, one can get various useful partial cases of the
/// function. This means, one can compute the distorted coordinates for a sparse set of points or apply
/// a perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
///
/// ## C++ default parameters
/// * jacobian: noArray()
/// * aspect_ratio: 0
// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:953
// ("cv::projectPoints", vec![(pred!(mut, ["objectPoints", "rvec", "tvec", "cameraMatrix", "distCoeffs", "imagePoints", "jacobian", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn project_points(object_points: &impl ToInputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, image_points: &mut impl ToOutputArray, jacobian: &mut impl ToOutputArray, aspect_ratio: f64) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(image_points);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_projectPoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(object_points.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), image_points.as_raw__OutputArray(), jacobian.as_raw__OutputArray(), aspect_ratio, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Parameters
/// * sampled_pts: Point cloud after sampling.
///                    Support cv::Mat(size * sampled_scale, 3, CV_32F), std::vector<cv::Point3f>.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
///                      that is, sampled size = original size * sampled_scale.
/// * rng: Optional random number generator used for cv::randShuffle;
///                      if it is nullptr, theRNG () is used instead.
///
/// ## Note
/// This alternative version of [random_sampling_1] function uses the following default values for its arguments:
/// * rng: nullptr
// cv::randomSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:215
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_scale"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float"]), _)]),
#[inline]
pub fn random_sampling_1_def(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32) -> Result<()> {
	output_array_arg!(sampled_pts);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_float(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by randomly select points.
///
/// Use cv::randShuffle to shuffle the point index list,
/// then take the points corresponding to the front part of the list.
///
/// ## Parameters
/// * sampled_pts: Point cloud after sampling.
///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_pts_size: The desired point cloud size after sampling.
/// * rng: Optional random number generator used for cv::randShuffle;
///                      if it is nullptr, theRNG () is used instead.
///
/// ## Overloaded parameters
///
///
/// * sampled_pts: Point cloud after sampling.
///                    Support cv::Mat(size * sampled_scale, 3, CV_32F), std::vector<cv::Point3f>.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_scale: Range (0, 1), the percentage of the sampled point cloud to the original size,
///                      that is, sampled size = original size * sampled_scale.
/// * rng: Optional random number generator used for cv::randShuffle;
///                      if it is nullptr, theRNG () is used instead.
///
/// ## C++ default parameters
/// * rng: nullptr
// randomSampling(OutputArray, InputArray, float, RNG *)(OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:215
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_scale", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "cv::RNG*"]), _)]),
#[inline]
pub fn random_sampling_1(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_scale: f32, rng: &mut impl core::RNGTrait) -> Result<()> {
	output_array_arg!(sampled_pts);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_float_RNGX(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_scale, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by randomly select points.
///
/// Use cv::randShuffle to shuffle the point index list,
/// then take the points corresponding to the front part of the list.
///
/// ## Parameters
/// * sampled_pts: Point cloud after sampling.
///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_pts_size: The desired point cloud size after sampling.
/// * rng: Optional random number generator used for cv::randShuffle;
///                      if it is nullptr, theRNG () is used instead.
///
/// ## Note
/// This alternative version of [random_sampling] function uses the following default values for its arguments:
/// * rng: nullptr
// cv::randomSampling(OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:201
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_pts_size"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn random_sampling_def(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32) -> Result<()> {
	output_array_arg!(sampled_pts);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_int(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by randomly select points.
///
/// Use cv::randShuffle to shuffle the point index list,
/// then take the points corresponding to the front part of the list.
///
/// ## Parameters
/// * sampled_pts: Point cloud after sampling.
///                    Support cv::Mat(sampled_pts_size, 3, CV_32F), std::vector<cv::Point3f>.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * sampled_pts_size: The desired point cloud size after sampling.
/// * rng: Optional random number generator used for cv::randShuffle;
///                      if it is nullptr, theRNG () is used instead.
///
/// ## C++ default parameters
/// * rng: nullptr
// randomSampling(OutputArray, InputArray, int, RNG *)(OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:201
// ("cv::randomSampling", vec![(pred!(mut, ["sampled_pts", "input_pts", "sampled_pts_size", "rng"], ["const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::RNG*"]), _)]),
#[inline]
pub fn random_sampling(sampled_pts: &mut impl ToOutputArray, input_pts: &impl ToInputArray, sampled_pts_size: i32, rng: &mut impl core::RNGTrait) -> Result<()> {
	output_array_arg!(sampled_pts);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_randomSampling_const__OutputArrayR_const__InputArrayR_int_RNGX(sampled_pts.as_raw__OutputArray(), input_pts.as_raw__InputArray(), sampled_pts_size, rng.as_raw_mut_RNG(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Recovers the relative camera rotation and the translation from corresponding points in two images from two different cameras, using chirality check. Returns the number of
/// inliers that pass the check.
///
/// ## Parameters
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix1: Input/output camera matrix for the first camera, the same as in
/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
/// [calibrateCamera].
/// * cameraMatrix2: Input/output camera matrix for the first camera, the same as in
/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
/// * distCoeffs2: Input/output vector of distortion coefficients, the same as in
/// [calibrateCamera].
/// * E: The output essential matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// described below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
/// possible pose hypotheses by doing chirality check. The chirality check means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
///
/// This function can be used to process the output E and mask from [findEssentialMat]. In this
/// scenario, points1 and points2 are the same input for findEssentialMat.:
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    // Input: camera calibration of both cameras, for example using intrinsic chessboard calibration.
///    Mat cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2;
///
///    // Output: Essential matrix, relative rotation and relative translation.
///    Mat E, R, t, mask;
///
///    recoverPose(points1, points2, cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2, E, R, t, mask);
/// ```
///
///
/// ## Note
/// This alternative version of [recover_pose_2_cameras] function uses the following default values for its arguments:
/// * method: cv::RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1619
// ("cv::recoverPose", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "E", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn recover_pose_2_cameras_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, e: &mut impl ToOutputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	output_array_arg!(e);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), e.as_raw__OutputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Recovers the relative camera rotation and the translation from corresponding points in two images from two different cameras, using chirality check. Returns the number of
/// inliers that pass the check.
///
/// ## Parameters
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix1: Input/output camera matrix for the first camera, the same as in
/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
/// * distCoeffs1: Input/output vector of distortion coefficients, the same as in
/// [calibrateCamera].
/// * cameraMatrix2: Input/output camera matrix for the first camera, the same as in
/// [calibrateCamera]. Furthermore, for the stereo case, additional flags may be used, see below.
/// * distCoeffs2: Input/output vector of distortion coefficients, the same as in
/// [calibrateCamera].
/// * E: The output essential matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// described below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * method: Method for computing an essential matrix.
/// *   [RANSAC] for the RANSAC algorithm.
/// *   [LMEDS] for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
/// possible pose hypotheses by doing chirality check. The chirality check means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
///
/// This function can be used to process the output E and mask from [findEssentialMat]. In this
/// scenario, points1 and points2 are the same input for findEssentialMat.:
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    // Input: camera calibration of both cameras, for example using intrinsic chessboard calibration.
///    Mat cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2;
///
///    // Output: Essential matrix, relative rotation and relative translation.
///    Mat E, R, t, mask;
///
///    recoverPose(points1, points2, cameraMatrix1, distCoeffs1, cameraMatrix2, distCoeffs2, E, R, t, mask);
/// ```
///
///
/// ## C++ default parameters
/// * method: cv::RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// recoverPose(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, int, double, double, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1619
// ("cv::recoverPose", vec![(pred!(mut, ["points1", "points2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "E", "R", "t", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn recover_pose_2_cameras(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, e: &mut impl ToOutputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, method: i32, prob: f64, threshold: f64, mask: &mut impl ToInputOutputArray) -> Result<i32> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	output_array_arg!(e);
	output_array_arg!(r);
	output_array_arg!(t);
	input_output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_double_const__InputOutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), e.as_raw__OutputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), method, prob, threshold, mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Recovers the relative camera rotation and the translation from an estimated essential
/// matrix and the corresponding points in two images, using chirality check. Returns the number of
/// inliers that pass the check.
///
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// described below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
/// possible pose hypotheses by doing chirality check. The chirality check means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
///
/// This function can be used to process the output E and mask from [findEssentialMat]. In this
/// scenario, points1 and points2 are the same input for [find_essential_mat] :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
///
///    Mat E, R, t, mask;
///
///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
///
///
/// ## Note
/// This alternative version of [recover_pose_estimated] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1676
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn recover_pose_estimated_def(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(e);
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Recovers the relative camera rotation and the translation from an estimated essential
/// matrix and the corresponding points in two images, using chirality check. Returns the number of
/// inliers that pass the check.
///
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// described below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
/// possible pose hypotheses by doing chirality check. The chirality check means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
///
/// This function can be used to process the output E and mask from [findEssentialMat]. In this
/// scenario, points1 and points2 are the same input for [find_essential_mat] :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
///
///    Mat E, R, t, mask;
///
///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
///
///
/// ## C++ default parameters
/// * mask: noArray()
// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1676
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn recover_pose_estimated(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, mask: &mut impl ToInputOutputArray) -> Result<i32> {
	input_array_arg!(e);
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	output_array_arg!(r);
	output_array_arg!(t);
	input_output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// description below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * distanceThresh: threshold distance which is used to filter out far away points (i.e. infinite
/// points).
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
/// * triangulatedPoints: 3D points which were reconstructed by triangulation.
///
/// This function differs from the one above that it outputs the triangulated 3D point that are used for
/// the chirality check.
///
/// ## Note
/// This alternative version of [recover_pose_triangulated] function uses the following default values for its arguments:
/// * mask: noArray()
/// * triangulated_points: noArray()
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1739
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn recover_pose_triangulated_def(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, distance_thresh: f64) -> Result<i32> {
	input_array_arg!(e);
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), distance_thresh, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Recovers the relative camera rotation and the translation from an estimated essential
/// matrix and the corresponding points in two images, using chirality check. Returns the number of
/// inliers that pass the check.
///
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// described below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
/// possible pose hypotheses by doing chirality check. The chirality check means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
///
/// This function can be used to process the output E and mask from [findEssentialMat]. In this
/// scenario, points1 and points2 are the same input for [find_essential_mat] :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
///
///    Mat E, R, t, mask;
///
///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
///
///
/// ## Overloaded parameters
///
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// description below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * distanceThresh: threshold distance which is used to filter out far away points (i.e. infinite
/// points).
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
/// * triangulatedPoints: 3D points which were reconstructed by triangulation.
///
/// This function differs from the one above that it outputs the triangulated 3D point that are used for
/// the chirality check.
///
/// ## C++ default parameters
/// * mask: noArray()
/// * triangulated_points: noArray()
// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double, InputOutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1739
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "cameraMatrix", "R", "t", "distanceThresh", "mask", "triangulatedPoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn recover_pose_triangulated(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, distance_thresh: f64, mask: &mut impl ToInputOutputArray, triangulated_points: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(e);
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	output_array_arg!(r);
	output_array_arg!(t);
	input_output_array_arg!(mask);
	output_array_arg!(triangulated_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_const__InputOutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), distance_thresh, mask.as_raw__InputOutputArray(), triangulated_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// description below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * focal: Focal length of the camera. Note that this function assumes that points1 and points2
/// are feature points from cameras with same focal length and principal point.
/// * pp: principal point of the camera.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
/// principal point:
///
/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
///
/// ## Note
/// This alternative version of [recover_pose] function uses the following default values for its arguments:
/// * focal: 1.0
/// * pp: Point2d(0,0)
/// * mask: noArray()
// cv::recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1709
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn recover_pose_def(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(e);
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Recovers the relative camera rotation and the translation from an estimated essential
/// matrix and the corresponding points in two images, using chirality check. Returns the number of
/// inliers that pass the check.
///
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix.
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// described below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function decomposes an essential matrix using [decomposeEssentialMat] and then verifies
/// possible pose hypotheses by doing chirality check. The chirality check means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Nister03).
///
/// This function can be used to process the output E and mask from [findEssentialMat]. In this
/// scenario, points1 and points2 are the same input for [find_essential_mat] :
/// ```C++
///    // Example. Estimation of fundamental matrix using the RANSAC algorithm
///    int point_count = 100;
///    vector<Point2f> points1(point_count);
///    vector<Point2f> points2(point_count);
///
///    // initialize the points here ...
///    for( int i = 0; i < point_count; i++ )
///    {
///        points1[i] = ...;
///        points2[i] = ...;
///    }
///
///    // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
///    Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
///
///    Mat E, R, t, mask;
///
///    E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
///    recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
///
///
/// ## Overloaded parameters
///
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * R: Output rotation matrix. Together with the translation vector, this matrix makes up a tuple
/// that performs a change of basis from the first camera's coordinate system to the second camera's
/// coordinate system. Note that, in general, t can not be used for this tuple, see the parameter
/// description below.
/// * t: Output translation vector. This vector is obtained by [decomposeEssentialMat] and
/// therefore is only known up to scale, i.e. t is the direction of the translation vector and has unit
/// length.
/// * focal: Focal length of the camera. Note that this function assumes that points1 and points2
/// are feature points from cameras with same focal length and principal point.
/// * pp: principal point of the camera.
/// * mask: Input/output mask for inliers in points1 and points2. If it is not empty, then it marks
/// inliers in points1 and points2 for the given essential matrix E. Only these inliers will be used to
/// recover pose. In the output mask only inliers which pass the chirality check.
///
/// This function differs from the one above that it computes camera intrinsic matrix from focal length and
/// principal point:
///
/// ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%0A%5Cbegin%7Bbmatrix%7D%0Af%20%26%200%20%26%20x%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%20f%20%26%20y%5F%7Bpp%7D%20%20%5C%5C%0A0%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D)
///
/// ## C++ default parameters
/// * focal: 1.0
/// * pp: Point2d(0,0)
/// * mask: noArray()
// recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray, double, Point2d, InputOutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, SimpleClass, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1709
// ("cv::recoverPose", vec![(pred!(mut, ["E", "points1", "points2", "R", "t", "focal", "pp", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Point2d", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn recover_pose(e: &impl ToInputArray, points1: &impl ToInputArray, points2: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, focal: f64, pp: core::Point2d, mask: &mut impl ToInputOutputArray) -> Result<i32> {
	input_array_arg!(e);
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(r);
	output_array_arg!(t);
	input_output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_recoverPose_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_Point2d_const__InputOutputArrayR(e.as_raw__InputArray(), points1.as_raw__InputArray(), points2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), focal, &pp, mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Registers depth data to an external camera
/// Registration is performed by creating a depth cloud, transforming the cloud by
/// the rigid body transformation between the cameras, and then projecting the
/// transformed points into the RGB camera.
///
/// uv_rgb = K_rgb * [R | t] * z * inv(K_ir) * uv_ir
///
/// Currently does not check for negative depth values.
///
/// ## Parameters
/// * unregisteredCameraMatrix: the camera matrix of the depth camera
/// * registeredCameraMatrix: the camera matrix of the external camera
/// * registeredDistCoeffs: the distortion coefficients of the external camera
/// * Rt: the rigid body transform between the cameras. Transforms points from depth camera frame to external camera frame.
/// * unregisteredDepth: the input depth data
/// * outputImagePlaneSize: the image plane dimensions of the external camera (width, height)
/// * registeredDepth: the result of transforming the depth into the external camera
/// * depthDilation: whether or not the depth is dilated to avoid holes and occlusion errors (optional)
///
/// ## Note
/// This alternative version of [register_depth] function uses the following default values for its arguments:
/// * depth_dilation: false
// cv::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:95
// ("cv::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn register_depth_def(unregistered_camera_matrix: &impl ToInputArray, registered_camera_matrix: &impl ToInputArray, registered_dist_coeffs: &impl ToInputArray, rt: &impl ToInputArray, unregistered_depth: &impl ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(unregistered_camera_matrix);
	input_array_arg!(registered_camera_matrix);
	input_array_arg!(registered_dist_coeffs);
	input_array_arg!(rt);
	input_array_arg!(unregistered_depth);
	output_array_arg!(registered_depth);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Registers depth data to an external camera
/// Registration is performed by creating a depth cloud, transforming the cloud by
/// the rigid body transformation between the cameras, and then projecting the
/// transformed points into the RGB camera.
///
/// uv_rgb = K_rgb * [R | t] * z * inv(K_ir) * uv_ir
///
/// Currently does not check for negative depth values.
///
/// ## Parameters
/// * unregisteredCameraMatrix: the camera matrix of the depth camera
/// * registeredCameraMatrix: the camera matrix of the external camera
/// * registeredDistCoeffs: the distortion coefficients of the external camera
/// * Rt: the rigid body transform between the cameras. Transforms points from depth camera frame to external camera frame.
/// * unregisteredDepth: the input depth data
/// * outputImagePlaneSize: the image plane dimensions of the external camera (width, height)
/// * registeredDepth: the result of transforming the depth into the external camera
/// * depthDilation: whether or not the depth is dilated to avoid holes and occlusion errors (optional)
///
/// ## C++ default parameters
/// * depth_dilation: false
// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:95
// ("cv::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn register_depth(unregistered_camera_matrix: &impl ToInputArray, registered_camera_matrix: &impl ToInputArray, registered_dist_coeffs: &impl ToInputArray, rt: &impl ToInputArray, unregistered_depth: &impl ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl ToOutputArray, depth_dilation: bool) -> Result<()> {
	input_array_arg!(unregistered_camera_matrix);
	input_array_arg!(registered_camera_matrix);
	input_array_arg!(registered_dist_coeffs);
	input_array_arg!(rt);
	input_array_arg!(unregistered_depth);
	output_array_arg!(registered_depth);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), depth_dilation, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
/// by depth_factor to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
/// Otherwise, the image is simply converted to floats
/// ## Parameters
/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), it is assumed in meters)
/// * type: the desired output depth (CV_32F or CV_64F)
/// * out: The rescaled float depth image
/// * depth_factor: (optional) factor by which depth is converted to distance (by default = 1000.0 for Kinect sensor)
///
/// ## Note
/// This alternative version of [rescale_depth] function uses the following default values for its arguments:
/// * depth_factor: 1000.0
// cv::rescaleDepth(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:128
// ("cv::rescaleDepth", vec![(pred!(mut, ["in", "type", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rescale_depth_def(in_: &impl ToInputArray, typ: i32, out: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_.as_raw__InputArray(), typ, out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
/// by depth_factor to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
/// Otherwise, the image is simply converted to floats
/// ## Parameters
/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), it is assumed in meters)
/// * type: the desired output depth (CV_32F or CV_64F)
/// * out: The rescaled float depth image
/// * depth_factor: (optional) factor by which depth is converted to distance (by default = 1000.0 for Kinect sensor)
///
/// ## C++ default parameters
/// * depth_factor: 1000.0
// rescaleDepth(InputArray, int, OutputArray, double)(InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:128
// ("cv::rescaleDepth", vec![(pred!(mut, ["in", "type", "out", "depth_factor"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn rescale_depth(in_: &impl ToInputArray, typ: i32, out: &mut impl ToOutputArray, depth_factor: f64) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_.as_raw__InputArray(), typ, out.as_raw__OutputArray(), depth_factor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates the Sampson Distance between two points.
///
/// The function cv::sampsonDistance calculates and returns the first order approximation of the geometric error as:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Asd%28%20%5Ctexttt%7Bpt1%7D%20%2C%20%5Ctexttt%7Bpt2%7D%20%29%3D%0A%5Cfrac%7B%28%5Ctexttt%7Bpt2%7D%5Et%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%5E2%7D%0A%7B%28%28%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%280%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%281%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%5Et%20%5Ccdot%20%5Ctexttt%7Bpt2%7D%29%280%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%5Et%20%5Ccdot%20%5Ctexttt%7Bpt2%7D%29%281%29%29%5E2%7D%0A)
/// The fundamental matrix may be calculated using the [find_fundamental_mat] function. See [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.4.3 for details.
/// ## Parameters
/// * pt1: first homogeneous 2d point
/// * pt2: second homogeneous 2d point
/// * F: fundamental matrix
/// ## Returns
/// The computed Sampson distance.
// sampsonDistance(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1833
// ("cv::sampsonDistance", vec![(pred!(mut, ["pt1", "pt2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn sampson_distance(pt1: &impl ToInputArray, pt2: &impl ToInputArray, f: &impl ToInputArray) -> Result<f64> {
	input_array_arg!(pt1);
	input_array_arg!(pt2);
	input_array_arg!(f);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sampsonDistance_const__InputArrayR_const__InputArrayR_const__InputArrayR(pt1.as_raw__InputArray(), pt2.as_raw__InputArray(), f.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Saves a mesh to a specified file.
///
/// The function saves mesh to the specified file.
/// File format is chosen based on the filename extension.
///
/// ## Parameters
/// * filename: Name of the file.
/// * vertices: vertex coordinates, each value contains 3 floats
/// * indices: per-face list of vertices, each value is a vector of ints
/// * normals: per-vertex normals, each value contains 3 floats
/// * colors: per-vertex colors, each value contains 3 floats
/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
///
/// ## Note
/// This alternative version of [save_mesh] function uses the following default values for its arguments:
/// * normals: noArray()
/// * colors: noArray()
/// * tex_coords: noArray()
// cv::saveMesh(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2910
// ("cv::saveMesh", vec![(pred!(mut, ["filename", "vertices", "indices"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn save_mesh_def(filename: &str, vertices: &impl ToInputArray, indices: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(filename);
	input_array_arg!(vertices);
	input_array_arg!(indices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), indices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Saves a mesh to a specified file.
///
/// The function saves mesh to the specified file.
/// File format is chosen based on the filename extension.
///
/// ## Parameters
/// * filename: Name of the file.
/// * vertices: vertex coordinates, each value contains 3 floats
/// * indices: per-face list of vertices, each value is a vector of ints
/// * normals: per-vertex normals, each value contains 3 floats
/// * colors: per-vertex colors, each value contains 3 floats
/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
///
/// ## C++ default parameters
/// * normals: noArray()
/// * colors: noArray()
/// * tex_coords: noArray()
// saveMesh(const String &, InputArray, InputArrayOfArrays, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2910
// ("cv::saveMesh", vec![(pred!(mut, ["filename", "vertices", "indices", "normals", "colors", "texCoords"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn save_mesh(filename: &str, vertices: &impl ToInputArray, indices: &impl ToInputArray, normals: &impl ToInputArray, colors: &impl ToInputArray, tex_coords: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(filename);
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_array_arg!(normals);
	input_array_arg!(colors);
	input_array_arg!(tex_coords);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), indices.as_raw__InputArray(), normals.as_raw__InputArray(), colors.as_raw__InputArray(), tex_coords.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Saves a point cloud to a specified file.
///
/// The function saves point cloud to the specified file.
/// File format is chosen based on the filename extension.
///
/// ## Parameters
/// * filename: Name of the file
/// * vertices: vertex coordinates, each value contains 3 floats
/// * normals: per-vertex normals, each value contains 3 floats
/// * rgb: per-vertex colors, each value contains 3 floats
///
/// ## Note
/// This alternative version of [save_point_cloud] function uses the following default values for its arguments:
/// * normals: noArray()
/// * rgb: noArray()
// cv::savePointCloud(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2873
// ("cv::savePointCloud", vec![(pred!(mut, ["filename", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn save_point_cloud_def(filename: &str, vertices: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(filename);
	input_array_arg!(vertices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_savePointCloud_const_StringR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Saves a point cloud to a specified file.
///
/// The function saves point cloud to the specified file.
/// File format is chosen based on the filename extension.
///
/// ## Parameters
/// * filename: Name of the file
/// * vertices: vertex coordinates, each value contains 3 floats
/// * normals: per-vertex normals, each value contains 3 floats
/// * rgb: per-vertex colors, each value contains 3 floats
///
/// ## C++ default parameters
/// * normals: noArray()
/// * rgb: noArray()
// savePointCloud(const String &, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2873
// ("cv::savePointCloud", vec![(pred!(mut, ["filename", "vertices", "normals", "rgb"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn save_point_cloud(filename: &str, vertices: &impl ToInputArray, normals: &impl ToInputArray, rgb: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(filename);
	input_array_arg!(vertices);
	input_array_arg!(normals);
	input_array_arg!(rgb);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_savePointCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), normals.as_raw__InputArray(), rgb.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3 3D-2D point correspondences.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, 3x3 1-channel or
/// 1x3/3x1 3-channel. vector\<Point3f\> can be also passed here.
/// * imagePoints: Array of corresponding image points, 3x2 1-channel or 1x3/3x1 2-channel.
///  vector\<Point2f\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvecs: Output rotation vectors (see [Rodrigues] ) that, together with tvecs, brings points from
/// the model coordinate system to the camera coordinate system. A P3P problem has up to 4 solutions.
/// * tvecs: Output translation vectors.
/// * flags: Method for solving a P3P problem:
/// *   [SOLVEPNP_P3P] Method is based on the paper of X.S. Gao, X.-R. Hou, J. Tang, H.-F. Chang
/// "Complete Solution Classification for the Perspective-Three-Point Problem" ([gao2003complete](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_gao2003complete)).
/// *   [SOLVEPNP_AP3P] Method is based on the paper of T. Ke and S. Roumeliotis.
/// "An Efficient Algebraic Solution to the Perspective-Three-Point Problem" ([Ke17](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Ke17)).
///
/// The function estimates the object pose given 3 object points, their corresponding image
/// projections, as well as the camera intrinsic matrix and the distortion coefficients.
///
///
/// Note:
/// The solutions are sorted by reprojection errors (lowest to highest).
// solveP3P(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1132
// ("cv::solveP3P", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn solve_p3p(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32) -> Result<i32> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solveP3P_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences.
/// ## See also
/// [calib3d_solvePnP]
///
/// This function returns a list of all the possible solutions (a solution is a <rotation vector, translation vector>
/// couple), depending on the number of input points and the chosen method:
/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): 3 or 4 input points. Number of returned solutions can be between 0 and 4 with 3 input points.
/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar. Returns 2 solutions.
/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
/// Number of input points must be 4 and 2 solutions are returned. Object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
/// Only 1 solution is returned.
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvecs: Vector of output rotation vectors (see [Rodrigues] ) that, together with tvecs, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvecs: Vector of output translation vectors.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
/// * rvec: Rotation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
/// and useExtrinsicGuess is set to true.
/// * tvec: Translation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
/// and useExtrinsicGuess is set to true.
/// * reprojectionError: Optional vector of reprojection error, that is the RMS error
/// (![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctext%7BRMSE%7D%20%3D%20%5Csqrt%7B%5Cfrac%7B%5Csum%5F%7Bi%7D%5E%7BN%7D%20%5Cleft%20%28%20%5Chat%7By%5Fi%7D%20%2D%20y%5Fi%20%5Cright%20%29%5E2%7D%7BN%7D%7D%20)) between the input image points
/// and the 3D object points projected with the estimated pose.
///
/// More information is described in [calib3d_solvePnP]
///
///
/// Note:
///    *   An example of how to use solvePnP for planar augmented reality can be found at
///        opencv_source_code/samples/python/plane_ar.py
///    *   If you are using Python:
///        - Numpy array slices won't work as input because solvePnP requires contiguous
///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
///        modules/3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
///        which requires 2-channel information.
///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
///    *   The methods [SOLVEPNP_DLS] and [SOLVEPNP_UPNP] cannot be used as the current implementations are
///        unstable and sometimes give completely wrong results. If you pass one of these two
///        flags, [SOLVEPNP_EPNP] method will be used instead.
///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
///        global solution to converge.
///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
///        Number of input points must be 4. Object points must be defined in the following order:
///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
///
/// ## Note
/// This alternative version of [solve_pnp_generic] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
/// * rvec: noArray()
/// * tvec: noArray()
/// * reprojection_error: noArray()
// cv::solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1272
// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_generic_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences.
/// ## See also
/// [calib3d_solvePnP]
///
/// This function returns a list of all the possible solutions (a solution is a <rotation vector, translation vector>
/// couple), depending on the number of input points and the chosen method:
/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): 3 or 4 input points. Number of returned solutions can be between 0 and 4 with 3 input points.
/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar. Returns 2 solutions.
/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
/// Number of input points must be 4 and 2 solutions are returned. Object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
/// Only 1 solution is returned.
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvecs: Vector of output rotation vectors (see [Rodrigues] ) that, together with tvecs, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvecs: Vector of output translation vectors.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
/// * rvec: Rotation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
/// and useExtrinsicGuess is set to true.
/// * tvec: Translation vector used to initialize an iterative PnP refinement algorithm, when flag is [SOLVEPNP_ITERATIVE]
/// and useExtrinsicGuess is set to true.
/// * reprojectionError: Optional vector of reprojection error, that is the RMS error
/// (![inline formula](https://latex.codecogs.com/png.latex?%20%5Ctext%7BRMSE%7D%20%3D%20%5Csqrt%7B%5Cfrac%7B%5Csum%5F%7Bi%7D%5E%7BN%7D%20%5Cleft%20%28%20%5Chat%7By%5Fi%7D%20%2D%20y%5Fi%20%5Cright%20%29%5E2%7D%7BN%7D%7D%20)) between the input image points
/// and the 3D object points projected with the estimated pose.
///
/// More information is described in [calib3d_solvePnP]
///
///
/// Note:
///    *   An example of how to use solvePnP for planar augmented reality can be found at
///        opencv_source_code/samples/python/plane_ar.py
///    *   If you are using Python:
///        - Numpy array slices won't work as input because solvePnP requires contiguous
///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
///        modules/3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
///        which requires 2-channel information.
///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
///    *   The methods [SOLVEPNP_DLS] and [SOLVEPNP_UPNP] cannot be used as the current implementations are
///        unstable and sometimes give completely wrong results. If you pass one of these two
///        flags, [SOLVEPNP_EPNP] method will be used instead.
///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
///        global solution to converge.
///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
///        Number of input points must be 4. Object points must be defined in the following order:
///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
/// * rvec: noArray()
/// * tvec: noArray()
/// * reprojection_error: noArray()
// solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, bool, int, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1272
// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "useExtrinsicGuess", "flags", "rvec", "tvec", "reprojectionError"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_generic(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: i32, rvec: &impl ToInputArray, tvec: &impl ToInputArray, reprojection_error: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	output_array_arg!(reprojection_error);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), use_extrinsic_guess, flags, rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), reprojection_error.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences using the RANSAC scheme.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for [SOLVEPNP_ITERATIVE]. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * iterationsCount: Number of iterations.
/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
/// is the maximum allowed distance between the observed and computed point projections to consider it
/// an inlier.
/// * confidence: The probability that the algorithm produces a useful result.
/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
/// * flags: Method for solving a PnP problem (see [solvePnP] ).
///
/// The function estimates an object pose given a set of object points, their corresponding image
/// projections, as well as the camera intrinsic matrix and the distortion coefficients. This function finds such
/// a pose that minimizes reprojection error, that is, the sum of squared distances between the observed
/// projections imagePoints and the projected (using [projectPoints] ) objectPoints. The use of RANSAC
/// makes the function resistant to outliers.
///
///
/// Note:
///    *   An example of how to use solvePNPRansac for object detection can be found at
///        opencv_source_code/samples/cpp/tutorial_code/3d/real_time_pose_estimation/
///    *   The default method used to estimate the camera pose for the Minimal Sample Sets step
///        is #SOLVEPNP_EPNP. Exceptions are:
///          - if you choose [SOLVEPNP_P3P] or #SOLVEPNP_AP3P, these methods will be used.
///          - if the number of input points is equal to 4, [SOLVEPNP_P3P] is used.
///    *   The method used to estimate the camera pose using all the inliers is defined by the
///        flags parameters unless it is equal to [SOLVEPNP_P3P] or #SOLVEPNP_AP3P. In this case,
///        the method [SOLVEPNP_EPNP] will be used instead.
///
/// ## Note
/// This alternative version of [solve_pnp_ransac] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
/// * iterations_count: 100
/// * reprojection_error: 8.0
/// * confidence: 0.99
/// * inliers: noArray()
/// * flags: SOLVEPNP_ITERATIVE
// cv::solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1089
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_ransac_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences using the RANSAC scheme.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for [SOLVEPNP_ITERATIVE]. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * iterationsCount: Number of iterations.
/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
/// is the maximum allowed distance between the observed and computed point projections to consider it
/// an inlier.
/// * confidence: The probability that the algorithm produces a useful result.
/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
/// * flags: Method for solving a PnP problem (see [solvePnP] ).
///
/// The function estimates an object pose given a set of object points, their corresponding image
/// projections, as well as the camera intrinsic matrix and the distortion coefficients. This function finds such
/// a pose that minimizes reprojection error, that is, the sum of squared distances between the observed
/// projections imagePoints and the projected (using [projectPoints] ) objectPoints. The use of RANSAC
/// makes the function resistant to outliers.
///
///
/// Note:
///    *   An example of how to use solvePNPRansac for object detection can be found at
///        opencv_source_code/samples/cpp/tutorial_code/3d/real_time_pose_estimation/
///    *   The default method used to estimate the camera pose for the Minimal Sample Sets step
///        is #SOLVEPNP_EPNP. Exceptions are:
///          - if you choose [SOLVEPNP_P3P] or #SOLVEPNP_AP3P, these methods will be used.
///          - if the number of input points is equal to 4, [SOLVEPNP_P3P] is used.
///    *   The method used to estimate the camera pose using all the inliers is defined by the
///        flags parameters unless it is equal to [SOLVEPNP_P3P] or #SOLVEPNP_AP3P. In this case,
///        the method [SOLVEPNP_EPNP] will be used instead.
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
/// * iterations_count: 100
/// * reprojection_error: 8.0
/// * confidence: 0.99
/// * inliers: noArray()
/// * flags: SOLVEPNP_ITERATIVE
// solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, double, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1089
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "confidence", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int", "float", "double", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn solve_pnp_ransac(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, confidence: f64, inliers: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int_float_double_const__OutputArrayR_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, confidence, inliers.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [solve_pnp_ransac_1] function uses the following default values for its arguments:
/// * params: UsacParams()
// cv::solvePnPRansac(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1100
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_ransac_1_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_output_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * params: UsacParams()
// solvePnPRansac(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray, const UsacParams &)(InputArray, InputArray, InputOutputArray, InputArray, OutputArray, OutputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1100
// ("cv::solvePnPRansac", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "inliers", "params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::UsacParams*"]), _)]),
#[inline]
pub fn solve_pnp_ransac_1(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, params: crate::mod_3d::UsacParams) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_output_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRansac_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_UsacParamsR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), inliers.as_raw__OutputArray(), &params, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
/// where N is the number of points. vector\<Point3d\> can also be passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can also be passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
///
/// The function refines the object pose given at least 3 object points, their corresponding image
/// projections, an initial solution for the rotation and translation vector,
/// as well as the camera intrinsic matrix and the distortion coefficients.
/// The function minimizes the projection error with respect to the rotation and the translation vectors, according
/// to a Levenberg-Marquardt iterative minimization [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) process.
///
/// ## Note
/// This alternative version of [solve_pnp_refine_lm] function uses the following default values for its arguments:
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
// cv::solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1161
// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_refine_lm_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
/// where N is the number of points. vector\<Point3d\> can also be passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can also be passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
///
/// The function refines the object pose given at least 3 object points, their corresponding image
/// projections, an initial solution for the rotation and translation vector,
/// as well as the camera intrinsic matrix and the distortion coefficients.
/// The function minimizes the projection error with respect to the rotation and the translation vectors, according
/// to a Levenberg-Marquardt iterative minimization [Madsen04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Eade13) process.
///
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
// solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1161
// ("cv::solvePnPRefineLM", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria"]), _)]),
#[inline]
pub fn solve_pnp_refine_lm(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, criteria: core::TermCriteria) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRefineLM_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
/// where N is the number of points. vector\<Point3d\> can also be passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can also be passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
/// * VVSlambda: Gain for the virtual visual servoing control law, equivalent to the ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha)
/// gain in the Damped Gauss-Newton formulation.
///
/// The function refines the object pose given at least 3 object points, their corresponding image
/// projections, an initial solution for the rotation and translation vector,
/// as well as the camera intrinsic matrix and the distortion coefficients.
/// The function minimizes the projection error with respect to the rotation and the translation vectors, using a
/// virtual visual servoing (VVS) [Chaumette06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Chaumette06) [Marchand16](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Marchand16) scheme.
///
/// ## Note
/// This alternative version of [solve_pnp_refine_vvs] function uses the following default values for its arguments:
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
/// * vv_slambda: 1
// cv::solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1193
// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_refine_vvs_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Refine a pose (the translation and the rotation that transform a 3D point expressed in the object coordinate frame
/// to the camera coordinate frame) from a 3D-2D point correspondences and starting from an initial solution.
/// ## See also
/// [calib3d_solvePnP]
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or 1xN/Nx1 3-channel,
/// where N is the number of points. vector\<Point3d\> can also be passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can also be passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Input/Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system. Input values are used as an initial solution.
/// * tvec: Input/Output translation vector. Input values are used as an initial solution.
/// * criteria: Criteria when to stop the Levenberg-Marquard iterative algorithm.
/// * VVSlambda: Gain for the virtual visual servoing control law, equivalent to the ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha)
/// gain in the Damped Gauss-Newton formulation.
///
/// The function refines the object pose given at least 3 object points, their corresponding image
/// projections, an initial solution for the rotation and translation vector,
/// as well as the camera intrinsic matrix and the distortion coefficients.
/// The function minimizes the projection error with respect to the rotation and the translation vectors, using a
/// virtual visual servoing (VVS) [Chaumette06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Chaumette06) [Marchand16](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Marchand16) scheme.
///
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
/// * vv_slambda: 1
// solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria, double)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1193
// ("cv::solvePnPRefineVVS", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "criteria", "VVSlambda"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::TermCriteria", "double"]), _)]),
#[inline]
pub fn solve_pnp_refine_vvs(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, criteria: core::TermCriteria, vv_slambda: f64) -> Result<()> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnPRefineVVS_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_TermCriteria_double(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &criteria, vv_slambda, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences.
/// ## See also
/// [calib3d_solvePnP]
///
/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
/// coordinate frame to the camera coordinate frame, using different methods:
/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
/// Number of input points must be 4. Object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
///
/// More information about Perspective-n-Points is described in [calib3d_solvePnP]
///
///
/// Note:
///    *   An example of how to use solvePnP for planar augmented reality can be found at
///        opencv_source_code/samples/python/plane_ar.py
///    *   If you are using Python:
///        - Numpy array slices won't work as input because solvePnP requires contiguous
///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
///        modules/3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
///        which requires 2-channel information.
///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
///    *   The methods [SOLVEPNP_DLS] and [SOLVEPNP_UPNP] cannot be used as the current implementations are
///        unstable and sometimes give completely wrong results. If you pass one of these two
///        flags, [SOLVEPNP_EPNP] method will be used instead.
///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
///        global solution to converge.
///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
///        Number of input points must be 4. Object points must be defined in the following order:
///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
///    *  With [SOLVEPNP_SQPNP] input points must be >= 3
///
/// ## Note
/// This alternative version of [solve_pnp] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
// cv::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1041
// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an object pose from 3D-2D point correspondences.
/// ## See also
/// [calib3d_solvePnP]
///
/// This function returns the rotation and the translation vectors that transform a 3D point expressed in the object
/// coordinate frame to the camera coordinate frame, using different methods:
/// - P3P methods ([SOLVEPNP_P3P], [SOLVEPNP_AP3P]): need 4 input points to return a unique solution.
/// - [SOLVEPNP_IPPE] Input points must be >= 4 and object points must be coplanar.
/// - [SOLVEPNP_IPPE_SQUARE] Special case suitable for marker pose estimation.
/// Number of input points must be 4. Object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
/// - for all the other flags, number of input points must be >= 4 and object points can be in any configuration.
///
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3d\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2d\> can be also passed here.
/// * cameraMatrix: Input camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffs). If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Output rotation vector (see [Rodrigues] ) that, together with tvec, brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem: see [calib3d_solvePnP_flags]
///
/// More information about Perspective-n-Points is described in [calib3d_solvePnP]
///
///
/// Note:
///    *   An example of how to use solvePnP for planar augmented reality can be found at
///        opencv_source_code/samples/python/plane_ar.py
///    *   If you are using Python:
///        - Numpy array slices won't work as input because solvePnP requires contiguous
///        arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
///        modules/3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of [undistort_points] (around line 75 of modules/3d/src/solvepnp.cpp version 2.4.9)
///        which requires 2-channel information.
///        - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
///        it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
///        np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
///    *   The methods [SOLVEPNP_DLS] and [SOLVEPNP_UPNP] cannot be used as the current implementations are
///        unstable and sometimes give completely wrong results. If you pass one of these two
///        flags, [SOLVEPNP_EPNP] method will be used instead.
///    *   The minimum number of points is 4 in the general case. In the case of [SOLVEPNP_P3P] and [SOLVEPNP_AP3P]
///        methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
///        of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
///    *   With [SOLVEPNP_ITERATIVE] method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
///        are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
///        global solution to converge.
///    *   With [SOLVEPNP_IPPE] input points must be >= 4 and object points must be coplanar.
///    *   With [SOLVEPNP_IPPE_SQUARE] this is a special case suitable for marker pose estimation.
///        Number of input points must be 4. Object points must be defined in the following order:
///          - point 0: [-squareLength / 2,  squareLength / 2, 0]
///          - point 1: [ squareLength / 2,  squareLength / 2, 0]
///          - point 2: [ squareLength / 2, -squareLength / 2, 0]
///          - point 3: [-squareLength / 2, -squareLength / 2, 0]
///    *  With [SOLVEPNP_SQPNP] input points must be >= 3
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1041
// ("cv::solvePnP", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "int"]), _)]),
#[inline]
pub fn solve_pnp(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_solvePnP_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Overloaded version of triangleRasterize() with color-only rendering
///
/// ## Parameters
/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
/// Should contain CV_32SC3 values or compatible
/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
/// If the values are out of [0; 1] range, the result correctness is not guaranteed
/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
/// * fovY: field of view in vertical direction, given in radians
/// * zNear: minimum Z value to render, everything closer is clipped
/// * zFar: maximum Z value to render, everything farther is clipped
/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
/// with CW culling and with disabled GL compatibility
///
/// ## Note
/// This alternative version of [triangle_rasterize_color] function uses the following default values for its arguments:
/// * settings: TriangleRasterizeSettings()
// cv::triangleRasterizeColor(InputArray, InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3039
// ("cv::triangleRasterizeColor", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
#[inline]
pub fn triangle_rasterize_color_def(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64) -> Result<()> {
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_array_arg!(colors);
	input_output_array_arg!(color_buf);
	input_array_arg!(world2cam);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Overloaded version of triangleRasterize() with color-only rendering
///
/// ## Parameters
/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
/// Should contain CV_32SC3 values or compatible
/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
/// If the values are out of [0; 1] range, the result correctness is not guaranteed
/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
/// * fovY: field of view in vertical direction, given in radians
/// * zNear: minimum Z value to render, everything closer is clipped
/// * zFar: maximum Z value to render, everything farther is clipped
/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
/// with CW culling and with disabled GL compatibility
///
/// ## C++ default parameters
/// * settings: TriangleRasterizeSettings()
// triangleRasterizeColor(InputArray, InputArray, InputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3039
// ("cv::triangleRasterizeColor", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
#[inline]
pub fn triangle_rasterize_color(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64, settings: crate::mod_3d::TriangleRasterizeSettings) -> Result<()> {
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_array_arg!(colors);
	input_output_array_arg!(color_buf);
	input_array_arg!(world2cam);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, &settings, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Overloaded version of triangleRasterize() with depth-only rendering
///
/// ## Parameters
/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
/// Should contain CV_32SC3 values or compatible
/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
/// * fovY: field of view in vertical direction, given in radians
/// * zNear: minimum Z value to render, everything closer is clipped
/// * zFar: maximum Z value to render, everything farther is clipped
/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
/// with CW culling and with disabled GL compatibility
///
/// ## Note
/// This alternative version of [triangle_rasterize_depth] function uses the following default values for its arguments:
/// * settings: TriangleRasterizeSettings()
// cv::triangleRasterizeDepth(InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3019
// ("cv::triangleRasterizeDepth", vec![(pred!(mut, ["vertices", "indices", "depthBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
#[inline]
pub fn triangle_rasterize_depth_def(vertices: &impl ToInputArray, indices: &impl ToInputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64) -> Result<()> {
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_output_array_arg!(depth_buf);
	input_array_arg!(world2cam);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Overloaded version of triangleRasterize() with depth-only rendering
///
/// ## Parameters
/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
/// Should contain CV_32SC3 values or compatible
/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
/// * fovY: field of view in vertical direction, given in radians
/// * zNear: minimum Z value to render, everything closer is clipped
/// * zFar: maximum Z value to render, everything farther is clipped
/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
/// with CW culling and with disabled GL compatibility
///
/// ## C++ default parameters
/// * settings: TriangleRasterizeSettings()
// triangleRasterizeDepth(InputArray, InputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:3019
// ("cv::triangleRasterizeDepth", vec![(pred!(mut, ["vertices", "indices", "depthBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
#[inline]
pub fn triangle_rasterize_depth(vertices: &impl ToInputArray, indices: &impl ToInputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64, settings: crate::mod_3d::TriangleRasterizeSettings) -> Result<()> {
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_output_array_arg!(depth_buf);
	input_array_arg!(world2cam);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, &settings, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Renders a set of triangles on a depth and color image
///
/// Triangles can be drawn white (1.0, 1.0, 1.0), flat-shaded or with a color interpolation between vertices.
/// In flat-shaded mode the 1st vertex color of each triangle is used to fill the whole triangle.
///
/// The world2cam is an inverted camera pose matrix in fact. It transforms vertices from world to
/// camera coordinate system.
///
/// The camera coordinate system emulates the OpenGL's coordinate system having coordinate origin in a screen center,
/// X axis pointing right, Y axis pointing up and Z axis pointing towards the viewer
/// except that image is vertically flipped after the render.
/// This means that all visible objects are placed in z-negative area, or exactly in -zNear > z > -zFar since
/// zNear and zFar are positive.
/// For example, at fovY = PI/2 the point (0, 1, -1) will be projected to (width/2, 0) screen point,
/// (1, 0, -1) to (width/2 + height/2, height/2). Increasing fovY makes projection smaller and vice versa.
///
/// The function does not create or clear output images before the rendering. This means that it can be used
/// for drawing over an existing image or for rendering a model into a 3D scene using pre-filled Z-buffer.
///
/// Empty scene results in a depth buffer filled by the maximum value since every pixel is infinitely far from the camera.
/// Therefore, before rendering anything from scratch the depthBuf should be filled by zFar values (or by ones in INVDEPTH mode).
///
/// There are special versions of this function named triangleRasterizeDepth and triangleRasterizeColor
/// for cases if a user needs a color image or a depth image alone; they may run slightly faster.
///
/// ## Parameters
/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
/// Should contain CV_32SC3 values or compatible
/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
/// If the values are out of [0; 1] range, the result correctness is not guaranteed
/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
/// * fovY: field of view in vertical direction, given in radians
/// * zNear: minimum Z value to render, everything closer is clipped
/// * zFar: maximum Z value to render, everything farther is clipped
/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
/// with CW culling and with disabled GL compatibility
///
/// ## Note
/// This alternative version of [triangle_rasterize] function uses the following default values for its arguments:
/// * settings: TriangleRasterizeSettings()
// cv::triangleRasterize(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2999
// ("cv::triangleRasterize", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "depthBuf", "world2cam", "fovY", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double"]), _)]),
#[inline]
pub fn triangle_rasterize_def(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64) -> Result<()> {
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_array_arg!(colors);
	input_output_array_arg!(color_buf);
	input_output_array_arg!(depth_buf);
	input_array_arg!(world2cam);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Renders a set of triangles on a depth and color image
///
/// Triangles can be drawn white (1.0, 1.0, 1.0), flat-shaded or with a color interpolation between vertices.
/// In flat-shaded mode the 1st vertex color of each triangle is used to fill the whole triangle.
///
/// The world2cam is an inverted camera pose matrix in fact. It transforms vertices from world to
/// camera coordinate system.
///
/// The camera coordinate system emulates the OpenGL's coordinate system having coordinate origin in a screen center,
/// X axis pointing right, Y axis pointing up and Z axis pointing towards the viewer
/// except that image is vertically flipped after the render.
/// This means that all visible objects are placed in z-negative area, or exactly in -zNear > z > -zFar since
/// zNear and zFar are positive.
/// For example, at fovY = PI/2 the point (0, 1, -1) will be projected to (width/2, 0) screen point,
/// (1, 0, -1) to (width/2 + height/2, height/2). Increasing fovY makes projection smaller and vice versa.
///
/// The function does not create or clear output images before the rendering. This means that it can be used
/// for drawing over an existing image or for rendering a model into a 3D scene using pre-filled Z-buffer.
///
/// Empty scene results in a depth buffer filled by the maximum value since every pixel is infinitely far from the camera.
/// Therefore, before rendering anything from scratch the depthBuf should be filled by zFar values (or by ones in INVDEPTH mode).
///
/// There are special versions of this function named triangleRasterizeDepth and triangleRasterizeColor
/// for cases if a user needs a color image or a depth image alone; they may run slightly faster.
///
/// ## Parameters
/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
/// Should contain CV_32SC3 values or compatible
/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
/// If the values are out of [0; 1] range, the result correctness is not guaranteed
/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
/// * fovY: field of view in vertical direction, given in radians
/// * zNear: minimum Z value to render, everything closer is clipped
/// * zFar: maximum Z value to render, everything farther is clipped
/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
/// with CW culling and with disabled GL compatibility
///
/// ## C++ default parameters
/// * settings: TriangleRasterizeSettings()
// triangleRasterize(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, double, double, double, const TriangleRasterizeSettings &)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2999
// ("cv::triangleRasterize", vec![(pred!(mut, ["vertices", "indices", "colors", "colorBuf", "depthBuf", "world2cam", "fovY", "zNear", "zFar", "settings"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "double", "double", "double", "const cv::TriangleRasterizeSettings*"]), _)]),
#[inline]
pub fn triangle_rasterize(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64, settings: crate::mod_3d::TriangleRasterizeSettings) -> Result<()> {
	input_array_arg!(vertices);
	input_array_arg!(indices);
	input_array_arg!(colors);
	input_output_array_arg!(color_buf);
	input_output_array_arg!(depth_buf);
	input_array_arg!(world2cam);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, &settings, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// This function reconstructs 3-dimensional points (in homogeneous coordinates) by using
/// their observations with a stereo camera.
///
/// ## Parameters
/// * projMatr1: 3x4 projection matrix of the first camera, i.e. this matrix projects 3D points
/// given in the world's coordinate system into the first image.
/// * projMatr2: 3x4 projection matrix of the second camera, i.e. this matrix projects 3D points
/// given in the world's coordinate system into the second image.
/// * projPoints1: 2xN array of feature points in the first image. In the case of the c++ version,
/// it can be also a vector of feature points or two-channel matrix of size 1xN or Nx1.
/// * projPoints2: 2xN array of corresponding points in the second image. In the case of the c++
/// version, it can be also a vector of feature points or two-channel matrix of size 1xN or Nx1.
/// * points4D: 4xN array of reconstructed points in homogeneous coordinates. These points are
/// returned in the world's coordinate system.
///
///
/// Note:
///    Keep in mind that all input data should be of float type in order for this function to work.
///
///
/// Note:
///    If the projection matrices from [stereoRectify] are used, then the returned points are
///    represented in the first camera's rectified coordinate system.
/// ## See also
/// reprojectImageTo3D
// triangulatePoints(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:1794
// ("cv::triangulatePoints", vec![(pred!(mut, ["projMatr1", "projMatr2", "projPoints1", "projPoints2", "points4D"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn triangulate_points(proj_matr1: &impl ToInputArray, proj_matr2: &impl ToInputArray, proj_points1: &impl ToInputArray, proj_points2: &impl ToInputArray, points4_d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(proj_matr1);
	input_array_arg!(proj_matr2);
	input_array_arg!(proj_points1);
	input_array_arg!(proj_points2);
	output_array_arg!(points4_d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_triangulatePoints_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(proj_matr1.as_raw__InputArray(), proj_matr2.as_raw__InputArray(), proj_points1.as_raw__InputArray(), proj_points2.as_raw__InputArray(), points4_d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Compute undistorted image points position
///
/// ## Parameters
/// * src: Observed points position, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or vector\<Point2f\> ).
/// * dst: Output undistorted points position (1xN/Nx1 2-channel or vector\<Point2f\> ).
/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Distortion coefficients
///
/// ## Note
/// This alternative version of [undistort_image_points] function uses the following default values for its arguments:
/// * unnamed: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,0.01)
// cv::undistortImagePoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2472
// ("cv::undistortImagePoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn undistort_image_points_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Compute undistorted image points position
///
/// ## Parameters
/// * src: Observed points position, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or vector\<Point2f\> ).
/// * dst: Output undistorted points position (1xN/Nx1 2-channel or vector\<Point2f\> ).
/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Distortion coefficients
///
/// ## C++ default parameters
/// * unnamed: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5,0.01)
// undistortImagePoints(InputArray, OutputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2472
// ("cv::undistortImagePoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "unnamed"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
#[inline]
pub fn undistort_image_points(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, unnamed: core::TermCriteria) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_undistortImagePoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), &unnamed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the ideal point coordinates from the observed point coordinates.
///
/// The function is similar to [undistort] and [init_undistort_rectify_map] but it operates on a
/// sparse set of points instead of a raster image. Also the function performs a reverse transformation
/// to  #projectPoints. In case of a 3D object, it does not reconstruct its 3D coordinates, but for a
/// planar object, it does, up to a translation vector, if the proper R is specified.
///
/// For each observed point coordinate ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) the function computes:
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Barray%7D%7Bl%7D%0Ax%5E%7B%22%7D%20%20%5Cleftarrow%20%28u%20%2D%20c%5Fx%29%2Ff%5Fx%20%20%5C%5C%0Ay%5E%7B%22%7D%20%20%5Cleftarrow%20%28v%20%2D%20c%5Fy%29%2Ff%5Fy%20%20%5C%5C%0A%28x%27%2Cy%27%29%20%3D%20undistort%28x%5E%7B%22%7D%2Cy%5E%7B%22%7D%2C%20%5Ctexttt%7BdistCoeffs%7D%29%20%5C%5C%0A%7B%5BX%5C%2CY%5C%2CW%5D%7D%20%5ET%20%20%5Cleftarrow%20R%2A%5Bx%27%20%5C%2C%20y%27%20%5C%2C%201%5D%5ET%20%20%5C%5C%0Ax%20%20%5Cleftarrow%20X%2FW%20%20%5C%5C%0Ay%20%20%5Cleftarrow%20Y%2FW%20%20%5C%5C%0A%5Ctext%7Bonly%20performed%20if%20P%20is%20specified%3A%7D%20%5C%5C%0Au%27%20%20%5Cleftarrow%20x%20%7Bf%27%7D%5Fx%20%2B%20%7Bc%27%7D%5Fx%20%20%5C%5C%0Av%27%20%20%5Cleftarrow%20y%20%7Bf%27%7D%5Fy%20%2B%20%7Bc%27%7D%5Fy%0A%5Cend%7Barray%7D%0A)
///
/// where *undistort* is an approximate iterative algorithm that estimates the normalized original
/// point coordinates out of the normalized distorted point coordinates ("normalized" means that the
/// coordinates do not depend on the camera matrix).
///
/// The function can be used for both a stereo camera head or a monocular camera (when R is empty).
/// ## Parameters
/// * src: Observed point coordinates, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or
/// vector\<Point2f\> ).
/// * dst: Output ideal point coordinates (1xN/Nx1 2-channel or vector\<Point2f\> ) after undistortion and reverse perspective
/// transformation. If matrix P is identity or omitted, dst will contain normalized point coordinates.
/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * R: Rectification transformation in the object space (3x3 matrix). R1 or R2 computed by
/// [stereo_rectify] can be passed here. If the matrix is empty, the identity transformation is used.
/// * P: New camera matrix (3x3) or new projection matrix (3x4) ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%7Bf%27%7D%5Fx%20%26%200%20%26%20%7Bc%27%7D%5Fx%20%26%20t%5Fx%20%5C%5C%200%20%26%20%7Bf%27%7D%5Fy%20%26%20%7Bc%27%7D%5Fy%20%26%20t%5Fy%20%5C%5C%200%20%26%200%20%26%201%20%26%20t%5Fz%20%5Cend%7Bbmatrix%7D). P1 or P2 computed by
/// [stereo_rectify] can be passed here. If the matrix is empty, the identity new camera matrix is used.
/// * criteria: termination criteria for the iterative point undistortion algorithm
///
/// ## Note
/// This alternative version of [undistort_points] function uses the following default values for its arguments:
/// * r: noArray()
/// * p: noArray()
/// * criteria: TermCriteria(TermCriteria::MAX_ITER,5,0.01)
// cv::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2457
// ("cv::undistortPoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn undistort_points_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the ideal point coordinates from the observed point coordinates.
///
/// The function is similar to [undistort] and [init_undistort_rectify_map] but it operates on a
/// sparse set of points instead of a raster image. Also the function performs a reverse transformation
/// to  #projectPoints. In case of a 3D object, it does not reconstruct its 3D coordinates, but for a
/// planar object, it does, up to a translation vector, if the proper R is specified.
///
/// For each observed point coordinate ![inline formula](https://latex.codecogs.com/png.latex?%28u%2C%20v%29) the function computes:
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Barray%7D%7Bl%7D%0Ax%5E%7B%22%7D%20%20%5Cleftarrow%20%28u%20%2D%20c%5Fx%29%2Ff%5Fx%20%20%5C%5C%0Ay%5E%7B%22%7D%20%20%5Cleftarrow%20%28v%20%2D%20c%5Fy%29%2Ff%5Fy%20%20%5C%5C%0A%28x%27%2Cy%27%29%20%3D%20undistort%28x%5E%7B%22%7D%2Cy%5E%7B%22%7D%2C%20%5Ctexttt%7BdistCoeffs%7D%29%20%5C%5C%0A%7B%5BX%5C%2CY%5C%2CW%5D%7D%20%5ET%20%20%5Cleftarrow%20R%2A%5Bx%27%20%5C%2C%20y%27%20%5C%2C%201%5D%5ET%20%20%5C%5C%0Ax%20%20%5Cleftarrow%20X%2FW%20%20%5C%5C%0Ay%20%20%5Cleftarrow%20Y%2FW%20%20%5C%5C%0A%5Ctext%7Bonly%20performed%20if%20P%20is%20specified%3A%7D%20%5C%5C%0Au%27%20%20%5Cleftarrow%20x%20%7Bf%27%7D%5Fx%20%2B%20%7Bc%27%7D%5Fx%20%20%5C%5C%0Av%27%20%20%5Cleftarrow%20y%20%7Bf%27%7D%5Fy%20%2B%20%7Bc%27%7D%5Fy%0A%5Cend%7Barray%7D%0A)
///
/// where *undistort* is an approximate iterative algorithm that estimates the normalized original
/// point coordinates out of the normalized distorted point coordinates ("normalized" means that the
/// coordinates do not depend on the camera matrix).
///
/// The function can be used for both a stereo camera head or a monocular camera (when R is empty).
/// ## Parameters
/// * src: Observed point coordinates, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel (CV_32FC2 or CV_64FC2) (or
/// vector\<Point2f\> ).
/// * dst: Output ideal point coordinates (1xN/Nx1 2-channel or vector\<Point2f\> ) after undistortion and reverse perspective
/// transformation. If matrix P is identity or omitted, dst will contain normalized point coordinates.
/// * cameraMatrix: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * R: Rectification transformation in the object space (3x3 matrix). R1 or R2 computed by
/// [stereo_rectify] can be passed here. If the matrix is empty, the identity transformation is used.
/// * P: New camera matrix (3x3) or new projection matrix (3x4) ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%7Bf%27%7D%5Fx%20%26%200%20%26%20%7Bc%27%7D%5Fx%20%26%20t%5Fx%20%5C%5C%200%20%26%20%7Bf%27%7D%5Fy%20%26%20%7Bc%27%7D%5Fy%20%26%20t%5Fy%20%5C%5C%200%20%26%200%20%26%201%20%26%20t%5Fz%20%5Cend%7Bbmatrix%7D). P1 or P2 computed by
/// [stereo_rectify] can be passed here. If the matrix is empty, the identity new camera matrix is used.
/// * criteria: termination criteria for the iterative point undistortion algorithm
///
/// ## C++ default parameters
/// * r: noArray()
/// * p: noArray()
/// * criteria: TermCriteria(TermCriteria::MAX_ITER,5,0.01)
// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, TermCriteria)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2457
// ("cv::undistortPoints", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "R", "P", "criteria"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
#[inline]
pub fn undistort_points(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray, criteria: core::TermCriteria) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(r);
	input_array_arg!(p);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_TermCriteria(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Transforms an image to compensate for lens distortion.
///
/// The function transforms an image to compensate radial and tangential lens distortion.
///
/// The function is simply a combination of [init_undistort_rectify_map] (with unity R ) and [remap]
/// (with bilinear interpolation). See the former function for details of the transformation being
/// performed.
///
/// Those pixels in the destination image, for which there is no correspondent pixels in the source
/// image, are filled with zeros (black color).
///
/// A particular subset of the source image that will be visible in the corrected image can be regulated
/// by newCameraMatrix. You can use [get_optimal_new_camera_matrix] to compute the appropriate
/// newCameraMatrix depending on your requirements.
///
/// The camera matrix and the distortion parameters can be determined using #calibrateCamera. If
/// the resolution of images is different from the resolution used at the calibration stage, ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%2C%0Af%5Fy%2C%20c%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) need to be scaled accordingly, while the distortion coefficients remain
/// the same.
///
/// ## Parameters
/// * src: Input (distorted) image.
/// * dst: Output (corrected) image that has the same size and type as src .
/// * cameraMatrix: Input camera matrix ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * newCameraMatrix: Camera matrix of the distorted image. By default, it is the same as
/// cameraMatrix but you may additionally scale and shift the result by using a different matrix.
///
/// ## Note
/// This alternative version of [undistort] function uses the following default values for its arguments:
/// * new_camera_matrix: noArray()
// cv::undistort(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2175
// ("cv::undistort", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn undistort_def(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Transforms an image to compensate for lens distortion.
///
/// The function transforms an image to compensate radial and tangential lens distortion.
///
/// The function is simply a combination of [init_undistort_rectify_map] (with unity R ) and [remap]
/// (with bilinear interpolation). See the former function for details of the transformation being
/// performed.
///
/// Those pixels in the destination image, for which there is no correspondent pixels in the source
/// image, are filled with zeros (black color).
///
/// A particular subset of the source image that will be visible in the corrected image can be regulated
/// by newCameraMatrix. You can use [get_optimal_new_camera_matrix] to compute the appropriate
/// newCameraMatrix depending on your requirements.
///
/// The camera matrix and the distortion parameters can be determined using #calibrateCamera. If
/// the resolution of images is different from the resolution used at the calibration stage, ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%2C%0Af%5Fy%2C%20c%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) need to be scaled accordingly, while the distortion coefficients remain
/// the same.
///
/// ## Parameters
/// * src: Input (distorted) image.
/// * dst: Output (corrected) image that has the same size and type as src .
/// * cameraMatrix: Input camera matrix ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) .
/// * distCoeffs: Input vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
/// of 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are assumed.
/// * newCameraMatrix: Camera matrix of the distorted image. By default, it is the same as
/// cameraMatrix but you may additionally scale and shift the result by using a different matrix.
///
/// ## C++ default parameters
/// * new_camera_matrix: noArray()
// undistort(InputArray, OutputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2175
// ("cv::undistort", vec![(pred!(mut, ["src", "dst", "cameraMatrix", "distCoeffs", "newCameraMatrix"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn undistort(src: &impl ToInputArray, dst: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, new_camera_matrix: &impl ToInputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(new_camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_undistort_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), new_camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point cloud sampling by Voxel Grid filter downsampling.
///
/// Creates a 3D voxel grid (a set of tiny 3D boxes in space) over the input
/// point cloud data, in each voxel (i.e., 3D box), all the points present will be
/// approximated (i.e., downsampled) with the point closest to their centroid.
///
/// ## Parameters
/// * sampled_point_flags:[out] Flags of the sampled point, (pass in std::vector<int> or std::vector<char> etc.)
///                    sampled_point_flags[i] is 1 means i-th point selected, 0 means it is not selected.
/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
/// * length: Grid length.
/// * width: Grid width.
/// * height: Grid height.
/// ## Returns
/// The number of points actually sampled.
// voxelGridSampling(OutputArray, InputArray, float, float, float)(OutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:185
// ("cv::voxelGridSampling", vec![(pred!(mut, ["sampled_point_flags", "input_pts", "length", "width", "height"], ["const cv::_OutputArray*", "const cv::_InputArray*", "float", "float", "float"]), _)]),
#[inline]
pub fn voxel_grid_sampling(sampled_point_flags: &mut impl ToOutputArray, input_pts: &impl ToInputArray, length: f32, width: f32, height: f32) -> Result<i32> {
	output_array_arg!(sampled_point_flags);
	input_array_arg!(input_pts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_voxelGridSampling_const__OutputArrayR_const__InputArrayR_float_float_float(sampled_point_flags.as_raw__OutputArray(), input_pts.as_raw__InputArray(), length, width, height, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Warps depth or RGB-D image by reprojecting it in 3d, applying Rt transformation
/// and then projecting it back onto the image plane.
/// This function can be used to visualize the results of the Odometry algorithm.
/// ## Parameters
/// * depth: Depth data, should be 1-channel CV_16U, CV_16S, CV_32F or CV_64F
/// * image: RGB image (optional), should be 1-, 3- or 4-channel CV_8U
/// * mask: Mask of used pixels (optional), should be CV_8UC1
/// * Rt: Rotation+translation matrix (3x4 or 4x4) to be applied to depth points
/// * cameraMatrix: Camera intrinsics matrix (3x3)
/// * warpedDepth: The warped depth data (optional)
/// * warpedImage: The warped RGB image (optional)
/// * warpedMask: The mask of valid pixels in warped image (optional)
///
/// ## Note
/// This alternative version of [warp_frame] function uses the following default values for its arguments:
/// * warped_depth: noArray()
/// * warped_image: noArray()
/// * warped_mask: noArray()
// cv::warpFrame(InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:142
// ("cv::warpFrame", vec![(pred!(mut, ["depth", "image", "mask", "Rt", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn warp_frame_def(depth: &impl ToInputArray, image: &impl ToInputArray, mask: &impl ToInputArray, rt: &impl ToInputArray, camera_matrix: &impl ToInputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(image);
	input_array_arg!(mask);
	input_array_arg!(rt);
	input_array_arg!(camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(depth.as_raw__InputArray(), image.as_raw__InputArray(), mask.as_raw__InputArray(), rt.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Warps depth or RGB-D image by reprojecting it in 3d, applying Rt transformation
/// and then projecting it back onto the image plane.
/// This function can be used to visualize the results of the Odometry algorithm.
/// ## Parameters
/// * depth: Depth data, should be 1-channel CV_16U, CV_16S, CV_32F or CV_64F
/// * image: RGB image (optional), should be 1-, 3- or 4-channel CV_8U
/// * mask: Mask of used pixels (optional), should be CV_8UC1
/// * Rt: Rotation+translation matrix (3x4 or 4x4) to be applied to depth points
/// * cameraMatrix: Camera intrinsics matrix (3x3)
/// * warpedDepth: The warped depth data (optional)
/// * warpedImage: The warped RGB image (optional)
/// * warpedMask: The mask of valid pixels in warped image (optional)
///
/// ## C++ default parameters
/// * warped_depth: noArray()
/// * warped_image: noArray()
/// * warped_mask: noArray()
// warpFrame(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:142
// ("cv::warpFrame", vec![(pred!(mut, ["depth", "image", "mask", "Rt", "cameraMatrix", "warpedDepth", "warpedImage", "warpedMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn warp_frame(depth: &impl ToInputArray, image: &impl ToInputArray, mask: &impl ToInputArray, rt: &impl ToInputArray, camera_matrix: &impl ToInputArray, warped_depth: &mut impl ToOutputArray, warped_image: &mut impl ToOutputArray, warped_mask: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(image);
	input_array_arg!(mask);
	input_array_arg!(rt);
	input_array_arg!(camera_matrix);
	output_array_arg!(warped_depth);
	output_array_arg!(warped_image);
	output_array_arg!(warped_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), image.as_raw__InputArray(), mask.as_raw__InputArray(), rt.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), warped_depth.as_raw__OutputArray(), warped_image.as_raw__OutputArray(), warped_mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::mod_3d::LevMarq]
// LevMarq /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:521
pub trait LevMarqTraitConst {
	fn as_raw_LevMarq(&self) -> *const c_void;

}

/// Mutable methods for [crate::mod_3d::LevMarq]
pub trait LevMarqTrait: crate::mod_3d::LevMarqTraitConst {
	fn as_raw_mut_LevMarq(&mut self) -> *mut c_void;

	/// Runs Levenberg-Marquadt algorithm using current settings and given parameters vector.
	/// The method returns the optimization report.
	// optimize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:714
	// ("cv::LevMarq::optimize", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn optimize(&mut self) -> Result<crate::mod_3d::LevMarq_Report> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_optimize(self.as_raw_mut_LevMarq(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Report::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Runs optimization using the passed vector of parameters as the start point.
	///
	/// The final vector of parameters (whether the algorithm converged or not) is stored at the same
	/// vector.
	/// This method can be used instead of the optimize() method if rerun with different start points is required.
	/// The method returns the optimization report.
	///
	/// ## Parameters
	/// * param: initial/final vector of parameters.
	///
	/// Note that the dimensionality of parameter space is defined by the size of param vector,
	/// and the dimensionality of optimized criteria is defined by the size of err vector
	/// computed by the callback.
	// run(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:729
	// ("cv::LevMarq::run", vec![(pred!(mut, ["param"], ["const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn run(&mut self, param: &mut impl ToInputOutputArray) -> Result<crate::mod_3d::LevMarq_Report> {
		input_output_array_arg!(param);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_run_const__InputOutputArrayR(self.as_raw_mut_LevMarq(), param.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Report::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Levenberg-Marquadt solver
///
/// A Levenberg-Marquadt algorithm locally minimizes an objective function value (aka energy, cost or error) starting from
/// current param vector.
/// To do that, at each iteration it repeatedly calculates the energy at probe points until it's reduced.
/// To calculate a probe point, a linear equation is solved: (J^T*J + lambda*D)*dx = -J^T*b where J is a function jacobian,
/// b is a vector of residuals (aka errors or energy terms), D is a diagonal matrix generated from J^T*J diagonal
/// and lambda changes for each probe point. Then the resulting dx is "added" to current variable and it forms
/// a probe value. "Added" is quoted because in some groups (e.g. SO(3) group) such an increment can be a non-trivial operation.
///
/// For more details, please refer to Wikipedia page (<https://en.wikipedia.org/wiki/Levenberg%E2%80%93Marquardt_algorithm>).
///
/// This solver supports fixed variables and two forms of callback function:
/// 1. Generating ordinary jacobian J and residual vector err ("long")
/// 2. Generating normal equation matrix J^T*J and gradient vector J^T*err
///
/// Currently the solver supports dense jacobian matrix and linear parameter increment.
// LevMarq /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:521
pub struct LevMarq {
	ptr: *mut c_void,
}

opencv_type_boxed! { LevMarq }

impl Drop for LevMarq {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LevMarq_delete(self.as_raw_mut_LevMarq()) };
	}
}

unsafe impl Send for LevMarq {}

impl crate::mod_3d::LevMarqTraitConst for LevMarq {
	#[inline] fn as_raw_LevMarq(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::LevMarqTrait for LevMarq {
	#[inline] fn as_raw_mut_LevMarq(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LevMarq, crate::mod_3d::LevMarqTraitConst, as_raw_LevMarq, crate::mod_3d::LevMarqTrait, as_raw_mut_LevMarq }

impl LevMarq {
}

impl std::fmt::Debug for LevMarq {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LevMarq")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::LevMarq_Report]
// Report /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:528
pub trait LevMarq_ReportTraitConst {
	fn as_raw_LevMarq_Report(&self) -> *const c_void;

	// cv::LevMarq::Report::found() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:535
	// ("cv::LevMarq::Report::found", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn found(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Report_propFound_const(self.as_raw_LevMarq_Report()) };
		ret
	}

	// cv::LevMarq::Report::iters() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:537
	// ("cv::LevMarq::Report::iters", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn iters(&self) -> i32 {
		let ret = unsafe { sys::cv_LevMarq_Report_propIters_const(self.as_raw_LevMarq_Report()) };
		ret
	}

	// cv::LevMarq::Report::energy() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:539
	// ("cv::LevMarq::Report::energy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn energy(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Report_propEnergy_const(self.as_raw_LevMarq_Report()) };
		ret
	}

}

/// Mutable methods for [crate::mod_3d::LevMarq_Report]
pub trait LevMarq_ReportTrait: crate::mod_3d::LevMarq_ReportTraitConst {
	fn as_raw_mut_LevMarq_Report(&mut self) -> *mut c_void;

	// cv::LevMarq::Report::setFound(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:535
	// ("cv::LevMarq::Report::setFound", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_found(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Report_propFound_const_bool(self.as_raw_mut_LevMarq_Report(), val) };
		ret
	}

	// cv::LevMarq::Report::setIters(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:537
	// ("cv::LevMarq::Report::setIters", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_iters(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LevMarq_Report_propIters_const_int(self.as_raw_mut_LevMarq_Report(), val) };
		ret
	}

	// cv::LevMarq::Report::setEnergy(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:539
	// ("cv::LevMarq::Report::setEnergy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_energy(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Report_propEnergy_const_double(self.as_raw_mut_LevMarq_Report(), val) };
		ret
	}

}

/// Optimization report
///
/// The structure is returned when optimization is over.
// Report /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:528
pub struct LevMarq_Report {
	ptr: *mut c_void,
}

opencv_type_boxed! { LevMarq_Report }

impl Drop for LevMarq_Report {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LevMarq_Report_delete(self.as_raw_mut_LevMarq_Report()) };
	}
}

unsafe impl Send for LevMarq_Report {}

impl crate::mod_3d::LevMarq_ReportTraitConst for LevMarq_Report {
	#[inline] fn as_raw_LevMarq_Report(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::LevMarq_ReportTrait for LevMarq_Report {
	#[inline] fn as_raw_mut_LevMarq_Report(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LevMarq_Report, crate::mod_3d::LevMarq_ReportTraitConst, as_raw_LevMarq_Report, crate::mod_3d::LevMarq_ReportTrait, as_raw_mut_LevMarq_Report }

impl LevMarq_Report {
	// Report(bool, int, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:530
	// ("cv::LevMarq::Report::Report", vec![(pred!(mut, ["isFound", "nIters", "finalEnergy"], ["bool", "int", "double"]), _)]),
	#[inline]
	pub fn new(is_found: bool, n_iters: i32, final_energy: f64) -> Result<crate::mod_3d::LevMarq_Report> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Report_Report_bool_int_double(is_found, n_iters, final_energy, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Report::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for LevMarq_Report {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LevMarq_Report")
			.field("found", &crate::mod_3d::LevMarq_ReportTraitConst::found(self))
			.field("iters", &crate::mod_3d::LevMarq_ReportTraitConst::iters(self))
			.field("energy", &crate::mod_3d::LevMarq_ReportTraitConst::energy(self))
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::LevMarq_Settings]
// Settings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:549
pub trait LevMarq_SettingsTraitConst {
	fn as_raw_LevMarq_Settings(&self) -> *const c_void;

	// cv::LevMarq::Settings::jacobiScaling() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:575
	// ("cv::LevMarq::Settings::jacobiScaling", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn jacobi_scaling(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propJacobiScaling_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::upDouble() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:577
	// ("cv::LevMarq::Settings::upDouble", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn up_double(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propUpDouble_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::useStepQuality() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:579
	// ("cv::LevMarq::Settings::useStepQuality", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_step_quality(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propUseStepQuality_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::clampDiagonal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:581
	// ("cv::LevMarq::Settings::clampDiagonal", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clamp_diagonal(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propClampDiagonal_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::stepNormInf() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:583
	// ("cv::LevMarq::Settings::stepNormInf", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn step_norm_inf(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormInf_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::checkRelEnergyChange() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:585
	// ("cv::LevMarq::Settings::checkRelEnergyChange", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn check_rel_energy_change(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propCheckRelEnergyChange_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::checkMinGradient() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:587
	// ("cv::LevMarq::Settings::checkMinGradient", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn check_min_gradient(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propCheckMinGradient_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::checkStepNorm() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:589
	// ("cv::LevMarq::Settings::checkStepNorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn check_step_norm(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propCheckStepNorm_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::geodesic() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:591
	// ("cv::LevMarq::Settings::geodesic", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn geodesic(&self) -> bool {
		let ret = unsafe { sys::cv_LevMarq_Settings_propGeodesic_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::hGeo() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:593
	// ("cv::LevMarq::Settings::hGeo", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn h_geo(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propHGeo_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::geoScale() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:595
	// ("cv::LevMarq::Settings::geoScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn geo_scale(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propGeoScale_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::stepNormTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:597
	// ("cv::LevMarq::Settings::stepNormTolerance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn step_norm_tolerance(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormTolerance_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::relEnergyDeltaTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:599
	// ("cv::LevMarq::Settings::relEnergyDeltaTolerance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rel_energy_delta_tolerance(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::minGradientTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:601
	// ("cv::LevMarq::Settings::minGradientTolerance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_gradient_tolerance(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propMinGradientTolerance_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::smallEnergyTolerance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:603
	// ("cv::LevMarq::Settings::smallEnergyTolerance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn small_energy_tolerance(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propSmallEnergyTolerance_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::maxIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:605
	// ("cv::LevMarq::Settings::maxIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_iterations(&self) -> u32 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propMaxIterations_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::initialLambda() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:608
	// ("cv::LevMarq::Settings::initialLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initial_lambda(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLambda_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::initialLmUpFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:609
	// ("cv::LevMarq::Settings::initialLmUpFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initial_lm_up_factor(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmUpFactor_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

	// cv::LevMarq::Settings::initialLmDownFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:610
	// ("cv::LevMarq::Settings::initialLmDownFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initial_lm_down_factor(&self) -> f64 {
		let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmDownFactor_const(self.as_raw_LevMarq_Settings()) };
		ret
	}

}

/// Mutable methods for [crate::mod_3d::LevMarq_Settings]
pub trait LevMarq_SettingsTrait: crate::mod_3d::LevMarq_SettingsTraitConst {
	fn as_raw_mut_LevMarq_Settings(&mut self) -> *mut c_void;

	// cv::LevMarq::Settings::setJacobiScaling(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:575
	// ("cv::LevMarq::Settings::setJacobiScaling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_jacobi_scaling(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propJacobiScaling_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setUpDouble(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:577
	// ("cv::LevMarq::Settings::setUpDouble", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_up_double(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propUpDouble_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setUseStepQuality(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:579
	// ("cv::LevMarq::Settings::setUseStepQuality", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_step_quality(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propUseStepQuality_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setClampDiagonal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:581
	// ("cv::LevMarq::Settings::setClampDiagonal", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_clamp_diagonal(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propClampDiagonal_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setStepNormInf(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:583
	// ("cv::LevMarq::Settings::setStepNormInf", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_step_norm_inf(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormInf_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setCheckRelEnergyChange(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:585
	// ("cv::LevMarq::Settings::setCheckRelEnergyChange", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_check_rel_energy_change(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propCheckRelEnergyChange_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setCheckMinGradient(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:587
	// ("cv::LevMarq::Settings::setCheckMinGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_check_min_gradient(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propCheckMinGradient_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setCheckStepNorm(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:589
	// ("cv::LevMarq::Settings::setCheckStepNorm", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_check_step_norm(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propCheckStepNorm_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setGeodesic(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:591
	// ("cv::LevMarq::Settings::setGeodesic", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_geodesic(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propGeodesic_const_bool(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setHGeo(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:593
	// ("cv::LevMarq::Settings::setHGeo", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_h_geo(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propHGeo_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setGeoScale(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:595
	// ("cv::LevMarq::Settings::setGeoScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_geo_scale(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propGeoScale_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setStepNormTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:597
	// ("cv::LevMarq::Settings::setStepNormTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_step_norm_tolerance(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propStepNormTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setRelEnergyDeltaTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:599
	// ("cv::LevMarq::Settings::setRelEnergyDeltaTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_rel_energy_delta_tolerance(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propRelEnergyDeltaTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setMinGradientTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:601
	// ("cv::LevMarq::Settings::setMinGradientTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_gradient_tolerance(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propMinGradientTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setSmallEnergyTolerance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:603
	// ("cv::LevMarq::Settings::setSmallEnergyTolerance", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_small_energy_tolerance(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propSmallEnergyTolerance_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:605
	// ("cv::LevMarq::Settings::setMaxIterations", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
	#[inline]
	fn set_max_iterations(&mut self, val: u32) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propMaxIterations_const_unsigned_int(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setInitialLambda(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:608
	// ("cv::LevMarq::Settings::setInitialLambda", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_initial_lambda(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLambda_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setInitialLmUpFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:609
	// ("cv::LevMarq::Settings::setInitialLmUpFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_initial_lm_up_factor(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmUpFactor_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// cv::LevMarq::Settings::setInitialLmDownFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:610
	// ("cv::LevMarq::Settings::setInitialLmDownFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_initial_lm_down_factor(&mut self, val: f64) {
		let ret = unsafe { sys::cv_LevMarq_Settings_propInitialLmDownFactor_const_double(self.as_raw_mut_LevMarq_Settings(), val) };
		ret
	}

	// setJacobiScaling(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:553
	// ("cv::LevMarq::Settings::setJacobiScaling", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_jacobi_scaling_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setJacobiScaling_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setUpDouble(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:554
	// ("cv::LevMarq::Settings::setUpDouble", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_up_double_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setUpDouble_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setUseStepQuality(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:555
	// ("cv::LevMarq::Settings::setUseStepQuality", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_use_step_quality_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setUseStepQuality_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setClampDiagonal(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:556
	// ("cv::LevMarq::Settings::setClampDiagonal", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_clamp_diagonal_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setClampDiagonal_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setStepNormInf(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:557
	// ("cv::LevMarq::Settings::setStepNormInf", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_step_norm_inf_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setStepNormInf_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setCheckRelEnergyChange(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:558
	// ("cv::LevMarq::Settings::setCheckRelEnergyChange", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_check_rel_energy_change_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setCheckRelEnergyChange_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setCheckMinGradient(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:559
	// ("cv::LevMarq::Settings::setCheckMinGradient", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_check_min_gradient_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setCheckMinGradient_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setCheckStepNorm(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:560
	// ("cv::LevMarq::Settings::setCheckStepNorm", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_check_step_norm_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setCheckStepNorm_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setGeodesic(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:561
	// ("cv::LevMarq::Settings::setGeodesic", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_geodesic_1(&mut self, v: bool) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setGeodesic_bool(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setHGeo(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:562
	// ("cv::LevMarq::Settings::setHGeo", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_h_geo_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setHGeo_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setGeoScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:563
	// ("cv::LevMarq::Settings::setGeoScale", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_geo_scale_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setGeoScale_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setStepNormTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:564
	// ("cv::LevMarq::Settings::setStepNormTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_step_norm_tolerance_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setStepNormTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setRelEnergyDeltaTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:565
	// ("cv::LevMarq::Settings::setRelEnergyDeltaTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_rel_energy_delta_tolerance_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setRelEnergyDeltaTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setMinGradientTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:566
	// ("cv::LevMarq::Settings::setMinGradientTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_min_gradient_tolerance_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setMinGradientTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setSmallEnergyTolerance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:567
	// ("cv::LevMarq::Settings::setSmallEnergyTolerance", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_small_energy_tolerance_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setSmallEnergyTolerance_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setMaxIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:568
	// ("cv::LevMarq::Settings::setMaxIterations", vec![(pred!(mut, ["v"], ["int"]), _)]),
	#[inline]
	fn set_max_iterations_1(&mut self, v: i32) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setMaxIterations_int(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setInitialLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:569
	// ("cv::LevMarq::Settings::setInitialLambda", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_initial_lambda_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setInitialLambda_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setInitialLmUpFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:570
	// ("cv::LevMarq::Settings::setInitialLmUpFactor", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_initial_lm_up_factor_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setInitialLmUpFactor_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setInitialLmDownFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:571
	// ("cv::LevMarq::Settings::setInitialLmDownFactor", vec![(pred!(mut, ["v"], ["double"]), _)]),
	#[inline]
	fn set_initial_lm_down_factor_1(&mut self, v: f64) -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_setInitialLmDownFactor_double(self.as_raw_mut_LevMarq_Settings(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Structure to keep LevMarq settings
///
/// The structure allows a user to pass algorithm parameters along with their names like this:
/// ```C++
/// MySolver solver(nVars, callback, MySolver::Settings().geodesicS(true).geoScale(1.0));
/// ```
///
// Settings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:549
pub struct LevMarq_Settings {
	ptr: *mut c_void,
}

opencv_type_boxed! { LevMarq_Settings }

impl Drop for LevMarq_Settings {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LevMarq_Settings_delete(self.as_raw_mut_LevMarq_Settings()) };
	}
}

unsafe impl Send for LevMarq_Settings {}

impl crate::mod_3d::LevMarq_SettingsTraitConst for LevMarq_Settings {
	#[inline] fn as_raw_LevMarq_Settings(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::LevMarq_SettingsTrait for LevMarq_Settings {
	#[inline] fn as_raw_mut_LevMarq_Settings(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LevMarq_Settings, crate::mod_3d::LevMarq_SettingsTraitConst, as_raw_LevMarq_Settings, crate::mod_3d::LevMarq_SettingsTrait, as_raw_mut_LevMarq_Settings }

impl LevMarq_Settings {
	// Settings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:551
	// ("cv::LevMarq::Settings::Settings", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mod_3d::LevMarq_Settings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LevMarq_Settings_Settings(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::LevMarq_Settings::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for LevMarq_Settings {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LevMarq_Settings")
			.field("jacobi_scaling", &crate::mod_3d::LevMarq_SettingsTraitConst::jacobi_scaling(self))
			.field("up_double", &crate::mod_3d::LevMarq_SettingsTraitConst::up_double(self))
			.field("use_step_quality", &crate::mod_3d::LevMarq_SettingsTraitConst::use_step_quality(self))
			.field("clamp_diagonal", &crate::mod_3d::LevMarq_SettingsTraitConst::clamp_diagonal(self))
			.field("step_norm_inf", &crate::mod_3d::LevMarq_SettingsTraitConst::step_norm_inf(self))
			.field("check_rel_energy_change", &crate::mod_3d::LevMarq_SettingsTraitConst::check_rel_energy_change(self))
			.field("check_min_gradient", &crate::mod_3d::LevMarq_SettingsTraitConst::check_min_gradient(self))
			.field("check_step_norm", &crate::mod_3d::LevMarq_SettingsTraitConst::check_step_norm(self))
			.field("geodesic", &crate::mod_3d::LevMarq_SettingsTraitConst::geodesic(self))
			.field("h_geo", &crate::mod_3d::LevMarq_SettingsTraitConst::h_geo(self))
			.field("geo_scale", &crate::mod_3d::LevMarq_SettingsTraitConst::geo_scale(self))
			.field("step_norm_tolerance", &crate::mod_3d::LevMarq_SettingsTraitConst::step_norm_tolerance(self))
			.field("rel_energy_delta_tolerance", &crate::mod_3d::LevMarq_SettingsTraitConst::rel_energy_delta_tolerance(self))
			.field("min_gradient_tolerance", &crate::mod_3d::LevMarq_SettingsTraitConst::min_gradient_tolerance(self))
			.field("small_energy_tolerance", &crate::mod_3d::LevMarq_SettingsTraitConst::small_energy_tolerance(self))
			.field("max_iterations", &crate::mod_3d::LevMarq_SettingsTraitConst::max_iterations(self))
			.field("initial_lambda", &crate::mod_3d::LevMarq_SettingsTraitConst::initial_lambda(self))
			.field("initial_lm_up_factor", &crate::mod_3d::LevMarq_SettingsTraitConst::initial_lm_up_factor(self))
			.field("initial_lm_down_factor", &crate::mod_3d::LevMarq_SettingsTraitConst::initial_lm_down_factor(self))
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::Octree]
// Octree /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2687
pub trait OctreeTraitConst {
	fn as_raw_Octree(&self) -> *const c_void;

	/// Determine whether the point is within the space range of the specific cube.
	///
	/// ## Parameters
	/// * point: The point coordinates.
	/// ## Returns
	/// If point is in bound, return ture. Otherwise, false.
	// isPointInBound(const Point3f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2752
	// ("cv::Octree::isPointInBound", vec![(pred!(const, ["point"], ["const cv::Point3f*"]), _)]),
	#[inline]
	fn is_point_in_bound(&self, point: core::Point3f) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_isPointInBound_const_const_Point3fR(self.as_raw_Octree(), &point, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// returns true if the rootnode is NULL.
	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2755
	// ("cv::Octree::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_empty_const(self.as_raw_Octree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Radius Nearest Neighbor Search in Octree.
	///
	/// Search all points that are less than or equal to radius.
	/// And return the number of searched points.
	/// ## Parameters
	/// * query: Query point.
	/// * radius: Retrieved radius value.
	/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
	/// can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
	/// can be omitted if not needed
	/// ## Returns
	/// the number of searched points.
	///
	/// ## C++ default parameters
	/// * square_dists: noArray()
	// radiusNNSearch(const Point3f &, float, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2794
	// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points", "squareDists"], ["const cv::Point3f*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn radius_nn_search(&self, query: core::Point3f, radius: f32, points: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<i32> {
		output_array_arg!(points);
		output_array_arg!(square_dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, radius, points.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Radius Nearest Neighbor Search in Octree.
	///
	/// Search all points that are less than or equal to radius.
	/// And return the number of searched points.
	/// ## Parameters
	/// * query: Query point.
	/// * radius: Retrieved radius value.
	/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
	/// can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
	/// can be omitted if not needed
	/// ## Returns
	/// the number of searched points.
	///
	/// ## Note
	/// This alternative version of [OctreeTraitConst::radius_nn_search] function uses the following default values for its arguments:
	/// * square_dists: noArray()
	// cv::Octree::radiusNNSearch(SimpleClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2794
	// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points"], ["const cv::Point3f*", "float", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn radius_nn_search_def(&self, query: core::Point3f, radius: f32, points: &mut impl ToOutputArray) -> Result<i32> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR(self.as_raw_Octree(), &query, radius, points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Radius Nearest Neighbor Search in Octree.
	///
	/// Search all points that are less than or equal to radius.
	/// And return the number of searched points.
	/// ## Parameters
	/// * query: Query point.
	/// * radius: Retrieved radius value.
	/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
	/// can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
	/// can be omitted if not needed
	/// ## Returns
	/// the number of searched points.
	///
	/// ## Overloaded parameters
	///
	///  Radius Nearest Neighbor Search in Octree.
	///
	/// Search all points that are less than or equal to radius.
	/// And return the number of searched points.
	/// * query: Query point.
	/// * radius: Retrieved radius value.
	/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
	/// can be replaced by noArray() if not needed
	/// * colors: Color output. Contains colors corresponding to points in pointSet, can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
	/// can be replaced by noArray() if not needed
	/// ## Returns
	/// the number of searched points.
	// radiusNNSearch(const Point3f &, float, OutputArray, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2810
	// ("cv::Octree::radiusNNSearch", vec![(pred!(const, ["query", "radius", "points", "colors", "squareDists"], ["const cv::Point3f*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn radius_nn_search_1(&self, query: core::Point3f, radius: f32, points: &mut impl ToOutputArray, colors: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<i32> {
		output_array_arg!(points);
		output_array_arg!(colors);
		output_array_arg!(square_dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, radius, points.as_raw__OutputArray(), colors.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// K Nearest Neighbor Search in Octree.
	///
	/// Find the K nearest neighbors to the query point.
	/// ## Parameters
	/// * query: Query point.
	/// * K: amount of nearest neighbors to find
	/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
	/// can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
	/// can be omitted if not needed
	///
	/// ## C++ default parameters
	/// * square_dists: noArray()
	// KNNSearch(const Point3f &, const int, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2822
	// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points", "squareDists"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn knn_search(&self, query: core::Point3f, k: i32, points: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(square_dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, k, points.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// K Nearest Neighbor Search in Octree.
	///
	/// Find the K nearest neighbors to the query point.
	/// ## Parameters
	/// * query: Query point.
	/// * K: amount of nearest neighbors to find
	/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
	/// can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
	/// can be omitted if not needed
	///
	/// ## Note
	/// This alternative version of [OctreeTraitConst::knn_search] function uses the following default values for its arguments:
	/// * square_dists: noArray()
	// cv::Octree::KNNSearch(SimpleClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2822
	// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn knn_search_def(&self, query: core::Point3f, k: i32, points: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR(self.as_raw_Octree(), &query, k, points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// K Nearest Neighbor Search in Octree.
	///
	/// Find the K nearest neighbors to the query point.
	/// ## Parameters
	/// * query: Query point.
	/// * K: amount of nearest neighbors to find
	/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
	/// can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
	/// can be omitted if not needed
	///
	/// ## Overloaded parameters
	///
	///  K Nearest Neighbor Search in Octree.
	///
	/// Find the K nearest neighbors to the query point.
	/// * query: Query point.
	/// * K: amount of nearest neighbors to find
	/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
	/// can be replaced by noArray() if not needed
	/// * colors: Color output. Contains colors corresponding to points in pointSet, can be replaced by noArray() if not needed
	/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
	/// can be replaced by noArray() if not needed
	// KNNSearch(const Point3f &, const int, OutputArray, OutputArray, OutputArray)(SimpleClass, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2836
	// ("cv::Octree::KNNSearch", vec![(pred!(const, ["query", "K", "points", "colors", "squareDists"], ["const cv::Point3f*", "const int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn knn_search_1(&self, query: core::Point3f, k: i32, points: &mut impl ToOutputArray, colors: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(colors);
		output_array_arg!(square_dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, k, points.as_raw__OutputArray(), colors.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::Octree]
pub trait OctreeTrait: crate::mod_3d::OctreeTraitConst {
	fn as_raw_mut_Octree(&mut self) -> *mut c_void;

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// Insert a point data with color to a OctreeNode.
	///
	/// ## Parameters
	/// * point: The point data in Point3f format.
	/// * color: The color attribute of point in Point3f format.
	/// ## Returns
	/// Returns whether the insertion is successful.
	///
	/// ## C++ default parameters
	/// * color: {}
	// insertPoint(const Point3f &, const Point3f &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2745
	// ("cv::Octree::insertPoint", vec![(pred!(mut, ["point", "color"], ["const cv::Point3f*", "const cv::Point3f*"]), _)]),
	#[inline]
	fn insert_point(&mut self, point: core::Point3f, color: core::Point3f) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_insertPoint_const_Point3fR_const_Point3fR(self.as_raw_mut_Octree(), &point, &color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// Insert a point data with color to a OctreeNode.
	///
	/// ## Parameters
	/// * point: The point data in Point3f format.
	/// * color: The color attribute of point in Point3f format.
	/// ## Returns
	/// Returns whether the insertion is successful.
	///
	/// ## Note
	/// This alternative version of [OctreeTrait::insert_point] function uses the following default values for its arguments:
	/// * color: {}
	// cv::Octree::insertPoint(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2745
	// ("cv::Octree::insertPoint", vec![(pred!(mut, ["point"], ["const cv::Point3f*"]), _)]),
	#[inline]
	fn insert_point_def(&mut self, point: core::Point3f) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_insertPoint_const_Point3fR(self.as_raw_mut_Octree(), &point, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Reset all octree parameter.
	///
	/// Clear all the nodes of the octree and initialize the parameters.
	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2761
	// ("cv::Octree::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_clear(self.as_raw_mut_Octree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Delete a given point from the Octree.
	///
	/// Delete the corresponding element from the pointList in the corresponding leaf node. If the leaf node
	/// does not contain other points after deletion, this node will be deleted. In the same way,
	/// its parent node may also be deleted if its last child is deleted.
	/// ## Parameters
	/// * point: The point coordinates, comparison is epsilon-based
	/// ## Returns
	/// return ture if the point is deleted successfully.
	// deletePoint(const Point3f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2771
	// ("cv::Octree::deletePoint", vec![(pred!(mut, ["point"], ["const cv::Point3f*"]), _)]),
	#[inline]
	fn delete_point(&mut self, point: core::Point3f) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_deletePoint_const_Point3fR(self.as_raw_mut_Octree(), &point, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// restore point cloud data from Octree.
	///
	/// Restore the point cloud data from existing octree. The points in same leaf node will be seen as the same point.
	/// This point is the center of the leaf node. If the resolution is small, it will work as a downSampling function.
	/// ## Parameters
	/// * restoredPointCloud: The output point cloud data, can be replaced by noArray() if not needed
	/// * restoredColor: The color attribute of point cloud data, can be omitted if not needed
	///
	/// ## C++ default parameters
	/// * restored_color: noArray()
	// getPointCloudByOctree(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2780
	// ("cv::Octree::getPointCloudByOctree", vec![(pred!(mut, ["restoredPointCloud", "restoredColor"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_point_cloud_by_octree(&mut self, restored_point_cloud: &mut impl ToOutputArray, restored_color: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(restored_point_cloud);
		output_array_arg!(restored_color);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_getPointCloudByOctree_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Octree(), restored_point_cloud.as_raw__OutputArray(), restored_color.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// restore point cloud data from Octree.
	///
	/// Restore the point cloud data from existing octree. The points in same leaf node will be seen as the same point.
	/// This point is the center of the leaf node. If the resolution is small, it will work as a downSampling function.
	/// ## Parameters
	/// * restoredPointCloud: The output point cloud data, can be replaced by noArray() if not needed
	/// * restoredColor: The color attribute of point cloud data, can be omitted if not needed
	///
	/// ## Note
	/// This alternative version of [OctreeTrait::get_point_cloud_by_octree] function uses the following default values for its arguments:
	/// * restored_color: noArray()
	// cv::Octree::getPointCloudByOctree(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2780
	// ("cv::Octree::getPointCloudByOctree", vec![(pred!(mut, ["restoredPointCloud"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_point_cloud_by_octree_def(&mut self, restored_point_cloud: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(restored_point_cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_getPointCloudByOctree_const__OutputArrayR(self.as_raw_mut_Octree(), restored_point_cloud.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Octree for 3D vision.
///
/// In 3D vision filed, the Octree is used to process and accelerate the pointcloud data. The class Octree represents
/// the Octree data structure. Each Octree will have a fixed depth. The depth of Octree refers to the distance from
/// the root node to the leaf node.All OctreeNodes will not exceed this depth.Increasing the depth will increase
/// the amount of calculation exponentially. And the small number of depth refers low resolution of Octree.
/// Each node contains 8 children, which are used to divide the space cube into eight parts. Each octree node represents
/// a cube. And these eight children will have a fixed order, the order is described as follows:
///
/// For illustration, assume,
///
/// rootNode: origin == (0, 0, 0), size == 2
///
/// Then,
///
/// children[0]: origin == (0, 0, 0), size == 1
///
/// children[1]: origin == (1, 0, 0), size == 1, along X-axis next to child 0
///
/// children[2]: origin == (0, 1, 0), size == 1, along Y-axis next to child 0
///
/// children[3]: origin == (1, 1, 0), size == 1, in X-Y plane
///
/// children[4]: origin == (0, 0, 1), size == 1, along Z-axis next to child 0
///
/// children[5]: origin == (1, 0, 1), size == 1, in X-Z plane
///
/// children[6]: origin == (0, 1, 1), size == 1, in Y-Z plane
///
/// children[7]: origin == (1, 1, 1), size == 1, furthest from child 0
// Octree /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2687
pub struct Octree {
	ptr: *mut c_void,
}

opencv_type_boxed! { Octree }

impl Drop for Octree {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Octree_delete(self.as_raw_mut_Octree()) };
	}
}

unsafe impl Send for Octree {}

impl crate::mod_3d::OctreeTraitConst for Octree {
	#[inline] fn as_raw_Octree(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::OctreeTrait for Octree {
	#[inline] fn as_raw_mut_Octree(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Octree, crate::mod_3d::OctreeTraitConst, as_raw_Octree, crate::mod_3d::OctreeTrait, as_raw_mut_Octree }

impl Octree {
	/// Default constructor.
	// Octree()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2691
	// ("cv::Octree::Octree", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mod_3d::Octree> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_Octree(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::Octree::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// Creates an empty Octree with given maximum depth
	///
	/// ## Parameters
	/// * maxDepth: The max depth of the Octree
	/// * size: bounding box size for the Octree
	/// * origin: Initial center coordinate
	/// * withColors: Whether to keep per-point colors or not
	/// ## Returns
	/// resulting Octree
	///
	/// ## C++ default parameters
	/// * origin: {}
	/// * with_colors: false
	// createWithDepth(int, double, const Point3f &, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2702
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "size", "origin", "withColors"], ["int", "double", "const cv::Point3f*", "bool"]), _)]),
	#[inline]
	pub fn create_with_depth(max_depth: i32, size: f64, origin: core::Point3f, with_colors: bool) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithDepth_int_double_const_Point3fR_bool(max_depth, size, &origin, with_colors, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// Creates an empty Octree with given maximum depth
	///
	/// ## Parameters
	/// * maxDepth: The max depth of the Octree
	/// * size: bounding box size for the Octree
	/// * origin: Initial center coordinate
	/// * withColors: Whether to keep per-point colors or not
	/// ## Returns
	/// resulting Octree
	///
	/// ## Note
	/// This alternative version of [Octree::create_with_depth] function uses the following default values for its arguments:
	/// * origin: {}
	/// * with_colors: false
	// cv::Octree::createWithDepth(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2702
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "size"], ["int", "double"]), _)]),
	#[inline]
	pub fn create_with_depth_def(max_depth: i32, size: f64) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithDepth_int_double(max_depth, size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// Create an Octree from the PointCloud data with the specific maxDepth
	///
	/// ## Parameters
	/// * maxDepth: Max depth of the octree
	/// * pointCloud: point cloud data, should be 3-channel float array
	/// * colors: color attribute of point cloud in the same 3-channel float format
	/// ## Returns
	/// resulting Octree
	///
	/// ## C++ default parameters
	/// * colors: noArray()
	// createWithDepth(int, InputArray, InputArray)(Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2712
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "pointCloud", "colors"], ["int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create_with_depth_1(max_depth: i32, point_cloud: &impl ToInputArray, colors: &impl ToInputArray) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		input_array_arg!(point_cloud);
		input_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithDepth_int_const__InputArrayR_const__InputArrayR(max_depth, point_cloud.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// Create an Octree from the PointCloud data with the specific maxDepth
	///
	/// ## Parameters
	/// * maxDepth: Max depth of the octree
	/// * pointCloud: point cloud data, should be 3-channel float array
	/// * colors: color attribute of point cloud in the same 3-channel float format
	/// ## Returns
	/// resulting Octree
	///
	/// ## Note
	/// This alternative version of [Octree::create_with_depth] function uses the following default values for its arguments:
	/// * colors: noArray()
	// cv::Octree::createWithDepth(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2712
	// ("cv::Octree::createWithDepth", vec![(pred!(mut, ["maxDepth", "pointCloud"], ["int", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create_with_depth_def_1(max_depth: i32, point_cloud: &impl ToInputArray) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		input_array_arg!(point_cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithDepth_int_const__InputArrayR(max_depth, point_cloud.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// Creates an empty Octree with given resolution
	///
	/// ## Parameters
	/// * resolution: The size of the octree leaf node
	/// * size: bounding box size for the Octree
	/// * origin: Initial center coordinate
	/// * withColors: Whether to keep per-point colors or not
	/// ## Returns
	/// resulting Octree
	///
	/// ## C++ default parameters
	/// * origin: {}
	/// * with_colors: false
	// createWithResolution(double, double, const Point3f &, bool)(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2723
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "size", "origin", "withColors"], ["double", "double", "const cv::Point3f*", "bool"]), _)]),
	#[inline]
	pub fn create_with_resolution(resolution: f64, size: f64, origin: core::Point3f, with_colors: bool) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithResolution_double_double_const_Point3fR_bool(resolution, size, &origin, with_colors, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// Creates an empty Octree with given resolution
	///
	/// ## Parameters
	/// * resolution: The size of the octree leaf node
	/// * size: bounding box size for the Octree
	/// * origin: Initial center coordinate
	/// * withColors: Whether to keep per-point colors or not
	/// ## Returns
	/// resulting Octree
	///
	/// ## Note
	/// This alternative version of [Octree::create_with_resolution] function uses the following default values for its arguments:
	/// * origin: {}
	/// * with_colors: false
	// cv::Octree::createWithResolution(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2723
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "size"], ["double", "double"]), _)]),
	#[inline]
	pub fn create_with_resolution_def(resolution: f64, size: f64) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithResolution_double_double(resolution, size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// Create an Octree from the PointCloud data with the specific resolution
	///
	/// ## Parameters
	/// * resolution: The size of the octree leaf node
	/// * pointCloud: point cloud data, should be 3-channel float array
	/// * colors: color attribute of point cloud in the same 3-channel float format
	/// ## Returns
	/// resulting octree
	///
	/// ## C++ default parameters
	/// * colors: noArray()
	// createWithResolution(double, InputArray, InputArray)(Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2733
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "pointCloud", "colors"], ["double", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create_with_resolution_1(resolution: f64, point_cloud: &impl ToInputArray, colors: &impl ToInputArray) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		input_array_arg!(point_cloud);
		input_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithResolution_double_const__InputArrayR_const__InputArrayR(resolution, point_cloud.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// @overload
	/// Create an Octree from the PointCloud data with the specific resolution
	///
	/// ## Parameters
	/// * resolution: The size of the octree leaf node
	/// * pointCloud: point cloud data, should be 3-channel float array
	/// * colors: color attribute of point cloud in the same 3-channel float format
	/// ## Returns
	/// resulting octree
	///
	/// ## Note
	/// This alternative version of [Octree::create_with_resolution] function uses the following default values for its arguments:
	/// * colors: noArray()
	// cv::Octree::createWithResolution(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2733
	// ("cv::Octree::createWithResolution", vec![(pred!(mut, ["resolution", "pointCloud"], ["double", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create_with_resolution_def_1(resolution: f64, point_cloud: &impl ToInputArray) -> Result<core::Ptr<crate::mod_3d::Octree>> {
		input_array_arg!(point_cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Octree_createWithResolution_double_const__InputArrayR(resolution, point_cloud.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::Octree>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Octree {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Octree")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::Odometry]
// Odometry /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:38
pub trait OdometryTraitConst {
	fn as_raw_Odometry(&self) -> *const c_void;

	/// Prepare frame for odometry calculation
	/// ## Parameters
	/// * frame: odometry prepare this frame as src frame and dst frame simultaneously
	// prepareFrame(OdometryFrame &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:49
	// ("cv::Odometry::prepareFrame", vec![(pred!(const, ["frame"], ["cv::OdometryFrame*"]), _)]),
	#[inline]
	fn prepare_frame(&self, frame: &mut impl crate::mod_3d::OdometryFrameTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_prepareFrame_const_OdometryFrameR(self.as_raw_Odometry(), frame.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Prepare frame for odometry calculation
	/// ## Parameters
	/// * srcFrame: frame will be prepared as src frame ("original" image)
	/// * dstFrame: frame will be prepared as dsr frame ("rotated" image)
	// prepareFrames(OdometryFrame &, OdometryFrame &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:55
	// ("cv::Odometry::prepareFrames", vec![(pred!(const, ["srcFrame", "dstFrame"], ["cv::OdometryFrame*", "cv::OdometryFrame*"]), _)]),
	#[inline]
	fn prepare_frames(&self, src_frame: &mut impl crate::mod_3d::OdometryFrameTrait, dst_frame: &mut impl crate::mod_3d::OdometryFrameTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_prepareFrames_const_OdometryFrameR_OdometryFrameR(self.as_raw_Odometry(), src_frame.as_raw_mut_OdometryFrame(), dst_frame.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compute Rigid Transformation between two frames so that Rt * src = dst
	/// Both frames, source and destination, should have been prepared by calling prepareFrame() first
	///
	/// ## Parameters
	/// * srcFrame: src frame ("original" image)
	/// * dstFrame: dst frame ("rotated" image)
	/// * Rt: Rigid transformation, which will be calculated, in form:
	/// { R_11 R_12 R_13 t_1
	///   R_21 R_22 R_23 t_2
	///   R_31 R_32 R_33 t_3
	///   0    0    0    1  }
	/// ## Returns
	/// true on success, false if failed to find the transformation
	// compute(const OdometryFrame &, const OdometryFrame &, OutputArray)(TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:69
	// ("cv::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["const cv::OdometryFrame*", "const cv::OdometryFrame*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute(&self, src_frame: &impl crate::mod_3d::OdometryFrameTraitConst, dst_frame: &impl crate::mod_3d::OdometryFrameTraitConst, rt: &mut impl ToOutputArray) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_compute_const_const_OdometryFrameR_const_OdometryFrameR_const__OutputArrayR(self.as_raw_Odometry(), src_frame.as_raw_OdometryFrame(), dst_frame.as_raw_OdometryFrame(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compute Rigid Transformation between two frames so that Rt * src = dst
	///
	/// ## Parameters
	/// * srcDepth: source depth ("original" image)
	/// * dstDepth: destination depth ("rotated" image)
	/// * Rt: Rigid transformation, which will be calculated, in form:
	/// { R_11 R_12 R_13 t_1
	///   R_21 R_22 R_23 t_2
	///   R_31 R_32 R_33 t_3
	///   0    0    0    1  }
	/// ## Returns
	/// true on success, false if failed to find the transformation
	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:82
	// ("cv::Odometry::compute", vec![(pred!(const, ["srcDepth", "dstDepth", "Rt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_1(&self, src_depth: &impl ToInputArray, dst_depth: &impl ToInputArray, rt: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(src_depth);
		input_array_arg!(dst_depth);
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_Odometry(), src_depth.as_raw__InputArray(), dst_depth.as_raw__InputArray(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compute Rigid Transformation between two frames so that Rt * src = dst
	///
	/// ## Parameters
	/// * srcDepth: source depth ("original" image)
	/// * srcRGB: source RGB
	/// * dstDepth: destination depth ("rotated" image)
	/// * dstRGB: destination RGB
	/// * Rt: Rigid transformation, which will be calculated, in form:
	/// { R_11 R_12 R_13 t_1
	///   R_21 R_22 R_23 t_2
	///   R_31 R_32 R_33 t_3
	///   0    0    0    1  }
	/// ## Returns
	/// true on success, false if failed to find the transformation
	// compute(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:97
	// ("cv::Odometry::compute", vec![(pred!(const, ["srcDepth", "srcRGB", "dstDepth", "dstRGB", "Rt"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_2(&self, src_depth: &impl ToInputArray, src_rgb: &impl ToInputArray, dst_depth: &impl ToInputArray, dst_rgb: &impl ToInputArray, rt: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(src_depth);
		input_array_arg!(src_rgb);
		input_array_arg!(dst_depth);
		input_array_arg!(dst_rgb);
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_Odometry(), src_depth.as_raw__InputArray(), src_rgb.as_raw__InputArray(), dst_depth.as_raw__InputArray(), dst_rgb.as_raw__InputArray(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the normals computer object used for normals calculation (if presented).
	/// The normals computer is generated at first need during prepareFrame when normals are required for the ICP algorithm
	/// but not presented by a user. Re-generated each time the related settings change or a new frame arrives with the different size.
	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:104
	// ("cv::Odometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_normals_computer(&self) -> Result<core::Ptr<crate::mod_3d::RgbdNormals>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_getNormalsComputer_const(self.as_raw_Odometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::Odometry]
pub trait OdometryTrait: crate::mod_3d::OdometryTraitConst {
	fn as_raw_mut_Odometry(&mut self) -> *mut c_void;

}

// Odometry /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:38
pub struct Odometry {
	ptr: *mut c_void,
}

opencv_type_boxed! { Odometry }

impl Drop for Odometry {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Odometry_delete(self.as_raw_mut_Odometry()) };
	}
}

unsafe impl Send for Odometry {}

impl crate::mod_3d::OdometryTraitConst for Odometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::OdometryTrait for Odometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Odometry, crate::mod_3d::OdometryTraitConst, as_raw_Odometry, crate::mod_3d::OdometryTrait, as_raw_mut_Odometry }

impl Odometry {
	// Odometry()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:41
	// ("cv::Odometry::Odometry", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mod_3d::Odometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_Odometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::Odometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	// Odometry(OdometryType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:42
	// ("cv::Odometry::Odometry", vec![(pred!(mut, ["otype"], ["cv::OdometryType"]), _)]),
	#[inline]
	pub fn new(otype: crate::mod_3d::OdometryType) -> Result<crate::mod_3d::Odometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_Odometry_OdometryType(otype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::Odometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	// Odometry(OdometryType, const OdometrySettings &, OdometryAlgoType)(Enum, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry.hpp:43
	// ("cv::Odometry::Odometry", vec![(pred!(mut, ["otype", "settings", "algtype"], ["cv::OdometryType", "const cv::OdometrySettings*", "cv::OdometryAlgoType"]), _)]),
	#[inline]
	pub fn new_1(otype: crate::mod_3d::OdometryType, settings: &impl crate::mod_3d::OdometrySettingsTraitConst, algtype: crate::mod_3d::OdometryAlgoType) -> Result<crate::mod_3d::Odometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Odometry_Odometry_OdometryType_const_OdometrySettingsR_OdometryAlgoType(otype, settings.as_raw_OdometrySettings(), algtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::Odometry::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Odometry {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Odometry")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::OdometryFrame]
// OdometryFrame /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:36
pub trait OdometryFrameTraitConst {
	fn as_raw_OdometryFrame(&self) -> *const c_void;

	/// Get the original user-provided BGR/Gray image
	///
	/// ## Parameters
	/// * image: Output image
	// getImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:57
	// ("cv::OdometryFrame::getImage", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_image(&self, image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getImage_const_const__OutputArrayR(self.as_raw_OdometryFrame(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the gray image generated from the user-provided BGR/Gray image
	///
	/// ## Parameters
	/// * image: Output image
	// getGrayImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:63
	// ("cv::OdometryFrame::getGrayImage", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_gray_image(&self, image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getGrayImage_const_const__OutputArrayR(self.as_raw_OdometryFrame(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the original user-provided depth image
	///
	/// ## Parameters
	/// * depth: Output image
	// getDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:69
	// ("cv::OdometryFrame::getDepth", vec![(pred!(const, ["depth"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_depth(&self, depth: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getDepth_const_const__OutputArrayR(self.as_raw_OdometryFrame(), depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the depth image generated from the user-provided one after conversion, rescale or filtering for ICP algorithm needs
	///
	/// ## Parameters
	/// * depth: Output image
	// getProcessedDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:75
	// ("cv::OdometryFrame::getProcessedDepth", vec![(pred!(const, ["depth"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_processed_depth(&self, depth: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getProcessedDepth_const_const__OutputArrayR(self.as_raw_OdometryFrame(), depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the valid pixels mask generated for the ICP calculations intersected with the user-provided mask
	///
	/// ## Parameters
	/// * mask: Output image
	// getMask(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:81
	// ("cv::OdometryFrame::getMask", vec![(pred!(const, ["mask"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_mask(&self, mask: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getMask_const_const__OutputArrayR(self.as_raw_OdometryFrame(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the normals image either generated for the ICP calculations or user-provided
	///
	/// ## Parameters
	/// * normals: Output image
	// getNormals(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:87
	// ("cv::OdometryFrame::getNormals", vec![(pred!(const, ["normals"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_normals(&self, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getNormals_const_const__OutputArrayR(self.as_raw_OdometryFrame(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the amount of levels in pyramids (all of them if not empty should have the same number of levels)
	/// or 0 if no pyramids were prepared yet
	// getPyramidLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:93
	// ("cv::OdometryFrame::getPyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pyramid_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getPyramidLevels_const(self.as_raw_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the image generated for the ICP calculations from one of the pyramids specified by pyrType. Returns empty image if
	/// the pyramid is empty or there's no such pyramid level
	///
	/// ## Parameters
	/// * img: Output image
	/// * pyrType: Type of pyramid
	/// * level: Level in the pyramid
	// getPyramidAt(OutputArray, OdometryFramePyramidType, size_t)(OutputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:102
	// ("cv::OdometryFrame::getPyramidAt", vec![(pred!(const, ["img", "pyrType", "level"], ["const cv::_OutputArray*", "cv::OdometryFramePyramidType", "size_t"]), _)]),
	#[inline]
	fn get_pyramid_at(&self, img: &mut impl ToOutputArray, pyr_type: crate::mod_3d::OdometryFramePyramidType, level: size_t) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_getPyramidAt_const_const__OutputArrayR_OdometryFramePyramidType_size_t(self.as_raw_OdometryFrame(), img.as_raw__OutputArray(), pyr_type, level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::OdometryFrame]
pub trait OdometryFrameTrait: crate::mod_3d::OdometryFrameTraitConst {
	fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void;

}

/// An object that keeps per-frame data for Odometry algorithms from user-provided images to algorithm-specific precalculated data.
/// When not empty, it contains a depth image, a mask of valid pixels and a set of pyramids generated from that data.
/// A BGR/Gray image and normals are optional.
/// OdometryFrame is made to be used together with Odometry class to reuse precalculated data between Rt data calculations.
/// A correct way to do that is to call Odometry::prepareFrames() on prev and next frames and then pass them to Odometry::compute() method.
// OdometryFrame /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:36
pub struct OdometryFrame {
	ptr: *mut c_void,
}

opencv_type_boxed! { OdometryFrame }

impl Drop for OdometryFrame {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_OdometryFrame_delete(self.as_raw_mut_OdometryFrame()) };
	}
}

unsafe impl Send for OdometryFrame {}

impl crate::mod_3d::OdometryFrameTraitConst for OdometryFrame {
	#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::OdometryFrameTrait for OdometryFrame {
	#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OdometryFrame, crate::mod_3d::OdometryFrameTraitConst, as_raw_OdometryFrame, crate::mod_3d::OdometryFrameTrait, as_raw_mut_OdometryFrame }

impl OdometryFrame {
	/// Construct a new OdometryFrame object. All non-empty images should have the same size.
	///
	/// ## Parameters
	/// * depth: A depth image, should be CV_8UC1
	/// * image: An BGR or grayscale image (or noArray() if it's not required for used ICP algorithm).
	/// Should be CV_8UC3 or CV_8C4 if it's BGR image or CV_8UC1 if it's grayscale. If it's BGR then it's converted to grayscale
	/// image automatically.
	/// * mask: A user-provided mask of valid pixels, should be CV_8UC1
	/// * normals: A user-provided normals to the depth surface, should be CV_32FC4
	///
	/// ## C++ default parameters
	/// * depth: noArray()
	/// * image: noArray()
	/// * mask: noArray()
	/// * normals: noArray()
	// OdometryFrame(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:49
	// ("cv::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["depth", "image", "mask", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn new(depth: &impl ToInputArray, image: &impl ToInputArray, mask: &impl ToInputArray, normals: &impl ToInputArray) -> Result<crate::mod_3d::OdometryFrame> {
		input_array_arg!(depth);
		input_array_arg!(image);
		input_array_arg!(mask);
		input_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_OdometryFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(depth.as_raw__InputArray(), image.as_raw__InputArray(), mask.as_raw__InputArray(), normals.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Construct a new OdometryFrame object. All non-empty images should have the same size.
	///
	/// ## Parameters
	/// * depth: A depth image, should be CV_8UC1
	/// * image: An BGR or grayscale image (or noArray() if it's not required for used ICP algorithm).
	/// Should be CV_8UC3 or CV_8C4 if it's BGR image or CV_8UC1 if it's grayscale. If it's BGR then it's converted to grayscale
	/// image automatically.
	/// * mask: A user-provided mask of valid pixels, should be CV_8UC1
	/// * normals: A user-provided normals to the depth surface, should be CV_32FC4
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * depth: noArray()
	/// * image: noArray()
	/// * mask: noArray()
	/// * normals: noArray()
	// cv::OdometryFrame::OdometryFrame() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_frame.hpp:49
	// ("cv::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::mod_3d::OdometryFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometryFrame_OdometryFrame(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for OdometryFrame {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_OdometryFrame_implicitClone_const(self.as_raw_OdometryFrame())) }
	}
}

impl std::fmt::Debug for OdometryFrame {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("OdometryFrame")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::OdometrySettings]
// OdometrySettings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:13
pub trait OdometrySettingsTraitConst {
	fn as_raw_OdometrySettings(&self) -> *const c_void;

	// getCameraMatrix(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:21
	// ("cv::OdometrySettings::getCameraMatrix", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_camera_matrix(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getCameraMatrix_const_const__OutputArrayR(self.as_raw_OdometrySettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterCounts(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:23
	// ("cv::OdometrySettings::getIterCounts", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_iter_counts(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getIterCounts_const_const__OutputArrayR(self.as_raw_OdometrySettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:26
	// ("cv::OdometrySettings::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMinDepth_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:28
	// ("cv::OdometrySettings::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMaxDepth_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:30
	// ("cv::OdometrySettings::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMaxDepthDiff_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:32
	// ("cv::OdometrySettings::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMaxPointsPart_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSobelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:35
	// ("cv::OdometrySettings::getSobelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sobel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getSobelSize_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSobelScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:37
	// ("cv::OdometrySettings::getSobelScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sobel_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getSobelScale_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalWinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:40
	// ("cv::OdometrySettings::getNormalWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_normal_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getNormalWinSize_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalDiffThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:42
	// ("cv::OdometrySettings::getNormalDiffThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_normal_diff_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getNormalDiffThreshold_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalMethod()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:44
	// ("cv::OdometrySettings::getNormalMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_normal_method(&self) -> Result<crate::mod_3d::RgbdNormals_RgbdNormalsMethod> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getNormalMethod_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAngleThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:47
	// ("cv::OdometrySettings::getAngleThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_angle_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getAngleThreshold_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:49
	// ("cv::OdometrySettings::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMaxTranslation_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:51
	// ("cv::OdometrySettings::getMaxRotation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_rotation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMaxRotation_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinGradientMagnitude()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:54
	// ("cv::OdometrySettings::getMinGradientMagnitude", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_gradient_magnitude(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMinGradientMagnitude_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinGradientMagnitudes(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:56
	// ("cv::OdometrySettings::getMinGradientMagnitudes", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_min_gradient_magnitudes(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_getMinGradientMagnitudes_const_const__OutputArrayR(self.as_raw_OdometrySettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::OdometrySettings]
pub trait OdometrySettingsTrait: crate::mod_3d::OdometrySettingsTraitConst {
	fn as_raw_mut_OdometrySettings(&mut self) -> *mut c_void;

	// operator=(const OdometrySettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:18
	// ("cv::OdometrySettings::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::OdometrySettings*"]), _)]),
	#[inline]
	fn set(&mut self, unnamed: &impl crate::mod_3d::OdometrySettingsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_operatorST_const_OdometrySettingsR(self.as_raw_mut_OdometrySettings(), unnamed.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCameraMatrix(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:20
	// ("cv::OdometrySettings::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setCameraMatrix_const__InputArrayR(self.as_raw_mut_OdometrySettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterCounts(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:22
	// ("cv::OdometrySettings::setIterCounts", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_iter_counts(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setIterCounts_const__InputArrayR(self.as_raw_mut_OdometrySettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:25
	// ("cv::OdometrySettings::setMinDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMinDepth_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:27
	// ("cv::OdometrySettings::setMaxDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMaxDepth_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:29
	// ("cv::OdometrySettings::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMaxDepthDiff_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:31
	// ("cv::OdometrySettings::setMaxPointsPart", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMaxPointsPart_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSobelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:34
	// ("cv::OdometrySettings::setSobelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_sobel_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setSobelSize_int(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSobelScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:36
	// ("cv::OdometrySettings::setSobelScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_sobel_scale(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setSobelScale_double(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNormalWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:39
	// ("cv::OdometrySettings::setNormalWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_normal_win_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setNormalWinSize_int(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNormalDiffThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:41
	// ("cv::OdometrySettings::setNormalDiffThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_normal_diff_threshold(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setNormalDiffThreshold_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNormalMethod(RgbdNormals::RgbdNormalsMethod)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:43
	// ("cv::OdometrySettings::setNormalMethod", vec![(pred!(mut, ["nm"], ["cv::RgbdNormals::RgbdNormalsMethod"]), _)]),
	#[inline]
	fn set_normal_method(&mut self, nm: crate::mod_3d::RgbdNormals_RgbdNormalsMethod) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setNormalMethod_RgbdNormalsMethod(self.as_raw_mut_OdometrySettings(), nm, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setAngleThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:46
	// ("cv::OdometrySettings::setAngleThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_angle_threshold(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setAngleThreshold_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:48
	// ("cv::OdometrySettings::setMaxTranslation", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMaxTranslation_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:50
	// ("cv::OdometrySettings::setMaxRotation", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_rotation(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMaxRotation_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinGradientMagnitude(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:53
	// ("cv::OdometrySettings::setMinGradientMagnitude", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_min_gradient_magnitude(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMinGradientMagnitude_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinGradientMagnitudes(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:55
	// ("cv::OdometrySettings::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_setMinGradientMagnitudes_const__InputArrayR(self.as_raw_mut_OdometrySettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// OdometrySettings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:13
pub struct OdometrySettings {
	ptr: *mut c_void,
}

opencv_type_boxed! { OdometrySettings }

impl Drop for OdometrySettings {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_OdometrySettings_delete(self.as_raw_mut_OdometrySettings()) };
	}
}

unsafe impl Send for OdometrySettings {}

impl crate::mod_3d::OdometrySettingsTraitConst for OdometrySettings {
	#[inline] fn as_raw_OdometrySettings(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::OdometrySettingsTrait for OdometrySettings {
	#[inline] fn as_raw_mut_OdometrySettings(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OdometrySettings, crate::mod_3d::OdometrySettingsTraitConst, as_raw_OdometrySettings, crate::mod_3d::OdometrySettingsTrait, as_raw_mut_OdometrySettings }

impl OdometrySettings {
	// OdometrySettings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:16
	// ("cv::OdometrySettings::OdometrySettings", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mod_3d::OdometrySettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_OdometrySettings(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::OdometrySettings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// OdometrySettings(const OdometrySettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/odometry_settings.hpp:17
	// ("cv::OdometrySettings::OdometrySettings", vec![(pred!(mut, ["unnamed"], ["const cv::OdometrySettings*"]), _)]),
	#[inline]
	pub fn copy(unnamed: &impl crate::mod_3d::OdometrySettingsTraitConst) -> Result<crate::mod_3d::OdometrySettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_OdometrySettings_OdometrySettings_const_OdometrySettingsR(unnamed.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::OdometrySettings::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for OdometrySettings {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_OdometrySettings_implicitClone_const(self.as_raw_OdometrySettings())) }
	}
}

impl std::fmt::Debug for OdometrySettings {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("OdometrySettings")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::RegionGrowing3D]
// RegionGrowing3D /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:298
pub trait RegionGrowing3DTraitConst {
	fn as_raw_RegionGrowing3D(&self) -> *const c_void;

	/// Get the minimum size of region.
	// getMinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:342
	// ("cv::RegionGrowing3D::getMinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getMinSize_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the maximum size of region.
	// getMaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:349
	// ("cv::RegionGrowing3D::getMaxSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getMaxSize_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get whether to use the smoothness mode.
	// getSmoothModeFlag()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:357
	// ("cv::RegionGrowing3D::getSmoothModeFlag", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_smooth_mode_flag(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getSmoothModeFlag_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get threshold value of the angle between normals.
	// getSmoothnessThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:364
	// ("cv::RegionGrowing3D::getSmoothnessThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_smoothness_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getSmoothnessThreshold_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get threshold value of curvature.
	// getCurvatureThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:372
	// ("cv::RegionGrowing3D::getCurvatureThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_curvature_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getCurvatureThreshold_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the maximum number of neighbors including itself.
	// getMaxNumberOfNeighbors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:379
	// ("cv::RegionGrowing3D::getMaxNumberOfNeighbors", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_number_of_neighbors(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getMaxNumberOfNeighbors_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the maximum number of regions you want.
	// getNumberOfRegions()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:386
	// ("cv::RegionGrowing3D::getNumberOfRegions", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_number_of_regions(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getNumberOfRegions_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get whether the results need to be sorted you have set.
	// getNeedSort()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:392
	// ("cv::RegionGrowing3D::getNeedSort", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_need_sort(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getNeedSort_const(self.as_raw_RegionGrowing3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the seed points.
	// getSeeds(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:401
	// ("cv::RegionGrowing3D::getSeeds", vec![(pred!(const, ["seeds"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_seeds(&self, seeds: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(seeds);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getSeeds_const_const__OutputArrayR(self.as_raw_RegionGrowing3D(), seeds.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the curvature of each point if you have set.
	// getCurvatures(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:407
	// ("cv::RegionGrowing3D::getCurvatures", vec![(pred!(const, ["curvatures"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_curvatures(&self, curvatures: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(curvatures);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_getCurvatures_const_const__OutputArrayR(self.as_raw_RegionGrowing3D(), curvatures.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::RegionGrowing3D]
pub trait RegionGrowing3DTrait: crate::mod_3d::RegionGrowing3DTraitConst {
	fn as_raw_mut_RegionGrowing3D(&mut self) -> *mut c_void;

	/// Execute segmentation using the Region Growing algorithm.
	///
	/// ## Parameters
	/// * regions_idx:[out] Index information of all points in each region, support
	///               vector<vector<int>>, vector<Mat>.
	/// * labels:[out] The label corresponds to the model number, 0 means it does not belong to
	///               any model, range [0, Number of final resultant models obtained]. Support
	///               vector<int> and Mat.
	/// * input_pts: Original point cloud, support vector<Point3f> and Mat of size Nx3/3xN.
	/// * normals: Normal of each point, support vector<Point3f> and Mat of size Nx3.
	/// * nn_idx: Index information of nearest neighbors of all points. The first nearest
	///               neighbor of each point is itself. Support vector<vector<int>>, vector<Mat> and
	///               Mat of size NxK. If the information in a row is
	///               [0, 2, 1, -5, -1, 4, 7 ... negative number]
	///               it will use only non-negative indexes until it meets a negative number or bound
	///               of this row i.e. [0, 2, 1].
	/// ## Returns
	/// Number of final resultant regions obtained by segmentation.
	// segment(OutputArrayOfArrays, OutputArray, InputArray, InputArray, InputArrayOfArrays)(OutputArray, OutputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:332
	// ("cv::RegionGrowing3D::segment", vec![(pred!(mut, ["regions_idx", "labels", "input_pts", "normals", "nn_idx"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn segment(&mut self, regions_idx: &mut impl ToOutputArray, labels: &mut impl ToOutputArray, input_pts: &impl ToInputArray, normals: &impl ToInputArray, nn_idx: &impl ToInputArray) -> Result<i32> {
		output_array_arg!(regions_idx);
		output_array_arg!(labels);
		input_array_arg!(input_pts);
		input_array_arg!(normals);
		input_array_arg!(nn_idx);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_segment_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_RegionGrowing3D(), regions_idx.as_raw__OutputArray(), labels.as_raw__OutputArray(), input_pts.as_raw__InputArray(), normals.as_raw__InputArray(), nn_idx.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the minimum size of region.
	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:339
	// ("cv::RegionGrowing3D::setMinSize", vec![(pred!(mut, ["min_size"], ["int"]), _)]),
	#[inline]
	fn set_min_size(&mut self, min_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setMinSize_int(self.as_raw_mut_RegionGrowing3D(), min_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the maximum size of region.
	// setMaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:346
	// ("cv::RegionGrowing3D::setMaxSize", vec![(pred!(mut, ["max_size"], ["int"]), _)]),
	#[inline]
	fn set_max_size(&mut self, max_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setMaxSize_int(self.as_raw_mut_RegionGrowing3D(), max_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set whether to use the smoothness mode. Default will be true.
	/// If true it will check the angle between the normal of the current point and the normal of its neighbor.
	/// Otherwise, it will check the angle between the normal of the seed point and the normal of current neighbor.
	// setSmoothModeFlag(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:354
	// ("cv::RegionGrowing3D::setSmoothModeFlag", vec![(pred!(mut, ["smooth_mode"], ["bool"]), _)]),
	#[inline]
	fn set_smooth_mode_flag(&mut self, smooth_mode: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setSmoothModeFlag_bool(self.as_raw_mut_RegionGrowing3D(), smooth_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set threshold value of the angle between normals, the input value is in radian.
	// setSmoothnessThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:361
	// ("cv::RegionGrowing3D::setSmoothnessThreshold", vec![(pred!(mut, ["smoothness_thr"], ["double"]), _)]),
	#[inline]
	fn set_smoothness_threshold(&mut self, smoothness_thr: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setSmoothnessThreshold_double(self.as_raw_mut_RegionGrowing3D(), smoothness_thr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set threshold value of curvature. Default will be 0.05.
	/// Only points with curvature less than the threshold will be considered to belong to the same region.
	/// If the curvature of each point is not set, this option will not work.
	// setCurvatureThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:369
	// ("cv::RegionGrowing3D::setCurvatureThreshold", vec![(pred!(mut, ["curvature_thr"], ["double"]), _)]),
	#[inline]
	fn set_curvature_threshold(&mut self, curvature_thr: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setCurvatureThreshold_double(self.as_raw_mut_RegionGrowing3D(), curvature_thr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the maximum number of neighbors want to use including itself.
	/// Setting to a non-positive number or default will use the information from nn_idx.
	// setMaxNumberOfNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:376
	// ("cv::RegionGrowing3D::setMaxNumberOfNeighbors", vec![(pred!(mut, ["max_neighbor_num"], ["int"]), _)]),
	#[inline]
	fn set_max_number_of_neighbors(&mut self, max_neighbor_num: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setMaxNumberOfNeighbors_int(self.as_raw_mut_RegionGrowing3D(), max_neighbor_num, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the maximum number of regions you want.
	// setNumberOfRegions(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:383
	// ("cv::RegionGrowing3D::setNumberOfRegions", vec![(pred!(mut, ["region_num"], ["int"]), _)]),
	#[inline]
	fn set_number_of_regions(&mut self, region_num: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setNumberOfRegions_int(self.as_raw_mut_RegionGrowing3D(), region_num, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set whether the results need to be sorted in descending order by the number of points.
	// setNeedSort(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:389
	// ("cv::RegionGrowing3D::setNeedSort", vec![(pred!(mut, ["need_sort"], ["bool"]), _)]),
	#[inline]
	fn set_need_sort(&mut self, need_sort: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setNeedSort_bool(self.as_raw_mut_RegionGrowing3D(), need_sort, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the seed points, it will grow according to the seeds.
	/// If noArray() is set, the default method will be used:
	/// 1. If the curvature of each point is set, the seeds will be sorted in ascending order of curvatures.
	/// 2. Otherwise, the natural order of the point cloud will be used.
	// setSeeds(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:398
	// ("cv::RegionGrowing3D::setSeeds", vec![(pred!(mut, ["seeds"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_seeds(&mut self, seeds: &impl ToInputArray) -> Result<()> {
		input_array_arg!(seeds);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setSeeds_const__InputArrayR(self.as_raw_mut_RegionGrowing3D(), seeds.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the curvature of each point, support vector<float> and Mat. If not, you can set it to noArray().
	// setCurvatures(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:404
	// ("cv::RegionGrowing3D::setCurvatures", vec![(pred!(mut, ["curvatures"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_curvatures(&mut self, curvatures: &impl ToInputArray) -> Result<()> {
		input_array_arg!(curvatures);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_setCurvatures_const__InputArrayR(self.as_raw_mut_RegionGrowing3D(), curvatures.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Region Growing algorithm in 3D point cloud.
///
/// The key idea of region growing is to merge the nearest neighbor points that satisfy a certain
/// angle threshold into the same region according to the normal between the two points, so as to
/// achieve the purpose of segmentation. For more details, please refer to [Rabbani2006SegmentationOP](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Rabbani2006SegmentationOP).
// RegionGrowing3D /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:298
pub struct RegionGrowing3D {
	ptr: *mut c_void,
}

opencv_type_boxed! { RegionGrowing3D }

impl Drop for RegionGrowing3D {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_RegionGrowing3D_delete(self.as_raw_mut_RegionGrowing3D()) };
	}
}

unsafe impl Send for RegionGrowing3D {}

impl crate::mod_3d::RegionGrowing3DTraitConst for RegionGrowing3D {
	#[inline] fn as_raw_RegionGrowing3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::RegionGrowing3DTrait for RegionGrowing3D {
	#[inline] fn as_raw_mut_RegionGrowing3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RegionGrowing3D, crate::mod_3d::RegionGrowing3DTraitConst, as_raw_RegionGrowing3D, crate::mod_3d::RegionGrowing3DTrait, as_raw_mut_RegionGrowing3D }

impl RegionGrowing3D {
	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:303
	// ("cv::RegionGrowing3D::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::mod_3d::RegionGrowing3D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RegionGrowing3D_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::RegionGrowing3D>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for RegionGrowing3D {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RegionGrowing3D")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::RgbdNormals]
// RgbdNormals /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:26
pub trait RgbdNormalsTraitConst {
	fn as_raw_RgbdNormals(&self) -> *const c_void;

	/// Given a set of 3d points in a depth image, compute the normals at each point.
	/// ## Parameters
	/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
	/// * normals: a rows x cols x 3 matrix
	// apply(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:57
	// ("cv::RgbdNormals::apply", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_apply_const_const__InputArrayR_const__OutputArrayR(self.as_raw_RgbdNormals(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Prepares cached data required for calculation
	/// If not called by user, called automatically at first calculation
	// cache()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:62
	// ("cv::RgbdNormals::cache", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cache(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_cache_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getRows()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:64
	// ("cv::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_getRows_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCols()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:66
	// ("cv::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_getCols_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:68
	// ("cv::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:70
	// ("cv::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getK(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:71
	// ("cv::RgbdNormals::getK", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_k(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_getK_const_const__OutputArrayR(self.as_raw_RgbdNormals(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:73
	// ("cv::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_method(&self) -> Result<crate::mod_3d::RgbdNormals_RgbdNormalsMethod> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_getMethod_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::RgbdNormals]
pub trait RgbdNormalsTrait: crate::mod_3d::RgbdNormalsTraitConst {
	fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void;

	// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:65
	// ("cv::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_rows(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:67
	// ("cv::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_cols(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:69
	// ("cv::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setK(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:72
	// ("cv::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_k(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_setK_const__InputArrayR(self.as_raw_mut_RgbdNormals(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Object that can compute the normals in an image.
/// It is an object as it can cache data for speed efficiency
/// The implemented methods are either:
/// - FALS (the fastest) and SRI from
/// ``Fast and Accurate Computation of Surface Normals from Range Images``
/// by H. Badino, D. Huber, Y. Park and T. Kanade
/// - the normals with bilateral filtering on a depth image from
/// ``Gradient Response Maps for Real-Time Detection of Texture-Less Objects``
/// by S. Hinterstoisser, C. Cagniart, S. Ilic, P. Sturm, N. Navab, P. Fua, and V. Lepetit
// RgbdNormals /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:26
pub struct RgbdNormals {
	ptr: *mut c_void,
}

opencv_type_boxed! { RgbdNormals }

impl Drop for RgbdNormals {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_RgbdNormals_delete(self.as_raw_mut_RgbdNormals()) };
	}
}

unsafe impl Send for RgbdNormals {}

impl crate::mod_3d::RgbdNormalsTraitConst for RgbdNormals {
	#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::RgbdNormalsTrait for RgbdNormals {
	#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdNormals, crate::mod_3d::RgbdNormalsTraitConst, as_raw_RgbdNormals, crate::mod_3d::RgbdNormalsTrait, as_raw_mut_RgbdNormals }

impl RgbdNormals {
	/// Creates new RgbdNormals object
	/// ## Parameters
	/// * rows: the number of rows of the depth image normals will be computed on
	/// * cols: the number of cols of the depth image normals will be computed on
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * K: the calibration matrix to use
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * diff_threshold: threshold in depth difference, used in LINEMOD algirithm
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	///
	/// ## C++ default parameters
	/// * rows: 0
	/// * cols: 0
	/// * depth: 0
	/// * k: noArray()
	/// * window_size: 5
	/// * diff_threshold: 50.f
	/// * method: RgbdNormals::RgbdNormalsMethod::RGBD_NORMALS_METHOD_FALS
	// create(int, int, int, InputArray, int, float, RgbdNormals::RgbdNormalsMethod)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:49
	// ("cv::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "diff_threshold", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "float", "cv::RgbdNormals::RgbdNormalsMethod"]), _)]),
	#[inline]
	pub fn create(rows: i32, cols: i32, depth: i32, k: &impl ToInputArray, window_size: i32, diff_threshold: f32, method: crate::mod_3d::RgbdNormals_RgbdNormalsMethod) -> Result<core::Ptr<crate::mod_3d::RgbdNormals>> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_create_int_int_int_const__InputArrayR_int_float_RgbdNormalsMethod(rows, cols, depth, k.as_raw__InputArray(), window_size, diff_threshold, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates new RgbdNormals object
	/// ## Parameters
	/// * rows: the number of rows of the depth image normals will be computed on
	/// * cols: the number of cols of the depth image normals will be computed on
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * K: the calibration matrix to use
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * diff_threshold: threshold in depth difference, used in LINEMOD algirithm
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	///
	/// ## Note
	/// This alternative version of [RgbdNormals::create] function uses the following default values for its arguments:
	/// * rows: 0
	/// * cols: 0
	/// * depth: 0
	/// * k: noArray()
	/// * window_size: 5
	/// * diff_threshold: 50.f
	/// * method: RgbdNormals::RgbdNormalsMethod::RGBD_NORMALS_METHOD_FALS
	// cv::RgbdNormals::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/depth.hpp:49
	// ("cv::RgbdNormals::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::mod_3d::RgbdNormals>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RgbdNormals_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for RgbdNormals {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RgbdNormals")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::SACSegmentation]
// SACSegmentation /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:56
pub trait SACSegmentationTraitConst {
	fn as_raw_SACSegmentation(&self) -> *const c_void;

	/// Get the type of sample consensus model used.
	// getSacModelType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:106
	// ("cv::SACSegmentation::getSacModelType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sac_model_type(&self) -> Result<crate::mod_3d::SacModelType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getSacModelType_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the type of sample consensus method used.
	// getSacMethodType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:112
	// ("cv::SACSegmentation::getSacMethodType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sac_method_type(&self) -> Result<crate::mod_3d::SacMethod> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getSacMethodType_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the distance to the model threshold.
	// getDistanceThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:119
	// ("cv::SACSegmentation::getDistanceThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_distance_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getDistanceThreshold_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the minimum and maximum radius limits for the model.
	// getRadiusLimits(double &, double &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:126
	// ("cv::SACSegmentation::getRadiusLimits", vec![(pred!(const, ["radius_min", "radius_max"], ["double*", "double*"]), _)]),
	#[inline]
	fn get_radius_limits(&self, radius_min: &mut f64, radius_max: &mut f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getRadiusLimits_const_doubleR_doubleR(self.as_raw_SACSegmentation(), radius_min, radius_max, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the maximum number of iterations to attempt.
	// getMaxIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:132
	// ("cv::SACSegmentation::getMaxIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getMaxIterations_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the confidence that ensure at least one of selections is an error-free set of data points.
	// getConfidence()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:138
	// ("cv::SACSegmentation::getConfidence", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_confidence(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getConfidence_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the expected number of models.
	// getNumberOfModelsExpected()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:144
	// ("cv::SACSegmentation::getNumberOfModelsExpected", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_number_of_models_expected(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getNumberOfModelsExpected_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get whether to use parallelism or not.
	// isParallel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:151
	// ("cv::SACSegmentation::isParallel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_parallel(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_isParallel_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get state used to initialize the RNG(Random Number Generator).
	// getRandomGeneratorState()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:157
	// ("cv::SACSegmentation::getRandomGeneratorState", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_random_generator_state(&self) -> Result<u64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_getRandomGeneratorState_const(self.as_raw_SACSegmentation(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::SACSegmentation]
pub trait SACSegmentationTrait: crate::mod_3d::SACSegmentationTraitConst {
	fn as_raw_mut_SACSegmentation(&mut self) -> *mut c_void;

	/// Execute segmentation using the sample consensus method.
	///
	/// ## Parameters
	/// * input_pts: Original point cloud, vector of Point3 or Mat of size Nx3/3xN.
	/// * labels:[out] The label corresponds to the model number, 0 means it
	/// does not belong to any model, range [0, Number of final resultant models obtained].
	/// * models_coefficients:[out] The resultant models coefficients.
	/// Currently supports passing in cv::Mat. Models coefficients are placed in a matrix of NxK
	/// with depth CV_64F (will automatically adjust if the passing one does not look like this),
	/// where N is the number of models and K is the number of coefficients of one model.
	/// The coefficients for each model refer to the comments inside enumeration type SacModelType.
	/// ## Returns
	/// Number of final resultant models obtained by segmentation.
	// segment(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:98
	// ("cv::SACSegmentation::segment", vec![(pred!(mut, ["input_pts", "labels", "models_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn segment(&mut self, input_pts: &impl ToInputArray, labels: &mut impl ToOutputArray, models_coefficients: &mut impl ToOutputArray) -> Result<i32> {
		input_array_arg!(input_pts);
		output_array_arg!(labels);
		output_array_arg!(models_coefficients);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_segment_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SACSegmentation(), input_pts.as_raw__InputArray(), labels.as_raw__OutputArray(), models_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the type of sample consensus model to use.
	// setSacModelType(SacModelType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:103
	// ("cv::SACSegmentation::setSacModelType", vec![(pred!(mut, ["sac_model_type"], ["cv::SacModelType"]), _)]),
	#[inline]
	fn set_sac_model_type(&mut self, sac_model_type: crate::mod_3d::SacModelType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setSacModelType_SacModelType(self.as_raw_mut_SACSegmentation(), sac_model_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the type of sample consensus method to use.
	// setSacMethodType(SacMethod)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:109
	// ("cv::SACSegmentation::setSacMethodType", vec![(pred!(mut, ["sac_method"], ["cv::SacMethod"]), _)]),
	#[inline]
	fn set_sac_method_type(&mut self, sac_method: crate::mod_3d::SacMethod) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setSacMethodType_SacMethod(self.as_raw_mut_SACSegmentation(), sac_method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the distance to the model threshold.
	/// Considered as inlier point if distance to the model less than threshold.
	// setDistanceThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:116
	// ("cv::SACSegmentation::setDistanceThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_distance_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setDistanceThreshold_double(self.as_raw_mut_SACSegmentation(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the minimum and maximum radius limits for the model.
	/// Only used for models whose model parameters include a radius.
	// setRadiusLimits(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:123
	// ("cv::SACSegmentation::setRadiusLimits", vec![(pred!(mut, ["radius_min", "radius_max"], ["double", "double"]), _)]),
	#[inline]
	fn set_radius_limits(&mut self, radius_min: f64, radius_max: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setRadiusLimits_double_double(self.as_raw_mut_SACSegmentation(), radius_min, radius_max, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the maximum number of iterations to attempt.
	// setMaxIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:129
	// ("cv::SACSegmentation::setMaxIterations", vec![(pred!(mut, ["max_iterations"], ["int"]), _)]),
	#[inline]
	fn set_max_iterations(&mut self, max_iterations: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setMaxIterations_int(self.as_raw_mut_SACSegmentation(), max_iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the confidence that ensure at least one of selections is an error-free set of data points.
	// setConfidence(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:135
	// ("cv::SACSegmentation::setConfidence", vec![(pred!(mut, ["confidence"], ["double"]), _)]),
	#[inline]
	fn set_confidence(&mut self, confidence: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setConfidence_double(self.as_raw_mut_SACSegmentation(), confidence, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the number of models expected.
	// setNumberOfModelsExpected(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:141
	// ("cv::SACSegmentation::setNumberOfModelsExpected", vec![(pred!(mut, ["number_of_models_expected"], ["int"]), _)]),
	#[inline]
	fn set_number_of_models_expected(&mut self, number_of_models_expected: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setNumberOfModelsExpected_int(self.as_raw_mut_SACSegmentation(), number_of_models_expected, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set whether to use parallelism or not.
	/// The number of threads is set by cv::setNumThreads(int nthreads).
	// setParallel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:148
	// ("cv::SACSegmentation::setParallel", vec![(pred!(mut, ["is_parallel"], ["bool"]), _)]),
	#[inline]
	fn set_parallel(&mut self, is_parallel: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setParallel_bool(self.as_raw_mut_SACSegmentation(), is_parallel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set state used to initialize the RNG(Random Number Generator).
	// setRandomGeneratorState(uint64)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:154
	// ("cv::SACSegmentation::setRandomGeneratorState", vec![(pred!(mut, ["rng_state"], ["uint64_t"]), _)]),
	#[inline]
	fn set_random_generator_state(&mut self, rng_state: u64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_setRandomGeneratorState_uint64_t(self.as_raw_mut_SACSegmentation(), rng_state, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Sample Consensus algorithm segmentation of 3D point cloud model.
///
/// Example of segmenting plane from a 3D point cloud using the RANSAC algorithm:
/// [planeSegmentationUsingRANSAC](https://github.com/opencv/opencv/blob/5.0.0/samples/cpp/tutorial_code/snippets/3d_sac_segmentation.cpp#L1)
/// ## See also
/// 1. Supported algorithms: enum SacMethod in ptcloud.hpp.
/// 2. Supported models: enum SacModelType in ptcloud.hpp.
// SACSegmentation /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:56
pub struct SACSegmentation {
	ptr: *mut c_void,
}

opencv_type_boxed! { SACSegmentation }

impl Drop for SACSegmentation {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_SACSegmentation_delete(self.as_raw_mut_SACSegmentation()) };
	}
}

unsafe impl Send for SACSegmentation {}

impl crate::mod_3d::SACSegmentationTraitConst for SACSegmentation {
	#[inline] fn as_raw_SACSegmentation(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::SACSegmentationTrait for SACSegmentation {
	#[inline] fn as_raw_mut_SACSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SACSegmentation, crate::mod_3d::SACSegmentationTraitConst, as_raw_SACSegmentation, crate::mod_3d::SACSegmentationTrait, as_raw_mut_SACSegmentation }

impl SACSegmentation {
	/// ## C++ default parameters
	/// * sac_model_type: SAC_MODEL_PLANE
	/// * sac_method: SAC_METHOD_RANSAC
	/// * threshold: 0.5
	/// * max_iterations: 1000
	// create(SacModelType, SacMethod, double, int)(Enum, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:72
	// ("cv::SACSegmentation::create", vec![(pred!(mut, ["sac_model_type", "sac_method", "threshold", "max_iterations"], ["cv::SacModelType", "cv::SacMethod", "double", "int"]), _)]),
	#[inline]
	pub fn create(sac_model_type: crate::mod_3d::SacModelType, sac_method: crate::mod_3d::SacMethod, threshold: f64, max_iterations: i32) -> Result<core::Ptr<crate::mod_3d::SACSegmentation>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_create_SacModelType_SacMethod_double_int(sac_model_type, sac_method, threshold, max_iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::SACSegmentation>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SACSegmentation::create] function uses the following default values for its arguments:
	/// * sac_model_type: SAC_MODEL_PLANE
	/// * sac_method: SAC_METHOD_RANSAC
	/// * threshold: 0.5
	/// * max_iterations: 1000
	// cv::SACSegmentation::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/ptcloud.hpp:72
	// ("cv::SACSegmentation::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::mod_3d::SACSegmentation>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SACSegmentation_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::mod_3d::SACSegmentation>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for SACSegmentation {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SACSegmentation")
			.finish()
	}
}

/// Structure to keep settings for rasterization
// TriangleRasterizeSettings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2943
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TriangleRasterizeSettings {
	pub shading_type: crate::mod_3d::TriangleShadingType,
	pub culling_mode: crate::mod_3d::TriangleCullingMode,
	pub gl_compatible_mode: crate::mod_3d::TriangleGlCompatibleMode,
}

opencv_type_simple! { crate::mod_3d::TriangleRasterizeSettings }

impl TriangleRasterizeSettings {
	// TriangleRasterizeSettings()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2945
	// ("cv::TriangleRasterizeSettings::TriangleRasterizeSettings", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mod_3d::TriangleRasterizeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TriangleRasterizeSettings_TriangleRasterizeSettings(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setShadingType(TriangleShadingType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2947
	// ("cv::TriangleRasterizeSettings::setShadingType", vec![(pred!(mut, ["st"], ["cv::TriangleShadingType"]), _)]),
	#[inline]
	pub fn set_shading_type(self, st: crate::mod_3d::TriangleShadingType) -> Result<crate::mod_3d::TriangleRasterizeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TriangleRasterizeSettings_setShadingType_TriangleShadingType(&self, st, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCullingMode(TriangleCullingMode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2948
	// ("cv::TriangleRasterizeSettings::setCullingMode", vec![(pred!(mut, ["cm"], ["cv::TriangleCullingMode"]), _)]),
	#[inline]
	pub fn set_culling_mode(self, cm: crate::mod_3d::TriangleCullingMode) -> Result<crate::mod_3d::TriangleRasterizeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TriangleRasterizeSettings_setCullingMode_TriangleCullingMode(&self, cm, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setGlCompatibleMode(TriangleGlCompatibleMode)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:2949
	// ("cv::TriangleRasterizeSettings::setGlCompatibleMode", vec![(pred!(mut, ["gm"], ["cv::TriangleGlCompatibleMode"]), _)]),
	#[inline]
	pub fn set_gl_compatible_mode(self, gm: crate::mod_3d::TriangleGlCompatibleMode) -> Result<crate::mod_3d::TriangleRasterizeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TriangleRasterizeSettings_setGlCompatibleMode_TriangleGlCompatibleMode(&self, gm, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// UsacParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:430
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UsacParams {
	pub confidence: f64,
	pub is_parallel: bool,
	pub lo_iterations: i32,
	pub lo_method: crate::mod_3d::LocalOptimMethod,
	pub lo_sample_size: i32,
	pub max_iterations: i32,
	pub neighbors_search: crate::mod_3d::NeighborSearchMethod,
	pub random_generator_state: i32,
	pub sampler: crate::mod_3d::SamplingMethod,
	pub score: crate::mod_3d::ScoreMethod,
	pub threshold: f64,
	pub final_polisher: crate::mod_3d::PolishingMethod,
	pub final_polisher_iterations: i32,
}

opencv_type_simple! { crate::mod_3d::UsacParams }

impl UsacParams {
	// UsacParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d.hpp:432
	// ("cv::UsacParams::UsacParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::mod_3d::UsacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UsacParams_UsacParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::mod_3d::Volume]
// Volume /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:15
pub trait VolumeTraitConst {
	fn as_raw_Volume(&self) -> *const c_void;

	/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
	///
	/// Rendered image size and camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * cameraPose: the pose of camera in global coordinates.
	/// * points: image to store rendered points.
	/// * normals: image to store rendered normals corresponding to points.
	// raycast(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:68
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn raycast(&self, camera_pose: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(camera_pose);
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
	///
	/// Rendered image size and camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * cameraPose: the pose of camera in global coordinates.
	/// * points: image to store rendered points.
	/// * normals: image to store rendered normals corresponding to points.
	/// * colors: image to store rendered colors corresponding to points (only for ColorTSDF).
	// raycast(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:79
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "points", "normals", "colors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn raycast_color(&self, camera_pose: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(camera_pose);
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
	///
	/// Rendered image size and camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * cameraPose: the pose of camera in global coordinates.
	/// * height: the height of result image
	/// * width: the width of result image
	/// * K: camera raycast intrinsics
	/// * points: image to store rendered points.
	/// * normals: image to store rendered normals corresponding to points.
	// raycast(InputArray, int, int, InputArray, OutputArray, OutputArray)(InputArray, Primitive, Primitive, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:92
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "height", "width", "K", "points", "normals"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn raycast_ex(&self, camera_pose: &impl ToInputArray, height: i32, width: i32, k: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(camera_pose);
		input_array_arg!(k);
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), height, width, k.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
	///
	/// Rendered image size and camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * cameraPose: the pose of camera in global coordinates.
	/// * height: the height of result image
	/// * width: the width of result image
	/// * K: camera raycast intrinsics
	/// * points: image to store rendered points.
	/// * normals: image to store rendered normals corresponding to points.
	/// * colors: image to store rendered colors corresponding to points (only for ColorTSDF).
	// raycast(InputArray, int, int, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, Primitive, Primitive, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:107
	// ("cv::Volume::raycast", vec![(pred!(const, ["cameraPose", "height", "width", "K", "points", "normals", "colors"], ["const cv::_InputArray*", "int", "int", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn raycast_ex_color(&self, camera_pose: &impl ToInputArray, height: i32, width: i32, k: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(camera_pose);
		input_array_arg!(k);
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), height, width, k.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Extract the all data from volume.
	/// ## Parameters
	/// * points: the input exist point.
	/// * normals: the storage of normals (corresponding to input points) in the image.
	// fetchNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:113
	// ("cv::Volume::fetchNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn fetch_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Volume(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Extract the all data from volume.
	/// ## Parameters
	/// * points: the storage of all points.
	/// * normals: the storage of all normals, corresponding to points.
	// fetchPointsNormals(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:118
	// ("cv::Volume::fetchPointsNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn fetch_points_normals(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Extract the all data from volume.
	/// ## Parameters
	/// * points: the storage of all points.
	/// * normals: the storage of all normals, corresponding to points.
	/// * colors: the storage of all colors, corresponding to points (only for ColorTSDF).
	// fetchPointsNormalsColors(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:124
	// ("cv::Volume::fetchPointsNormalsColors", vec![(pred!(const, ["points", "normals", "colors"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn fetch_points_normals_colors(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// return visible blocks in volume.
	// getVisibleBlocks()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:133
	// ("cv::Volume::getVisibleBlocks", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_visible_blocks(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_getVisibleBlocks_const(self.as_raw_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// return number of volume units in volume.
	// getTotalVolumeUnits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:137
	// ("cv::Volume::getTotalVolumeUnits", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_total_volume_units(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_getTotalVolumeUnits_const(self.as_raw_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets bounding box in volume coordinates with given precision:
	/// VOLUME_UNIT - up to volume unit
	/// VOXEL - up to voxel (currently not supported)
	/// ## Parameters
	/// * bb: 6-float 1d array containing (min_x, min_y, min_z, max_x, max_y, max_z) in volume coordinates
	/// * precision: bounding box calculation precision
	// getBoundingBox(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:152
	// ("cv::Volume::getBoundingBox", vec![(pred!(const, ["bb", "precision"], ["const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn get_bounding_box(&self, bb: &mut impl ToOutputArray, precision: i32) -> Result<()> {
		output_array_arg!(bb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_getBoundingBox_const_const__OutputArrayR_int(self.as_raw_Volume(), bb.as_raw__OutputArray(), precision, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns if new volume units are allocated during integration or not.
	/// Makes sense for HashTSDF only.
	// getEnableGrowth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:163
	// ("cv::Volume::getEnableGrowth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_enable_growth(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_getEnableGrowth_const(self.as_raw_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::Volume]
pub trait VolumeTrait: crate::mod_3d::VolumeTraitConst {
	fn as_raw_mut_Volume(&mut self) -> *mut c_void;

	/// Integrates the input data to the volume.
	///
	/// Camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * frame: the object from which to take depth and image data.
	///   For color TSDF a depth data should be registered with color data, i.e. have the same intrinsics & camera pose.
	///   This can be done using function registerDepth() from 3d module.
	/// * pose: the pose of camera in global coordinates.
	// integrate(const OdometryFrame &, InputArray)(TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:35
	// ("cv::Volume::integrate", vec![(pred!(mut, ["frame", "pose"], ["const cv::OdometryFrame*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn integrate_frame(&mut self, frame: &impl crate::mod_3d::OdometryFrameTraitConst, pose: &impl ToInputArray) -> Result<()> {
		input_array_arg!(pose);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_integrate_const_OdometryFrameR_const__InputArrayR(self.as_raw_mut_Volume(), frame.as_raw_OdometryFrame(), pose.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Integrates the input data to the volume.
	///
	/// Camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * depth: the depth image.
	/// * pose: the pose of camera in global coordinates.
	// integrate(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:44
	// ("cv::Volume::integrate", vec![(pred!(mut, ["depth", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn integrate(&mut self, depth: &impl ToInputArray, pose: &impl ToInputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(pose);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_integrate_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Volume(), depth.as_raw__InputArray(), pose.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Integrates the input data to the volume.
	///
	/// Camera intrinsics are taken from volume settings structure.
	///
	/// ## Parameters
	/// * depth: the depth image.
	/// * image: the color image (only for ColorTSDF).
	///   For color TSDF a depth data should be registered with color data, i.e. have the same intrinsics & camera pose.
	///   This can be done using function registerDepth() from 3d module.
	/// * pose: the pose of camera in global coordinates.
	// integrate(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:56
	// ("cv::Volume::integrate", vec![(pred!(mut, ["depth", "image", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn integrate_color(&mut self, depth: &impl ToInputArray, image: &impl ToInputArray, pose: &impl ToInputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(image);
		input_array_arg!(pose);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_integrate_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Volume(), depth.as_raw__InputArray(), image.as_raw__InputArray(), pose.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// clear all data in volume.
	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:128
	// ("cv::Volume::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_reset(self.as_raw_mut_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Enables or disables new volume unit allocation during integration.
	/// Makes sense for HashTSDF only.
	// setEnableGrowth(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:158
	// ("cv::Volume::setEnableGrowth", vec![(pred!(mut, ["v"], ["bool"]), _)]),
	#[inline]
	fn set_enable_growth(&mut self, v: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_setEnableGrowth_bool(self.as_raw_mut_Volume(), v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Volume /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:15
pub struct Volume {
	ptr: *mut c_void,
}

opencv_type_boxed! { Volume }

impl Drop for Volume {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Volume_delete(self.as_raw_mut_Volume()) };
	}
}

unsafe impl Send for Volume {}

impl crate::mod_3d::VolumeTraitConst for Volume {
	#[inline] fn as_raw_Volume(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::VolumeTrait for Volume {
	#[inline] fn as_raw_mut_Volume(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Volume, crate::mod_3d::VolumeTraitConst, as_raw_Volume, crate::mod_3d::VolumeTrait, as_raw_mut_Volume }

impl Volume {
	/// Constructor of custom volume.
	/// ## Parameters
	/// * vtype: the volume type [TSDF, HashTSDF, ColorTSDF].
	/// * settings: the custom settings for volume.
	///
	/// ## C++ default parameters
	/// * vtype: VolumeType::TSDF
	/// * settings: VolumeSettings(VolumeType::TSDF)
	// Volume(VolumeType, const VolumeSettings &)(Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:22
	// ("cv::Volume::Volume", vec![(pred!(mut, ["vtype", "settings"], ["cv::VolumeType", "const cv::VolumeSettings*"]), _)]),
	#[inline]
	pub fn new(vtype: crate::mod_3d::VolumeType, settings: &impl crate::mod_3d::VolumeSettingsTraitConst) -> Result<crate::mod_3d::Volume> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_Volume_VolumeType_const_VolumeSettingsR(vtype, settings.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::Volume::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor of custom volume.
	/// ## Parameters
	/// * vtype: the volume type [TSDF, HashTSDF, ColorTSDF].
	/// * settings: the custom settings for volume.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * vtype: VolumeType::TSDF
	/// * settings: VolumeSettings(VolumeType::TSDF)
	// cv::Volume::Volume() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume.hpp:22
	// ("cv::Volume::Volume", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::mod_3d::Volume> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Volume_Volume(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::Volume::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Volume {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Volume")
			.finish()
	}
}

/// Constant methods for [crate::mod_3d::VolumeSettings]
// VolumeSettings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:23
pub trait VolumeSettingsTraitConst {
	fn as_raw_VolumeSettings(&self) -> *const c_void;

	/// Returns the width of the image for integration.
	// getIntegrateWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:42
	// ("cv::VolumeSettings::getIntegrateWidth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_integrate_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getIntegrateWidth_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the height of the image for integration.
	// getIntegrateHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:51
	// ("cv::VolumeSettings::getIntegrateHeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_integrate_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getIntegrateHeight_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the width of the raycasted image, used when user does not provide it at raycast() call.
	// getRaycastWidth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:60
	// ("cv::VolumeSettings::getRaycastWidth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_raycast_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getRaycastWidth_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the height of the raycasted image, used when user does not provide it at raycast() call.
	// getRaycastHeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:69
	// ("cv::VolumeSettings::getRaycastHeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_raycast_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getRaycastHeight_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns depth factor, witch is the number for depth scaling.
	// getDepthFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:78
	// ("cv::VolumeSettings::getDepthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_depth_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getDepthFactor_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the size of voxel.
	// getVoxelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:87
	// ("cv::VolumeSettings::getVoxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_voxel_size(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getVoxelSize_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns TSDF truncation distance. Distances greater than value from surface will be truncated to 1.0.
	// getTsdfTruncateDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:96
	// ("cv::VolumeSettings::getTsdfTruncateDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tsdf_truncate_distance(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getTsdfTruncateDistance_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns threshold for depth truncation in meters. Truncates the depth greater than threshold to 0.
	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:105
	// ("cv::VolumeSettings::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getMaxDepth_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns max number of frames to integrate per voxel.
	/// Represents the max number of frames over which a running average of the TSDF is calculated for a voxel.
	// getMaxWeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:116
	// ("cv::VolumeSettings::getMaxWeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_weight(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getMaxWeight_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns length of single raycast step.
	/// Describes the percentage of voxel length that is skipped per march.
	// getRaycastStepFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:127
	// ("cv::VolumeSettings::getRaycastStepFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_raycast_step_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getRaycastStepFactor_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets volume pose.
	/// ## Parameters
	/// * val: output value.
	// getVolumePose(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:137
	// ("cv::VolumeSettings::getVolumePose", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_volume_pose(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getVolumePose_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Resolution of voxel space.
	///    Number of voxels in each dimension.
	///    Applicable only for TSDF Volume.
	///    HashTSDF volume only supports equal resolution in all three dimensions.
	/// ## Parameters
	/// * val: output value.
	// getVolumeResolution(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:153
	// ("cv::VolumeSettings::getVolumeResolution", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_volume_resolution(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getVolumeResolution_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns 3 integers representing strides by x, y and z dimension.
	///    Can be used to iterate over raw volume unit data.
	/// ## Parameters
	/// * val: output value.
	// getVolumeStrides(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:159
	// ("cv::VolumeSettings::getVolumeStrides", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_volume_strides(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getVolumeStrides_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns intrinsics of camera for integrations.
	/// Format of output:
	/// [ fx  0 cx ]
	/// [  0 fy cy ]
	/// [  0  0  1 ]
	/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
	/// ## Parameters
	/// * val: output value.
	// getCameraIntegrateIntrinsics(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:179
	// ("cv::VolumeSettings::getCameraIntegrateIntrinsics", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_camera_integrate_intrinsics(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getCameraIntegrateIntrinsics_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns camera intrinsics for raycast image, used when user does not provide them at raycast() call.
	/// Format of output:
	/// [ fx  0 cx ]
	/// [  0 fy cy ]
	/// [  0  0  1 ]
	/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
	/// ## Parameters
	/// * val: output value.
	// getCameraRaycastIntrinsics(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:199
	// ("cv::VolumeSettings::getCameraRaycastIntrinsics", vec![(pred!(const, ["val"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_camera_raycast_intrinsics(&self, val: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_getCameraRaycastIntrinsics_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::mod_3d::VolumeSettings]
pub trait VolumeSettingsTrait: crate::mod_3d::VolumeSettingsTraitConst {
	fn as_raw_mut_VolumeSettings(&mut self) -> *mut c_void;

	// operator=(const VolumeSettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:32
	// ("cv::VolumeSettings::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::VolumeSettings*"]), _)]),
	#[inline]
	fn set(&mut self, unnamed: &impl crate::mod_3d::VolumeSettingsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_operatorST_const_VolumeSettingsR(self.as_raw_mut_VolumeSettings(), unnamed.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the width of the image for integration.
	/// ## Parameters
	/// * val: input value.
	// setIntegrateWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:38
	// ("cv::VolumeSettings::setIntegrateWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_integrate_width(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setIntegrateWidth_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the height of the image for integration.
	/// ## Parameters
	/// * val: input value.
	// setIntegrateHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:47
	// ("cv::VolumeSettings::setIntegrateHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_integrate_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setIntegrateHeight_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the width of the raycasted image, used when user does not provide it at raycast() call.
	/// ## Parameters
	/// * val: input value.
	// setRaycastWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:56
	// ("cv::VolumeSettings::setRaycastWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_raycast_width(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setRaycastWidth_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the height of the raycasted image, used when user does not provide it at raycast() call.
	/// ## Parameters
	/// * val: input value.
	// setRaycastHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:65
	// ("cv::VolumeSettings::setRaycastHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_raycast_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setRaycastHeight_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets depth factor, witch is the number for depth scaling.
	/// ## Parameters
	/// * val: input value.
	// setDepthFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:74
	// ("cv::VolumeSettings::setDepthFactor", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setDepthFactor_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the size of voxel.
	/// ## Parameters
	/// * val: input value.
	// setVoxelSize(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:83
	// ("cv::VolumeSettings::setVoxelSize", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_voxel_size(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setVoxelSize_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets TSDF truncation distance. Distances greater than value from surface will be truncated to 1.0.
	/// ## Parameters
	/// * val: input value.
	// setTsdfTruncateDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:92
	// ("cv::VolumeSettings::setTsdfTruncateDistance", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_tsdf_truncate_distance(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setTsdfTruncateDistance_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets threshold for depth truncation in meters. Truncates the depth greater than threshold to 0.
	/// ## Parameters
	/// * val: input value.
	// setMaxDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:101
	// ("cv::VolumeSettings::setMaxDepth", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setMaxDepth_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets max number of frames to integrate per voxel.
	///    Represents the max number of frames over which a running average of the TSDF is calculated for a voxel.
	/// ## Parameters
	/// * val: input value.
	// setMaxWeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:111
	// ("cv::VolumeSettings::setMaxWeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_max_weight(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setMaxWeight_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets length of single raycast step.
	///    Describes the percentage of voxel length that is skipped per march.
	/// ## Parameters
	/// * val: input value.
	// setRaycastStepFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:122
	// ("cv::VolumeSettings::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setRaycastStepFactor_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets volume pose.
	/// ## Parameters
	/// * val: input value.
	// setVolumePose(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:132
	// ("cv::VolumeSettings::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_volume_pose(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setVolumePose_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Resolution of voxel space.
	///    Number of voxels in each dimension.
	///    Applicable only for TSDF Volume.
	///    HashTSDF volume only supports equal resolution in all three dimensions.
	/// ## Parameters
	/// * val: input value.
	// setVolumeResolution(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:145
	// ("cv::VolumeSettings::setVolumeResolution", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_volume_resolution(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setVolumeResolution_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets intrinsics of camera for integrations.
	/// Format of input:
	/// [ fx  0 cx ]
	/// [  0 fy cy ]
	/// [  0  0  1 ]
	/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
	/// ## Parameters
	/// * val: input value.
	// setCameraIntegrateIntrinsics(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:169
	// ("cv::VolumeSettings::setCameraIntegrateIntrinsics", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_camera_integrate_intrinsics(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setCameraIntegrateIntrinsics_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets camera intrinsics for raycast image which, used when user does not provide them at raycast() call.
	/// Format of input:
	/// [ fx  0 cx ]
	/// [  0 fy cy ]
	/// [  0  0  1 ]
	/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
	/// ## Parameters
	/// * val: input value.
	// setCameraRaycastIntrinsics(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:189
	// ("cv::VolumeSettings::setCameraRaycastIntrinsics", vec![(pred!(mut, ["val"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_camera_raycast_intrinsics(&mut self, val: &impl ToInputArray) -> Result<()> {
		input_array_arg!(val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_setCameraRaycastIntrinsics_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// VolumeSettings /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:23
pub struct VolumeSettings {
	ptr: *mut c_void,
}

opencv_type_boxed! { VolumeSettings }

impl Drop for VolumeSettings {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_VolumeSettings_delete(self.as_raw_mut_VolumeSettings()) };
	}
}

unsafe impl Send for VolumeSettings {}

impl crate::mod_3d::VolumeSettingsTraitConst for VolumeSettings {
	#[inline] fn as_raw_VolumeSettings(&self) -> *const c_void { self.as_raw() }
}

impl crate::mod_3d::VolumeSettingsTrait for VolumeSettings {
	#[inline] fn as_raw_mut_VolumeSettings(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VolumeSettings, crate::mod_3d::VolumeSettingsTraitConst, as_raw_VolumeSettings, crate::mod_3d::VolumeSettingsTrait, as_raw_mut_VolumeSettings }

impl VolumeSettings {
	/// Constructor of settings for custom Volume type.
	/// ## Parameters
	/// * volumeType: volume type.
	///
	/// ## C++ default parameters
	/// * volume_type: VolumeType::TSDF
	// VolumeSettings(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:29
	// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
	#[inline]
	pub fn new(volume_type: crate::mod_3d::VolumeType) -> Result<crate::mod_3d::VolumeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_VolumeSettings_VolumeType(volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::VolumeSettings::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor of settings for custom Volume type.
	/// ## Parameters
	/// * volumeType: volume type.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * volume_type: VolumeType::TSDF
	// cv::VolumeSettings::VolumeSettings() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:29
	// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::mod_3d::VolumeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_VolumeSettings(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::VolumeSettings::opencv_from_extern(ret) };
		Ok(ret)
	}

	// VolumeSettings(const VolumeSettings &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/3d/volume_settings.hpp:31
	// ("cv::VolumeSettings::VolumeSettings", vec![(pred!(mut, ["vs"], ["const cv::VolumeSettings*"]), _)]),
	#[inline]
	pub fn copy(vs: &impl crate::mod_3d::VolumeSettingsTraitConst) -> Result<crate::mod_3d::VolumeSettings> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_VolumeSettings_VolumeSettings_const_VolumeSettingsR(vs.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::mod_3d::VolumeSettings::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for VolumeSettings {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_VolumeSettings_implicitClone_const(self.as_raw_VolumeSettings())) }
	}
}

impl std::fmt::Debug for VolumeSettings {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("VolumeSettings")
			.finish()
	}
}
