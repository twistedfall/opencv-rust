#define CODE_CATCH(return_type, exc_type, code, msg) \
catch (exc_type& e) { \
   return_type ret; \
   memset(&ret, 0x00, sizeof(ret)); \
   ret.error_code = code; \
   ret.error_msg = strdup(e.what()); \
   return ret; \
}

#define CVRS_CATCH(return_type) \
catch (cv::Exception& e) { \
   return_type ret; \
   memset(&ret, 0x00, sizeof(ret)); \
   ret.error_code = e.code; \
   ret.error_msg = strdup(e.what()); \
   return ret; \
} catch (...) { \
   return_type ret; \
   memset(&ret, 0x00, sizeof(ret)); \
   ret.error_code = -99999; \
   ret.error_msg = strdup("unspecified error in OpenCV guts"); \
   return ret; \
}

#define VEC_CATCH(return_type) CODE_CATCH(return_type, std::out_of_range, Error::Code::StsOutOfRange, "index out of bounds")
