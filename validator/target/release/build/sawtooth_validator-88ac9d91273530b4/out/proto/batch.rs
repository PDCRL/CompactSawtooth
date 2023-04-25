// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `batch.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct BatchHeader {
    // message fields
    pub signer_public_key: ::std::string::String,
    pub transaction_ids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BatchHeader {
    fn default() -> &'a BatchHeader {
        <BatchHeader as ::protobuf::Message>::default_instance()
    }
}

impl BatchHeader {
    pub fn new() -> BatchHeader {
        ::std::default::Default::default()
    }

    // string signer_public_key = 1;


    pub fn get_signer_public_key(&self) -> &str {
        &self.signer_public_key
    }
    pub fn clear_signer_public_key(&mut self) {
        self.signer_public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_signer_public_key(&mut self, v: ::std::string::String) {
        self.signer_public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signer_public_key(&mut self) -> &mut ::std::string::String {
        &mut self.signer_public_key
    }

    // Take field
    pub fn take_signer_public_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.signer_public_key, ::std::string::String::new())
    }

    // repeated string transaction_ids = 2;


    pub fn get_transaction_ids(&self) -> &[::std::string::String] {
        &self.transaction_ids
    }
    pub fn clear_transaction_ids(&mut self) {
        self.transaction_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_transaction_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.transaction_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transaction_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.transaction_ids
    }

