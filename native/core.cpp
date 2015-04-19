#include "opencv2/opencv_modules.hpp"
#include "opencv2/core/core.hpp"
#include "types.h" 

extern "C" {
    void* cv_core_Mat_create() { return new cv::Mat(); }
    void* cv_core_Mat_create_III(int w, int h, int t) { return new cv::Mat(w,h,t); }

    cv_struct_Size2i cv_core_Mat_size(void* mat) {
        cv::Size2i s = ((cv::Mat*) mat)->size();
        return *((cv_struct_Size2i*)&s);
    }
}
