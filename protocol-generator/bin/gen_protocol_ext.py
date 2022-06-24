#!/usr/bin/env python3

import argparse
import re
from os.path import basename


def parse():
    parser = argparse.ArgumentParser(
        description="The gernerator for candlebase protocol in rust."
    )
    parser.add_argument("--proto_file", required=True, type=argparse.FileType("r"))
    parser.add_argument("--enum_type_name", required=True)
    args = parser.parse_args()
    return args.proto_file, args.enum_type_name


def extract(content, enum_type_name):
    enum_type_def_pattern = r"enum\s+" + enum_type_name + "\s+{([^}]+)}"
    enum_type_def_match = re.search(enum_type_def_pattern, content)

    if enum_type_def_match:
        enum_pairs_pattern = r"([A-Z_0-9]+)\s*=\s*([0-9]+);"
        enum_pairs = re.findall(enum_pairs_pattern, enum_type_def_match.group(1))
        return enum_pairs
    else:
        return []


def capitalize(name):
    return "".join(map(lambda s: s.capitalize(), name.lower().split("_")))


def spaces(n):
    return " " * n


def build_use_decls(module_name):
    return (
        f"""use super::{module_name}::*;\n"""
        f"""use actix::Message as ActixMessage;\n"""
        f"""use bytes::{{BufMut, Bytes, BytesMut}};\n"""
        f"""pub use prost::DecodeError;\n"""
        f"""use prost::Message as ProstMessage;\n"""
        f"""use std::fmt::{{Debug, Formatter, Result as FmtResult}};\n"""
        f"""use std::result::Result as StdResult;"""
    )


def build_protocol_msg_enum_def(enum_pairs):
    protocol_msg_variant_defs = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            protocol_msg_variant_defs.append(f"""{spaces(2)}None,""")
        else:
            protocol_msg_variant_defs.append(
                f"""{spaces(2)}{capitalize(enum_name)}({capitalize(enum_name)}),"""
            )
    protocol_msg_variant_defs_output = "\n".join(protocol_msg_variant_defs)
    protocol_msg_enum_def_output = (
        f"""pub enum ProtocolMsg {{\n{protocol_msg_variant_defs_output}\n}}"""
    )
    return protocol_msg_enum_def_output


def build_protocol_msg_debug_impl(enum_pairs):
    match_arm_decls = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            match_arm_decls.append(
                f"""{spaces(6)}ProtocolMsg::None => write!(f, "None"),"""
            )
        else:
            match_arm_decls.append(
                f"""{spaces(6)}ProtocolMsg::{capitalize(enum_name)}(msg) => write!(f, "{{:?}}", msg),"""
            )
    match_arms_decls_output = "\n".join(match_arm_decls)
    match_expr_decl_output = (
        f"""{spaces(4)}match self {{\n{match_arms_decls_output}\n{spaces(4)}}}"""
    )
    fmt_output = (
        f"""{spaces(2)}fn fmt(&self, f: &mut Formatter) -> FmtResult {{\n"""
        f"""{match_expr_decl_output}\n"""
        f"""{spaces(2)}}}"""
    )
    protocol_msg_debug_impl_output = (
        f"""impl Debug for ProtocolMsg {{\n{fmt_output}\n}}"""
    )
    return protocol_msg_debug_impl_output


def build_protocol_msg_impl():
    return (
        f"""impl ProtocolMsg {{\n"""
        f"""{spaces(2)}#[inline]\n"""
        f"""{spaces(2)}pub fn is_none(&self) -> bool {{\n"""
        f"""{spaces(4)}match self {{\n"""
        f"""{spaces(6)}Self::None => true,\n"""
        f"""{spaces(6)}_ => false,\n"""
        f"""{spaces(4)}}}\n"""
        f"""{spaces(2)}}}\n\n"""
        f"""{spaces(2)}#[inline]\n"""
        f"""{spaces(2)}pub fn is_some(&self) -> bool {{\n"""
        f"""{spaces(4)}match self {{\n"""
        f"""{spaces(6)}Self::None => false,\n"""
        f"""{spaces(6)}_ => true,\n"""
        f"""{spaces(4)}}}\n"""
        f"""{spaces(2)}}}\n"""
        f"""}}"""
    )


def build_send_error_struct_def():
    return (
        f"""#[derive(Debug)]\n"""
        f"""pub enum SendError {{\n"""
        f"""{spaces(2)}Timeout,\n"""
        f"""{spaces(2)}Closed,\n"""
        f"""{spaces(2)}Any(Box<dyn std::error::Error + Send + Sync>),\n"""
        f"""}}"""
    )


