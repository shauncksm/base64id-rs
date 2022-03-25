// ####################################### //
// ########## 24 bit translation ######### //
// ####################################### //
//              Data  - 3 Octets           //
// 000001   00 | 0001   0000 | 01   000001 //
// 000001 | 00   0001 | 0000   01 | 000001 //
//             Base64 - 4 Chars            //
// ####################################### //

const _ALPHABET_BASE64URL: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

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

#[cfg(test)]
mod tests {
    use crate::base64;

    const QUANTUM_BINARY: [u8; 3] = [0b00000100, 0b00010000, 0b01000001];
    const QUANTUM_BASE64: [u8; 4] = [1, 1, 1, 1];

    const PARTIAL_16_BINARY: [u8; 2] = [0b00000100, 0b00010001];
    const PARTIAL_16_BASE64: [u8; 3] = [1, 1, 4];
    
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
}
