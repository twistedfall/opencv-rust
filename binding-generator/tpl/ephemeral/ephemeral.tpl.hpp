#include "ocvrs_common.hpp"
{{includes}}

/* Dummy namespace that contains types that will be used in the bindings generator to resolve clang::Type from
 * string. Due to limitations of libclang it's not possible to resolve arbitrary strings. That's why we pre-add
 * typedefs that will be analyzed in the initial parsing step and added to the global cache that can resolve
 * e.g. "void" to a specific clang::Type.
 *
 * Only typedef's are analyzed in this namespace and each entry should alias distinct type, the name of the
 * typedef is ignored by the parser.
 */
namespace ocvrs_resolve_types {
	{{resolve_types}}
}

/* This is used to generate types otherwise not referenced in the OpenCV headers. It's useful for example to
 * generate Ptr<Child> when OpenCV function only accepts Ptr<Parent>.
 */
namespace cv {
	OCVRS_ONLY_DEPENDENT_TYPES void dependent_types_dummy(
		{{generate_types}}
	) {}
}
