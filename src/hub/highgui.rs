//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>
//! High-level GUI
//! 
//! # High-level GUI
//! 
//! While OpenCV was designed for use in full-scale applications and can be used within functionally
//! rich UI frameworks (such as Qt\*, WinForms\*, or Cocoa\*) or without any UI at all, sometimes there
//! it is required to try functionality quickly and visualize the results. This is what the HighGUI
//! module has been designed for.
//! 
//! It provides easy interface to:
//! 
//! *   Create and manipulate windows that can display images and "remember" their content (no need to
//! handle repaint events from OS).
//! *   Add trackbars to the windows, handle simple mouse events as well as keyboard commands.
//! 
//! @{
//! OpenGL support
//! 
//! # OpenGL support
//! Qt New Functions
//! 
//! # Qt New Functions
//! 
//! ![image](pics/qtgui.png)
//! 
//! This figure explains new functionality implemented with Qt\* GUI. The new GUI provides a statusbar,
//! a toolbar, and a control panel. The control panel can have trackbars and buttonbars attached to it.
//! If you cannot see the control panel, press Ctrl+P or right-click any Qt window and select **Display
//! properties window**.
//! 
//! *   To attach a trackbar, the window name parameter must be NULL.
//! 
//! *   To attach a buttonbar, a button must be created. If the last bar attached to the control panel
//! is a buttonbar, the new button is added to the right of the last button. If the last bar
//! attached to the control panel is a trackbar, or the control panel is empty, a new buttonbar is
//! created. Then, a new button is attached to it.
//! 
//! See below the example used to generate the figure:
//! ```ignore
//! int main(int argc, char *argv[])
//! {
//! 
//! int value = 50;
//! int value2 = 0;
//! 
//! 
//! namedWindow("main1",WINDOW_NORMAL);
//! namedWindow("main2",WINDOW_AUTOSIZE | CV_GUI_NORMAL);
//! createTrackbar( "track1", "main1", &value, 255,  NULL);
//! 
//! String nameb1 = "button1";
//! String nameb2 = "button2";
//! 
//! createButton(nameb1,callbackButton,&nameb1,QT_CHECKBOX,1);
//! createButton(nameb2,callbackButton,NULL,QT_CHECKBOX,0);
//! createTrackbar( "track2", NULL, &value2, 255, NULL);
//! createButton("button5",callbackButton1,NULL,QT_RADIOBOX,0);
//! createButton("button6",callbackButton2,NULL,QT_RADIOBOX,1);
//! 
//! setMouseCallback( "main2",on_mouse,NULL );
//! 
//! Mat img1 = imread("files/flower.jpg");
//! VideoCapture video;
//! video.open("files/hockey.avi");
//! 
//! Mat img2,img3;
//! 
//! while( waitKey(33) != 27 )
//! {
//! img1.convertTo(img2,-1,1,value);
//! video >> img3;
//! 
//! imshow("main1",img2);
//! imshow("main2",img3);
//! }
//! 
//! destroyAllWindows();
//! 
//! return 0;
//! }
//! ```
//! 
//! 
//! 
//! WinRT support
//! 
//! # WinRT support
//! 
//! This figure explains new functionality implemented with WinRT GUI. The new GUI provides an Image control,
//! and a slider panel. Slider panel holds trackbars attached to it.
//! 
//! Sliders are attached below the image control. Every new slider is added below the previous one.
//! 
//! See below the example used to generate the figure:
//! ```ignore
//! void sample_app::MainPage::ShowWindow()
//! {
//! static cv::String windowName("sample");
//! cv::winrt_initContainer(this->cvContainer);
//! cv::namedWindow(windowName); // not required
//! 
//! cv::Mat image = cv::imread("Assets/sample.jpg");
//! cv::Mat converted = cv::Mat(image.rows, image.cols, CV_8UC4);
//! cv::cvtColor(image, converted, COLOR_BGR2BGRA);
//! cv::imshow(windowName, converted); // this will create window if it hasn't been created before
//! 
//! int state = 42;
//! cv::TrackbarCallback callback = [](int pos, void* userdata)
//! {
//! if (pos == 0) {
//! cv::destroyWindow(windowName);
//! }
//! };
//! cv::TrackbarCallback callbackTwin = [](int pos, void* userdata)
//! {
//! if (pos >= 70) {
//! cv::destroyAllWindows();
//! }
//! };
//! cv::createTrackbar("Sample trackbar", windowName, &state, 100, callback);
//! cv::createTrackbar("Twin brother", windowName, &state, 100, callbackTwin);
//! }
//! ```
//! 
//! 
//! C API
//! 
//! # C API
//! @}

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};
pub const CV_CHECKBOX: i32 = 1;
pub const CV_EVENT_FLAG_ALTKEY: i32 = 32;
pub const CV_EVENT_FLAG_CTRLKEY: i32 = 8;
pub const CV_EVENT_FLAG_LBUTTON: i32 = 1;
pub const CV_EVENT_FLAG_MBUTTON: i32 = 4;
pub const CV_EVENT_FLAG_RBUTTON: i32 = 2;
pub const CV_EVENT_FLAG_SHIFTKEY: i32 = 16;
pub const CV_EVENT_LBUTTONDBLCLK: i32 = 7;
pub const CV_EVENT_LBUTTONDOWN: i32 = 1;
pub const CV_EVENT_LBUTTONUP: i32 = 4;
pub const CV_EVENT_MBUTTONDBLCLK: i32 = 9;
pub const CV_EVENT_MBUTTONDOWN: i32 = 3;
pub const CV_EVENT_MBUTTONUP: i32 = 6;
pub const CV_EVENT_MOUSEHWHEEL: i32 = 11;
pub const CV_EVENT_MOUSEMOVE: i32 = 0;
pub const CV_EVENT_MOUSEWHEEL: i32 = 10;
pub const CV_EVENT_RBUTTONDBLCLK: i32 = 8;
pub const CV_EVENT_RBUTTONDOWN: i32 = 2;
pub const CV_EVENT_RBUTTONUP: i32 = 5;
pub const CV_FONT_BLACK: i32 = 87;
pub const CV_FONT_BOLD: i32 = 75;
pub const CV_FONT_DEMIBOLD: i32 = 63;
pub const CV_FONT_LIGHT: i32 = 25;
pub const CV_FONT_NORMAL: i32 = 50;
pub const CV_GUI_EXPANDED: i32 = 0x00000000;
pub const CV_GUI_NORMAL: i32 = 0x00000010;
pub const CV_PUSH_BUTTON: i32 = 0;
pub const CV_RADIOBOX: i32 = 2;
pub const CV_STYLE_ITALIC: i32 = 1;
pub const CV_STYLE_NORMAL: i32 = 0;
pub const CV_STYLE_OBLIQUE: i32 = 2;
pub const CV_WINDOW_AUTOSIZE: i32 = 0x00000001;
pub const CV_WINDOW_FREERATIO: i32 = 0x00000100;
pub const CV_WINDOW_FULLSCREEN: i32 = 1;
pub const CV_WINDOW_KEEPRATIO: i32 = 0x00000000;
pub const CV_WINDOW_NORMAL: i32 = 0x00000000;
pub const CV_WINDOW_OPENGL: i32 = 0x00001000;
pub const CV_WND_PROP_ASPECTRATIO: i32 = 2;
pub const CV_WND_PROP_AUTOSIZE: i32 = 1;
pub const CV_WND_PROP_FULLSCREEN: i32 = 0;
pub const CV_WND_PROP_OPENGL: i32 = 3;
pub const CV_WND_PROP_VISIBLE: i32 = 4;
pub const EVENT_FLAG_ALTKEY: i32 = 32;
pub const EVENT_FLAG_CTRLKEY: i32 = 8;
pub const EVENT_FLAG_LBUTTON: i32 = 1;
pub const EVENT_FLAG_MBUTTON: i32 = 4;
pub const EVENT_FLAG_RBUTTON: i32 = 2;
pub const EVENT_FLAG_SHIFTKEY: i32 = 16;
pub const EVENT_LBUTTONDBLCLK: i32 = 7;
pub const EVENT_LBUTTONDOWN: i32 = 1;
pub const EVENT_LBUTTONUP: i32 = 4;
pub const EVENT_MBUTTONDBLCLK: i32 = 9;
pub const EVENT_MBUTTONDOWN: i32 = 3;
pub const EVENT_MBUTTONUP: i32 = 6;
pub const EVENT_MOUSEHWHEEL: i32 = 11;
pub const EVENT_MOUSEMOVE: i32 = 0;
pub const EVENT_MOUSEWHEEL: i32 = 10;
pub const EVENT_RBUTTONDBLCLK: i32 = 8;
pub const EVENT_RBUTTONDOWN: i32 = 2;
pub const EVENT_RBUTTONUP: i32 = 5;
pub const QT_CHECKBOX: i32 = 1;
pub const QT_FONT_BLACK: i32 = 87;
pub const QT_FONT_BOLD: i32 = 75;
pub const QT_FONT_DEMIBOLD: i32 = 63;
pub const QT_FONT_LIGHT: i32 = 25;
pub const QT_FONT_NORMAL: i32 = 50;
pub const QT_NEW_BUTTONBAR: i32 = 1024;
pub const QT_PUSH_BUTTON: i32 = 0;
pub const QT_RADIOBOX: i32 = 2;
pub const QT_STYLE_ITALIC: i32 = 1;
pub const QT_STYLE_NORMAL: i32 = 0;
pub const QT_STYLE_OBLIQUE: i32 = 2;
pub const WINDOW_AUTOSIZE: i32 = 0x00000001;
pub const WINDOW_FREERATIO: i32 = 0x00000100;
pub const WINDOW_FULLSCREEN: i32 = 1;
pub const WINDOW_GUI_EXPANDED: i32 = 0x00000000;
pub const WINDOW_GUI_NORMAL: i32 = 0x00000010;
pub const WINDOW_KEEPRATIO: i32 = 0x00000000;
pub const WINDOW_NORMAL: i32 = 0x00000000;
pub const WINDOW_OPENGL: i32 = 0x00001000;
pub const WND_PROP_ASPECT_RATIO: i32 = 2;
pub const WND_PROP_AUTOSIZE: i32 = 1;
pub const WND_PROP_FULLSCREEN: i32 = 0;
pub const WND_PROP_OPENGL: i32 = 3;
pub const WND_PROP_VISIBLE: i32 = 4;

