#define CVRS_CATCH(return_val) \
catch (cv::Exception& e) { \
   char* msg = strdup(e.what()); \
   return_val ret; \
   memset(&ret, 0x00, sizeof(ret)); \
   ret.error_code = e.code; \
   ret.error_msg = msg; \
   return ret; \
} catch (...) { \
   char* msg = strdup("unspecified error in OpenCV guts"); \
   return_val ret; \
   memset(&ret, 0x00, sizeof(ret)); \
   ret.error_code = -99999; \
   ret.error_msg = msg; \
   return ret; \
}
