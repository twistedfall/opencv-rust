## Rust OpenCV bindings

This is my shot at generating Rust bindings for OpenCV.

This is absolutely not ready for prime time.

Instead of binding the deprecated C-compatible interface, I chose to mimick the
python and java wrappers: parsing C++ headers, generating a C interface to the 
C++ api, and wrapping this in Rust.

## Current progress, notes and todo list

Stuff that work (with examples):
* video capture
* conversions
* orb detection (and probably description too)

* Are there any other modules that should be added to the list ? Can I disco them ?
* Working with cv 2 here. What about version 3 ?
* Safe high-level Rust interface could be nicer
* No tests
* Discovered some troublesome types in flann. They do not conform to
  cv convention, so I think I'll just wrap them by hand and feedback their
  existence somehow to the generator.
