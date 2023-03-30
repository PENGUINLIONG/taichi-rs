from taichi_json import (Alias, BitField, BuiltInType, Callback, Definition,
                         EntryBase, Enumeration, Field, Function, Handle,
                         Module, Structure, Union)
import pathlib
import re


def get_type_name(x: EntryBase):
    if isinstance(x, BuiltInType):
        return x.type_name
    elif isinstance(x, (Alias, Callback, Handle, Enumeration, Structure, Union)):
        return x.name.upper_camel_case
    elif isinstance(x, (BitField)):
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


def get_declr(module: Module, x: EntryBase, enum_aliases, with_docs=True):
    out = []

    if isinstance(x, BuiltInType):
        out += [""]

    elif isinstance(x, Alias):
        if with_docs:
            out += get_api_ref(module, x)
        ty_name = get_type_name(x)
        enum_aliases[ty_name[2:]] = ty_name
        out += [f"pub type {ty_name} = {get_type_name(x.alias_of)};"]

    elif isinstance(x, Callback):
        pass

    elif isinstance(x, Definition):
        if with_docs:
            out += get_api_ref(module, x)
        name = x.name.screaming_snake_case
        enum_aliases[name[3:]] = name
        out += [f"pub const {name}: u32 = {x.value};"]

    elif isinstance(x, Handle):
        if with_docs:
            out += get_api_ref(module, x)
        ty_name = get_type_name(x)
        # NOTE: (penguinliong) DO NOT register handle aliases because we write
        # wrappers for it.
        #enum_aliases[ty_name[2:]] = ty_name
        out += [
            "#[repr(transparent)]",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
            f"pub struct {ty_name}(pub usize);",
            f"impl {ty_name} {{",
            "    pub fn null() -> Self {",
            f"        {ty_name}(0)",
            "    }",
            "}",
        ]

    elif isinstance(x, Enumeration):
        if with_docs:
            out += get_api_ref(module, x)
        ty_name = get_type_name(x)
        enum_aliases[ty_name[2:]] = ty_name
        out += [
            "#[repr(i32)]" if x.name.snake_case == "ti_error" else "#[repr(u32)]",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
            "pub enum " + ty_name + " {",
        ]
        for name, value in x.cases.items():
            # Workaround types that start with a number.
            if name.screaming_snake_case[0].isdigit():
                if x.name.upper_camel_case == "TiImageDimension":
                    if with_docs:
                        out += get_api_field_ref(module, x, str(name))
                    out += [f"  D{name.upper_camel_case} = {value},"]
                else:
                    raise RuntimeError("dont know how to workaround a enum case that starts with a number")
            else:
                out += [f"  {name.upper_camel_case} = {value},"]
        out += ["}"]

    elif isinstance(x, BitField):
        ty_name = x.name.extend('flags').upper_camel_case
        enum_aliases[ty_name[2:]] = ty_name
        out += ["bitflags! {"]
        if with_docs:
            out += get_api_ref(module, x)
        out += [
            "#[repr(transparent)]",
            "pub struct " + ty_name + ": u32 {"
        ]
        for name, value in x.bits.items():
            if with_docs:
                out += get_api_field_ref(module, x, str(name))
            out += [
                f"  const {name.extend('bit').screaming_snake_case} = 1 << {value};"
            ]
        out += [
            "}",
            "}",
        ]

    elif isinstance(x, Structure):
        if with_docs:
            out += get_api_ref(module, x)
        ty_name = get_type_name(x)
        # NOTE: (penguinliong) DO NOT register structures unless they are
        # suffixed by `Info`, meaning that they are simple param structs.
        # Otherwise we should write wrappers for them.
        if ty_name.endswith("Info"):
            enum_aliases[ty_name[2:]] = ty_name
        out += [
            "#[repr(C)]",
            "#[derive(Clone, Copy)]",
            "pub struct " + ty_name + " {"
        ]
        for field in x.fields:
            if with_docs:
                out += get_api_field_ref(module, x, field.name)
            out += [f"  pub {get_field(field)},"]
        out += ["}"]
        return '\n'.join(out)

    elif isinstance(x, Union):
        if with_docs:
            out += get_api_ref(module, x)
        ty_name = get_type_name(x)
        enum_aliases[ty_name[2:]] = ty_name
        out = [
            "#[repr(C)]",
            "#[derive(Clone, Copy)]",
            "pub union " + ty_name + " {"
        ]
        for variant in x.variants:
            if with_docs:
                out += get_api_field_ref(module, x, variant.name)
            out += [f"  pub {get_field(variant)},"]
        out += ["}"]
        return '\n'.join(out)

    elif isinstance(x, Function):
        fn_name = x.name.snake_case
        enum_aliases[fn_name[3:]] = fn_name
        return_value_type = "()" if x.return_value_type == None else get_type_name(
            x.return_value_type)
        out += [
            "#[link(name = \"taichi_c_api\")]",
            "extern \"C\" {",
        ]
        if with_docs:
            out += get_api_ref(module, x)
            if x.params:
                out2 = []
                for param in x.params:
                    out2 += get_api_fn_param_ref(module, x, param.name)
                if out2:
                    out += [
                        "///",
                        "/// Parameters:",
                    ]
                    out += out2
        out += [
            "pub fn " + x.name.snake_case + "(",
        ]
        if x.params:
            for param in x.params:
                out += [f"  {get_field(param)},"]
        out += [
            f") -> {return_value_type};",
            "}",
        ]
        return '\n'.join(out)

    else:
        raise RuntimeError(f"'{x.id}' doesn't need declaration")

    return '\n'.join(out)


