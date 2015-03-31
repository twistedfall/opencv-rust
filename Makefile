CXXFLAGS=`pkg-config --cflags opencv` -g -std=c++11
LDLIBS=`pkg-config --libs opencv` -lpthread

libopencvc.a: core.o
	ar ru $@ $^
	ranlib $@


core.cpp: gen_rust.py
	python gen_rust.py hdr_parser.py core \
        /usr/local/include/opencv2/core/core.hpp \
        #/Users/kali/dev/github/opencv/modules/core/include/opencv2/core.hpp /Users/kali/dev/github/opencv/modules/core/include/opencv2/core/utility.hpp /Users/kali/dev/github/opencv/modules/core/misc/java/src/cpp/core_manual.hpp

