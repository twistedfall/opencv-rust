//! # Camera Calibration and 3D Reconstruction
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
//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?A) (notation used as in [Zhang2000](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zhang2000) and also generally notated
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
//! ![Pinhole camera model](https://docs.opencv.org/3.4.20/pinhole_camera_model.png)
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
//! ![](https://docs.opencv.org/3.4.20/distortion_examples.png)
//! ![](https://docs.opencv.org/3.4.20/distortion_examples2.png)
//!
//! In some cases, the image sensor may be tilted in order to focus an oblique plane in front of the
//! camera (Scheimpflug principle). This can be useful for particle image velocimetry (PIV) or
//! triangulation with a laser fan. The tilt causes a perspective distortion of ![inline formula](https://latex.codecogs.com/png.latex?x%27%27) and
//! ![inline formula](https://latex.codecogs.com/png.latex?y%27%27). This distortion can be modeled in the following way, see e.g. [Louhichi07](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Louhichi07).
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
//!    ![block formula](https://latex.codecogs.com/png.latex?x%20%3D%20Xc%5F1%20%5C%5C%20y%20%3D%20Xc%5F2%20%5C%5C%20z%20%3D%20Xc%5F3)
//!
//!    The pinhole projection coordinates of P is [a; b] where
//!
//!    ![block formula](https://latex.codecogs.com/png.latex?a%20%3D%20x%20%2F%20z%20%5C%20and%20%5C%20b%20%3D%20y%20%2F%20z%20%5C%5C%20r%5E2%20%3D%20a%5E2%20%2B%20b%5E2%20%5C%5C%20%5Ctheta%20%3D%20atan%28r%29)
//!
//!    Fisheye distortion:
//!
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Ctheta%5Fd%20%3D%20%5Ctheta%20%281%20%2B%20k%5F1%20%5Ctheta%5E2%20%2B%20k%5F2%20%5Ctheta%5E4%20%2B%20k%5F3%20%5Ctheta%5E6%20%2B%20k%5F4%20%5Ctheta%5E8%29)
//!
//!    The distorted point coordinates are [x'; y'] where
//!
//!    ![block formula](https://latex.codecogs.com/png.latex?x%27%20%3D%20%28%5Ctheta%5Fd%20%2F%20r%29%20a%20%5C%5C%20y%27%20%3D%20%28%5Ctheta%5Fd%20%2F%20r%29%20b%20)
//!
//!    Finally, conversion into pixel coordinates: The final pixel coordinates vector [u; v] where:
//!
//!    ![block formula](https://latex.codecogs.com/png.latex?u%20%3D%20f%5Fx%20%28x%27%20%2B%20%5Calpha%20y%27%29%20%2B%20c%5Fx%20%5C%5C%0A%20%20%20%20v%20%3D%20f%5Fy%20y%27%20%2B%20c%5Fy)
//!
//!    Summary:
//!    Generic camera model [Kannala2006](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Kannala2006) with perspective projection and without distortion correction
//!
//!    # C API
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{CirclesGridFinderParameters2Trait, CirclesGridFinderParameters2TraitConst, CirclesGridFinderParametersTrait, CirclesGridFinderParametersTraitConst, StereoBMTrait, StereoBMTraitConst, StereoMatcherTrait, StereoMatcherTraitConst, StereoSGBMTrait, StereoSGBMTraitConst};
}

// CALIB_CB_ADAPTIVE_THRESH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:478
pub const CALIB_CB_ADAPTIVE_THRESH: i32 = 1;
// CALIB_CB_ASYMMETRIC_GRID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:485
pub const CALIB_CB_ASYMMETRIC_GRID: i32 = 2;
// CALIB_CB_CLUSTERING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:486
pub const CALIB_CB_CLUSTERING: i32 = 4;
// CALIB_CB_FAST_CHECK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:481
pub const CALIB_CB_FAST_CHECK: i32 = 8;
// CALIB_CB_FILTER_QUADS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:480
pub const CALIB_CB_FILTER_QUADS: i32 = 4;
// CALIB_CB_NORMALIZE_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:479
pub const CALIB_CB_NORMALIZE_IMAGE: i32 = 2;
// CALIB_CB_SYMMETRIC_GRID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:484
pub const CALIB_CB_SYMMETRIC_GRID: i32 = 1;
// CALIB_FIX_ASPECT_RATIO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:490
pub const CALIB_FIX_ASPECT_RATIO: i32 = 2;
// CALIB_FIX_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:493
pub const CALIB_FIX_FOCAL_LENGTH: i32 = 16;
// CALIB_FIX_INTRINSIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:508
pub const CALIB_FIX_INTRINSIC: i32 = 256;
// CALIB_FIX_K1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:494
pub const CALIB_FIX_K1: i32 = 32;
// CALIB_FIX_K2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:495
pub const CALIB_FIX_K2: i32 = 64;
// CALIB_FIX_K3 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:496
pub const CALIB_FIX_K3: i32 = 128;
// CALIB_FIX_K4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:497
pub const CALIB_FIX_K4: i32 = 2048;
// CALIB_FIX_K5 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:498
pub const CALIB_FIX_K5: i32 = 4096;
// CALIB_FIX_K6 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:499
pub const CALIB_FIX_K6: i32 = 8192;
// CALIB_FIX_PRINCIPAL_POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:491
pub const CALIB_FIX_PRINCIPAL_POINT: i32 = 4;
// CALIB_FIX_S1_S2_S3_S4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:502
pub const CALIB_FIX_S1_S2_S3_S4: i32 = 65536;
// CALIB_FIX_TANGENT_DIST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:506
pub const CALIB_FIX_TANGENT_DIST: i32 = 2097152;
// CALIB_FIX_TAUX_TAUY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:504
pub const CALIB_FIX_TAUX_TAUY: i32 = 524288;
/// On-line Hand-Eye Calibration [Andreff99](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Andreff99)
// CALIB_HAND_EYE_ANDREFF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:528
pub const CALIB_HAND_EYE_ANDREFF: i32 = 3;
/// Hand-Eye Calibration Using Dual Quaternions [Daniilidis98](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Daniilidis98)
// CALIB_HAND_EYE_DANIILIDIS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:529
pub const CALIB_HAND_EYE_DANIILIDIS: i32 = 4;
/// Hand-eye Calibration [Horaud95](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Horaud95)
// CALIB_HAND_EYE_HORAUD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:527
pub const CALIB_HAND_EYE_HORAUD: i32 = 2;
/// Robot Sensor Calibration: Solving AX = XB on the Euclidean Group [Park94](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Park94)
// CALIB_HAND_EYE_PARK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:526
pub const CALIB_HAND_EYE_PARK: i32 = 1;
/// A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/Eye Calibration [Tsai89](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Tsai89)
// CALIB_HAND_EYE_TSAI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:525
pub const CALIB_HAND_EYE_TSAI: i32 = 0;
// CALIB_RATIONAL_MODEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:500
pub const CALIB_RATIONAL_MODEL: i32 = 16384;
// CALIB_SAME_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:509
pub const CALIB_SAME_FOCAL_LENGTH: i32 = 512;
// CALIB_THIN_PRISM_MODEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:501
pub const CALIB_THIN_PRISM_MODEL: i32 = 32768;
// CALIB_TILTED_MODEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:503
pub const CALIB_TILTED_MODEL: i32 = 262144;
/// for stereoCalibrate
// CALIB_USE_EXTRINSIC_GUESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:513
pub const CALIB_USE_EXTRINSIC_GUESS: i32 = 4194304;
// CALIB_USE_INTRINSIC_GUESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:489
pub const CALIB_USE_INTRINSIC_GUESS: i32 = 1;
/// use LU instead of SVD decomposition for solving. much faster but potentially less precise
// CALIB_USE_LU /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:512
pub const CALIB_USE_LU: i32 = 131072;
/// use QR instead of SVD decomposition for solving. Faster but potentially less precise
// CALIB_USE_QR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:505
pub const CALIB_USE_QR: i32 = 1048576;
// CALIB_ZERO_DISPARITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:511
pub const CALIB_ZERO_DISPARITY: i32 = 1024;
// CALIB_ZERO_TANGENT_DIST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:492
pub const CALIB_ZERO_TANGENT_DIST: i32 = 8;
// CV_CALIB_CB_ADAPTIVE_THRESH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:208
pub const CV_CALIB_CB_ADAPTIVE_THRESH: i32 = 1;
// CV_CALIB_CB_FAST_CHECK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:211
pub const CV_CALIB_CB_FAST_CHECK: i32 = 8;
// CV_CALIB_CB_FILTER_QUADS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:210
pub const CV_CALIB_CB_FILTER_QUADS: i32 = 4;
// CV_CALIB_CB_NORMALIZE_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:209
pub const CV_CALIB_CB_NORMALIZE_IMAGE: i32 = 2;
// CV_CALIB_FIX_ASPECT_RATIO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:233
pub const CV_CALIB_FIX_ASPECT_RATIO: i32 = 2;
// CV_CALIB_FIX_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:236
pub const CV_CALIB_FIX_FOCAL_LENGTH: i32 = 16;
// CV_CALIB_FIX_INTRINSIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:278
pub const CV_CALIB_FIX_INTRINSIC: i32 = 256;
// CV_CALIB_FIX_K1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:237
pub const CV_CALIB_FIX_K1: i32 = 32;
// CV_CALIB_FIX_K2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:238
pub const CV_CALIB_FIX_K2: i32 = 64;
// CV_CALIB_FIX_K3 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:239
pub const CV_CALIB_FIX_K3: i32 = 128;
// CV_CALIB_FIX_K4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:240
pub const CV_CALIB_FIX_K4: i32 = 2048;
// CV_CALIB_FIX_K5 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:241
pub const CV_CALIB_FIX_K5: i32 = 4096;
// CV_CALIB_FIX_K6 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:242
pub const CV_CALIB_FIX_K6: i32 = 8192;
// CV_CALIB_FIX_PRINCIPAL_POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:234
pub const CV_CALIB_FIX_PRINCIPAL_POINT: i32 = 4;
// CV_CALIB_FIX_S1_S2_S3_S4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:245
pub const CV_CALIB_FIX_S1_S2_S3_S4: i32 = 65536;
// CV_CALIB_FIX_TANGENT_DIST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:248
pub const CV_CALIB_FIX_TANGENT_DIST: i32 = 2097152;
// CV_CALIB_FIX_TAUX_TAUY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:247
pub const CV_CALIB_FIX_TAUX_TAUY: i32 = 524288;
// CV_CALIB_NINTRINSIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:250
pub const CV_CALIB_NINTRINSIC: i32 = 18;
// CV_CALIB_RATIONAL_MODEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:243
pub const CV_CALIB_RATIONAL_MODEL: i32 = 16384;
// CV_CALIB_SAME_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:279
pub const CV_CALIB_SAME_FOCAL_LENGTH: i32 = 512;
// CV_CALIB_THIN_PRISM_MODEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:244
pub const CV_CALIB_THIN_PRISM_MODEL: i32 = 32768;
// CV_CALIB_TILTED_MODEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:246
pub const CV_CALIB_TILTED_MODEL: i32 = 262144;
// CV_CALIB_USE_INTRINSIC_GUESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:232
pub const CV_CALIB_USE_INTRINSIC_GUESS: i32 = 1;
// CV_CALIB_ZERO_DISPARITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:294
pub const CV_CALIB_ZERO_DISPARITY: i32 = 1024;
// CV_CALIB_ZERO_TANGENT_DIST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:235
pub const CV_CALIB_ZERO_TANGENT_DIST: i32 = 8;
// CV_FM_7POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:83
pub const CV_FM_7POINT: i32 = 1;
// CV_FM_8POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:84
pub const CV_FM_8POINT: i32 = 2;
// CV_FM_LMEDS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:91
pub const CV_FM_LMEDS: i32 = CV_LMEDS;
// CV_FM_LMEDS_ONLY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:89
pub const CV_FM_LMEDS_ONLY: i32 = CV_LMEDS;
// CV_FM_RANSAC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:92
pub const CV_FM_RANSAC: i32 = CV_RANSAC;
// CV_FM_RANSAC_ONLY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:90
pub const CV_FM_RANSAC_ONLY: i32 = CV_RANSAC;
// CV_LMEDS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:86
pub const CV_LMEDS: i32 = 4;
// CV_RANSAC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:87
pub const CV_RANSAC: i32 = 8;
// CV_STEREO_BM_BASIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:358
pub const CV_STEREO_BM_BASIC: i32 = 0;
// CV_STEREO_BM_FISH_EYE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:359
pub const CV_STEREO_BM_FISH_EYE: i32 = 1;
// CV_STEREO_BM_NARROW /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:360
pub const CV_STEREO_BM_NARROW: i32 = 2;
// CV_STEREO_BM_NORMALIZED_RESPONSE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:320
pub const CV_STEREO_BM_NORMALIZED_RESPONSE: i32 = 0;
// CV_STEREO_BM_XSOBEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d/calib3d_c.h:321
pub const CV_STEREO_BM_XSOBEL: i32 = 1;
// ASYMMETRIC_GRID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1205
pub const CirclesGridFinderParameters_ASYMMETRIC_GRID: i32 = 1;
// SYMMETRIC_GRID /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1205
pub const CirclesGridFinderParameters_SYMMETRIC_GRID: i32 = 0;
/// 7-point algorithm
// FM_7POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:517
pub const FM_7POINT: i32 = 1;
/// 8-point algorithm
// FM_8POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:518
pub const FM_8POINT: i32 = 2;
/// least-median algorithm. 7-point algorithm is used.
// FM_LMEDS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:519
pub const FM_LMEDS: i32 = 4;
/// RANSAC algorithm. It needs at least 15 points. 7-point algorithm is used.
// FM_RANSAC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:520
pub const FM_RANSAC: i32 = 8;
// CALIB_CHECK_COND /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2810
pub const Fisheye_CALIB_CHECK_COND: i32 = 4;
// CALIB_FIX_INTRINSIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2816
pub const Fisheye_CALIB_FIX_INTRINSIC: i32 = 256;
// CALIB_FIX_K1 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2812
pub const Fisheye_CALIB_FIX_K1: i32 = 16;
// CALIB_FIX_K2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2813
pub const Fisheye_CALIB_FIX_K2: i32 = 32;
// CALIB_FIX_K3 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2814
pub const Fisheye_CALIB_FIX_K3: i32 = 64;
// CALIB_FIX_K4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2815
pub const Fisheye_CALIB_FIX_K4: i32 = 128;
// CALIB_FIX_PRINCIPAL_POINT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2817
pub const Fisheye_CALIB_FIX_PRINCIPAL_POINT: i32 = 512;
// CALIB_FIX_SKEW /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2811
pub const Fisheye_CALIB_FIX_SKEW: i32 = 8;
// CALIB_RECOMPUTE_EXTRINSIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2809
pub const Fisheye_CALIB_RECOMPUTE_EXTRINSIC: i32 = 2;
// CALIB_USE_INTRINSIC_GUESS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2808
pub const Fisheye_CALIB_USE_INTRINSIC_GUESS: i32 = 1;
// CALIB_ZERO_DISPARITY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2818
pub const Fisheye_CALIB_ZERO_DISPARITY: i32 = 1024;
/// least-median of squares algorithm
// LMEDS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:447
pub const LMEDS: i32 = 4;
/// RANSAC algorithm
// RANSAC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:448
pub const RANSAC: i32 = 8;
/// RHO algorithm
// RHO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:449
pub const RHO: i32 = 16;
/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Ke17)
// SOLVEPNP_AP3P /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:462
pub const SOLVEPNP_AP3P: i32 = 5;
/// **Broken implementation. Using this flag will fallback to EPnP.** 
///
/// A Direct Least-Squares (DLS) Method for PnP [hesch2011direct](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_hesch2011direct)
// SOLVEPNP_DLS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:458
pub const SOLVEPNP_DLS: i32 = 3;
/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
// SOLVEPNP_EPNP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:456
pub const SOLVEPNP_EPNP: i32 = 1;
/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Collins14) 
///
/// Object points must be coplanar.
// SOLVEPNP_IPPE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:463
pub const SOLVEPNP_IPPE: i32 = 6;
/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Collins14) 
///
/// This is a special case suitable for marker pose estimation.
///
/// 4 coplanar object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
// SOLVEPNP_IPPE_SQUARE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:465
pub const SOLVEPNP_IPPE_SQUARE: i32 = 7;
/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Eade13) 
///
/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
///
/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
// SOLVEPNP_ITERATIVE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:453
pub const SOLVEPNP_ITERATIVE: i32 = 0;
/// Used for count
// SOLVEPNP_MAX_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:474
pub const SOLVEPNP_MAX_COUNT: i32 = 9;
/// Complete Solution Classification for the Perspective-Three-Point Problem [gao2003complete](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_gao2003complete)
// SOLVEPNP_P3P /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:457
pub const SOLVEPNP_P3P: i32 = 2;
/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
// SOLVEPNP_SQPNP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:472
pub const SOLVEPNP_SQPNP: i32 = 8;
/// **Broken implementation. Using this flag will fallback to EPnP.** 
///
/// Exhaustive Linearization for Robust Camera Pose and Focal Length Estimation [penate2013exhaustive](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_penate2013exhaustive)
// SOLVEPNP_UPNP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:460
pub const SOLVEPNP_UPNP: i32 = 4;
// PREFILTER_NORMALIZED_RESPONSE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2665
pub const StereoBM_PREFILTER_NORMALIZED_RESPONSE: i32 = 0;
// PREFILTER_XSOBEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2666
pub const StereoBM_PREFILTER_XSOBEL: i32 = 1;
// DISP_SCALE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2625
pub const StereoMatcher_DISP_SCALE: i32 = 16;
// DISP_SHIFT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2624
pub const StereoMatcher_DISP_SHIFT: i32 = 4;
// MODE_HH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2733
pub const StereoSGBM_MODE_HH: i32 = 1;
// MODE_HH4 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2735
pub const StereoSGBM_MODE_HH4: i32 = 3;
// MODE_SGBM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2732
pub const StereoSGBM_MODE_SGBM: i32 = 0;
// MODE_SGBM_3WAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2734
pub const StereoSGBM_MODE_SGBM_3WAY: i32 = 2;
// GridType /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1203
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CirclesGridFinderParameters_GridType {
	SYMMETRIC_GRID = 0,
	ASYMMETRIC_GRID = 1,
}

