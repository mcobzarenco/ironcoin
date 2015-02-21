// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct HashedBlock {
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signed_block: ::protobuf::SingularPtrField<SignedBlock>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl HashedBlock {
    pub fn new() -> HashedBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HashedBlock {
        static mut instance: ::protobuf::lazy::Lazy<HashedBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HashedBlock,
        };
        unsafe {
            instance.get(|| {
                HashedBlock {
                    hash: ::protobuf::SingularField::none(),
                    signed_block: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash.set_default();
        };
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash<'a>(&'a self) -> &'a [u8] {
        match self.hash.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .simples.SignedBlock signed_block = 2;

    pub fn clear_signed_block(&mut self) {
        self.signed_block.clear();
    }

    pub fn has_signed_block(&self) -> bool {
        self.signed_block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signed_block(&mut self, v: SignedBlock) {
        self.signed_block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signed_block<'a>(&'a mut self) -> &'a mut SignedBlock {
        if self.signed_block.is_none() {
            self.signed_block.set_default();
        };
        self.signed_block.as_mut().unwrap()
    }

    // Take field
    pub fn take_signed_block(&mut self) -> SignedBlock {
        self.signed_block.take().unwrap_or_else(|| SignedBlock::new())
    }

    pub fn get_signed_block<'a>(&'a self) -> &'a SignedBlock {
        self.signed_block.as_ref().unwrap_or_else(|| SignedBlock::default_instance())
    }
}

