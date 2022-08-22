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
        p2[0], p2[1], p2[2], p1[3],
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

    const QUANTUM_BINARY: [u8; 3] = [0b00000100, 0b00010000, 0b01000001];
    const QUANTUM_BASE64: [u8; 4] = [1, 1, 1, 1];

    const PARTIAL_16_BINARY: [u8; 2] = [0b00000100, 0b00010001];
    const PARTIAL_16_BASE64: [u8; 3] = [1, 1, 4];

    const U64_MAX_INT: u64 = u64::MAX;
    const U64_MAX_BASE64: [char; 11] = ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '8'];
    
    #[test]
    fn encode_u64_validation() {
        let input: u64 = U64_MAX_INT;
        let output = base64::encode_u64(input);

        assert_eq!(output, U64_MAX_BASE64);
    }

    #[test]
    fn decode_u64_validation() {
        let input: [char; 11] = U64_MAX_BASE64;
        let output = base64::decode_u64(input)
            .expect("failed to decode input");

        assert_eq!(output, U64_MAX_INT);
    }

    #[test]
    fn encode_quantum_validation() {
        let input = QUANTUM_BINARY;
        let output = base64::encode_quantum(input);
        assert_eq!(output, QUANTUM_BASE64);
    }

    #[test]
    fn encode_partial_16_validation() {
        let output = base64::encode_partial_16(PARTIAL_16_BINARY);
        assert_eq!(output, PARTIAL_16_BASE64);
    }

    #[test]
    fn decode_quantum_validation() {
        let input = QUANTUM_BASE64;
        let output = base64::decode_quantum(input);
        assert_eq!(output, QUANTUM_BINARY);
    }

    #[test]
    fn decode_partial_16_validation() {
        let output = base64::decode_partial_16(PARTIAL_16_BASE64);
        assert_eq!(output, PARTIAL_16_BINARY);
    }
}