impl TryFrom<i32> for CirclesGridFinderParameters_GridType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SYMMETRIC_GRID),
			1 => Ok(Self::ASYMMETRIC_GRID),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::calib3d::CirclesGridFinderParameters_GridType"))),
		}
	}
}

opencv_type_enum! { crate::calib3d::CirclesGridFinderParameters_GridType }

// HandEyeCalibrationMethod /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:523
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HandEyeCalibrationMethod {
	/// A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/Eye Calibration [Tsai89](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Tsai89)
	CALIB_HAND_EYE_TSAI = 0,
	/// Robot Sensor Calibration: Solving AX = XB on the Euclidean Group [Park94](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Park94)
	CALIB_HAND_EYE_PARK = 1,
	/// Hand-eye Calibration [Horaud95](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Horaud95)
	CALIB_HAND_EYE_HORAUD = 2,
	/// On-line Hand-Eye Calibration [Andreff99](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Andreff99)
	CALIB_HAND_EYE_ANDREFF = 3,
	/// Hand-Eye Calibration Using Dual Quaternions [Daniilidis98](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Daniilidis98)
	CALIB_HAND_EYE_DANIILIDIS = 4,
}

impl TryFrom<i32> for HandEyeCalibrationMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CALIB_HAND_EYE_TSAI),
			1 => Ok(Self::CALIB_HAND_EYE_PARK),
			2 => Ok(Self::CALIB_HAND_EYE_HORAUD),
			3 => Ok(Self::CALIB_HAND_EYE_ANDREFF),
			4 => Ok(Self::CALIB_HAND_EYE_DANIILIDIS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::calib3d::HandEyeCalibrationMethod"))),
		}
	}
}

opencv_type_enum! { crate::calib3d::HandEyeCalibrationMethod }

// SolvePnPMethod /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:452
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SolvePnPMethod {
	/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Eade13) 
	///
	/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
	///
	/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
	SOLVEPNP_ITERATIVE = 0,
	/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
	SOLVEPNP_EPNP = 1,
	/// Complete Solution Classification for the Perspective-Three-Point Problem [gao2003complete](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_gao2003complete)
	SOLVEPNP_P3P = 2,
	/// **Broken implementation. Using this flag will fallback to EPnP.** 
	///
	/// A Direct Least-Squares (DLS) Method for PnP [hesch2011direct](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_hesch2011direct)
	SOLVEPNP_DLS = 3,
	/// **Broken implementation. Using this flag will fallback to EPnP.** 
	///
	/// Exhaustive Linearization for Robust Camera Pose and Focal Length Estimation [penate2013exhaustive](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_penate2013exhaustive)
	SOLVEPNP_UPNP = 4,
	/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Ke17)
	SOLVEPNP_AP3P = 5,
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Collins14) 
	///
	/// Object points must be coplanar.
	SOLVEPNP_IPPE = 6,
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Collins14) 
	///
	/// This is a special case suitable for marker pose estimation.
	///
	/// 4 coplanar object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	SOLVEPNP_IPPE_SQUARE = 7,
	/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
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
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::calib3d::SolvePnPMethod"))),
		}
	}
}

opencv_type_enum! { crate::calib3d::SolvePnPMethod }

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
/// object, e.g. see [Slabaugh](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned tree rotation matrices and corresponding three Euler angles
/// are only one of the possible solutions.
///
/// ## Note
/// This alternative version of [rq_decomp3x3] function uses the following default values for its arguments:
/// * qx: noArray()
/// * qy: noArray()
/// * qz: noArray()
// cv::RQDecomp3x3(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:654
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
/// object, e.g. see [Slabaugh](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned tree rotation matrices and corresponding three Euler angles
/// are only one of the possible solutions.
///
/// ## C++ default parameters
/// * qx: noArray()
/// * qy: noArray()
/// * qz: noArray()
// RQDecomp3x3(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:654
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
///    - A Compact Formula for the Derivative of a 3-D Rotation in Exponential Coordinates, Guillermo Gallego, Anthony J. Yezzi [Gallego2014ACF](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Gallego2014ACF)
///
///
/// Note: Useful information on SE(3) and Lie Groups can be found in:
///    - A tutorial on SE(3) transformation parameterizations and on-manifold optimization, Jose-Luis Blanco [blanco2010tutorial](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_blanco2010tutorial)
///    - Lie Groups for 2D and 3D Transformation, Ethan Eade [Eade17](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Eade17)
///    - A micro Lie theory for state estimation in robotics, Joan Solà, Jérémie Deray, Dinesh Atchuthan [Sol2018AML](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Sol2018AML)
///
/// ## Note
/// This alternative version of [rodrigues] function uses the following default values for its arguments:
/// * jacobian: noArray()
// cv::Rodrigues(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:559
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
///    - A Compact Formula for the Derivative of a 3-D Rotation in Exponential Coordinates, Guillermo Gallego, Anthony J. Yezzi [Gallego2014ACF](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Gallego2014ACF)
///
///
/// Note: Useful information on SE(3) and Lie Groups can be found in:
///    - A tutorial on SE(3) transformation parameterizations and on-manifold optimization, Jose-Luis Blanco [blanco2010tutorial](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_blanco2010tutorial)
///    - Lie Groups for 2D and 3D Transformation, Ethan Eade [Eade17](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Eade17)
///    - A micro Lie theory for state estimation in robotics, Joan Solà, Jérémie Deray, Dinesh Atchuthan [Sol2018AML](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Sol2018AML)
///
/// ## C++ default parameters
/// * jacobian: noArray()
// Rodrigues(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:559
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

