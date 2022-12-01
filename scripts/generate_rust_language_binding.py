from taichi_json import (Alias, BitField, BuiltInType, Definition, EntryBase,
                         Enumeration, Field, Function, Handle, Module,
                         Structure, Union)
import pathlib


def get_type_name(x: EntryBase):
    ty = type(x)
    if ty in [BuiltInType]:
        return x.type_name
    elif ty in [Alias, Handle, Enumeration, Structure, Union]:
        return x.name.upper_camel_case
    elif ty in [BitField]:
        return x.name.extend('flags').upper_camel_case
    else:
        raise RuntimeError(f"'{x.id}' is not a type")


def get_field(x: Field):
    # `count` is an integer so it's a static array.
    is_dyn_array = x.count and not isinstance(x.count, int)

    name = str(x.name)
    # Reserved names.
    if name in ["type", "i32", "f32"]:
        name = "r#" + name
    is_ptr = x.by_ref or x.by_mut or is_dyn_array
    const_q = "const " if not x.by_mut else "mut "
    type_name = get_type_name(x.type)

    if is_ptr:
        return f"{name}: *{const_q}{type_name}"
    elif x.count:
        return f"{name}: [{type_name}; {x.count}]"
    else:
        return f"{name}: {type_name}"


def get_declr(x: EntryBase):
    ty = type(x)
    if ty is BuiltInType:
        return ""

    elif ty is Alias:
        return f"pub type {get_type_name(x)} = {get_type_name(x.alias_of)};"

    elif ty is Definition:
        return f"pub const {x.name.screaming_snake_case}: u32 = {x.value};"

    elif ty is Handle:
        ty_name = get_type_name(x)
        return '\n'.join([
            "#[repr(transparent)]",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
            f"pub struct {ty_name}(pub usize);",
            f"impl {ty_name} {{",
            "    pub fn null() -> Self {",
            f"        {ty_name}(0)",
            "    }",
            "}",
        ])

    elif ty is Enumeration:
        out = [
            "#[repr(i32)]" if x.name.snake_case == "ti_error" else "#[repr(u32)]",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
            "pub enum " + get_type_name(x) + " {",
        ]
        for name, value in x.cases.items():
            # Workaround types that start with a number.
            if name.screaming_snake_case[0].isdigit():
                if x.name.upper_camel_case == "TiImageDimension":
                    out += [f"  D{name.upper_camel_case} = {value},"]
                else:
                    raise RuntimeError("dont know how to workaround a enum case that starts with a number")
            else:
                out += [f"  {name.upper_camel_case} = {value},"]
        out += ["}"]
        return '\n'.join(out)

    elif ty is BitField:
        bit_type_name = x.name.extend('flags').upper_camel_case
        out = [
            "bitflags! {",
            "#[repr(transparent)]",
            "pub struct " + bit_type_name + ": u32 {"
        ]
        for name, value in x.bits.items():
            out += [
                f"  const {name.extend('bit').screaming_snake_case} = 1 << {value};"
            ]
        out += [
            "}",
            "}",
        ]
        return '\n'.join(out)

    elif ty is Structure:
        out = [
            "#[repr(C)]",
            "#[derive(Clone, Copy)]",
            "pub struct " + get_type_name(x) + " {"
        ]
        for field in x.fields:
            out += [f"  pub {get_field(field)},"]
        out += ["}"]
        return '\n'.join(out)

    elif ty is Union:
        out = [
            "#[repr(C)]",
            "#[derive(Clone, Copy)]",
            "pub union " + get_type_name(x) + " {"
        ]
        for variant in x.variants:
            out += [f"  pub {get_field(variant)},"]
        out += ["}"]
        return '\n'.join(out)

    elif ty is Function:
        return_value_type = "()" if x.return_value_type == None else get_type_name(
            x.return_value_type)
        out = [
            "#[link(name = \"taichi_c_api\")]",
            "extern \"C\" {",
            "pub fn " + x.name.snake_case + "(",
        ]
        if x.params:
            out += [',\n'.join(f"  {get_field(param)}" for param in x.params)]
        out += [
            f") -> {return_value_type};",
            "}",
        ]
        return '\n'.join(out)

    else:
        raise RuntimeError(f"'{x.id}' doesn't need declaration")


def get_human_readable_name(x: EntryBase):
    ty = type(x)
    if ty is BuiltInType:
        return ""

    elif ty is Alias:
        return f"{get_type_name(x)}"

    elif ty is Definition:
        return f"{x.name.screaming_snake_case}"

    elif isinstance(x, (Handle, Enumeration, BitField, Structure, Union)):
        return f"{get_type_name(x)}"

    elif ty is Function:
        return f"{x.name.snake_case}"

    else:
        raise RuntimeError(f"'{x.id}' doesn't have a human readable name")


def print_module_header(module):
    out = []

    out += [
        "#[allow(unused_imports)]",
        "use std::os::raw::{c_void, c_char};",
        "#[allow(unused_imports)]",
        "use bitflags::bitflags;",
    ]

    for x in module.required_modules:
        if "taichi/" not in x:
            # FIXME: (penguinliong) non-taichi modules shouldn't be a part of
            # the header since taichi-dev/taichi PR #6199.
            #raise RuntimeError("unexpected module requirement")
            return ""
        if x == "taichi/taichi_platform.h":
            continue
        module_name = x[len("taichi/"):-len(".h")]
        out += [
            "#[allow(unused_imports)]",
            f"use crate::{module_name}::*;",
        ]

    for x in module.declr_reg:
        out += [
            "",
            f"// {x}",
            get_declr(module.declr_reg.resolve(x)),
        ]

    out += [""]

    return '\n'.join(out)


def generate_module_header(module):
    if module.is_built_in:
        return

    path = pathlib.Path("c_api/rust")
    if not path.exists():
        path.mkdir()

    module_name = str(module.name)[len("taichi/"):-len(".h")] + ".rs"

    print(f"processing module '{module_name}'")
    path = f"c_api/rust/{module_name}"
    with open(path, "w") as f:
        f.write(print_module_header(module))


if __name__ == "__main__":
    builtin_tys = {
        BuiltInType("uint64_t", "u64"),
        BuiltInType("int64_t", "i64"),
        BuiltInType("uint32_t", "u32"),
        BuiltInType("int32_t", "i32"),
        BuiltInType("float", "f32"),
        BuiltInType("const char*", "*const c_char"),
        BuiltInType("const char**", "*const *const c_char"),
        BuiltInType("void*", "*mut c_void"),
        BuiltInType("const void*", "*const c_void"),
        BuiltInType("char", "c_char"),
    }

    for module in Module.load_all(builtin_tys):
        generate_module_header(module)