def build_actix_message_impl(enum_pairs):
    impls = []
    impls.append(
        f"""impl ActixMessage for ProtocolMsg {{\n"""
        f"""{spaces(2)}type Result = StdResult<ProtocolMsg, SendError>;\n"""
        f"""}}"""
    )
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            continue
        elif enum_name[-3:] == "REQ":
            req = capitalize(enum_name)
            rep = capitalize(enum_name[0:-3] + "REP")
            impls.append(
                f"""impl ActixMessage for {req} {{\n"""
                f"""{spaces(2)}type Result = StdResult<{rep}, SendError>;\n"""
                f"""}}"""
            )
    impls_output = "\n\n".join(impls)
    return impls_output


def build_into_enum_trait_def():
    return (
        f"""pub trait IntoEnum {{\n"""
        f"""{spaces(2)}fn into_enum(self) -> ProtocolMsg;\n"""
        f"""}}"""
    )


def build_into_enum_impls(enum_pairs):
    impls = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            continue
        impls.append(
            f"""impl IntoEnum for {capitalize(enum_name)} {{\n"""
            f"""{spaces(2)}#[inline]\n"""
            f"""{spaces(2)}fn into_enum(self) -> ProtocolMsg {{\n"""
            f"""{spaces(4)}ProtocolMsg::{capitalize(enum_name)}(self)\n"""
            f"""{spaces(2)}}}\n"""
            f"""}}"""
        )
    impls_output = "\n\n".join(impls)
    return impls_output


def build_encode_trait_def():
    return (
        f"""pub trait Encode: ProstMessage + Sized {{\n"""
        f"""{spaces(2)}fn encode_type(bytes: &mut BytesMut);\n\n"""
        f"""{spaces(2)}#[inline]\n"""
        f"""{spaces(2)}fn encode_body(&self, bytes: &mut BytesMut) {{\n"""
        f"""{spaces(4)}if let Err(err) = self.encode(bytes) {{\n"""
        f"""{spaces(6)}panic!("Failed to encode msg: {{:?}}", err);\n"""
        f"""{spaces(4)}}}\n"""
        f"""{spaces(2)}}}\n\n"""
        f"""{spaces(2)}fn encode_msg(&self) -> Bytes {{\n"""
        f"""{spaces(4)}let size = self.encoded_len() as usize;\n"""
        f"""{spaces(4)}let mut bytes = BytesMut::with_capacity(size + 1);\n"""
        f"""{spaces(4)}Self::encode_type(&mut bytes);\n"""
        f"""{spaces(4)}self.encode_body(&mut bytes);\n"""
        f"""{spaces(4)}return bytes.freeze();\n"""
        f"""{spaces(2)}}}\n"""
        f"""}}"""
    )


def build_encode_impls(enum_pairs):
    impls = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            continue
        impls.append(
            f"""impl Encode for {capitalize(enum_name)} {{\n"""
            f"""{spaces(2)}#[inline]\n"""
            f"""{spaces(2)}fn encode_type(bytes: &mut BytesMut) {{\n"""
            f"""{spaces(4)}bytes.put_u8({enum_value});\n"""
            f"""{spaces(2)}}}\n"""
            f"""}}"""
        )
    impls_output = "\n\n".join(impls)
    return impls_output


def build_encode_fn_def(enum_pairs):
    match_arm_decls = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::None => panic!("Failed to encode ProtocolMsg::None"),"""
            )
        else:
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::{capitalize(enum_name)}(msg) => msg.encode_msg(),"""
            )
    match_arms_decls_output = "\n".join(match_arm_decls)
    match_expr_decl_output = f"""{spaces(2)}match protocol_msg {{\n{match_arms_decls_output}\n{spaces(2)}}}"""
    encode_fn_def_output = (
        f"""pub fn encode(protocol_msg: &ProtocolMsg) -> Bytes {{\n"""
        f"""{match_expr_decl_output}\n"""
        f"""}}"""
    )

    return encode_fn_def_output


