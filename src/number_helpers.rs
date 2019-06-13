
pub fn i32_to_array_of_u8(x:i32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn as_i32(array: &[u8]) -> i32 {
    ((array[0] as i32) << 24) +
    ((array[1] as i32) << 16) +
    ((array[2] as i32) <<  8) +
    ((array[3] as i32) <<  0)
}