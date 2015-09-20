import sys, re, os.path
import logging
import textwrap

from pprint import pformat
from string import Template

if sys.version_info[0] >= 3:
    from io import StringIO
else:
    from cStringIO import StringIO

#
#       EXCEPTIONS TO AUTO GENERATION
#

ManualFuncs = {
    "core" : [
         [ "class cv.Mat" , "", [], [] ],
         [ "cv.Mat.Mat", "Mat", [], [] ],
         [ "cv.Mat.Mat", "Mat", [],
            [ [ "int", "rows" ], [ "int", "cols" ], [ "int" , "type" ] ] ],
         [ "cv.Mat.depth", "int", ["/C"], [] ],
         [ "cv.Mat.type", "int", ["/C"], [] ],
         [ "cv.Mat.channels", "int", ["/C"], [] ],
         [ "cv.Mat.size", "Size", ["/C"], [] ],
         [ "cv.Mat.elemSize", "size_t", ["/C"], [] ],
         [ "cv.Mat.isContinuous", "bool", ["/C"], [] ],
         [ "cv.Mat.clone", "Mat", ["/C"], [] ],
         [ "cv.Mat.copyTo", "void", ["/C"], [["Mat", "OutputArray"]] ],
         [ "cv.Mat.convertTo", "void", ["/C"], [ ["Mat", "OutputArray"], ["int", "rtype"], ["double", "scale"]] ],
         [ "cv.Mat.ptr", "uchar*", ["/C"], [["int", "Row"]] ],
    ]
}

renamed_funcs = {
    "cv_core_cv_divide_MMMDI": "divide_mat",
    "cv_core_cv_norm_MMIM":"norm_dist",
    "cv_core_cv_ellipse_MPSDDDSIII": "ellipse_tilted",
    "cv_core_cv_Mat_Mat_III": "for_rows_and_cols",
    "cv_core_cv_Mat_type": "cv_type",
    "cv_calib3d_cv_StereoSGBM_StereoSGBM_IIIIIIIIIIB": "for_params",
    "cv_calib3d_cv_StereoBM_StereoBM_III": "for_params",
    "cv_features2d_cv_BOWKMeansTrainer_cluster_M": "cluster_with_desc",
    "cv_features2d_cv_BOWTrainer_cluster_M": "cluster_with_desc",
    "cv_features2d_cv_DescriptorMatcher_match_MMVM" : "matches",
    "cv_features2d_cv_DescriptorMatcher_match_MVV" : "matches",
    "cv_features2d_cv_KeyPoint_KeyPoint_FFFFFII" : "for_params",
    "cv_features2d_cv_DMatch_DMatch_IIF" : "for_params",
    "cv_features2d_cv_DMatch_DMatch_IIIF" : "for_image",
    "cv_features2d_cv_DescriptorMatcher_knnMatch_MMVIMB" : "knnTrainMatch",
    "cv_features2d_cv_DescriptorMatcher_match_MMVM": "trainAndMatch",
    "cv_features2d_cv_BRISK_BRISK_VVFFV" : "for_pattern",
    "cv_highgui_cv_VideoWriter_VideoWriter_SIDSB" : "for_params",
    "cv_highgui_cv_VideoCapture_VideoCapture_S" : "for_file",
    "cv_highgui_cv_VideoCapture_VideoCapture_I" : "for_device",
    "cv_highgui_cv_VideoCapture_open_S" : "open_file",
    "cv_highgui_cv_VideoCapture_open_I" : "open_fd",
    "cv_imgproc_cv_integral_MMMI" : "integral_squares",
    "cv_imgproc_cv_integral_MMMMI" : "integral_squares_tilted",
    "cv_imgproc_cv_distanceTransform_MMMIII" : "distance_tranform_labels",
    "cv_imgproc_cv_Subdiv2D_Subdiv2D_R" : "for_rect",
    "cv_imgproc_cv_Subdiv2D_insert_V" : "insert_multi",
    "cv_objdetect_cv_HOGDescriptor_HOGDescriptor_S": "for_file",
    "cv_objdetect_cv_HOGDescriptor_HOGDescriptor_SSSSIIDIDBI": "for_params",
    "cv_objdetect_cv_CascadeClassifier_detectMultiScale_MVVVDIISSB" : "detectMultiScaleFull",
    "cv_objdetect_cv_CascadeClassifier_CascadeClassifier_S": "for_file",
    "cv_video_cv_calcOpticalFlowSF_MMMIIIDDIDDDIDDD" : "calc_optical_flow_full",
    "cv_video_cv_KalmanFilter_KalmanFilter_IIII" : "for_params",
    "cv_video_cv_BackgroundSubtractorMOG_BackgroundSubtractorMOG_IIDD" : "for_params",
    "cv_video_cv_BackgroundSubtractorMOG2_BackgroundSubtractorMOG2_IFB" : "for_params",
}

class_ignore_list = (
    #core
    "CvMat", "CvArr", "CvSeq", "CvPoint.*", "CvRect", "CvTermCriteria", # have c++ equiv
    "CvSize", "CvSlice", "CvScalar",
    "Cv[A-Z].*",
    "cv::Param",
    "cv::Mat::MStep",
    "cv::Mat::MSize",
    "cv::Mutex",
    "Ipl.*",
    "BinaryFunc", "ConvertData", "ConvertScaleData",
    "cv::FileNode", "cv::FileStorage", "cv::FileNodeIterator",
    "cv::KDTree", "IndexParams", "Params", "CvAttrList", "WString",
    "cv::Exception", "cv::ErrorCallback",
    "cv::RNG.*", # maybe
    "cv::SVD",
    "cv::TLSDataContainer",
    "NAryMatIterator",
    "cv::MatConstIterator",
    "cv::CommandLineParser",
    "cv::_InputArray", "cv::_OutputArray",
    "OutputArrayOfArrays", "InputArrayOfArrays", # FIXME ?
    "cv::MatAllocator",
    "cv::SparseMat",
    "cv::AlgorithmInfo",
    #videoio
#    "VideoWriter",
    # imgproc
    "Vertex", "QuadEdge",
    "GeneralizedHough",
    "cv::BaseColumnFilter", "cv::BaseRowFilter", "cv::BaseFilter", # abstract
    "Subdiv2D", # lots of protected stuff exported (may work now)
    # features
    "cv::GenericDescriptorMatcher", # abstract
    "DescriptorCollection", "KeyPointCollection", # nested
    "cv::BOWTrainer", # abstract
)

