#include "ocvrs_common.hpp"
{{includes}}

/* This is used to generate types otherwise not referenced in the OpenCV headers. It's useful for example to
 * generate Ptr<Child> when OpenCV function only accepts Ptr<Parent>.
 */
namespace cv {
	OCVRS_ONLY_DEPENDENT_TYPES void dependent_types_dummy(
		{{generate_types}}
	) {}
}