def get_human_readable_name(x: EntryBase):
    if isinstance(x, BuiltInType):
        return ""

    elif isinstance(x, Alias):
        return f"{get_type_name(x)}"

    elif isinstance(x, Definition):
        return f"{x.name.screaming_snake_case}"

    elif isinstance(x, (Handle, Enumeration, BitField, Structure, Union)):
        return f"{get_type_name(x)}"

    elif isinstance(x, Function):
        return f"{x.name.snake_case}"

    else:
        raise RuntimeError(f"'{x.id}' doesn't have a human readable name")


def get_api_ref(module: Module, x: EntryBase) -> list:
    out = [f"/// {get_title(x)}"]
    if module.doc and x.id in module.doc.api_refs:
        out += [
            f"/// {resolve_inline_symbols_to_names(module, y)}"
            for y in module.doc.api_refs[x.id]
        ]
    return out


def get_api_field_ref(module: Module, x: EntryBase, field_sym: str) -> list:
    field_sym = f"{x.id}.{field_sym}"
    if module.doc and field_sym in module.doc.api_field_refs:
        return [f"  /// {module.doc.api_field_refs[field_sym]}"]
    return []


def get_api_fn_param_ref(module: Module, x: EntryBase, field_sym: str) -> list:
    field_sym2 = f"{x.id}.{field_sym}"
    if module.doc and field_sym2 in module.doc.api_field_refs:
        return [f"/// - `{field_sym}`: {module.doc.api_field_refs[field_sym2]}"]
    return []


def get_title(x: EntryBase):
    if isinstance(x, BuiltInType):
        return ""

    extra = ""
    if isinstance(x, Function) and x.is_device_command:
        extra += " (Device Command)"

    if isinstance(x, (Alias, Definition, Handle, Enumeration, BitField,
                      Structure, Union, Function)):
        return f"{type(x).__name__} `{get_human_readable_name(x)}`" + extra
    else:
        raise RuntimeError(f"'{x.id}' doesn't need title")


def resolve_symbol_to_name(module: Module, id: str):
    """Returns the resolved symbol and its hyperlink (if available)"""
    try:
        ifirst_dot = id.index('.')
    except ValueError:
        return None

    field_name = ""
    try:
        isecond_dot = id.index('.', ifirst_dot + 1)
        field_name = id[isecond_dot + 1:]
        id = id[:isecond_dot]
    except ValueError:
        pass

    out = module.declr_reg.resolve(id)
    href = None

    try:
        if field_name:
            out = get_human_readable_field_name(out, field_name)
        else:
            href = "#" + get_title(out).lower().replace(' ', '-').replace(
                '`', '').replace('(', '').replace(')', '')
            out = get_human_readable_name(out)
    except:
        print(f"WARNING: Unable to resolve symbol {id}")
        out = id

    return out, href


def resolve_inline_symbols_to_names(module: Module, line: str):
    SYM_PATTERN = r"\`(\w+\.\w+(?:\.\w+)?)\`"
    matches = re.findall(SYM_PATTERN, line)

    replacements = {}
    for m in matches:
        id = str(m)
        replacements[id] = resolve_symbol_to_name(module, id)

    for old, (new, href) in replacements.items():
        if new is None:
            print(f"WARNING: Unresolved inline symbol `{old}`")
        else:
            if href is None:
                new = f"`{new}`"
            else:
                new = f"[`{new}`]({href})"
            line = line.replace(f"`{old}`", new)
    return line


def get_human_readable_field_name(x: EntryBase, field_name: str):
    out = None
    if isinstance(x, Enumeration):
        out = x.name.extend(field_name).screaming_snake_case
    elif isinstance(x, BitField):
        out = x.name.extend(field_name).extend('bit').screaming_snake_case
    elif isinstance(x, Structure):
        for field in x.fields:
            if str(field.name) == field_name:
                out = str(field.name)
                break
    elif isinstance(x, Union):
        for field in x.variants:
            if str(field.name) == field_name:
                out = str(field.name)
                break
    elif isinstance(x, Function):
        for field in x.params:
            if str(field.name) == field_name:
                out = str(field.name)
                break
    return out


def print_module_header(module):
    out = []


    if module.doc is not None:
        out += [
            f"/// {resolve_inline_symbols_to_names(module, x)}"
            for x in module.doc.module_doc
        ]

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

    enum_aliases = {}
    for x in module.declr_reg:
        out += [
            "",
            get_declr(module, module.declr_reg.resolve(x), enum_aliases),
        ]

    out += [
        "",
        "pub mod aliases {",
    ]

    for name, target in enum_aliases.items():
        out += [f"pub use super::{target} as {name};"]

    out += [
        "}",
        ""
    ]

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
        BuiltInType("uint16_t", "u16"),
        BuiltInType("int16_t", "i16"),
        BuiltInType("uint8_t", "u8"),
        BuiltInType("int8_t", "i8"),
        BuiltInType("double", "f64"),
        BuiltInType("float", "f32"),
        BuiltInType("const char*", "*const c_char"),
        BuiltInType("const char**", "*const *const c_char"),
        BuiltInType("void*", "*mut c_void"),
        BuiltInType("const void*", "*const c_void"),
        BuiltInType("char", "c_char"),
    }

    for module in Module.load_all(builtin_tys):
        generate_module_header(module)