func_ignore_list = (
    "cv.glob", "cv.fastFree", "cv.fastMalloc",
    "cv.getBuildInformation", "cv.scalarToRawData", "cv.noArray", "()", "cv.Mat.MSize.operator[]",
    "const int*", "=", "==", "!=", "--", "++", "*", ">>", "<<", "<", ">", "operator==", "operator()",
    "cv.Mat.MStep.operator[]",
    "cv.swap",
    "cv.minMaxLoc", "cv.minMaxIdx", # return prims by pointer
    "cv.merge", # pointer to array of matrix
    "cv.split",
    "cv.mixChannels", "cv.insertChannel",
    "cv.hconcat", "cv.vconcat", "cv.repeat", # maybe: repeat(*((const Mat&*)src), ny, nx)
    "cv.min", "cv.max", "cv.exp", "cv.log", "cv.fastAtan2", # return prims by pointer (may be make to work)
    "cv.magnitude", "cv.patchNaNs", "cv.setIdentity", "cv.completeSymm", "cv.calcCovarMatrix",
    "cv.fillConvexPoly", "cv.fillPoly", "cv.polylines", # Point**
    "cv.getTextSize", # return prim by ptr
    "cv.SparseMat.Hdr.clear",
    "cv.PCA.computeVar", # what ?
    "cvSetPreprocessFuncWin32_", "cvSetPostprocessFuncWin32_",
    # features
    "cv.BOWImgDescriptorExtractor.getVocabulary",
)

const_ignore_list = (
    "CV_EXPORTS_W", "CV_EXPORTS_W_SIMPLE", "CV_EXPORTS_W_MAP", "CV_MAKE_TYPE",
    "CV_IS_CONT_MAT", "CV_RNG_COEFF", "IPL_IMAGE_MAGIC_VAL",
    "CV_SET_ELEM_FREE_FLAG", "CV_FOURCC_DEFAULT",
    "CV_WHOLE_ARR", "CV_WHOLE_SEQ", "CV_PI", "CV_LOG2",
    "CV_TYPE_NAME_IMAGE", 

)

func_arg_fix = {
}

#
#       TYPES MAPPING
#

primitives = {
    u"void"  : { u"ctype": "void", "rtype": "()" },
    u"bool"  : { u"ctype": "int", u"rtype": "bool" },
    u"uchar" : { u"ctype": "unsigned char", u"rtype": "u8" },
    u"char"  : { u"ctype": "char", u"rtype": "i8" },
    u"short" : { u"ctype": "short", u"rtype": "u16" },
    u"int"   : { u"ctype": "int", u"rtype": "i32" },
    u"uint"  : { u"ctype": "unsigned int", u"rtype": "u32" },
    u"size_t": { u"ctype": "std::size_t", u"rtype": "::libc::types::os::arch::c95::size_t" },
    u"int64" : { u"ctype": "int64", u"rtype": "i64" },
    u"float" : { u"ctype": "float", u"rtype": "f32" },
    u"double": { u"ctype": "double", u"rtype": "f64" },
    u"uchar*": { u"ctype": "unsigned char*", u"rtype": "*mut u8" }
}

# trait_classes = [ "Algorithm" ]

forced_boxed_classes = { }

value_struct_types = {
    "cv::Point" : (("x", "int"), ("y", "int")),
    "cv::Point2d" : (("x", "double"), ("y", "double")),
    "cv::Point2f" : (("x", "float"), ("y", "float")),
    "cv::Size" : (("width", "int"), ("height", "int")),
    "cv::Size2f" : (("width", "float"), ("height", "float")),
    "cv::Rect" : (("x", "int"), ("y", "int"), ("width", "int"), ("height", "int")),
    "cv::RotatedRect" : (("x", "float"), ("y", "float"), ("width", "float"),("height", "float"), ("angle", "float")),
    "cv::TermCriteria" : (("type", "int"), ("maxCount", "int"), ("epsilon", "double")),
    "cv::Scalar" : (("data", "double[4]"),),
    "CvRNG" : (("data", "int64"),)
}

for s in [2,3,4,6]:
    for t in [("uchar","b"),("short","s"),("int","i"),("double","d"),("float","f")]:
        value_struct_types["cv::Vec%d%s"%(s,t[1])] = ("data", "%s[%d]"%(t[0],s)),

#
#       TEMPLATES
#

T_CPP_MODULE = """
//
// This file is auto-generated, please don't edit!
//

#define LOG_TAG "org.opencv.$m"

#include "stdint.h"
#include "common.h"

typedef int64_t int64;

#include "types.h"
#include <iostream>

#include "return_types.h"

#include "opencv2/opencv_modules.hpp"
#ifdef HAVE_OPENCV_$M

#include <string>

#include "opencv2/$m/$m.hpp"

$namespaces

$includes

extern "C" {

$code

} // extern "C"

#endif // HAVE_OPENCV_$M
"""

T_RUST_MODULE = """
//
// This file is auto-generated, please don't edit!
//


use ::sys::$m::*;

pub mod $m {
use sys::types::*;
use std::ffi::{ CStr, CString };
use std::mem::transmute;
use libc::types::common::c95::c_void;

$module_import
$code
}
"""

const_private_list = (
    "CV_MOP_.+",
    "CV_INTER_.+",
    "CV_THRESH_.+",
    "CV_INPAINT_.+",
    "CV_RETR_.+",
    "CV_CHAIN_APPROX_.+",
    "OPPONENTEXTRACTOR",
    "GRIDDETECTOR",
    "PYRAMIDDETECTOR",
    "DYNAMICDETECTOR",
)

#
#       AST-LIKE
#

class GeneralInfo():
    def __init__(self, gen, name, namespaces):
        self.gen = gen
        self.fullname, self.namespace, self.classpath, self.classname, self.name = self.parseName(name, namespaces)

    def parseName(self, name, namespaces):
        r = self.doParseName(name,namespaces)
        logging.info("parseName: %s with %s -> fullname:%s namespace:%s classpath:%s classname:%s name:%s"%(name, namespaces, \
            r[0], r[1], r[2], r[3], r[4]))
        return r

    def doParseName(self, name, namespaces):
        '''
        input: full name and available namespaces
        returns: (fullname, namespace, classpath, classname, name)
            fullname clean of prefix like "const, class, ..."
        '''
        name = name.replace("const ", "").replace("struct " , "").replace("class ","").replace(".", "::")
        spaceName = ""
        localName = name # <classes>.<name>
        for namespace in sorted(namespaces, key=len, reverse=True):
            if name.startswith(namespace + "::"):
                spaceName = namespace
                localName = name.replace(namespace + "::", "")
                break
        pieces = localName.split("::")
        if len(pieces) > 2: # <class>.<class>.<class>.<name>
            return name, spaceName, "::".join(pieces[:-1]), pieces[-2], pieces[-1]
        elif len(pieces) == 2: # <class>.<name>
            return name, spaceName, pieces[0], pieces[0], pieces[1]
        elif len(pieces) == 1: # <name>
            return name, spaceName, "", "", pieces[0]
        else:
            return name, spaceName, "", "" # error?!

