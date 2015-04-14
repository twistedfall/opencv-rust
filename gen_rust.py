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

ManualFuncs = { }

class_ignore_list = (
    #core
    "FileNode", "FileStorage", "KDTree", "KeyPoint", "DMatch",
    #videoio
#    "VideoWriter",
)

func_arg_fix = {
}

#
#       TYPES MAPPING
#

type_mapping = {
    u"void"  : { u"ctype": "void", "rtype": "()" },
    u"bool"  : { u"ctype": "int", u"rtype": "bool" },
    u"uchar" : { u"ctype": "unsigned char", u"rtype": "u8" },
    u"short" : { u"ctype": "short", u"rtype": "u16" },
    u"int"   : { u"ctype": "int", u"rtype": "u32" },
    u"size_t": { u"ctype": "std::size_t", u"rtype": "::libc::types::os::arch::c95::size_t" },
    u"int64" : { u"ctype": "int64", u"rtype": "i64" },
    u"float" : { u"ctype": "float", u"rtype": "f32" },
    u"double": { u"ctype": "double", u"rtype": "f64" },
    u"string": { u"ctype": "const char *", u"rtype": "*const libc::types::os::arch::c95::c_char",
                 u"return_cpp_to_c": Template("return strdup($src.c_str());\n") },
}

boxed_classes = [ "Mat", "Subdiv2D" ]
trait_classes = [ "Algorithm" ]

forced_boxed_classes = { "core" : [ "Mat" ] }

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

#include "common.h"
#include "types.h"
#include <iostream>

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


use $m::*;

extern "C" {

$externs

} // extern "C"