/// @overload double calibrateCamera( InputArrayOfArrays objectPoints,
/// InputArrayOfArrays imagePoints, Size imageSize,
/// InputOutputArray cameraMatrix, InputOutputArray distCoeffs,
/// OutputArrayOfArrays rvecs, OutputArrayOfArrays tvecs,
/// OutputArray stdDeviations, OutputArray perViewErrors,
/// int flags = 0, TermCriteria criteria = TermCriteria(
///    TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON) )
///
/// ## Note
/// This alternative version of [calibrate_camera] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// cv::calibrateCamera(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1400
// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// describtion above.
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
/// views. The algorithm is based on [Zhang2000](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zhang2000) and [BouguetMCT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BouguetMCT) . The coordinates of 3D object
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
///
/// Note:
///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
/// ## See also
/// findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
///
/// ## Note
/// This alternative version of [calibrate_camera_extended] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// cv::calibrateCamera(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1382
// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// describtion above.
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
/// views. The algorithm is based on [Zhang2000](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zhang2000) and [BouguetMCT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BouguetMCT) . The coordinates of 3D object
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
///
/// Note:
///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
/// ## See also
/// findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCamera(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1382
// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// describtion above.
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
/// views. The algorithm is based on [Zhang2000](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Zhang2000) and [BouguetMCT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BouguetMCT) . The coordinates of 3D object
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
///
/// Note:
///    If you use a non-square (i.e. non-N-by-N) grid and [findChessboardCorners] for calibration,
///    and [calibrateCamera] returns bad values (zero distortion coefficients, ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx) and
///    ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) very far from the image center, and/or large differences between ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and
///    ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy) (ratios of 10:1 or more)), then you are probably using patternSize=cvSize(rows,cols)
///    instead of using patternSize=cvSize(cols,rows) in [findChessboardCorners].
/// ## See also
/// findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
///
/// ## Overloaded parameters
///  double calibrateCamera( InputArrayOfArrays objectPoints,
/// InputArrayOfArrays imagePoints, Size imageSize,
/// InputOutputArray cameraMatrix, InputOutputArray distCoeffs,
/// OutputArrayOfArrays rvecs, OutputArrayOfArrays tvecs,
/// OutputArray stdDeviations, OutputArray perViewErrors,
/// int flags = 0, TermCriteria criteria = TermCriteria(
///    TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON) )
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCamera(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1400
// ("cv::calibrateCamera", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// with the following implemented method:
///   - N. Andreff, R. Horaud, B. Espiau On-line Hand-Eye Calibration \cite Andreff99
///   - K. Daniilidis Hand-Eye Calibration Using Dual Quaternions \cite Daniilidis98
///
/// The following picture describes the Hand-Eye calibration problem where the transformation between a camera ("eye")
/// mounted on a robot gripper ("hand") has to be estimated.
///
/// ![](https://docs.opencv.org/3.4.20/hand-eye_figure.png)
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
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Balign%2A%7D%0A%20%20%20%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%5C%5C%0A%0A%20%20%20%20%28%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%28%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%29%5E%7B%2D1%7D%20%5C%5C%0A%0A%20%20%20%20%5Ctextrm%7BA%7D%5Fi%20%5Ctextrm%7BX%7D%20%26%3D%20%5Ctextrm%7BX%7D%20%5Ctextrm%7BB%7D%5Fi%20%5C%5C%0A%20%20%20%20%5Cend%7Balign%2A%7D%0A)
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
// cv::calibrateHandEye(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1905
// ("cv::calibrateHandEye", vec![(pred!(mut, ["R_gripper2base", "t_gripper2base", "R_target2cam", "t_target2cam", "R_cam2gripper", "t_cam2gripper"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// with the following implemented method:
///   - N. Andreff, R. Horaud, B. Espiau On-line Hand-Eye Calibration \cite Andreff99
///   - K. Daniilidis Hand-Eye Calibration Using Dual Quaternions \cite Daniilidis98
///
/// The following picture describes the Hand-Eye calibration problem where the transformation between a camera ("eye")
/// mounted on a robot gripper ("hand") has to be estimated.
///
/// ![](https://docs.opencv.org/3.4.20/hand-eye_figure.png)
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
/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%5Cbegin%7Balign%2A%7D%0A%20%20%20%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%5C%5C%0A%0A%20%20%20%20%28%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%282%29%7D%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bb%7D%7B%5Ctextrm%7BT%7D%5Fg%7D%5E%7B%281%29%7D%20%5Chspace%7B0%2E2em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%26%3D%0A%20%20%20%20%5Chspace%7B0%2E1em%7D%20%5E%7Bg%7D%5Ctextrm%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%282%29%7D%20%28%5E%7Bc%7D%7B%5Ctextrm%7BT%7D%5Ft%7D%5E%7B%281%29%7D%29%5E%7B%2D1%7D%20%5C%5C%0A%0A%20%20%20%20%5Ctextrm%7BA%7D%5Fi%20%5Ctextrm%7BX%7D%20%26%3D%20%5Ctextrm%7BX%7D%20%5Ctextrm%7BB%7D%5Fi%20%5C%5C%0A%20%20%20%20%5Cend%7Balign%2A%7D%0A)
///
/// \note
/// Additional information can be found on this [website](http://campar.in.tum.de/Chair/HandEyeCalibration).
/// \note
/// A minimum of 2 motions with non parallel rotation axes are necessary to determine the hand-eye transformation.
/// So at least 3 different poses are required, but it is strongly recommended to use many more poses.
///
/// ## C++ default parameters
/// * method: CALIB_HAND_EYE_TSAI
// calibrateHandEye(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, HandEyeCalibrationMethod)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1905
// ("cv::calibrateHandEye", vec![(pred!(mut, ["R_gripper2base", "t_gripper2base", "R_target2cam", "t_target2cam", "R_cam2gripper", "t_cam2gripper", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::HandEyeCalibrationMethod"]), _)]),
#[inline]
pub fn calibrate_hand_eye(r_gripper2base: &impl ToInputArray, t_gripper2base: &impl ToInputArray, r_target2cam: &impl ToInputArray, t_target2cam: &impl ToInputArray, r_cam2gripper: &mut impl ToOutputArray, t_cam2gripper: &mut impl ToOutputArray, method: crate::calib3d::HandEyeCalibrationMethod) -> Result<()> {
	input_array_arg!(r_gripper2base);
	input_array_arg!(t_gripper2base);
	input_array_arg!(r_target2cam);
	input_array_arg!(t_target2cam);
	output_array_arg!(r_cam2gripper);
	output_array_arg!(t_cam2gripper);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calibrateHandEye_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_HandEyeCalibrationMethod(r_gripper2base.as_raw__InputArray(), t_gripper2base.as_raw__InputArray(), r_target2cam.as_raw__InputArray(), t_target2cam.as_raw__InputArray(), r_cam2gripper.as_raw__OutputArray(), t_cam2gripper.as_raw__OutputArray(), method, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes useful camera characteristics from the camera intrinsic matrix.
///
/// ## Parameters
/// * cameraMatrix: Input camera intrinsic matrix that can be estimated by calibrateCamera or
/// stereoCalibrate .
/// * imageSize: Input image size in pixels.
/// * apertureWidth: Physical width in mm of the sensor.
/// * apertureHeight: Physical height in mm of the sensor.
/// * fovx: Output field of view in degrees along the horizontal sensor axis.
/// * fovy: Output field of view in degrees along the vertical sensor axis.
/// * focalLength: Focal length of the lens in mm.
/// * principalPoint: Principal point in mm.
/// * aspectRatio: ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy%2Ff%5Fx)
///
/// The function computes various useful camera characteristics from the previously estimated camera
/// matrix.
///
///
/// Note:
///    Do keep in mind that the unity measure 'mm' stands for whatever unit of measure one chooses for
///    the chessboard pitch (it can thus be any value).
// calibrationMatrixValues(InputArray, Size, double, double, double &, double &, double &, Point2d &, double &)(InputArray, SimpleClass, Primitive, Primitive, Indirect, Indirect, Indirect, SimpleClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1427
// ("cv::calibrationMatrixValues", vec![(pred!(mut, ["cameraMatrix", "imageSize", "apertureWidth", "apertureHeight", "fovx", "fovy", "focalLength", "principalPoint", "aspectRatio"], ["const cv::_InputArray*", "cv::Size", "double", "double", "double*", "double*", "double*", "cv::Point2d*", "double*"]), _)]),
#[inline]
pub fn calibration_matrix_values(camera_matrix: &impl ToInputArray, image_size: core::Size, aperture_width: f64, aperture_height: f64, fovx: &mut f64, fovy: &mut f64, focal_length: &mut f64, principal_point: &mut core::Point2d, aspect_ratio: &mut f64) -> Result<()> {
	input_array_arg!(camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_calibrationMatrixValues_const__InputArrayR_Size_double_double_doubleR_doubleR_doubleR_Point2dR_doubleR(camera_matrix.as_raw__InputArray(), &image_size, aperture_width, aperture_height, fovx, fovy, focal_length, principal_point, aspect_ratio, ocvrs_return.as_mut_ptr()) };
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
// cv::composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:732
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
// composeRT(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:732
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
// computeCorrespondEpilines(InputArray, int, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2255
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
///
/// The function converts points homogeneous to Euclidean space using perspective projection. That is,
/// each point (x1, x2, ... x(n-1), xn) is converted to (x1/xn, x2/xn, ..., x(n-1)/xn). When xn=0, the
/// output point coordinates will be (0,0,0,...).
// convertPointsFromHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1929
// ("cv::convertPointsFromHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn convert_points_from_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsFromHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
// convertPointsHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1941
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
///
/// The function converts points from Euclidean to homogeneous space by appending 1's to the tuple of
/// point coordinates. That is, each point (x1, x2, ..., xn) is converted to (x1, x2, ..., xn, 1).
// convertPointsToHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1918
// ("cv::convertPointsToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn convert_points_to_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_convertPointsToHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
/// The function implements the Optimal Triangulation Method (see Multiple View Geometry [HartleyZ00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_HartleyZ00) for details).
/// For each given point correspondence points1[i] \<-\> points2[i], and a fundamental matrix F, it
/// computes the corrected correspondences newPoints1[i] \<-\> newPoints2[i] that minimize the geometric
/// error ![inline formula](https://latex.codecogs.com/png.latex?d%28points1%5Bi%5D%2C%20newPoints1%5Bi%5D%29%5E2%20%2B%20d%28points2%5Bi%5D%2CnewPoints2%5Bi%5D%29%5E2) (where ![inline formula](https://latex.codecogs.com/png.latex?d%28a%2Cb%29) is the
/// geometric distance between points ![inline formula](https://latex.codecogs.com/png.latex?a) and ![inline formula](https://latex.codecogs.com/png.latex?b) ) subject to the epipolar constraint
/// ![inline formula](https://latex.codecogs.com/png.latex?newPoints2%5ET%20%5Ccdot%20F%20%5Ccdot%20newPoints1%20%3D%200) .
// correctMatches(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2301
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
/// This function decomposes the essential matrix E using svd decomposition [HartleyZ00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_HartleyZ00). In
/// general, four possible poses exist for the decomposition of E. They are ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20t%5D),
/// ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20%2Dt%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20%2Dt%5D).
///
/// If E gives the epipolar constraint ![inline formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20A%5E%7B%2DT%7D%20E%20A%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200) between the image
/// points ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) in the first image and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) in second image, then any of the tuples
/// ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F1%2C%20%2Dt%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20t%5D), ![inline formula](https://latex.codecogs.com/png.latex?%5BR%5F2%2C%20%2Dt%5D) is a change of basis from the first
/// camera's coordinate system to the second camera's coordinate system. However, by decomposing E, one
/// can only get the direction of the translation. For this reason, the translation t is returned with
/// unit length.
// decomposeEssentialMat(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2113
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
/// the homography matrix H is described in detail in [Malis2007](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Malis2007).
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
// decomposeHomographyMat(InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2588
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
/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
/// tree rotation matrices and corresponding three Euler angles are only one of the possible solutions.
///
/// The function is based on [rq_decomp3x3] .
///
/// ## Note
/// This alternative version of [decompose_projection_matrix] function uses the following default values for its arguments:
/// * rot_matrix_x: noArray()
/// * rot_matrix_y: noArray()
/// * rot_matrix_z: noArray()
/// * euler_angles: noArray()
// cv::decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:681
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
/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
/// tree rotation matrices and corresponding three Euler angles are only one of the possible solutions.
///
/// The function is based on [rq_decomp3x3] .
///
/// ## C++ default parameters
/// * rot_matrix_x: noArray()
/// * rot_matrix_y: noArray()
/// * rot_matrix_z: noArray()
/// * euler_angles: noArray()
// decomposeProjectionMatrix(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:681
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

/// Renders the detected chessboard corners.
///
/// ## Parameters
/// * image: Destination image. It must be an 8-bit color image.
/// * patternSize: Number of inner corners per a chessboard row and column
/// (patternSize = cv::Size(points_per_row,points_per_column)).
/// * corners: Array of detected corners, the output of #findChessboardCorners.
/// * patternWasFound: Parameter indicating whether the complete board was found or not. The
/// return value of [find_chessboard_corners] should be passed here.
///
/// The function draws individual chessboard corners detected either as red circles if the board was not
/// found, or as colored corners connected with lines if the board was found.
// drawChessboardCorners(InputOutputArray, Size, InputArray, bool)(InputOutputArray, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1164
// ("cv::drawChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners", "patternWasFound"], ["const cv::_InputOutputArray*", "cv::Size", "const cv::_InputArray*", "bool"]), _)]),
#[inline]
pub fn draw_chessboard_corners(image: &mut impl ToInputOutputArray, pattern_size: core::Size, corners: &impl ToInputArray, pattern_was_found: bool) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawChessboardCorners_const__InputOutputArrayR_Size_const__InputArrayR_bool(image.as_raw__InputOutputArray(), &pattern_size, corners.as_raw__InputArray(), pattern_was_found, ocvrs_return.as_mut_ptr()) };
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
// cv::drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1183
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
// drawFrameAxes(InputOutputArray, InputArray, InputArray, InputArray, InputArray, float, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1183
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
// cv::estimateAffine2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2507
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
// estimateAffine2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2507
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
// cv::estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2441
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
// estimateAffine3D(InputArray, InputArray, OutputArray, OutputArray, double, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2441
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
// cv::estimateAffinePartial2D(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2555
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
// estimateAffinePartial2D(InputArray, InputArray, OutputArray, int, double, size_t, double, size_t)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2555
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
/// information as described in [Malis2007](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Malis2007) . The summary of the method: the [decompose_homography_mat] function
/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
/// sets of points visible in the camera frame before and after the homography transformation is applied,
/// we can determine which are the true potential solutions and which are the opposites by verifying which
/// homographies are consistent with all visible reference points being in front of the camera. The inputs
/// are left unchanged; the filtered solution set is returned as indices into the existing one.
///
/// ## Note
/// This alternative version of [filter_homography_decomp_by_visible_refpoints] function uses the following default values for its arguments:
/// * points_mask: noArray()
// cv::filterHomographyDecompByVisibleRefpoints(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2612
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
/// information as described in [Malis2007](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Malis2007) . The summary of the method: the [decompose_homography_mat] function
/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
/// sets of points visible in the camera frame before and after the homography transformation is applied,
/// we can determine which are the true potential solutions and which are the opposites by verifying which
/// homographies are consistent with all visible reference points being in front of the camera. The inputs
/// are left unchanged; the filtered solution set is returned as indices into the existing one.
///
/// ## C++ default parameters
/// * points_mask: noArray()
// filterHomographyDecompByVisibleRefpoints(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2612
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

/// Filters off small noise blobs (speckles) in the disparity map
///
/// ## Parameters
/// * img: The input 16-bit signed disparity image
/// * newVal: The disparity value used to paint-off the speckles
/// * maxSpeckleSize: The maximum speckle size to consider it a speckle. Larger blobs are not
/// affected by the algorithm
/// * maxDiff: Maximum difference between neighbor disparity pixels to put them into the same
/// blob. Note that since StereoBM, StereoSGBM and may be other algorithms return a fixed-point
/// disparity map, where disparity values are multiplied by 16, this scale factor should be taken into
/// account when specifying this parameter value.
/// * buf: The optional temporary buffer to avoid memory allocation within the function.
///
/// ## Note
/// This alternative version of [filter_speckles] function uses the following default values for its arguments:
/// * buf: noArray()
// cv::filterSpeckles(InputOutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2316
// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff"], ["const cv::_InputOutputArray*", "double", "int", "double"]), _)]),
#[inline]
pub fn filter_speckles_def(img: &mut impl ToInputOutputArray, new_val: f64, max_speckle_size: i32, max_diff: f64) -> Result<()> {
	input_output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_filterSpeckles_const__InputOutputArrayR_double_int_double(img.as_raw__InputOutputArray(), new_val, max_speckle_size, max_diff, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Filters off small noise blobs (speckles) in the disparity map
///
/// ## Parameters
/// * img: The input 16-bit signed disparity image
/// * newVal: The disparity value used to paint-off the speckles
/// * maxSpeckleSize: The maximum speckle size to consider it a speckle. Larger blobs are not
/// affected by the algorithm
/// * maxDiff: Maximum difference between neighbor disparity pixels to put them into the same
/// blob. Note that since StereoBM, StereoSGBM and may be other algorithms return a fixed-point
/// disparity map, where disparity values are multiplied by 16, this scale factor should be taken into
/// account when specifying this parameter value.
/// * buf: The optional temporary buffer to avoid memory allocation within the function.
///
/// ## C++ default parameters
/// * buf: noArray()
// filterSpeckles(InputOutputArray, double, int, double, InputOutputArray)(InputOutputArray, Primitive, Primitive, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2316
// ("cv::filterSpeckles", vec![(pred!(mut, ["img", "newVal", "maxSpeckleSize", "maxDiff", "buf"], ["const cv::_InputOutputArray*", "double", "int", "double", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn filter_speckles(img: &mut impl ToInputOutputArray, new_val: f64, max_speckle_size: i32, max_diff: f64, buf: &mut impl ToInputOutputArray) -> Result<()> {
	input_output_array_arg!(img);
	input_output_array_arg!(buf);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_filterSpeckles_const__InputOutputArrayR_double_int_double_const__InputOutputArrayR(img.as_raw__InputOutputArray(), new_val, max_speckle_size, max_diff, buf.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// finds subpixel-accurate positions of the chessboard corners
// find4QuadCornerSubpix(InputArray, InputOutputArray, Size)(InputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1150
// ("cv::find4QuadCornerSubpix", vec![(pred!(mut, ["img", "corners", "region_size"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Size"]), _)]),
#[inline]
pub fn find4_quad_corner_subpix(img: &impl ToInputArray, corners: &mut impl ToInputOutputArray, region_size: core::Size) -> Result<bool> {
	input_array_arg!(img);
	input_output_array_arg!(corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_find4QuadCornerSubpix_const__InputArrayR_const__InputOutputArrayR_Size(img.as_raw__InputArray(), corners.as_raw__InputOutputArray(), &region_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds the positions of internal corners of the chessboard.
///
/// ## Parameters
/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
/// * patternSize: Number of inner corners per a chessboard row and column
/// ( patternSize = cv::Size(points_per_row,points_per_colum) = cv::Size(columns,rows) ).
/// * corners: Output array of detected corners.
/// * flags: Various operation flags that can be zero or a combination of the following values:
/// *   [CALIB_CB_ADAPTIVE_THRESH] Use adaptive thresholding to convert the image to black
/// and white, rather than a fixed threshold level (computed from the average image brightness).
/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with [equalize_hist] before
/// applying fixed or adaptive thresholding.
/// *   [CALIB_CB_FILTER_QUADS] Use additional criteria (like contour area, perimeter,
/// square-like shape) to filter out false quads extracted at the contour retrieval stage.
/// *   [CALIB_CB_FAST_CHECK] Run a fast check on the image that looks for chessboard corners,
/// and shortcut the call if none is found. This can drastically speed up the call in the
/// degenerate condition when no chessboard is observed.
///
/// The function attempts to determine whether the input image is a view of the chessboard pattern and
/// locate the internal chessboard corners. The function returns a non-zero value if all of the corners
/// are found and they are placed in a certain order (row by row, left to right in every row).
/// Otherwise, if the function fails to find all the corners or reorder them, it returns 0. For example,
/// a regular chessboard has 8 x 8 squares and 7 x 7 internal corners, that is, points where the black
/// squares touch each other. The detected coordinates are approximate, and to determine their positions
/// more accurately, the function calls #cornerSubPix. You also may use the function [corner_sub_pix] with
/// different parameters if returned coordinates are not accurate enough.
///
/// Sample usage of detecting and drawing chessboard corners: :
/// ```C++
///    Size patternsize(8,6); //interior number of corners
///    Mat gray = ....; //source image
///    vector<Point2f> corners; //this will be filled by the detected corners
///
///    //CALIB_CB_FAST_CHECK saves a lot of time on images
///    //that do not contain any chessboard corners
///    bool patternfound = findChessboardCorners(gray, patternsize, corners,
///            CALIB_CB_ADAPTIVE_THRESH + CALIB_CB_NORMALIZE_IMAGE
///            + CALIB_CB_FAST_CHECK);
///
///    if(patternfound)
///       cornerSubPix(gray, corners, Size(11, 11), Size(-1, -1),
///        TermCriteria(CV_TERMCRIT_EPS + CV_TERMCRIT_ITER, 30, 0.1));
///
///    drawChessboardCorners(img, patternsize, Mat(corners), patternfound);
/// ```
///
///
/// Note: The function requires white space (like a square-thick border, the wider the better) around
/// the board to make the detection more robust in various environments. Otherwise, if there is no
/// border and the background is dark, the outer black squares cannot be segmented properly and so the
/// square grouping and ordering algorithm fails.
///
/// ## Note
/// This alternative version of [find_chessboard_corners] function uses the following default values for its arguments:
/// * flags: CALIB_CB_ADAPTIVE_THRESH+CALIB_CB_NORMALIZE_IMAGE
// cv::findChessboardCorners(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1146
// ("cv::findChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_chessboard_corners_def(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds the positions of internal corners of the chessboard.
///
/// ## Parameters
/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
/// * patternSize: Number of inner corners per a chessboard row and column
/// ( patternSize = cv::Size(points_per_row,points_per_colum) = cv::Size(columns,rows) ).
/// * corners: Output array of detected corners.
/// * flags: Various operation flags that can be zero or a combination of the following values:
/// *   [CALIB_CB_ADAPTIVE_THRESH] Use adaptive thresholding to convert the image to black
/// and white, rather than a fixed threshold level (computed from the average image brightness).
/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with [equalize_hist] before
/// applying fixed or adaptive thresholding.
/// *   [CALIB_CB_FILTER_QUADS] Use additional criteria (like contour area, perimeter,
/// square-like shape) to filter out false quads extracted at the contour retrieval stage.
/// *   [CALIB_CB_FAST_CHECK] Run a fast check on the image that looks for chessboard corners,
/// and shortcut the call if none is found. This can drastically speed up the call in the
/// degenerate condition when no chessboard is observed.
///
/// The function attempts to determine whether the input image is a view of the chessboard pattern and
/// locate the internal chessboard corners. The function returns a non-zero value if all of the corners
/// are found and they are placed in a certain order (row by row, left to right in every row).
/// Otherwise, if the function fails to find all the corners or reorder them, it returns 0. For example,
/// a regular chessboard has 8 x 8 squares and 7 x 7 internal corners, that is, points where the black
/// squares touch each other. The detected coordinates are approximate, and to determine their positions
/// more accurately, the function calls #cornerSubPix. You also may use the function [corner_sub_pix] with
/// different parameters if returned coordinates are not accurate enough.
///
/// Sample usage of detecting and drawing chessboard corners: :
/// ```C++
///    Size patternsize(8,6); //interior number of corners
///    Mat gray = ....; //source image
///    vector<Point2f> corners; //this will be filled by the detected corners
///
///    //CALIB_CB_FAST_CHECK saves a lot of time on images
///    //that do not contain any chessboard corners
///    bool patternfound = findChessboardCorners(gray, patternsize, corners,
///            CALIB_CB_ADAPTIVE_THRESH + CALIB_CB_NORMALIZE_IMAGE
///            + CALIB_CB_FAST_CHECK);
///
///    if(patternfound)
///       cornerSubPix(gray, corners, Size(11, 11), Size(-1, -1),
///        TermCriteria(CV_TERMCRIT_EPS + CV_TERMCRIT_ITER, 30, 0.1));
///
///    drawChessboardCorners(img, patternsize, Mat(corners), patternfound);
/// ```
///
///
/// Note: The function requires white space (like a square-thick border, the wider the better) around
/// the board to make the detection more robust in various environments. Otherwise, if there is no
/// border and the background is dark, the outer black squares cannot be segmented properly and so the
/// square grouping and ordering algorithm fails.
///
/// ## C++ default parameters
/// * flags: CALIB_CB_ADAPTIVE_THRESH+CALIB_CB_NORMALIZE_IMAGE
// findChessboardCorners(InputArray, Size, OutputArray, int)(InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1146
// ("cv::findChessboardCorners", vec![(pred!(mut, ["image", "patternSize", "corners", "flags"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn find_chessboard_corners(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR_int(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
// findCirclesGrid2(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &, CirclesGridFinderParameters2)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1257
// ("cv::findCirclesGrid2", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector", "parameters"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*", "cv::CirclesGridFinderParameters2"]), _)]),
#[inline]
pub fn find_circles_grid2(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray, flags: i32, blob_detector: &core::Ptr<crate::features2d::Feature2D>, mut parameters: impl crate::calib3d::CirclesGridFinderParameters2Trait) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(centers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findCirclesGrid2_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR_CirclesGridFinderParameters2(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), flags, blob_detector.as_raw_PtrOfFeature2D(), parameters.as_raw_mut_CirclesGridFinderParameters2(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_circles_grid_1] function uses the following default values for its arguments:
/// * flags: CALIB_CB_SYMMETRIC_GRID
/// * blob_detector: SimpleBlobDetector::create()
// cv::findCirclesGrid(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1263
// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_circles_grid_1_def(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(centers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds centers in the grid of circles.
///
/// ## Parameters
/// * image: grid view of input circles; it must be an 8-bit grayscale or color image.
/// * patternSize: number of circles per row and column
/// ( patternSize = Size(points_per_row, points_per_colum) ).
/// * centers: output array of detected centers.
/// * flags: various operation flags that can be one of the following values:
/// *   [CALIB_CB_SYMMETRIC_GRID] uses symmetric pattern of circles.
/// *   [CALIB_CB_ASYMMETRIC_GRID] uses asymmetric pattern of circles.
/// *   [CALIB_CB_CLUSTERING] uses a special algorithm for grid detection. It is more robust to
/// perspective distortions but much more sensitive to background clutter.
/// * blobDetector: feature detector that finds blobs like dark circles on light background.
///                    If `blobDetector` is NULL then `image` represents Point2f array of candidates.
/// * parameters: struct for finding circles in a grid pattern.
///
/// The function attempts to determine whether the input image contains a grid of circles. If it is, the
/// function locates centers of the circles. The function returns a non-zero value if all of the centers
/// have been found and they have been placed in a certain order (row by row, left to right in every
/// row). Otherwise, if the function fails to find all the corners or reorder them, it returns 0.
///
/// Sample usage of detecting and drawing the centers of circles: :
/// ```C++
///    Size patternsize(7,7); //number of centers
///    Mat gray = ...; //source image
///    vector<Point2f> centers; //this will be filled by the detected centers
///
///    bool patternfound = findCirclesGrid(gray, patternsize, centers);
///
///    drawChessboardCorners(img, patternsize, Mat(centers), patternfound);
/// ```
///
///
/// Note: The function requires white space (like a square-thick border, the wider the better) around
/// the board to make the detection more robust in various environments.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * flags: CALIB_CB_SYMMETRIC_GRID
/// * blob_detector: SimpleBlobDetector::create()
// findCirclesGrid(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1263
// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*"]), _)]),
#[inline]
pub fn find_circles_grid_1(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray, flags: i32, blob_detector: Option<&core::Ptr<crate::features2d::Feature2D>>) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(centers);
	smart_ptr_option_arg!(unsafe ref blob_detector);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), flags, blob_detector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds centers in the grid of circles.
///
/// ## Parameters
/// * image: grid view of input circles; it must be an 8-bit grayscale or color image.
/// * patternSize: number of circles per row and column
/// ( patternSize = Size(points_per_row, points_per_colum) ).
/// * centers: output array of detected centers.
/// * flags: various operation flags that can be one of the following values:
/// *   [CALIB_CB_SYMMETRIC_GRID] uses symmetric pattern of circles.
/// *   [CALIB_CB_ASYMMETRIC_GRID] uses asymmetric pattern of circles.
/// *   [CALIB_CB_CLUSTERING] uses a special algorithm for grid detection. It is more robust to
/// perspective distortions but much more sensitive to background clutter.
/// * blobDetector: feature detector that finds blobs like dark circles on light background.
///                    If `blobDetector` is NULL then `image` represents Point2f array of candidates.
/// * parameters: struct for finding circles in a grid pattern.
///
/// The function attempts to determine whether the input image contains a grid of circles. If it is, the
/// function locates centers of the circles. The function returns a non-zero value if all of the centers
/// have been found and they have been placed in a certain order (row by row, left to right in every
/// row). Otherwise, if the function fails to find all the corners or reorder them, it returns 0.
///
/// Sample usage of detecting and drawing the centers of circles: :
/// ```C++
///    Size patternsize(7,7); //number of centers
///    Mat gray = ...; //source image
///    vector<Point2f> centers; //this will be filled by the detected centers
///
///    bool patternfound = findCirclesGrid(gray, patternsize, centers);
///
///    drawChessboardCorners(img, patternsize, Mat(centers), patternfound);
/// ```
///
///
/// Note: The function requires white space (like a square-thick border, the wider the better) around
/// the board to make the detection more robust in various environments.
// findCirclesGrid(InputArray, Size, OutputArray, int, const Ptr<FeatureDetector> &, CirclesGridFinderParameters)(InputArray, SimpleClass, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1251
// ("cv::findCirclesGrid", vec![(pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector", "parameters"], ["const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "int", "const cv::Ptr<cv::Feature2D>*", "cv::CirclesGridFinderParameters"]), _)]),
#[inline]
pub fn find_circles_grid(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray, flags: i32, blob_detector: Option<&core::Ptr<crate::features2d::Feature2D>>, mut parameters: impl crate::calib3d::CirclesGridFinderParametersTrait) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(centers);
	smart_ptr_option_arg!(unsafe ref blob_detector);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR_CirclesGridFinderParameters(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), flags, blob_detector.as_raw_PtrOfFeature2D(), parameters.as_raw_mut_CirclesGridFinderParameters(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_essential_mat_2] function uses the following default values for its arguments:
/// * focal: 1.0
/// * pp: Point2d(0,0)
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2090
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_2_def(points1: &impl ToInputArray, points2: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [find_essential_mat_matrix] function uses the following default values for its arguments:
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2048
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_matrix_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray) -> Result<core::Mat> {
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

/// Calculates an essential matrix from the corresponding points in two images.
///
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use
/// `undistortPoints()` with `P = cv::NoArray()` for both cameras to transform image points
/// to normalized image coordinates, which are valid for the identity camera intrinsic matrix. When
/// passing these coordinates, pass the identity matrix for this parameter.
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
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2048
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_matrix(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, method: i32, prob: f64, threshold: f64, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), method, prob, threshold, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use
/// `undistortPoints()` with `P = cv::NoArray()` for both cameras to transform image points
/// to normalized image coordinates, which are valid for the identity camera intrinsic matrix. When
/// passing these coordinates, pass the identity matrix for this parameter.
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
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## Note
/// This alternative version of [find_essential_mat] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2042
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "cameraMatrix", "method", "prob", "threshold", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "double", "int"]), _)]),
#[inline]
pub fn find_essential_mat_def(points1: &impl ToInputArray, points2: &impl ToInputArray, camera_matrix: &impl ToInputArray, method: i32, prob: f64, threshold: f64, max_iters: i32) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(camera_matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_double_double_int(points1.as_raw__InputArray(), points2.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), method, prob, threshold, max_iters, ocvrs_return.as_mut_ptr()) };
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
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use
/// `undistortPoints()` with `P = cv::NoArray()` for both cameras to transform image points
/// to normalized image coordinates, which are valid for the identity camera intrinsic matrix. When
/// passing these coordinates, pass the identity matrix for this parameter.
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
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## C++ default parameters
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2042
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
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use
/// `undistortPoints()` with `P = cv::NoArray()` for both cameras to transform image points
/// to normalized image coordinates, which are valid for the identity camera intrinsic matrix. When
/// passing these coordinates, pass the identity matrix for this parameter.
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
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Bp%5F2%3B%201%5D%5ET%20K%5E%7B%2DT%7D%20E%20K%5E%7B%2D1%7D%20%5Bp%5F1%3B%201%5D%20%3D%200)
///
/// where ![inline formula](https://latex.codecogs.com/png.latex?E) is an essential matrix, ![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// [decompose_essential_mat] or [recover_pose] to recover the relative pose between cameras.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * focal: 1.0
/// * pp: Point2d(0,0)
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2090
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn find_essential_mat_2(points1: &impl ToInputArray, points2: &impl ToInputArray, focal: f64, pp: core::Point2d, method: i32, prob: f64, threshold: f64, mask: &mut impl ToOutputArray) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	output_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), focal, &pp, method, prob, threshold, mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
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
/// * mask: noArray()
// cv::findEssentialMat(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2083
// ("cv::findEssentialMat", vec![(pred!(mut, ["points1", "points2", "focal", "pp", "method", "prob", "threshold", "maxIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "cv::Point2d", "int", "double", "double", "int"]), _)]),
#[inline]
pub fn find_essential_mat_1_def(points1: &impl ToInputArray, points2: &impl ToInputArray, focal: f64, pp: core::Point2d, method: i32, prob: f64, threshold: f64, max_iters: i32) -> Result<core::Mat> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_findEssentialMat_const__InputArrayR_const__InputArrayR_double_Point2d_int_double_double_int(points1.as_raw__InputArray(), points2.as_raw__InputArray(), focal, &pp, method, prob, threshold, max_iters, ocvrs_return.as_mut_ptr()) };
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
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera intrinsic matrix. If this assumption does not hold for your use case, use
/// `undistortPoints()` with `P = cv::NoArray()` for both cameras to transform image points
/// to normalized image coordinates, which are valid for the identity camera intrinsic matrix. When
/// passing these coordinates, pass the identity matrix for this parameter.
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
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
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
/// * mask: noArray()
// findEssentialMat(InputArray, InputArray, double, Point2d, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2083
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
// cv::findFundamentalMat(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1999
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
// cv::findFundamentalMat(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2005
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
// findFundamentalMat(InputArray, InputArray, OutputArray, int, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2005
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
// findFundamentalMat(InputArray, InputArray, int, double, double, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1999
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
// cv::findFundamentalMat(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1994
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
// findFundamentalMat(InputArray, InputArray, int, double, double, int, OutputArray)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1994
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
/// determined up to a scale. Thus, it is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1). Note that whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix
/// cannot be estimated, an empty one will be returned.
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
// cv::findHomography(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:626
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
// cv::findHomography(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:632
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
/// determined up to a scale. Thus, it is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1). Note that whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix
/// cannot be estimated, an empty one will be returned.
/// ## See also
/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
/// perspectiveTransform
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: 0
/// * ransac_reproj_threshold: 3
// findHomography(InputArray, InputArray, OutputArray, int, double)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:632
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
/// determined up to a scale. Thus, it is normalized so that ![inline formula](https://latex.codecogs.com/png.latex?h%5F%7B33%7D%3D1). Note that whenever an ![inline formula](https://latex.codecogs.com/png.latex?H) matrix
/// cannot be estimated, an empty one will be returned.
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
// findHomography(InputArray, InputArray, int, double, OutputArray, const int, const double)(InputArray, InputArray, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:626
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

