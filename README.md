## Rust OpenCV bindings

This is my shot at generating Rust bindings for OpenCV.

This is absolutely not ready for prime time.

Instead of binding the deprecated C-compatible interface, I chose to mimick the
python and java wrappers: parsing C++ headers, generating a C interface to the 
C++ api, and wrapping this in Rust.

## Current progress, notes and todo list

To give you an idea of how far this is to be usable...

```
calib3d.txt:PORTED FUNCs LIST (36 of 38):
core.txt:PORTED FUNCs LIST (124 of 124):
features2d.txt:PORTED FUNCs LIST (55 of 59):
highgui.txt:PORTED FUNCs LIST (34 of 34):
imgproc.txt:PORTED FUNCs LIST (114 of 114):
objdetect.txt:PORTED FUNCs LIST (24 of 24):
photo.txt:PORTED FUNCs LIST (5 of 5):
video.txt:PORTED FUNCs LIST (20 of 21):
```

* Are there any other modules that should be added to the list ? Can I disco them ?
* Working with cv 2 here. What about version 3 ?
* No support for error handling yet
* No safe high-level Rust interface yet
* No tests
* Big doubts about what to do with cv::Ptr and std::vector. The current gen code
  discovers template instanciation and generate opaque void* wrappers with Drops
  but I'm not 100% this is right. Later.
* Discovered some troublesome types in flann. They do not conform to
  cv convention, so I think I'll just wrap them by hand and feedback their
  existence somehow to the generator.