impl ::protobuf::Message for HashedBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hash.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.signed_block.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.hash.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.signed_block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.signed_block.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HashedBlock>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HashedBlock {
    fn new() -> HashedBlock {
        HashedBlock::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<HashedBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "hash",
                    HashedBlock::has_hash,
                    HashedBlock::get_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "signed_block",
                    HashedBlock::has_signed_block,
                    HashedBlock::get_signed_block,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HashedBlock>(
                    "HashedBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HashedBlock {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_signed_block();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HashedBlock {
    fn eq(&self, other: &HashedBlock) -> bool {
        self.hash == other.hash &&
        self.signed_block == other.signed_block &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HashedBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SignedBlock {
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    block: ::protobuf::SingularPtrField<Block>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SignedBlock {
    pub fn new() -> SignedBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedBlock {
        static mut instance: ::protobuf::lazy::Lazy<SignedBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedBlock,
        };
        unsafe {
            instance.get(|| {
                SignedBlock {
                    signature: ::protobuf::SingularField::none(),
                    block: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        };
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature<'a>(&'a self) -> &'a [u8] {
        match self.signature.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .simples.Block block = 2;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: Block) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block<'a>(&'a mut self) -> &'a mut Block {
        if self.block.is_none() {
            self.block.set_default();
        };
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> Block {
        self.block.take().unwrap_or_else(|| Block::new())
    }

    pub fn get_block<'a>(&'a self) -> &'a Block {
        self.block.as_ref().unwrap_or_else(|| Block::default_instance())
    }
}

impl ::protobuf::Message for SignedBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.signature.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.block.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.signature.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signature.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.block.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SignedBlock>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SignedBlock {
    fn new() -> SignedBlock {
        SignedBlock::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<SignedBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "signature",
                    SignedBlock::has_signature,
                    SignedBlock::get_signature,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    SignedBlock::has_block,
                    SignedBlock::get_block,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedBlock>(
                    "SignedBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedBlock {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SignedBlock {
    fn eq(&self, other: &SignedBlock) -> bool {
        self.signature == other.signature &&
        self.block == other.block &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SignedBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Block {
    staker_pk: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    previous: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<i64>,
    height: ::std::option::Option<u32>,
    target_hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    transactions: ::protobuf::RepeatedField<Transaction>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(|| {
                Block {
                    staker_pk: ::protobuf::SingularField::none(),
                    previous: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    target_hash: ::protobuf::SingularField::none(),
                    transactions: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes staker_pk = 1;

    pub fn clear_staker_pk(&mut self) {
        self.staker_pk.clear();
    }

    pub fn has_staker_pk(&self) -> bool {
        self.staker_pk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_staker_pk(&mut self, v: ::std::vec::Vec<u8>) {
        self.staker_pk = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_staker_pk<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.staker_pk.is_none() {
            self.staker_pk.set_default();
        };
        self.staker_pk.as_mut().unwrap()
    }

    // Take field
    pub fn take_staker_pk(&mut self) -> ::std::vec::Vec<u8> {
        self.staker_pk.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_staker_pk<'a>(&'a self) -> &'a [u8] {
        match self.staker_pk.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional bytes previous = 2;

    pub fn clear_previous(&mut self) {
        self.previous.clear();
    }

    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previous(&mut self, v: ::std::vec::Vec<u8>) {
        self.previous = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.previous.is_none() {
            self.previous.set_default();
        };
        self.previous.as_mut().unwrap()
    }

    // Take field
    pub fn take_previous(&mut self) -> ::std::vec::Vec<u8> {
        self.previous.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_previous<'a>(&'a self) -> &'a [u8] {
        match self.previous.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    // optional uint32 height = 4;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> u32 {
        self.height.unwrap_or(0)
    }

    // optional bytes target_hash = 5;

    pub fn clear_target_hash(&mut self) {
        self.target_hash.clear();
    }

    pub fn has_target_hash(&self) -> bool {
        self.target_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.target_hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target_hash<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.target_hash.is_none() {
            self.target_hash.set_default();
        };
        self.target_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_target_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.target_hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_target_hash<'a>(&'a self) -> &'a [u8] {
        match self.target_hash.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // repeated .simples.Transaction transactions = 6;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<Transaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<Transaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions<'a>(&'a self) -> &'a [Transaction] {
        self.transactions.as_slice()
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.staker_pk.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.previous.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.target_hash.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.staker_pk.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.previous.iter() {
            my_size += ::protobuf::rt::bytes_size(2, value.as_slice());
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.target_hash.iter() {
            my_size += ::protobuf::rt::bytes_size(5, value.as_slice());
        };
        for value in self.transactions.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.staker_pk.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.previous.as_ref() {
            try!(os.write_bytes(2, v.as_slice()));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.height {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.target_hash.as_ref() {
            try!(os.write_bytes(5, v.as_slice()));
        };
        for v in self.transactions.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Block>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "staker_pk",
                    Block::has_staker_pk,
                    Block::get_staker_pk,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "previous",
                    Block::has_previous,
                    Block::get_previous,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp",
                    Block::has_timestamp,
                    Block::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "height",
                    Block::has_height,
                    Block::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "target_hash",
                    Block::has_target_hash,
                    Block::get_target_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "transactions",
                    Block::get_transactions,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_staker_pk();
        self.clear_previous();
        self.clear_timestamp();
        self.clear_height();
        self.clear_target_hash();
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.staker_pk == other.staker_pk &&
        self.previous == other.previous &&
        self.timestamp == other.timestamp &&
        self.height == other.height &&
        self.target_hash == other.target_hash &&
        self.transactions == other.transactions &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Commitment {
    tx_type: ::std::option::Option<Commitment_Type>,
    bounty_pk: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    bounty: ::std::option::Option<u64>,
    transfers: ::protobuf::RepeatedField<Transfer>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Commitment {
    pub fn new() -> Commitment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Commitment {
        static mut instance: ::protobuf::lazy::Lazy<Commitment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Commitment,
        };
        unsafe {
            instance.get(|| {
                Commitment {
                    tx_type: ::std::option::Option::None,
                    bounty_pk: ::protobuf::SingularField::none(),
                    bounty: ::std::option::Option::None,
                    transfers: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.Commitment.Type tx_type = 1;

    pub fn clear_tx_type(&mut self) {
        self.tx_type = ::std::option::Option::None;
    }

    pub fn has_tx_type(&self) -> bool {
        self.tx_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tx_type(&mut self, v: Commitment_Type) {
        self.tx_type = ::std::option::Option::Some(v);
    }

    pub fn get_tx_type<'a>(&self) -> Commitment_Type {
        self.tx_type.unwrap_or(Commitment_Type::INVALID)
    }

    // optional bytes bounty_pk = 2;

    pub fn clear_bounty_pk(&mut self) {
        self.bounty_pk.clear();
    }

    pub fn has_bounty_pk(&self) -> bool {
        self.bounty_pk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounty_pk(&mut self, v: ::std::vec::Vec<u8>) {
        self.bounty_pk = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bounty_pk<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.bounty_pk.is_none() {
            self.bounty_pk.set_default();
        };
        self.bounty_pk.as_mut().unwrap()
    }

    // Take field
    pub fn take_bounty_pk(&mut self) -> ::std::vec::Vec<u8> {
        self.bounty_pk.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bounty_pk<'a>(&'a self) -> &'a [u8] {
        match self.bounty_pk.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional uint64 bounty = 3;

    pub fn clear_bounty(&mut self) {
        self.bounty = ::std::option::Option::None;
    }

    pub fn has_bounty(&self) -> bool {
        self.bounty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounty(&mut self, v: u64) {
        self.bounty = ::std::option::Option::Some(v);
    }

    pub fn get_bounty<'a>(&self) -> u64 {
        self.bounty.unwrap_or(0)
    }

    // repeated .simples.Transfer transfers = 4;

    pub fn clear_transfers(&mut self) {
        self.transfers.clear();
    }

    // Param is passed by value, moved
    pub fn set_transfers(&mut self, v: ::protobuf::RepeatedField<Transfer>) {
        self.transfers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transfers<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Transfer> {
        &mut self.transfers
    }

    // Take field
    pub fn take_transfers(&mut self) -> ::protobuf::RepeatedField<Transfer> {
        ::std::mem::replace(&mut self.transfers, ::protobuf::RepeatedField::new())
    }

    pub fn get_transfers<'a>(&'a self) -> &'a [Transfer] {
        self.transfers.as_slice()
    }
}

impl ::protobuf::Message for Commitment {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.tx_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bounty_pk.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.bounty = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transfers));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tx_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.bounty_pk.iter() {
            my_size += ::protobuf::rt::bytes_size(2, value.as_slice());
        };
        for value in self.bounty.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.transfers.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tx_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.bounty_pk.as_ref() {
            try!(os.write_bytes(2, v.as_slice()));
        };
        if let Some(v) = self.bounty {
            try!(os.write_uint64(3, v));
        };
        for v in self.transfers.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Commitment>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Commitment {
    fn new() -> Commitment {
        Commitment::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Commitment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "tx_type",
                    Commitment::has_tx_type,
                    Commitment::get_tx_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bounty_pk",
                    Commitment::has_bounty_pk,
                    Commitment::get_bounty_pk,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "bounty",
                    Commitment::has_bounty,
                    Commitment::get_bounty,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "transfers",
                    Commitment::get_transfers,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Commitment>(
                    "Commitment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Commitment {
    fn clear(&mut self) {
        self.clear_tx_type();
        self.clear_bounty_pk();
        self.clear_bounty();
        self.clear_transfers();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Commitment {
    fn eq(&self, other: &Commitment) -> bool {
        self.tx_type == other.tx_type &&
        self.bounty_pk == other.bounty_pk &&
        self.bounty == other.bounty &&
        self.transfers == other.transfers &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Commitment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Commitment_Type {
    INVALID = 0,
    TRANSFER = 1,
}

impl ::protobuf::ProtobufEnum for Commitment_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Commitment_Type> {
        match value {
            0 => ::std::option::Option::Some(Commitment_Type::INVALID),
            1 => ::std::option::Option::Some(Commitment_Type::TRANSFER),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Commitment_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Commitment_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Commitment_Type {
}

#[derive(Clone,Default)]
pub struct DetachedSignature {
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DetachedSignature {
    pub fn new() -> DetachedSignature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DetachedSignature {
        static mut instance: ::protobuf::lazy::Lazy<DetachedSignature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DetachedSignature,
        };
        unsafe {
            instance.get(|| {
                DetachedSignature {
                    public_key: ::protobuf::SingularField::none(),
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes public_key = 1;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key<'a>(&'a self) -> &'a [u8] {
        match self.public_key.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload<'a>(&'a self) -> &'a [u8] {
        match self.payload.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for DetachedSignature {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.public_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.payload.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.public_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::bytes_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(2, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DetachedSignature>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DetachedSignature {
    fn new() -> DetachedSignature {
        DetachedSignature::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<DetachedSignature>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    DetachedSignature::has_public_key,
                    DetachedSignature::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    DetachedSignature::has_payload,
                    DetachedSignature::get_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DetachedSignature>(
                    "DetachedSignature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DetachedSignature {
    fn clear(&mut self) {
        self.clear_public_key();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DetachedSignature {
    fn eq(&self, other: &DetachedSignature) -> bool {
        self.public_key == other.public_key &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DetachedSignature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Transaction {
    signatures: ::protobuf::RepeatedField<DetachedSignature>,
    commit: ::protobuf::SingularPtrField<Commitment>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(|| {
                Transaction {
                    signatures: ::protobuf::RepeatedField::new(),
                    commit: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .simples.DetachedSignature signatures = 1;

    pub fn clear_signatures(&mut self) {
        self.signatures.clear();
    }

    // Param is passed by value, moved
    pub fn set_signatures(&mut self, v: ::protobuf::RepeatedField<DetachedSignature>) {
        self.signatures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signatures<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<DetachedSignature> {
        &mut self.signatures
    }

    // Take field
    pub fn take_signatures(&mut self) -> ::protobuf::RepeatedField<DetachedSignature> {
        ::std::mem::replace(&mut self.signatures, ::protobuf::RepeatedField::new())
    }

    pub fn get_signatures<'a>(&'a self) -> &'a [DetachedSignature] {
        self.signatures.as_slice()
    }

    // optional .simples.Commitment commit = 2;

    pub fn clear_commit(&mut self) {
        self.commit.clear();
    }

    pub fn has_commit(&self) -> bool {
        self.commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: Commitment) {
        self.commit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit<'a>(&'a mut self) -> &'a mut Commitment {
        if self.commit.is_none() {
            self.commit.set_default();
        };
        self.commit.as_mut().unwrap()
    }

    // Take field
    pub fn take_commit(&mut self) -> Commitment {
        self.commit.take().unwrap_or_else(|| Commitment::new())
    }

    pub fn get_commit<'a>(&'a self) -> &'a Commitment {
        self.commit.as_ref().unwrap_or_else(|| Commitment::default_instance())
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.signatures));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.commit.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.signatures.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.commit.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.signatures.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.commit.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Transaction>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "signatures",
                    Transaction::get_signatures,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "commit",
                    Transaction::has_commit,
                    Transaction::get_commit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_signatures();
        self.clear_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Transaction {
    fn eq(&self, other: &Transaction) -> bool {
        self.signatures == other.signatures &&
        self.commit == other.commit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Transfer {
    op_index: ::std::option::Option<u32>,
    tokens: ::std::option::Option<u64>,
    source_pk: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    destination_pk: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Transfer {
    pub fn new() -> Transfer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transfer {
        static mut instance: ::protobuf::lazy::Lazy<Transfer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transfer,
        };
        unsafe {
            instance.get(|| {
                Transfer {
                    op_index: ::std::option::Option::None,
                    tokens: ::std::option::Option::None,
                    source_pk: ::protobuf::SingularField::none(),
                    destination_pk: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 op_index = 1;

    pub fn clear_op_index(&mut self) {
        self.op_index = ::std::option::Option::None;
    }

    pub fn has_op_index(&self) -> bool {
        self.op_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op_index(&mut self, v: u32) {
        self.op_index = ::std::option::Option::Some(v);
    }

    pub fn get_op_index<'a>(&self) -> u32 {
        self.op_index.unwrap_or(0)
    }

    // optional uint64 tokens = 2;

    pub fn clear_tokens(&mut self) {
        self.tokens = ::std::option::Option::None;
    }

    pub fn has_tokens(&self) -> bool {
        self.tokens.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: u64) {
        self.tokens = ::std::option::Option::Some(v);
    }

    pub fn get_tokens<'a>(&self) -> u64 {
        self.tokens.unwrap_or(0)
    }

    // optional bytes source_pk = 3;

    pub fn clear_source_pk(&mut self) {
        self.source_pk.clear();
    }

    pub fn has_source_pk(&self) -> bool {
        self.source_pk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_pk(&mut self, v: ::std::vec::Vec<u8>) {
        self.source_pk = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_pk<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.source_pk.is_none() {
            self.source_pk.set_default();
        };
        self.source_pk.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_pk(&mut self) -> ::std::vec::Vec<u8> {
        self.source_pk.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_source_pk<'a>(&'a self) -> &'a [u8] {
        match self.source_pk.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional bytes destination_pk = 4;

    pub fn clear_destination_pk(&mut self) {
        self.destination_pk.clear();
    }

    pub fn has_destination_pk(&self) -> bool {
        self.destination_pk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destination_pk(&mut self, v: ::std::vec::Vec<u8>) {
        self.destination_pk = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination_pk<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.destination_pk.is_none() {
            self.destination_pk.set_default();
        };
        self.destination_pk.as_mut().unwrap()
    }

    // Take field
    pub fn take_destination_pk(&mut self) -> ::std::vec::Vec<u8> {
        self.destination_pk.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_destination_pk<'a>(&'a self) -> &'a [u8] {
        match self.destination_pk.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for Transfer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.op_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.tokens = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.source_pk.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.destination_pk.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.op_index.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.tokens.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.source_pk.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        for value in self.destination_pk.iter() {
            my_size += ::protobuf::rt::bytes_size(4, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.op_index {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.tokens {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.source_pk.as_ref() {
            try!(os.write_bytes(3, v.as_slice()));
        };
        if let Some(v) = self.destination_pk.as_ref() {
            try!(os.write_bytes(4, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Transfer>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Transfer {
    fn new() -> Transfer {
        Transfer::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Transfer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "op_index",
                    Transfer::has_op_index,
                    Transfer::get_op_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "tokens",
                    Transfer::has_tokens,
                    Transfer::get_tokens,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "source_pk",
                    Transfer::has_source_pk,
                    Transfer::get_source_pk,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "destination_pk",
                    Transfer::has_destination_pk,
                    Transfer::get_destination_pk,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transfer>(
                    "Transfer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transfer {
    fn clear(&mut self) {
        self.clear_op_index();
        self.clear_tokens();
        self.clear_source_pk();
        self.clear_destination_pk();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Transfer {
    fn eq(&self, other: &Transfer) -> bool {
        self.op_index == other.op_index &&
        self.tokens == other.tokens &&
        self.source_pk == other.source_pk &&
        self.destination_pk == other.destination_pk &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Transfer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Balance {
    tokens: ::std::option::Option<u64>,
    op_index: ::std::option::Option<u32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Balance {
    pub fn new() -> Balance {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Balance {
        static mut instance: ::protobuf::lazy::Lazy<Balance> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Balance,
        };
        unsafe {
            instance.get(|| {
                Balance {
                    tokens: ::std::option::Option::None,
                    op_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 tokens = 2;

    pub fn clear_tokens(&mut self) {
        self.tokens = ::std::option::Option::None;
    }

    pub fn has_tokens(&self) -> bool {
        self.tokens.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: u64) {
        self.tokens = ::std::option::Option::Some(v);
    }

    pub fn get_tokens<'a>(&self) -> u64 {
        self.tokens.unwrap_or(0)
    }

    // optional uint32 op_index = 3;

    pub fn clear_op_index(&mut self) {
        self.op_index = ::std::option::Option::None;
    }

    pub fn has_op_index(&self) -> bool {
        self.op_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op_index(&mut self, v: u32) {
        self.op_index = ::std::option::Option::Some(v);
    }

    pub fn get_op_index<'a>(&self) -> u32 {
        self.op_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for Balance {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.tokens = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.op_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tokens.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.op_index.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tokens {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op_index {
            try!(os.write_uint32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Balance>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Balance {
    fn new() -> Balance {
        Balance::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Balance>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "tokens",
                    Balance::has_tokens,
                    Balance::get_tokens,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "op_index",
                    Balance::has_op_index,
                    Balance::get_op_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Balance>(
                    "Balance",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Balance {
    fn clear(&mut self) {
        self.clear_tokens();
        self.clear_op_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Balance {
    fn eq(&self, other: &Balance) -> bool {
        self.tokens == other.tokens &&
        self.op_index == other.op_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Balance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BalancePatch {
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    before: ::protobuf::SingularPtrField<Balance>,
    after: ::protobuf::SingularPtrField<Balance>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BalancePatch {
    pub fn new() -> BalancePatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BalancePatch {
        static mut instance: ::protobuf::lazy::Lazy<BalancePatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BalancePatch,
        };
        unsafe {
            instance.get(|| {
                BalancePatch {
                    public_key: ::protobuf::SingularField::none(),
                    before: ::protobuf::SingularPtrField::none(),
                    after: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes public_key = 1;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key<'a>(&'a self) -> &'a [u8] {
        match self.public_key.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .simples.Balance before = 2;

    pub fn clear_before(&mut self) {
        self.before.clear();
    }

    pub fn has_before(&self) -> bool {
        self.before.is_some()
    }

    // Param is passed by value, moved
    pub fn set_before(&mut self, v: Balance) {
        self.before = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_before<'a>(&'a mut self) -> &'a mut Balance {
        if self.before.is_none() {
            self.before.set_default();
        };
        self.before.as_mut().unwrap()
    }

    // Take field
    pub fn take_before(&mut self) -> Balance {
        self.before.take().unwrap_or_else(|| Balance::new())
    }

    pub fn get_before<'a>(&'a self) -> &'a Balance {
        self.before.as_ref().unwrap_or_else(|| Balance::default_instance())
    }

    // optional .simples.Balance after = 3;

    pub fn clear_after(&mut self) {
        self.after.clear();
    }

    pub fn has_after(&self) -> bool {
        self.after.is_some()
    }

    // Param is passed by value, moved
    pub fn set_after(&mut self, v: Balance) {
        self.after = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_after<'a>(&'a mut self) -> &'a mut Balance {
        if self.after.is_none() {
            self.after.set_default();
        };
        self.after.as_mut().unwrap()
    }

    // Take field
    pub fn take_after(&mut self) -> Balance {
        self.after.take().unwrap_or_else(|| Balance::new())
    }

    pub fn get_after<'a>(&'a self) -> &'a Balance {
        self.after.as_ref().unwrap_or_else(|| Balance::default_instance())
    }
}

impl ::protobuf::Message for BalancePatch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.public_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.before.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.after.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.public_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.before.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.after.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.before.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.after.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BalancePatch>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BalancePatch {
    fn new() -> BalancePatch {
        BalancePatch::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<BalancePatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    BalancePatch::has_public_key,
                    BalancePatch::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "before",
                    BalancePatch::has_before,
                    BalancePatch::get_before,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "after",
                    BalancePatch::has_after,
                    BalancePatch::get_after,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BalancePatch>(
                    "BalancePatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BalancePatch {
    fn clear(&mut self) {
        self.clear_public_key();
        self.clear_before();
        self.clear_after();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BalancePatch {
    fn eq(&self, other: &BalancePatch) -> bool {
        self.public_key == other.public_key &&
        self.before == other.before &&
        self.after == other.after &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BalancePatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlockWithDiff {
    hashed_block: ::protobuf::SingularPtrField<HashedBlock>,
    diff: ::protobuf::RepeatedField<BalancePatch>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlockWithDiff {
    pub fn new() -> BlockWithDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockWithDiff {
        static mut instance: ::protobuf::lazy::Lazy<BlockWithDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockWithDiff,
        };
        unsafe {
            instance.get(|| {
                BlockWithDiff {
                    hashed_block: ::protobuf::SingularPtrField::none(),
                    diff: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.HashedBlock hashed_block = 1;

    pub fn clear_hashed_block(&mut self) {
        self.hashed_block.clear();
    }

    pub fn has_hashed_block(&self) -> bool {
        self.hashed_block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hashed_block(&mut self, v: HashedBlock) {
        self.hashed_block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hashed_block<'a>(&'a mut self) -> &'a mut HashedBlock {
        if self.hashed_block.is_none() {
            self.hashed_block.set_default();
        };
        self.hashed_block.as_mut().unwrap()
    }

    // Take field
    pub fn take_hashed_block(&mut self) -> HashedBlock {
        self.hashed_block.take().unwrap_or_else(|| HashedBlock::new())
    }

    pub fn get_hashed_block<'a>(&'a self) -> &'a HashedBlock {
        self.hashed_block.as_ref().unwrap_or_else(|| HashedBlock::default_instance())
    }

    // repeated .simples.BalancePatch diff = 2;

    pub fn clear_diff(&mut self) {
        self.diff.clear();
    }

    // Param is passed by value, moved
    pub fn set_diff(&mut self, v: ::protobuf::RepeatedField<BalancePatch>) {
        self.diff = v;
    }

    // Mutable pointer to the field.
    pub fn mut_diff<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<BalancePatch> {
        &mut self.diff
    }

    // Take field
    pub fn take_diff(&mut self) -> ::protobuf::RepeatedField<BalancePatch> {
        ::std::mem::replace(&mut self.diff, ::protobuf::RepeatedField::new())
    }

    pub fn get_diff<'a>(&'a self) -> &'a [BalancePatch] {
        self.diff.as_slice()
    }
}

impl ::protobuf::Message for BlockWithDiff {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hashed_block.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.diff));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.hashed_block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.diff.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hashed_block.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.diff.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlockWithDiff>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockWithDiff {
    fn new() -> BlockWithDiff {
        BlockWithDiff::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<BlockWithDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "hashed_block",
                    BlockWithDiff::has_hashed_block,
                    BlockWithDiff::get_hashed_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "diff",
                    BlockWithDiff::get_diff,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockWithDiff>(
                    "BlockWithDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockWithDiff {
    fn clear(&mut self) {
        self.clear_hashed_block();
        self.clear_diff();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlockWithDiff {
    fn eq(&self, other: &BlockWithDiff) -> bool {
        self.hashed_block == other.hashed_block &&
        self.diff == other.diff &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlockWithDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Wallet {
    keys: ::protobuf::RepeatedField<WalletKey>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Wallet {
    pub fn new() -> Wallet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Wallet {
        static mut instance: ::protobuf::lazy::Lazy<Wallet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Wallet,
        };
        unsafe {
            instance.get(|| {
                Wallet {
                    keys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .simples.WalletKey keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<WalletKey>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<WalletKey> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<WalletKey> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [WalletKey] {
        self.keys.as_slice()
    }
}

impl ::protobuf::Message for Wallet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.keys.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.keys.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Wallet>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Wallet {
    fn new() -> Wallet {
        Wallet::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Wallet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    Wallet::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Wallet>(
                    "Wallet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Wallet {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Wallet {
    fn eq(&self, other: &Wallet) -> bool {
        self.keys == other.keys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WalletKey {
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    secret_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WalletKey {
    pub fn new() -> WalletKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WalletKey {
        static mut instance: ::protobuf::lazy::Lazy<WalletKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WalletKey,
        };
        unsafe {
            instance.get(|| {
                WalletKey {
                    public_key: ::protobuf::SingularField::none(),
                    secret_key: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes public_key = 1;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key<'a>(&'a self) -> &'a [u8] {
        match self.public_key.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional bytes secret_key = 2;

    pub fn clear_secret_key(&mut self) {
        self.secret_key.clear();
    }

    pub fn has_secret_key(&self) -> bool {
        self.secret_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.secret_key.is_none() {
            self.secret_key.set_default();
        };
        self.secret_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_secret_key(&mut self) -> ::std::vec::Vec<u8> {
        self.secret_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_secret_key<'a>(&'a self) -> &'a [u8] {
        match self.secret_key.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for WalletKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.public_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.secret_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.public_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.secret_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, value.as_slice());
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(3, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.secret_key.as_ref() {
            try!(os.write_bytes(2, v.as_slice()));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WalletKey>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WalletKey {
    fn new() -> WalletKey {
        WalletKey::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<WalletKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    WalletKey::has_public_key,
                    WalletKey::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "secret_key",
                    WalletKey::has_secret_key,
                    WalletKey::get_secret_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    WalletKey::has_name,
                    WalletKey::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WalletKey>(
                    "WalletKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WalletKey {
    fn clear(&mut self) {
        self.clear_public_key();
        self.clear_secret_key();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WalletKey {
    fn eq(&self, other: &WalletKey) -> bool {
        self.public_key == other.public_key &&
        self.secret_key == other.secret_key &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WalletKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpcRequest {
    method: ::std::option::Option<RpcRequest_Method>,
    pub_block: ::protobuf::SingularPtrField<PublishBlockRequest>,
    pub_transaction: ::protobuf::SingularPtrField<PublishTransactionRequest>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RpcRequest {
    pub fn new() -> RpcRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcRequest {
        static mut instance: ::protobuf::lazy::Lazy<RpcRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcRequest,
        };
        unsafe {
            instance.get(|| {
                RpcRequest {
                    method: ::std::option::Option::None,
                    pub_block: ::protobuf::SingularPtrField::none(),
                    pub_transaction: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.RpcRequest.Method method = 1;

    pub fn clear_method(&mut self) {
        self.method = ::std::option::Option::None;
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: RpcRequest_Method) {
        self.method = ::std::option::Option::Some(v);
    }

    pub fn get_method<'a>(&self) -> RpcRequest_Method {
        self.method.unwrap_or(RpcRequest_Method::INVALID)
    }

    // optional .simples.PublishBlockRequest pub_block = 3;

    pub fn clear_pub_block(&mut self) {
        self.pub_block.clear();
    }

    pub fn has_pub_block(&self) -> bool {
        self.pub_block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pub_block(&mut self, v: PublishBlockRequest) {
        self.pub_block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_block<'a>(&'a mut self) -> &'a mut PublishBlockRequest {
        if self.pub_block.is_none() {
            self.pub_block.set_default();
        };
        self.pub_block.as_mut().unwrap()
    }

    // Take field
    pub fn take_pub_block(&mut self) -> PublishBlockRequest {
        self.pub_block.take().unwrap_or_else(|| PublishBlockRequest::new())
    }

    pub fn get_pub_block<'a>(&'a self) -> &'a PublishBlockRequest {
        self.pub_block.as_ref().unwrap_or_else(|| PublishBlockRequest::default_instance())
    }

    // optional .simples.PublishTransactionRequest pub_transaction = 2;

    pub fn clear_pub_transaction(&mut self) {
        self.pub_transaction.clear();
    }

    pub fn has_pub_transaction(&self) -> bool {
        self.pub_transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pub_transaction(&mut self, v: PublishTransactionRequest) {
        self.pub_transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_transaction<'a>(&'a mut self) -> &'a mut PublishTransactionRequest {
        if self.pub_transaction.is_none() {
            self.pub_transaction.set_default();
        };
        self.pub_transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_pub_transaction(&mut self) -> PublishTransactionRequest {
        self.pub_transaction.take().unwrap_or_else(|| PublishTransactionRequest::new())
    }

    pub fn get_pub_transaction<'a>(&'a self) -> &'a PublishTransactionRequest {
        self.pub_transaction.as_ref().unwrap_or_else(|| PublishTransactionRequest::default_instance())
    }
}

impl ::protobuf::Message for RpcRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.method = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pub_block.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pub_transaction.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.method.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.pub_block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pub_transaction.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.method {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.pub_block.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pub_transaction.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RpcRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcRequest {
    fn new() -> RpcRequest {
        RpcRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<RpcRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "method",
                    RpcRequest::has_method,
                    RpcRequest::get_method,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pub_block",
                    RpcRequest::has_pub_block,
                    RpcRequest::get_pub_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pub_transaction",
                    RpcRequest::has_pub_transaction,
                    RpcRequest::get_pub_transaction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcRequest>(
                    "RpcRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcRequest {
    fn clear(&mut self) {
        self.clear_method();
        self.clear_pub_block();
        self.clear_pub_transaction();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpcRequest {
    fn eq(&self, other: &RpcRequest) -> bool {
        self.method == other.method &&
        self.pub_block == other.pub_block &&
        self.pub_transaction == other.pub_transaction &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpcRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum RpcRequest_Method {
    INVALID = 0,
    PUBLISH_TRANSACTION = 1,
    PUBLISH_BLOCK = 2,
}

impl ::protobuf::ProtobufEnum for RpcRequest_Method {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcRequest_Method> {
        match value {
            0 => ::std::option::Option::Some(RpcRequest_Method::INVALID),
            1 => ::std::option::Option::Some(RpcRequest_Method::PUBLISH_TRANSACTION),
            2 => ::std::option::Option::Some(RpcRequest_Method::PUBLISH_BLOCK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<RpcRequest_Method>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcRequest_Method", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcRequest_Method {
}

#[derive(Clone,Default)]
pub struct RpcResponse {
    status: ::std::option::Option<RpcResponse_Status>,
    description: ::protobuf::SingularField<::std::string::String>,
    pub_transaction: ::protobuf::SingularPtrField<PublishTransactionResponse>,
    pub_block: ::protobuf::SingularPtrField<PublishBlockResponse>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RpcResponse {
    pub fn new() -> RpcResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcResponse {
        static mut instance: ::protobuf::lazy::Lazy<RpcResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcResponse,
        };
        unsafe {
            instance.get(|| {
                RpcResponse {
                    status: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
                    pub_transaction: ::protobuf::SingularPtrField::none(),
                    pub_block: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.RpcResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: RpcResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> RpcResponse_Status {
        self.status.unwrap_or(RpcResponse_Status::INVALID)
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description<'a>(&'a self) -> &'a str {
        match self.description.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .simples.PublishTransactionResponse pub_transaction = 3;

    pub fn clear_pub_transaction(&mut self) {
        self.pub_transaction.clear();
    }

    pub fn has_pub_transaction(&self) -> bool {
        self.pub_transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pub_transaction(&mut self, v: PublishTransactionResponse) {
        self.pub_transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_transaction<'a>(&'a mut self) -> &'a mut PublishTransactionResponse {
        if self.pub_transaction.is_none() {
            self.pub_transaction.set_default();
        };
        self.pub_transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_pub_transaction(&mut self) -> PublishTransactionResponse {
        self.pub_transaction.take().unwrap_or_else(|| PublishTransactionResponse::new())
    }

    pub fn get_pub_transaction<'a>(&'a self) -> &'a PublishTransactionResponse {
        self.pub_transaction.as_ref().unwrap_or_else(|| PublishTransactionResponse::default_instance())
    }

    // optional .simples.PublishBlockResponse pub_block = 4;

    pub fn clear_pub_block(&mut self) {
        self.pub_block.clear();
    }

    pub fn has_pub_block(&self) -> bool {
        self.pub_block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pub_block(&mut self, v: PublishBlockResponse) {
        self.pub_block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_block<'a>(&'a mut self) -> &'a mut PublishBlockResponse {
        if self.pub_block.is_none() {
            self.pub_block.set_default();
        };
        self.pub_block.as_mut().unwrap()
    }

    // Take field
    pub fn take_pub_block(&mut self) -> PublishBlockResponse {
        self.pub_block.take().unwrap_or_else(|| PublishBlockResponse::new())
    }

    pub fn get_pub_block<'a>(&'a self) -> &'a PublishBlockResponse {
        self.pub_block.as_ref().unwrap_or_else(|| PublishBlockResponse::default_instance())
    }
}

impl ::protobuf::Message for RpcResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.description.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pub_transaction.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pub_block.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.description.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.pub_transaction.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pub_block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(2, v.as_slice()));
        };
        if let Some(v) = self.pub_transaction.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pub_block.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RpcResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcResponse {
    fn new() -> RpcResponse {
        RpcResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<RpcResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    RpcResponse::has_status,
                    RpcResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    RpcResponse::has_description,
                    RpcResponse::get_description,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pub_transaction",
                    RpcResponse::has_pub_transaction,
                    RpcResponse::get_pub_transaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pub_block",
                    RpcResponse::has_pub_block,
                    RpcResponse::get_pub_block,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcResponse>(
                    "RpcResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_description();
        self.clear_pub_transaction();
        self.clear_pub_block();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpcResponse {
    fn eq(&self, other: &RpcResponse) -> bool {
        self.status == other.status &&
        self.description == other.description &&
        self.pub_transaction == other.pub_transaction &&
        self.pub_block == other.pub_block &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpcResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum RpcResponse_Status {
    INVALID = 0,
    OK = 1,
    INVALID_MESSAGE = 2,
    INVALID_METHOD = 3,
    INTERNAL_ERROR = 4,
}

impl ::protobuf::ProtobufEnum for RpcResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(RpcResponse_Status::INVALID),
            1 => ::std::option::Option::Some(RpcResponse_Status::OK),
            2 => ::std::option::Option::Some(RpcResponse_Status::INVALID_MESSAGE),
            3 => ::std::option::Option::Some(RpcResponse_Status::INVALID_METHOD),
            4 => ::std::option::Option::Some(RpcResponse_Status::INTERNAL_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<RpcResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcResponse_Status {
}

#[derive(Clone,Default)]
pub struct PublishTransactionRequest {
    transaction: ::protobuf::SingularPtrField<Transaction>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PublishTransactionRequest {
    pub fn new() -> PublishTransactionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishTransactionRequest {
        static mut instance: ::protobuf::lazy::Lazy<PublishTransactionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishTransactionRequest,
        };
        unsafe {
            instance.get(|| {
                PublishTransactionRequest {
                    transaction: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.Transaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: Transaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction<'a>(&'a mut self) -> &'a mut Transaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        };
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> Transaction {
        self.transaction.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_transaction<'a>(&'a self) -> &'a Transaction {
        self.transaction.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }
}

impl ::protobuf::Message for PublishTransactionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.transaction.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.transaction.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublishTransactionRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublishTransactionRequest {
    fn new() -> PublishTransactionRequest {
        PublishTransactionRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<PublishTransactionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "transaction",
                    PublishTransactionRequest::has_transaction,
                    PublishTransactionRequest::get_transaction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishTransactionRequest>(
                    "PublishTransactionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishTransactionRequest {
    fn clear(&mut self) {
        self.clear_transaction();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublishTransactionRequest {
    fn eq(&self, other: &PublishTransactionRequest) -> bool {
        self.transaction == other.transaction &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublishTransactionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PublishTransactionResponse {
    status: ::std::option::Option<PublishTransactionResponse_Status>,
    description: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PublishTransactionResponse {
    pub fn new() -> PublishTransactionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishTransactionResponse {
        static mut instance: ::protobuf::lazy::Lazy<PublishTransactionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishTransactionResponse,
        };
        unsafe {
            instance.get(|| {
                PublishTransactionResponse {
                    status: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.PublishTransactionResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: PublishTransactionResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> PublishTransactionResponse_Status {
        self.status.unwrap_or(PublishTransactionResponse_Status::INVALID)
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description<'a>(&'a self) -> &'a str {
        match self.description.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for PublishTransactionResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.description.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.description.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(2, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublishTransactionResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublishTransactionResponse {
    fn new() -> PublishTransactionResponse {
        PublishTransactionResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<PublishTransactionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    PublishTransactionResponse::has_status,
                    PublishTransactionResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    PublishTransactionResponse::has_description,
                    PublishTransactionResponse::get_description,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishTransactionResponse>(
                    "PublishTransactionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishTransactionResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublishTransactionResponse {
    fn eq(&self, other: &PublishTransactionResponse) -> bool {
        self.status == other.status &&
        self.description == other.description &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublishTransactionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum PublishTransactionResponse_Status {
    INVALID = 0,
    OK = 1,
    INVALID_REQUEST = 2,
}

impl ::protobuf::ProtobufEnum for PublishTransactionResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PublishTransactionResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(PublishTransactionResponse_Status::INVALID),
            1 => ::std::option::Option::Some(PublishTransactionResponse_Status::OK),
            2 => ::std::option::Option::Some(PublishTransactionResponse_Status::INVALID_REQUEST),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<PublishTransactionResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PublishTransactionResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PublishTransactionResponse_Status {
}

#[derive(Clone,Default)]
pub struct PublishBlockRequest {
    block: ::protobuf::SingularPtrField<HashedBlock>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PublishBlockRequest {
    pub fn new() -> PublishBlockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishBlockRequest {
        static mut instance: ::protobuf::lazy::Lazy<PublishBlockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishBlockRequest,
        };
        unsafe {
            instance.get(|| {
                PublishBlockRequest {
                    block: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.HashedBlock block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: HashedBlock) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block<'a>(&'a mut self) -> &'a mut HashedBlock {
        if self.block.is_none() {
            self.block.set_default();
        };
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> HashedBlock {
        self.block.take().unwrap_or_else(|| HashedBlock::new())
    }

    pub fn get_block<'a>(&'a self) -> &'a HashedBlock {
        self.block.as_ref().unwrap_or_else(|| HashedBlock::default_instance())
    }
}

impl ::protobuf::Message for PublishBlockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.block.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.block.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublishBlockRequest>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublishBlockRequest {
    fn new() -> PublishBlockRequest {
        PublishBlockRequest::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<PublishBlockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    PublishBlockRequest::has_block,
                    PublishBlockRequest::get_block,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishBlockRequest>(
                    "PublishBlockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishBlockRequest {
    fn clear(&mut self) {
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublishBlockRequest {
    fn eq(&self, other: &PublishBlockRequest) -> bool {
        self.block == other.block &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublishBlockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PublishBlockResponse {
    status: ::std::option::Option<PublishBlockResponse_Status>,
    description: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PublishBlockResponse {
    pub fn new() -> PublishBlockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishBlockResponse {
        static mut instance: ::protobuf::lazy::Lazy<PublishBlockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishBlockResponse,
        };
        unsafe {
            instance.get(|| {
                PublishBlockResponse {
                    status: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .simples.PublishBlockResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: PublishBlockResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> PublishBlockResponse_Status {
        self.status.unwrap_or(PublishBlockResponse_Status::INVALID)
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description<'a>(&'a self) -> &'a str {
        match self.description.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for PublishBlockResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.description.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.description.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(2, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublishBlockResponse>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublishBlockResponse {
    fn new() -> PublishBlockResponse {
        PublishBlockResponse::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<PublishBlockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    PublishBlockResponse::has_status,
                    PublishBlockResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    PublishBlockResponse::has_description,
                    PublishBlockResponse::get_description,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishBlockResponse>(
                    "PublishBlockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishBlockResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublishBlockResponse {
    fn eq(&self, other: &PublishBlockResponse) -> bool {
        self.status == other.status &&
        self.description == other.description &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublishBlockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum PublishBlockResponse_Status {
    INVALID = 0,
    OK = 1,
    INVALID_REQUEST = 2,
    INVALID_BLOCK = 3,
}

impl ::protobuf::ProtobufEnum for PublishBlockResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PublishBlockResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(PublishBlockResponse_Status::INVALID),
            1 => ::std::option::Option::Some(PublishBlockResponse_Status::OK),
            2 => ::std::option::Option::Some(PublishBlockResponse_Status::INVALID_REQUEST),
            3 => ::std::option::Option::Some(PublishBlockResponse_Status::INVALID_BLOCK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<PublishBlockResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PublishBlockResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PublishBlockResponse_Status {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x73, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x73, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x07, 0x73, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x73, 0x22, 0x47, 0x0a, 0x0b, 0x48, 0x61, 0x73, 0x68, 0x65, 0x64, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x0c, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x2a, 0x0a, 0x0c, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x5f, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c,
    0x65, 0x73, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x22, 0x3f,
    0x0a, 0x0b, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x11, 0x0a,
    0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x1d, 0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0e, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x22,
    0x90, 0x01, 0x0a, 0x05, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61,
    0x6b, 0x65, 0x72, 0x5f, 0x70, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08,
    0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11,
    0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x03, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x2a, 0x0a, 0x0c, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x73,
    0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x22, 0xa3, 0x01, 0x0a, 0x0a, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e,
    0x74, 0x12, 0x29, 0x0a, 0x07, 0x74, 0x78, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x18, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11, 0x0a, 0x09,
    0x62, 0x6f, 0x75, 0x6e, 0x74, 0x79, 0x5f, 0x70, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x0e, 0x0a, 0x06, 0x62, 0x6f, 0x75, 0x6e, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x24, 0x0a, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x73, 0x18, 0x04, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x66, 0x65, 0x72, 0x22, 0x21, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a,
    0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x54, 0x52,
    0x41, 0x4e, 0x53, 0x46, 0x45, 0x52, 0x10, 0x01, 0x22, 0x38, 0x0a, 0x11, 0x44, 0x65, 0x74, 0x61,
    0x63, 0x68, 0x65, 0x64, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x12, 0x0a,
    0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x62, 0x0a, 0x0b, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x2e, 0x0a, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e,
    0x44, 0x65, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
    0x65, 0x12, 0x23, 0x0a, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x13, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0x57, 0x0a, 0x08, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x65, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x70, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x70,
    0x6b, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x16, 0x0a, 0x0e, 0x64, 0x65, 0x73, 0x74, 0x69,
    0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0x2b, 0x0a, 0x07, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x74, 0x6f,
    0x6b, 0x65, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x70,
    0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x65, 0x0a, 0x0c,
    0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x61, 0x74, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x0a,
    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x20, 0x0a, 0x06, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x10, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x42, 0x61, 0x6c, 0x61, 0x6e,
    0x63, 0x65, 0x12, 0x1f, 0x0a, 0x05, 0x61, 0x66, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x42, 0x61, 0x6c, 0x61,
    0x6e, 0x63, 0x65, 0x22, 0x60, 0x0a, 0x0d, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x57, 0x69, 0x74, 0x68,
    0x44, 0x69, 0x66, 0x66, 0x12, 0x2a, 0x0a, 0x0c, 0x68, 0x61, 0x73, 0x68, 0x65, 0x64, 0x5f, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x73, 0x69, 0x6d,
    0x70, 0x6c, 0x65, 0x73, 0x2e, 0x48, 0x61, 0x73, 0x68, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x12, 0x23, 0x0a, 0x04, 0x64, 0x69, 0x66, 0x66, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65,
    0x50, 0x61, 0x74, 0x63, 0x68, 0x22, 0x2a, 0x0a, 0x06, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x12,
    0x20, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e,
    0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x4b, 0x65,
    0x79, 0x22, 0x41, 0x0a, 0x09, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x12,
    0x0a, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x22, 0xe9, 0x01, 0x0a, 0x0a, 0x52, 0x70, 0x63, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x2a, 0x0a, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x52, 0x70,
    0x63, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x12,
    0x2f, 0x0a, 0x09, 0x70, 0x75, 0x62, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x50, 0x75, 0x62,
    0x6c, 0x69, 0x73, 0x68, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x3b, 0x0a, 0x0f, 0x70, 0x75, 0x62, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x73, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x73, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x41, 0x0a,
    0x06, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c,
    0x49, 0x44, 0x10, 0x00, 0x12, 0x17, 0x0a, 0x13, 0x50, 0x55, 0x42, 0x4c, 0x49, 0x53, 0x48, 0x5f,
    0x54, 0x52, 0x41, 0x4e, 0x53, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x11, 0x0a,
    0x0d, 0x50, 0x55, 0x42, 0x4c, 0x49, 0x53, 0x48, 0x5f, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x10, 0x02,
    0x22, 0x9b, 0x02, 0x0a, 0x0b, 0x52, 0x70, 0x63, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x2b, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x1b, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x52, 0x70, 0x63, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x13, 0x0a,
    0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x3c, 0x0a, 0x0f, 0x70, 0x75, 0x62, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x30, 0x0a, 0x09, 0x70, 0x75, 0x62, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x73, 0x68, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x5a, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07,
    0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x06, 0x0a, 0x02, 0x4f, 0x4b, 0x10,
    0x01, 0x12, 0x13, 0x0a, 0x0f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49,
    0x44, 0x5f, 0x4d, 0x45, 0x54, 0x48, 0x4f, 0x44, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x4e,
    0x54, 0x45, 0x52, 0x4e, 0x41, 0x4c, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x04, 0x22, 0x46,
    0x0a, 0x19, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x0b, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa1, 0x01, 0x0a, 0x1a, 0x50, 0x75, 0x62, 0x6c, 0x69,
    0x73, 0x68, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3a, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2a, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e,
    0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x12, 0x13, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x32, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x0b, 0x0a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x06, 0x0a,
    0x02, 0x4f, 0x4b, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44,
    0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x02, 0x22, 0x3a, 0x0a, 0x13, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x73, 0x68, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x23, 0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x48, 0x61, 0x73, 0x68, 0x65,
    0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x22, 0xa8, 0x01, 0x0a, 0x14, 0x50, 0x75, 0x62, 0x6c, 0x69,
    0x73, 0x68, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x34, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x24, 0x2e, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73,
    0x68, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x13, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x45, 0x0a, 0x06, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10,
    0x00, 0x12, 0x06, 0x0a, 0x02, 0x4f, 0x4b, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x49, 0x4e, 0x56,
    0x41, 0x4c, 0x49, 0x44, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x02, 0x12, 0x11,
    0x0a, 0x0d, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x10,
    0x03, 0x4a, 0xff, 0x24, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04,
    0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x05, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x05, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x06, 0x0d, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x09, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0a, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x0b, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0b, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x13, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x0e, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x0e, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x10, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x13, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x11, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x11, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x11, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x12, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x13, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x13,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x13, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x13, 0x13, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x13, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x15, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x15, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x15, 0x28,
    0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x18, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04,
    0x00, 0x12, 0x04, 0x19, 0x04, 0x1c, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x19, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x1a, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1a, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x1a, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x1b, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x1e, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x13, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x1f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1f, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1f,
    0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x20, 0x04, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x20, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x20, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x23,
    0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x23, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x24, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x24, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x25, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x28, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x28, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x29, 0x04, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x1f, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x29, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03,
    0x2a, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2a, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x2d, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x2d, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x04, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x2f, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x14, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x30, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x30, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x30, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x31, 0x04, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x31, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x31, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x36,
    0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x36, 0x08, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x37, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x37, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x37, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x38, 0x04,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x38, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12,
    0x04, 0x3b, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x3b, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x04, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3c, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03,
    0x3d, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3d, 0x0d, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x15, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x3e, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3e,
    0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3e, 0x1d, 0x1e,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x41, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x41, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x42, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x42, 0x0d,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x19, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x28, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x43, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x43, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x43, 0x1a, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43,
    0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x48, 0x00, 0x4a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x48, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x03, 0x49, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x49, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x17,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x1e, 0x1f, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x4c, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0b, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12,
    0x03, 0x4d, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4d,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4d, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x13, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x4e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x4e, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x4f, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x54, 0x00,
    0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x54, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x04, 0x00, 0x12, 0x04, 0x55, 0x04, 0x59, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x04, 0x00, 0x01, 0x12, 0x03, 0x55, 0x09, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x56, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x56, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x57, 0x08, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x57, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x58, 0x08, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x58, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x58, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12,
    0x03, 0x5a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5a, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x14, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x5c, 0x0d, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x5c, 0x21, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5c, 0x2d,
    0x2e, 0x0a, 0x91, 0x01, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x60, 0x04, 0x3b, 0x1a,
    0x83, 0x01, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x47, 0x65, 0x74, 0x48,
    0x65, 0x61, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x67, 0x65, 0x74, 0x5f, 0x68,
    0x65, 0x61, 0x64, 0x20, 0x3d, 0x20, 0x34, 0x3b, 0x0a, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x20, 0x47, 0x65, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x74, 0x72, 0x65, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x67, 0x65, 0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x74, 0x72, 0x65, 0x65, 0x20, 0x3d, 0x20, 0x35, 0x3b, 0x0a, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x48, 0x65, 0x61, 0x64, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x70, 0x75, 0x62, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x20,
    0x3d, 0x20, 0x36, 0x3b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x60, 0x0d,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x27, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x60, 0x39, 0x3a, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0d, 0x12, 0x04, 0x63, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x03, 0x63, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x04, 0x00, 0x12, 0x04, 0x64,
    0x04, 0x6a, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x04, 0x00, 0x01, 0x12, 0x03, 0x64, 0x09,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x65, 0x08, 0x14,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x08, 0x0f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x65, 0x12, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x66, 0x08, 0x0f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x66, 0x08, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x66, 0x0d, 0x0e, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x67, 0x08, 0x1c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x67, 0x08, 0x17, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x67, 0x1a, 0x1b, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x68, 0x08, 0x1b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x68, 0x08, 0x16, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x68, 0x19, 0x1a, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x69, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x0d, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x69, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x0d, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x69, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x6b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x6b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x6c, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x03,
    0x6e, 0x04, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x06, 0x12, 0x03, 0x6e, 0x0d, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6e, 0x28, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6e, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x03, 0x12, 0x03, 0x6f, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x6f, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6f,
    0x22, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6f, 0x2e, 0x2f,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x72, 0x00, 0x74, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0e, 0x01, 0x12, 0x03, 0x72, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00,
    0x12, 0x03, 0x73, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x73, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x73, 0x0d,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x73, 0x19, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x73, 0x27, 0x28, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0f, 0x12, 0x04, 0x76, 0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01,
    0x12, 0x03, 0x76, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x04, 0x00, 0x12, 0x04, 0x77,
    0x04, 0x7b, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x04, 0x00, 0x01, 0x12, 0x03, 0x77, 0x09,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x78, 0x08, 0x14,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x08, 0x0f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x78, 0x12, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x79, 0x08, 0x0f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x79, 0x08, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x79, 0x0d, 0x0e, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x08, 0x1c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x17, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x7c, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x7c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x7c, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x7c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x04, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7d, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x7d, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06,
    0x80, 0x01, 0x00, 0x82, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x80,
    0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x81, 0x01, 0x04,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0d, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x19, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x06, 0x84, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x11, 0x01, 0x12, 0x04, 0x84, 0x01, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x04, 0x00,
    0x12, 0x06, 0x85, 0x01, 0x04, 0x8a, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x04, 0x00,
    0x01, 0x12, 0x04, 0x85, 0x01, 0x09, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x86, 0x01, 0x08, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0x86, 0x01, 0x12, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x11, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x88, 0x01, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x11, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x08, 0x17, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x11, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x88, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x11, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x89, 0x01, 0x08, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x11, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x15, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x11, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x89, 0x01, 0x18, 0x19, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x8c, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x8c, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c,
    0x01, 0x22, 0x23,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