/// Performs camera calibration
///
/// ## Parameters
/// * objectPoints: vector of vectors of calibration pattern points in the calibration pattern
///    coordinate space.
/// * imagePoints: vector of vectors of the projections of calibration pattern points.
///    imagePoints.size() and objectPoints.size() and imagePoints[i].size() must be equal to
///    objectPoints[i].size() for each i.
/// * image_size: Size of the image used only to initialize the camera intrinsic matrix.
/// * K: Output 3x3 floating-point camera intrinsic matrix
///    ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If
///    [fisheye::CALIB_USE_INTRINSIC_GUESS] is specified, some or all of fx, fy, cx, cy must be
///    initialized before calling the function.
/// * D: Output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view.
///    That is, each k-th rotation vector together with the corresponding k-th translation vector (see
///    the next output parameter description) brings the calibration pattern from the model coordinate
///    space (in which object points are specified) to the world coordinate space, that is, a real
///    position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * flags: Different flags that may be zero or a combination of the following values:
///    *    [fisheye::CALIB_USE_INTRINSIC_GUESS]  cameraMatrix contains valid initial values of
///    fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
///    center ( imageSize is used), and focal distances are computed in a least-squares fashion.
///    *    [fisheye::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
///    of intrinsic optimization.
///    *    [fisheye::CALIB_CHECK_COND]  The functions will check validity of condition number.
///    *    [fisheye::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
///    *    [fisheye::CALIB_FIX_K1],..., [fisheye::CALIB_FIX_K4] Selected distortion coefficients
///    are set to zeros and stay zero.
///    *    [fisheye::CALIB_FIX_PRINCIPAL_POINT]  The principal point is not changed during the global
/// optimization. It stays at the center or at a different location specified when [fisheye::CALIB_USE_INTRINSIC_GUESS] is set too.
/// * criteria: Termination criteria for the iterative optimization algorithm.
///
/// ## Note
/// This alternative version of [calibrate] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
// cv::fisheye::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2974
// ("cv::fisheye::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "image_size", "K", "D", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Performs camera calibration
///
/// ## Parameters
/// * objectPoints: vector of vectors of calibration pattern points in the calibration pattern
///    coordinate space.
/// * imagePoints: vector of vectors of the projections of calibration pattern points.
///    imagePoints.size() and objectPoints.size() and imagePoints[i].size() must be equal to
///    objectPoints[i].size() for each i.
/// * image_size: Size of the image used only to initialize the camera intrinsic matrix.
/// * K: Output 3x3 floating-point camera intrinsic matrix
///    ![inline formula](https://latex.codecogs.com/png.latex?%5Ccameramatrix%7BA%7D) . If
///    [fisheye::CALIB_USE_INTRINSIC_GUESS] is specified, some or all of fx, fy, cx, cy must be
///    initialized before calling the function.
/// * D: Output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye).
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view.
///    That is, each k-th rotation vector together with the corresponding k-th translation vector (see
///    the next output parameter description) brings the calibration pattern from the model coordinate
///    space (in which object points are specified) to the world coordinate space, that is, a real
///    position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * flags: Different flags that may be zero or a combination of the following values:
///    *    [fisheye::CALIB_USE_INTRINSIC_GUESS]  cameraMatrix contains valid initial values of
///    fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
///    center ( imageSize is used), and focal distances are computed in a least-squares fashion.
///    *    [fisheye::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
///    of intrinsic optimization.
///    *    [fisheye::CALIB_CHECK_COND]  The functions will check validity of condition number.
///    *    [fisheye::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
///    *    [fisheye::CALIB_FIX_K1],..., [fisheye::CALIB_FIX_K4] Selected distortion coefficients
///    are set to zeros and stay zero.
///    *    [fisheye::CALIB_FIX_PRINCIPAL_POINT]  The principal point is not changed during the global
/// optimization. It stays at the center or at a different location specified when [fisheye::CALIB_USE_INTRINSIC_GUESS] is set too.
/// * criteria: Termination criteria for the iterative optimization algorithm.
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
// calibrate(InputArrayOfArrays, InputArrayOfArrays, const Size &, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2974
// ("cv::fisheye::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "image_size", "K", "D", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
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
/// This means if you want to transform back points undistorted with undistortPoints() you have to
/// multiply them with ![inline formula](https://latex.codecogs.com/png.latex?P%5E%7B%2D1%7D).
///
/// ## Note
/// This alternative version of [fisheye_distort_points] function uses the following default values for its arguments:
/// * alpha: 0
// cv::fisheye::distortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2861
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
/// This means if you want to transform back points undistorted with undistortPoints() you have to
/// multiply them with ![inline formula](https://latex.codecogs.com/png.latex?P%5E%7B%2D1%7D).
///
/// ## C++ default parameters
/// * alpha: 0
// distortPoints(InputArray, OutputArray, InputArray, InputArray, double)(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2861
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
// cv::fisheye::estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, SimpleClass, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2938
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
// estimateNewCameraMatrixForUndistortRectify(InputArray, InputArray, const Size &, InputArray, OutputArray, double, const Size &, double)(InputArray, InputArray, SimpleClass, InputArray, OutputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2938
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
// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2891
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
// cv::fisheye::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2841
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
// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2841
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
// cv::fisheye::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2845
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
// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, double, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2845
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
/// any of [fisheye::CALIB_USE_INTRINSIC_GUESS] , [fisheye::CALIB_FIX_INTRINSIC] are specified,
/// some or all of the matrix components must be initialized.
/// * D1: Input/output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) of 4 elements.
/// * K2: Input/output second camera intrinsic matrix. The parameter is similar to K1 .
/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
/// similar to D1 .
/// * imageSize: Size of the image used only to initialize camera intrinsic matrix.
/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
/// * T: Output translation vector between the coordinate systems of the cameras.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *    [fisheye::CALIB_FIX_INTRINSIC]  Fix K1, K2? and D1, D2? so that only R, T matrices
/// are estimated.
/// *    [fisheye::CALIB_USE_INTRINSIC_GUESS]  K1, K2 contains valid initial values of
/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
/// *    [fisheye::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
/// of intrinsic optimization.
/// *    [fisheye::CALIB_CHECK_COND]  The functions will check validity of condition number.
/// *    [fisheye::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
/// *   [fisheye::CALIB_FIX_K1],..., [fisheye::CALIB_FIX_K4] Selected distortion coefficients are set to zeros and stay
/// zero.
/// * criteria: Termination criteria for the iterative optimization algorithm.
///
/// ## Note
/// This alternative version of [fisheye_stereo_calibrate] function uses the following default values for its arguments:
/// * flags: fisheye::CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
// cv::fisheye::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3044
// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// any of [fisheye::CALIB_USE_INTRINSIC_GUESS] , [fisheye::CALIB_FIX_INTRINSIC] are specified,
/// some or all of the matrix components must be initialized.
/// * D1: Input/output vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%5Cdistcoeffsfisheye) of 4 elements.
/// * K2: Input/output second camera intrinsic matrix. The parameter is similar to K1 .
/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
/// similar to D1 .
/// * imageSize: Size of the image used only to initialize camera intrinsic matrix.
/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
/// * T: Output translation vector between the coordinate systems of the cameras.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *    [fisheye::CALIB_FIX_INTRINSIC]  Fix K1, K2? and D1, D2? so that only R, T matrices
/// are estimated.
/// *    [fisheye::CALIB_USE_INTRINSIC_GUESS]  K1, K2 contains valid initial values of
/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
/// *    [fisheye::CALIB_RECOMPUTE_EXTRINSIC]  Extrinsic will be recomputed after each iteration
/// of intrinsic optimization.
/// *    [fisheye::CALIB_CHECK_COND]  The functions will check validity of condition number.
/// *    [fisheye::CALIB_FIX_SKEW]  Skew coefficient (alpha) is set to zero and stay zero.
/// *   [fisheye::CALIB_FIX_K1],..., [fisheye::CALIB_FIX_K4] Selected distortion coefficients are set to zeros and stay
/// zero.
/// * criteria: Termination criteria for the iterative optimization algorithm.
///
/// ## C++ default parameters
/// * flags: fisheye::CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,DBL_EPSILON)
// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3044
// ("cv::fisheye::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "K1", "D1", "K2", "D2", "imageSize", "R", "T", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo rectification for fisheye camera model
///
/// ## Parameters
/// * K1: First camera intrinsic matrix.
/// * D1: First camera distortion parameters.
/// * K2: Second camera intrinsic matrix.
/// * D2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix between the coordinate systems of the first and the second
/// cameras.
/// * tvec: Translation vector between coordinate systems of the cameras.
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see [reproject_image_to_3d] ).
/// * flags: Operation flags that may be zero or [fisheye::CALIB_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * fov_scale: Divisor for new focal length.
///
/// ## Note
/// This alternative version of [fisheye_stereo_rectify] function uses the following default values for its arguments:
/// * new_image_size: Size()
/// * balance: 0.0
/// * fov_scale: 1.0
// cv::fisheye::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3008
// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn fisheye_stereo_rectify_def(k1: &impl ToInputArray, d1: &impl ToInputArray, k2: &impl ToInputArray, d2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, tvec: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(k1);
	input_array_arg!(d1);
	input_array_arg!(k2);
	input_array_arg!(d2);
	input_array_arg!(r);
	input_array_arg!(tvec);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(k1.as_raw__InputArray(), d1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), tvec.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo rectification for fisheye camera model