class ArgInfo():
    def __init__(self, gen, arg_tuple): # [ ctype, name, def val, [mod], argno ]
        self.gen = gen
        self.pointer = False
        type = arg_tuple[0]
        if type.endswith("*"):
            type = type[:-1]
            self.pointer = True
        self.type = self.gen.get_type_info(type)
        self.name = arg_tuple[1]
        self.defval = ""
        self.typeinfo = None
        if len(arg_tuple) > 2:
            self.defval = arg_tuple[2]
        self.out = ""
        if len(arg_tuple) > 3 and "/O" in arg_tuple[3]:
            self.out = "O"
        if len(arg_tuple) > 3 and "/IO" in arg_tuple[3]:
            self.out = "IO"

    def rsname(self):
        rsname = self.name
        if rsname in ["type","box"]:
            rsname = "_" + rsname
        return rsname


    def __repr__(self):
        return template("ARG $ctype$p $name=$defval").substitute(ctype=self.type,
                                                                  p=" *" if self.pointer else "",
                                                                  name=self.name,
                                                                  defval="" #self.defval
                                                                )


class FuncInfo(GeneralInfo):

    KIND_FUNCTION    = "(function)"
    KIND_METHOD      = "(method)"
    KIND_CONSTRUCTOR = "(constructor)"

    def __init__(self, gen, decl, namespaces=[]): # [ funcname, return_ctype, [modifiers], [args] ]
        GeneralInfo.__init__(self, gen, decl[0], namespaces)

        if self.classname:
            self.ci = gen.get_class(self.classname)
            if not self.ci:
                raise NameError("class not found: " + self.classname)
            if self.classname == self.name:
                self.kind = self.KIND_CONSTRUCTOR
                self.name = "new"
                self.type = gen.get_type_info(self.classname)
            else:
                self.kind = self.KIND_METHOD
                self.type =  gen.get_type_info(decl[1])
        else:
            self.kind = self.KIND_FUNCTION
            self.ci = None
            self.type = gen.get_type_info(decl[1])

        self.args = []

        self.overridename = self.name
        for m in decl[2]:
            if m.startswith("="):
                self.overridename = m[1:]

        for a in decl[3]:
            self.args.append(ArgInfo(gen, a))
        self.const = "/C" in decl[2]
        self.static = ["","static"][ "/S" in decl[2] ]

        self.cname = self.cppname = self.name

        if self.name.startswith("~"):
            logging.info("ignore destructor %s %s in %s"%(self.kind, self.name, self.ci))
            return

        if self.name.startswith("operator"):
            logging.info("ignore %s %s in %s"%(self.kind, self.name, self.ci))
            return

        # register self to class or generator
        self.discriminator = 0
        if self.kind == self.KIND_FUNCTION:
            logging.info("register %s %s"%(self.kind, self.name))
            gen.register_function(self)
        else:
            logging.info("register %s %s in %s"%(self.kind, self.name, self.ci))
            self.ci.add_method(self)

    def isconstructor(self):
        return self.kind == self.KIND_CONSTRUCTOR

    def rv_type(self):
        if self.isconstructor():
            if self.ci:
                return self.gen.get_type_info(self.ci.nested_cppname)
            else:
                return None
        else:
            return self.type

    def reason_to_skip(self):
        if self.overridename == "operator ()":
            return "can not map operator () yet"

        for f in func_ignore_list:
            if self.fullname.endswith(f):
                return "manual ignore"

        if not self.rv_type():
            return "rv_header_type returns None. this is an error. (class not found ?)"

        if self.type.is_ignored:
            return "class is ignored"

        if self.rv_type().is_ignored:
            return "return value type is ignored"

        for a in self.args:
            if a.type.is_ignored:
                return "can not map type %s yet"%(a.type)
            if a.pointer and a.type.is_by_value:
                return "returning primitive by pointer is not supported (FIXME ?)"
            if a.pointer and a.type.typeid.endswith("*"):
                return "** not supported yet"
            if a.type.typeid.endswith("]"):
                return "passing raw arrays will wait (FIXME ?)"
            if a.type.typeid == "const char" and a.pointer:
                return "const char* will wait"
            if a.type.typeid == "...":
                return "variadic will have to wait"

        return None

    def gen_cpp_prelude(self):
        io = StringIO()
        io.write("// parsed: %s\n"%(self.fullname))
        io.write("// as:     %s\n"%(self))
        for a in self.args:
            ignored = ptr = ""
            if a.type.is_ignored:
                ignored = "(ignored)"
            if a.pointer:
                ptr = "(ptr)"
            io.write("// Arg %s %s %s =%s %s\n"%(a.name, ptr, a.type, a.defval, ignored))
        io.write("// Return value: %s\n"%(self.rv_type()))
        return io.getvalue()

    def c_name(self):
        if self.discriminator != 0:
            suffix = "_%d"%(self.discriminator)
        else:
            suffix = ""
        if self.ci == None:
            return "cv_%s_%s%s"%(self.gen.module, self.overridename, suffix);
        else:
            return "cv_%s_%s_%s%s"%(self.gen.module, self.ci.nested_cname, self.overridename, suffix);

    def r_name(self):
        if self.discriminator != 0:
            suffix = "_%d"%(self.discriminator)
        else:
            suffix = ""
        rname = renamed_funcs.get(self.c_name()) or ("new" if self.isconstructor() else self.overridename)
        return rname + suffix

    # "const", "mut", or None
    def instance(self):
        if not self.ci == None and not self.isconstructor():
            return "const" if self.const else "mut"
        return None

    def gen_rust_extern(self):
        rust_extern_rs = "rv::cv_return_value_%s"%(self.rv_type().ctype.replace("*","_").replace(" ","_").replace(":","_"))

        args = []
        if self.instance():
            args.append("instance: *%s c_void"%(self.instance()))
        for a in self.args:
            args.append(a.rsname() + ": " + (a.type.rctype or a.type.rtype))

        return "pub fn %s(%s) -> %s;\n"%(self.c_name(), ", ".join(args), rust_extern_rs)

    def gen_safe_rust(self):
        args = []
        call_args = []
        if self.instance() == "const":
            args.append("&self")
            call_args.append("self.as_ptr()")
        elif self.instance() == "mut":
            args.append("&mut self")
            call_args.append("self.as_ptr()")

        for a in self.args:
            if isinstance(a.type,StringTypeInfo):
                args.append("%s:&str"%(a.rsname()))
            elif a.type.is_by_value:
                args.append(a.rsname() + ": " + a.type.rtype)
            elif a.out == "O" or a.out == "IO":
                args.append(a.rsname() + ":&mut " + a.type.rtype)
            else:
                args.append(a.rsname() + ":& " + a.type.rtype)

            if isinstance(a.type, BoxedClassTypeInfo) or a.type.is_by_ptr:
                call_args.append("%s.ptr"%(a.rsname()))
            elif isinstance(a.type,StringTypeInfo):
                call_args.append("CString::new(%s).unwrap().as_ptr()"%(a.rsname()))
            else:
                call_args.append("%s"%(a.rsname()))

        pub = "" if self.ci and self.ci.type_info().is_trait else "pub "

        io = StringIO()
        io.write("/// %s\n"%(self))
        for a in self.args:
            if a.defval != "":
                io.write("/// * %s: default %s\n"%(a.rsname(), a.defval))
        io.write("%sfn %s(%s) -> Result<%s,String> {\n"%(pub, self.r_name(), ", ".join(args), self.rv_type().rrvtype or self.rv_type().rtype))
        io.write("  unsafe {\n")
        io.write("    let rv = ::%s(%s);\n"%(self.c_name(), ", ".join(call_args)))
        io.write("    if rv.error_msg as i32 != 0i32 {\n")
        io.write("        let v = CStr::from_ptr(rv.error_msg).to_bytes().to_vec();\n");
        io.write("        ::libc::free(rv.error_msg as *mut c_void);\n")
        io.write("        return Err(String::from_utf8(v).unwrap())\n")
        io.write("    }\n");
        if self.type == "void":
            io.write("    Ok(())\n");
        elif isinstance(self.rv_type(), StringTypeInfo):
            io.write("    let v = CStr::from_ptr(rv.result).to_bytes().to_vec();\n");
            io.write("    ::libc::free(rv.result as *mut c_void);\n");
            io.write("    Ok(String::from_utf8(v).unwrap())\n");
        elif isinstance(self.rv_type(), BoxedClassTypeInfo):
            io.write("    Ok(%s{ ptr: rv.result })\n"%(self.rv_type().rtype))
        elif self.type == "bool":
            io.write("    Ok(rv.result!=0)\n")
        else:
            io.write("    Ok(rv.result)\n")
        io.write("  }\n");
        io.write("}\n\n")

        block = io.getvalue()
        if self.kind == self.KIND_FUNCTION:
            return block
        else:
            return re.sub("^(.+)$", "  \\1", block, flags=re.M)

    def __repr__(self):
        if self.kind == self.KIND_FUNCTION:
            return "%s %s"%(self.fullname, self.kind)
        else:
            return "%s %s %s . %s"%(self.fullname, self.kind, self.ci, self.name)

