// ####################################### //
// ########## 24 bit translation ######### //
// ####################################### //
//              Data  - 3 Octets           //
// 000001   00 | 0001   0000 | 01   000001 //
// 000001 | 00   0001 | 0000   01 | 000001 //
//             Base64 - 4 Chars            //
// ####################################### //

const ALPHABET_BASE64URL: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn encode_u64(input: u64) -> [char; 11] {
    let b = input.to_be_bytes();

    let p1 = encode_quantum([ b[0], b[1], b[2] ]);
    let p2 = encode_quantum([ b[3], b[4], b[5] ]);
    let p3 = encode_partial_16([ b[6], b[7] ]);

    let product = [
        p1[0], p1[1], p1[2], p1[3],
        p2[0], p2[1], p2[2], p2[3],
        p3[0], p3[1], p3[2]
    ];

    let alphabet = ALPHABET_BASE64URL.as_bytes();

    product.map(|d| {
        char::from(
            alphabet[usize::from(d)]
        )
    })
}

pub fn decode_u64(input: [char; 11]) -> Result<u64, &'static str> {
    let mut c: [u8; 11] = [0; 11];

    for i in 0..=10 {
        let idx = ALPHABET_BASE64URL.find(input[i])
            .ok_or("char contains a non base64url character")?;

        c[i] = u8::try_from(idx)
            .map_err(|_| "infallible. failed to convert usize between 0 - 63 to u8")?;
    }

    let p1 = decode_quantum([c[0], c[1], c[2], c[3] ]);
    let p2 = decode_quantum([c[4], c[5], c[6], c[7] ]);
    let p3 = decode_partial_16([c[8], c[9], c[10] ]);

    Ok(
        u64::from_be_bytes([
            p1[0], p1[1], p1[2],
            p2[0], p2[1], p2[2],
            p3[0], p3[1]
        ])
    )
}

fn encode_quantum(input: [u8; 3]) -> [u8; 4] {
    let c1 = input[0] >> 2;

    let c2 = (
        input[1] >> 4
    ) | (
        input[0] << 4 &
        0b00110000
    );
    
    let c3 = (
        input[2] >> 6
    ) | (
        input[1] << 2 &
        0b00111100
    );
    
    let c4 = input[2] & 0b00111111;

    [c1, c2, c3, c4]
}

fn encode_partial_16(input: [u8; 2]) -> [u8; 3] {
    let c1 = input[0] >> 2;

    let c2 = (
        input[1] >> 4
    ) | (
        input[0] << 4 &
        0b00110000
    );
    
    let c3 = input[1] << 2 & 0b00111100;

    [c1, c2, c3]
}

fn decode_quantum(input: [u8; 4]) -> [u8; 3] {
    let d1 = (
        input[0] << 2
    ) | (
        input[1] >> 4 &
        0b00000011
    );

    let d2 = (
        input[1] << 4
    ) | (
        input[2] >> 2 &
        0b00001111
    );

    let d3 = (
        input[2] << 6
    ) | (
        input[3] &
        0b00111111
    );

    [d1, d2, d3]
}

fn decode_partial_16(input: [u8; 3]) -> [u8; 2] {
    let d1 = (
        input[0] << 2
    ) | (
        input[1] >> 4 &
        0b00000011
    );

    let d2 = (
        input[1] << 4
    ) | (
        input[2] >> 2 &
        0b00001111
    );

    [d1, d2]
}

#[cfg(test)]
mod tests {
    extern crate std;
    
    use crate::base64;

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

    const U64_INT: [u64; 12] = [
        u64::MAX,
        u64::MIN,
        14854615441804557868,
        2685194398265091357,
        3161873750843059683,
        10803570549866541035,
        8763564039737214670,
        17137545079498398768,
        17243084105736903942,
        4264493385337507395,
        16856446005642022447,
        6135322894040689502,
    ];
    
    const U64_BASE64: [[char; 11]; 12] = [
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '8'],
        ['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'],
        ['z', 'i', 'Y', 'y', 'C', 'o', 'v', 'K', '_', 'i', 'w'],
        ['J', 'U', 'O', '6', 'F', 'B', 'V', '1', 'p', 'R', '0'],
        ['K', '-', 'E', '7', 'b', 'I', '-', 'h', '7', 'e', 'M'],
        ['l', 'e', '3', '-', 'L', 'O', 'K', 'q', 'r', '-', 's'],
        ['e', 'Z', '5', 'v', 'E', 'B', 'L', 's', 'h', 's', '4'],
        ['7', 'd', 'T', 'K', 'c', '4', 'G', '-', '3', 'D', 'A'],
        ['7', '0', 'u', '9', 'o', 'u', 'q', 'h', '3', 'Q', 'Y'],
        ['O', 'y', '6', 'G', 'I', '8', 'Q', 'L', 'j', 'k', 'M'],
        ['6', 'e', '4', 'g', 'V', 'T', 'X', 'Y', 'T', 'i', '8'],
        ['V', 'S', 'U', 'L', 'q', 'n', 'G', 'd', '6', '1', '4'],
    ];
    
    #[test]
    fn encode_u64_validation() {
        for i in 0..=11 {
            let output = base64::encode_u64(U64_INT[i]);
            assert_eq!(output, U64_BASE64[i]);
        }
    }

    #[test]
    fn decode_u64_validation() {
        for i in 0..=11 {
            let output = base64::decode_u64(U64_BASE64[i])
                .expect("failed to decode input");
            assert_eq!(output, U64_INT[i]);
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
    fn decode_quantum_validation() {
        for i in 0..=11 {
            let output = base64::decode_quantum(QUANTUM_BASE64[i]);
            assert_eq!(output, QUANTUM_BINARY[i]);
        }
    }

    #[test]
    fn decode_partial_16_validation() {
        for i in 0..=11 {
            let output = base64::decode_partial_16(PARTIAL_16_BASE64[i]);
            assert_eq!(output, PARTIAL_16_BINARY[i]);
        }
    }
}
