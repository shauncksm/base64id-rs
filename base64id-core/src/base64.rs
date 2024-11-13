// ####################################### //
// ########## 24 bit translation ######### //
// ####################################### //
//              Data  - 3 Octets           //
// 000001   00 | 0001   0000 | 01   000001 //
// 000001 | 00   0001 | 0000   01 | 000001 //
//             Base64 - 4 Chars            //
// ####################################### //

use crate::Error;

const ALPHABET_BASE64URL: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const ALPHABET_BASE64URL_BYTES: &[u8] = ALPHABET_BASE64URL.as_bytes();

#[must_use]
fn decode_char(c: char) -> Result<u8, Error> {
    let idx = match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '-' => 62,
        '_' => 63,
        _ => return Err(Error::InvalidCharacter),
    };

    Ok(idx)
}

#[must_use]
pub fn encode_i64(input: i64) -> [char; 11] {
    encode_64(input.to_be_bytes())
}

#[must_use]
pub fn encode_u64(input: u64) -> [char; 11] {
    encode_64(input.to_be_bytes())
}

#[must_use]
#[rustfmt::skip]
fn encode_64(bytes: [u8; 8]) -> [char; 11] {
    let p1 = encode_quantum([bytes[0], bytes[1], bytes[2]]);
    let p2 = encode_quantum([bytes[3], bytes[4], bytes[5]]);
    let p3 = encode_partial_16([bytes[6], bytes[7]]);

    let product = [
        p1[0], p1[1], p1[2], p1[3],
        p2[0], p2[1], p2[2], p2[3],
        p3[0], p3[1], p3[2],
    ];

    product.map(|d| char::from(ALPHABET_BASE64URL_BYTES[usize::from(d)]))
}

#[must_use]
pub fn encode_i32(input: i32) -> [char; 6] {
    encode_32(input.to_be_bytes())
}

#[must_use]
pub fn encode_u32(input: u32) -> [char; 6] {
    encode_32(input.to_be_bytes())
}

#[must_use]
#[rustfmt::skip]
fn encode_32(bytes: [u8; 4]) -> [char; 6] {
    let p1 = encode_quantum([bytes[0], bytes[1], bytes[2]]);
    let p2 = encode_partial_8(bytes[3]);

    let product = [
        p1[0], p1[1], p1[2], p1[3],
        p2[0], p2[1],
    ];

    product.map(|d| char::from(ALPHABET_BASE64URL_BYTES[usize::from(d)]))
}

#[must_use]
pub fn encode_i16(input: i16) -> [char; 3] {
    encode_16(input.to_be_bytes())
}

#[must_use]
pub fn encode_u16(input: u16) -> [char; 3] {
    encode_16(input.to_be_bytes())
}

#[must_use]
fn encode_16(bytes: [u8; 2]) -> [char; 3] {
    let product = encode_partial_16([bytes[0], bytes[1]]);

    product.map(|d| char::from(ALPHABET_BASE64URL_BYTES[usize::from(d)]))
}

#[must_use]
pub fn decode_i64(input: [char; 11]) -> Result<i64, Error> {
    let bytes = decode_64(input)?;
    Ok(i64::from_be_bytes(bytes))
}

#[must_use]
pub fn decode_u64(input: [char; 11]) -> Result<u64, Error> {
    let bytes = decode_64(input)?;
    Ok(u64::from_be_bytes(bytes))
}

#[must_use]
#[rustfmt::skip]
fn decode_64(input: [char; 11]) -> Result<[u8; 8], Error> {
    let mut c: [u8; 11] = [0; 11];

    for i in 0..=10 {
        c[i] = decode_char(input[i])?;
    }

    let p1 = decode_quantum([c[0], c[1], c[2], c[3]]);
    let p2 = decode_quantum([c[4], c[5], c[6], c[7]]);
    let p3 = decode_partial_16([c[8], c[9], c[10]])?;

    Ok([
        p1[0], p1[1], p1[2],
        p2[0], p2[1], p2[2],
        p3[0], p3[1]
    ])
}