class ClassPropInfo():
    def __init__(self, decl): # [f_ctype, f_name, '', '/RW']
        self.ctype = decl[0]
        self.name = decl[1]
        self.rw = "/RW" in decl[3]

    def __repr__(self):
        return template("PROP $ctype $name").substitute(ctype=self.ctype, name=self.name)

class ClassInfo(GeneralInfo):
    def __init__(self, gen, decl, namespaces=[]): # [ 'class/struct cname', ': base', [modlist] ]
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.methods = {}
        self.simple = False
        self.is_ignored = False
        self.classname = self.name
        for m in decl[2]:
            if m == "/Simple" or m == "/Map" :
                self.simple = True
            if m == "/Hidden":
                self.is_ignored = True
        if self.classpath and gen.get_class(self.classpath).is_ignored:
            self.is_ignored = True

        name = decl[0].replace("struct ", "").replace("class ","").replace("const ","")
        self.nested_cppname = name.replace(".", "::")
        self.nested_cname = name.replace(".", "_")

        self.bases = decl[1][1:].strip()
        if len(self.bases):
            self.bases = map(lambda x:x.strip(), self.bases.split(","))
        else:
            self.bases = []

        # class props
        self.props= []
        for p in decl[3]:
            self.props.append( ClassPropInfo(p) )

        self.is_ignored = self.is_ignored or self.gen.class_is_ignored(self.nested_cppname)

        # register
        logging.info("register class %s (%s)%s%s", self.nested_cppname, decl,
            " [ignored]" if self.is_ignored else "",
            " impl:"+",".join(self.bases) if len(self.bases) else "")
        gen.classes[self.nested_cppname] = self


    def __repr__(self):
        return self.nested_cppname

    def add_method(self, fi):
        if self.methods.get(fi.name) is None:
            self.methods[fi.name] = []
        fi.discriminator = len(self.methods[fi.name])
        self.methods[fi.name].append(fi)

    def getAllMethods(self):
        result = []
        result.extend([fi for fi in sorted(self.methods) if fi.isconstructor()])
        result.extend([fi for fi in sorted(self.methods) if not fi.isconstructor()])
        return result

    def has_constructor(self):
        for fis in self.methods.values():
            for fi in fis:
                if fi.isconstructor():
                    return True
        return False

    def type_info(self):
        return self.gen.get_type_info(self.nested_cppname)

class ConstInfo(GeneralInfo):
    def __init__(self, gen, decl, addedManually=False, namespaces=[]):
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        if len(self.fullname.split("::")) > 1:
            self.rustname = "_".join(self.fullname.split("::")[1:])
        else:
            self.rustname = self.fullname
        self.cname = self.name.replace(".", "::")
        self.value = decl[1]
        self.addedManually = addedManually

        # register
        if self.isIgnored():
            logging.info('ignored: %s', self)
        elif not gen.get_const(self.name):
            gen.consts.append(self)

    def __repr__(self):
        return template("CONST $name=$value$manual").substitute(name=self.name,
                                                                 value=self.value,
                                                                 manual="(manual)" if self.addedManually else "")

    def isIgnored(self):
        for c in const_ignore_list:
            if re.match(c, self.name):
                return True
        return False

    def gen_rust(self):
        if self.value.startswith('"'):
            return "pub const %s:&'static str = %s;\n"%(self.rustname, self.value)
        elif re.match("^(-?[0-9]+|0x[0-9A-F]+)$", self.value):
            return "pub const %s:i32 = %s;\n"%(self.rustname, self.value)
        return None

    def gen_cpp_for_complex(self):
        # only use C-constant dumping for unnested const
        if len(self.fullname.split(".")) > 2:
            return ""
        else:
            return """    printf("pub const %s:i32 = 0x%%x;\\n", %s);\n"""%(self.rustname, self.fullname)

class TypeInfo:
    def __init__(self, gen, typeid):
        self.typeid = typeid
        self.gen = gen
        self.is_ignored = False
        self.is_ignored = False
        self.is_by_ptr = False
        self.is_by_value = False
        self.is_trait = False # FIXME

class StringTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.ctype = "const char*"
        self.cpptype = "string"
        self.rtype = self.rctype = "*const ::libc::types::os::arch::c95::c_char"
        self.rrvtype = "String"

    def __str__(self):
        return "string"

class IgnoredTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_ignored = True

    def __str__(self):
        return "Ignored(%s)"%(self.typeid)

class PrimitiveTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_value = True
        self.ctype = primitives[typeid]["ctype"]
        self.cpptype = typeid
        self.rtype = self.rctype = self.rrvtype = primitives[typeid]["rtype"]

    def __str__(self):
        return "Primitive(%s)"%(self.cpptype)

class EmptyTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.ctype = self.cpptype = self.rtype = self.rctype = self.rrvtype = "void"

class SimpleClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_value = True
        self.cpptype = typeid
        self.rtype = self.rctype = self.rrvtype = typeid.replace("::","_")
        self.ctype = "struct_" + self.rctype
        self.is_trait = False # FIXME

    def __str__(self):
        return "SimpleClass(%s)"%(self.cpptype)

class BoxedClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.ci = gen.get_class(typeid)
        self.ctype = "void*"
        self.cpptype = self.ci.nested_cppname
        self.rtype = self.rrvtype = self.ci.classname
        self.rctype = "*mut c_void"
        self.is_by_ptr = True
        self.is_trait = False # FIXME
        self.is_ignored = self.ci.is_ignored

    def __str__(self):
        return "Boxed[%s]"%(self.typeid)

class VectorTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_ptr = True
        self.inner = inner
        self.is_ignored = inner.is_ignored
        if not self.is_ignored:
            self.ctype = "void*"
            self.cpptype = "vector<%s >"%(inner.cpptype)
            self.rctype = "*mut c_void"
            self.rtype = self.rrvtype = "VectorOf"+inner.rrvtype
            self.gen_template_wrapper_rust_struct()

    def gen_template_wrapper_rust_struct(self):
        with open(self.gen.output_path+"/"+self.rtype+".type.rs", "w") as f:
            f.write(template("""
                extern "C" {
                    fn cv_new_$rtype() -> *mut c_void;
                    fn cv_delete_$rtype(ptr:*mut c_void) -> ();
                    fn cv_${rtype}_len(ptr:*mut c_void) -> i32;
                    fn cv_${rtype}_data(ptr:*mut c_void) -> *mut c_void;
                }
                #[allow(dead_code)] pub struct $rtype {
                    pub ptr: *mut c_void
                }
                impl $rtype {
                    pub fn new() -> $rtype {
                        unsafe { return $rtype { ptr:cv_new_$rtype() } };
                    }
                    pub fn len(&self) -> i32 {
                        unsafe { return cv_${rtype}_len(self.ptr); }
                    }
                }
                impl ::std::ops::Deref for $rtype {
                    type Target = [$output_rtype];
                    fn deref(&self) -> &[$output_rtype] {
                        unsafe {
                            let length = cv_${rtype}_len(self.ptr) as usize;
                            let data = cv_${rtype}_data(self.ptr);
                            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
                        }
                    }
                }
                impl Drop for $rtype {
                    fn drop(&mut self) {
                        unsafe { cv_delete_$rtype(self.ptr) };
                    }
                }\n""").substitute(rtype=self.rtype, output_rtype=self.inner.rtype))
        with open(self.gen.output_path+"/"+self.rtype+".type.cpp", "w") as f:
            f.write(template("""
                #include "opencv2/opencv_modules.hpp"
                #include "opencv2/$module/$module.hpp"
                #include "types.h"
                using namespace cv;
                extern "C" { 
                    void* cv_new_$rtype() { return new std::$cpptype(); }
                    void cv_delete_$rtype(void* ptr) { delete (($cpptype*) ptr); }
                    int cv_${rtype}_len(void* ptr) { return (($cpptype*) ptr)->size(); }
                    $output_ctype* cv_${rtype}_data(void* ptr) {
                        return ($output_ctype*) ((($cpptype*) ptr)->data());
                    }
                }\n""").substitute(
            rtype=self.rtype, cpptype=self.cpptype, module=self.gen.module,
            output_ctype=self.inner.ctype))

class SmartPtrTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_ptr = True
        self.inner = inner
        self.is_ignored = self.inner.is_ignored
        if not self.is_ignored:
            self.ctype = "void*"
            self.rctype = "*mut c_void"
            self.cpptype = "Ptr<%s>"%(self.inner.cpptype)
            self.rtype = self.rrvtype = "PtrOf" + inner.rtype
            self.gen_template_wrapper_rust_struct()

    def gen_template_wrapper_rust_struct(self):
        with open(self.gen.output_path+"/"+self.rtype+".type.rs", "w") as f:
            f.write("// safe rust wrapper for %s\n"%(self))
            f.write("#[allow(dead_code)] pub struct %s { pub ptr: *mut c_void }\n"%(self.rtype));

    def __str__(self):
        return "SmartPtr[%s]"%(self.inner)

class RawPtrTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        TypeInfo.__init__(self,gen,typeid)
        self.inner = inner
        self.is_ignored = True

    def __str__(self):
        return "RawPtr[%s]"%(self.inner)

class UnknownTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_ignored = True
        logging.info("Registering an unknown type: %s", typeid)

    def __str__(self):
        return "Unknown[%s]"%(self.typeid)

class ReferenceTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        TypeInfo.__init__(self,gen,typeid)
        self.inner = inner
        self.is_ignored = True

def parse_type(gen, typeid):
    typeid = typeid.strip()
    if typeid == "unsigned":
        typeid = "uint"
    typeid = typeid.replace("const ", "").replace("..", ".")
    if typeid.endswith("&"):
        return ReferenceTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-1]))
    elif typeid.endswith("*"):
        return RawPtrTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-1]))
    elif typeid == "string":
        return StringTypeInfo(gen,typeid)
    elif typeid in primitives:
        return PrimitiveTypeInfo(gen, typeid)
    elif typeid == "":
        return EmptyTypeInfo(gen, typeid)
    elif typeid.startswith("Ptr<"):
        return SmartPtrTypeInfo(gen, typeid, gen.get_type_info(typeid[4:-1]))
    elif typeid.startswith("vector<"):
        inner = gen.get_type_info(typeid[7:-1])
        if not inner:
            raise NameError("inner type `%s' not found"%(typeid[7:-1]))
        return VectorTypeInfo(gen, typeid, inner)
    elif gen.get_value_struct(typeid):
        return SimpleClassTypeInfo(gen, gen.get_value_struct(typeid))
    elif gen.class_is_ignored(typeid):
        return IgnoredTypeInfo(gen,typeid)
    else:
        ci = gen.get_class(typeid)
        if ci:
            return BoxedClassTypeInfo(gen, ci.nested_cppname)
    return UnknownTypeInfo(gen, typeid)

#
#       GENERATOR
#

def template(text):
    return Template(textwrap.dedent(text))