mod $m {
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

class ArgInfo():
    def __init__(self, arg_tuple): # [ ctype, name, def val, [mod], argno ]
        self.pointer = False
        cpptype = arg_tuple[0]
        if cpptype.endswith("*"):
            cpptype = cpptype[:-1]
            self.pointer = True
#        if cpptype == 'vector_Point2d':
#            cpptype = 'vector_Point2f'
#        elif cpptype == 'vector_Point3d':
#            cpptype = 'vector_Point3f'
        self.ctype = cpptype
        self.cpptype = cpptype.replace("_", "::")
        self.name = arg_tuple[1]
        self.defval = arg_tuple[2]
        self.out = ""
        if "/O" in arg_tuple[3]:
            self.out = "O"
        if "/IO" in arg_tuple[3]:
            self.out = "IO"

    def __repr__(self):
        return Template("ARG $ctype$p $name=$defval").substitute(ctype=self.cpptype,
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
            self.cpptype = "_".join(decl[0].split(".")[1:-1])
        else:
            self.cpptype = decl[1]
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
        return Template("FUNC <$cpptype $namespace.$classpath.$name $args>").substitute(**self.__dict__)

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
#        self.methods_suffixes = {}
#        self.consts = [] # using a list to save the occurence order
#        self.private_consts = []
#        self.imports = set()
#        self.jname = self.name
#        self.j_code = None # java code stream
#        self.jn_code = None # jni code stream
#        self.cpp_code = None # cpp code stream
        for m in decl[2]:
#            if m.startswith("="):
#                self.jname = m[1:]
            if m == "/Simple":
                self.simple = True
        if len(decl[0].split(".")) > 2:
            self.nested = True
        self.nested_cppname = "::".join(decl[0].split(".")[1:])
        self.nested_cname = "_".join(decl[0].split(".")[1:])
#        self.base = ''
#        if decl[1]:
            #self.base = re.sub(r"\b"+self.jname+r"\b", "", decl[1].replace(":", "")).strip()
#            self.base = re.sub(r"^.*:", "", decl[1].split(",")[0]).strip().replace(self.jname, "")

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

#    def isWrapped(self, classname):
#        name = classname or self.Module
#        return name in self.classes

    def add_class(self, decl):
        classinfo = ClassInfo(decl, namespaces=self.namespaces)
        name = classinfo.nested_cppname
        if not name in class_ignore_list:
            self.classes[name] = classinfo

    def add_const(self, decl): # [ "const cname", val, [], [] ]
        pass

    def add_func(self, decl):
        fi = FuncInfo(decl, namespaces=self.namespaces)
        if fi.class_nested_cppname == "":
            self.functions.append(fi)
        elif fi.class_nested_cppname in class_ignore_list:
            logging.info('ignored: %s', fi)
        elif fi.class_nested_cppname in ManualFuncs and fi.jname in ManualFuncs[classname]:
            logging.info('manual: %s', fi)
#        elif not self.isWrapped(fi.classname):
#            logging.info('not wrapped: %s', fi)
        elif fi.class_nested_cppname in class_ignore_list:
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

    def gen(self, srcfiles, module, output_path):
        parser = hdr_parser.CppHeaderParser()
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
                name = decl[0]
                if name.startswith("struct") or name.startswith("class"):
                    self.add_class(decl)
                elif name.startswith("const"):
                    self.add_const(decl)
                else: # function
                    self.add_func(decl)

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
#            if ci.name == "Mat":
#                continue
#            ci.initCodeStreams(self.Module)
            self.gen_class(ci)
#            classJavaCode = ci.generateJavaCode(self.module, self.Module)
#            self.save("%s/%s+%s.java" % (output_path, module, ci.jname), classJavaCode)
#            self.moduleCppCode.write(ci.generateCppCode())
#            ci.cleanupCodeStreams(
#)

        with open(output_path+"/types.h", "a") as f:
            f.write("#include <cstddef>\n");
            f.write(self.moduleCppTypes.getvalue())

        self.save(output_path+"/"+module+".cpp", Template(T_CPP_MODULE).substitute(m = module, M = module.upper(), code = self.moduleCppCode.getvalue(), includes = "\n".join(includes)))
        self.save(output_path+"/"+module+".rs", Template(T_RUST_MODULE).substitute(m = module, M = module.upper(), code = self.moduleRustCode.getvalue(), externs=self.moduleRustExterns.getvalue()))
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

    def is_boxed(self, type_name):
        return type_name in boxed_classes

    def is_trait(self, type_name):
        return type_name in trait_classes

    def is_native(self, type_name):
        return type_name in type_mapping

    def is_value(self, type_name):
        for k in value_struct_types:
            if k[1] == type_name:
                return True
        return self.is_simple(type_name)

    def is_pointer(self, type_name):
        return not self.is_native(type_name) and not self.is_value(type_name) and not self.is_simple(type_name)

    def is_mapped(self, type_name):
        return not self.is_unmapped(type_name)

    def is_simple(self, type_name):
        return type_name in self.classes and self.classes[type_name].simple

    def is_unmapped(self, type_name):
        return type_name.startswith("vector") \
            or type_name.startswith("Ptr")

    def map_type(self, type_name):
        if self.is_value(type_name):
            return {    "ctype" : "cv_struct_%s"%(type_name.replace("::","_")),
                        "rtype": "%s"%(type_name.replace("::", "_")) }
        elif self.is_simple(type_name):
            return {    "ctype" : "cv_struct_%s"%(type_name.replace("::","_")),
                        "rtype": "%s"%(type_name.replace("::", "_")) }
        elif type_name in type_mapping:
            return type_mapping[type_name]
        elif self.is_pointer(type_name):
            return { "ctype" : "void*", "rtype": "%s"%(type_name) }

    def gen_func(self, ci, fi, prop_name=''):
        if fi.isconstructor:
            rv_cpptype = ci.nested_cppname
        else:
            rv_cpptype = fi.cpptype;
        if not ci == None and not self.is_mapped(ci.name):
            msg = "unmapped class %s "%(ci.name)
            self.skipped_func_list.append("%s\n   %s\n"%(fi,msg))
            return
        if fi.cppname == "()":
            msg = "can not map operator() yet"
            self.skipped_func_list.append("%s\n   %s\n"%(fi,msg))
            return
        if not self.is_mapped(rv_cpptype):
            msg = "can not map return value %s"%(rv_cpptype)
            self.skipped_func_list.append("%s\n   %s\n"%(fi,msg))
            return
        for a in fi.args:
            if not self.is_mapped(a.cpptype):
                msg = "can not map arg [%s]"%(a)
                self.skipped_func_list.append("%s\n   %s\n"%(fi,msg))
                return

        rv = self.map_type(rv_cpptype)

        self.ported_func_list.append(fi.__repr__())

        self.moduleCppCode.write("// %s %s\n"%(fi.cppname,
            "(constructor)" if fi.isconstructor else "(method)"))
        self.moduleCppCode.write("// %s\n"%(fi))

        decl_c_args = "\n        "
        call_cpp_args = ""
        decl_rust_extern_args = ""
        suffix = "_" if len(fi.args) > 0 else ""
        if not ci == None and not fi.isconstructor and self.is_pointer(ci.name):
            decl_c_args += ci.name + " *instance"
        for a in fi.args:
            atype = self.map_type(a.cpptype)
            if not decl_c_args.strip() == "":
                decl_c_args+=",\n        "
            if not call_cpp_args == "":
                call_cpp_args += ", "
                decl_rust_extern_args += ", "
            suffix += a.cpptype[0].capitalize()

            rw = a.out == "O" or a.out == "IO"

            decl_c_args += "/* "
            decl_c_args += a.__repr__() + "\n        "
            decl_c_args += atype.__repr__() + "\n        "
            if a.cpptype in self.classes:
                decl_c_args += "%s\n        "%(self.classes[a.cpptype])
            else:
                decl_c_args += "not in classes:" + a.cpptype + " "
            if rw:
                decl_c_args += "rw "
            if self.is_pointer(a.cpptype):
                decl_c_args += "heap "
            if self.is_simple(a.cpptype):
                decl_c_args += "simple "
            decl_c_args += "\n        */ "

            arg_decl_star = not self.is_pointer(a.cpptype) and rw
            if arg_decl_star:
                decl_c_args += atype["ctype"] + " *" + a.name
            else:
                decl_c_args += atype["ctype"] + " " + a.name


            if self.is_pointer(a.cpptype):
                call_cpp_args += "*((%s*)%s)"%(a.cpptype, a.name)
            elif "arg_c_to_cpp" in atype:
                call_cpp_args += atype["arg_c_to_cpp"].substitute(src=a.name)
            elif self.is_value(a.cpptype) or (a.cpptype in self.classes and self.classes[a.cpptype].simple):
                if arg_decl_star and a.pointer:
                    call_cpp_args += "reinterpret_cast<" + a.cpptype + "*>(" +  a.name + ")"
                elif arg_decl_star and not a.pointer:
                    call_cpp_args += "*reinterpret_cast<" + a.cpptype + "*>(" +  a.name + ")"
                elif a.pointer:
                    call_cpp_args += "reinterpret_cast<" + a.cpptype + "*>(&" +  a.name + ")"
                else:
                    call_cpp_args += "*reinterpret_cast<" + a.cpptype + "*>(&" +  a.name + ")"
            else:
                if arg_decl_star and a.pointer:
                    call_cpp_args += a.name
                elif not arg_decl_star and not a.pointer:
                    call_cpp_args += a.name
                else:
                    call_cpp_args += "*" + a.name

            rsname = a.name
            if rsname in ["type","box"]:
                rsname = "_" + rsname
            decl_rust_extern_args += rsname + ": " + atype["rtype"]

        if ci == None:
            c_name = "cv_%s_%s%s"%(module, fi.cppname, suffix);
        else:
            c_name = "cv_%s_%s_%s%s"%(module, ci.nested_cname, fi.cppname, suffix);

        # C function prototype
        if self.is_pointer(rv_cpptype):
            self.moduleCppCode.write("%s* %s(%s) {\n"%(rv_cpptype, c_name, decl_c_args));
        else:
            self.moduleCppCode.write("%s %s(%s) {\n"%(rv["ctype"], c_name, decl_c_args));

        # cpp method call with prefix
        if ci == None:
            call_name = "cv::" + fi.cppname
        elif fi.isconstructor and self.is_pointer(ci.name):
            call_name = ci.nested_cppname
        else:
            call_name = "instance->" + fi.cppname

        # actual call
        if fi.cpptype == "void":
            self.moduleCppCode.write("  %s(%s);\n"%(call_name, call_cpp_args));
        elif fi.isconstructor and self.is_pointer(rv_cpptype):
            self.moduleCppCode.write("  %s* cpp_return_value = new %s(%s);\n"%(rv_cpptype, call_name,
                call_cpp_args));
        elif fi.isconstructor:
            self.moduleCppCode.write("  %s cpp_return_value(%s);\n"%(rv_cpptype, call_cpp_args));
        else:
            self.moduleCppCode.write("  %s cpp_return_value = %s(%s);\n"%(rv_cpptype, call_name,
                call_cpp_args));

        # return value
        if fi.cpptype == "void":
            pass
        elif self.is_pointer(rv_cpptype) and not fi.isconstructor:
            self.moduleCppCode.write("  return new %s(cpp_return_value);\n"%(rv_cpptype));
        elif self.is_pointer(rv_cpptype) and fi.isconstructor:
            self.moduleCppCode.write("  return cpp_return_value;\n")
        elif self.is_value(rv_cpptype):
            self.moduleCppCode.write("  return *reinterpret_cast<cv_struct_%s*>(&cpp_return_value);\n"%(rv_cpptype.replace("::", "_")))
        elif "return_cpp_to_c" in rv:
            self.moduleCppCode.write(rv["return_cpp_to_c"].substitute(src="cpp_return_value"));
        else:
            self.moduleCppCode.write("  return cpp_return_value;\n");
        self.moduleCppCode.write("}\n\n");

        self.moduleRustExterns.write("pub fn %s(%s) -> %s;\n"%(c_name, decl_rust_extern_args, rv["rtype"]));

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
        self.moduleRustCode.write("#[repr(C)] pub struct %s {\n"%(c[1]))
        for field in value_struct_types[c]:
            self.gen_value_struct_field(field[0], field[1])
        self.moduleCppTypes.write("} cv_struct_%s;\n\n"%(c[1]))
        self.moduleRustCode.write("}\n")

    def gen_simple_class(self,ci):
        self.moduleCppTypes.write("typedef struct cv_struct_%s {\n"%(ci.nested_cname))
        self.moduleRustCode.write("#[repr(C)] pub struct %s {\n"%(ci.nested_cname))
        for p in ci.props:
            self.gen_value_struct_field(p.name, p.ctype)
        self.moduleRustCode.write("}\n")
        self.moduleCppTypes.write("} cv_struct_%s;\n\n"%(ci.nested_cname))

    def gen_boxed_class(self, name):
        cname = name
        cppname = name
        if name in self.classes:
            cname = self.classes[name].nested_cname
            cppname = self.classes[name].nested_cppname
        self.moduleRustExterns.write("pub fn cv_%s_delete_%s(ptr : *mut i8);\n"%(self.module,cname));

        self.moduleRustCode.write("#[repr(C)]#[allow(dead_code)] pub struct %s { ptr: *mut i8 }\n"%(cname));
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
        if not self.is_mapped(ci.nested_cppname):
            logging.info("Skip class %s", ci.nested_cppname)
        if self.is_pointer(ci.nested_cppname):
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
