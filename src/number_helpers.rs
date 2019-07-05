
/// Converts an i32 to an Array of u8
/// Returns a u8 Array representing the 32-bit value
/// 
/// # Arguments
///
/// * `x` - An i32 that holds the value to be converted 
pub fn i32_to_array_of_u8(x:i32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

/// interprets an u8 ArraySlice as a single 32-bit integer
/// Returns an i32 representing the u8
/// 
/// # Arguments
///
/// * `array` - An u8 Array slice of length 4 that holds the values to be interpreted 
pub fn as_i32(array: &[u8]) -> i32 {
    ((array[0] as i32) << 24) +
    ((array[1] as i32) << 16) +
    ((array[2] as i32) <<  8) +
    ((array[3] as i32) <<  0)
}