    // Take field
    pub fn take_transaction_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.transaction_ids, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for BatchHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.signer_public_key)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.transaction_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.signer_public_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.signer_public_key);
        }
        for value in &self.transaction_ids {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.signer_public_key.is_empty() {
            os.write_string(1, &self.signer_public_key)?;
        }
        for v in &self.transaction_ids {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BatchHeader {
        BatchHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "signer_public_key",
                |m: &BatchHeader| { &m.signer_public_key },
                |m: &mut BatchHeader| { &mut m.signer_public_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "transaction_ids",
                |m: &BatchHeader| { &m.transaction_ids },
                |m: &mut BatchHeader| { &mut m.transaction_ids },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BatchHeader>(
                "BatchHeader",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BatchHeader {
        static instance: ::protobuf::rt::LazyV2<BatchHeader> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BatchHeader::new)
    }
}

impl ::protobuf::Clear for BatchHeader {
    fn clear(&mut self) {
        self.signer_public_key.clear();
        self.transaction_ids.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Batch {
    // message fields
    pub header: ::std::vec::Vec<u8>,
    pub header_signature: ::std::string::String,
    pub transactions: ::protobuf::RepeatedField<super::transaction::Transaction>,
    pub trace: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Batch {
    fn default() -> &'a Batch {
        <Batch as ::protobuf::Message>::default_instance()
    }
}

impl Batch {
    pub fn new() -> Batch {
        ::std::default::Default::default()
    }

    // bytes header = 1;


    pub fn get_header(&self) -> &[u8] {
        &self.header
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ::std::vec::Vec<u8>) {
        self.header = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.header
    }

    // Take field
    pub fn take_header(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.header, ::std::vec::Vec::new())
    }

    // string header_signature = 2;


    pub fn get_header_signature(&self) -> &str {
        &self.header_signature
    }
    pub fn clear_header_signature(&mut self) {
        self.header_signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_header_signature(&mut self, v: ::std::string::String) {
        self.header_signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header_signature(&mut self) -> &mut ::std::string::String {
        &mut self.header_signature
    }

    // Take field
    pub fn take_header_signature(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.header_signature, ::std::string::String::new())
    }

    // repeated .Transaction transactions = 3;


    pub fn get_transactions(&self) -> &[super::transaction::Transaction] {
        &self.transactions
    }
    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<super::transaction::Transaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<super::transaction::Transaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<super::transaction::Transaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    // bool trace = 4;


    pub fn get_trace(&self) -> bool {
        self.trace
    }
    pub fn clear_trace(&mut self) {
        self.trace = false;
    }

    // Param is passed by value, moved
    pub fn set_trace(&mut self, v: bool) {
        self.trace = v;
    }
}

impl ::protobuf::Message for Batch {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.header_signature)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.trace = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.header.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.header);
        }
        if !self.header_signature.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.header_signature);
        }
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.trace != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.header.is_empty() {
            os.write_bytes(1, &self.header)?;
        }
        if !self.header_signature.is_empty() {
            os.write_string(2, &self.header_signature)?;
        }
        for v in &self.transactions {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.trace != false {
            os.write_bool(4, self.trace)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Batch {
        Batch::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "header",
                |m: &Batch| { &m.header },
                |m: &mut Batch| { &mut m.header },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "header_signature",
                |m: &Batch| { &m.header_signature },
                |m: &mut Batch| { &mut m.header_signature },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::transaction::Transaction>>(
                "transactions",
                |m: &Batch| { &m.transactions },
                |m: &mut Batch| { &mut m.transactions },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "trace",
                |m: &Batch| { &m.trace },
                |m: &mut Batch| { &mut m.trace },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Batch>(
                "Batch",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Batch {
        static instance: ::protobuf::rt::LazyV2<Batch> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Batch::new)
    }
}

impl ::protobuf::Clear for Batch {
    fn clear(&mut self) {
        self.header.clear();
        self.header_signature.clear();
        self.transactions.clear();
        self.trace = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Batch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Batch {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchList {
    // message fields
    pub batches: ::protobuf::RepeatedField<Batch>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BatchList {
    fn default() -> &'a BatchList {
        <BatchList as ::protobuf::Message>::default_instance()
    }
}

impl BatchList {
    pub fn new() -> BatchList {
        ::std::default::Default::default()
    }

    // repeated .Batch batches = 1;


    pub fn get_batches(&self) -> &[Batch] {
        &self.batches
    }
    pub fn clear_batches(&mut self) {
        self.batches.clear();
    }

    // Param is passed by value, moved
    pub fn set_batches(&mut self, v: ::protobuf::RepeatedField<Batch>) {
        self.batches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_batches(&mut self) -> &mut ::protobuf::RepeatedField<Batch> {
        &mut self.batches
    }

    // Take field
    pub fn take_batches(&mut self) -> ::protobuf::RepeatedField<Batch> {
        ::std::mem::replace(&mut self.batches, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for BatchList {
    fn is_initialized(&self) -> bool {
        for v in &self.batches {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.batches)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.batches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.batches {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BatchList {
        BatchList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Batch>>(
                "batches",
                |m: &BatchList| { &m.batches },
                |m: &mut BatchList| { &mut m.batches },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BatchList>(
                "BatchList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BatchList {
        static instance: ::protobuf::rt::LazyV2<BatchList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BatchList::new)
    }
}

impl ::protobuf::Clear for BatchList {
    fn clear(&mut self) {
        self.batches.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bbatch.proto\x1a\x11transaction.proto\"b\n\x0bBatchHeader\x12*\n\
    \x11signer_public_key\x18\x01\x20\x01(\tR\x0fsignerPublicKey\x12'\n\x0ft\
    ransaction_ids\x18\x02\x20\x03(\tR\x0etransactionIds\"\x92\x01\n\x05Batc\
    h\x12\x16\n\x06header\x18\x01\x20\x01(\x0cR\x06header\x12)\n\x10header_s\
    ignature\x18\x02\x20\x01(\tR\x0fheaderSignature\x120\n\x0ctransactions\
    \x18\x03\x20\x03(\x0b2\x0c.TransactionR\x0ctransactions\x12\x14\n\x05tra\
    ce\x18\x04\x20\x01(\x08R\x05trace\"-\n\tBatchList\x12\x20\n\x07batches\
    \x18\x01\x20\x03(\x0b2\x06.BatchR\x07batchesB$\n\x15sawtooth.sdk.protobu\
    fP\x01Z\tbatch_pb2b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}