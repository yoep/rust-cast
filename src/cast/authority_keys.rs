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
pub struct AuthorityKeys {
    // message fields
    keys: ::protobuf::RepeatedField<AuthorityKeys_Key>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthorityKeys {}

impl AuthorityKeys {
    pub fn new() -> AuthorityKeys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthorityKeys {
        static mut instance: ::protobuf::lazy::Lazy<AuthorityKeys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthorityKeys,
        };
        unsafe {
            instance.get(|| {
                AuthorityKeys {
                    keys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .extensions.api.cast_channel.proto.AuthorityKeys.Key keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<AuthorityKeys_Key>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AuthorityKeys_Key> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<AuthorityKeys_Key> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [AuthorityKeys_Key] {
        &self.keys
    }
}

impl ::protobuf::Message for AuthorityKeys {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
                }
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                                    wire_type,
                                                                    is,
                                                                    self.mut_unknown_fields()));
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.keys.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        for v in self.keys.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        }
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
        ::std::any::TypeId::of::<AuthorityKeys>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthorityKeys {
    fn new() -> AuthorityKeys {
        AuthorityKeys::new()
    }
}

impl ::protobuf::Clear for AuthorityKeys {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthorityKeys {
    fn eq(&self, other: &AuthorityKeys) -> bool {
        self.keys == other.keys && self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct AuthorityKeys_Key {
    // message fields
    fingerprint: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthorityKeys_Key {}

impl AuthorityKeys_Key {
    pub fn new() -> AuthorityKeys_Key {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthorityKeys_Key {
        static mut instance: ::protobuf::lazy::Lazy<AuthorityKeys_Key> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthorityKeys_Key,
        };
        unsafe {
            instance.get(|| {
                AuthorityKeys_Key {
                    fingerprint: ::protobuf::SingularField::none(),
                    public_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes fingerprint = 1;

    pub fn clear_fingerprint(&mut self) {
        self.fingerprint.clear();
    }

    pub fn has_fingerprint(&self) -> bool {
        self.fingerprint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fingerprint(&mut self, v: ::std::vec::Vec<u8>) {
        self.fingerprint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fingerprint<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.fingerprint.is_none() {
            self.fingerprint.set_default();
        };
        self.fingerprint.as_mut().unwrap()
    }

    // Take field
    pub fn take_fingerprint(&mut self) -> ::std::vec::Vec<u8> {
        self.fingerprint.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_fingerprint<'a>(&'a self) -> &'a [u8] {
        match self.fingerprint.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes public_key = 2;

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
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for AuthorityKeys_Key {
    fn is_initialized(&self) -> bool {
        if self.fingerprint.is_none() {
            return false;
        };
        if self.public_key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type,
                                                                  is,
                                                                  &mut self.fingerprint));
                }
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type,
                                                                  is,
                                                                  &mut self.public_key));
                }
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                                    wire_type,
                                                                    is,
                                                                    self.mut_unknown_fields()));
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.fingerprint.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        }
        for value in self.public_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fingerprint.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<AuthorityKeys_Key>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthorityKeys_Key {
    fn new() -> AuthorityKeys_Key {
        AuthorityKeys_Key::new()
    }
}

impl ::protobuf::Clear for AuthorityKeys_Key {
    fn clear(&mut self) {
        self.clear_fingerprint();
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthorityKeys_Key {
    fn eq(&self, other: &AuthorityKeys_Key) -> bool {
        self.fingerprint == other.fingerprint && self.public_key == other.public_key &&
        self.unknown_fields == other.unknown_fields
    }
}