class RustWrapperGenerator(object):
    def __init__(self):
        self.clear()


    def clear(self):
        self.module = ""
        self.Module = ""
        self.classes = { }
        self.functions = { };
        self.ported_func_list = []
        self.skipped_func_list = []
        self.consts = []
        self.type_infos = {}

    def get_class(self, classname):
        print("lookup class "+classname)
        c = self.classes.get(classname)
        if c:
            return c
        for c in self.classes.values():
            print("  might be "+c.fullname)
            if c.fullname.endswith("::"+classname):
                return c
        return None

    def get_value_struct(self, classname):
        c = value_struct_types.get(classname)
        if c:
            return classname
        for c in value_struct_types.keys():
            if c.endswith("::" + classname):
                return c
        return None

    def get_type_info(self, typeid):
        if not typeid in self.type_infos:
            self.type_infos[typeid] = parse_type(self, typeid)
        return self.type_infos[typeid]

    def get_const(self, name):
        for c in self.consts:
            if c.cname == name:
                return c
        return None

    def add_decl(self, decl):
        if decl[0] == "cv.Algorithm":
            decl[0] = "cv.Algorithm.Algorithm"
        name = decl[0]
        if name.startswith("struct") or name.startswith("class"):
            ClassInfo(self, decl, namespaces=self.namespaces)
        elif name.startswith("const"):
            ConstInfo(self, decl, namespaces=self.namespaces)
        else:
            if not "/H" in decl[2]:
                FuncInfo(self, decl, namespaces=self.namespaces)

    def register_function(self, f):
        if self.functions.get(f.cname) is None:
            self.functions[f.cname] = []
        f.discriminator = len(self.functions[f.cname])
        self.functions[f.cname].append(f)

    def gen(self, srcfiles, module, output_path):
        parser = hdr_parser.CppHeaderParser()
        self.output_path = output_path
        self.module = module
        self.Module = module.capitalize()
        includes = [];

        self.namespaces = parser.namespaces
        self.namespaces.add("cv")

        if module in ManualFuncs:
            for decl in ManualFuncs[self.module]:
                logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                self.add_decl(decl)

        for hdr in srcfiles:
            decls = parser.parse(hdr, False)
            self.namespaces = map(lambda n:n.replace(".", "::"), parser.namespaces)
            logging.info("\n\n===== Header: %s =====", hdr)
            logging.info("Namespaces: %s", parser.namespaces)
            if decls:
                includes.append('#include "' + hdr + '"')
            for decl in decls:
                logging.info("\n--- Incoming ---\n%s", pformat(decl, 4))
                self.add_decl(decl)

        logging.info("\n\n===== Generating... =====")
        self.moduleCppTypes = StringIO()
        self.moduleCppCode = StringIO()
        self.moduleCppConsts = StringIO()
        self.moduleSafeRust = StringIO()
        self.moduleRustExterns = StringIO()

        for co in sorted(self.consts, key=lambda c: c.rustname):
            rust = co.gen_rust()
            if rust:
                self.moduleSafeRust.write(rust)
            else:
                self.moduleCppConsts.write(co.gen_cpp_for_complex())

        self.moduleSafeRust.write("\n");

        if self.moduleCppConsts.getvalue != "":
            self.moduleSafeRust.write(
                """include!(concat!(env!("OUT_DIR"), "/%s.consts.rs"));\n\n"""%(self.module)
            )

        for ci in sorted(self.classes.values(), key=lambda ci: ci.fullname):
            if ci.classpath:
                self.gen_nested_class_decl(ci)

        if module == "core":
            for c in sorted(value_struct_types, key= lambda c: c[0]):
                self.gen_value_struct(c)

        for c in self.classes.values():
            if c.simple and not c.is_ignored:
                self.gen_simple_class(c)

        for fis in sorted(self.functions.values(), key=lambda fis: fis[0].fullname):
            for fi in fis:
                self.gen_func(fi)

        if module in forced_boxed_classes:
            for cb in sorted(forced_boxed_classes[module]):
                self.gen_boxed_class(cb)

        for ci in sorted(self.classes.values(), key=lambda ci:ci.fullname):
            if not ci.is_ignored:
                self.gen_class(ci)

        with open(output_path+"/types.h", "a") as f:
            f.write(self.moduleCppTypes.getvalue())

        with open(output_path+"/" + self.module + ".consts.cpp", "w") as f:
            f.write("""#include <cstdio>\n""")
            f.write("""#include "opencv2/opencv_modules.hpp"\n""")
            f.write("""#include "opencv2/%s/%s.hpp"\n"""%(module,module))
            f.write("""using namespace cv;\n""")
            f.write("int main(int argc, char**argv) {\n");
            f.write(self.moduleCppConsts.getvalue())
            f.write("}\n");

        namespaces = ""
        for namespace in self.namespaces:
            if namespace != "":
                namespaces += "using namespace %s;\n"%(namespace.replace(".", "::"))
        with open(output_path+"/"+module+".cpp", "w") as f:
            f.write(template(T_CPP_MODULE).substitute(m = module, M = module.upper(), code = self.moduleCppCode.getvalue(), includes = "\n".join(includes), namespaces=namespaces))

        with open(output_path+"/%s.externs.rs"%(module), "w") as f:
            f.write("extern \"C\" {\n")
            f.write(self.moduleRustExterns.getvalue())
            f.write("}\n")

        with open(output_path+"/"+module+".rs", "w") as f:
            f.write(template(T_RUST_MODULE).substitute(m = module, M = module.upper(), code = self.moduleSafeRust.getvalue(), module_import = ("use ::sys::core::*;\n" if not module == "core" else "")))

        with open(output_path+"/"+module+".txt", "w") as f:
            f.write(self.makeReport())

    def makeReport(self):
        '''
        Returns string with generator report
        '''
        report = StringIO()
        total_count = len(self.ported_func_list)+len(self.skipped_func_list)
        report.write("FOUND FUNCs: %i\n\n" % (total_count))
        report.write("PORTED FUNCs: %i\n\n" % (len(self.ported_func_list)))
        for f in self.ported_func_list:
            report.write("PORTED: " + f + "\n")
        if len(self.skipped_func_list) > 0:
            report.write("\n\nSKIPPED FUNCs: %i\n\n" % (len(self.skipped_func_list)))
            for f in self.skipped_func_list:
                report.write("SKIPPED: " + f + "\n")
        return report.getvalue()

    def class_is_ignored(self, type_name):
        for c in class_ignore_list:
            if re.match(c, type_name):
                return True
        return False

    def gen_vector_struct_for(self, name):
        struct_name = "cv_vector_of_"+name
        self.defined_in_types_h.appand(struct_name)
        self.moduleCppTypes.write

    def gen_func(self, fi):
        logging.info("Generating %s"%(fi))
        reason = fi.reason_to_skip()
        if reason:
            logging.info("  ignored: " + reason)
            self.skipped_func_list.append("%s\n   %s\n"%(fi,reason))
            return
        self.ported_func_list.append(fi.__repr__())

        self.moduleCppCode.write(fi.gen_cpp_prelude())

        decl_c_args = "\n        "
        call_cpp_args = ""
        if not fi.ci == None and not fi.isconstructor():
            decl_c_args += fi.ci.type_info().ctype + " instance"
        for a in fi.args:

            if not decl_c_args.strip() == "":
                decl_c_args+=",\n        "
            if not call_cpp_args == "":
                call_cpp_args += ", "

            rw = a.out == "O" or a.out == "IO"

            arg_decl_star = not isinstance(a.type, BoxedClassTypeInfo) and rw
            if isinstance(a.type, StringTypeInfo):
                decl_c_args += "const char *" + a.name
            elif arg_decl_star:
                decl_c_args += a.type.ctype + " *" + a.name
            else:
                decl_c_args += a.type.ctype + " " + a.name

            if a.type.is_by_ptr:
                if a.pointer:
                    call_cpp_args += "((%s*)%s)"%(a.type.cpptype.replace("&",""), a.name)
                else:
                    call_cpp_args += "*((%s*)%s)"%(a.type.cpptype.replace("&",""), a.name)
            elif isinstance(a.type, StringTypeInfo):
                call_cpp_args += a.name
            elif a.type.is_by_value:
                if arg_decl_star and a.pointer:
                    call_cpp_args += "reinterpret_cast<" + a.type.cpptype + "*>(" +  a.name + ")"
                elif arg_decl_star and not a.pointer:
                    call_cpp_args += "*reinterpret_cast<" + a.type.cpptype + "*>(" +  a.name + ")"
                elif a.pointer:
                    call_cpp_args += "reinterpret_cast<" + a.type.cpptype + "*>(&" +  a.name + ")"
                else:
                    call_cpp_args += "*reinterpret_cast<" + a.type.cpptype + "*>(&" +  a.name + ")"
            else:
                if arg_decl_star and a.pointer:
                    call_cpp_args += a.name
                elif not arg_decl_star and not a.pointer:
                    call_cpp_args += a.name
                else:
                    call_cpp_args += "*" + a.name


        # C function prototype
        self.moduleCppCode.write("struct cv_return_value_%s %s(%s) {\n"%(fi.rv_type().ctype.replace(" ","_").replace(":","_").replace(" ","_").replace("*", "_"), fi.c_name(), decl_c_args));

        self.moduleCppCode.write("  try {\n");
        # cpp method call with prefix
        if fi.ci == None and (fi.cppname.startswith("cv") or fi.cppname.startswith("CV")):
            call_name = fi.cppname
        elif fi.ci == None:
            call_name = fi.fullname.replace(".", "::")
        elif fi.isconstructor() and isinstance(fi.ci.type_info(), BoxedClassTypeInfo):
            call_name = fi.ci.nested_cppname
        elif fi.cppname == "()":
            call_name = "(*((%s*) instance))"%(fi.ci.nested_cppname)
        elif isinstance(self.get_type_info(fi.ci.name), BoxedClassTypeInfo):
            call_name = "((%s*) instance)->%s"%(fi.ci.nested_cppname, fi.cppname)
        else:
            call_name = "((%s*) &instance)->%s"%(fi.ci.nested_cppname, fi.cppname)

        # actual call
        if fi.type.ctype == "void":
            self.moduleCppCode.write("  %s(%s);\n"%(call_name, call_cpp_args))
        elif fi.isconstructor() and isinstance(fi.rv_type(), BoxedClassTypeInfo):
            self.moduleCppCode.write("  %s* cpp_return_value = new %s(%s);\n"%(fi.rv_type().cpptype, call_name,
                call_cpp_args));
        elif fi.isconstructor() and call_cpp_args != "":
            self.moduleCppCode.write("  %s cpp_return_value(%s);\n"%(fi.rv_type().cpptype, call_cpp_args));
        elif fi.isconstructor():
            self.moduleCppCode.write("  %s cpp_return_value;\n"%(fi.rv_type().cpptype));
        else:
            self.moduleCppCode.write("  %s cpp_return_value = %s(%s);\n"%(fi.rv_type().cpptype, call_name,
                call_cpp_args));

        self.gen_c_return_value_type(fi.rv_type());

        # return value
        if fi.type.ctype == "void":
            self.moduleCppCode.write("  return { NULL, 0 };\n");
        elif isinstance(fi.rv_type(), StringTypeInfo):
            self.moduleCppCode.write("  return { NULL, strdup(cpp_return_value.c_str()) };");
        elif isinstance(fi.rv_type(), BoxedClassTypeInfo) and not fi.isconstructor():
            self.moduleCppCode.write("  return { NULL, new %s(cpp_return_value) };\n"%(fi.rv_type().cpptype));
        elif isinstance(fi.rv_type(), BoxedClassTypeInfo) and fi.isconstructor():
            self.moduleCppCode.write("  return { NULL, cpp_return_value };\n")
        elif isinstance(fi.rv_type(), SimpleClassTypeInfo):
            self.moduleCppCode.write("  return { NULL, *reinterpret_cast<%s*>(&cpp_return_value) };\n"%(fi.rv_type().ctype))
        elif isinstance(fi.rv_type(), VectorTypeInfo):
            self.moduleCppCode.write("  return { NULL, (void*) new %s(cpp_return_value) };\n"%(fi.rv_type().cpptype));
        else: # prim
            self.moduleCppCode.write("  return { NULL, cpp_return_value };\n");

        self.moduleCppCode.write("} catch (cv::Exception& e) {\n");
        self.moduleCppCode.write("    char* msg = strdup(e.what());\n");
        self.moduleCppCode.write("    return { msg, 0 };\n");
        self.moduleCppCode.write("} catch (...) {\n");
        self.moduleCppCode.write("    char* msg = strdup(\"unspecified error in OpenCV guts\");\n");
        self.moduleCppCode.write("    return { msg, 0 };\n");
        self.moduleCppCode.write("}\n");

        self.moduleCppCode.write("}\n\n");

        # rust's extern C
        self.moduleRustExterns.write(fi.gen_rust_extern())

        # rust safe wrapper
        self.moduleSafeRust.write(fi.gen_safe_rust())

    def gen_value_struct_field(self, name, typ):
        rsname = name
        if rsname in ["box", "type"]:
            rsname = "_" + rsname
        if "[" in typ:
            bracket = typ.index("[")
            cppt = typ[:bracket]
            ct = self.get_type_info(cppt).ctype
            size = typ[bracket+1:-1]
            rst = self.get_type_info(cppt).rtype
            self.moduleCppTypes.write("    %s %s[%s];\n"%(ct, name, size))
            self.moduleSafeRust.write("    pub %s: [%s;%s],\n"%(rsname, rst, size))
        else:
            typ = self.get_type_info(typ)
            self.moduleCppTypes.write("    %s %s;\n"%(typ.ctype, name))
            self.moduleSafeRust.write("    pub %s: %s,\n"%(rsname, typ.rtype))

    def gen_value_struct(self, c):
        self.moduleCppTypes.write("typedef struct struct_%s {\n"%(c.replace("::","_")))
        self.moduleSafeRust.write("// manually defined value struct %s\n"%(c))
        self.moduleSafeRust.write("#[repr(C)]#[derive(Debug,PartialEq)]\npub struct %s {\n"%(c.replace("::","_")))
        for field in value_struct_types[c]:
            self.gen_value_struct_field(field[0], field[1])
        self.moduleCppTypes.write("} struct_%s;\n\n"%(c.replace("::", "_")))
        self.moduleSafeRust.write("}\n\n")

    def gen_simple_class(self,ci):
        self.moduleCppTypes.write("typedef struct struct_%s {\n"%(ci.nested_cname))
        self.moduleSafeRust.write("// simple class from headers %s\n"%(ci))
        self.moduleSafeRust.write("#[repr(C)]#[derive(Debug,PartialEq)] pub struct %s {\n"%(ci.nested_cname))
        for p in ci.props:
            self.gen_value_struct_field(p.name, p.ctype)
        self.moduleSafeRust.write("}\n")
        self.moduleCppTypes.write("} struct_%s;\n\n"%(ci.nested_cname))

    def gen_c_return_value_type(self, typ):
        with open(self.output_path+"/cv_return_value_"+typ.ctype.replace("*","_").replace(" ","_").replace(":","_")+".type.h", "w") as f:
            f.write(template("""struct cv_return_value_$sane {
               char* error_msg;
               $ctype result;
            };\n""").substitute(
                sane=typ.ctype.replace("*","_").replace(" ","_").replace(":","_"),
                ctype="int" if typ.ctype == "void" else typ.ctype
            ))
        with open(self.output_path+"/cv_return_value_"+typ.ctype.replace("*","_").replace(" ","_").replace(":","_")+".rv.rs", "w") as f:
            if typ.ctype == "void":
                f.write("""#[repr(C)]\npub struct cv_return_value_void {
                   pub error_msg: *const ::libc::types::os::arch::c95::c_char,
                }\n""")
            else:
                f.write(template("""#[repr(C)]\npub struct cv_return_value_$sane {
                   pub error_msg: *const ::libc::types::os::arch::c95::c_char,
                   pub result: $rtype
                }\n""").substitute(
                    sane=typ.ctype.replace("*","_").replace(" ","_").replace(":","_"),
                    rtype=typ.rctype or typ.rtype
                ))

    def gen_boxed_class(self, name):
        ci = self.get_class(name)
        if not ci:
            return
        cname = name
        cppname = name
        if name in self.classes:
            cname = self.classes[name].nested_cname
            cppname = self.classes[name].nested_cppname

        self.moduleCppCode.write("// boxed class: %s\n"%(ci))
        self.moduleCppCode.write("void cv_%s_delete_%s(void* instance) {\n"%(self.module, cname));
        self.moduleCppCode.write("  delete (%s*) instance;\n"%(cppname));
        self.moduleCppCode.write("}\n\n");
        self.moduleRustExterns.write("pub fn cv_%s_delete_%s(ptr : *mut c_void);\n"%(self.module,cname));

        self.moduleSafeRust.write(template("""
            // boxed class $help
            #[allow(dead_code)]
            pub struct $name {
                pub ptr: *mut c_void
            }
            impl Drop for $name {
                fn drop(&mut self) {
                    unsafe { ::cv_${module}_delete_${cname}(self.ptr) };
                }
            }
            impl $name {
                fn as_ptr(&self) -> *mut c_void { self.ptr }
            }
        """).substitute(help=ci.__repr__(), cname=cname, name=ci.name, module=self.module))
        if ci and len(ci.bases):
            self.moduleSafeRust.write("// have %s bases %s"%(len(ci.bases), ci.bases))
            for base in ci.bases:
                cibase = self.get_class(base)
                print("base:%s cibase:%s"%(base, cibase))
                self.moduleSafeRust.write(template("""
                    impl $base for $rust_name {
                        fn as_ptr(&self) -> *mut c_void { self.ptr }
                    }
                """).substitute(rust_name=ci.name, base=cibase.name))

    def gen_nested_class_decl(self, ci):
        pass
        #self.moduleCppCode.write("class %s;\n"%(ci.nested_cname));

    def gen_class(self, ci):
        if ci.is_ignored:
            logging.info("Manual ignore class %s", ci)
            return
        t = self.get_type_info(ci.nested_cppname)
        if not t:
            logging.info("Ignore class %s (not found)", ci)
            return
        logging.info("Generating %s", ci)
        if ci.namespace == "":
            logging.info("Not namespaced. Skipped %s", ci)
            return
        if t.is_trait:
            self.moduleSafeRust.write("pub trait %s {\n"%(ci.name))
            self.moduleSafeRust.write("  fn as_ptr(&self) -> *mut c_void;\n")
            for fis in ci.methods.values():
                for fi in fis:
                    self.gen_func(fi)
            self.moduleSafeRust.write("} // trait %s\n"%(ci.name))
        else:
            if isinstance(t, BoxedClassTypeInfo):
                self.gen_boxed_class(ci.nested_cppname)
            self.moduleSafeRust.write("impl %s {\n\n"%(ci.name))
            for fis in ci.methods.values():
                for fi in fis:
                    self.gen_func(fi)
            self.moduleSafeRust.write("}\n");

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage:\n", \
            os.path.basename(sys.argv[0]), \
            "<full path to hdr_parser.py> <out_dir> <module name> <C++ header> [<C++ header>...]")
        print("Current args are: ", ", ".join(["'"+a+"'" for a in sys.argv]))
        exit(0)

    hdr_parser_path = os.path.abspath(sys.argv[1])
    if hdr_parser_path.endswith(".py"):
        hdr_parser_path = os.path.dirname(hdr_parser_path)
    sys.path.append(hdr_parser_path)
    import hdr_parser
    dstdir = sys.argv[2]
    module = sys.argv[3]
    srcfiles = sys.argv[4:]
    logging.basicConfig(filename='%s/%s.log' % (dstdir, module), format=None, filemode='w', level=logging.INFO)
    handler = logging.StreamHandler()
    handler.setLevel(logging.WARNING)
    logging.getLogger().addHandler(handler)
    print("Generating module '" + module + "' from headers:\n\t" + "\n\t".join(srcfiles))
    generator = RustWrapperGenerator()
    generator.gen(srcfiles, module, dstdir)
