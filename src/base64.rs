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

#[cfg(test)]
mod tests {
    use crate::base64;
    #[test]
    fn encode_quantum_validation() {
        let input: [u8; 3] = [0b00000100, 0b00010000, 0b01000001];
        let output = base64::encode_quantum(input);
        assert_eq!(output, [1, 1, 1, 1]);
    }
}
