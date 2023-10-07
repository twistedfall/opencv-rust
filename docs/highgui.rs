pub mod highgui {
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
	//!    handle repaint events from OS).
	//! *   Add trackbars to the windows, handle simple mouse events as well as keyboard commands.
	//!    # Flags related creating and manipulating HighGUI windows and mouse events
	//!    # OpenGL support
	//!    # Qt New Functions
	//! 
	//!    ![image](https://docs.opencv.org/4.8.1/qtgui.png)
	//! 
	//!    This figure explains new functionality implemented with Qt\* GUI. The new GUI provides a statusbar,
	//!    a toolbar, and a control panel. The control panel can have trackbars and buttonbars attached to it.
	//!    If you cannot see the control panel, press Ctrl+P or right-click any Qt window and select **Display
	//!    properties window**.
	//! 
	//!    *   To attach a trackbar, the window name parameter must be NULL.
	//! 
	//!    *   To attach a buttonbar, a button must be created. If the last bar attached to the control panel
	//!        is a buttonbar, the new button is added to the right of the last button. If the last bar
	//!        attached to the control panel is a trackbar, or the control panel is empty, a new buttonbar is
	//!        created. Then, a new button is attached to it.
	//! 
	//!    See below the example used to generate the figure:
	//!    ```C++
	//!        int main(int argc, char *argv[])
	//!        {
	//! 
	//!            int value = 50;
	//!            int value2 = 0;
	//! 
	//! 
	//!            namedWindow("main1",WINDOW_NORMAL);
	//!            namedWindow("main2",WINDOW_AUTOSIZE | WINDOW_GUI_NORMAL);
	//!            createTrackbar( "track1", "main1", &value, 255,  NULL);
	//! 
	//!            String nameb1 = "button1";
	//!            String nameb2 = "button2";
	//! 
	//!            createButton(nameb1,callbackButton,&nameb1,QT_CHECKBOX,1);
	//!            createButton(nameb2,callbackButton,NULL,QT_CHECKBOX,0);
	//!            createTrackbar( "track2", NULL, &value2, 255, NULL);
	//!            createButton("button5",callbackButton1,NULL,QT_RADIOBOX,0);
	//!            createButton("button6",callbackButton2,NULL,QT_RADIOBOX,1);
	//! 
	//!            setMouseCallback( "main2",on_mouse,NULL );
	//! 
	//!            Mat img1 = imread("files/flower.jpg");
	//!            VideoCapture video;
	//!            video.open("files/hockey.avi");
	//! 
	//!            Mat img2,img3;
	//! 
	//!            while( waitKey(33) != 27 )
	//!            {
	//!                img1.convertTo(img2,-1,1,value);
	//!                video >> img3;
	//! 
	//!                imshow("main1",img2);
	//!                imshow("main2",img3);
	//!            }
	//! 
	//!            destroyAllWindows();
	//! 
	//!            return 0;
	//!        }
	//!    ```
	//! 
	//! 
	//! 
	//!    # WinRT support
	//! 
	//!    This figure explains new functionality implemented with WinRT GUI. The new GUI provides an Image control,
	//!    and a slider panel. Slider panel holds trackbars attached to it.
	//! 
	//!    Sliders are attached below the image control. Every new slider is added below the previous one.
	//! 
	//!    See below the example used to generate the figure:
	//!    ```C++
	//!        void sample_app::MainPage::ShowWindow()
	//!        {
	//!            static cv::String windowName("sample");
	//!            cv::winrt_initContainer(this->cvContainer);
	//!            cv::namedWindow(windowName); // not required
	//! 
	//!            cv::Mat image = cv::imread("Assets/sample.jpg");
	//!            cv::Mat converted = cv::Mat(image.rows, image.cols, CV_8UC4);
	//!            cv::cvtColor(image, converted, COLOR_BGR2BGRA);
	//!            cv::imshow(windowName, converted); // this will create window if it hasn't been created before
	//! 
	//!            int state = 42;
	//!            cv::TrackbarCallback callback = [](int pos, void* userdata)
	//!            {
	//!                if (pos == 0) {
	//!                    cv::destroyWindow(windowName);
	//!                }
	//!            };
	//!            cv::TrackbarCallback callbackTwin = [](int pos, void* userdata)
	//!            {
	//!                if (pos >= 70) {
	//!                    cv::destroyAllWindows();
	//!                }
	//!            };
	//!            cv::createTrackbar("Sample trackbar", windowName, &state, 100, callback);
	//!            cv::createTrackbar("Twin brother", windowName, &state, 100, callbackTwin);
	//!        }
	//!    ```
	//! 
	//! 
	//!    # C API
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::QtFontTraitConst, super::QtFontTrait };
	}
	
	/// indicates that ALT Key is pressed.
	pub const EVENT_FLAG_ALTKEY: i32 = 32;
	/// indicates that CTRL Key is pressed.
	pub const EVENT_FLAG_CTRLKEY: i32 = 8;
	/// indicates that the left mouse button is down.
	pub const EVENT_FLAG_LBUTTON: i32 = 1;
	/// indicates that the middle mouse button is down.
	pub const EVENT_FLAG_MBUTTON: i32 = 4;
	/// indicates that the right mouse button is down.
	pub const EVENT_FLAG_RBUTTON: i32 = 2;
	/// indicates that SHIFT Key is pressed.
	pub const EVENT_FLAG_SHIFTKEY: i32 = 16;
	/// indicates that left mouse button is double clicked.
	pub const EVENT_LBUTTONDBLCLK: i32 = 7;
	/// indicates that the left mouse button is pressed.
	pub const EVENT_LBUTTONDOWN: i32 = 1;
	/// indicates that left mouse button is released.
	pub const EVENT_LBUTTONUP: i32 = 4;
	/// indicates that middle mouse button is double clicked.
	pub const EVENT_MBUTTONDBLCLK: i32 = 9;
	/// indicates that the middle mouse button is pressed.
	pub const EVENT_MBUTTONDOWN: i32 = 3;
	/// indicates that middle mouse button is released.
	pub const EVENT_MBUTTONUP: i32 = 6;
	/// positive and negative values mean right and left scrolling, respectively.
	pub const EVENT_MOUSEHWHEEL: i32 = 11;
	/// indicates that the mouse pointer has moved over the window.
	pub const EVENT_MOUSEMOVE: i32 = 0;
	/// positive and negative values mean forward and backward scrolling, respectively.
	pub const EVENT_MOUSEWHEEL: i32 = 10;
	/// indicates that right mouse button is double clicked.
	pub const EVENT_RBUTTONDBLCLK: i32 = 8;
	/// indicates that the right mouse button is pressed.
	pub const EVENT_RBUTTONDOWN: i32 = 2;
	/// indicates that right mouse button is released.
	pub const EVENT_RBUTTONUP: i32 = 5;
	/// Checkbox button.
	pub const QT_CHECKBOX: i32 = 1;
	/// Weight of 87
	pub const QT_FONT_BLACK: i32 = 87;
	/// Weight of 75
	pub const QT_FONT_BOLD: i32 = 75;
	/// Weight of 63
	pub const QT_FONT_DEMIBOLD: i32 = 63;
	/// Weight of 25
	pub const QT_FONT_LIGHT: i32 = 25;
	/// Weight of 50
	pub const QT_FONT_NORMAL: i32 = 50;
	/// Button should create a new buttonbar
	pub const QT_NEW_BUTTONBAR: i32 = 1024;
	/// Push button.
	pub const QT_PUSH_BUTTON: i32 = 0;
	/// Radiobox button.
	pub const QT_RADIOBOX: i32 = 2;
	/// Italic font.
	pub const QT_STYLE_ITALIC: i32 = 1;
	/// Normal font.
	pub const QT_STYLE_NORMAL: i32 = 0;
	/// Oblique font.
	pub const QT_STYLE_OBLIQUE: i32 = 2;
	/// the user cannot resize the window, the size is constrainted by the image displayed.
	pub const WINDOW_AUTOSIZE: i32 = 1;
	/// the image expends as much as it can (no ratio constraint).
	pub const WINDOW_FREERATIO: i32 = 256;
	/// change the window to fullscreen.
	pub const WINDOW_FULLSCREEN: i32 = 1;
	/// status bar and tool bar
	pub const WINDOW_GUI_EXPANDED: i32 = 0;
	/// old fashious way
	pub const WINDOW_GUI_NORMAL: i32 = 16;
	/// the ratio of the image is respected.
	pub const WINDOW_KEEPRATIO: i32 = 0;
	/// the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
	pub const WINDOW_NORMAL: i32 = 0;
	/// window with opengl support.
	pub const WINDOW_OPENGL: i32 = 4096;
	/// window's aspect ration (can be set to WINDOW_FREERATIO or WINDOW_KEEPRATIO).
	pub const WND_PROP_ASPECT_RATIO: i32 = 2;
	/// autosize property      (can be WINDOW_NORMAL or WINDOW_AUTOSIZE).
	pub const WND_PROP_AUTOSIZE: i32 = 1;
	/// fullscreen property    (can be WINDOW_NORMAL or WINDOW_FULLSCREEN).
	pub const WND_PROP_FULLSCREEN: i32 = 0;
	/// opengl support.
	pub const WND_PROP_OPENGL: i32 = 3;
	/// property to toggle normal window being topmost or not
	pub const WND_PROP_TOPMOST: i32 = 5;
	/// checks whether the window exists and is visible
	pub const WND_PROP_VISIBLE: i32 = 4;
	/// enable or disable VSYNC (in OpenGL mode)
	pub const WND_PROP_VSYNC: i32 = 6;
	/// Mouse Event Flags see cv::MouseCallback
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum MouseEventFlags {
		/// indicates that the left mouse button is down.
		EVENT_FLAG_LBUTTON = 1,
		/// indicates that the right mouse button is down.
		EVENT_FLAG_RBUTTON = 2,
		/// indicates that the middle mouse button is down.
		EVENT_FLAG_MBUTTON = 4,
		/// indicates that CTRL Key is pressed.
		EVENT_FLAG_CTRLKEY = 8,
		/// indicates that SHIFT Key is pressed.
		EVENT_FLAG_SHIFTKEY = 16,
		/// indicates that ALT Key is pressed.
		EVENT_FLAG_ALTKEY = 32,
	}
	
	opencv_type_enum! { crate::highgui::MouseEventFlags }
	
	/// Mouse Events see cv::MouseCallback
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum MouseEventTypes {
		/// indicates that the mouse pointer has moved over the window.
		EVENT_MOUSEMOVE = 0,
		/// indicates that the left mouse button is pressed.
		EVENT_LBUTTONDOWN = 1,
		/// indicates that the right mouse button is pressed.
		EVENT_RBUTTONDOWN = 2,
		/// indicates that the middle mouse button is pressed.
		EVENT_MBUTTONDOWN = 3,
		/// indicates that left mouse button is released.
		EVENT_LBUTTONUP = 4,
		/// indicates that right mouse button is released.
		EVENT_RBUTTONUP = 5,
		/// indicates that middle mouse button is released.
		EVENT_MBUTTONUP = 6,
		/// indicates that left mouse button is double clicked.
		EVENT_LBUTTONDBLCLK = 7,
		/// indicates that right mouse button is double clicked.
		EVENT_RBUTTONDBLCLK = 8,
		/// indicates that middle mouse button is double clicked.
		EVENT_MBUTTONDBLCLK = 9,
		/// positive and negative values mean forward and backward scrolling, respectively.
		EVENT_MOUSEWHEEL = 10,
		/// positive and negative values mean right and left scrolling, respectively.
		EVENT_MOUSEHWHEEL = 11,
	}
	
	opencv_type_enum! { crate::highgui::MouseEventTypes }
	
	/// Qt "button" type
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QtButtonTypes {
		/// Push button.
		QT_PUSH_BUTTON = 0,
		/// Checkbox button.
		QT_CHECKBOX = 1,
		/// Radiobox button.
		QT_RADIOBOX = 2,
		/// Button should create a new buttonbar
		QT_NEW_BUTTONBAR = 1024,
	}
	
	opencv_type_enum! { crate::highgui::QtButtonTypes }
	
	/// Qt font style
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QtFontStyles {
		/// Normal font.
		QT_STYLE_NORMAL = 0,
		/// Italic font.
		QT_STYLE_ITALIC = 1,
		/// Oblique font.
		QT_STYLE_OBLIQUE = 2,
	}
	
	opencv_type_enum! { crate::highgui::QtFontStyles }
	
	/// Qt font weight
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QtFontWeights {
		/// Weight of 25
		QT_FONT_LIGHT = 25,
		/// Weight of 50
		QT_FONT_NORMAL = 50,
		/// Weight of 63
		QT_FONT_DEMIBOLD = 63,
		/// Weight of 75
		QT_FONT_BOLD = 75,
		/// Weight of 87
		QT_FONT_BLACK = 87,
	}
	
	opencv_type_enum! { crate::highgui::QtFontWeights }
	
	/// Flags for cv::namedWindow
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum WindowFlags {
		/// the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
		WINDOW_NORMAL = 0,
		/// the user cannot resize the window, the size is constrainted by the image displayed.
		WINDOW_AUTOSIZE = 1,
		/// window with opengl support.
		WINDOW_OPENGL = 4096,
		// change the window to fullscreen.
		// Duplicate, use WINDOW_AUTOSIZE instead
		// WINDOW_FULLSCREEN = 1,
		/// the image expends as much as it can (no ratio constraint).
		WINDOW_FREERATIO = 256,
		// the ratio of the image is respected.
		// Duplicate, use WINDOW_NORMAL instead
		// WINDOW_KEEPRATIO = 0,
		// status bar and tool bar
		// Duplicate, use WINDOW_KEEPRATIO instead
		// WINDOW_GUI_EXPANDED = 0,
		/// old fashious way
		WINDOW_GUI_NORMAL = 16,
	}
	
	opencv_type_enum! { crate::highgui::WindowFlags }
	
	/// Flags for cv::setWindowProperty / cv::getWindowProperty
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum WindowPropertyFlags {
		/// fullscreen property    (can be WINDOW_NORMAL or WINDOW_FULLSCREEN).
		WND_PROP_FULLSCREEN = 0,
		/// autosize property      (can be WINDOW_NORMAL or WINDOW_AUTOSIZE).
		WND_PROP_AUTOSIZE = 1,
		/// window's aspect ration (can be set to WINDOW_FREERATIO or WINDOW_KEEPRATIO).
		WND_PROP_ASPECT_RATIO = 2,
		/// opengl support.
		WND_PROP_OPENGL = 3,
		/// checks whether the window exists and is visible
		WND_PROP_VISIBLE = 4,
		/// property to toggle normal window being topmost or not
		WND_PROP_TOPMOST = 5,
		/// enable or disable VSYNC (in OpenGL mode)
		WND_PROP_VSYNC = 6,
	}
	
	opencv_type_enum! { crate::highgui::WindowPropertyFlags }
	
	/// Callback function for a button created by cv::createButton
	/// ## Parameters
	/// * state: current state of the button. It could be -1 for a push button, 0 or 1 for a check/radio box button.
	/// * userdata: The optional parameter.
	pub type ButtonCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
	/// Callback function for mouse events. see cv::setMouseCallback
	/// ## Parameters
	/// * event: one of the cv::MouseEventTypes constants.
	/// * x: The x-coordinate of the mouse event.
	/// * y: The y-coordinate of the mouse event.
	/// * flags: one of the cv::MouseEventFlags constants.
	/// * userdata: The optional parameter.
	pub type MouseCallback = Option<Box<dyn FnMut(i32, i32, i32, i32) -> () + Send + Sync + 'static>>;
	/// Callback function defined to be called every frame. See cv::setOpenGlDrawCallback
	/// ## Parameters
	/// * userdata: The optional parameter.
	pub type OpenGlDrawCallback = Option<Box<dyn FnMut() -> () + Send + Sync + 'static>>;
	/// Callback function for Trackbar see cv::createTrackbar
	/// ## Parameters
	/// * pos: current position of the specified trackbar.
	/// * userdata: The optional parameter.
	pub type TrackbarCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
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
	#[inline]
	pub fn add_text(img: &core::Mat, text: &str, org: core::Point, font: &crate::highgui::QtFont) -> Result<()> {
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_addText_const_MatR_const_StringR_Point_const_QtFontR(img.as_raw_Mat(), text.opencv_as_extern(), org.opencv_as_extern(), font.as_raw_QtFont(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [add_text_with_font] function uses the following default values for its arguments:
	/// * point_size: -1
	/// * color: Scalar::all(0)
	/// * weight: QT_FONT_NORMAL
	/// * style: QT_STYLE_NORMAL
	/// * spacing: 0
	#[inline]
	pub fn add_text_with_font_def(img: &core::Mat, text: &str, org: core::Point, name_font: &str) -> Result<()> {
		extern_container_arg!(text);
		extern_container_arg!(name_font);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_addText_const_MatR_const_StringR_Point_const_StringR(img.as_raw_Mat(), text.opencv_as_extern(), org.opencv_as_extern(), name_font.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## C++ default parameters
	/// * point_size: -1
	/// * color: Scalar::all(0)
	/// * weight: QT_FONT_NORMAL
	/// * style: QT_STYLE_NORMAL
	/// * spacing: 0
	#[inline]
	pub fn add_text_with_font(img: &core::Mat, text: &str, org: core::Point, name_font: &str, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> Result<()> {
		extern_container_arg!(text);
		extern_container_arg!(name_font);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int(img.as_raw_Mat(), text.opencv_as_extern(), org.opencv_as_extern(), name_font.opencv_as_extern(), point_size, color.opencv_as_extern(), weight, style, spacing, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Attaches a button to the control panel.
	/// 
	/// The function createButton attaches a button to the control panel. Each button is added to a
	/// buttonbar to the right of the last button. A new buttonbar is created if nothing was attached to the
	/// control panel before, or if the last element attached to the control panel was a trackbar or if the
	/// QT_NEW_BUTTONBAR flag is added to the type.
	/// 
	/// See below various examples of the cv::createButton function call: :
	/// ```C++
	///    createButton("",callbackButton);//create a push button "button 0", that will call callbackButton.
	///    createButton("button2",callbackButton,NULL,QT_CHECKBOX,0);
	///    createButton("button3",callbackButton,&value);
	///    createButton("button5",callbackButton1,NULL,QT_RADIOBOX);
	///    createButton("button6",callbackButton2,NULL,QT_PUSH_BUTTON,1);
	///    createButton("button6",callbackButton2,NULL,QT_PUSH_BUTTON|QT_NEW_BUTTONBAR);// create a push button in a new row
	/// ```
	/// 
	/// 
	/// ## Parameters
	/// * bar_name: Name of the button.
	/// * on_change: Pointer to the function to be called every time the button changes its state.
	/// This function should be prototyped as void Foo(int state,\*void); . *state* is the current state
	/// of the button. It could be -1 for a push button, 0 or 1 for a check/radio box button.
	/// * userdata: Pointer passed to the callback function.
	/// * type: Optional type of the button. Available types are: (cv::QtButtonTypes)
	/// * initial_button_state: Default state of the button. Use for checkbox and radiobox. Its
	/// value could be 0 or 1. (__Optional__)
	/// 
	/// ## Note
	/// This alternative version of [create_button] function uses the following default values for its arguments:
	/// * userdata: 0
	/// * typ: QT_PUSH_BUTTON
	/// * initial_button_state: false
	/// 
	/// ## C++ default parameters
	/// * userdata: 0
	#[inline]
	pub fn create_button_def(bar_name: &str, on_change: crate::highgui::ButtonCallback) -> Result<i32> {
		extern_container_arg!(bar_name);
		callback_arg!(on_change_trampoline(state: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_change(state: i32) -> ());
		userdata_arg!(userdata in callbacks => on_change);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createButton_const_StringR_ButtonCallback_voidX(bar_name.opencv_as_extern(), on_change_trampoline, userdata, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Attaches a button to the control panel.
	/// 
	/// The function createButton attaches a button to the control panel. Each button is added to a
	/// buttonbar to the right of the last button. A new buttonbar is created if nothing was attached to the
	/// control panel before, or if the last element attached to the control panel was a trackbar or if the
	/// QT_NEW_BUTTONBAR flag is added to the type.
	/// 
	/// See below various examples of the cv::createButton function call: :
	/// ```C++
	///    createButton("",callbackButton);//create a push button "button 0", that will call callbackButton.
	///    createButton("button2",callbackButton,NULL,QT_CHECKBOX,0);
	///    createButton("button3",callbackButton,&value);
	///    createButton("button5",callbackButton1,NULL,QT_RADIOBOX);
	///    createButton("button6",callbackButton2,NULL,QT_PUSH_BUTTON,1);
	///    createButton("button6",callbackButton2,NULL,QT_PUSH_BUTTON|QT_NEW_BUTTONBAR);// create a push button in a new row
	/// ```
	/// 
	/// 
	/// ## Parameters
	/// * bar_name: Name of the button.
	/// * on_change: Pointer to the function to be called every time the button changes its state.
	/// This function should be prototyped as void Foo(int state,\*void); . *state* is the current state
	/// of the button. It could be -1 for a push button, 0 or 1 for a check/radio box button.
	/// * userdata: Pointer passed to the callback function.
	/// * type: Optional type of the button. Available types are: (cv::QtButtonTypes)
	/// * initial_button_state: Default state of the button. Use for checkbox and radiobox. Its
	/// value could be 0 or 1. (__Optional__)
	/// 
	/// ## C++ default parameters
	/// * userdata: 0
	/// * typ: QT_PUSH_BUTTON
	/// * initial_button_state: false
	#[inline]
	pub fn create_button(bar_name: &str, on_change: crate::highgui::ButtonCallback, typ: i32, initial_button_state: bool) -> Result<i32> {
		extern_container_arg!(bar_name);
		callback_arg!(on_change_trampoline(state: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_change(state: i32) -> ());
		userdata_arg!(userdata in callbacks => on_change);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createButton_const_StringR_ButtonCallback_voidX_int_bool(bar_name.opencv_as_extern(), on_change_trampoline, userdata, typ, initial_button_state, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates a trackbar and attaches it to the specified window.
	/// 
	/// The function createTrackbar creates a trackbar (a slider or range control) with the specified name
	/// and range, assigns a variable value to be a position synchronized with the trackbar and specifies
	/// the callback function onChange to be called on the trackbar position change. The created trackbar is
	/// displayed in the specified window winname.
	/// 
	/// 
	/// Note:
	/// 
	/// [__Qt Backend Only__] winname can be empty if the trackbar should be attached to the
	/// control panel.
	/// 
	/// Clicking the label of each trackbar enables editing the trackbar values manually.
	/// 
	/// ## Parameters
	/// * trackbarname: Name of the created trackbar.
	/// * winname: Name of the window that will be used as a parent of the created trackbar.
	/// * value: Optional pointer to an integer variable whose value reflects the position of the
	/// slider. Upon creation, the slider position is defined by this variable.
	/// * count: Maximal position of the slider. The minimal position is always 0.
	/// * onChange: Pointer to the function to be called every time the slider changes position. This
	/// function should be prototyped as void Foo(int,void\*); , where the first parameter is the trackbar
	/// position and the second parameter is the user data (see the next parameter). If the callback is
	/// the NULL pointer, no callbacks are called, but only value is updated.
	/// * userdata: User data that is passed as is to the callback. It can be used to handle trackbar
	/// events without using global variables.
	/// 
	/// ## C++ default parameters
	/// * on_change: 0
	/// * userdata: 0
	#[inline]
	pub fn create_trackbar(trackbarname: &str, winname: &str, value: Option<&mut i32>, count: i32, on_change: crate::highgui::TrackbarCallback) -> Result<i32> {
		extern_container_arg!(trackbarname);
		extern_container_arg!(winname);
		callback_arg!(on_change_trampoline(pos: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_change(pos: i32) -> ());
		userdata_arg!(userdata in callbacks => on_change);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createTrackbar_const_StringR_const_StringR_intX_int_TrackbarCallback_voidX(trackbarname.opencv_as_extern(), winname.opencv_as_extern(), value.map_or(::core::ptr::null_mut(), |value| value as *mut _), count, on_change_trampoline, userdata, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Destroys all of the HighGUI windows.
	/// 
	/// The function destroyAllWindows destroys all of the opened HighGUI windows.
	#[inline]
	pub fn destroy_all_windows() -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_destroyAllWindows(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Destroys the specified window.
	/// 
	/// The function destroyWindow destroys the window with the given name.
	/// 
	/// ## Parameters
	/// * winname: Name of the window to be destroyed.
	#[inline]
	pub fn destroy_window(winname: &str) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_destroyWindow_const_StringR(winname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [display_overlay] function uses the following default values for its arguments:
	/// * delayms: 0
	#[inline]
	pub fn display_overlay_def(winname: &str, text: &str) -> Result<()> {
		extern_container_arg!(winname);
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_displayOverlay_const_StringR_const_StringR(winname.opencv_as_extern(), text.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## C++ default parameters
	/// * delayms: 0
	#[inline]
	pub fn display_overlay(winname: &str, text: &str, delayms: i32) -> Result<()> {
		extern_container_arg!(winname);
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_displayOverlay_const_StringR_const_StringR_int(winname.opencv_as_extern(), text.opencv_as_extern(), delayms, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [display_status_bar] function uses the following default values for its arguments:
	/// * delayms: 0
	#[inline]
	pub fn display_status_bar_def(winname: &str, text: &str) -> Result<()> {
		extern_container_arg!(winname);
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_displayStatusBar_const_StringR_const_StringR(winname.opencv_as_extern(), text.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## C++ default parameters
	/// * delayms: 0
	#[inline]
	pub fn display_status_bar(winname: &str, text: &str, delayms: i32) -> Result<()> {
		extern_container_arg!(winname);
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_displayStatusBar_const_StringR_const_StringR_int(winname.opencv_as_extern(), text.opencv_as_extern(), delayms, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates the font to draw a text on an image.
	/// 
	/// The function fontQt creates a cv::QtFont object. This cv::QtFont is not compatible with putText .
	/// 
	/// A basic usage of this function is the following: :
	/// ```C++
	///    QtFont font = fontQt("Times");
	///    addText( img1, "Hello World !", Point(50,50), font);
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
	/// ## Note
	/// This alternative version of [font_qt] function uses the following default values for its arguments:
	/// * point_size: -1
	/// * color: Scalar::all(0)
	/// * weight: QT_FONT_NORMAL
	/// * style: QT_STYLE_NORMAL
	/// * spacing: 0
	#[inline]
	pub fn font_qt_def(name_font: &str) -> Result<crate::highgui::QtFont> {
		extern_container_arg!(name_font);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fontQt_const_StringR(name_font.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::highgui::QtFont::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates the font to draw a text on an image.
	/// 
	/// The function fontQt creates a cv::QtFont object. This cv::QtFont is not compatible with putText .
	/// 
	/// A basic usage of this function is the following: :
	/// ```C++
	///    QtFont font = fontQt("Times");
	///    addText( img1, "Hello World !", Point(50,50), font);
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
	/// ## C++ default parameters
	/// * point_size: -1
	/// * color: Scalar::all(0)
	/// * weight: QT_FONT_NORMAL
	/// * style: QT_STYLE_NORMAL
	/// * spacing: 0
	#[inline]
	pub fn font_qt(name_font: &str, point_size: i32, color: core::Scalar, weight: i32, style: i32, spacing: i32) -> Result<crate::highgui::QtFont> {
		extern_container_arg!(name_font);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_fontQt_const_StringR_int_Scalar_int_int_int(name_font.opencv_as_extern(), point_size, color.opencv_as_extern(), weight, style, spacing, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::highgui::QtFont::opencv_from_extern(ret) };
		Ok(ret)
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
	/// 
	/// Note:
	/// 
	/// Mouse-wheel events are currently supported only on Windows and Cocoa
	/// 
	/// ## Parameters
	/// * flags: The mouse callback flags parameter.
	#[inline]
	pub fn get_mouse_wheel_delta(flags: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getMouseWheelDelta_int(flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the trackbar position.
	/// 
	/// The function returns the current position of the specified trackbar.
	/// 
	/// 
	/// Note:
	/// 
	/// [__Qt Backend Only__] winname can be empty if the trackbar is attached to the control
	/// panel.
	/// 
	/// ## Parameters
	/// * trackbarname: Name of the trackbar.
	/// * winname: Name of the window that is the parent of the trackbar.
	#[inline]
	pub fn get_trackbar_pos(trackbarname: &str, winname: &str) -> Result<i32> {
		extern_container_arg!(trackbarname);
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getTrackbarPos_const_StringR_const_StringR(trackbarname.opencv_as_extern(), winname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Provides rectangle of image in the window.
	/// 
	/// The function getWindowImageRect returns the client screen coordinates, width and height of the image rendering area.
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	/// ## See also
	/// resizeWindow moveWindow
	#[inline]
	pub fn get_window_image_rect(winname: &str) -> Result<core::Rect> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getWindowImageRect_const_StringR(winname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Provides parameters of a window.
	/// 
	/// The function getWindowProperty returns properties of a window.
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	/// * prop_id: Window property to retrieve. The following operation flags are available: (cv::WindowPropertyFlags)
	/// ## See also
	/// setWindowProperty
	#[inline]
	pub fn get_window_property(winname: &str, prop_id: i32) -> Result<f64> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getWindowProperty_const_StringR_int(winname.opencv_as_extern(), prop_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Displays an image in the specified window.
	/// 
	/// The function imshow displays an image in the specified window. If the window was created with the
	/// cv::WINDOW_AUTOSIZE flag, the image is shown with its original size, however it is still limited by the screen resolution.
	/// Otherwise, the image is scaled to fit the window. The function may scale the image, depending on its depth:
	/// 
	/// *   If the image is 8-bit unsigned, it is displayed as is.
	/// *   If the image is 16-bit unsigned, the pixels are divided by 256. That is, the
	///    value range [0,255\*256] is mapped to [0,255].
	/// *   If the image is 32-bit or 64-bit floating-point, the pixel values are multiplied by 255. That is, the
	///    value range [0,1] is mapped to [0,255].
	/// *   32-bit integer images are not processed anymore due to ambiguouty of required transform.
	///    Convert to 8-bit unsigned matrix using a custom preprocessing specific to image's context.
	/// 
	/// If window was created with OpenGL support, cv::imshow also support ogl::Buffer , ogl::Texture2D and
	/// cuda::GpuMat as input.
	/// 
	/// If the window was not created before this function, it is assumed creating a window with cv::WINDOW_AUTOSIZE.
	/// 
	/// If you need to show an image that is bigger than the screen resolution, you will need to call namedWindow("", WINDOW_NORMAL) before the imshow.
	/// 
	/// 
	/// Note: This function should be followed by a call to cv::waitKey or cv::pollKey to perform GUI
	/// housekeeping tasks that are necessary to actually show the given image and make the window respond
	/// to mouse and keyboard events. Otherwise, it won't display the image and the window might lock up.
	/// For example, **waitKey(0)** will display the window infinitely until any keypress (it is suitable
	/// for image display). **waitKey(25)** will display a frame and wait approximately 25 ms for a key
	/// press (suitable for displaying a video frame-by-frame). To remove the window, use cv::destroyWindow.
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
	#[inline]
	pub fn imshow(winname: &str, mat: &impl core::ToInputArray) -> Result<()> {
		extern_container_arg!(winname);
		input_array_arg!(mat);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_imshow_const_StringR_const__InputArrayR(winname.opencv_as_extern(), mat.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Loads parameters of the specified window.
	/// 
	/// The function loadWindowParameters loads size, location, flags, trackbars value, zoom and panning
	/// location of the window windowName.
	/// 
	/// ## Parameters
	/// * windowName: Name of the window.
	#[inline]
	pub fn load_window_parameters(window_name: &str) -> Result<()> {
		extern_container_arg!(window_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_loadWindowParameters_const_StringR(window_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Moves the window to the specified position
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	/// * x: The new x-coordinate of the window.
	/// * y: The new y-coordinate of the window.
	#[inline]
	pub fn move_window(winname: &str, x: i32, y: i32) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_moveWindow_const_StringR_int_int(winname.opencv_as_extern(), x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	///  *   **WINDOW_NORMAL or WINDOW_AUTOSIZE:** WINDOW_NORMAL enables you to resize the
	///      window, whereas WINDOW_AUTOSIZE adjusts automatically the window size to fit the
	///      displayed image (see imshow ), and you cannot change the window size manually.
	///  *   **WINDOW_FREERATIO or WINDOW_KEEPRATIO:** WINDOW_FREERATIO adjusts the image
	///      with no respect to its ratio, whereas WINDOW_KEEPRATIO keeps the image ratio.
	///  *   **WINDOW_GUI_NORMAL or WINDOW_GUI_EXPANDED:** WINDOW_GUI_NORMAL is the old way to draw the window
	///      without statusbar and toolbar, whereas WINDOW_GUI_EXPANDED is a new enhanced GUI.
	/// By default, flags == WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED
	/// 
	/// ## Parameters
	/// * winname: Name of the window in the window caption that may be used as a window identifier.
	/// * flags: Flags of the window. The supported flags are: (cv::WindowFlags)
	/// 
	/// ## Note
	/// This alternative version of [named_window] function uses the following default values for its arguments:
	/// * flags: WINDOW_AUTOSIZE
	#[inline]
	pub fn named_window_def(winname: &str) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_namedWindow_const_StringR(winname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	///  *   **WINDOW_NORMAL or WINDOW_AUTOSIZE:** WINDOW_NORMAL enables you to resize the
	///      window, whereas WINDOW_AUTOSIZE adjusts automatically the window size to fit the
	///      displayed image (see imshow ), and you cannot change the window size manually.
	///  *   **WINDOW_FREERATIO or WINDOW_KEEPRATIO:** WINDOW_FREERATIO adjusts the image
	///      with no respect to its ratio, whereas WINDOW_KEEPRATIO keeps the image ratio.
	///  *   **WINDOW_GUI_NORMAL or WINDOW_GUI_EXPANDED:** WINDOW_GUI_NORMAL is the old way to draw the window
	///      without statusbar and toolbar, whereas WINDOW_GUI_EXPANDED is a new enhanced GUI.
	/// By default, flags == WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED
	/// 
	/// ## Parameters
	/// * winname: Name of the window in the window caption that may be used as a window identifier.
	/// * flags: Flags of the window. The supported flags are: (cv::WindowFlags)
	/// 
	/// ## C++ default parameters
	/// * flags: WINDOW_AUTOSIZE
	#[inline]
	pub fn named_window(winname: &str, flags: i32) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_namedWindow_const_StringR_int(winname.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Polls for a pressed key.
	/// 
	/// The function pollKey polls for a key event without waiting. It returns the code of the pressed key
	/// or -1 if no key was pressed since the last invocation. To wait until a key was pressed, use #waitKey.
	/// 
	/// 
	/// Note: The functions [wait_key] and [poll_key] are the only methods in HighGUI that can fetch and handle
	/// GUI events, so one of them needs to be called periodically for normal event processing unless
	/// HighGUI is used within an environment that takes care of event processing.
	/// 
	/// 
	/// Note: The function only works if there is at least one HighGUI window created and the window is
	/// active. If there are several HighGUI windows, any of them can be active.
	#[inline]
	pub fn poll_key() -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_pollKey(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Resizes the window to the specified size
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
	/// 
	/// ## Overloaded parameters
	/// 
	/// * winname: Window name.
	/// * size: The new window size.
	#[inline]
	pub fn resize_window_size(winname: &str, size: core::Size) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_resizeWindow_const_StringR_const_SizeR(winname.opencv_as_extern(), &size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Resizes the window to the specified size
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
	#[inline]
	pub fn resize_window(winname: &str, width: i32, height: i32) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_resizeWindow_const_StringR_int_int(winname.opencv_as_extern(), width, height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Saves parameters of the specified window.
	/// 
	/// The function saveWindowParameters saves size, location, flags, trackbars value, zoom and panning
	/// location of the window windowName.
	/// 
	/// ## Parameters
	/// * windowName: Name of the window.
	#[inline]
	pub fn save_window_parameters(window_name: &str) -> Result<()> {
		extern_container_arg!(window_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saveWindowParameters_const_StringR(window_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Allows users to select a ROI on the given image.
	/// 
	/// The function creates a window and allows users to select a ROI using the mouse.
	/// Controls: use `space` or `enter` to finish selection, use key `c` to cancel selection (function will return the zero cv::Rect).
	/// 
	/// ## Parameters
	/// * windowName: name of the window where selection process will be shown.
	/// * img: image to select a ROI.
	/// * showCrosshair: if true crosshair of selection rectangle will be shown.
	/// * fromCenter: if true center of selection will match initial mouse position. In opposite case a corner of
	/// selection rectangle will correspont to the initial mouse position.
	/// * printNotice: if true a notice to select ROI or cancel selection will be printed in console.
	/// ## Returns
	/// selected ROI or empty rect if selection canceled.
	/// 
	/// 
	/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
	/// After finish of work an empty callback will be set for the used window.
	/// 
	/// ## Note
	/// This alternative version of [select_roi] function uses the following default values for its arguments:
	/// * show_crosshair: true
	/// * from_center: false
	/// * print_notice: true
	#[inline]
	pub fn select_roi_def(window_name: &str, img: &impl core::ToInputArray) -> Result<core::Rect> {
		extern_container_arg!(window_name);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_selectROI_const_StringR_const__InputArrayR(window_name.opencv_as_extern(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Allows users to select a ROI on the given image.
	/// 
	/// The function creates a window and allows users to select a ROI using the mouse.
	/// Controls: use `space` or `enter` to finish selection, use key `c` to cancel selection (function will return the zero cv::Rect).
	/// 
	/// ## Parameters
	/// * windowName: name of the window where selection process will be shown.
	/// * img: image to select a ROI.
	/// * showCrosshair: if true crosshair of selection rectangle will be shown.
	/// * fromCenter: if true center of selection will match initial mouse position. In opposite case a corner of
	/// selection rectangle will correspont to the initial mouse position.
	/// * printNotice: if true a notice to select ROI or cancel selection will be printed in console.
	/// ## Returns
	/// selected ROI or empty rect if selection canceled.
	/// 
	/// 
	/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
	/// After finish of work an empty callback will be set for the used window.
	/// 
	/// ## C++ default parameters
	/// * show_crosshair: true
	/// * from_center: false
	/// * print_notice: true
	#[inline]
	pub fn select_roi(window_name: &str, img: &impl core::ToInputArray, show_crosshair: bool, from_center: bool, print_notice: bool) -> Result<core::Rect> {
		extern_container_arg!(window_name);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_selectROI_const_StringR_const__InputArrayR_bool_bool_bool(window_name.opencv_as_extern(), img.as_raw__InputArray(), show_crosshair, from_center, print_notice, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [select_roi_1] function uses the following default values for its arguments:
	/// * show_crosshair: true
	/// * from_center: false
	/// * print_notice: true
	#[inline]
	pub fn select_roi_1_def(img: &impl core::ToInputArray) -> Result<core::Rect> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_selectROI_const__InputArrayR(img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Allows users to select a ROI on the given image.
	/// 
	/// The function creates a window and allows users to select a ROI using the mouse.
	/// Controls: use `space` or `enter` to finish selection, use key `c` to cancel selection (function will return the zero cv::Rect).
	/// 
	/// ## Parameters
	/// * windowName: name of the window where selection process will be shown.
	/// * img: image to select a ROI.
	/// * showCrosshair: if true crosshair of selection rectangle will be shown.
	/// * fromCenter: if true center of selection will match initial mouse position. In opposite case a corner of
	/// selection rectangle will correspont to the initial mouse position.
	/// * printNotice: if true a notice to select ROI or cancel selection will be printed in console.
	/// ## Returns
	/// selected ROI or empty rect if selection canceled.
	/// 
	/// 
	/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
	/// After finish of work an empty callback will be set for the used window.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * show_crosshair: true
	/// * from_center: false
	/// * print_notice: true
	#[inline]
	pub fn select_roi_1(img: &impl core::ToInputArray, show_crosshair: bool, from_center: bool, print_notice: bool) -> Result<core::Rect> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_selectROI_const__InputArrayR_bool_bool_bool(img.as_raw__InputArray(), show_crosshair, from_center, print_notice, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Allows users to select multiple ROIs on the given image.
	/// 
	/// The function creates a window and allows users to select multiple ROIs using the mouse.
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
	/// * printNotice: if true a notice to select ROI or cancel selection will be printed in console.
	/// 
	/// 
	/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
	/// After finish of work an empty callback will be set for the used window.
	/// 
	/// ## Note
	/// This alternative version of [select_ro_is] function uses the following default values for its arguments:
	/// * show_crosshair: true
	/// * from_center: false
	/// * print_notice: true
	#[inline]
	pub fn select_ro_is_def(window_name: &str, img: &impl core::ToInputArray, bounding_boxes: &mut core::Vector<core::Rect>) -> Result<()> {
		extern_container_arg!(window_name);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_selectROIs_const_StringR_const__InputArrayR_vectorLRectGR(window_name.opencv_as_extern(), img.as_raw__InputArray(), bounding_boxes.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Allows users to select multiple ROIs on the given image.
	/// 
	/// The function creates a window and allows users to select multiple ROIs using the mouse.
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
	/// * printNotice: if true a notice to select ROI or cancel selection will be printed in console.
	/// 
	/// 
	/// Note: The function sets it's own mouse callback for specified window using cv::setMouseCallback(windowName, ...).
	/// After finish of work an empty callback will be set for the used window.
	/// 
	/// ## C++ default parameters
	/// * show_crosshair: true
	/// * from_center: false
	/// * print_notice: true
	#[inline]
	pub fn select_ro_is(window_name: &str, img: &impl core::ToInputArray, bounding_boxes: &mut core::Vector<core::Rect>, show_crosshair: bool, from_center: bool, print_notice: bool) -> Result<()> {
		extern_container_arg!(window_name);
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_selectROIs_const_StringR_const__InputArrayR_vectorLRectGR_bool_bool_bool(window_name.opencv_as_extern(), img.as_raw__InputArray(), bounding_boxes.as_raw_mut_VectorOfRect(), show_crosshair, from_center, print_notice, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @example samples/cpp/create_mask.cpp
	/// This program demonstrates using mouse events and how to make and use a mask image (black and white) .
	/// 
	/// Sets mouse handler for the specified window
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	/// * onMouse: Callback function for mouse events. See OpenCV samples on how to specify and use the callback.
	/// * userdata: The optional parameter passed to the callback.
	/// 
	/// ## C++ default parameters
	/// * userdata: 0
	#[inline]
	pub fn set_mouse_callback(winname: &str, on_mouse: crate::highgui::MouseCallback) -> Result<()> {
		extern_container_arg!(winname);
		callback_arg!(on_mouse_trampoline(event: i32, x: i32, y: i32, flags: i32, userdata: *mut c_void) -> () => userdata in callbacks => on_mouse(event: i32, x: i32, y: i32, flags: i32) -> ());
		userdata_arg!(userdata in callbacks => on_mouse);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setMouseCallback_const_StringR_MouseCallback_voidX(winname.opencv_as_extern(), on_mouse_trampoline, userdata, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the specified window as current OpenGL context.
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	#[inline]
	pub fn set_opengl_context(winname: &str) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setOpenGlContext_const_StringR(winname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets a callback function to be called to draw on top of displayed image.
	/// 
	/// The function setOpenGlDrawCallback can be used to draw 3D data on the window. See the example of
	/// callback function below:
	/// ```C++
	///    void on_opengl(void* param)
	///    {
	///        glLoadIdentity();
	/// 
	///        glTranslated(0.0, 0.0, -1.0);
	/// 
	///        glRotatef( 55, 1, 0, 0 );
	///        glRotatef( 45, 0, 1, 0 );
	///        glRotatef( 0, 0, 0, 1 );
	/// 
	///        static const int coords[6][4][3] = {
	///            { { +1, -1, -1 }, { -1, -1, -1 }, { -1, +1, -1 }, { +1, +1, -1 } },
	///            { { +1, +1, -1 }, { -1, +1, -1 }, { -1, +1, +1 }, { +1, +1, +1 } },
	///            { { +1, -1, +1 }, { +1, -1, -1 }, { +1, +1, -1 }, { +1, +1, +1 } },
	///            { { -1, -1, -1 }, { -1, -1, +1 }, { -1, +1, +1 }, { -1, +1, -1 } },
	///            { { +1, -1, +1 }, { -1, -1, +1 }, { -1, -1, -1 }, { +1, -1, -1 } },
	///            { { -1, -1, +1 }, { +1, -1, +1 }, { +1, +1, +1 }, { -1, +1, +1 } }
	///        };
	/// 
	///        for (int i = 0; i < 6; ++i) {
	///                    glColor3ub( i*20, 100+i*10, i*42 );
	///                    glBegin(GL_QUADS);
	///                    for (int j = 0; j < 4; ++j) {
	///                             glVertex3d(0.2 * coords[i][j][0], 0.2 * coords[i][j][1], 0.2 * coords[i][j][2]);
	///                    }
	///                    glEnd();
	///        }
	///    }
	/// ```
	/// 
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	/// * onOpenGlDraw: Pointer to the function to be called every frame. This function should be
	/// prototyped as void Foo(void\*) .
	/// * userdata: Pointer passed to the callback function.(__Optional__)
	/// 
	/// ## C++ default parameters
	/// * userdata: 0
	#[inline]
	pub fn set_opengl_draw_callback(winname: &str, on_opengl_draw: crate::highgui::OpenGlDrawCallback) -> Result<()> {
		extern_container_arg!(winname);
		callback_arg!(on_opengl_draw_trampoline(userdata: *mut c_void) -> () => userdata in callbacks => on_opengl_draw() -> ());
		userdata_arg!(userdata in callbacks => on_opengl_draw);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setOpenGlDrawCallback_const_StringR_OpenGlDrawCallback_voidX(winname.opencv_as_extern(), on_opengl_draw_trampoline, userdata, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the trackbar maximum position.
	/// 
	/// The function sets the maximum position of the specified trackbar in the specified window.
	/// 
	/// 
	/// Note:
	/// 
	/// [__Qt Backend Only__] winname can be empty if the trackbar is attached to the control
	/// panel.
	/// 
	/// ## Parameters
	/// * trackbarname: Name of the trackbar.
	/// * winname: Name of the window that is the parent of trackbar.
	/// * maxval: New maximum position.
	#[inline]
	pub fn set_trackbar_max(trackbarname: &str, winname: &str, maxval: i32) -> Result<()> {
		extern_container_arg!(trackbarname);
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setTrackbarMax_const_StringR_const_StringR_int(trackbarname.opencv_as_extern(), winname.opencv_as_extern(), maxval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the trackbar minimum position.
	/// 
	/// The function sets the minimum position of the specified trackbar in the specified window.
	/// 
	/// 
	/// Note:
	/// 
	/// [__Qt Backend Only__] winname can be empty if the trackbar is attached to the control
	/// panel.
	/// 
	/// ## Parameters
	/// * trackbarname: Name of the trackbar.
	/// * winname: Name of the window that is the parent of trackbar.
	/// * minval: New minimum position.
	#[inline]
	pub fn set_trackbar_min(trackbarname: &str, winname: &str, minval: i32) -> Result<()> {
		extern_container_arg!(trackbarname);
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setTrackbarMin_const_StringR_const_StringR_int(trackbarname.opencv_as_extern(), winname.opencv_as_extern(), minval, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the trackbar position.
	/// 
	/// The function sets the position of the specified trackbar in the specified window.
	/// 
	/// 
	/// Note:
	/// 
	/// [__Qt Backend Only__] winname can be empty if the trackbar is attached to the control
	/// panel.
	/// 
	/// ## Parameters
	/// * trackbarname: Name of the trackbar.
	/// * winname: Name of the window that is the parent of trackbar.
	/// * pos: New position.
	#[inline]
	pub fn set_trackbar_pos(trackbarname: &str, winname: &str, pos: i32) -> Result<()> {
		extern_container_arg!(trackbarname);
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setTrackbarPos_const_StringR_const_StringR_int(trackbarname.opencv_as_extern(), winname.opencv_as_extern(), pos, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Changes parameters of a window dynamically.
	/// 
	/// The function setWindowProperty enables changing properties of a window.
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	/// * prop_id: Window property to edit. The supported operation flags are: (cv::WindowPropertyFlags)
	/// * prop_value: New value of the window property. The supported flags are: (cv::WindowFlags)
	#[inline]
	pub fn set_window_property(winname: &str, prop_id: i32, prop_value: f64) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setWindowProperty_const_StringR_int_double(winname.opencv_as_extern(), prop_id, prop_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Updates window title
	/// ## Parameters
	/// * winname: Name of the window.
	/// * title: New title.
	#[inline]
	pub fn set_window_title(winname: &str, title: &str) -> Result<()> {
		extern_container_arg!(winname);
		extern_container_arg!(title);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_setWindowTitle_const_StringR_const_StringR(winname.opencv_as_extern(), title.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn start_loop(pt2_func: Option<unsafe extern "C" fn(i32, *mut *mut c_char) -> i32>, argc: i32, argv: &mut [&str]) -> Result<i32> {
		string_array_arg_mut!(argv);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_startLoop_int__X__int__charXX__int_charXX(pt2_func, argc, argv.as_mut_ptr(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn start_window_thread() -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_startWindowThread(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn stop_loop() -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stopLoop(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Force window to redraw its context and call draw callback ( See cv::setOpenGlDrawCallback ).
	/// 
	/// ## Parameters
	/// * winname: Name of the window.
	#[inline]
	pub fn update_window(winname: &str) -> Result<()> {
		extern_container_arg!(winname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_updateWindow_const_StringR(winname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Waits for a pressed key.
	/// 
	/// The function waitKey waits for a key event infinitely (when ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdelay%7D%5Cleq%200) ) or for delay
	/// milliseconds, when it is positive. Since the OS has a minimum time between switching threads, the
	/// function will not wait exactly delay ms, it will wait at least delay ms, depending on what else is
	/// running on your computer at that time. It returns the code of the pressed key or -1 if no key was
	/// pressed before the specified time had elapsed. To check for a key press but not wait for it, use
	/// #pollKey.
	/// 
	/// 
	/// Note: The functions [wait_key] and [poll_key] are the only methods in HighGUI that can fetch and handle
	/// GUI events, so one of them needs to be called periodically for normal event processing unless
	/// HighGUI is used within an environment that takes care of event processing.
	/// 
	/// 
	/// Note: The function only works if there is at least one HighGUI window created and the window is
	/// active. If there are several HighGUI windows, any of them can be active.
	/// 
	/// ## Parameters
	/// * delay: Delay in milliseconds. 0 is the special value that means "forever".
	/// 
	/// ## Note
	/// This alternative version of [wait_key] function uses the following default values for its arguments:
	/// * delay: 0
	#[inline]
	pub fn wait_key_def() -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_waitKey(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Similar to #waitKey, but returns full key code.
	/// 
	/// 
	/// Note:
	/// 
	/// Key code is implementation specific and depends on used backend: QT/GTK/Win32/etc
	/// 
	/// ## Note
	/// This alternative version of [wait_key_ex] function uses the following default values for its arguments:
	/// * delay: 0
	#[inline]
	pub fn wait_key_ex_def() -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_waitKeyEx(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Similar to #waitKey, but returns full key code.
	/// 
	/// 
	/// Note:
	/// 
	/// Key code is implementation specific and depends on used backend: QT/GTK/Win32/etc
	/// 
	/// ## C++ default parameters
	/// * delay: 0
	#[inline]
	pub fn wait_key_ex(delay: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_waitKeyEx_int(delay, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Waits for a pressed key.
	/// 
	/// The function waitKey waits for a key event infinitely (when ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdelay%7D%5Cleq%200) ) or for delay
	/// milliseconds, when it is positive. Since the OS has a minimum time between switching threads, the
	/// function will not wait exactly delay ms, it will wait at least delay ms, depending on what else is
	/// running on your computer at that time. It returns the code of the pressed key or -1 if no key was
	/// pressed before the specified time had elapsed. To check for a key press but not wait for it, use
	/// #pollKey.
	/// 
	/// 
	/// Note: The functions [wait_key] and [poll_key] are the only methods in HighGUI that can fetch and handle
	/// GUI events, so one of them needs to be called periodically for normal event processing unless
	/// HighGUI is used within an environment that takes care of event processing.
	/// 
	/// 
	/// Note: The function only works if there is at least one HighGUI window created and the window is
	/// active. If there are several HighGUI windows, any of them can be active.
	/// 
	/// ## Parameters
	/// * delay: Delay in milliseconds. 0 is the special value that means "forever".
	/// 
	/// ## C++ default parameters
	/// * delay: 0
	#[inline]
	pub fn wait_key(delay: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_waitKey_int(delay, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::highgui::QtFont]
	pub trait QtFontTraitConst {
		fn as_raw_QtFont(&self) -> *const c_void;
	
		/// Name of the font
		#[inline]
		fn name_font(&self) -> String {
			let ret = unsafe { sys::cv_QtFont_propNameFont_const(self.as_raw_QtFont()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// Color of the font. Scalar(blue_component, green_component, red_component[, alpha_component])
		#[inline]
		fn color(&self) -> core::Scalar {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QtFont_propColor_const(self.as_raw_QtFont(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// See cv::QtFontStyles
		#[inline]
		fn font_face(&self) -> i32 {
			let ret = unsafe { sys::cv_QtFont_propFont_face_const(self.as_raw_QtFont()) };
			ret
		}
		
		/// font data and metrics
		#[inline]
		fn ascii(&self) -> *const i32 {
			let ret = unsafe { sys::cv_QtFont_propAscii_const(self.as_raw_QtFont()) };
			ret
		}
		
		#[inline]
		fn greek(&self) -> *const i32 {
			let ret = unsafe { sys::cv_QtFont_propGreek_const(self.as_raw_QtFont()) };
			ret
		}
		
		#[inline]
		fn cyrillic(&self) -> *const i32 {
			let ret = unsafe { sys::cv_QtFont_propCyrillic_const(self.as_raw_QtFont()) };
			ret
		}
		
		#[inline]
		fn hscale(&self) -> f32 {
			let ret = unsafe { sys::cv_QtFont_propHscale_const(self.as_raw_QtFont()) };
			ret
		}
		
		#[inline]
		fn vscale(&self) -> f32 {
			let ret = unsafe { sys::cv_QtFont_propVscale_const(self.as_raw_QtFont()) };
			ret
		}
		
		/// slope coefficient: 0 - normal, >0 - italic
		#[inline]
		fn shear(&self) -> f32 {
			let ret = unsafe { sys::cv_QtFont_propShear_const(self.as_raw_QtFont()) };
			ret
		}
		
		/// See cv::QtFontWeights
		#[inline]
		fn thickness(&self) -> i32 {
			let ret = unsafe { sys::cv_QtFont_propThickness_const(self.as_raw_QtFont()) };
			ret
		}
		
		/// horizontal interval between letters
		#[inline]
		fn dx(&self) -> f32 {
			let ret = unsafe { sys::cv_QtFont_propDx_const(self.as_raw_QtFont()) };
			ret
		}
		
		/// PointSize
		#[inline]
		fn line_type(&self) -> i32 {
			let ret = unsafe { sys::cv_QtFont_propLine_type_const(self.as_raw_QtFont()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::highgui::QtFont]
	pub trait QtFontTrait: crate::highgui::QtFontTraitConst {
		fn as_raw_mut_QtFont(&mut self) -> *mut c_void;
	
		/// Color of the font. Scalar(blue_component, green_component, red_component[, alpha_component])
		#[inline]
		fn set_color(&mut self, val: core::Scalar) {
			let ret = unsafe { sys::cv_QtFont_propColor_Scalar(self.as_raw_mut_QtFont(), val.opencv_as_extern()) };
			ret
		}
		
		/// See cv::QtFontStyles
		#[inline]
		fn set_font_face(&mut self, val: i32) {
			let ret = unsafe { sys::cv_QtFont_propFont_face_int(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
		#[inline]
		fn set_hscale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_QtFont_propHscale_float(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
		#[inline]
		fn set_vscale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_QtFont_propVscale_float(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
		/// slope coefficient: 0 - normal, >0 - italic
		#[inline]
		fn set_shear(&mut self, val: f32) {
			let ret = unsafe { sys::cv_QtFont_propShear_float(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
		/// See cv::QtFontWeights
		#[inline]
		fn set_thickness(&mut self, val: i32) {
			let ret = unsafe { sys::cv_QtFont_propThickness_int(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
		/// horizontal interval between letters
		#[inline]
		fn set_dx(&mut self, val: f32) {
			let ret = unsafe { sys::cv_QtFont_propDx_float(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
		/// PointSize
		#[inline]
		fn set_line_type(&mut self, val: i32) {
			let ret = unsafe { sys::cv_QtFont_propLine_type_int(self.as_raw_mut_QtFont(), val) };
			ret
		}
		
	}
	
	/// QtFont available only for Qt. See cv::fontQt
	pub struct QtFont {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QtFont }
	
	impl Drop for QtFont {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QtFont_delete(self.as_raw_mut_QtFont()) };
		}
	}
	
	unsafe impl Send for QtFont {}
	
	impl crate::highgui::QtFontTraitConst for QtFont {
		#[inline] fn as_raw_QtFont(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::highgui::QtFontTrait for QtFont {
		#[inline] fn as_raw_mut_QtFont(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl QtFont {
	}
	
	impl std::fmt::Debug for QtFont {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QtFont")
				.field("name_font", &crate::highgui::QtFontTraitConst::name_font(self))
				.field("color", &crate::highgui::QtFontTraitConst::color(self))
				.field("font_face", &crate::highgui::QtFontTraitConst::font_face(self))
				.field("ascii", &crate::highgui::QtFontTraitConst::ascii(self))
				.field("greek", &crate::highgui::QtFontTraitConst::greek(self))
				.field("cyrillic", &crate::highgui::QtFontTraitConst::cyrillic(self))
				.field("hscale", &crate::highgui::QtFontTraitConst::hscale(self))
				.field("vscale", &crate::highgui::QtFontTraitConst::vscale(self))
				.field("shear", &crate::highgui::QtFontTraitConst::shear(self))
				.field("thickness", &crate::highgui::QtFontTraitConst::thickness(self))
				.field("dx", &crate::highgui::QtFontTraitConst::dx(self))
				.field("line_type", &crate::highgui::QtFontTraitConst::line_type(self))
				.finish()
		}
	}
}