///
/// ## Parameters
/// * K1: First camera intrinsic matrix.
/// * D1: First camera distortion parameters.
/// * K2: Second camera intrinsic matrix.
/// * D2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix between the coordinate systems of the first and the second
/// cameras.
/// * tvec: Translation vector between coordinate systems of the cameras.
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see [reproject_image_to_3d] ).
/// * flags: Operation flags that may be zero or [fisheye::CALIB_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * fov_scale: Divisor for new focal length.
///
/// ## C++ default parameters
/// * new_image_size: Size()
/// * balance: 0.0
/// * fov_scale: 1.0
// stereoRectify(InputArray, InputArray, InputArray, InputArray, const Size &, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, const Size &, double, double)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:3008
// ("cv::fisheye::stereoRectify", vec![(pred!(mut, ["K1", "D1", "K2", "D2", "imageSize", "R", "tvec", "R1", "R2", "P1", "P2", "Q", "flags", "newImageSize", "balance", "fov_scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::Size*", "double", "double"]), _)]),
#[inline]
pub fn fisheye_stereo_rectify(k1: &impl ToInputArray, d1: &impl ToInputArray, k2: &impl ToInputArray, d2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, tvec: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray, flags: i32, new_image_size: core::Size, balance: f64, fov_scale: f64) -> Result<()> {
	input_array_arg!(k1);
	input_array_arg!(d1);
	input_array_arg!(k2);
	input_array_arg!(d2);
	input_array_arg!(r);
	input_array_arg!(tvec);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SizeR_double_double(k1.as_raw__InputArray(), d1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), tvec.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), flags, &new_image_size, balance, fov_scale, ocvrs_return.as_mut_ptr()) };
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
/// ![image](https://docs.opencv.org/3.4.20/fisheye_undistorted.jpg)
///
/// ## Note
/// This alternative version of [fisheye_undistort_image] function uses the following default values for its arguments:
/// * knew: cv::noArray()
/// * new_size: Size()
// cv::fisheye::undistortImage(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2922
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
/// ![image](https://docs.opencv.org/3.4.20/fisheye_undistorted.jpg)
///
/// ## C++ default parameters
/// * knew: cv::noArray()
/// * new_size: Size()
// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, const Size &)(InputArray, OutputArray, InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2922
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
/// * undistorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// ## Note
/// This alternative version of [fisheye_undistort_points] function uses the following default values for its arguments:
/// * r: noArray()
/// * p: noArray()
// cv::fisheye::undistortPoints(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2874
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
/// * undistorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// ## C++ default parameters
/// * r: noArray()
/// * p: noArray()
// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2874
// ("cv::fisheye::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "R", "P"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn fisheye_undistort_points(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(r);
	input_array_arg!(p);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_fisheye_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
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
// cv::getOptimalNewCameraMatrix(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1772
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
// getOptimalNewCameraMatrix(InputArray, InputArray, Size, double, Size, Rect *, bool)(InputArray, InputArray, SimpleClass, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1772
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

/// computes valid disparity ROI from the valid ROIs of the rectified images (that are returned by cv::stereoRectify())
// getValidDisparityROI(Rect, Rect, int, int, int)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2321
// ("cv::getValidDisparityROI", vec![(pred!(mut, ["roi1", "roi2", "minDisparity", "numberOfDisparities", "blockSize"], ["cv::Rect", "cv::Rect", "int", "int", "int"]), _)]),
#[inline]
pub fn get_valid_disparity_roi(roi1: core::Rect, roi2: core::Rect, min_disparity: i32, number_of_disparities: i32, block_size: i32) -> Result<core::Rect> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getValidDisparityROI_Rect_Rect_int_int_int(&roi1, &roi2, min_disparity, number_of_disparities, block_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Finds an initial camera intrinsic matrix from 3D-2D point correspondences.
///
/// ## Parameters
/// * objectPoints: Vector of vectors of the calibration pattern points in the calibration pattern
/// coordinate space. In the old interface all the per-view vectors are concatenated. See
/// calibrateCamera for details.
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
// cv::initCameraMatrix2D(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1093
// ("cv::initCameraMatrix2D", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
#[inline]
pub fn init_camera_matrix_2d_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size) -> Result<core::Mat> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Finds an initial camera intrinsic matrix from 3D-2D point correspondences.
///
/// ## Parameters
/// * objectPoints: Vector of vectors of the calibration pattern points in the calibration pattern
/// coordinate space. In the old interface all the per-view vectors are concatenated. See
/// calibrateCamera for details.
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
// initCameraMatrix2D(InputArrayOfArrays, InputArrayOfArrays, Size, double)(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1093
// ("cv::initCameraMatrix2D", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "aspectRatio"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "double"]), _)]),
#[inline]
pub fn init_camera_matrix_2d(object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, aspect_ratio: f64) -> Result<core::Mat> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_initCameraMatrix2D_const__InputArrayR_const__InputArrayR_Size_double(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, aspect_ratio, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
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
// matMulDeriv(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:701
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
// cv::projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:773
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
// projectPoints(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:773
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
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03).
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
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2165
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
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03).
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
// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2165
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
// cv::recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2228
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
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03).
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
// recoverPose(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, double, InputOutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2228
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
// cv::recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2198
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
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Nister03).
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
// recoverPose(InputArray, InputArray, InputArray, OutputArray, OutputArray, double, Point2d, InputOutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, SimpleClass, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2198
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

/// computes the rectification transformations for 3-head camera, where all the heads are on the same line.
// rectify3Collinear(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArrayOfArrays, InputArrayOfArrays, Size, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, double, Size, Rect *, Rect *, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1735
// ("cv::rectify3Collinear", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "cameraMatrix3", "distCoeffs3", "imgpt1", "imgpt3", "imageSize", "R12", "T12", "R13", "T13", "R1", "R2", "R3", "P1", "P2", "P3", "Q", "alpha", "newImgSize", "roi1", "roi2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "cv::Size", "cv::Rect*", "cv::Rect*", "int"]), _)]),
#[inline]
pub fn rectify3_collinear(camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, camera_matrix3: &impl ToInputArray, dist_coeffs3: &impl ToInputArray, imgpt1: &impl ToInputArray, imgpt3: &impl ToInputArray, image_size: core::Size, r12: &impl ToInputArray, t12: &impl ToInputArray, r13: &impl ToInputArray, t13: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, r3: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, p3: &mut impl ToOutputArray, q: &mut impl ToOutputArray, alpha: f64, new_img_size: core::Size, roi1: &mut core::Rect, roi2: &mut core::Rect, flags: i32) -> Result<f32> {
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	input_array_arg!(camera_matrix3);
	input_array_arg!(dist_coeffs3);
	input_array_arg!(imgpt1);
	input_array_arg!(imgpt3);
	input_array_arg!(r12);
	input_array_arg!(t12);
	input_array_arg!(r13);
	input_array_arg!(t13);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(r3);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(p3);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rectify3Collinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_double_Size_RectX_RectX_int(camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), camera_matrix3.as_raw__InputArray(), dist_coeffs3.as_raw__InputArray(), imgpt1.as_raw__InputArray(), imgpt3.as_raw__InputArray(), &image_size, r12.as_raw__InputArray(), t12.as_raw__InputArray(), r13.as_raw__InputArray(), t13.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), r3.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), p3.as_raw__OutputArray(), q.as_raw__OutputArray(), alpha, &new_img_size, roi1, roi2, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reprojects a disparity image to 3D space.
