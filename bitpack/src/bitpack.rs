
/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if n == -1 {
        return true;
    }
    if width == 0 {
        return n == 0;
    }

    if width >= 63 {
        return true;
    }

    let min_value = -(1i64 << (width - 1));
    let max_value = (1i64 << (width - 1)) - 1;

    return n >= min_value && n <= max_value;
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width >= 64{
        return true;
    }
    return n < safe_left_shift(1, width);
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
   assert!(width <= 64, "Invalid Width: {}", width);
    assert!(width+lsb <= 64, "Width + LSB is > 64: {} + {} > 64", width, lsb);
    
    return safe_right_shift_s(safe_left_shift_s(word as i64, 64 - (lsb + width)), 64 - width);
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    assert!(width <= 64, "Invalid Width: {}", width);
    assert!(width+lsb <= 64, "Width + LSB is > 64: {} + {} > 64", width, lsb);
    
    return safe_right_shift(safe_left_shift(word, 64 - (lsb + width)), 64 - width);
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    assert!(width <= 64, "Invalid Width: {}", width);
    assert!(width+lsb <= 64, "Width + LSB is > 64: {} + {} > 64", width, lsb);

    if !fitsu(value, width) {
        return None;
    }

    let left: u64 = safe_left_shift(safe_right_shift(word, lsb + width), lsb + width);
    let middle: u64 = safe_left_shift(value, lsb);
    let right: u64 = safe_right_shift(safe_left_shift(word, 64 - lsb), 64 - lsb);


    return Some(left | middle | right);
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    assert!(width <= 64, "Invalid Width: {}", width);
    assert!(width+lsb <= 64, "Width + LSB is > 64: {} + {} > 64", width, lsb);

    // special case where width == 1, since -1 cant be represented as unsigned
    if width == 1 && (value == 0 || value == -1) {
        let mask = 1u64 << lsb;
        if value == 0 {
            return Some(word & !mask);
        } else {
            return Some(word | mask);
        }
    }
    
    if !fitss(value, width) {
        return None;
    }
    // check_laws();

    let value_u: u64 = value as u64;
    println!("value_u: {:b}", value_u);

    let left: u64 = safe_left_shift(safe_right_shift(word, lsb + width), lsb + width);
    let middle: u64 = safe_right_shift(safe_left_shift(value_u, 64 - width), 64 - (lsb+width));
    let right: u64 = safe_right_shift(safe_left_shift(word, 64 - lsb), 64 - lsb);

    return Some(left | middle | right);
}

/**
 * Shifts a 64-bit unsigned integer value to the left by the specified amount.
 * 
 * If `shift` is greater than or equal to 64, the function returns 0.
 * 
 * # Arguments:
 * * `value`: The value to shift left
 * * `shift`: The number of bits to shift the value to the left
 * 
 * # Returns:
 * The shifted value
 */
pub fn safe_left_shift(value: u64, shift: u64) -> u64 {
    if shift >= 64 {
        0
    } else {
        value << shift
    }
}

/**
 * Shifts a 64-bit unsigned integer value to the right by the specified amount.
 * 
 * If `shift` is greater than or equal to 64, the function returns 0.
 * 
 * # Arguments:
 * * `value`: The value to shift right
 * * `shift`: The number of bits to shift the value to the right
 * 
 * # Returns:
 * The shifted value
 */
pub fn safe_right_shift(value: u64, shift: u64) -> u64 {
    if shift >= 64 {
        0
    } else {
        value >> shift
    }
}

/**
 * Shifts a 64-bit signed integer value to the left by the specified amount.
 * 
 * If `shift` is greater than or equal to 64, the function returns 0.
 * 
 * # Arguments:
 * * `value`: The value to shift left
 * * `shift`: The number of bits to shift the value to the left
 * 
 * # Returns:
 * The shifted value
 */
pub fn safe_left_shift_s(value: i64, shift: u64) -> i64 {
    if shift >= 64 {
        0
    } else {
        value << shift
    }
}

/**
 * Shifts a 64-bit signed integer value to the right by the specified amount.
 * 
 * If `shift` is greater than or equal to 64, the function returns 0.
 * 
 * # Arguments:
 * * `value`: The value to shift right
 * * `shift`: The number of bits to shift the value to the right
 * 
 * # Returns:
 * The shifted value
 */
pub fn safe_right_shift_s(value: i64, shift: u64) -> i64 {
    if shift >= 64 {
        0
    } else {
        value >> shift
    }
}