pub fn cv_destroy_all_windows() -> Result<()> {
// identifier: cvDestroyAllWindows
  unsafe {
    let rv = sys::cv_highgui_cvDestroyAllWindows();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn cv_start_window_thread() -> Result<i32> {
// identifier: cvStartWindowThread
  unsafe {
    let rv = sys::cv_highgui_cvStartWindowThread();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

pub fn cv_stop_loop() -> Result<()> {
// identifier: cvStopLoop
  unsafe {
    let rv = sys::cv_highgui_cvStopLoop();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

///
/// ## C++ default parameters:
/// * delay: 0
pub fn cv_wait_key(delay: i32) -> Result<i32> {
// identifier: cvWaitKey_int_delay
  unsafe {
    let rv = sys::cv_highgui_cvWaitKey_int_delay(delay);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Draws a text on the image.
/// 
/// The function addText draws *text* on the image *img* using a specific font *font* (see example cv::fontQt
/// )
/// 
/// ## Parameters
/// * img: 8-bit 3-channel image where the text should be drawn.
/// * text: Text to write on an image.
/// * org: Point(x,y) where the text should start on an image.
/// * font: Font to use to draw a text.
pub fn add_text(img: &core::Mat, text:&str, org: core::Point, font: &super::highgui::QtFont) -> Result<()> {
// identifier: cv_addText_Mat_img_String_text_Point_org_QtFont_font
  unsafe {
    let text = CString::new(text).unwrap();
    let rv = sys::cv_highgui_cv_addText_Mat_img_String_text_Point_org_QtFont_font(img.as_raw_Mat(), text.as_ptr() as _, org, font.as_raw_QtFont());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Draws a text on the image.
/// 
/// ## Parameters
/// * img: 8-bit 3-channel image where the text should be drawn.
/// * text: Text to write on an image.
/// * org: Point(x,y) where the text should start on an image.
/// * nameFont: Name of the font. The name should match the name of a system font (such as
/// *Times*). If the font is not found, a default one is used.
/// * pointSize: Size of the font. If not specified, equal zero or negative, the point size of the
/// font is set to a system-dependent default value. Generally, this is 12 points.
/// * color: Color of the font in BGRA where A = 255 is fully transparent.
/// * weight: Font weight. Available operation flags are : cv::QtFontWeights You can also specify a positive integer for better control.
/// * style: Font style. Available operation flags are : cv::QtFontStyles
/// * spacing: Spacing between characters. It can be negative or positive.
///
/// ## C++ default parameters:
/// * point_size: -1
/// * color: Scalar::all(0)
/// * weight: QT_FONT_NORMAL
/// * style: QT_STYLE_NORMAL
/// * spacing: 0
pub fn add_text_v0(img: &core::Mat, text:&str, org: core::Point, name_font:&str, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> Result<()> {
// identifier: cv_addText_Mat_img_String_text_Point_org_String_nameFont_int_pointSize_Scalar_color_int_weight_int_style_int_spacing
  unsafe {
    let text = CString::new(text).unwrap();
    let name_font = CString::new(name_font).unwrap();
    let rv = sys::cv_highgui_cv_addText_Mat_img_String_text_Point_org_String_nameFont_int_pointSize_Scalar_color_int_weight_int_style_int_spacing(img.as_raw_Mat(), text.as_ptr() as _, org, name_font.as_ptr() as _, point_size, color, weight, style, spacing);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Destroys all of the HighGUI windows.
/// 
/// The function destroyAllWindows destroys all of the opened HighGUI windows.
pub fn destroy_all_windows() -> Result<()> {
// identifier: cv_destroyAllWindows
  unsafe {
    let rv = sys::cv_highgui_cv_destroyAllWindows();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Destroys the specified window.
/// 
/// The function destroyWindow destroys the window with the given name.
/// 
/// ## Parameters
/// * winname: Name of the window to be destroyed.
pub fn destroy_window(winname:&str) -> Result<()> {
// identifier: cv_destroyWindow_String_winname
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_destroyWindow_String_winname(winname.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Displays a text on a window image as an overlay for a specified duration.
/// 
/// The function displayOverlay displays useful information/tips on top of the window for a certain
/// amount of time *delayms*. The function does not modify the image, displayed in the window, that is,
/// after the specified delay the original content of the window is restored.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * text: Overlay text to write on a window image.
/// * delayms: The period (in milliseconds), during which the overlay text is displayed. If this
/// function is called before the previous overlay text timed out, the timer is restarted and the text
/// is updated. If this value is zero, the text never disappears.
///
/// ## C++ default parameters:
/// * delayms: 0
pub fn display_overlay(winname:&str, text:&str, delayms: i32) -> Result<()> {
// identifier: cv_displayOverlay_String_winname_String_text_int_delayms
  unsafe {
    let winname = CString::new(winname).unwrap();
    let text = CString::new(text).unwrap();
    let rv = sys::cv_highgui_cv_displayOverlay_String_winname_String_text_int_delayms(winname.as_ptr() as _, text.as_ptr() as _, delayms);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Displays a text on the window statusbar during the specified period of time.
/// 
/// The function displayStatusBar displays useful information/tips on top of the window for a certain
/// amount of time *delayms* . This information is displayed on the window statusbar (the window must be
/// created with the CV_GUI_EXPANDED flags).
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * text: Text to write on the window statusbar.
/// * delayms: Duration (in milliseconds) to display the text. If this function is called before
/// the previous text timed out, the timer is restarted and the text is updated. If this value is
/// zero, the text never disappears.
///
/// ## C++ default parameters:
/// * delayms: 0
pub fn display_status_bar(winname:&str, text:&str, delayms: i32) -> Result<()> {
// identifier: cv_displayStatusBar_String_winname_String_text_int_delayms
  unsafe {
    let winname = CString::new(winname).unwrap();
    let text = CString::new(text).unwrap();
    let rv = sys::cv_highgui_cv_displayStatusBar_String_winname_String_text_int_delayms(winname.as_ptr() as _, text.as_ptr() as _, delayms);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Creates the font to draw a text on an image.
/// 
/// The function fontQt creates a cv::QtFont object. This cv::QtFont is not compatible with putText .
/// 
/// A basic usage of this function is the following: :
/// ```ignore
/// QtFont font = fontQt("Times");
/// addText( img1, "Hello World !", Point(50,50), font);
/// ```
/// 
/// 
/// ## Parameters
/// * nameFont: Name of the font. The name should match the name of a system font (such as
/// *Times*). If the font is not found, a default one is used.
/// * pointSize: Size of the font. If not specified, equal zero or negative, the point size of the
/// font is set to a system-dependent default value. Generally, this is 12 points.
/// * color: Color of the font in BGRA where A = 255 is fully transparent. Use the macro CV_RGB
/// for simplicity.
/// * weight: Font weight. Available operation flags are : cv::QtFontWeights You can also specify a positive integer for better control.
/// * style: Font style. Available operation flags are : cv::QtFontStyles
/// * spacing: Spacing between characters. It can be negative or positive.
///
/// ## C++ default parameters:
/// * point_size: -1
/// * color: Scalar::all(0)
/// * weight: QT_FONT_NORMAL
/// * style: QT_STYLE_NORMAL
/// * spacing: 0
pub fn font_qt(name_font:&str, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> Result<super::highgui::QtFont> {
// identifier: cv_fontQt_String_nameFont_int_pointSize_Scalar_color_int_weight_int_style_int_spacing
  unsafe {
    let name_font = CString::new(name_font).unwrap();
    let rv = sys::cv_highgui_cv_fontQt_String_nameFont_int_pointSize_Scalar_color_int_weight_int_style_int_spacing(name_font.as_ptr() as _, point_size, color, weight, style, spacing);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(super::highgui::QtFont { ptr: rv.result })
    }
  }
}

/// Gets the mouse-wheel motion delta, when handling mouse-wheel events cv::EVENT_MOUSEWHEEL and
/// cv::EVENT_MOUSEHWHEEL.
/// 
/// For regular mice with a scroll-wheel, delta will be a multiple of 120. The value 120 corresponds to
/// a one notch rotation of the wheel or the threshold for action to be taken and one such action should
/// occur for each delta. Some high-precision mice with higher-resolution freely-rotating wheels may
/// generate smaller values.
/// 
/// For cv::EVENT_MOUSEWHEEL positive and negative values mean forward and backward scrolling,
/// respectively. For cv::EVENT_MOUSEHWHEEL, where available, positive and negative values mean right and
/// left scrolling, respectively.
/// 
/// With the C API, the macro CV_GET_WHEEL_DELTA(flags) can be used alternatively.
/// 
/// 
/// Note:
/// 
/// Mouse-wheel events are currently supported only on Windows.
/// 
/// ## Parameters
/// * flags: The mouse callback flags parameter.
pub fn get_mouse_wheel_delta(flags: i32) -> Result<i32> {
// identifier: cv_getMouseWheelDelta_int_flags
  unsafe {
    let rv = sys::cv_highgui_cv_getMouseWheelDelta_int_flags(flags);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Returns the trackbar position.
/// 
/// The function returns the current position of the specified trackbar.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of the trackbar.
pub fn get_trackbar_pos(trackbarname:&str, winname:&str) -> Result<i32> {
// identifier: cv_getTrackbarPos_String_trackbarname_String_winname
  unsafe {
    let trackbarname = CString::new(trackbarname).unwrap();
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_getTrackbarPos_String_trackbarname_String_winname(trackbarname.as_ptr() as _, winname.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Provides rectangle of image in the window.
/// 
/// The function getWindowImageRect returns the client screen coordinates, width and height of the image rendering area.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// 
/// @sa resizeWindow moveWindow
pub fn get_window_image_rect(winname:&str) -> Result<core::Rect> {
// identifier: cv_getWindowImageRect_String_winname
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_getWindowImageRect_String_winname(winname.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Provides parameters of a window.
/// 
/// The function getWindowProperty returns properties of a window.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * prop_id: Window property to retrieve. The following operation flags are available: (cv::WindowPropertyFlags)
/// 
/// @sa setWindowProperty
pub fn get_window_property(winname:&str, prop_id: i32) -> Result<f64> {
// identifier: cv_getWindowProperty_String_winname_int_prop_id
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_getWindowProperty_String_winname_int_prop_id(winname.as_ptr() as _, prop_id);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Displays an image in the specified window.
/// 
/// The function imshow displays an image in the specified window. If the window was created with the
/// cv::WINDOW_AUTOSIZE flag, the image is shown with its original size, however it is still limited by the screen resolution.
/// Otherwise, the image is scaled to fit the window. The function may scale the image, depending on its depth:
/// 
/// *   If the image is 8-bit unsigned, it is displayed as is.
/// *   If the image is 16-bit unsigned or 32-bit integer, the pixels are divided by 256. That is, the
/// value range [0,255\*256] is mapped to [0,255].
/// *   If the image is 32-bit or 64-bit floating-point, the pixel values are multiplied by 255. That is, the
/// value range [0,1] is mapped to [0,255].
/// 
/// If window was created with OpenGL support, cv::imshow also support ogl::Buffer , ogl::Texture2D and
/// cuda::GpuMat as input.
/// 
/// If the window was not created before this function, it is assumed creating a window with cv::WINDOW_AUTOSIZE.
/// 
/// If you need to show an image that is bigger than the screen resolution, you will need to call namedWindow("", WINDOW_NORMAL) before the imshow.
/// 
/// 
/// Note: This function should be followed by cv::waitKey function which displays the image for specified
/// milliseconds. Otherwise, it won't display the image. For example, **waitKey(0)** will display the window
/// infinitely until any keypress (it is suitable for image display). **waitKey(25)** will display a frame
/// for 25 ms, after which display will be automatically closed. (If you put it in a loop to read
/// videos, it will display the video frame-by-frame)
/// 
/// 
/// Note:
/// 
/// [__Windows Backend Only__] Pressing Ctrl+C will copy the image to the clipboard.
/// 
/// [__Windows Backend Only__] Pressing Ctrl+S will show a dialog to save the image.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * mat: Image to be shown.
pub fn imshow(winname:&str, mat: &core::Mat) -> Result<()> {
// identifier: cv_imshow_String_winname_Mat_mat
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_imshow_String_winname_Mat_mat(winname.as_ptr() as _, mat.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Loads parameters of the specified window.
/// 
/// The function loadWindowParameters loads size, location, flags, trackbars value, zoom and panning
/// location of the window windowName.
/// 
/// ## Parameters
/// * windowName: Name of the window.
pub fn load_window_parameters(window_name:&str) -> Result<()> {
// identifier: cv_loadWindowParameters_String_windowName
  unsafe {
    let window_name = CString::new(window_name).unwrap();
    let rv = sys::cv_highgui_cv_loadWindowParameters_String_windowName(window_name.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Moves window to the specified position
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * x: The new x-coordinate of the window.
/// * y: The new y-coordinate of the window.
pub fn move_window(winname:&str, x: i32, y: i32) -> Result<()> {
// identifier: cv_moveWindow_String_winname_int_x_int_y
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_moveWindow_String_winname_int_x_int_y(winname.as_ptr() as _, x, y);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Creates a window.
/// 
/// The function namedWindow creates a window that can be used as a placeholder for images and
/// trackbars. Created windows are referred to by their names.
/// 
/// If a window with the same name already exists, the function does nothing.
/// 
/// You can call cv::destroyWindow or cv::destroyAllWindows to close the window and de-allocate any associated
/// memory usage. For a simple program, you do not really have to call these functions because all the
/// resources and windows of the application are closed automatically by the operating system upon exit.
/// 
/// 
/// Note:
/// 
/// Qt backend supports additional flags:
/// *   **WINDOW_NORMAL or WINDOW_AUTOSIZE:** WINDOW_NORMAL enables you to resize the
/// window, whereas WINDOW_AUTOSIZE adjusts automatically the window size to fit the
/// displayed image (see imshow ), and you cannot change the window size manually.
/// *   **WINDOW_FREERATIO or WINDOW_KEEPRATIO:** WINDOW_FREERATIO adjusts the image
/// with no respect to its ratio, whereas WINDOW_KEEPRATIO keeps the image ratio.
/// *   **WINDOW_GUI_NORMAL or WINDOW_GUI_EXPANDED:** WINDOW_GUI_NORMAL is the old way to draw the window
/// without statusbar and toolbar, whereas WINDOW_GUI_EXPANDED is a new enhanced GUI.
/// By default, flags == WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED
/// 
/// ## Parameters
/// * winname: Name of the window in the window caption that may be used as a window identifier.
/// * flags: Flags of the window. The supported flags are: (cv::WindowFlags)
///
/// ## C++ default parameters:
/// * flags: WINDOW_AUTOSIZE
pub fn named_window(winname:&str, flags: i32) -> Result<()> {
// identifier: cv_namedWindow_String_winname_int_flags
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_namedWindow_String_winname_int_flags(winname.as_ptr() as _, flags);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Resizes window to the specified size
/// 
/// 
/// Note:
/// 
/// *   The specified window size is for the image area. Toolbars are not counted.
/// *   Only windows created without cv::WINDOW_AUTOSIZE flag can be resized.
/// 
/// ## Parameters
/// * winname: Window name.
/// * width: The new window width.
/// * height: The new window height.
pub fn resize_window(winname:&str, width: i32, height: i32) -> Result<()> {
// identifier: cv_resizeWindow_String_winname_int_width_int_height
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_resizeWindow_String_winname_int_width_int_height(winname.as_ptr() as _, width, height);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Saves parameters of the specified window.
/// 
/// The function saveWindowParameters saves size, location, flags, trackbars value, zoom and panning
/// location of the window windowName.
/// 
/// ## Parameters
/// * windowName: Name of the window.
pub fn save_window_parameters(window_name:&str) -> Result<()> {
// identifier: cv_saveWindowParameters_String_windowName
  unsafe {
    let window_name = CString::new(window_name).unwrap();
    let rv = sys::cv_highgui_cv_saveWindowParameters_String_windowName(window_name.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// @overload
///
/// ## C++ default parameters:
/// * show_crosshair: true
/// * from_center: false
pub fn select_roi(img: &core::Mat, show_crosshair: bool, from_center: bool) -> Result<core::Rect> {
// identifier: cv_selectROI_Mat_img_bool_showCrosshair_bool_fromCenter
  unsafe {
    let rv = sys::cv_highgui_cv_selectROI_Mat_img_bool_showCrosshair_bool_fromCenter(img.as_raw_Mat(), show_crosshair, from_center);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Selects ROI on the given image.
/// Function creates a window and allows user to select a ROI using mouse.
/// Controls: use `space` or `enter` to finish selection, use key `c` to cancel selection (function will return the zero cv::Rect).
/// 
/// ## Parameters
/// * windowName: name of the window where selection process will be shown.
/// * img: image to select a ROI.
/// * showCrosshair: if true crosshair of selection rectangle will be shown.
/// * fromCenter: if true center of selection will match initial mouse position. In opposite case a corner of
/// selection rectangle will correspont to the initial mouse position.
/// @return selected ROI or empty rect if selection canceled.
/// 
/// 
/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
/// After finish of work an empty callback will be set for the used window.
///
/// ## C++ default parameters:
/// * show_crosshair: true
/// * from_center: false
pub fn select_roi_v0(window_name:&str, img: &core::Mat, show_crosshair: bool, from_center: bool) -> Result<core::Rect> {
// identifier: cv_selectROI_String_windowName_Mat_img_bool_showCrosshair_bool_fromCenter
  unsafe {
    let window_name = CString::new(window_name).unwrap();
    let rv = sys::cv_highgui_cv_selectROI_String_windowName_Mat_img_bool_showCrosshair_bool_fromCenter(window_name.as_ptr() as _, img.as_raw_Mat(), show_crosshair, from_center);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Selects ROIs on the given image.
/// Function creates a window and allows user to select a ROIs using mouse.
/// Controls: use `space` or `enter` to finish current selection and start a new one,
/// use `esc` to terminate multiple ROI selection process.
/// 
/// ## Parameters
/// * windowName: name of the window where selection process will be shown.
/// * img: image to select a ROI.
/// * boundingBoxes: selected ROIs.
/// * showCrosshair: if true crosshair of selection rectangle will be shown.
/// * fromCenter: if true center of selection will match initial mouse position. In opposite case a corner of
/// selection rectangle will correspont to the initial mouse position.
/// 
/// 
/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
/// After finish of work an empty callback will be set for the used window.
///
/// ## C++ default parameters:
/// * show_crosshair: true
/// * from_center: false
pub fn select_ro_is(window_name:&str, img: &core::Mat, bounding_boxes: &types::VectorOfRect, show_crosshair: bool, from_center: bool) -> Result<()> {
// identifier: cv_selectROIs_String_windowName_Mat_img_VectorOfRect_boundingBoxes_bool_showCrosshair_bool_fromCenter
  unsafe {
    let window_name = CString::new(window_name).unwrap();
    let rv = sys::cv_highgui_cv_selectROIs_String_windowName_Mat_img_VectorOfRect_boundingBoxes_bool_showCrosshair_bool_fromCenter(window_name.as_ptr() as _, img.as_raw_Mat(), bounding_boxes.as_raw_VectorOfRect(), show_crosshair, from_center);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Sets the specified window as current OpenGL context.
/// 
/// ## Parameters
/// * winname: Name of the window.
pub fn set_open_gl_context(winname:&str) -> Result<()> {
// identifier: cv_setOpenGlContext_String_winname
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_setOpenGlContext_String_winname(winname.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Sets the trackbar maximum position.
/// 
/// The function sets the maximum position of the specified trackbar in the specified window.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of trackbar.
/// * maxval: New maximum position.
pub fn set_trackbar_max(trackbarname:&str, winname:&str, maxval: i32) -> Result<()> {
// identifier: cv_setTrackbarMax_String_trackbarname_String_winname_int_maxval
  unsafe {
    let trackbarname = CString::new(trackbarname).unwrap();
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_setTrackbarMax_String_trackbarname_String_winname_int_maxval(trackbarname.as_ptr() as _, winname.as_ptr() as _, maxval);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Sets the trackbar minimum position.
/// 
/// The function sets the minimum position of the specified trackbar in the specified window.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of trackbar.
/// * minval: New minimum position.
pub fn set_trackbar_min(trackbarname:&str, winname:&str, minval: i32) -> Result<()> {
// identifier: cv_setTrackbarMin_String_trackbarname_String_winname_int_minval
  unsafe {
    let trackbarname = CString::new(trackbarname).unwrap();
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_setTrackbarMin_String_trackbarname_String_winname_int_minval(trackbarname.as_ptr() as _, winname.as_ptr() as _, minval);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Sets the trackbar position.
/// 
/// The function sets the position of the specified trackbar in the specified window.
/// 
/// 
/// Note:
/// 
/// [__Qt Backend Only__] winname can be empty (or NULL) if the trackbar is attached to the control
/// panel.
/// 
/// ## Parameters
/// * trackbarname: Name of the trackbar.
/// * winname: Name of the window that is the parent of trackbar.
/// * pos: New position.
pub fn set_trackbar_pos(trackbarname:&str, winname:&str, pos: i32) -> Result<()> {
// identifier: cv_setTrackbarPos_String_trackbarname_String_winname_int_pos
  unsafe {
    let trackbarname = CString::new(trackbarname).unwrap();
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_setTrackbarPos_String_trackbarname_String_winname_int_pos(trackbarname.as_ptr() as _, winname.as_ptr() as _, pos);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Changes parameters of a window dynamically.
/// 
/// The function setWindowProperty enables changing properties of a window.
/// 
/// ## Parameters
/// * winname: Name of the window.
/// * prop_id: Window property to edit. The supported operation flags are: (cv::WindowPropertyFlags)
/// * prop_value: New value of the window property. The supported flags are: (cv::WindowFlags)
pub fn set_window_property(winname:&str, prop_id: i32, prop_value: f64) -> Result<()> {
// identifier: cv_setWindowProperty_String_winname_int_prop_id_double_prop_value
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_setWindowProperty_String_winname_int_prop_id_double_prop_value(winname.as_ptr() as _, prop_id, prop_value);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Updates window title
/// ## Parameters
/// * winname: Name of the window.
/// * title: New title.
pub fn set_window_title(winname:&str, title:&str) -> Result<()> {
// identifier: cv_setWindowTitle_String_winname_String_title
  unsafe {
    let winname = CString::new(winname).unwrap();
    let title = CString::new(title).unwrap();
    let rv = sys::cv_highgui_cv_setWindowTitle_String_winname_String_title(winname.as_ptr() as _, title.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn start_window_thread() -> Result<i32> {
// identifier: cv_startWindowThread
  unsafe {
    let rv = sys::cv_highgui_cv_startWindowThread();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

pub fn stop_loop() -> Result<()> {
// identifier: cv_stopLoop
  unsafe {
    let rv = sys::cv_highgui_cv_stopLoop();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Force window to redraw its context and call draw callback ( See cv::setOpenGlDrawCallback ).
/// 
/// ## Parameters
/// * winname: Name of the window.
pub fn update_window(winname:&str) -> Result<()> {
// identifier: cv_updateWindow_String_winname
  unsafe {
    let winname = CString::new(winname).unwrap();
    let rv = sys::cv_highgui_cv_updateWindow_String_winname(winname.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Similar to #waitKey, but returns full key code.
/// 
/// 
/// Note:
/// 
/// Key code is implementation specific and depends on used backend: QT/GTK/Win32/etc
///
/// ## C++ default parameters:
/// * delay: 0
pub fn wait_key_ex(delay: i32) -> Result<i32> {
// identifier: cv_waitKeyEx_int_delay
  unsafe {
    let rv = sys::cv_highgui_cv_waitKeyEx_int_delay(delay);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Waits for a pressed key.
/// 
/// The function waitKey waits for a key event infinitely (when <span lang='latex'>\texttt{delay}\leq 0</span> ) or for delay
/// milliseconds, when it is positive. Since the OS has a minimum time between switching threads, the
/// function will not wait exactly delay ms, it will wait at least delay ms, depending on what else is
/// running on your computer at that time. It returns the code of the pressed key or -1 if no key was
/// pressed before the specified time had elapsed.
/// 
/// 
/// Note:
/// 
/// This function is the only method in HighGUI that can fetch and handle events, so it needs to be
/// called periodically for normal event processing unless HighGUI is used within an environment that
/// takes care of event processing.
/// 
/// 
/// Note:
/// 
/// The function only works if there is at least one HighGUI window created and the window is active.
/// If there are several HighGUI windows, any of them can be active.
/// 
/// ## Parameters
/// * delay: Delay in milliseconds. 0 is the special value that means "forever".
///
/// ## C++ default parameters:
/// * delay: 0
pub fn wait_key(delay: i32) -> Result<i32> {
// identifier: cv_waitKey_int_delay
  unsafe {
    let rv = sys::cv_highgui_cv_waitKey_int_delay(delay);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

// boxed class cv::QtFont
/// QtFont available only for Qt. See cv::fontQt

#[allow(dead_code)]
pub struct QtFont {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::highgui::QtFont {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_QtFont(self.ptr) };
    }
}
impl super::highgui::QtFont {
    #[doc(hidden)] pub fn as_raw_QtFont(&self) -> *mut c_void { self.ptr }
}
impl QtFont {

}
pub const HG_AUTOSIZE: i32 = 0x1;
