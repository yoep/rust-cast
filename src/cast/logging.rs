// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default,Debug)]
pub struct SocketEvent {
    // message fields
    field_type: ::std::option::Option<EventType>,
    timestamp_micros: ::std::option::Option<i64>,
    details: ::protobuf::SingularField<::std::string::String>,
    net_return_value: ::std::option::Option<i32>,
    message_namespace: ::protobuf::SingularField<::std::string::String>,
    ready_state: ::std::option::Option<ReadyState>,
    connection_state: ::std::option::Option<ConnectionState>,
    read_state: ::std::option::Option<ReadState>,
    write_state: ::std::option::Option<WriteState>,
    error_state: ::std::option::Option<ErrorState>,
    challenge_reply_error_type: ::std::option::Option<ChallengeReplyErrorType>,
    nss_error_code: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SocketEvent {}

impl SocketEvent {
    pub fn new() -> SocketEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SocketEvent {
        static mut instance: ::protobuf::lazy::Lazy<SocketEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SocketEvent,
        };
        unsafe {
            instance.get(|| {
                SocketEvent {
                    field_type: ::std::option::Option::None,
                    timestamp_micros: ::std::option::Option::None,
                    details: ::protobuf::SingularField::none(),
                    net_return_value: ::std::option::Option::None,
                    message_namespace: ::protobuf::SingularField::none(),
                    ready_state: ::std::option::Option::None,
                    connection_state: ::std::option::Option::None,
                    read_state: ::std::option::Option::None,
                    write_state: ::std::option::Option::None,
                    error_state: ::std::option::Option::None,
                    challenge_reply_error_type: ::std::option::Option::None,
                    nss_error_code: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .extensions.api.cast_channel.proto.EventType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EventType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> EventType {
        self.field_type.unwrap_or(EventType::EVENT_TYPE_UNKNOWN)
    }

    // optional int64 timestamp_micros = 2;

    pub fn clear_timestamp_micros(&mut self) {
        self.timestamp_micros = ::std::option::Option::None;
    }

    pub fn has_timestamp_micros(&self) -> bool {
        self.timestamp_micros.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_micros(&mut self, v: i64) {
        self.timestamp_micros = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_micros<'a>(&self) -> i64 {
        self.timestamp_micros.unwrap_or(0)
    }

    // optional string details = 3;

    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    pub fn has_details(&self) -> bool {
        self.details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: ::std::string::String) {
        self.details = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_details<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.details.is_none() {
            self.details.set_default();
        };
        self.details.as_mut().unwrap()
    }

    // Take field
    pub fn take_details(&mut self) -> ::std::string::String {
        self.details.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_details<'a>(&'a self) -> &'a str {
        match self.details.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 net_return_value = 4;

    pub fn clear_net_return_value(&mut self) {
        self.net_return_value = ::std::option::Option::None;
    }

    pub fn has_net_return_value(&self) -> bool {
        self.net_return_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_return_value(&mut self, v: i32) {
        self.net_return_value = ::std::option::Option::Some(v);
    }

    pub fn get_net_return_value<'a>(&self) -> i32 {
        self.net_return_value.unwrap_or(0)
    }

    // optional string message_namespace = 5;

    pub fn clear_message_namespace(&mut self) {
        self.message_namespace.clear();
    }

    pub fn has_message_namespace(&self) -> bool {
        self.message_namespace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_namespace(&mut self, v: ::std::string::String) {
        self.message_namespace = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_namespace<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message_namespace.is_none() {
            self.message_namespace.set_default();
        };
        self.message_namespace.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_namespace(&mut self) -> ::std::string::String {
        self.message_namespace.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message_namespace<'a>(&'a self) -> &'a str {
        match self.message_namespace.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .extensions.api.cast_channel.proto.ReadyState ready_state = 6;

    pub fn clear_ready_state(&mut self) {
        self.ready_state = ::std::option::Option::None;
    }

    pub fn has_ready_state(&self) -> bool {
        self.ready_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ready_state(&mut self, v: ReadyState) {
        self.ready_state = ::std::option::Option::Some(v);
    }

    pub fn get_ready_state<'a>(&self) -> ReadyState {
        self.ready_state.unwrap_or(ReadyState::READY_STATE_NONE)
    }

    // optional .extensions.api.cast_channel.proto.ConnectionState connection_state = 7;

    pub fn clear_connection_state(&mut self) {
        self.connection_state = ::std::option::Option::None;
    }

    pub fn has_connection_state(&self) -> bool {
        self.connection_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_state(&mut self, v: ConnectionState) {
        self.connection_state = ::std::option::Option::Some(v);
    }

    pub fn get_connection_state<'a>(&self) -> ConnectionState {
        self.connection_state.unwrap_or(ConnectionState::CONN_STATE_UNKNOWN)
    }

    // optional .extensions.api.cast_channel.proto.ReadState read_state = 8;

    pub fn clear_read_state(&mut self) {
        self.read_state = ::std::option::Option::None;
    }

    pub fn has_read_state(&self) -> bool {
        self.read_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_read_state(&mut self, v: ReadState) {
        self.read_state = ::std::option::Option::Some(v);
    }

    pub fn get_read_state<'a>(&self) -> ReadState {
        self.read_state.unwrap_or(ReadState::READ_STATE_UNKNOWN)
    }

    // optional .extensions.api.cast_channel.proto.WriteState write_state = 9;

    pub fn clear_write_state(&mut self) {
        self.write_state = ::std::option::Option::None;
    }

    pub fn has_write_state(&self) -> bool {
        self.write_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_write_state(&mut self, v: WriteState) {
        self.write_state = ::std::option::Option::Some(v);
    }

    pub fn get_write_state<'a>(&self) -> WriteState {
        self.write_state.unwrap_or(WriteState::WRITE_STATE_UNKNOWN)
    }

    // optional .extensions.api.cast_channel.proto.ErrorState error_state = 10;

    pub fn clear_error_state(&mut self) {
        self.error_state = ::std::option::Option::None;
    }

    pub fn has_error_state(&self) -> bool {
        self.error_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_state(&mut self, v: ErrorState) {
        self.error_state = ::std::option::Option::Some(v);
    }

    pub fn get_error_state<'a>(&self) -> ErrorState {
        self.error_state.unwrap_or(ErrorState::CHANNEL_ERROR_NONE)
    }

    // optional .extensions.api.cast_channel.proto.ChallengeReplyErrorType challenge_reply_error_type = 11;

    pub fn clear_challenge_reply_error_type(&mut self) {
        self.challenge_reply_error_type = ::std::option::Option::None;
    }

    pub fn has_challenge_reply_error_type(&self) -> bool {
        self.challenge_reply_error_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_reply_error_type(&mut self, v: ChallengeReplyErrorType) {
        self.challenge_reply_error_type = ::std::option::Option::Some(v);
    }

    pub fn get_challenge_reply_error_type<'a>(&self) -> ChallengeReplyErrorType {
        self.challenge_reply_error_type.unwrap_or(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NONE)
    }

    // optional int32 nss_error_code = 12;

    pub fn clear_nss_error_code(&mut self) {
        self.nss_error_code = ::std::option::Option::None;
    }

    pub fn has_nss_error_code(&self) -> bool {
        self.nss_error_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nss_error_code(&mut self, v: i32) {
        self.nss_error_code = ::std::option::Option::Some(v);
    }

    pub fn get_nss_error_code<'a>(&self) -> i32 {
        self.nss_error_code.unwrap_or(0)
    }
}

impl ::protobuf::Message for SocketEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp_micros = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.details));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.net_return_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message_namespace));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.ready_state = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.connection_state = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.read_state = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.write_state = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.error_state = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.challenge_reply_error_type = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.nss_error_code = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.timestamp_micros.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.details.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.net_return_value.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.message_namespace.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.ready_state.iter() {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in self.connection_state.iter() {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        for value in self.read_state.iter() {
            my_size += ::protobuf::rt::enum_size(8, *value);
        };
        for value in self.write_state.iter() {
            my_size += ::protobuf::rt::enum_size(9, *value);
        };
        for value in self.error_state.iter() {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        for value in self.challenge_reply_error_type.iter() {
            my_size += ::protobuf::rt::enum_size(11, *value);
        };
        for value in self.nss_error_code.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.timestamp_micros {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.details.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.net_return_value {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.message_namespace.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.ready_state {
            try!(os.write_enum(6, v.value()));
        };
        if let Some(v) = self.connection_state {
            try!(os.write_enum(7, v.value()));
        };
        if let Some(v) = self.read_state {
            try!(os.write_enum(8, v.value()));
        };
        if let Some(v) = self.write_state {
            try!(os.write_enum(9, v.value()));
        };
        if let Some(v) = self.error_state {
            try!(os.write_enum(10, v.value()));
        };
        if let Some(v) = self.challenge_reply_error_type {
            try!(os.write_enum(11, v.value()));
        };
        if let Some(v) = self.nss_error_code {
            try!(os.write_int32(12, v));
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
        ::std::any::TypeId::of::<SocketEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SocketEvent {
    fn new() -> SocketEvent {
        SocketEvent::new()
    }
}

impl ::protobuf::Clear for SocketEvent {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_timestamp_micros();
        self.clear_details();
        self.clear_net_return_value();
        self.clear_message_namespace();
        self.clear_ready_state();
        self.clear_connection_state();
        self.clear_read_state();
        self.clear_write_state();
        self.clear_error_state();
        self.clear_challenge_reply_error_type();
        self.clear_nss_error_code();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SocketEvent {
    fn eq(&self, other: &SocketEvent) -> bool {
        self.field_type == other.field_type &&
        self.timestamp_micros == other.timestamp_micros &&
        self.details == other.details &&
        self.net_return_value == other.net_return_value &&
        self.message_namespace == other.message_namespace &&
        self.ready_state == other.ready_state &&
        self.connection_state == other.connection_state &&
        self.read_state == other.read_state &&
        self.write_state == other.write_state &&
        self.error_state == other.error_state &&
        self.challenge_reply_error_type == other.challenge_reply_error_type &&
        self.nss_error_code == other.nss_error_code &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct AggregatedSocketEvent {
    // message fields
    id: ::std::option::Option<i32>,
    endpoint_id: ::std::option::Option<i32>,
    channel_auth_type: ::std::option::Option<ChannelAuth>,
    socket_event: ::protobuf::RepeatedField<SocketEvent>,
    bytes_read: ::std::option::Option<i64>,
    bytes_written: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AggregatedSocketEvent {}

impl AggregatedSocketEvent {
    pub fn new() -> AggregatedSocketEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AggregatedSocketEvent {
        static mut instance: ::protobuf::lazy::Lazy<AggregatedSocketEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AggregatedSocketEvent,
        };
        unsafe {
            instance.get(|| {
                AggregatedSocketEvent {
                    id: ::std::option::Option::None,
                    endpoint_id: ::std::option::Option::None,
                    channel_auth_type: ::std::option::Option::None,
                    socket_event: ::protobuf::RepeatedField::new(),
                    bytes_read: ::std::option::Option::None,
                    bytes_written: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    // optional int32 endpoint_id = 2;

    pub fn clear_endpoint_id(&mut self) {
        self.endpoint_id = ::std::option::Option::None;
    }

    pub fn has_endpoint_id(&self) -> bool {
        self.endpoint_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endpoint_id(&mut self, v: i32) {
        self.endpoint_id = ::std::option::Option::Some(v);
    }

    pub fn get_endpoint_id<'a>(&self) -> i32 {
        self.endpoint_id.unwrap_or(0)
    }

    // optional .extensions.api.cast_channel.proto.ChannelAuth channel_auth_type = 3;

    pub fn clear_channel_auth_type(&mut self) {
        self.channel_auth_type = ::std::option::Option::None;
    }

    pub fn has_channel_auth_type(&self) -> bool {
        self.channel_auth_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_auth_type(&mut self, v: ChannelAuth) {
        self.channel_auth_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_auth_type<'a>(&self) -> ChannelAuth {
        self.channel_auth_type.unwrap_or(ChannelAuth::SSL)
    }

    // repeated .extensions.api.cast_channel.proto.SocketEvent socket_event = 4;

    pub fn clear_socket_event(&mut self) {
        self.socket_event.clear();
    }

    // Param is passed by value, moved
    pub fn set_socket_event(&mut self, v: ::protobuf::RepeatedField<SocketEvent>) {
        self.socket_event = v;
    }

    // Mutable pointer to the field.
    pub fn mut_socket_event<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SocketEvent> {
        &mut self.socket_event
    }

    // Take field
    pub fn take_socket_event(&mut self) -> ::protobuf::RepeatedField<SocketEvent> {
        ::std::mem::replace(&mut self.socket_event, ::protobuf::RepeatedField::new())
    }

    pub fn get_socket_event<'a>(&'a self) -> &'a [SocketEvent] {
        &self.socket_event
    }

    // optional int64 bytes_read = 5;

    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = ::std::option::Option::None;
    }

    pub fn has_bytes_read(&self) -> bool {
        self.bytes_read.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_read(&mut self, v: i64) {
        self.bytes_read = ::std::option::Option::Some(v);
    }

    pub fn get_bytes_read<'a>(&self) -> i64 {
        self.bytes_read.unwrap_or(0)
    }

    // optional int64 bytes_written = 6;

    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = ::std::option::Option::None;
    }

    pub fn has_bytes_written(&self) -> bool {
        self.bytes_written.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_written(&mut self, v: i64) {
        self.bytes_written = ::std::option::Option::Some(v);
    }

    pub fn get_bytes_written<'a>(&self) -> i64 {
        self.bytes_written.unwrap_or(0)
    }
}

impl ::protobuf::Message for AggregatedSocketEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.endpoint_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.channel_auth_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.socket_event));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.bytes_read = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.bytes_written = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.endpoint_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.channel_auth_type.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.socket_event.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.bytes_read.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.bytes_written.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.endpoint_id {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.channel_auth_type {
            try!(os.write_enum(3, v.value()));
        };
        for v in self.socket_event.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.bytes_read {
            try!(os.write_int64(5, v));
        };
        if let Some(v) = self.bytes_written {
            try!(os.write_int64(6, v));
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
        ::std::any::TypeId::of::<AggregatedSocketEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AggregatedSocketEvent {
    fn new() -> AggregatedSocketEvent {
        AggregatedSocketEvent::new()
    }
}

impl ::protobuf::Clear for AggregatedSocketEvent {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_endpoint_id();
        self.clear_channel_auth_type();
        self.clear_socket_event();
        self.clear_bytes_read();
        self.clear_bytes_written();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AggregatedSocketEvent {
    fn eq(&self, other: &AggregatedSocketEvent) -> bool {
        self.id == other.id &&
        self.endpoint_id == other.endpoint_id &&
        self.channel_auth_type == other.channel_auth_type &&
        self.socket_event == other.socket_event &&
        self.bytes_read == other.bytes_read &&
        self.bytes_written == other.bytes_written &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct Log {
    // message fields
    aggregated_socket_event: ::protobuf::RepeatedField<AggregatedSocketEvent>,
    num_evicted_aggregated_socket_events: ::std::option::Option<i32>,
    num_evicted_socket_events: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Log {}

impl Log {
    pub fn new() -> Log {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Log {
        static mut instance: ::protobuf::lazy::Lazy<Log> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Log,
        };
        unsafe {
            instance.get(|| {
                Log {
                    aggregated_socket_event: ::protobuf::RepeatedField::new(),
                    num_evicted_aggregated_socket_events: ::std::option::Option::None,
                    num_evicted_socket_events: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .extensions.api.cast_channel.proto.AggregatedSocketEvent aggregated_socket_event = 1;

    pub fn clear_aggregated_socket_event(&mut self) {
        self.aggregated_socket_event.clear();
    }

    // Param is passed by value, moved
    pub fn set_aggregated_socket_event(&mut self, v: ::protobuf::RepeatedField<AggregatedSocketEvent>) {
        self.aggregated_socket_event = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aggregated_socket_event<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AggregatedSocketEvent> {
        &mut self.aggregated_socket_event
    }

    // Take field
    pub fn take_aggregated_socket_event(&mut self) -> ::protobuf::RepeatedField<AggregatedSocketEvent> {
        ::std::mem::replace(&mut self.aggregated_socket_event, ::protobuf::RepeatedField::new())
    }

    pub fn get_aggregated_socket_event<'a>(&'a self) -> &'a [AggregatedSocketEvent] {
        &self.aggregated_socket_event
    }

    // optional int32 num_evicted_aggregated_socket_events = 2;

    pub fn clear_num_evicted_aggregated_socket_events(&mut self) {
        self.num_evicted_aggregated_socket_events = ::std::option::Option::None;
    }

    pub fn has_num_evicted_aggregated_socket_events(&self) -> bool {
        self.num_evicted_aggregated_socket_events.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_evicted_aggregated_socket_events(&mut self, v: i32) {
        self.num_evicted_aggregated_socket_events = ::std::option::Option::Some(v);
    }

    pub fn get_num_evicted_aggregated_socket_events<'a>(&self) -> i32 {
        self.num_evicted_aggregated_socket_events.unwrap_or(0)
    }

    // optional int32 num_evicted_socket_events = 3;

    pub fn clear_num_evicted_socket_events(&mut self) {
        self.num_evicted_socket_events = ::std::option::Option::None;
    }

    pub fn has_num_evicted_socket_events(&self) -> bool {
        self.num_evicted_socket_events.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_evicted_socket_events(&mut self, v: i32) {
        self.num_evicted_socket_events = ::std::option::Option::Some(v);
    }

    pub fn get_num_evicted_socket_events<'a>(&self) -> i32 {
        self.num_evicted_socket_events.unwrap_or(0)
    }
}

impl ::protobuf::Message for Log {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aggregated_socket_event));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_evicted_aggregated_socket_events = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_evicted_socket_events = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.aggregated_socket_event.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.num_evicted_aggregated_socket_events.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.num_evicted_socket_events.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.aggregated_socket_event.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.num_evicted_aggregated_socket_events {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.num_evicted_socket_events {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<Log>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Log {
    fn new() -> Log {
        Log::new()
    }
}

impl ::protobuf::Clear for Log {
    fn clear(&mut self) {
        self.clear_aggregated_socket_event();
        self.clear_num_evicted_aggregated_socket_events();
        self.clear_num_evicted_socket_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Log {
    fn eq(&self, other: &Log) -> bool {
        self.aggregated_socket_event == other.aggregated_socket_event &&
        self.num_evicted_aggregated_socket_events == other.num_evicted_aggregated_socket_events &&
        self.num_evicted_socket_events == other.num_evicted_socket_events &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EventType {
    EVENT_TYPE_UNKNOWN = 0,
    CAST_SOCKET_CREATED = 1,
    READY_STATE_CHANGED = 2,
    CONNECTION_STATE_CHANGED = 3,
    READ_STATE_CHANGED = 4,
    WRITE_STATE_CHANGED = 5,
    ERROR_STATE_CHANGED = 6,
    CONNECT_FAILED = 7,
    TCP_SOCKET_CONNECT = 8,
    TCP_SOCKET_SET_KEEP_ALIVE = 9,
    SSL_CERT_WHITELISTED = 10,
    SSL_SOCKET_CONNECT = 11,
    SSL_INFO_OBTAINED = 12,
    DER_ENCODED_CERT_OBTAIN = 13,
    RECEIVED_CHALLENGE_REPLY = 14,
    AUTH_CHALLENGE_REPLY = 15,
    CONNECT_TIMED_OUT = 16,
    SEND_MESSAGE_FAILED = 17,
    MESSAGE_ENQUEUED = 18,
    SOCKET_WRITE = 19,
    MESSAGE_WRITTEN = 20,
    SOCKET_READ = 21,
    MESSAGE_READ = 22,
    SOCKET_CLOSED = 25,
    SSL_CERT_EXCESSIVE_LIFETIME = 26,
    CHANNEL_POLICY_ENFORCED = 27,
    TCP_SOCKET_CONNECT_COMPLETE = 28,
    SSL_SOCKET_CONNECT_COMPLETE = 29,
    SSL_SOCKET_CONNECT_FAILED = 30,
    SEND_AUTH_CHALLENGE_FAILED = 31,
    AUTH_CHALLENGE_REPLY_INVALID = 32,
    PING_WRITE_ERROR = 33,
}

impl ::protobuf::ProtobufEnum for EventType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EventType> {
        match value {
            0 => ::std::option::Option::Some(EventType::EVENT_TYPE_UNKNOWN),
            1 => ::std::option::Option::Some(EventType::CAST_SOCKET_CREATED),
            2 => ::std::option::Option::Some(EventType::READY_STATE_CHANGED),
            3 => ::std::option::Option::Some(EventType::CONNECTION_STATE_CHANGED),
            4 => ::std::option::Option::Some(EventType::READ_STATE_CHANGED),
            5 => ::std::option::Option::Some(EventType::WRITE_STATE_CHANGED),
            6 => ::std::option::Option::Some(EventType::ERROR_STATE_CHANGED),
            7 => ::std::option::Option::Some(EventType::CONNECT_FAILED),
            8 => ::std::option::Option::Some(EventType::TCP_SOCKET_CONNECT),
            9 => ::std::option::Option::Some(EventType::TCP_SOCKET_SET_KEEP_ALIVE),
            10 => ::std::option::Option::Some(EventType::SSL_CERT_WHITELISTED),
            11 => ::std::option::Option::Some(EventType::SSL_SOCKET_CONNECT),
            12 => ::std::option::Option::Some(EventType::SSL_INFO_OBTAINED),
            13 => ::std::option::Option::Some(EventType::DER_ENCODED_CERT_OBTAIN),
            14 => ::std::option::Option::Some(EventType::RECEIVED_CHALLENGE_REPLY),
            15 => ::std::option::Option::Some(EventType::AUTH_CHALLENGE_REPLY),
            16 => ::std::option::Option::Some(EventType::CONNECT_TIMED_OUT),
            17 => ::std::option::Option::Some(EventType::SEND_MESSAGE_FAILED),
            18 => ::std::option::Option::Some(EventType::MESSAGE_ENQUEUED),
            19 => ::std::option::Option::Some(EventType::SOCKET_WRITE),
            20 => ::std::option::Option::Some(EventType::MESSAGE_WRITTEN),
            21 => ::std::option::Option::Some(EventType::SOCKET_READ),
            22 => ::std::option::Option::Some(EventType::MESSAGE_READ),
            25 => ::std::option::Option::Some(EventType::SOCKET_CLOSED),
            26 => ::std::option::Option::Some(EventType::SSL_CERT_EXCESSIVE_LIFETIME),
            27 => ::std::option::Option::Some(EventType::CHANNEL_POLICY_ENFORCED),
            28 => ::std::option::Option::Some(EventType::TCP_SOCKET_CONNECT_COMPLETE),
            29 => ::std::option::Option::Some(EventType::SSL_SOCKET_CONNECT_COMPLETE),
            30 => ::std::option::Option::Some(EventType::SSL_SOCKET_CONNECT_FAILED),
            31 => ::std::option::Option::Some(EventType::SEND_AUTH_CHALLENGE_FAILED),
            32 => ::std::option::Option::Some(EventType::AUTH_CHALLENGE_REPLY_INVALID),
            33 => ::std::option::Option::Some(EventType::PING_WRITE_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EventType] = &[
            EventType::EVENT_TYPE_UNKNOWN,
            EventType::CAST_SOCKET_CREATED,
            EventType::READY_STATE_CHANGED,
            EventType::CONNECTION_STATE_CHANGED,
            EventType::READ_STATE_CHANGED,
            EventType::WRITE_STATE_CHANGED,
            EventType::ERROR_STATE_CHANGED,
            EventType::CONNECT_FAILED,
            EventType::TCP_SOCKET_CONNECT,
            EventType::TCP_SOCKET_SET_KEEP_ALIVE,
            EventType::SSL_CERT_WHITELISTED,
            EventType::SSL_SOCKET_CONNECT,
            EventType::SSL_INFO_OBTAINED,
            EventType::DER_ENCODED_CERT_OBTAIN,
            EventType::RECEIVED_CHALLENGE_REPLY,
            EventType::AUTH_CHALLENGE_REPLY,
            EventType::CONNECT_TIMED_OUT,
            EventType::SEND_MESSAGE_FAILED,
            EventType::MESSAGE_ENQUEUED,
            EventType::SOCKET_WRITE,
            EventType::MESSAGE_WRITTEN,
            EventType::SOCKET_READ,
            EventType::MESSAGE_READ,
            EventType::SOCKET_CLOSED,
            EventType::SSL_CERT_EXCESSIVE_LIFETIME,
            EventType::CHANNEL_POLICY_ENFORCED,
            EventType::TCP_SOCKET_CONNECT_COMPLETE,
            EventType::SSL_SOCKET_CONNECT_COMPLETE,
            EventType::SSL_SOCKET_CONNECT_FAILED,
            EventType::SEND_AUTH_CHALLENGE_FAILED,
            EventType::AUTH_CHALLENGE_REPLY_INVALID,
            EventType::PING_WRITE_ERROR,
        ];
        values
    }
}

impl ::std::marker::Copy for EventType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChannelAuth {
    SSL = 1,
    SSL_VERIFIED = 2,
}

impl ::protobuf::ProtobufEnum for ChannelAuth {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChannelAuth> {
        match value {
            1 => ::std::option::Option::Some(ChannelAuth::SSL),
            2 => ::std::option::Option::Some(ChannelAuth::SSL_VERIFIED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChannelAuth] = &[
            ChannelAuth::SSL,
            ChannelAuth::SSL_VERIFIED,
        ];
        values
    }
}

impl ::std::marker::Copy for ChannelAuth {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReadyState {
    READY_STATE_NONE = 1,
    READY_STATE_CONNECTING = 2,
    READY_STATE_OPEN = 3,
    READY_STATE_CLOSING = 4,
    READY_STATE_CLOSED = 5,
}

impl ::protobuf::ProtobufEnum for ReadyState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadyState> {
        match value {
            1 => ::std::option::Option::Some(ReadyState::READY_STATE_NONE),
            2 => ::std::option::Option::Some(ReadyState::READY_STATE_CONNECTING),
            3 => ::std::option::Option::Some(ReadyState::READY_STATE_OPEN),
            4 => ::std::option::Option::Some(ReadyState::READY_STATE_CLOSING),
            5 => ::std::option::Option::Some(ReadyState::READY_STATE_CLOSED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReadyState] = &[
            ReadyState::READY_STATE_NONE,
            ReadyState::READY_STATE_CONNECTING,
            ReadyState::READY_STATE_OPEN,
            ReadyState::READY_STATE_CLOSING,
            ReadyState::READY_STATE_CLOSED,
        ];
        values
    }
}

impl ::std::marker::Copy for ReadyState {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ConnectionState {
    CONN_STATE_UNKNOWN = 1,
    CONN_STATE_TCP_CONNECT = 2,
    CONN_STATE_TCP_CONNECT_COMPLETE = 3,
    CONN_STATE_SSL_CONNECT = 4,
    CONN_STATE_SSL_CONNECT_COMPLETE = 5,
    CONN_STATE_AUTH_CHALLENGE_SEND = 6,
    CONN_STATE_AUTH_CHALLENGE_SEND_COMPLETE = 7,
    CONN_STATE_AUTH_CHALLENGE_REPLY_COMPLETE = 8,
    CONN_STATE_START_CONNECT = 9,
    CONN_STATE_FINISHED = 100,
    CONN_STATE_ERROR = 101,
    CONN_STATE_TIMEOUT = 102,
}

impl ::protobuf::ProtobufEnum for ConnectionState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ConnectionState> {
        match value {
            1 => ::std::option::Option::Some(ConnectionState::CONN_STATE_UNKNOWN),
            2 => ::std::option::Option::Some(ConnectionState::CONN_STATE_TCP_CONNECT),
            3 => ::std::option::Option::Some(ConnectionState::CONN_STATE_TCP_CONNECT_COMPLETE),
            4 => ::std::option::Option::Some(ConnectionState::CONN_STATE_SSL_CONNECT),
            5 => ::std::option::Option::Some(ConnectionState::CONN_STATE_SSL_CONNECT_COMPLETE),
            6 => ::std::option::Option::Some(ConnectionState::CONN_STATE_AUTH_CHALLENGE_SEND),
            7 => ::std::option::Option::Some(ConnectionState::CONN_STATE_AUTH_CHALLENGE_SEND_COMPLETE),
            8 => ::std::option::Option::Some(ConnectionState::CONN_STATE_AUTH_CHALLENGE_REPLY_COMPLETE),
            9 => ::std::option::Option::Some(ConnectionState::CONN_STATE_START_CONNECT),
            100 => ::std::option::Option::Some(ConnectionState::CONN_STATE_FINISHED),
            101 => ::std::option::Option::Some(ConnectionState::CONN_STATE_ERROR),
            102 => ::std::option::Option::Some(ConnectionState::CONN_STATE_TIMEOUT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ConnectionState] = &[
            ConnectionState::CONN_STATE_UNKNOWN,
            ConnectionState::CONN_STATE_TCP_CONNECT,
            ConnectionState::CONN_STATE_TCP_CONNECT_COMPLETE,
            ConnectionState::CONN_STATE_SSL_CONNECT,
            ConnectionState::CONN_STATE_SSL_CONNECT_COMPLETE,
            ConnectionState::CONN_STATE_AUTH_CHALLENGE_SEND,
            ConnectionState::CONN_STATE_AUTH_CHALLENGE_SEND_COMPLETE,
            ConnectionState::CONN_STATE_AUTH_CHALLENGE_REPLY_COMPLETE,
            ConnectionState::CONN_STATE_START_CONNECT,
            ConnectionState::CONN_STATE_FINISHED,
            ConnectionState::CONN_STATE_ERROR,
            ConnectionState::CONN_STATE_TIMEOUT,
        ];
        values
    }
}

impl ::std::marker::Copy for ConnectionState {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReadState {
    READ_STATE_UNKNOWN = 1,
    READ_STATE_READ = 2,
    READ_STATE_READ_COMPLETE = 3,
    READ_STATE_DO_CALLBACK = 4,
    READ_STATE_HANDLE_ERROR = 5,
    READ_STATE_ERROR = 100,
}

impl ::protobuf::ProtobufEnum for ReadState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadState> {
        match value {
            1 => ::std::option::Option::Some(ReadState::READ_STATE_UNKNOWN),
            2 => ::std::option::Option::Some(ReadState::READ_STATE_READ),
            3 => ::std::option::Option::Some(ReadState::READ_STATE_READ_COMPLETE),
            4 => ::std::option::Option::Some(ReadState::READ_STATE_DO_CALLBACK),
            5 => ::std::option::Option::Some(ReadState::READ_STATE_HANDLE_ERROR),
            100 => ::std::option::Option::Some(ReadState::READ_STATE_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReadState] = &[
            ReadState::READ_STATE_UNKNOWN,
            ReadState::READ_STATE_READ,
            ReadState::READ_STATE_READ_COMPLETE,
            ReadState::READ_STATE_DO_CALLBACK,
            ReadState::READ_STATE_HANDLE_ERROR,
            ReadState::READ_STATE_ERROR,
        ];
        values
    }
}

impl ::std::marker::Copy for ReadState {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum WriteState {
    WRITE_STATE_UNKNOWN = 1,
    WRITE_STATE_WRITE = 2,
    WRITE_STATE_WRITE_COMPLETE = 3,
    WRITE_STATE_DO_CALLBACK = 4,
    WRITE_STATE_HANDLE_ERROR = 5,
    WRITE_STATE_ERROR = 100,
    WRITE_STATE_IDLE = 101,
}

impl ::protobuf::ProtobufEnum for WriteState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WriteState> {
        match value {
            1 => ::std::option::Option::Some(WriteState::WRITE_STATE_UNKNOWN),
            2 => ::std::option::Option::Some(WriteState::WRITE_STATE_WRITE),
            3 => ::std::option::Option::Some(WriteState::WRITE_STATE_WRITE_COMPLETE),
            4 => ::std::option::Option::Some(WriteState::WRITE_STATE_DO_CALLBACK),
            5 => ::std::option::Option::Some(WriteState::WRITE_STATE_HANDLE_ERROR),
            100 => ::std::option::Option::Some(WriteState::WRITE_STATE_ERROR),
            101 => ::std::option::Option::Some(WriteState::WRITE_STATE_IDLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WriteState] = &[
            WriteState::WRITE_STATE_UNKNOWN,
            WriteState::WRITE_STATE_WRITE,
            WriteState::WRITE_STATE_WRITE_COMPLETE,
            WriteState::WRITE_STATE_DO_CALLBACK,
            WriteState::WRITE_STATE_HANDLE_ERROR,
            WriteState::WRITE_STATE_ERROR,
            WriteState::WRITE_STATE_IDLE,
        ];
        values
    }
}

impl ::std::marker::Copy for WriteState {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorState {
    CHANNEL_ERROR_NONE = 1,
    CHANNEL_ERROR_CHANNEL_NOT_OPEN = 2,
    CHANNEL_ERROR_AUTHENTICATION_ERROR = 3,
    CHANNEL_ERROR_CONNECT_ERROR = 4,
    CHANNEL_ERROR_SOCKET_ERROR = 5,
    CHANNEL_ERROR_TRANSPORT_ERROR = 6,
    CHANNEL_ERROR_INVALID_MESSAGE = 7,
    CHANNEL_ERROR_INVALID_CHANNEL_ID = 8,
    CHANNEL_ERROR_CONNECT_TIMEOUT = 9,
    CHANNEL_ERROR_UNKNOWN = 10,
}

impl ::protobuf::ProtobufEnum for ErrorState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorState> {
        match value {
            1 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_NONE),
            2 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_CHANNEL_NOT_OPEN),
            3 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_AUTHENTICATION_ERROR),
            4 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_CONNECT_ERROR),
            5 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_SOCKET_ERROR),
            6 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_TRANSPORT_ERROR),
            7 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_INVALID_MESSAGE),
            8 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_INVALID_CHANNEL_ID),
            9 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_CONNECT_TIMEOUT),
            10 => ::std::option::Option::Some(ErrorState::CHANNEL_ERROR_UNKNOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorState] = &[
            ErrorState::CHANNEL_ERROR_NONE,
            ErrorState::CHANNEL_ERROR_CHANNEL_NOT_OPEN,
            ErrorState::CHANNEL_ERROR_AUTHENTICATION_ERROR,
            ErrorState::CHANNEL_ERROR_CONNECT_ERROR,
            ErrorState::CHANNEL_ERROR_SOCKET_ERROR,
            ErrorState::CHANNEL_ERROR_TRANSPORT_ERROR,
            ErrorState::CHANNEL_ERROR_INVALID_MESSAGE,
            ErrorState::CHANNEL_ERROR_INVALID_CHANNEL_ID,
            ErrorState::CHANNEL_ERROR_CONNECT_TIMEOUT,
            ErrorState::CHANNEL_ERROR_UNKNOWN,
        ];
        values
    }
}

impl ::std::marker::Copy for ErrorState {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChallengeReplyErrorType {
    CHALLENGE_REPLY_ERROR_NONE = 1,
    CHALLENGE_REPLY_ERROR_PEER_CERT_EMPTY = 2,
    CHALLENGE_REPLY_ERROR_WRONG_PAYLOAD_TYPE = 3,
    CHALLENGE_REPLY_ERROR_NO_PAYLOAD = 4,
    CHALLENGE_REPLY_ERROR_PAYLOAD_PARSING_FAILED = 5,
    CHALLENGE_REPLY_ERROR_MESSAGE_ERROR = 6,
    CHALLENGE_REPLY_ERROR_NO_RESPONSE = 7,
    CHALLENGE_REPLY_ERROR_FINGERPRINT_NOT_FOUND = 8,
    CHALLENGE_REPLY_ERROR_CERT_PARSING_FAILED = 9,
    CHALLENGE_REPLY_ERROR_CERT_NOT_SIGNED_BY_TRUSTED_CA = 10,
    CHALLENGE_REPLY_ERROR_CANNOT_EXTRACT_PUBLIC_KEY = 11,
    CHALLENGE_REPLY_ERROR_SIGNED_BLOBS_MISMATCH = 12,
}

impl ::protobuf::ProtobufEnum for ChallengeReplyErrorType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChallengeReplyErrorType> {
        match value {
            1 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NONE),
            2 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_PEER_CERT_EMPTY),
            3 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_WRONG_PAYLOAD_TYPE),
            4 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NO_PAYLOAD),
            5 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_PAYLOAD_PARSING_FAILED),
            6 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_MESSAGE_ERROR),
            7 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NO_RESPONSE),
            8 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_FINGERPRINT_NOT_FOUND),
            9 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_CERT_PARSING_FAILED),
            10 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_CERT_NOT_SIGNED_BY_TRUSTED_CA),
            11 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_CANNOT_EXTRACT_PUBLIC_KEY),
            12 => ::std::option::Option::Some(ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_SIGNED_BLOBS_MISMATCH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChallengeReplyErrorType] = &[
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NONE,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_PEER_CERT_EMPTY,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_WRONG_PAYLOAD_TYPE,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NO_PAYLOAD,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_PAYLOAD_PARSING_FAILED,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_MESSAGE_ERROR,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_NO_RESPONSE,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_FINGERPRINT_NOT_FOUND,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_CERT_PARSING_FAILED,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_CERT_NOT_SIGNED_BY_TRUSTED_CA,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_CANNOT_EXTRACT_PUBLIC_KEY,
            ChallengeReplyErrorType::CHALLENGE_REPLY_ERROR_SIGNED_BLOBS_MISMATCH,
        ];
        values
    }
}

impl ::std::marker::Copy for ChallengeReplyErrorType {
}
