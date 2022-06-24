#!/bin/bash

current_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )";
cd ${current_dir}

# Fetch proto files
if [[ ! -d proto ]]; then
    git clone git@bitbros.com:/candlebase-protocol proto;
fi

output_dir="../src/protocol"
mkdir -p ${output_dir}

touch ${output_dir}/mod.rs
echo "
mod candlebase_protocol;
mod candlebase_protocol_ext;

pub use candlebase_protocol::*;
pub use candlebase_protocol_ext::*;
" > ${output_dir}/mod.rs

# Generate xxx file by protoc
cargo run && mv ${output_dir}/candlebase.protocol.rs ${output_dir}/candlebase_protocol.rs

# bin/normalize_type_name.py --proto_file proto/candlebase_protocol.proto

# Generate xxx_ext file
bin/gen_protocol_ext.py \
    --proto_file proto/candlebase_protocol.proto \
    --enum_type_name MsgType
