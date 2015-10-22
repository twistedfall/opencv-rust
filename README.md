## Rust OpenCV bindings

[![Build Status](https://travis-ci.org/kali/opencv-rust.svg?branch=cv3)](https://travis-ci.org/kali/opencv-rust)

This is my shot at generating Rust bindings for OpenCV.

This is absolutely not ready for prime time.

Instead of binding the deprecated C-compatible interface, I chose to mimick the
python and java wrappers: parsing C++ headers, generating a C interface to the 
C++ api, and wrapping that in Rust.

## Try it

There are quite a few moving parts here. I'm working on MacOS, and Travis runs
on Ubuntu 12.0.4, so these two are known to work. I expect any relatively 
recent Linux to work fine, and I can't see anything serious preventing this
to work under Windows if you are willing to spend a few hours on it.

You will need OpenCV 2.4.11. Other 2.4 versions may work, feel free to try and
tell me what happens. 3.0 will not work as is.

You will also need python.

[API Documentation](http://www.poumeyrol.fr/doc/opencv-rust/opencv/index.html)
— or what I managed to extract from opencv widely inconsistent doxygen. At
least you can see what has been ported or not to rust and how. You'll probably
need to refer to the official [OpenCV C++ documentation](http://docs.opencv.org/).

All the major modules in the C++ API
are merged together in a huge cv:: namespace, leaving the client developper
to manage its namespace by selectively including relevant headers. I instead
made one rust module for each major OpenCV module. So C++ cv::Mat is 
::opencv::core::Mat in Rust, etc.

The methods and field names have been snake_cased. Methods arguments with
default value loose these default values, but they are reported in the
API documentation.

Overloaded methods have been — manually — given a different name.

All methods return a Result, because... yeah, exceptions.

## API coverage

I don't know or use the whole OpenCV library. I have deliberately let some
modules out of these bindings in order to focus on parts I understand and/or
have a need for. Due to the widely inconsistent use of C++ features and
syntax in the library headers, adding them may be a matter of minutes or days,
but we'll get there. PR are welcome.

Even in the covered modules, some classes or methods have been left out because
they were referring to some C++ feature that the script did not understand. If
we need them, we'll get them. Eventually. Please report them.

And at some point, obviously, I want this to move to OpenCV 3.0.

## Hack it

Please don't tell me how ugly these scripts look. Believe me, I am already
painfully aware of this.

hdr_parser.py comes from opencv python/java generator. I have tried not to mess
too much with this file, but had to make a few changes.

gen_rust is initially a copy of gen_java, also from
the OpenCV generators, but is now so far from the initial stuff that I
consider it an original work. 

The license for the original work is [WTFPL](http://www.wtfpl.net/).

Special thanks to [ttacon](https://github.com/ttacon) for yielding the crate name.