///
/// ## Parameters
/// * disparity: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. The values of 8-bit / 16-bit signed formats are assumed to have no
/// fractional bits. If the disparity is 16-bit signed format, as computed by [StereoBM] or
/// [StereoSGBM] and maybe other algorithms, it should be divided by 16 (and scaled to float) before
/// being used here.
/// * _3dImage: Output 3-channel floating-point image of the same size as disparity. Each element of
/// _3dImage(x,y) contains 3D coordinates of the point (x,y) computed from the disparity map. If one
/// uses Q obtained by [stereoRectify], then the returned points are represented in the first
/// camera's rectified coordinate system.
/// * Q: ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) perspective transformation matrix that can be obtained with
/// [stereoRectify].
/// * handleMissingValues: Indicates, whether the function should handle missing values (i.e.
/// points where the disparity was not computed). If handleMissingValues=true, then pixels with the
/// minimal disparity that corresponds to the outliers (see StereoMatcher::compute ) are transformed
/// to 3D points with a very large Z value (currently set to 10000).
/// * ddepth: The optional output array depth. If it is -1, the output image will have CV_32F
/// depth. ddepth can also be set to CV_16S, CV_32S or CV_32F.
///
/// The function transforms a single-channel disparity map to a 3-channel image representing a 3D
/// surface. That is, for each pixel (x,y) and the corresponding disparity d=disparity(x,y) , it
/// computes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%20%5C%5C%0AW%0A%5Cend%7Bbmatrix%7D%20%3D%20Q%20%5Cbegin%7Bbmatrix%7D%0Ax%20%5C%5C%0Ay%20%5C%5C%0A%5Ctexttt%7Bdisparity%7D%20%28x%2Cy%29%20%5C%5C%0Az%0A%5Cend%7Bbmatrix%7D%2E)
/// ## See also
/// To reproject a sparse set of points {(x,y,d),...} to 3D space, use perspectiveTransform.
///
/// ## Note
/// This alternative version of [reproject_image_to_3d] function uses the following default values for its arguments:
/// * handle_missing_values: false
/// * ddepth: -1
// cv::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2369
// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn reproject_image_to_3d_def(disparity: &impl ToInputArray, _3d_image: &mut impl ToOutputArray, q: &impl ToInputArray) -> Result<()> {
	input_array_arg!(disparity);
	output_array_arg!(_3d_image);
	input_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(disparity.as_raw__InputArray(), _3d_image.as_raw__OutputArray(), q.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reprojects a disparity image to 3D space.
///
/// ## Parameters
/// * disparity: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. The values of 8-bit / 16-bit signed formats are assumed to have no
/// fractional bits. If the disparity is 16-bit signed format, as computed by [StereoBM] or
/// [StereoSGBM] and maybe other algorithms, it should be divided by 16 (and scaled to float) before
/// being used here.
/// * _3dImage: Output 3-channel floating-point image of the same size as disparity. Each element of
/// _3dImage(x,y) contains 3D coordinates of the point (x,y) computed from the disparity map. If one
/// uses Q obtained by [stereoRectify], then the returned points are represented in the first
/// camera's rectified coordinate system.
/// * Q: ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) perspective transformation matrix that can be obtained with
/// [stereoRectify].
/// * handleMissingValues: Indicates, whether the function should handle missing values (i.e.
/// points where the disparity was not computed). If handleMissingValues=true, then pixels with the
/// minimal disparity that corresponds to the outliers (see StereoMatcher::compute ) are transformed
/// to 3D points with a very large Z value (currently set to 10000).
/// * ddepth: The optional output array depth. If it is -1, the output image will have CV_32F
/// depth. ddepth can also be set to CV_16S, CV_32S or CV_32F.
///
/// The function transforms a single-channel disparity map to a 3-channel image representing a 3D
/// surface. That is, for each pixel (x,y) and the corresponding disparity d=disparity(x,y) , it
/// computes:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%20%5C%5C%0AW%0A%5Cend%7Bbmatrix%7D%20%3D%20Q%20%5Cbegin%7Bbmatrix%7D%0Ax%20%5C%5C%0Ay%20%5C%5C%0A%5Ctexttt%7Bdisparity%7D%20%28x%2Cy%29%20%5C%5C%0Az%0A%5Cend%7Bbmatrix%7D%2E)
/// ## See also
/// To reproject a sparse set of points {(x,y,d),...} to 3D space, use perspectiveTransform.
///
/// ## C++ default parameters
/// * handle_missing_values: false
/// * ddepth: -1
// reprojectImageTo3D(InputArray, OutputArray, InputArray, bool, int)(InputArray, OutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2369
// ("cv::reprojectImageTo3D", vec![(pred!(mut, ["disparity", "_3dImage", "Q", "handleMissingValues", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "bool", "int"]), _)]),
#[inline]
pub fn reproject_image_to_3d(disparity: &impl ToInputArray, _3d_image: &mut impl ToOutputArray, q: &impl ToInputArray, handle_missing_values: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(disparity);
	output_array_arg!(_3d_image);
	input_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_int(disparity.as_raw__InputArray(), _3d_image.as_raw__OutputArray(), q.as_raw__InputArray(), handle_missing_values, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates the Sampson Distance between two points.
///
/// The function cv::sampsonDistance calculates and returns the first order approximation of the geometric error as:
/// ![block formula](https://latex.codecogs.com/png.latex?%0Asd%28%20%5Ctexttt%7Bpt1%7D%20%2C%20%5Ctexttt%7Bpt2%7D%20%29%3D%0A%5Cfrac%7B%28%5Ctexttt%7Bpt2%7D%5Et%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%5E2%7D%0A%7B%28%28%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%280%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpt1%7D%29%281%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%5Et%20%5Ccdot%20%5Ctexttt%7Bpt2%7D%29%280%29%29%5E2%20%2B%0A%28%28%5Ctexttt%7BF%7D%5Et%20%5Ccdot%20%5Ctexttt%7Bpt2%7D%29%281%29%29%5E2%7D%0A)
/// The fundamental matrix may be calculated using the cv::findFundamentalMat function. See [HartleyZ00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.4.3 for details.
/// ## Parameters
/// * pt1: first homogeneous 2d point
/// * pt2: second homogeneous 2d point
/// * F: fundamental matrix
/// ## Returns
/// The computed Sampson distance.
// sampsonDistance(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2391
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
/// "Complete Solution Classification for the Perspective-Three-Point Problem" ([gao2003complete](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_gao2003complete)).
/// *   [SOLVEPNP_AP3P] Method is based on the paper of T. Ke and S. Roumeliotis.
/// "An Efficient Algebraic Solution to the Perspective-Three-Point Problem" ([Ke17](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Ke17)).
///
/// The function estimates the object pose given 3 object points, their corresponding image
/// projections, as well as the camera intrinsic matrix and the distortion coefficients.
///
///
/// Note:
/// The solutions are sorted by reprojection errors (lowest to highest).
// solveP3P(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:933
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
///        modules/calib3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of cv::undistortPoints (around line 75 of modules/calib3d/src/solvepnp.cpp version 2.4.9)
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
// cv::solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1071
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
///        modules/calib3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of cv::undistortPoints (around line 75 of modules/calib3d/src/solvepnp.cpp version 2.4.9)
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
// solvePnPGeneric(InputArray, InputArray, InputArray, InputArray, OutputArrayOfArrays, OutputArrayOfArrays, bool, SolvePnPMethod, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Enum, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1071
// ("cv::solvePnPGeneric", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "useExtrinsicGuess", "flags", "rvec", "tvec", "reprojectionError"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::SolvePnPMethod", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn solve_pnp_generic(object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, use_extrinsic_guess: bool, flags: crate::calib3d::SolvePnPMethod, rvec: &impl ToInputArray, tvec: &impl ToInputArray, reprojection_error: &mut impl ToOutputArray) -> Result<i32> {
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
	unsafe { sys::cv_solvePnPGeneric_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_SolvePnPMethod_const__InputArrayR_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), use_extrinsic_guess, flags, rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), reprojection_error.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
///        opencv_source_code/samples/cpp/tutorial_code/calib3d/real_time_pose_estimation/
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
// cv::solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:899
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
///        opencv_source_code/samples/cpp/tutorial_code/calib3d/real_time_pose_estimation/
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
// solvePnPRansac(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int, float, double, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:899
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
/// to a Levenberg-Marquardt iterative minimization [Madsen04](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Eade13) process.
///
/// ## Note
/// This alternative version of [solve_pnp_refine_lm] function uses the following default values for its arguments:
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
// cv::solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:962
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
/// to a Levenberg-Marquardt iterative minimization [Madsen04](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Eade13) process.
///
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
// solvePnPRefineLM(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:962
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
/// virtual visual servoing (VVS) [Chaumette06](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Chaumette06) [Marchand16](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Marchand16) scheme.
///
/// ## Note
/// This alternative version of [solve_pnp_refine_vvs] function uses the following default values for its arguments:
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
/// * vv_slambda: 1
// cv::solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:993
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
/// virtual visual servoing (VVS) [Chaumette06](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Chaumette06) [Marchand16](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Marchand16) scheme.
///
/// ## C++ default parameters
/// * criteria: TermCriteria(TermCriteria::EPS+TermCriteria::COUNT,20,FLT_EPSILON)
/// * vv_slambda: 1
// solvePnPRefineVVS(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, TermCriteria, double)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:993
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
///        modules/calib3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of cv::undistortPoints (around line 75 of modules/calib3d/src/solvepnp.cpp version 2.4.9)
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
// cv::solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:851
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
///        modules/calib3d/src/solvepnp.cpp version 2.4.9)
///        - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
///        to its calling of cv::undistortPoints (around line 75 of modules/calib3d/src/solvepnp.cpp version 2.4.9)
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
// solvePnP(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:851
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
/// Similarly to calibrateCamera, the function minimizes the total re-projection error for all the
/// points in all the available views from both cameras. The function returns the final value of the
/// re-projection error.
///
/// ## Note
/// This alternative version of [stereo_calibrate_extended] function uses the following default values for its arguments:
/// * flags: CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,1e-6)
// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1555
// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_calibrate_extended_def(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// Similarly to calibrateCamera, the function minimizes the total re-projection error for all the
/// points in all the available views from both cameras. The function returns the final value of the
/// re-projection error.
///
/// ## C++ default parameters
/// * flags: CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,1e-6)
// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1555
// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn stereo_calibrate_extended(object_points: &impl ToInputArray, image_points1: &impl ToInputArray, image_points2: &impl ToInputArray, camera_matrix1: &mut impl ToInputOutputArray, dist_coeffs1: &mut impl ToInputOutputArray, camera_matrix2: &mut impl ToInputOutputArray, dist_coeffs2: &mut impl ToInputOutputArray, image_size: core::Size, r: &mut impl ToInputOutputArray, t: &mut impl ToInputOutputArray, e: &mut impl ToOutputArray, f: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [stereo_calibrate] function uses the following default values for its arguments:
/// * flags: CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,1e-6)
// cv::stereoCalibrate(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1564
// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
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
/// Similarly to calibrateCamera, the function minimizes the total re-projection error for all the
/// points in all the available views from both cameras. The function returns the final value of the
/// re-projection error.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * flags: CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,1e-6)
// stereoCalibrate(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, Size, OutputArray, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1564
// ("cv::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "E", "F", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
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
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes a rectification transform for an uncalibrated stereo camera.
///
/// ## Parameters
/// * points1: Array of feature points in the first image.
/// * points2: The corresponding points in the second image. The same formats as in
/// [find_fundamental_mat] are supported.
/// * F: Input fundamental matrix. It can be computed from the same set of point pairs using
/// [find_fundamental_mat] .
/// * imgSize: Size of the image.
/// * H1: Output rectification homography matrix for the first image.
/// * H2: Output rectification homography matrix for the second image.
/// * threshold: Optional threshold used to filter out the outliers. If the parameter is greater
/// than zero, all the point pairs that do not comply with the epipolar geometry (that is, the points
/// for which ![inline formula](https://latex.codecogs.com/png.latex?%7C%5Ctexttt%7Bpoints2%5Bi%5D%7D%5ET%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpoints1%5Bi%5D%7D%7C%3E%5Ctexttt%7Bthreshold%7D) )
/// are rejected prior to computing the homographies. Otherwise, all the points are considered inliers.
///
/// The function computes the rectification transformations without knowing intrinsic parameters of the
/// cameras and their relative position in the space, which explains the suffix "uncalibrated". Another
/// related difference from [stereo_rectify] is that the function outputs not the rectification
/// transformations in the object (3D) space, but the planar perspective transformations encoded by the
/// homography matrices H1 and H2 . The function implements the algorithm [Hartley99](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Hartley99) .
///
///
/// Note:
///    While the algorithm does not need to know the intrinsic parameters of the cameras, it heavily
///    depends on the epipolar geometry. Therefore, if the camera lenses have a significant distortion,
///    it would be better to correct it before computing the fundamental matrix and calling this
///    function. For example, distortion coefficients can be estimated for each head of stereo camera
///    separately by using [calibrate_camera] . Then, the images can be corrected using [undistort] , or
///    just the point coordinates can be corrected with [undistort_points] .
///
/// ## Note
/// This alternative version of [stereo_rectify_uncalibrated] function uses the following default values for its arguments:
/// * threshold: 5
// cv::stereoRectifyUncalibrated(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1729
// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_rectify_uncalibrated_def(points1: &impl ToInputArray, points2: &impl ToInputArray, f: &impl ToInputArray, img_size: core::Size, h1: &mut impl ToOutputArray, h2: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(f);
	output_array_arg!(h1);
	output_array_arg!(h2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR(points1.as_raw__InputArray(), points2.as_raw__InputArray(), f.as_raw__InputArray(), &img_size, h1.as_raw__OutputArray(), h2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes a rectification transform for an uncalibrated stereo camera.
///
/// ## Parameters
/// * points1: Array of feature points in the first image.
/// * points2: The corresponding points in the second image. The same formats as in
/// [find_fundamental_mat] are supported.
/// * F: Input fundamental matrix. It can be computed from the same set of point pairs using
/// [find_fundamental_mat] .
/// * imgSize: Size of the image.
/// * H1: Output rectification homography matrix for the first image.
/// * H2: Output rectification homography matrix for the second image.
/// * threshold: Optional threshold used to filter out the outliers. If the parameter is greater
/// than zero, all the point pairs that do not comply with the epipolar geometry (that is, the points
/// for which ![inline formula](https://latex.codecogs.com/png.latex?%7C%5Ctexttt%7Bpoints2%5Bi%5D%7D%5ET%20%5Ccdot%20%5Ctexttt%7BF%7D%20%5Ccdot%20%5Ctexttt%7Bpoints1%5Bi%5D%7D%7C%3E%5Ctexttt%7Bthreshold%7D) )
/// are rejected prior to computing the homographies. Otherwise, all the points are considered inliers.
///
/// The function computes the rectification transformations without knowing intrinsic parameters of the
/// cameras and their relative position in the space, which explains the suffix "uncalibrated". Another
/// related difference from [stereo_rectify] is that the function outputs not the rectification
/// transformations in the object (3D) space, but the planar perspective transformations encoded by the
/// homography matrices H1 and H2 . The function implements the algorithm [Hartley99](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Hartley99) .
///
///
/// Note:
///    While the algorithm does not need to know the intrinsic parameters of the cameras, it heavily
///    depends on the epipolar geometry. Therefore, if the camera lenses have a significant distortion,
///    it would be better to correct it before computing the fundamental matrix and calling this
///    function. For example, distortion coefficients can be estimated for each head of stereo camera
///    separately by using [calibrate_camera] . Then, the images can be corrected using [undistort] , or
///    just the point coordinates can be corrected with [undistort_points] .
///
/// ## C++ default parameters
/// * threshold: 5
// stereoRectifyUncalibrated(InputArray, InputArray, InputArray, Size, OutputArray, OutputArray, double)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1729
// ("cv::stereoRectifyUncalibrated", vec![(pred!(mut, ["points1", "points2", "F", "imgSize", "H1", "H2", "threshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn stereo_rectify_uncalibrated(points1: &impl ToInputArray, points2: &impl ToInputArray, f: &impl ToInputArray, img_size: core::Size, h1: &mut impl ToOutputArray, h2: &mut impl ToOutputArray, threshold: f64) -> Result<bool> {
	input_array_arg!(points1);
	input_array_arg!(points2);
	input_array_arg!(f);
	output_array_arg!(h1);
	output_array_arg!(h2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectifyUncalibrated_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__OutputArrayR_const__OutputArrayR_double(points1.as_raw__InputArray(), points2.as_raw__InputArray(), f.as_raw__InputArray(), &img_size, h1.as_raw__OutputArray(), h2.as_raw__OutputArray(), threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes rectification transforms for each head of a calibrated stereo camera.
///
/// ## Parameters
/// * cameraMatrix1: First camera intrinsic matrix.
/// * distCoeffs1: First camera distortion parameters.
/// * cameraMatrix2: Second camera intrinsic matrix.
/// * distCoeffs2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * T: Translation vector from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera. This matrix
/// brings points given in the unrectified first camera's coordinate system to points in the rectified
/// first camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified first camera's coordinate system to the rectified first camera's coordinate system.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera. This matrix
/// brings points given in the unrectified second camera's coordinate system to points in the rectified
/// second camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified second camera's coordinate system to the rectified second camera's coordinate system.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified first camera's image.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified second camera's image.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see [reprojectImageTo3D]).
/// * flags: Operation flags that may be zero or [CALIB_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * alpha: Free scaling parameter. If it is -1 or absent, the function performs the default
/// scaling. Otherwise, the parameter should be between 0 and 1. alpha=0 means that the rectified
/// images are zoomed and shifted so that only valid pixels are visible (no black areas after
/// rectification). alpha=1 means that the rectified image is decimated and shifted so that all the
/// pixels from the original images from the cameras are retained in the rectified images (no source
/// image pixels are lost). Any intermediate value yields an intermediate result between
/// those two extreme cases.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to a larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * validPixROI1: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
/// * validPixROI2: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
///
/// The function computes the rotation matrices for each camera that (virtually) make both camera image
/// planes the same plane. Consequently, this makes all the epipolar lines parallel and thus simplifies
/// the dense stereo correspondence problem. The function takes the matrices computed by [stereo_calibrate]
/// as input. As output, it provides two rotation matrices and also two projection matrices in the new
/// coordinates. The function distinguishes the following two cases:
///
/// *   **Horizontal stereo**: the first and the second camera views are shifted relative to each other
///    mainly along the x-axis (with possible small vertical shift). In the rectified images, the
///    corresponding epipolar lines in the left and right cameras are horizontal and have the same
///    y-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F2%20%26%20T%5Fx%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fx%7D%20%26%20%5Cfrac%7Bcx%5F1%20%2D%20cx%5F2%7D%7BT%5Fx%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fx) is a horizontal shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cx%5F1%3Dcx%5F2) if
///    [CALIB_ZERO_DISPARITY] is set.
///
/// *   **Vertical stereo**: the first and the second camera views are shifted relative to each other
///    mainly in the vertical direction (and probably a bit in the horizontal direction too). The epipolar
///    lines in the rectified images are vertical and have the same x-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F2%20%26%20T%5Fy%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fy%7D%20%26%20%5Cfrac%7Bcy%5F1%20%2D%20cy%5F2%7D%7BT%5Fy%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fy) is a vertical shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cy%5F1%3Dcy%5F2) if
///    [CALIB_ZERO_DISPARITY] is set.
///
/// As you can see, the first three columns of P1 and P2 will effectively be the new "rectified" camera
/// matrices. The matrices, together with R1 and R2 , can then be passed to [init_undistort_rectify_map] to
/// initialize the rectification map for each camera.
///
/// See below the screenshot from the stereo_calib.cpp sample. Some red horizontal lines pass through
/// the corresponding image regions. This means that the images are well rectified, which is what most
/// stereo correspondence algorithms rely on. The green rectangles are roi1 and roi2 . You see that
/// their interiors are all valid pixels.
///
/// ![image](https://docs.opencv.org/3.4.20/stereo_undistort.jpg)
///
/// ## Note
/// This alternative version of [stereo_rectify] function uses the following default values for its arguments:
/// * flags: CALIB_ZERO_DISPARITY
/// * alpha: -1
/// * new_image_size: Size()
/// * valid_pix_roi1: 0
/// * valid_pix_roi2: 0
// cv::stereoRectify(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1691
// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_rectify_def(camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, t: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes rectification transforms for each head of a calibrated stereo camera.
///
/// ## Parameters
/// * cameraMatrix1: First camera intrinsic matrix.
/// * distCoeffs1: First camera distortion parameters.
/// * cameraMatrix2: Second camera intrinsic matrix.
/// * distCoeffs2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * T: Translation vector from the coordinate system of the first camera to the second camera,
/// see [stereoCalibrate].
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera. This matrix
/// brings points given in the unrectified first camera's coordinate system to points in the rectified
/// first camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified first camera's coordinate system to the rectified first camera's coordinate system.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera. This matrix
/// brings points given in the unrectified second camera's coordinate system to points in the rectified
/// second camera's coordinate system. In more technical terms, it performs a change of basis from the
/// unrectified second camera's coordinate system to the rectified second camera's coordinate system.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified first camera's image.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera, i.e. it projects points given in the rectified first camera coordinate system into the
/// rectified second camera's image.
/// * Q: Output ![inline formula](https://latex.codecogs.com/png.latex?4%20%5Ctimes%204) disparity-to-depth mapping matrix (see [reprojectImageTo3D]).
/// * flags: Operation flags that may be zero or [CALIB_ZERO_DISPARITY] . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * alpha: Free scaling parameter. If it is -1 or absent, the function performs the default
/// scaling. Otherwise, the parameter should be between 0 and 1. alpha=0 means that the rectified
/// images are zoomed and shifted so that only valid pixels are visible (no black areas after
/// rectification). alpha=1 means that the rectified image is decimated and shifted so that all the
/// pixels from the original images from the cameras are retained in the rectified images (no source
/// image pixels are lost). Any intermediate value yields an intermediate result between
/// those two extreme cases.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// [init_undistort_rectify_map] (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to a larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * validPixROI1: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
/// * validPixROI2: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
///
/// The function computes the rotation matrices for each camera that (virtually) make both camera image
/// planes the same plane. Consequently, this makes all the epipolar lines parallel and thus simplifies
/// the dense stereo correspondence problem. The function takes the matrices computed by [stereo_calibrate]
/// as input. As output, it provides two rotation matrices and also two projection matrices in the new
/// coordinates. The function distinguishes the following two cases:
///
/// *   **Horizontal stereo**: the first and the second camera views are shifted relative to each other
///    mainly along the x-axis (with possible small vertical shift). In the rectified images, the
///    corresponding epipolar lines in the left and right cameras are horizontal and have the same
///    y-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%5F2%20%26%20T%5Fx%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fx%7D%20%26%20%5Cfrac%7Bcx%5F1%20%2D%20cx%5F2%7D%7BT%5Fx%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fx) is a horizontal shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cx%5F1%3Dcx%5F2) if
///    [CALIB_ZERO_DISPARITY] is set.
///
/// *   **Vertical stereo**: the first and the second camera views are shifted relative to each other
///    mainly in the vertical direction (and probably a bit in the horizontal direction too). The epipolar
///    lines in the rectified images are vertical and have the same x-coordinate. P1 and P2 look like:
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F1%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BP2%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%20%26%200%20%26%20cx%20%26%200%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%20%26%20cy%5F2%20%26%20T%5Fy%20%5Ccdot%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%201%20%26%200%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%2C)
///
///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BQ%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%201%20%26%200%20%26%200%20%26%20%2Dcx%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%201%20%26%200%20%26%20%2Dcy%5F1%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%200%20%26%20f%20%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%200%20%26%20%2D%5Cfrac%7B1%7D%7BT%5Fy%7D%20%26%20%5Cfrac%7Bcy%5F1%20%2D%20cy%5F2%7D%7BT%5Fy%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%20)
///
///    where ![inline formula](https://latex.codecogs.com/png.latex?T%5Fy) is a vertical shift between the cameras and ![inline formula](https://latex.codecogs.com/png.latex?cy%5F1%3Dcy%5F2) if
///    [CALIB_ZERO_DISPARITY] is set.
///
/// As you can see, the first three columns of P1 and P2 will effectively be the new "rectified" camera
/// matrices. The matrices, together with R1 and R2 , can then be passed to [init_undistort_rectify_map] to
/// initialize the rectification map for each camera.
///
/// See below the screenshot from the stereo_calib.cpp sample. Some red horizontal lines pass through
/// the corresponding image regions. This means that the images are well rectified, which is what most
/// stereo correspondence algorithms rely on. The green rectangles are roi1 and roi2 . You see that
/// their interiors are all valid pixels.
///
/// ![image](https://docs.opencv.org/3.4.20/stereo_undistort.jpg)
///
/// ## C++ default parameters
/// * flags: CALIB_ZERO_DISPARITY
/// * alpha: -1
/// * new_image_size: Size()
/// * valid_pix_roi1: 0
/// * valid_pix_roi2: 0
// stereoRectify(InputArray, InputArray, InputArray, InputArray, Size, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, int, double, Size, Rect *, Rect *)(InputArray, InputArray, InputArray, InputArray, SimpleClass, InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1691
// ("cv::stereoRectify", vec![(pred!(mut, ["cameraMatrix1", "distCoeffs1", "cameraMatrix2", "distCoeffs2", "imageSize", "R", "T", "R1", "R2", "P1", "P2", "Q", "flags", "alpha", "newImageSize", "validPixROI1", "validPixROI2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "double", "cv::Size", "cv::Rect*", "cv::Rect*"]), _)]),
#[inline]
pub fn stereo_rectify(camera_matrix1: &impl ToInputArray, dist_coeffs1: &impl ToInputArray, camera_matrix2: &impl ToInputArray, dist_coeffs2: &impl ToInputArray, image_size: core::Size, r: &impl ToInputArray, t: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray, q: &mut impl ToOutputArray, flags: i32, alpha: f64, new_image_size: core::Size, valid_pix_roi1: &mut core::Rect, valid_pix_roi2: &mut core::Rect) -> Result<()> {
	input_array_arg!(camera_matrix1);
	input_array_arg!(dist_coeffs1);
	input_array_arg!(camera_matrix2);
	input_array_arg!(dist_coeffs2);
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(r1);
	output_array_arg!(r2);
	output_array_arg!(p1);
	output_array_arg!(p2);
	output_array_arg!(q);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereoRectify_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_double_Size_RectX_RectX(camera_matrix1.as_raw__InputArray(), dist_coeffs1.as_raw__InputArray(), camera_matrix2.as_raw__InputArray(), dist_coeffs2.as_raw__InputArray(), &image_size, r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), q.as_raw__OutputArray(), flags, alpha, &new_image_size, valid_pix_roi1, valid_pix_roi2, ocvrs_return.as_mut_ptr()) };
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
// triangulatePoints(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2282
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

/// validates disparity using the left-right check. The matrix "cost" should be computed by the stereo correspondence algorithm
///
/// ## Note
/// This alternative version of [validate_disparity] function uses the following default values for its arguments:
/// * disp12_max_disp: 1
// cv::validateDisparity(InputOutputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2326
// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
#[inline]
pub fn validate_disparity_def(disparity: &mut impl ToInputOutputArray, cost: &impl ToInputArray, min_disparity: i32, number_of_disparities: i32) -> Result<()> {
	input_output_array_arg!(disparity);
	input_array_arg!(cost);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int(disparity.as_raw__InputOutputArray(), cost.as_raw__InputArray(), min_disparity, number_of_disparities, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// validates disparity using the left-right check. The matrix "cost" should be computed by the stereo correspondence algorithm
///
/// ## C++ default parameters
/// * disp12_max_disp: 1
// validateDisparity(InputOutputArray, InputArray, int, int, int)(InputOutputArray, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2326
// ("cv::validateDisparity", vec![(pred!(mut, ["disparity", "cost", "minDisparity", "numberOfDisparities", "disp12MaxDisp"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
#[inline]
pub fn validate_disparity(disparity: &mut impl ToInputOutputArray, cost: &impl ToInputArray, min_disparity: i32, number_of_disparities: i32, disp12_max_disp: i32) -> Result<()> {
	input_output_array_arg!(disparity);
	input_array_arg!(cost);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_validateDisparity_const__InputOutputArrayR_const__InputArrayR_int_int_int(disparity.as_raw__InputOutputArray(), cost.as_raw__InputArray(), min_disparity, number_of_disparities, disp12_max_disp, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::calib3d::CirclesGridFinderParameters]
// CirclesGridFinderParameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1186
pub trait CirclesGridFinderParametersTraitConst {
	fn as_raw_CirclesGridFinderParameters(&self) -> *const c_void;

	// cv::CirclesGridFinderParameters::densityNeighborhoodSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1189
	// ("cv::CirclesGridFinderParameters::densityNeighborhoodSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn density_neighborhood_size(&self) -> core::Size2f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CirclesGridFinderParameters_propDensityNeighborhoodSize_const(self.as_raw_CirclesGridFinderParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::CirclesGridFinderParameters::minDensity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1190
	// ("cv::CirclesGridFinderParameters::minDensity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_density(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinDensity_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::kmeansAttempts() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1191
	// ("cv::CirclesGridFinderParameters::kmeansAttempts", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kmeans_attempts(&self) -> i32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propKmeansAttempts_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::minDistanceToAddKeypoint() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1192
	// ("cv::CirclesGridFinderParameters::minDistanceToAddKeypoint", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_distance_to_add_keypoint(&self) -> i32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinDistanceToAddKeypoint_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::keypointScale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1193
	// ("cv::CirclesGridFinderParameters::keypointScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn keypoint_scale(&self) -> i32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propKeypointScale_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::minGraphConfidence() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1194
	// ("cv::CirclesGridFinderParameters::minGraphConfidence", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_graph_confidence(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinGraphConfidence_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::vertexGain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1195
	// ("cv::CirclesGridFinderParameters::vertexGain", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn vertex_gain(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propVertexGain_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::vertexPenalty() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1196
	// ("cv::CirclesGridFinderParameters::vertexPenalty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn vertex_penalty(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propVertexPenalty_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::existingVertexGain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1197
	// ("cv::CirclesGridFinderParameters::existingVertexGain", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn existing_vertex_gain(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propExistingVertexGain_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::edgeGain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1198
	// ("cv::CirclesGridFinderParameters::edgeGain", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn edge_gain(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propEdgeGain_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::edgePenalty() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1199
	// ("cv::CirclesGridFinderParameters::edgePenalty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn edge_penalty(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propEdgePenalty_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::convexHullFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1200
	// ("cv::CirclesGridFinderParameters::convexHullFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn convex_hull_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propConvexHullFactor_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::minRNGEdgeSwitchDist() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1201
	// ("cv::CirclesGridFinderParameters::minRNGEdgeSwitchDist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_rng_edge_switch_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinRNGEdgeSwitchDist_const(self.as_raw_CirclesGridFinderParameters()) };
		ret
	}

	// cv::CirclesGridFinderParameters::gridType() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1207
	// ("cv::CirclesGridFinderParameters::gridType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn grid_type(&self) -> crate::calib3d::CirclesGridFinderParameters_GridType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CirclesGridFinderParameters_propGridType_const(self.as_raw_CirclesGridFinderParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

}

/// Mutable methods for [crate::calib3d::CirclesGridFinderParameters]
pub trait CirclesGridFinderParametersTrait: crate::calib3d::CirclesGridFinderParametersTraitConst {
	fn as_raw_mut_CirclesGridFinderParameters(&mut self) -> *mut c_void;

	// cv::CirclesGridFinderParameters::setDensityNeighborhoodSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1189
	// ("cv::CirclesGridFinderParameters::setDensityNeighborhoodSize", vec![(pred!(mut, ["val"], ["const cv::Size2f"]), _)]),
	#[inline]
	fn set_density_neighborhood_size(&mut self, val: core::Size2f) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propDensityNeighborhoodSize_const_Size2f(self.as_raw_mut_CirclesGridFinderParameters(), &val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setMinDensity(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1190
	// ("cv::CirclesGridFinderParameters::setMinDensity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_min_density(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinDensity_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setKmeansAttempts(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1191
	// ("cv::CirclesGridFinderParameters::setKmeansAttempts", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_kmeans_attempts(&mut self, val: i32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propKmeansAttempts_const_int(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setMinDistanceToAddKeypoint(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1192
	// ("cv::CirclesGridFinderParameters::setMinDistanceToAddKeypoint", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_distance_to_add_keypoint(&mut self, val: i32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinDistanceToAddKeypoint_const_int(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setKeypointScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1193
	// ("cv::CirclesGridFinderParameters::setKeypointScale", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_keypoint_scale(&mut self, val: i32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propKeypointScale_const_int(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setMinGraphConfidence(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1194
	// ("cv::CirclesGridFinderParameters::setMinGraphConfidence", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_min_graph_confidence(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinGraphConfidence_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setVertexGain(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1195
	// ("cv::CirclesGridFinderParameters::setVertexGain", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_vertex_gain(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propVertexGain_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setVertexPenalty(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1196
	// ("cv::CirclesGridFinderParameters::setVertexPenalty", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_vertex_penalty(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propVertexPenalty_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setExistingVertexGain(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1197
	// ("cv::CirclesGridFinderParameters::setExistingVertexGain", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_existing_vertex_gain(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propExistingVertexGain_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setEdgeGain(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1198
	// ("cv::CirclesGridFinderParameters::setEdgeGain", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_edge_gain(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propEdgeGain_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setEdgePenalty(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1199
	// ("cv::CirclesGridFinderParameters::setEdgePenalty", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_edge_penalty(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propEdgePenalty_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setConvexHullFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1200
	// ("cv::CirclesGridFinderParameters::setConvexHullFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_convex_hull_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propConvexHullFactor_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setMinRNGEdgeSwitchDist(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1201
	// ("cv::CirclesGridFinderParameters::setMinRNGEdgeSwitchDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_min_rng_edge_switch_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propMinRNGEdgeSwitchDist_const_float(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

	// cv::CirclesGridFinderParameters::setGridType(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1207
	// ("cv::CirclesGridFinderParameters::setGridType", vec![(pred!(mut, ["val"], ["const cv::CirclesGridFinderParameters::GridType"]), _)]),
	#[inline]
	fn set_grid_type(&mut self, val: crate::calib3d::CirclesGridFinderParameters_GridType) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters_propGridType_const_GridType(self.as_raw_mut_CirclesGridFinderParameters(), val) };
		ret
	}

}

// CirclesGridFinderParameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1186
pub struct CirclesGridFinderParameters {
	ptr: *mut c_void,
}

opencv_type_boxed! { CirclesGridFinderParameters }

impl Drop for CirclesGridFinderParameters {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CirclesGridFinderParameters_delete(self.as_raw_mut_CirclesGridFinderParameters()) };
	}
}

unsafe impl Send for CirclesGridFinderParameters {}

impl crate::calib3d::CirclesGridFinderParametersTraitConst for CirclesGridFinderParameters {
	#[inline] fn as_raw_CirclesGridFinderParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::CirclesGridFinderParametersTrait for CirclesGridFinderParameters {
	#[inline] fn as_raw_mut_CirclesGridFinderParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CirclesGridFinderParameters, crate::calib3d::CirclesGridFinderParametersTraitConst, as_raw_CirclesGridFinderParameters, crate::calib3d::CirclesGridFinderParametersTrait, as_raw_mut_CirclesGridFinderParameters }

impl CirclesGridFinderParameters {
	// CirclesGridFinderParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1188
	// ("cv::CirclesGridFinderParameters::CirclesGridFinderParameters", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::calib3d::CirclesGridFinderParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CirclesGridFinderParameters_CirclesGridFinderParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::calib3d::CirclesGridFinderParameters::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for CirclesGridFinderParameters {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_CirclesGridFinderParameters_implicitClone_const(self.as_raw_CirclesGridFinderParameters())) }
	}
}

impl std::fmt::Debug for CirclesGridFinderParameters {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CirclesGridFinderParameters")
			.field("density_neighborhood_size", &crate::calib3d::CirclesGridFinderParametersTraitConst::density_neighborhood_size(self))
			.field("min_density", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_density(self))
			.field("kmeans_attempts", &crate::calib3d::CirclesGridFinderParametersTraitConst::kmeans_attempts(self))
			.field("min_distance_to_add_keypoint", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_distance_to_add_keypoint(self))
			.field("keypoint_scale", &crate::calib3d::CirclesGridFinderParametersTraitConst::keypoint_scale(self))
			.field("min_graph_confidence", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_graph_confidence(self))
			.field("vertex_gain", &crate::calib3d::CirclesGridFinderParametersTraitConst::vertex_gain(self))
			.field("vertex_penalty", &crate::calib3d::CirclesGridFinderParametersTraitConst::vertex_penalty(self))
			.field("existing_vertex_gain", &crate::calib3d::CirclesGridFinderParametersTraitConst::existing_vertex_gain(self))
			.field("edge_gain", &crate::calib3d::CirclesGridFinderParametersTraitConst::edge_gain(self))
			.field("edge_penalty", &crate::calib3d::CirclesGridFinderParametersTraitConst::edge_penalty(self))
			.field("convex_hull_factor", &crate::calib3d::CirclesGridFinderParametersTraitConst::convex_hull_factor(self))
			.field("min_rng_edge_switch_dist", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_rng_edge_switch_dist(self))
			.field("grid_type", &crate::calib3d::CirclesGridFinderParametersTraitConst::grid_type(self))
			.finish()
	}
}

/// Constant methods for [crate::calib3d::CirclesGridFinderParameters2]
// CirclesGridFinderParameters2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1210
pub trait CirclesGridFinderParameters2TraitConst: crate::calib3d::CirclesGridFinderParametersTraitConst {
	fn as_raw_CirclesGridFinderParameters2(&self) -> *const c_void;

	/// Distance between two adjacent points. Used by CALIB_CB_CLUSTERING.
	// cv::CirclesGridFinderParameters2::squareSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1214
	// ("cv::CirclesGridFinderParameters2::squareSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn square_size(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters2_propSquareSize_const(self.as_raw_CirclesGridFinderParameters2()) };
		ret
	}

	/// Max deviation from prediction. Used by CALIB_CB_CLUSTERING.
	// cv::CirclesGridFinderParameters2::maxRectifiedDistance() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1215
	// ("cv::CirclesGridFinderParameters2::maxRectifiedDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_rectified_distance(&self) -> f32 {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters2_propMaxRectifiedDistance_const(self.as_raw_CirclesGridFinderParameters2()) };
		ret
	}

}

/// Mutable methods for [crate::calib3d::CirclesGridFinderParameters2]
pub trait CirclesGridFinderParameters2Trait: crate::calib3d::CirclesGridFinderParameters2TraitConst + crate::calib3d::CirclesGridFinderParametersTrait {
	fn as_raw_mut_CirclesGridFinderParameters2(&mut self) -> *mut c_void;

	/// Distance between two adjacent points. Used by CALIB_CB_CLUSTERING.
	// cv::CirclesGridFinderParameters2::setSquareSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1214
	// ("cv::CirclesGridFinderParameters2::setSquareSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_square_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters2_propSquareSize_const_float(self.as_raw_mut_CirclesGridFinderParameters2(), val) };
		ret
	}

	/// Max deviation from prediction. Used by CALIB_CB_CLUSTERING.
	// cv::CirclesGridFinderParameters2::setMaxRectifiedDistance(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1215
	// ("cv::CirclesGridFinderParameters2::setMaxRectifiedDistance", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_max_rectified_distance(&mut self, val: f32) {
		let ret = unsafe { sys::cv_CirclesGridFinderParameters2_propMaxRectifiedDistance_const_float(self.as_raw_mut_CirclesGridFinderParameters2(), val) };
		ret
	}

}