#[must_use]
pub fn decode_i32(input: [char; 6]) -> Result<i32, Error> {
    let bytes = decode_32(input)?;
    Ok(i32::from_be_bytes(bytes))
}

#[must_use]
pub fn decode_u32(input: [char; 6]) -> Result<u32, Error> {
    let bytes = decode_32(input)?;
    Ok(u32::from_be_bytes(bytes))
}

#[must_use]
#[rustfmt::skip]
fn decode_32(input: [char; 6]) -> Result<[u8; 4], Error> {
    let mut c: [u8; 6] = [0; 6];

    for i in 0..=5 {
        c[i] = decode_char(input[i])?;
    }

    let p1 = decode_quantum([c[0], c[1], c[2], c[3]]);
    let p2 = decode_partial_8([c[4], c[5]])?;

    Ok([
        p1[0], p1[1], p1[2],
        p2
    ])
}

#[must_use]
pub fn decode_i16(input: [char; 3]) -> Result<i16, Error> {
    let bytes = decode_16(input)?;
    Ok(i16::from_be_bytes(bytes))
}

#[must_use]
pub fn decode_u16(input: [char; 3]) -> Result<u16, Error> {
    let bytes = decode_16(input)?;
    Ok(u16::from_be_bytes(bytes))
}

#[must_use]
fn decode_16(input: [char; 3]) -> Result<[u8; 2], Error> {
    let mut c: [u8; 3] = [0; 3];

    for i in 0..=2 {
        c[i] = decode_char(input[i])?;
    }

    let p1 = decode_partial_16(c)?;

    Ok(p1)
}

#[must_use]
#[rustfmt::skip]
fn encode_quantum(input: [u8; 3]) -> [u8; 4] {
    let c1 = input[0] >> 2;

    let c2 = (
        input[1] >> 4
    ) | (
        input[0] << 4 &
        0b0011_0000
    );
    
    let c3 = (
        input[2] >> 6
    ) | (
        input[1] << 2 &
        0b0011_1100
    );
    
    let c4 = input[2] & 0b0011_1111;

    [c1, c2, c3, c4]
}

#[must_use]
#[rustfmt::skip]
fn encode_partial_16(input: [u8; 2]) -> [u8; 3] {
    let c1 = input[0] >> 2;

    let c2 = (
        input[1] >> 4
    ) | (
        input[0] << 4 &
        0b0011_0000
    );
    
    let c3 = input[1] << 2 & 0b0011_1100;

    [c1, c2, c3]
}

#[must_use]
#[rustfmt::skip]
fn encode_partial_8(input: u8) -> [u8; 2] {
    let c1 = input >> 2;

    let c2 = input << 4 & 0b0011_0000;

    [c1, c2]
}

#[must_use]
#[rustfmt::skip]
fn decode_quantum(input: [u8; 4]) -> [u8; 3] {
    let d1 = (
        input[0] << 2
    ) | (
        input[1] >> 4 &
        0b0000_0011
    );

    let d2 = (
        input[1] << 4
    ) | (
        input[2] >> 2 &
        0b0000_1111
    );

    let d3 = (
        input[2] << 6
    ) | (
        input[3] &
        0b0011_1111
    );

    [d1, d2, d3]
}

#[must_use]
#[rustfmt::skip]
fn decode_partial_16(input: [u8; 3]) -> Result<[u8; 2], Error> {
    if input[2] & 0b0000_0011 != 0 {
        return Err(Error::OutOfBoundsCharacter);
    }
    
    let d1 = (
        input[0] << 2
    ) | (
        input[1] >> 4 &
        0b0000_0011
    );

    let d2 = (
        input[1] << 4
    ) | (
        input[2] >> 2 &
        0b0000_1111
    );

    Ok([d1, d2])
}

