#include "common.hpp"
#include <vector>

/* Dummy namespace that contains types that will be used in the bindings generator to resolve clang::Type from
   string. Due to limitations of libclang it's not possible to resolve arbitrary strings. That's why we pre-add
   typedefs that will be analyzed in the initial parsing step and added to the global cache that can resolve
   e.g. "void" to a specific clang::Type.

   Only typedef's are analyzed in this namespace and each entry should alias distinct type, the name of the
   typedef is ignored by the parser.
*/
namespace ocvrs_resolve_types {
	typedef void core1; // void is used as return type for property setters

	// base types
	typedef bool core2;
	typedef int core3;
	typedef unsigned int core4;
	typedef double core5;
	typedef const char* core6;
	// return of String
	typedef void* core7;

	// handling vector of strings
	typedef std::vector<cv::String> core8;
	typedef std::vector<std::string> core9;

	// for return of vector<DataType> types
	typedef cv::_InputArray ioa1;
	typedef cv::_OutputArray ioa2;
	typedef cv::_InputOutputArray ioa3;
}
