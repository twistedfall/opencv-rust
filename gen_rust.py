import sys, re, os.path
import logging
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
#         [ "cv.Mat.create", "Mat", [], [] ],
         [ "cv.Mat.depth", "int", [], [] ],
         [ "cv.Mat.size", "Size2i", [], [] ],
    ]
}

renamed_funcs = {   "cv_core_divide_MMMDI": "divide_mat",
                    "cv_core_norm_MMIM":"norm_dist",
                    "cv_core_ellipse_MPSDDDSIII": "ellipse_tilted",
                    "cv_features2d_DescriptorMatcher_match_MMVM" : "matches",
                    "cv_features2d_DescriptorMatcher_match_MVV" : "matches",
                    "cv_imgproc_integral_MMMI" : "integral_squares",
                    "cv_imgproc_integral_MMMMI" : "integral_squares_tilted",
                    "cv_imgproc_distanceTransform_MMMIII" : "distance_tranform_labels",
                    "cv_video_calcOpticalFlowSF_MMMIIIDDIDDDIDDD" : "calc_optical_flow_full",
        }

class_ignore_list = (
    #core
    "FileNode", "FileStorage", "KDTree", "IndexParams", "Params"
    #videoio
#    "VideoWriter",
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
    u"short" : { u"ctype": "short", u"rtype": "u16" },
    u"int"   : { u"ctype": "int", u"rtype": "u32" },
    u"size_t": { u"ctype": "std::size_t", u"rtype": "::libc::types::os::arch::c95::size_t" },
    u"int64" : { u"ctype": "int64", u"rtype": "i64" },
    u"float" : { u"ctype": "float", u"rtype": "f32" },
    u"double": { u"ctype": "double", u"rtype": "f64" }
}

# trait_classes = [ "Algorithm" ]

forced_boxed_classes = { }

value_struct_types = {
    ("core", "Point") : (("x", "int"), ("y", "int")),
    ("core", "Point2d") : (("x", "double"), ("y", "double")),
    ("core", "Point2f") : (("x", "float"), ("y", "float")),
    ("core", "Size") : (("width", "int"), ("height", "int")),
    ("core", "Size2i") : (("width", "int"), ("height", "int")),
    ("core", "Size2f") : (("width", "float"), ("height", "float")),
    ("core", "Rect") : (("x", "int"), ("y", "int"), ("width", "int"), ("height", "int")),
    ("core", "RotatedRect") : (("x", "float"), ("y", "float"), ("width", "float"),("height", "float"), ("angle", "float")),
    ("core", "TermCriteria") : (("type", "int"), ("maxCount", "int"), ("epsilon", "double")),
    ("core", "Scalar") : (("data", "double[4]"),)
}

for s in [2,3,4,6]:
    for t in [("uchar","b"),("short","s"),("int","i"),("double","d"),("float","f")]:
        value_struct_types[("core","Vec%d%s"%(s,t[1]))] = ("data", "%s[%d]"%(t[0],s)),

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
using namespace cv;

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
//    use ::core::*;
    use sys::types::*;
    use std::ffi::{ CStr, CString };
    use std::mem::transmute;
    $module_import
    $code
}