#[must_use]
#[rustfmt::skip]
fn decode_partial_8(input: [u8; 2]) -> Result<u8, Error> {
    if input[1] & 0b0000_1111 != 0 {
        return Err(Error::OutOfBoundsCharacter);
    }

    let d1 = (
        input[0] << 2
    ) | (
        input[1] >> 4
    );

    Ok(d1)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::{base64, Error};

    const QUANTUM_BINARY: [[u8; 3]; 12] = [
        [0b00000100, 0b00010000, 0b01000001],
        [0b10010011, 0b10100110, 0b01001110],
        [0b11111010, 0b01111101, 0b00011001],
        [0b10001101, 0b01110110, 0b00011111],
        [0b00011011, 0b11010010, 0b10011010],
        [0b11011111, 0b11001011, 0b01001101],
        [0b01100001, 0b01000010, 0b10100101],
        [0b01010011, 0b01101011, 0b01000101],
        [0b00011100, 0b11000011, 0b11100001],
        [0b01000010, 0b11111110, 0b01001100],
        [0b00000100, 0b10110001, 0b00111110],
        [0b11001100, 0b10101011, 0b00011100],
    ];

    const QUANTUM_BASE64: [[u8; 4]; 12] = [
        [1, 1, 1, 1],
        [36, 58, 25, 14],
        [62, 39, 52, 25],
        [35, 23, 24, 31],
        [6, 61, 10, 26],
        [55, 60, 45, 13],
        [24, 20, 10, 37],
        [20, 54, 45, 5],
        [7, 12, 15, 33],
        [16, 47, 57, 12],
        [1, 11, 4, 62],
        [51, 10, 44, 28],
    ];

    const PARTIAL_16_BINARY: [[u8; 2]; 12] = [
        [0b00000100, 0b00010001],
        [0b11010000, 0b10001110],
        [0b10110000, 0b00001100],
        [0b11011111, 0b10100010],
        [0b01100100, 0b00110101],
        [0b10001000, 0b01001001],
        [0b00001101, 0b00110100],
        [0b10010110, 0b00110010],
        [0b00001100, 0b00001110],
        [0b00010100, 0b11101110],
        [0b00010111, 0b01110111],
        [0b00100010, 0b11110101],
    ];

    const PARTIAL_16_BASE64: [[u8; 3]; 12] = [
        [1, 1, 4],
        [52, 8, 56],
        [44, 0, 48],
        [55, 58, 8],
        [25, 3, 20],
        [34, 4, 36],
        [3, 19, 16],
        [37, 35, 8],
        [3, 0, 56],
        [5, 14, 56],
        [5, 55, 28],
        [8, 47, 20],
    ];

    /// All base64 u8 values who's first two bits contain a 1
    const PARTIAL_16_BASE64_OUTOFBOUNDS: [u8; 48] = [
        0b000001, 0b000010, 0b000011, 0b000101, 0b000110, 0b000111, 0b001001, 0b001010, 0b001011,
        0b001101, 0b001110, 0b001111, 0b010001, 0b010010, 0b010011, 0b010101, 0b010110, 0b010111,
        0b011001, 0b011010, 0b011011, 0b011101, 0b011110, 0b011111, 0b100001, 0b100010, 0b100011,
        0b100101, 0b100110, 0b100111, 0b101001, 0b101010, 0b101011, 0b101101, 0b101110, 0b101111,
        0b110001, 0b110010, 0b110011, 0b110101, 0b110110, 0b110111, 0b111001, 0b111010, 0b111011,
        0b111101, 0b111110, 0b111111,
    ];

    /// All base64 u8 values who's first four bits contain a 1
    const PARTIAL_8_BASE64_OUTOFBOUNDS: [u8; 60] = [
        0b000001, 0b000010, 0b000011, 0b000100, 0b000101, 0b000110, 0b000111, 0b001000, 0b001001,
        0b001010, 0b001011, 0b001100, 0b001101, 0b001110, 0b001111, 0b010001, 0b010010, 0b010011,
        0b010100, 0b010101, 0b010110, 0b010111, 0b011000, 0b011001, 0b011010, 0b011011, 0b011100,
        0b011101, 0b011110, 0b011111, 0b100001, 0b100010, 0b100011, 0b100100, 0b100101, 0b100110,
        0b100111, 0b101000, 0b101001, 0b101010, 0b101011, 0b101100, 0b101101, 0b101110, 0b101111,
        0b110001, 0b110010, 0b110011, 0b110100, 0b110101, 0b110110, 0b110111, 0b111000, 0b111001,
        0b111010, 0b111011, 0b111100, 0b111101, 0b111110, 0b111111,
    ];

    const PARTIAL_8_BINARY: [u8; 12] = [1, 83, 207, 157, 81, 166, 160, 236, 107, 123, 195, 96];

    const PARTIAL_8_BASE64: [[u8; 2]; 12] = [
        [0b000000, 0b010000],
        [0b010100, 0b110000],
        [0b110011, 0b110000],
        [0b100111, 0b010000],
        [0b010100, 0b010000],
        [0b101001, 0b100000],
        [0b101000, 0b000000],
        [0b111011, 0b000000],
        [0b011010, 0b110000],
        [0b011110, 0b110000],
        [0b110000, 0b110000],
        [0b011000, 0b000000],
    ];

    const I64_INT: [i64; 12] = [
        i64::from_be_bytes(u64::MAX.to_be_bytes()),
        0,
        6926187988806058650,
        2685194398265091357,
        3161873750843059683,
        -100214607134046418,
        8763564039737214670,
        -1707663070116128918,
        -3237707832123248052,
        4264493385337507395,
        -8390618481221224288,
        6135322894040689502,
    ];

    const U64_INT: [u64; 12] = [
        u64::from_be_bytes(u64::MAX.to_be_bytes()),
        0,
        6926187988806058650,
        2685194398265091357,
        3161873750843059683,
        18346529466575505198,
        8763564039737214670,
        16739081003593422698,
        15209036241586303564,
        4264493385337507395,
        10056125592488327328,
        6135322894040689502,
    ];

    const BASE64_64_BIT: [[char; 11]; 12] = [
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '8'],
        ['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'],
        ['Y', 'B', '7', 'D', 'R', '4', 'B', 'c', 'w', 'p', 'o'],
        ['J', 'U', 'O', '6', 'F', 'B', 'V', '1', 'p', 'R', '0'],
        ['K', '-', 'E', '7', 'b', 'I', '-', 'h', '7', 'e', 'M'],
        ['_', 'p', 'v', '3', 'W', 'I', 'R', '4', 'R', 'y', '4'],
        ['e', 'Z', '5', 'v', 'E', 'B', 'L', 's', 'h', 's', '4'],
        ['6', 'E', '0', 'p', 'f', 'D', '2', 's', '1', '2', 'o'],
        ['0', 'x', 'F', 'Z', '4', 'X', 'v', 'L', 'M', 'k', 'w'],
        ['O', 'y', '6', 'G', 'I', '8', 'Q', 'L', 'j', 'k', 'M'],
        ['i', '4', '6', 'I', '8', 'l', 'z', '7', 'O', 'K', 'A'],
        ['V', 'S', 'U', 'L', 'q', 'n', 'G', 'd', '6', '1', '4'],
    ];

    const I32_INT: [i32; 12] = [
        -1,
        0,
        -1674465201,
        1200409755,
        -19087501,
        489625798,
        -2147100188,
        -1806006017,
        -818530506,
        -864315129,
        1069495787,
        -877220880,
    ];

    const U32_INT: [u32; 12] = [
        u32::from_be_bytes(u32::MAX.to_be_bytes()),
        0,
        2620502095,
        1200409755,
        4275879795,
        489625798,
        2147867108,
        2488961279,
        3476436790,
        3430652167,
        1069495787,
        3417746416,
    ];

    const BASE64_32_BIT: [[char; 6]; 12] = [
        ['_', '_', '_', '_', '_', 'w'],
        ['A', 'A', 'A', 'A', 'A', 'A'],
        ['n', 'D', 'G', 'w', 'T', 'w'],
        ['R', '4', 'z', 'M', 'm', 'w'],
        ['_', 't', 'y', '_', 'c', 'w'],
        ['H', 'S', '8', 'Y', 'x', 'g'],
        ['g', 'A', 'X', 'Z', '5', 'A'],
        ['l', 'F', 'q', 'I', '_', 'w'],
        ['z', 'z', 'Y', '3', 'N', 'g'],
        ['z', 'H', 'u', 'Z', 'B', 'w'],
        ['P', '7', '8', '1', '6', 'w'],
        ['y', '7', 'a', 'r', '8', 'A'],
    ];

    const I16_INT: [i16; 12] = [
        -1, 0, 30008, -2225, -14855, -23609, 30937, -6812, -24065, 13170, -8520, -13636,
    ];

    const U16_INT: [u16; 12] = [
        u16::from_be_bytes(u16::MAX.to_be_bytes()),
        0,
        30008,
        63311,
        50681,
        41927,
        30937,
        58724,
        41471,
        13170,
        57016,
        51900,
    ];

    const BASE64_16_BIT: [[char; 3]; 12] = [
        ['_', '_', '8'],
        ['A', 'A', 'A'],
        ['d', 'T', 'g'],
        ['9', '0', '8'],
        ['x', 'f', 'k'],
        ['o', '8', 'c'],
        ['e', 'N', 'k'],
        ['5', 'W', 'Q'],
        ['o', 'f', '8'],
        ['M', '3', 'I'],
        ['3', 'r', 'g'],
        ['y', 'r', 'w'],
    ];

    #[test]
    fn encode_i64_validation() {
        for i in 0..=11 {
            let output = base64::encode_i64(I64_INT[i]);
            assert_eq!(output, BASE64_64_BIT[i]);
        }
    }

    #[test]
    fn encode_i32_validation() {
        for i in 0..=11 {
            let output = base64::encode_i32(I32_INT[i]);
            assert_eq!(output, BASE64_32_BIT[i]);
        }
    }

    #[test]
    fn encode_i16_validation() {
        for i in 0..=11 {
            let output = base64::encode_i16(I16_INT[i]);
            assert_eq!(output, BASE64_16_BIT[i]);
        }
    }

    #[test]
    fn decode_i64_validation() {
        for i in 0..=11 {
            let output = base64::decode_i64(BASE64_64_BIT[i]).expect("failed to decode input");
            assert_eq!(output, I64_INT[i]);
        }
    }

    #[test]
    fn decode_i32_validation() {
        for i in 0..=11 {
            let output = base64::decode_i32(BASE64_32_BIT[i]).expect("failed to decode input");
            assert_eq!(output, I32_INT[i]);
        }
    }

    #[test]
    fn decode_i16_validation() {
        for i in 0..=11 {
            let output = base64::decode_i16(BASE64_16_BIT[i]).expect("failed to decode input");
            assert_eq!(output, I16_INT[i]);
        }
    }

    #[test]
    fn encode_u64_validation() {
        for i in 0..=11 {
            let output = base64::encode_u64(U64_INT[i]);
            assert_eq!(output, BASE64_64_BIT[i]);
        }
    }

    #[test]
    fn encode_u32_validation() {
        for i in 0..=11 {
            let output = base64::encode_u32(U32_INT[i]);
            assert_eq!(output, BASE64_32_BIT[i]);
        }
    }

    #[test]
    fn encode_u16_validation() {
        for i in 0..=11 {
            let output = base64::encode_u16(U16_INT[i]);
            assert_eq!(output, BASE64_16_BIT[i]);
        }
    }

    #[test]
    fn decode_u64_validation() {
        for i in 0..=11 {
            let output = base64::decode_u64(BASE64_64_BIT[i]).expect("failed to decode input");
            assert_eq!(output, U64_INT[i]);
        }
    }

    #[test]
    fn decode_u32_validation() {
        for i in 0..=11 {
            let output = base64::decode_u32(BASE64_32_BIT[i]).expect("failed to decode input");
            assert_eq!(output, U32_INT[i]);
        }
    }

    #[test]
    fn decode_u16_validation() {
        for i in 0..=11 {
            let output = base64::decode_u16(BASE64_16_BIT[i]).expect("failed to decode input");
            assert_eq!(output, U16_INT[i]);
        }
    }

    #[test]
    fn encode_quantum_validation() {
        for i in 0..=11 {
            let output = base64::encode_quantum(QUANTUM_BINARY[i]);
            assert_eq!(output, QUANTUM_BASE64[i]);
        }
    }

    #[test]
    fn encode_partial_16_validation() {
        for i in 0..=11 {
            let output = base64::encode_partial_16(PARTIAL_16_BINARY[i]);
            assert_eq!(output, PARTIAL_16_BASE64[i]);
        }
    }

    #[test]
    fn encode_partial_8_validation() {
        for i in 0..=11 {
            let output = base64::encode_partial_8(PARTIAL_8_BINARY[i]);
            assert_eq!(output, PARTIAL_8_BASE64[i]);
        }
    }

    #[test]
    fn decode_quantum_validation() {
        for i in 0..=11 {
            let output = base64::decode_quantum(QUANTUM_BASE64[i]);
            assert_eq!(output, QUANTUM_BINARY[i]);
        }
    }

    #[test]
    fn decode_partial_16_validation() {
        for i in 0..=11 {
            let output = base64::decode_partial_16(PARTIAL_16_BASE64[i])
                .expect("decode_partial_16 returned an unexpected Err");

            assert_eq!(output, PARTIAL_16_BINARY[i]);
        }
    }

    #[test]
    fn decode_partial_8_validation() {
        for i in 0..=11 {
            let output = base64::decode_partial_8(PARTIAL_8_BASE64[i])
                .expect("decode_partial_8 returned an unexpected Err");

            assert_eq!(output, PARTIAL_8_BINARY[i]);
        }
    }

    /// decode_partial_16() should return an Error::OutOfBoundsCharacter
    /// when an out of bounds character is encountered
    ///
    /// Such characters include any character who's base64 index number, expressed as a u8, has it's first and/or second bit set to 1.
    ///
    /// Such u8 values can be detected with the following test:
    /// ```
    /// // base64url character 'B'. Binary 0b00000001
    /// // First bit is a 1. This is out of bounds.
    /// let int = 1u8;
    ///
    /// if int & 0b00000011 != 0 {
    ///     panic!("It's out of bounds!")
    /// }
    /// ```
    #[test]
    fn decode_partial_16_out_of_bounds_detection() {
        for i in PARTIAL_16_BASE64_OUTOFBOUNDS {
            let output = base64::decode_partial_16([0, 0, i])
                .expect_err("decode_partial_16 did not return an Err");

            assert_eq!(output, Error::OutOfBoundsCharacter);
        }
    }

    /// decode_partial_8() should return an Error::OutOfBoundsCharacter
    /// when an out of bounds character is encountered
    ///
    /// Such characters include any character who's base64 index number, expressed as a u8, has any combination of it's first, second, third for forth bit set to 1.
    ///
    /// Such u8 values can be detected with the following test:
    /// ```
    /// // base64url character 'B'. Binary 0b00000001
    /// // First bit is a 1. This is out of bounds.
    /// let int = 1u8;
    ///
    /// if int & 0b00001111 != 0 {
    ///     panic!("It's out of bounds!")
    /// }
    /// ```
    #[test]
    fn decode_partial_8_out_of_bounds_detection() {
        for i in PARTIAL_8_BASE64_OUTOFBOUNDS {
            let output = base64::decode_partial_8([0, i])
                .expect_err("decode_partial_8 did not return an Err");

            assert_eq!(output, Error::OutOfBoundsCharacter);
        }
    }
}