// CirclesGridFinderParameters2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1210
pub struct CirclesGridFinderParameters2 {
	ptr: *mut c_void,
}

opencv_type_boxed! { CirclesGridFinderParameters2 }

impl Drop for CirclesGridFinderParameters2 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CirclesGridFinderParameters2_delete(self.as_raw_mut_CirclesGridFinderParameters2()) };
	}
}

unsafe impl Send for CirclesGridFinderParameters2 {}

impl crate::calib3d::CirclesGridFinderParametersTraitConst for CirclesGridFinderParameters2 {
	#[inline] fn as_raw_CirclesGridFinderParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::CirclesGridFinderParametersTrait for CirclesGridFinderParameters2 {
	#[inline] fn as_raw_mut_CirclesGridFinderParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CirclesGridFinderParameters2, crate::calib3d::CirclesGridFinderParametersTraitConst, as_raw_CirclesGridFinderParameters, crate::calib3d::CirclesGridFinderParametersTrait, as_raw_mut_CirclesGridFinderParameters }

impl crate::calib3d::CirclesGridFinderParameters2TraitConst for CirclesGridFinderParameters2 {
	#[inline] fn as_raw_CirclesGridFinderParameters2(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::CirclesGridFinderParameters2Trait for CirclesGridFinderParameters2 {
	#[inline] fn as_raw_mut_CirclesGridFinderParameters2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CirclesGridFinderParameters2, crate::calib3d::CirclesGridFinderParameters2TraitConst, as_raw_CirclesGridFinderParameters2, crate::calib3d::CirclesGridFinderParameters2Trait, as_raw_mut_CirclesGridFinderParameters2 }

impl CirclesGridFinderParameters2 {
	// CirclesGridFinderParameters2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:1212
	// ("cv::CirclesGridFinderParameters2::CirclesGridFinderParameters2", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::calib3d::CirclesGridFinderParameters2> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CirclesGridFinderParameters2_CirclesGridFinderParameters2(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::calib3d::CirclesGridFinderParameters2::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for CirclesGridFinderParameters2 {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_CirclesGridFinderParameters2_implicitClone_const(self.as_raw_CirclesGridFinderParameters2())) }
	}
}

boxed_cast_base! { CirclesGridFinderParameters2, crate::calib3d::CirclesGridFinderParameters, cv_CirclesGridFinderParameters2_to_CirclesGridFinderParameters }

impl std::fmt::Debug for CirclesGridFinderParameters2 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CirclesGridFinderParameters2")
			.field("square_size", &crate::calib3d::CirclesGridFinderParameters2TraitConst::square_size(self))
			.field("max_rectified_distance", &crate::calib3d::CirclesGridFinderParameters2TraitConst::max_rectified_distance(self))
			.field("density_neighborhood_size", &crate::calib3d::CirclesGridFinderParametersTraitConst::density_neighborhood_size(self))
			.field("min_density", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_density(self))
			.field("kmeans_attempts", &crate::calib3d::CirclesGridFinderParametersTraitConst::kmeans_attempts(self))
			.field("min_distance_to_add_keypoint", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_distance_to_add_keypoint(self))
			.field("keypoint_scale", &crate::calib3d::CirclesGridFinderParametersTraitConst::keypoint_scale(self))
			.field("min_graph_confidence", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_graph_confidence(self))
			.field("vertex_gain", &crate::calib3d::CirclesGridFinderParametersTraitConst::vertex_gain(self))
			.field("vertex_penalty", &crate::calib3d::CirclesGridFinderParametersTraitConst::vertex_penalty(self))
			.field("existing_vertex_gain", &crate::calib3d::CirclesGridFinderParametersTraitConst::existing_vertex_gain(self))
			.field("edge_gain", &crate::calib3d::CirclesGridFinderParametersTraitConst::edge_gain(self))
			.field("edge_penalty", &crate::calib3d::CirclesGridFinderParametersTraitConst::edge_penalty(self))
			.field("convex_hull_factor", &crate::calib3d::CirclesGridFinderParametersTraitConst::convex_hull_factor(self))
			.field("min_rng_edge_switch_dist", &crate::calib3d::CirclesGridFinderParametersTraitConst::min_rng_edge_switch_dist(self))
			.field("grid_type", &crate::calib3d::CirclesGridFinderParametersTraitConst::grid_type(self))
			.finish()
	}
}

/// Constant methods for [crate::calib3d::StereoBM]
// StereoBM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2662
pub trait StereoBMTraitConst: crate::calib3d::StereoMatcherTraitConst {
	fn as_raw_StereoBM(&self) -> *const c_void;

	// getPreFilterType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2669
	// ("cv::StereoBM::getPreFilterType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterType_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPreFilterSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2672
	// ("cv::StereoBM::getPreFilterSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterSize_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2675
	// ("cv::StereoBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_cap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterCap_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTextureThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2678
	// ("cv::StereoBM::getTextureThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_texture_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getTextureThreshold_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2681
	// ("cv::StereoBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_uniqueness_ratio(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getUniquenessRatio_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSmallerBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2684
	// ("cv::StereoBM::getSmallerBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_smaller_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getSmallerBlockSize_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getROI1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2687
	// ("cv::StereoBM::getROI1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_roi1(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getROI1_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getROI2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2690
	// ("cv::StereoBM::getROI2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_roi2(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getROI2_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::calib3d::StereoBM]
pub trait StereoBMTrait: crate::calib3d::StereoBMTraitConst + crate::calib3d::StereoMatcherTrait {
	fn as_raw_mut_StereoBM(&mut self) -> *mut c_void;