def build_decode_fn_def(enum_pairs):
    vars_decl_output = (
        f"""{spaces(2)}let msg_type = bytes[0] as i8;\n"""
        f"""{spaces(2)}let msg_body = bytes.slice(1..);"""
    )

    case_decls = []
    first = True
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            continue
        if first:
            case_name = f"""{spaces(2)}if"""
            first = False
        else:
            case_name = f"""{spaces(1)}else if"""

        case_decls.append(
            f"""{case_name} msg_type == {enum_value} {{\n"""
            f"""{spaces(4)}let res: Result<{capitalize(enum_name)}, DecodeError> = ProstMessage::decode(msg_body);\n"""
            f"""{spaces(4)}match res {{\n"""
            f"""{spaces(6)}Ok(msg) => Ok(ProtocolMsg::{capitalize(enum_name)}(msg)),\n"""
            f"""{spaces(6)}Err(err) => Err(err),\n"""
            f"""{spaces(4)}}}\n"""
            f"""{spaces(2)}}}"""
        )
    case_decls.append(
        f"""{spaces(1)}else {{\n"""
        f"""{spaces(4)}Err(DecodeError::new(format!("Invalid msg type: {{}}", msg_type)))\n"""
        f"""{spaces(2)}}}"""
    )
    cases_output = "".join(case_decls)

    decode_fn_def_output = (
        f"""pub fn decode(bytes: &Bytes) -> Result<ProtocolMsg, DecodeError> {{\n"""
        f"""{vars_decl_output}\n"""
        f"""{cases_output}\n"""
        f"""}}"""
    )

    return decode_fn_def_output


def build_set_round_ref_fn_def(enum_pairs):
    match_arm_decls = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::None => panic!("Failed to set round ref for ProtocolMsg::None"),"""
            )
        elif enum_name in [
            "DO_REQ",
            "DO_REP",
            "DO2_REQ",
            "DO2_REP",
            "OK2_REP",
            "ERROR2_REP",
        ]:
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::{capitalize(enum_name)}(msg) => msg.traces[0].r#round_ref = round_ref,"""
            )
        else:
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::{capitalize(enum_name)}(msg) => msg.r#round_ref = round_ref,"""
            )

    match_arms_decls_output = "\n".join(match_arm_decls)
    match_expr_decl_output = f"""{spaces(2)}match protocol_msg {{\n{match_arms_decls_output}\n{spaces(2)}}}"""
    set_round_ref_fn_def_output = (
        f"""pub fn set_round_ref(protocol_msg: &mut ProtocolMsg, round_ref: u32) -> &ProtocolMsg {{\n"""
        f"""{match_expr_decl_output}\n"""
        f"""{spaces(2)}protocol_msg\n"""
        f"""}}"""
    )

    return set_round_ref_fn_def_output


def build_get_round_ref_fn_def(enum_pairs):
    match_arm_decls = []
    for (enum_name, enum_value) in enum_pairs:
        if enum_name[0:7] == "UNKNOWN":
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::None => panic!("Failed to get round ref from ProtocolMsg::None"),"""
            )
        else:
            match_arm_decls.append(
                f"""{spaces(4)}ProtocolMsg::{capitalize(enum_name)}(msg) => msg.r#round_ref,"""
            )

    match_arms_decls_output = "\n".join(match_arm_decls)
    match_expr_decl_output = f"""{spaces(2)}match protocol_msg {{\n{match_arms_decls_output}\n{spaces(2)}}}"""
    get_round_ref_fn_def_output = (
        f"""pub fn get_round_ref(protocol_msg: &ProtocolMsg) -> u32 {{\n"""
        f"""{match_expr_decl_output}\n"""
        f"""}}"""
    )

    return get_round_ref_fn_def_output


def output(module_name, enum_pairs):
    output = (
        f"""{build_use_decls(module_name)}\n\n"""
        f"""{build_protocol_msg_enum_def(enum_pairs)}\n\n"""
        f"""{build_protocol_msg_debug_impl(enum_pairs)}\n\n"""
        # f"""{build_protocol_msg_impl()}\n\n"""
        f"""{build_send_error_struct_def()}\n\n"""
        f"""{build_actix_message_impl(enum_pairs)}\n\n"""
        f"""{build_into_enum_trait_def()}\n\n"""
        f"""{build_into_enum_impls(enum_pairs)}\n\n"""
        f"""{build_encode_trait_def()}\n\n"""
        f"""{build_encode_impls(enum_pairs)}\n\n"""
        f"""{build_encode_fn_def(enum_pairs)}\n\n"""
        f"""{build_decode_fn_def(enum_pairs)}\n\n"""
        f"""{build_set_round_ref_fn_def(enum_pairs)}\n\n"""
        f"""{build_get_round_ref_fn_def(enum_pairs)}"""
    )
    output_file_name = f"""../src/protocol/{module_name}_ext.rs"""
    with open(output_file_name, "w") as output_file:
        output_file.write(output)


if __name__ == "__main__":
    proto_file, enum_type_name = parse()
    module_name = re.sub(r"([^.]+).proto$", r"\1", basename(proto_file.name))
    content = proto_file.read().replace("\n", "")
    enum_pairs = extract(content, enum_type_name)

    output(module_name, enum_pairs)
