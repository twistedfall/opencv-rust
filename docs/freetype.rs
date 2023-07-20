pub mod freetype {
	//! # Drawing UTF-8 strings with freetype/harfbuzz
	//! 
	//! This modules is to draw UTF-8 strings with freetype/harfbuzz.
	//! 
	//! 1. Install freetype2 and harfbuzz in your system.
	//! 2. Create FreeType2 instance with createFreeType2() function.
	//! 3. Load font file with loadFontData() function.
	//! 4. Draw text with putText() function.
	//! 
	//! - If thickness parameter is negative, drawing glyph is filled.
	//! - If thickness parameter is positive, drawing glyph is outlined with thickness.
	//! - If line_type parameter is 16(or CV_AA), drawing glyph is smooth.
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::FreeType2TraitConst, super::FreeType2Trait };
	}
	
	/// Create FreeType2 Instance
	/// 
	/// The function createFreeType2 create instance to draw UTF-8 strings.
	#[inline]
	pub fn create_free_type2() -> Result<core::Ptr<crate::freetype::FreeType2>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_freetype_createFreeType2(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::freetype::FreeType2>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Constant methods for [crate::freetype::FreeType2]
	pub trait FreeType2TraitConst: core::AlgorithmTraitConst {
		fn as_raw_FreeType2(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::freetype::FreeType2]
	pub trait FreeType2Trait: core::AlgorithmTrait + crate::freetype::FreeType2TraitConst {
		fn as_raw_mut_FreeType2(&mut self) -> *mut c_void;
	
		/// Load font data.
		/// 
		/// The function loadFontData loads font data.
		/// 
		/// ## Parameters
		/// * fontFileName: FontFile Name
		/// * idx: face_index to select a font faces in a single file.
		#[inline]
		fn load_font_data(&mut self, font_file_name: &str, idx: i32) -> Result<()> {
			extern_container_arg!(mut font_file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_freetype_FreeType2_loadFontData_String_int(self.as_raw_mut_FreeType2(), font_file_name.opencv_as_extern_mut(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set Split Number from Bezier-curve to line
		/// 
		/// The function setSplitNumber set the number of split points from bezier-curve to line.
		/// If you want to draw large glyph, large is better.
		/// If you want to draw small glyph, small is better.
		/// 
		/// ## Parameters
		/// * num: number of split points from bezier-curve to line
		#[inline]
		fn set_split_number(&mut self, num: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_freetype_FreeType2_setSplitNumber_int(self.as_raw_mut_FreeType2(), num, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Draws a text string.
		/// 
		/// The function putText renders the specified text string in the image. Symbols that cannot be rendered using the specified font are replaced by "Tofu" or non-drawn.
		/// 
		/// ## Parameters
		/// * img: Image. (Only 8UC1/8UC3/8UC4 2D mat is supported.)
		/// * text: Text string to be drawn.
		/// * org: Bottom-left/Top-left corner of the text string in the image.
		/// * fontHeight: Drawing font size by pixel unit.
		/// * color: Text color.
		/// * thickness: Thickness of the lines used to draw a text when negative, the glyph is filled. Otherwise, the glyph is drawn with this thickness.
		/// * line_type: Line type. See the line for details.
		/// * bottomLeftOrigin: When true, the image data origin is at the bottom-left corner. Otherwise, it is at the top-left corner.
		#[inline]
		fn put_text(&mut self, img: &mut impl core::ToInputOutputArray, text: &str, org: core::Point, font_height: i32, color: core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool) -> Result<()> {
			input_output_array_arg!(img);
			extern_container_arg!(text);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_freetype_FreeType2_putText_const__InputOutputArrayR_const_StringR_Point_int_Scalar_int_int_bool(self.as_raw_mut_FreeType2(), img.as_raw__InputOutputArray(), text.opencv_as_extern(), org.opencv_as_extern(), font_height, color.opencv_as_extern(), thickness, line_type, bottom_left_origin, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Calculates the width and height of a text string.
		/// 
		/// The function getTextSize calculates and returns the approximate size of a box that contains the specified text.
		/// That is, the following code renders some text, the tight box surrounding it, and the baseline: :
		/// ```C++
		///    String text = "Funny text inside the box";
		///    int fontHeight = 60;
		///    int thickness = -1;
		///    int linestyle = LINE_8;
		/// 
		///    Mat img(600, 800, CV_8UC3, Scalar::all(0));
		/// 
		///    int baseline=0;
		/// 
		///    cv::Ptr<cv::freetype::FreeType2> ft2;
		///    ft2 = cv::freetype::createFreeType2();
		///    ft2->loadFontData( "./mplus-1p-regular.ttf", 0 );
		/// 
		///    Size textSize = ft2->getTextSize(text,
		///                                      fontHeight,
		///                                      thickness,
		///                                      &baseline);
		/// 
		///    if(thickness > 0){
		///        baseline += thickness;
		///    }
		/// 
		///    // center the text
		///    Point textOrg((img.cols - textSize.width) / 2,
		///                   (img.rows + textSize.height) / 2);
		/// 
		///    // draw the box
		///    rectangle(img, textOrg + Point(0, baseline),
		///               textOrg + Point(textSize.width, -textSize.height),
		///               Scalar(0,255,0),1,8);
		/// 
		///    // ... and the baseline first
		///    line(img, textOrg + Point(0, thickness),
		///          textOrg + Point(textSize.width, thickness),
		///          Scalar(0, 0, 255),1,8);
		/// 
		///    // then put the text itself
		///    ft2->putText(img, text, textOrg, fontHeight,
		///                  Scalar::all(255), thickness, linestyle, true );
		/// ```
		/// 
		/// 
		/// ## Parameters
		/// * text: Input text string.
		/// * fontHeight: Drawing font size by pixel unit.
		/// * thickness: Thickness of lines used to render the text. See putText for details.
		/// * baseLine:[out] y-coordinate of the baseline relative to the bottom-most text
		/// point.
		/// ## Returns
		/// The size of a box that contains the specified text.
		/// ## See also
		/// cv::putText
		#[inline]
		fn get_text_size(&mut self, text: &str, font_height: i32, thickness: i32, base_line: &mut i32) -> Result<core::Size> {
			extern_container_arg!(text);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_freetype_FreeType2_getTextSize_const_StringR_int_int_intX(self.as_raw_mut_FreeType2(), text.opencv_as_extern(), font_height, thickness, base_line, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct FreeType2 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FreeType2 }
	
	impl Drop for FreeType2 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_freetype_FreeType2_delete(self.as_raw_mut_FreeType2()) };
		}
	}
	
	unsafe impl Send for FreeType2 {}
	
	impl core::AlgorithmTraitConst for FreeType2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FreeType2 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::freetype::FreeType2TraitConst for FreeType2 {
		#[inline] fn as_raw_FreeType2(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::freetype::FreeType2Trait for FreeType2 {
		#[inline] fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FreeType2 {
	}
	
	boxed_cast_base! { FreeType2, core::Algorithm, cv_freetype_FreeType2_to_Algorithm }
	
	impl std::fmt::Debug for FreeType2 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FreeType2")
				.finish()
		}
	}
}
