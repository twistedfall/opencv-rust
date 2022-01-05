#!/bin/bash

set -e

C=bug-test
docker build -t "$C" .
docker run --rm -i -v "/home/pro/projects/opencv-rust:/root/opencv-rust" "$C" \
	/bin/bash -lc "cd /root/opencv-rust && cargo test -vv --test marshalling -- --nocapture"

# 'cv::saliency::ObjectnessBING::read' hides overloaded virtual function [-Woverloaded-virtual]