"""

#
#       AST-LIKE
#

class GeneralInfo():
    def __init__(self, name, namespaces):
        self.namespace, self.classpath, self.classname, self.name = self.parseName(name, namespaces)

    def parseName(self, name, namespaces):
        '''
        input: full name and available namespaces
        returns: (namespace, classpath, classname, name)
        '''
        name = name[name.find(" ")+1:].strip() # remove struct/class/const prefix
        spaceName = ""
        localName = name # <classes>.<name>
        for namespace in sorted(namespaces, key=len, reverse=True):
            if name.startswith(namespace + "."):
                spaceName = namespace
                localName = name.replace(namespace + ".", "")
                break
        pieces = localName.split(".")
        if len(pieces) > 2: # <class>.<class>.<class>.<name>
            return spaceName, ".".join(pieces[:-1]), pieces[-2], pieces[-1]
        elif len(pieces) == 2: # <class>.<name>
            return spaceName, pieces[0], pieces[0], pieces[1]
        elif len(pieces) == 1: # <name>
            return spaceName, "", "", pieces[0]
        else:
            return spaceName, "", "" # error?!

    def fullName(self, isCPP=False):
        result = ".".join([self.fullClass(), self.name])
        return result if not isCPP else result.replace(".", "::")

    def fullClass(self, isCPP=False):
        result = ".".join([f for f in [self.namespace] + self.classpath.split(".") if len(f)>0])
        return result if not isCPP else result.replace(".", "::")

def make_cpp_type(t):
    if(t == "size_t"):
        return t
    return t.replace("_", "::")

class ArgInfo():
    def __init__(self, arg_tuple): # [ ctype, name, def val, [mod], argno ]
        self.pointer = False
        type = arg_tuple[0]
        if type.endswith("*"):
            type = type[:-1]
            self.pointer = True
        if type == "String":
            type = "string"
        self.ctype = type
        self.type = make_cpp_type(type)
        self.name = arg_tuple[1]
        self.defval = arg_tuple[2]
        self.out = ""
        if "/O" in arg_tuple[3]:
            self.out = "O"
        if "/IO" in arg_tuple[3]:
            self.out = "IO"

    def __repr__(self):
        return Template("ARG $ctype$p $name=$defval").substitute(ctype=self.type,
                                                                  p=" *" if self.pointer else "",
                                                                  name=self.name,
                                                                  defval=self.defval)

class FuncInfo(GeneralInfo):
    def __init__(self, decl, namespaces=[]): # [ funcname, return_ctype, [modifiers], [args] ]
        GeneralInfo.__init__(self, decl[0], namespaces)
#        self.jname = self.name
        self.isconstructor = self.name == self.classname
#        if "[" in self.name:
#            self.jname = "getelem"
#        for m in decl[2]:
#            if m.startswith("="):
#                self.jname = m[1:]
        self.static = ["","static"][ "/S" in decl[2] ]
        if self.isconstructor:
            self.type = "::".join(decl[0].split(".")[1:-1])
        else:
            self.type = make_cpp_type(decl[1])
        self.cppname = self.name.replace(".", "::")
        self.cname = "_".join(decl[0].split(".")[1:])
        self.args = []
        self.class_nested_cppname = "::".join(decl[0].split(".")[1:-1])
#        func_fix_map = func_arg_fix.get(self.classname, {}).get(self.jname, {})
        for a in decl[3]:
#            arg = a[:]
#            arg_fix_map = func_fix_map.get(arg[1], {})
#            arg[0] = arg_fix_map.get('ctype',  arg[0]) #fixing arg type
#            arg[3] = arg_fix_map.get('attrib', arg[3]) #fixing arg attrib
            self.args.append(ArgInfo(a))

    def __repr__(self):
        return Template("FUNC <$type $namespace.$classpath.$name $args>").substitute(**self.__dict__)

class ClassPropInfo():
    def __init__(self, decl): # [f_ctype, f_name, '', '/RW']
        self.ctype = decl[0]
        self.name = decl[1]
        self.rw = "/RW" in decl[3]

    def __repr__(self):
        return Template("PROP $ctype $name").substitute(ctype=self.ctype, name=self.name)

class ClassInfo(GeneralInfo):
    def __init__(self, decl, namespaces=[]): # [ 'class/struct cname', ': base', [modlist] ]
        GeneralInfo.__init__(self, decl[0], namespaces)
        self.methods = []
        self.simple = False
        self.nested = False
        for m in decl[2]:
            if m == "/Simple" or m == "/Map" :
                self.simple = True
        if len(decl[0].split(".")) > 2:
            self.nested = True
        self.nested_cppname = "::".join(decl[0].split(".")[1:])
        self.nested_cname = "_".join(decl[0].split(".")[1:])

        # class props
        self.props= []
        for p in decl[3]:
            self.props.append( ClassPropInfo(p) )

    def __repr__(self):
#        return Template("CLASS $namespace.$classpath.$name : $base").substitute(**self.__dict__)
        return Template("CLASS $namespace.$classpath.$name").substitute(**self.__dict__)

    def addMethod(self, fi):
        self.methods.append(fi)

    def getAllMethods(self):
        result = []
        result.extend([fi for fi in sorted(self.methods) if fi.isconstructor])
        result.extend([fi for fi in sorted(self.methods) if not fi.isconstructor])
        return result
#
#       GENERATOR
#

class RustWrapperGenerator(object):
    def __init__(self):
        self.clear()

    def clear(self):
        self.module = ""
        self.Module = ""
        self.classes = { }
        self.functions = [];
#        self.classes = { "Mat" : ClassInfo([ 'class Mat', '', [], [] ], self.namespaces) }
        self.ported_func_list = []
        self.skipped_func_list = []
        self.def_args_hist = {} # { def_args_cnt : funcs_cnt }

    def getClass(self, classname):
        return self.classes[classname] # or self.Module]

    def add_class(self, decl):
        classinfo = ClassInfo(decl, namespaces=self.namespaces)
        name = classinfo.nested_cppname
        if not self.is_ignored(name):
            self.classes[name] = classinfo

    def add_const(self, decl): # [ "const cname", val, [], [] ]
        pass

    def add_func(self, decl):
        fi = FuncInfo(decl, namespaces=self.namespaces)
        if fi.class_nested_cppname == "":
            self.functions.append(fi)
        elif self.is_ignored(fi.class_nested_cppname):
            logging.info('ignored: %s', fi)
        elif fi.class_nested_cppname in ManualFuncs and fi.jname in ManualFuncs[classname]:
            logging.info('manual: %s', fi)
        elif self.is_ignored(fi.class_nested_cppname):
            pass
        else:
            self.getClass(fi.class_nested_cppname).addMethod(fi)
            # calc args with def val
            cnt = len([a for a in fi.args if a.defval])
            self.def_args_hist[cnt] = self.def_args_hist.get(cnt, 0) + 1

    def save(self, path, buf):
        f = open(path, "wt")
        f.write(buf)
        f.close()

    def add_decl(self, decl):
        name = decl[0]
        if name.startswith("struct") or name.startswith("class"):
            self.add_class(decl)
        elif name.startswith("const"):
            self.add_const(decl)
        else: # function
            self.add_func(decl)

    def gen(self, srcfiles, module, output_path):
        parser = hdr_parser.CppHeaderParser()
        self.output_path = output_path
        self.module = module
        self.Module = module.capitalize()
        includes = [];

        for hdr in srcfiles:
            decls = parser.parse(hdr)
            self.namespaces = parser.namespaces
            logging.info("\n\n===== Header: %s =====", hdr)
            logging.info("Namespaces: %s", parser.namespaces)
            if decls:
                includes.append('#include "' + hdr + '"')
            for decl in decls:
                logging.info("\n--- Incoming ---\n%s", pformat(decl, 4))
                self.add_decl(decl)

        if module in ManualFuncs:
            for decl in ManualFuncs[self.module]:
                logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                self.add_decl(decl)

        logging.info("\n\n===== Generating... =====")
        self.moduleCppTypes = StringIO()
        self.moduleCppCode = StringIO()
        self.moduleRustCode = StringIO()
        self.moduleRustExterns = StringIO()

        for ci in self.classes.values():
            if ci.nested:
                self.gen_nested_class_decl(ci)

        for c in value_struct_types:
            if c[0] == module:
                self.gen_value_struct(c)

        for c in self.classes.values():
            if c.simple:
                self.gen_simple_class(c)

        for fi in self.functions:
            self.gen_func(None, fi)

        if module in forced_boxed_classes:
            for cb in forced_boxed_classes[module]:
                self.gen_boxed_class(cb)

        for ci in self.classes.values():
            self.gen_class(ci)

        with open(output_path+"/types.h", "a") as f:
            f.write(self.moduleCppTypes.getvalue())

        self.save(output_path+"/"+module+".cpp", Template(T_CPP_MODULE).substitute(m = module, M = module.upper(), code = self.moduleCppCode.getvalue(), includes = "\n".join(includes)))

        with open(output_path+"/%s.externs.rs"%(module), "w") as f:
            f.write("extern \"C\" {\n")
            f.write(self.moduleRustExterns.getvalue())
            f.write("}\n")
        self.save(output_path+"/"+module+".rs", Template(T_RUST_MODULE).substitute(m = module, M = module.upper(), code = self.moduleRustCode.getvalue(), module_import = ("use ::sys::core::*;\n" if not module == "core" else "")))
        self.save(output_path+"/"+module+".txt", self.makeReport())

    def makeReport(self):
        '''
        Returns string with generator report
        '''
        report = StringIO()
        total_count = len(self.ported_func_list)+ len(self.skipped_func_list)
        report.write("PORTED FUNCs LIST (%i of %i):\n\n" % (len(self.ported_func_list), total_count))
        report.write("\n".join(self.ported_func_list))
        report.write("\n\nSKIPPED FUNCs LIST (%i of %i):\n\n" % (len(self.skipped_func_list), total_count))
        report.write("".join(self.skipped_func_list))
        for i in self.def_args_hist.keys():
            report.write("\n%i def args - %i funcs" % (i, self.def_args_hist[i]))
        return report.getvalue()

    def is_string(self, type_name):
        return type_name == "string"

    def is_primitive(self, type_name):
        return type_name in primitives

    # opencv classes with the /Simple modifiers
    def is_simple(self, type_name):
        return type_name in self.classes and self.classes[type_name].simple

    # special types from core, passed by value
    def is_value(self, type_name):
        for k in value_struct_types:
            if k[1] == type_name:
                return True
        return self.is_simple(type_name)

    def is_ptr(self, type_name):
        return type_name.startswith("Ptr::")

    def is_vector_of_vector(self, type_name):
        return type_name.startswith("vector::vector::")

    def is_vector(self, type_name):
        return type_name.startswith("vector::") and not self.is_vector_of_vector(type_name)

    def is_ignored(self, type_name):
        return type_name.split("::")[-1] in class_ignore_list

    def is_boxed(self, type_name):
        return not (self.is_value(type_name) or self.is_simple(type_name)
            or self.is_primitive(type_name) or self.is_string(type_name))

    def map_type(self, type_name):
        if self.is_value(type_name):
            return {    "ctype"  : "cv_struct_%s"%(type_name.replace("::","_")),
                        "cpptype": type_name,
                        "rtype"  : "%s"%(type_name.replace("::", "_")) }
        elif self.is_simple(type_name):
            return {    "ctype" : "cv_struct_%s"%(type_name.replace("::","_")),
                        "cpptype": type_name,
                        "rtype" : "%s"%(type_name.replace("::", "_")),
                    }
        elif self.is_primitive(type_name):
            primitives[type_name]["cpptype"] = type_name
            return primitives[type_name]
        elif self.is_vector(type_name):
            h = {       "ctype" : "void*",
                        "cpptype": "vector<%s>"%(type_name.split("::")[-1]),
                        "rctype" : "*mut i8",
                        "rtype" : "VectorOf%s"%(type_name.split("::")[1]) }
            self.gen_template_wrapper_rust_struct(h)
            return h
        elif self.is_vector_of_vector(type_name):
            h = {    "ctype" : "void*",
                        "rctype" : "*mut i8",
                        "cpptype": "vector< vector<%s> >"%(type_name.split("::")[-1]),
                        "rtype" : "VectorOfVectorOf%s"%(type_name.split("::")[2]) }
            self.gen_template_wrapper_rust_struct(h)
            return h
        elif self.is_ptr(type_name):
            h = {    "ctype" : "void*",
                        "rctype" : "*mut i8",
                        "cpptype": "Ptr<%s>"%(type_name.split("::")[-1]),
                        "rtype" : "PtrOf%s"%(type_name.split("::")[1]) }
            self.gen_template_wrapper_rust_struct(h)
            return h
        elif self.is_string(type_name):
            return { "ctype" : "const char*", "cpptype" : "string",
                "rtype": "*const ::libc::types::os::arch::c95::c_char",
                "rrvtype": "String" }
        else:
            return { "ctype" : "void*", "cpptype" : type_name, "rctype": "*mut i8", "rtype": "%s"%(type_name) }

    def gen_vector_struct_for(self, name):
        struct_name = "cv_vector_of_"+name
        self.defined_in_types_h.appand(struct_name)
        self.moduleCppTypes.write

    def gen_func(self, ci, fi, prop_name=''):
        if fi.isconstructor:
            rv_type = ci.nested_cppname
        else:
            rv_type = fi.type;
        if fi.cppname == "()":
            msg = "can not map operator() yet"
            self.skipped_func_list.append("%s\n   %s\n"%(fi,msg))
            return
        for a in fi.args:
            if self.is_ignored(a.type):
                msg = "can not map type %s yet"%(a.type)
                self.skipped_func_list.append("%s\n   %s\n"%(fi,msg))
                return

        rv = self.map_type(rv_type)

        self.ported_func_list.append(fi.__repr__())

        self.moduleCppCode.write("// %s %s\n"%(fi.cppname,
            "(constructor)" if fi.isconstructor else "(method)"))
        self.moduleCppCode.write("// %s\n"%(fi))
        self.moduleCppCode.write("// Return value: %s\n"%(rv))

        decl_c_args = "\n        "
        call_cpp_args = ""
        decl_rust_extern_args = ""
        decl_rust_args = ""
        call_rust_args = ""
        suffix = "_" if len(fi.args) > 0 else ""
        if not ci == None and not fi.isconstructor:
            decl_c_args += self.map_type(ci.name)["ctype"] + " instance"
        for a in fi.args:
            atype = self.map_type(a.type)
            if not decl_c_args.strip() == "":
                decl_c_args+=",\n        "
            if not call_cpp_args == "":
                call_cpp_args += ", "
                decl_rust_extern_args += ", "
                decl_rust_args += ", "
                call_rust_args += ", "
            suffix += a.type[0].capitalize()

            rsname = a.name
            if rsname in ["type","box"]:
                rsname = "_" + rsname

            rw = a.out == "O" or a.out == "IO"

            decl_c_args += "/* "
            decl_c_args += a.__repr__() + "\n        "
            decl_c_args += atype.__repr__() + "\n        "
            if a.type in self.classes:
                decl_c_args += "%s\n        "%(self.classes[a.type])
            else:
                decl_c_args += "%s is not a class of this module\n        "%(a.type)
            if rw:
                decl_c_args += "rw "
            if self.is_boxed(a.type):
                decl_c_args += "boxed "
            if self.is_simple(a.type):
                decl_c_args += "simple "
            decl_c_args += "\n        */ "

            arg_decl_star = not self.is_boxed(a.type) and rw
            if self.is_string(a.type):
                decl_c_args += "const char *" + a.name
            elif arg_decl_star:
                decl_c_args += atype["ctype"] + " *" + a.name
            else:
                decl_c_args += atype["ctype"] + " " + a.name

            if self.is_string(a.type):
                decl_rust_args += "%s:&str"%(rsname)
            elif self.is_primitive(a.type) or self.is_value(a.type) \
                    or self.is_simple(a.type):
                decl_rust_args += rsname + ":" + atype["rtype"]
            elif rw:
                decl_rust_args += rsname + ":&mut " + atype["rtype"]
            else:
                decl_rust_args += rsname + ":& " + atype["rtype"]

            if self.is_boxed(a.type) or self.is_vector(a.type) \
                    or self.is_vector_of_vector(a.type) or self.is_ptr(a.type):
                call_rust_args += "%s.ptr"%(rsname)
            elif self.is_string(a.type):
                call_rust_args += "CString::new(%s).unwrap().as_ptr()"%(rsname)
            else:
                call_rust_args += "%s"%(rsname)

            if self.is_boxed(a.type) or self.is_vector(a.type) \
                    or self.is_vector_of_vector(a.type) or self.is_ptr(a.type):
                call_cpp_args += "*((%s*)%s)"%(atype["cpptype"], a.name)
            elif a.type == "string":
                call_cpp_args += a.name
            elif "arg_c_to_cpp" in atype:
                call_cpp_args += atype["arg_c_to_cpp"].substitute(src=a.name)
            elif self.is_value(a.type) or (a.type in self.classes and self.classes[a.type].simple):
                if arg_decl_star and a.pointer:
                    call_cpp_args += "reinterpret_cast<" + atype["cpptype"] + "*>(" +  a.name + ")"
                elif arg_decl_star and not a.pointer:
                    call_cpp_args += "*reinterpret_cast<" + atype["cpptype"] + "*>(" +  a.name + ")"
                elif a.pointer:
                    call_cpp_args += "reinterpret_cast<" + atype["cpptype"] + "*>(&" +  a.name + ")"
                else:
                    call_cpp_args += "*reinterpret_cast<" + atype["cpptype"] + "*>(&" +  a.name + ")"
            else:
                if arg_decl_star and a.pointer:
                    call_cpp_args += a.name
                elif not arg_decl_star and not a.pointer:
                    call_cpp_args += a.name
                else:
                    call_cpp_args += "*" + a.name

            decl_rust_extern_args += rsname + ": " + (atype.get("rctype") or atype["rtype"])

        if ci == None:
            c_name = "cv_%s_%s%s"%(module, fi.cppname, suffix);
        else:
            c_name = "cv_%s_%s_%s%s"%(module, ci.nested_cname, fi.cppname, suffix);

        # C function prototype
#        if self.is_boxed(rv_type):
#            self.moduleCppCode.write("%s* %s(%s) {\n"%(rv_type, c_name, decl_c_args));
#        else:
        if fi.type == "void":
            self.moduleCppCode.write("int %s(%s) {\n"%(c_name, decl_c_args));
        else:
            self.moduleCppCode.write("struct cv_return_value_%s %s(%s) {\n"%(rv["ctype"].replace(" ","_").replace(":","_").replace(" ","_").replace("*", "_"), c_name, decl_c_args));

        # cpp method call with prefix
        if ci == None:
            call_name = "cv::" + fi.cppname
        elif fi.isconstructor and self.is_boxed(ci.name):
            call_name = ci.nested_cppname
        else:
            call_name = "((%s*) instance)->%s"%(self.map_type(ci.name)["cpptype"], fi.cppname)

        # actual call
        if fi.type == "void":
            self.moduleCppCode.write("  %s(%s);\n"%(call_name, call_cpp_args))
#        elif self.is_ptr(rv_type):
#            self.moduleCppCode.write("  %s cpp_return_value = %s(%s);\n"%(rv["cpptype"], call_cpp_args));
        elif fi.isconstructor and self.is_boxed(rv_type):
            self.moduleCppCode.write("  %s* cpp_return_value = new %s(%s);\n"%(rv["cpptype"], call_name,
                call_cpp_args));
        elif fi.isconstructor and call_cpp_args != "":
            self.moduleCppCode.write("  %s cpp_return_value(%s);\n"%(rv["cpptype"], call_cpp_args));
        elif fi.isconstructor:
            self.moduleCppCode.write("  %s cpp_return_value;\n"%(rv["cpptype"]));
        else:
            self.moduleCppCode.write("  %s cpp_return_value = %s(%s);\n"%(rv["cpptype"], call_name,
                call_cpp_args));

        if not fi.type == "void":
            self.gen_c_return_value_type(rv);

        # return value
        if fi.type == "void":
            self.moduleCppCode.write("  return 0;");
        elif self.is_string(rv_type):
            self.moduleCppCode.write("  return { 0, strdup(cpp_return_value.c_str()) };");
        elif self.is_boxed(rv_type) and not fi.isconstructor:
            self.moduleCppCode.write("  return { 0, new %s(cpp_return_value) };\n"%(rv["cpptype"]));
        elif self.is_boxed(rv_type) and fi.isconstructor:
            self.moduleCppCode.write("  return { 0, cpp_return_value };\n")
        elif self.is_value(rv_type):
            self.moduleCppCode.write("  return { 0, *reinterpret_cast<cv_struct_%s*>(&cpp_return_value) };\n"%(rv_type.replace("::", "_")))
        elif self.is_vector(rv_type):
            self.moduleCppCode.write("  return { 0, (void*) new %s(cpp_return_value) };\n"%(rv["cpptype"]));
        elif "return_cpp_to_c" in rv:
            self.moduleCppCode.write(rv["return_cpp_to_c"].substitute(src="cpp_return_value"));
        else:
            self.moduleCppCode.write("  return { 0, cpp_return_value };\n");
        self.moduleCppCode.write("}\n\n");

        if fi.type == "void":
            rust_extern_rs = "u32"
        else:
            rust_extern_rs = "rv::cv_return_value_%s"%(rv["ctype"].replace("*","_").replace(" ","_").replace(":","_"))

        self.moduleRustExterns.write("pub fn %s(%s) -> %s;\n"%(c_name, decl_rust_extern_args, rust_extern_rs))

        rname = renamed_funcs.get(c_name) or fi.name

        if not ci == None:
            self.moduleRustCode.write("impl %s {\n"%(ci.name))
        self.moduleRustCode.write("  pub fn %s(%s) -> Result<%s,u32> {\n"%(rname,
                decl_rust_args, rv.get("rrvtype") or rv.get("rtype")))
        self.moduleRustCode.write("    unsafe {\n")
        self.moduleRustCode.write("      let _rv = ::%s(%s);\n"%(c_name, call_rust_args))
        if not fi.type == "void":
            self.moduleRustCode.write("      let _rv = _rv.result;\n");
        if(self.is_string(rv_type)):
            self.moduleRustCode.write("      let v = CStr::from_ptr(_rv).to_bytes().to_vec();\n");
            self.moduleRustCode.write("      ::libc::free(_rv as *mut ::libc::types::common::c95::c_void);\n");
            self.moduleRustCode.write("      let _rv:String = String::from_utf8(v).unwrap();\n");
        elif self.is_boxed(rv_type):
            self.moduleRustCode.write("      let _rv = %s{ ptr: _rv };\n"%(rv["rtype"], ))
        elif fi.type == "bool":
            self.moduleRustCode.write("      let _rv = _rv!=0;\n")
        elif fi.type == "void":
            self.moduleRustCode.write("      let _rv = ();\n")
        self.moduleRustCode.write("      return Ok(_rv);\n")
        self.moduleRustCode.write("    }\n");
        self.moduleRustCode.write("  }\n")
        if not ci == None:
            self.moduleRustCode.write("}\n")

    def gen_value_struct_field(self, name, typ):
        rsname = name
        if rsname in ["box", "type"]:
            rsname = "_" + rsname
        if "[" in typ:
            bracket = typ.index("[")
            cppt = typ[:bracket]
            ct = self.map_type(cppt)["ctype"]
            size = typ[bracket+1:-1]
            rst = self.map_type(cppt)["rtype"]
            self.moduleCppTypes.write("    %s %s[%s];\n"%(ct, name, size))
            self.moduleRustCode.write("    %s: [%s;%s],\n"%(rsname, rst, size))
        else:
            cppt = typ
            ct = self.map_type(cppt)["ctype"]
            rst = self.map_type(cppt)["rtype"]
            self.moduleCppTypes.write("    %s %s;\n"%(ct, name))
            self.moduleRustCode.write("    %s: %s,\n"%(rsname, rst))

    def gen_value_struct(self, c):
        self.moduleCppTypes.write("typedef struct cv_struct_%s {\n"%(c[1]))
        self.moduleRustCode.write("#[repr(C)]#[derive(Debug)] pub struct %s {\n"%(c[1]))
        for field in value_struct_types[c]:
            self.gen_value_struct_field(field[0], field[1])
        self.moduleCppTypes.write("} cv_struct_%s;\n\n"%(c[1]))
        self.moduleRustCode.write("}\n")

    def gen_simple_class(self,ci):
        self.moduleCppTypes.write("typedef struct cv_struct_%s {\n"%(ci.nested_cname))
        self.moduleRustCode.write("#[repr(C)]#[derive(Debug)] pub struct %s {\n"%(ci.nested_cname))
        for p in ci.props:
            self.gen_value_struct_field(p.name, p.ctype)
        self.moduleRustCode.write("}\n")
        self.moduleCppTypes.write("} cv_struct_%s;\n\n"%(ci.nested_cname))

    def gen_template_wrapper_rust_struct(self, typ):
        with open(self.output_path+"/"+typ["rtype"]+".type.rs", "w") as f:
            f.write("#[allow(dead_code)] pub struct %s { pub ptr: *mut i8 }\n"%(typ["rtype"]));

    def gen_c_return_value_type(self, typ):
        with open(self.output_path+"/cv_return_value_"+typ["ctype"].replace("*","_").replace(" ","_").replace(":","_")+".type.h", "w") as f:
            f.write("struct cv_return_value_%s {\n"%(typ["ctype"].replace("*","_").replace(" ","_").replace(":","_")));
            f.write("   int error_code;\n");
            f.write("   %s result;\n"%(typ["ctype"]));
            f.write("};\n");
        with open(self.output_path+"/cv_return_value_"+typ["ctype"].replace("*","_").replace(" ","_").replace(":","_")+".rv.rs", "w") as f:
            f.write("/* %s\n*/\n"%(typ))
            f.write("#[repr(C)] pub struct cv_return_value_%s {\n"%(typ["ctype"].replace("*","_").replace(" ","_").replace(":","_")));
            f.write("   pub error_code:u32,\n");
            f.write("   pub result: %s,\n"%(typ.get("rctype") or typ["rtype"]));
            f.write("}\n");

    def gen_boxed_class(self, name):
        cname = name
        cppname = name
        if name in self.classes:
            cname = self.classes[name].nested_cname
            cppname = self.classes[name].nested_cppname
        self.moduleRustExterns.write("pub fn cv_%s_delete_%s(ptr : *mut i8);\n"%(self.module,cname));

        self.moduleRustCode.write("#[allow(dead_code)] pub struct %s { pub ptr: *mut i8 }\n"%(cname));
        self.moduleRustCode.write("impl Drop for %s {\n"%(cname));
        self.moduleRustCode.write("  fn drop(&mut self) { unsafe { ::cv_%s_delete_%s(self.ptr) }; }\n"%(self.module, cname));
        self.moduleRustCode.write("}\n")

        self.moduleCppCode.write("void cv_%s_delete_%s(void* instance) {\n"%(self.module, cname));
        self.moduleCppCode.write("  delete (cv::%s*) instance;\n"%(cppname));
        self.moduleCppCode.write("}\n");

    def gen_nested_class_decl(self, ci):
        pass
        #self.moduleCppCode.write("class %s;\n"%(ci.nested_cname));

    def gen_class(self, ci):
        if self.is_boxed(ci.nested_cppname):
            self.gen_boxed_class(ci.nested_cppname)
        for fi in ci.getAllMethods():
            self.gen_func(ci, fi)

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
