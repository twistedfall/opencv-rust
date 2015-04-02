#include "/opt/local/include/opencv2/core/core.hpp"

using namespace cv;

extern "C" {
    void cv_delete_Mat(void* mat) {
        delete (cv::Mat*) mat;
    }
}
