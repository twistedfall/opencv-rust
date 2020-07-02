#!/bin/bash

set -vex

cargo doc -vv --no-default-features --features=docs-only,contrib,opencv-4
