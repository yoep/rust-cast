use std::error::Error;
use std::io::Cursor;
use std::mem::transmute;
use std::ptr::copy_nonoverlapping;

use protobuf;

pub fn read_u32_from_buffer(buffer: &[u8]) -> u32 {
    unsafe { (*(buffer.as_ptr() as *const u32)).to_be() }
}

pub fn write_u32_to_buffer(buffer: &mut [u8], number: u32) {
    unsafe {
        let bytes = transmute::<_, [u8; 4]>(number.to_be());
        copy_nonoverlapping((&bytes).as_ptr(), buffer.as_mut_ptr(), 4);
    }
}

pub fn to_vec<M: protobuf::Message>(message: M) -> Result<Vec<u8>, Box<Error>> {
    let mut buffer: Vec<u8> = Vec::new();

    try!(message.write_to_writer(&mut buffer));

    Ok(buffer)
}

pub fn from_vec<M: protobuf::MessageStatic>(buffer: Vec<u8>) -> Result<M, Box<Error>> {
    let mut read_buffer = Cursor::new(buffer);

    Ok(try!(protobuf::parse_from_reader::<M>(&mut read_buffer)))
}