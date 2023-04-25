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
//! Generated file from `events.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct Event {
    // message fields
    pub event_type: ::std::string::String,
    pub attributes: ::protobuf::RepeatedField<Event_Attribute>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Event {
    fn default() -> &'a Event {
        <Event as ::protobuf::Message>::default_instance()
    }
}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    // string event_type = 1;


    pub fn get_event_type(&self) -> &str {
        &self.event_type
    }
    pub fn clear_event_type(&mut self) {
        self.event_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_type(&mut self, v: ::std::string::String) {
        self.event_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_type(&mut self) -> &mut ::std::string::String {
        &mut self.event_type
    }

    // Take field
    pub fn take_event_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.event_type, ::std::string::String::new())
    }

    // repeated .Event.Attribute attributes = 2;


    pub fn get_attributes(&self) -> &[Event_Attribute] {
        &self.attributes
    }
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::protobuf::RepeatedField<Event_Attribute>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes(&mut self) -> &mut ::protobuf::RepeatedField<Event_Attribute> {
        &mut self.attributes
    }

    // Take field
    pub fn take_attributes(&mut self) -> ::protobuf::RepeatedField<Event_Attribute> {
        ::std::mem::replace(&mut self.attributes, ::protobuf::RepeatedField::new())
    }

    // bytes data = 3;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        for v in &self.attributes {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.event_type)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attributes)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.event_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.event_type);
        }
        for value in &self.attributes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.event_type.is_empty() {
            os.write_string(1, &self.event_type)?;
        }
        for v in &self.attributes {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "event_type",
                |m: &Event| { &m.event_type },
                |m: &mut Event| { &mut m.event_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event_Attribute>>(
                "attributes",
                |m: &Event| { &m.attributes },
                |m: &mut Event| { &mut m.attributes },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "data",
                |m: &Event| { &m.data },
                |m: &mut Event| { &mut m.data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Event>(
                "Event",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Event {
        static instance: ::protobuf::rt::LazyV2<Event> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Event::new)
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.event_type.clear();
        self.attributes.clear();
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event_Attribute {
    // message fields
    pub key: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Event_Attribute {
    fn default() -> &'a Event_Attribute {
        <Event_Attribute as ::protobuf::Message>::default_instance()
    }
}

impl Event_Attribute {
    pub fn new() -> Event_Attribute {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    // string value = 2;


    pub fn get_value(&self) -> &str {
        &self.value
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Event_Attribute {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
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

    fn new() -> Event_Attribute {
        Event_Attribute::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key",
                |m: &Event_Attribute| { &m.key },
                |m: &mut Event_Attribute| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "value",
                |m: &Event_Attribute| { &m.value },
                |m: &mut Event_Attribute| { &mut m.value },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Event_Attribute>(
                "Event.Attribute",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Event_Attribute {
        static instance: ::protobuf::rt::LazyV2<Event_Attribute> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Event_Attribute::new)
    }
}

impl ::protobuf::Clear for Event_Attribute {
    fn clear(&mut self) {
        self.key.clear();
        self.value.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event_Attribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_Attribute {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventList {
    // message fields
    pub events: ::protobuf::RepeatedField<Event>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventList {
    fn default() -> &'a EventList {
        <EventList as ::protobuf::Message>::default_instance()
    }
}

impl EventList {
    pub fn new() -> EventList {
        ::std::default::Default::default()
    }

    // repeated .Event events = 1;


    pub fn get_events(&self) -> &[Event] {
        &self.events
    }
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<Event>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<Event> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<Event> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for EventList {
    fn is_initialized(&self) -> bool {
        for v in &self.events {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.events {
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

    fn new() -> EventList {
        EventList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event>>(
                "events",
                |m: &EventList| { &m.events },
                |m: &mut EventList| { &mut m.events },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventList>(
                "EventList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventList {
        static instance: ::protobuf::rt::LazyV2<EventList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventList::new)
    }
}

impl ::protobuf::Clear for EventList {
    fn clear(&mut self) {
        self.events.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventFilter {
    // message fields
    pub key: ::std::string::String,
    pub match_string: ::std::string::String,
    pub filter_type: EventFilter_FilterType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventFilter {
    fn default() -> &'a EventFilter {
        <EventFilter as ::protobuf::Message>::default_instance()
    }
}

impl EventFilter {
    pub fn new() -> EventFilter {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    // string match_string = 2;


    pub fn get_match_string(&self) -> &str {
        &self.match_string
    }
    pub fn clear_match_string(&mut self) {
        self.match_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_match_string(&mut self, v: ::std::string::String) {
        self.match_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_match_string(&mut self) -> &mut ::std::string::String {
        &mut self.match_string
    }

    // Take field
    pub fn take_match_string(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.match_string, ::std::string::String::new())
    }

    // .EventFilter.FilterType filter_type = 3;


    pub fn get_filter_type(&self) -> EventFilter_FilterType {
        self.filter_type
    }
    pub fn clear_filter_type(&mut self) {
        self.filter_type = EventFilter_FilterType::FILTER_TYPE_UNSET;
    }

    // Param is passed by value, moved
    pub fn set_filter_type(&mut self, v: EventFilter_FilterType) {
        self.filter_type = v;
    }
}

impl ::protobuf::Message for EventFilter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.match_string)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.filter_type, 3, &mut self.unknown_fields)?
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if !self.match_string.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.match_string);
        }
        if self.filter_type != EventFilter_FilterType::FILTER_TYPE_UNSET {
            my_size += ::protobuf::rt::enum_size(3, self.filter_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if !self.match_string.is_empty() {
            os.write_string(2, &self.match_string)?;
        }
        if self.filter_type != EventFilter_FilterType::FILTER_TYPE_UNSET {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.filter_type))?;
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

    fn new() -> EventFilter {
        EventFilter::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key",
                |m: &EventFilter| { &m.key },
                |m: &mut EventFilter| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "match_string",
                |m: &EventFilter| { &m.match_string },
                |m: &mut EventFilter| { &mut m.match_string },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EventFilter_FilterType>>(
                "filter_type",
                |m: &EventFilter| { &m.filter_type },
                |m: &mut EventFilter| { &mut m.filter_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventFilter>(
                "EventFilter",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventFilter {
        static instance: ::protobuf::rt::LazyV2<EventFilter> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventFilter::new)
    }
}

impl ::protobuf::Clear for EventFilter {
    fn clear(&mut self) {
        self.key.clear();
        self.match_string.clear();
        self.filter_type = EventFilter_FilterType::FILTER_TYPE_UNSET;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventFilter {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EventFilter_FilterType {
    FILTER_TYPE_UNSET = 0,
    SIMPLE_ANY = 1,
    SIMPLE_ALL = 2,
    REGEX_ANY = 3,
    REGEX_ALL = 4,
}

impl ::protobuf::ProtobufEnum for EventFilter_FilterType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EventFilter_FilterType> {
        match value {
            0 => ::std::option::Option::Some(EventFilter_FilterType::FILTER_TYPE_UNSET),
            1 => ::std::option::Option::Some(EventFilter_FilterType::SIMPLE_ANY),
            2 => ::std::option::Option::Some(EventFilter_FilterType::SIMPLE_ALL),
            3 => ::std::option::Option::Some(EventFilter_FilterType::REGEX_ANY),
            4 => ::std::option::Option::Some(EventFilter_FilterType::REGEX_ALL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EventFilter_FilterType] = &[
            EventFilter_FilterType::FILTER_TYPE_UNSET,
            EventFilter_FilterType::SIMPLE_ANY,
            EventFilter_FilterType::SIMPLE_ALL,
            EventFilter_FilterType::REGEX_ANY,
            EventFilter_FilterType::REGEX_ALL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<EventFilter_FilterType>("EventFilter.FilterType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for EventFilter_FilterType {
}

impl ::std::default::Default for EventFilter_FilterType {
    fn default() -> Self {
        EventFilter_FilterType::FILTER_TYPE_UNSET
    }
}

impl ::protobuf::reflect::ProtobufValue for EventFilter_FilterType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventSubscription {
    // message fields
    pub event_type: ::std::string::String,
    pub filters: ::protobuf::RepeatedField<EventFilter>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventSubscription {
    fn default() -> &'a EventSubscription {
        <EventSubscription as ::protobuf::Message>::default_instance()
    }
}

impl EventSubscription {
    pub fn new() -> EventSubscription {
        ::std::default::Default::default()
    }

    // string event_type = 1;


    pub fn get_event_type(&self) -> &str {
        &self.event_type
    }
    pub fn clear_event_type(&mut self) {
        self.event_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_type(&mut self, v: ::std::string::String) {
        self.event_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_type(&mut self) -> &mut ::std::string::String {
        &mut self.event_type
    }

    // Take field
    pub fn take_event_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.event_type, ::std::string::String::new())
    }

    // repeated .EventFilter filters = 2;


    pub fn get_filters(&self) -> &[EventFilter] {
        &self.filters
    }
    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: ::protobuf::RepeatedField<EventFilter>) {
        self.filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filters(&mut self) -> &mut ::protobuf::RepeatedField<EventFilter> {
        &mut self.filters
    }

    // Take field
    pub fn take_filters(&mut self) -> ::protobuf::RepeatedField<EventFilter> {
        ::std::mem::replace(&mut self.filters, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for EventSubscription {
    fn is_initialized(&self) -> bool {
        for v in &self.filters {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.event_type)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.filters)?;
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
        if !self.event_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.event_type);
        }
        for value in &self.filters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.event_type.is_empty() {
            os.write_string(1, &self.event_type)?;
        }
        for v in &self.filters {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> EventSubscription {
        EventSubscription::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "event_type",
                |m: &EventSubscription| { &m.event_type },
                |m: &mut EventSubscription| { &mut m.event_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EventFilter>>(
                "filters",
                |m: &EventSubscription| { &m.filters },
                |m: &mut EventSubscription| { &mut m.filters },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventSubscription>(
                "EventSubscription",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventSubscription {
        static instance: ::protobuf::rt::LazyV2<EventSubscription> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventSubscription::new)
    }
}

impl ::protobuf::Clear for EventSubscription {
    fn clear(&mut self) {
        self.event_type.clear();
        self.filters.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventSubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventSubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cevents.proto\"\xa1\x01\n\x05Event\x12\x1d\n\nevent_type\x18\x01\
    \x20\x01(\tR\teventType\x120\n\nattributes\x18\x02\x20\x03(\x0b2\x10.Eve\
    nt.AttributeR\nattributes\x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04dat\
    a\x1a3\n\tAttribute\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\
    \n\x05value\x18\x02\x20\x01(\tR\x05value\"+\n\tEventList\x12\x1e\n\x06ev\
    ents\x18\x01\x20\x03(\x0b2\x06.EventR\x06events\"\xdf\x01\n\x0bEventFilt\
    er\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12!\n\x0cmatch_string\
    \x18\x02\x20\x01(\tR\x0bmatchString\x128\n\x0bfilter_type\x18\x03\x20\
    \x01(\x0e2\x17.EventFilter.FilterTypeR\nfilterType\"a\n\nFilterType\x12\
    \x15\n\x11FILTER_TYPE_UNSET\x10\0\x12\x0e\n\nSIMPLE_ANY\x10\x01\x12\x0e\
    \n\nSIMPLE_ALL\x10\x02\x12\r\n\tREGEX_ANY\x10\x03\x12\r\n\tREGEX_ALL\x10\
    \x04\"Z\n\x11EventSubscription\x12\x1d\n\nevent_type\x18\x01\x20\x01(\tR\
    \teventType\x12&\n\x07filters\x18\x02\x20\x03(\x0b2\x0c.EventFilterR\x07\
    filtersB%\n\x15sawtooth.sdk.protobufP\x01Z\nevents_pb2b\x06proto3\
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
