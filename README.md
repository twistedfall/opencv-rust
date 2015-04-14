## Rust OpenCV bindings

This is my shot at generating Rust bindings for OpenCV.

This is absolutely not ready for prime time.

Instead of binding the deprecated C-compatible interface, I chose to mimick the
python and java wrappers: parsing C++ headers, generating a C interface to the 
C++ api, and wrapping this in Rust.

## Current progress

To give you an idea of how far this is to be usable...

* Some functions and methods wrappers are skipped. The main reason being the
  vector and Ptr types. Coverage so far :

```
calib3d.txt:PORTED FUNCs LIST (31 of 38):
core.txt:PORTED FUNCs LIST (109 of 124):
features2d.txt:PORTED FUNCs LIST (29 of 54):
highgui.txt:PORTED FUNCs LIST (32 of 34):
imgproc.txt:PORTED FUNCs LIST (105 of 114):
objdetect.txt:PORTED FUNCs LIST (14 of 24):
photo.txt:PORTED FUNCs LIST (3 of 5):
video.txt:PORTED FUNCs LIST (18 of 21):
```

* No support for error handling yet.
* No safe, high-level Rust interface yet.
* No tests.
