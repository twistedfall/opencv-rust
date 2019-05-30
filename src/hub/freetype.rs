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
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};


/// Create FreeType2 Instance
/// 
/// The function createFreeType2 create instance to draw UTF-8 strings.
pub fn create_free_type2() -> Result<types::PtrOfFreeType2> {
    unsafe { sys::cv_freetype_createFreeType2() }.into_result().map(|ptr| types::PtrOfFreeType2 { ptr })
}

// Generating impl for trait cv::freetype::FreeType2 (trait)
pub trait FreeType2: core::Algorithm {
    #[inline(always)] fn as_raw_FreeType2(&self) -> *mut c_void;
    /// Load font data.
    /// 
    /// The function loadFontData loads font data.
    /// 
    /// ## Parameters
    /// * fontFileName: FontFile Name
    /// * id: face_index to select a font faces in a single file.
    fn load_font_data(&mut self, font_file_name: &str, id: i32) -> Result<()> {
        string_arg!(mut font_file_name);
        unsafe { sys::cv_freetype_FreeType2_loadFontData_String_int(self.as_raw_FreeType2(), font_file_name.as_ptr() as _, id) }.into_result()
    }
    
    /// Set Split Number from Bezier-curve to line
    /// 
    /// The function setSplitNumber set the number of split points from bezier-curve to line.
    /// If you want to draw large glyph, large is better.
    /// If you want to draw small glyph, small is better.
    /// 
    /// ## Parameters
    /// * num: number of split points from bezier-curve to line
    fn set_split_number(&mut self, num: i32) -> Result<()> {
        unsafe { sys::cv_freetype_FreeType2_setSplitNumber_int(self.as_raw_FreeType2(), num) }.into_result()
    }
    
    /// Draws a text string.
    /// 
    /// The function putText renders the specified text string in the image. Symbols that cannot be rendered using the specified font are replaced by "Tofu" or non-drawn.
    /// 
    /// ## Parameters
    /// * img: Image.
    /// * text: Text string to be drawn.
    /// * org: Bottom-left/Top-left corner of the text string in the image.
    /// * fontHeight: Drawing font size by pixel unit.
    /// * color: Text color.
    /// * thickness: Thickness of the lines used to draw a text when negative, the glyph is filled. Otherwise, the glyph is drawn with this thickness.
    /// * line_type: Line type. See the line for details.
    /// * bottomLeftOrigin: When true, the image data origin is at the bottom-left corner. Otherwise, it is at the top-left corner.
    fn put_text(&mut self, img: &mut core::Mat, text: &str, org: core::Point, font_height: i32, color: core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool) -> Result<()> {
        string_arg!(text);
        unsafe { sys::cv_freetype_FreeType2_putText_Mat_String_Point_int_Scalar_int_int_bool(self.as_raw_FreeType2(), img.as_raw_Mat(), text.as_ptr(), org, font_height, color, thickness, line_type, bottom_left_origin) }.into_result()
    }
    
    /// Calculates the width and height of a text string.
    /// 
    /// The function getTextSize calculates and returns the approximate size of a box that contains the specified text.
    /// That is, the following code renders some text, the tight box surrounding it, and the baseline: :
    /// ```ignore
    /// String text = "Funny text inside the box";
    /// int fontHeight = 60;
    /// int thickness = -1;
    /// int linestyle = 8;
    /// 
    /// Mat img(600, 800, CV_8UC3, Scalar::all(0));
    /// 
    /// int baseline=0;
    /// 
    /// cv::Ptr<cv::freetype::FreeType2> ft2;
    /// ft2 = cv::freetype::createFreeType2();
    /// ft2->loadFontData( "./mplus-1p-regular.ttf", 0 );
    /// 
    /// Size textSize = ft2->getTextSize(text,
    /// fontHeight,
    /// thickness,
    /// &baseline);
    /// 
    /// if(thickness > 0){
    /// baseline += thickness;
    /// }
    /// 
    /// // center the text
    /// Point textOrg((img.cols - textSize.width) / 2,
    /// (img.rows + textSize.height) / 2);
    /// 
    /// // draw the box
    /// rectangle(img, textOrg + Point(0, baseline),
    /// textOrg + Point(textSize.width, -textSize.height),
    /// Scalar(0,255,0),1,8);
    /// 
    /// // ... and the baseline first
    /// line(img, textOrg + Point(0, thickness),
    /// textOrg + Point(textSize.width, thickness),
    /// Scalar(0, 0, 255),1,8);
    /// 
    /// // then put the text itself
    /// ft2->putText(img, text, textOrg, fontHeight,
    /// Scalar::all(255), thickness, linestyle, true );
    /// ```
    /// 
    /// 
    /// ## Parameters
    /// * text: Input text string.
    /// * fontHeight: Drawing font size by pixel unit.
    /// * thickness: Thickness of lines used to render the text. See putText for details.
    /// * baseLine: [out] y-coordinate of the baseline relative to the bottom-most text
    /// point.
    /// ## Returns
    /// The size of a box that contains the specified text.
    /// 
    /// @see cv::putText
    fn get_text_size(&mut self, text: &str, font_height: i32, thickness: i32, base_line: &mut i32) -> Result<core::Size> {
        string_arg!(text);
        unsafe { sys::cv_freetype_FreeType2_getTextSize_String_int_int_int_X(self.as_raw_FreeType2(), text.as_ptr(), font_height, thickness, base_line) }.into_result()
    }
    
}

impl<'a> FreeType2 + 'a {

}

