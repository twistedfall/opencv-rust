---
name: Bug report
about: Create a report to help us improve
title: ''
labels: ''
assignees: ''

---

Before reporting an issue please first check the
[troubleshooting guide](https://github.com/twistedfall/opencv-rust/blob/master/README.md#troubleshooting). If
the issue you're encountering is not solved thereby please state the following in your bugreport:
1. Operating system
2. The way you installed OpenCV: package, official binary distribution, manual compilation, etc.
3. OpenCV version
4. rustc version (`rustc --version`)
5. Attach the full output of the following command from your project directory:
   ```shell script
   RUST_BACKTRACE=full cargo build -vv 
   ```