	// setPreFilterType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2670
	// ("cv::StereoBM::setPreFilterType", vec![(pred!(mut, ["preFilterType"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_type(&mut self, pre_filter_type: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterType_int(self.as_raw_mut_StereoBM(), pre_filter_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPreFilterSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2673
	// ("cv::StereoBM::setPreFilterSize", vec![(pred!(mut, ["preFilterSize"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_size(&mut self, pre_filter_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterSize_int(self.as_raw_mut_StereoBM(), pre_filter_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2676
	// ("cv::StereoBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterCap_int(self.as_raw_mut_StereoBM(), pre_filter_cap, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTextureThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2679
	// ("cv::StereoBM::setTextureThreshold", vec![(pred!(mut, ["textureThreshold"], ["int"]), _)]),
	#[inline]
	fn set_texture_threshold(&mut self, texture_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setTextureThreshold_int(self.as_raw_mut_StereoBM(), texture_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2682
	// ("cv::StereoBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	#[inline]
	fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setUniquenessRatio_int(self.as_raw_mut_StereoBM(), uniqueness_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSmallerBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2685
	// ("cv::StereoBM::setSmallerBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	#[inline]
	fn set_smaller_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setSmallerBlockSize_int(self.as_raw_mut_StereoBM(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setROI1(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2688
	// ("cv::StereoBM::setROI1", vec![(pred!(mut, ["roi1"], ["cv::Rect"]), _)]),
	#[inline]
	fn set_roi1(&mut self, roi1: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setROI1_Rect(self.as_raw_mut_StereoBM(), &roi1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setROI2(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2691
	// ("cv::StereoBM::setROI2", vec![(pred!(mut, ["roi2"], ["cv::Rect"]), _)]),
	#[inline]
	fn set_roi2(&mut self, roi2: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setROI2_Rect(self.as_raw_mut_StereoBM(), &roi2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for computing stereo correspondence using the block matching algorithm, introduced and
/// contributed to OpenCV by K. Konolige.
// StereoBM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2662
pub struct StereoBM {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereoBM }

impl Drop for StereoBM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoBM_delete(self.as_raw_mut_StereoBM()) };
	}
}

unsafe impl Send for StereoBM {}

impl core::AlgorithmTraitConst for StereoBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoBM, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::calib3d::StereoMatcherTraitConst for StereoBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for StereoBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoBM, crate::calib3d::StereoMatcherTraitConst, as_raw_StereoMatcher, crate::calib3d::StereoMatcherTrait, as_raw_mut_StereoMatcher }

impl crate::calib3d::StereoBMTraitConst for StereoBM {
	#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoBMTrait for StereoBM {
	#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoBM, crate::calib3d::StereoBMTraitConst, as_raw_StereoBM, crate::calib3d::StereoBMTrait, as_raw_mut_StereoBM }

impl StereoBM {
	/// Creates StereoBM object
	///
	/// ## Parameters
	/// * numDisparities: the disparity search range. For each pixel algorithm will find the best
	/// disparity from 0 (default minimum disparity) to numDisparities. The search range can then be
	/// shifted by changing the minimum disparity.
	/// * blockSize: the linear size of the blocks compared by the algorithm. The size should be odd
	/// (as the block is centered at the current pixel). Larger block size implies smoother, though less
	/// accurate disparity map. Smaller block size gives more detailed disparity map, but there is higher
	/// chance for algorithm to find a wrong correspondence.
	///
	/// The function create StereoBM object. You can then call StereoBM::compute() to compute disparity for
	/// a specific stereo pair.
	///
	/// ## C++ default parameters
	/// * num_disparities: 0
	/// * block_size: 21
	// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2706
	// ("cv::StereoBM::create", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
	#[inline]
	pub fn create(num_disparities: i32, block_size: i32) -> Result<core::Ptr<crate::calib3d::StereoBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_create_int_int(num_disparities, block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates StereoBM object
	///
	/// ## Parameters
	/// * numDisparities: the disparity search range. For each pixel algorithm will find the best
	/// disparity from 0 (default minimum disparity) to numDisparities. The search range can then be
	/// shifted by changing the minimum disparity.
	/// * blockSize: the linear size of the blocks compared by the algorithm. The size should be odd
	/// (as the block is centered at the current pixel). Larger block size implies smoother, though less
	/// accurate disparity map. Smaller block size gives more detailed disparity map, but there is higher
	/// chance for algorithm to find a wrong correspondence.
	///
	/// The function create StereoBM object. You can then call StereoBM::compute() to compute disparity for
	/// a specific stereo pair.
	///
	/// ## Note
	/// This alternative version of [StereoBM::create] function uses the following default values for its arguments:
	/// * num_disparities: 0
	/// * block_size: 21
	// cv::StereoBM::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2706
	// ("cv::StereoBM::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::calib3d::StereoBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StereoBM, core::Algorithm, cv_StereoBM_to_Algorithm }

boxed_cast_base! { StereoBM, crate::calib3d::StereoMatcher, cv_StereoBM_to_StereoMatcher }

impl std::fmt::Debug for StereoBM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoBM")
			.finish()
	}
}

/// Constant methods for [crate::calib3d::StereoMatcher]
// StereoMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2621
pub trait StereoMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_StereoMatcher(&self) -> *const c_void;

	// getMinDisparity()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2639
	// ("cv::StereoMatcher::getMinDisparity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_disparity(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getMinDisparity_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2642
	// ("cv::StereoMatcher::getNumDisparities", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_disparities(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getNumDisparities_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2645
	// ("cv::StereoMatcher::getBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getBlockSize_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSpeckleWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2648
	// ("cv::StereoMatcher::getSpeckleWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_speckle_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getSpeckleWindowSize_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSpeckleRange()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2651
	// ("cv::StereoMatcher::getSpeckleRange", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_speckle_range(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getSpeckleRange_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDisp12MaxDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2654
	// ("cv::StereoMatcher::getDisp12MaxDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_disp12_max_diff(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getDisp12MaxDiff_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::calib3d::StereoMatcher]
pub trait StereoMatcherTrait: core::AlgorithmTrait + crate::calib3d::StereoMatcherTraitConst {
	fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void;

	/// Computes disparity map for the specified stereo pair
	///
	/// ## Parameters
	/// * left: Left 8-bit single-channel image.
	/// * right: Right image of the same size and the same type as the left one.
	/// * disparity: Output disparity map. It has the same size as the input images. Some algorithms,
	/// like StereoBM or StereoSGBM compute 16-bit fixed-point disparity map (where each disparity value
	/// has 4 fractional bits), whereas other algorithms output 32-bit floating-point disparity map.
	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2636
	// ("cv::StereoMatcher::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute(&mut self, left: &impl ToInputArray, right: &impl ToInputArray, disparity: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(left);
		input_array_arg!(right);
		output_array_arg!(disparity);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StereoMatcher(), left.as_raw__InputArray(), right.as_raw__InputArray(), disparity.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDisparity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2640
	// ("cv::StereoMatcher::setMinDisparity", vec![(pred!(mut, ["minDisparity"], ["int"]), _)]),
	#[inline]
	fn set_min_disparity(&mut self, min_disparity: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setMinDisparity_int(self.as_raw_mut_StereoMatcher(), min_disparity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2643
	// ("cv::StereoMatcher::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
	#[inline]
	fn set_num_disparities(&mut self, num_disparities: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setNumDisparities_int(self.as_raw_mut_StereoMatcher(), num_disparities, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2646
	// ("cv::StereoMatcher::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	#[inline]
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setBlockSize_int(self.as_raw_mut_StereoMatcher(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSpeckleWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2649
	// ("cv::StereoMatcher::setSpeckleWindowSize", vec![(pred!(mut, ["speckleWindowSize"], ["int"]), _)]),
	#[inline]
	fn set_speckle_window_size(&mut self, speckle_window_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setSpeckleWindowSize_int(self.as_raw_mut_StereoMatcher(), speckle_window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSpeckleRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2652
	// ("cv::StereoMatcher::setSpeckleRange", vec![(pred!(mut, ["speckleRange"], ["int"]), _)]),
	#[inline]
	fn set_speckle_range(&mut self, speckle_range: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setSpeckleRange_int(self.as_raw_mut_StereoMatcher(), speckle_range, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDisp12MaxDiff(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2655
	// ("cv::StereoMatcher::setDisp12MaxDiff", vec![(pred!(mut, ["disp12MaxDiff"], ["int"]), _)]),
	#[inline]
	fn set_disp12_max_diff(&mut self, disp12_max_diff: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setDisp12MaxDiff_int(self.as_raw_mut_StereoMatcher(), disp12_max_diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// The base class for stereo correspondence algorithms.
// StereoMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2621
pub struct StereoMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereoMatcher }

impl Drop for StereoMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoMatcher_delete(self.as_raw_mut_StereoMatcher()) };
	}
}

unsafe impl Send for StereoMatcher {}

impl core::AlgorithmTraitConst for StereoMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::calib3d::StereoMatcherTraitConst for StereoMatcher {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for StereoMatcher {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoMatcher, crate::calib3d::StereoMatcherTraitConst, as_raw_StereoMatcher, crate::calib3d::StereoMatcherTrait, as_raw_mut_StereoMatcher }

impl StereoMatcher {
}

boxed_cast_descendant! { StereoMatcher, crate::calib3d::StereoBM, cv_StereoMatcher_to_StereoBM }

boxed_cast_descendant! { StereoMatcher, crate::calib3d::StereoSGBM, cv_StereoMatcher_to_StereoSGBM }

boxed_cast_base! { StereoMatcher, core::Algorithm, cv_StereoMatcher_to_Algorithm }

impl std::fmt::Debug for StereoMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoMatcher")
			.finish()
	}
}

/// Constant methods for [crate::calib3d::StereoSGBM]
// StereoSGBM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2727
pub trait StereoSGBMTraitConst: crate::calib3d::StereoMatcherTraitConst {
	fn as_raw_StereoSGBM(&self) -> *const c_void;

	// getPreFilterCap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2738
	// ("cv::StereoSGBM::getPreFilterCap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pre_filter_cap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getPreFilterCap_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUniquenessRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2741
	// ("cv::StereoSGBM::getUniquenessRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_uniqueness_ratio(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getUniquenessRatio_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getP1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2744
	// ("cv::StereoSGBM::getP1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_p1(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getP1_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getP2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2747
	// ("cv::StereoSGBM::getP2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_p2(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getP2_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2750
	// ("cv::StereoSGBM::getMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_mode(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getMode_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::calib3d::StereoSGBM]
pub trait StereoSGBMTrait: crate::calib3d::StereoMatcherTrait + crate::calib3d::StereoSGBMTraitConst {
	fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void;

	// setPreFilterCap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2739
	// ("cv::StereoSGBM::setPreFilterCap", vec![(pred!(mut, ["preFilterCap"], ["int"]), _)]),
	#[inline]
	fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setPreFilterCap_int(self.as_raw_mut_StereoSGBM(), pre_filter_cap, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUniquenessRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2742
	// ("cv::StereoSGBM::setUniquenessRatio", vec![(pred!(mut, ["uniquenessRatio"], ["int"]), _)]),
	#[inline]
	fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setUniquenessRatio_int(self.as_raw_mut_StereoSGBM(), uniqueness_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setP1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2745
	// ("cv::StereoSGBM::setP1", vec![(pred!(mut, ["P1"], ["int"]), _)]),
	#[inline]
	fn set_p1(&mut self, p1: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setP1_int(self.as_raw_mut_StereoSGBM(), p1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setP2(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2748
	// ("cv::StereoSGBM::setP2", vec![(pred!(mut, ["P2"], ["int"]), _)]),
	#[inline]
	fn set_p2(&mut self, p2: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setP2_int(self.as_raw_mut_StereoSGBM(), p2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2751
	// ("cv::StereoSGBM::setMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	#[inline]
	fn set_mode(&mut self, mode: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setMode_int(self.as_raw_mut_StereoSGBM(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// The class implements the modified H. Hirschmuller algorithm [HH08](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_HH08) that differs from the original
/// one as follows:
///
/// *   By default, the algorithm is single-pass, which means that you consider only 5 directions
/// instead of 8. Set mode=StereoSGBM::MODE_HH in createStereoSGBM to run the full variant of the
/// algorithm but beware that it may consume a lot of memory.
/// *   The algorithm matches blocks, not individual pixels. Though, setting blockSize=1 reduces the
/// blocks to single pixels.
/// *   Mutual information cost function is not implemented. Instead, a simpler Birchfield-Tomasi
/// sub-pixel metric from [BT98](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BT98) is used. Though, the color images are supported as well.
/// *   Some pre- and post- processing steps from K. Konolige algorithm StereoBM are included, for
/// example: pre-filtering (StereoBM::PREFILTER_XSOBEL type) and post-filtering (uniqueness
/// check, quadratic interpolation and speckle filtering).
///
///
/// Note:
///    *   (Python) An example illustrating the use of the StereoSGBM matching algorithm can be found
///        at opencv_source_code/samples/python/stereo_match.py
// StereoSGBM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2727
pub struct StereoSGBM {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereoSGBM }

impl Drop for StereoSGBM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoSGBM_delete(self.as_raw_mut_StereoSGBM()) };
	}
}

unsafe impl Send for StereoSGBM {}

impl core::AlgorithmTraitConst for StereoSGBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoSGBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoSGBM, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::calib3d::StereoMatcherTraitConst for StereoSGBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for StereoSGBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoSGBM, crate::calib3d::StereoMatcherTraitConst, as_raw_StereoMatcher, crate::calib3d::StereoMatcherTrait, as_raw_mut_StereoMatcher }

impl crate::calib3d::StereoSGBMTraitConst for StereoSGBM {
	#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoSGBMTrait for StereoSGBM {
	#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereoSGBM, crate::calib3d::StereoSGBMTraitConst, as_raw_StereoSGBM, crate::calib3d::StereoSGBMTrait, as_raw_mut_StereoSGBM }

impl StereoSGBM {
	/// Creates StereoSGBM object
	///
	/// ## Parameters
	/// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
	/// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
	/// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
	/// zero. In the current implementation, this parameter must be divisible by 16.
	/// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
	/// somewhere in the 3..11 range.
	/// * P1: The first parameter controlling the disparity smoothness. See below.
	/// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
	/// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
	/// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
	/// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
	/// P1 and P2 values are shown (like 8\*number_of_image_channels\*blockSize\*blockSize and
	/// 32\*number_of_image_channels\*blockSize\*blockSize , respectively).
	/// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
	/// disparity check. Set it to a non-positive value to disable the check.
	/// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
	/// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
	/// The result values are passed to the Birchfield-Tomasi pixel cost function.
	/// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
	/// value should "win" the second best value to consider the found match correct. Normally, a value
	/// within the 5-15 range is good enough.
	/// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
	/// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
	/// 50-200 range.
	/// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
	/// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
	/// Normally, 1 or 2 is good enough.
	/// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
	/// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
	/// huge for HD-size pictures. By default, it is set to false .
	///
	/// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
	/// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
	/// to a custom value.
	///
	/// ## C++ default parameters
	/// * min_disparity: 0
	/// * num_disparities: 16
	/// * block_size: 3
	/// * p1: 0
	/// * p2: 0
	/// * disp12_max_diff: 0
	/// * pre_filter_cap: 0
	/// * uniqueness_ratio: 0
	/// * speckle_window_size: 0
	/// * speckle_range: 0
	/// * mode: StereoSGBM::MODE_SGBM
	// create(int, int, int, int, int, int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2790
	// ("cv::StereoSGBM::create", vec![(pred!(mut, ["minDisparity", "numDisparities", "blockSize", "P1", "P2", "disp12MaxDiff", "preFilterCap", "uniquenessRatio", "speckleWindowSize", "speckleRange", "mode"], ["int", "int", "int", "int", "int", "int", "int", "int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn create(min_disparity: i32, num_disparities: i32, block_size: i32, p1: i32, p2: i32, disp12_max_diff: i32, pre_filter_cap: i32, uniqueness_ratio: i32, speckle_window_size: i32, speckle_range: i32, mode: i32) -> Result<core::Ptr<crate::calib3d::StereoSGBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(min_disparity, num_disparities, block_size, p1, p2, disp12_max_diff, pre_filter_cap, uniqueness_ratio, speckle_window_size, speckle_range, mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoSGBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates StereoSGBM object
	///
	/// ## Parameters
	/// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
	/// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
	/// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
	/// zero. In the current implementation, this parameter must be divisible by 16.
	/// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
	/// somewhere in the 3..11 range.
	/// * P1: The first parameter controlling the disparity smoothness. See below.
	/// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
	/// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
	/// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
	/// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
	/// P1 and P2 values are shown (like 8\*number_of_image_channels\*blockSize\*blockSize and
	/// 32\*number_of_image_channels\*blockSize\*blockSize , respectively).
	/// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
	/// disparity check. Set it to a non-positive value to disable the check.
	/// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
	/// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
	/// The result values are passed to the Birchfield-Tomasi pixel cost function.
	/// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
	/// value should "win" the second best value to consider the found match correct. Normally, a value
	/// within the 5-15 range is good enough.
	/// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
	/// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
	/// 50-200 range.
	/// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
	/// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
	/// Normally, 1 or 2 is good enough.
	/// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
	/// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
	/// huge for HD-size pictures. By default, it is set to false .
	///
	/// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
	/// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
	/// to a custom value.
	///
	/// ## Note
	/// This alternative version of [StereoSGBM::create] function uses the following default values for its arguments:
	/// * min_disparity: 0
	/// * num_disparities: 16
	/// * block_size: 3
	/// * p1: 0
	/// * p2: 0
	/// * disp12_max_diff: 0
	/// * pre_filter_cap: 0
	/// * uniqueness_ratio: 0
	/// * speckle_window_size: 0
	/// * speckle_range: 0
	/// * mode: StereoSGBM::MODE_SGBM
	// cv::StereoSGBM::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/calib3d.hpp:2790
	// ("cv::StereoSGBM::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::calib3d::StereoSGBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoSGBM>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StereoSGBM, core::Algorithm, cv_StereoSGBM_to_Algorithm }

boxed_cast_base! { StereoSGBM, crate::calib3d::StereoMatcher, cv_StereoSGBM_to_StereoMatcher }

impl std::fmt::Debug for StereoSGBM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoSGBM")
			.finish()
	}
}
