use std::io;
use base64alphabet::ASCII_BASE64;

mod base64alphabet;

/// Encode utf-8 (backward compatible to ascii) to base64 format
fn encode(mut sen: String) -> String {
    // Trim begining and trailing whitespaces
    sen = sen.trim().to_string();

    // Split to single character unit and get each character binary representation
    let mut char_bin_rep: String = String::new();
    for char_str in sen.split("").collect::<Vec<_>>() {
        let bytes = char_str.as_bytes();
        
        if !bytes.is_empty() {
            let byte = &format!("{:08b}", bytes[0])[..];
            char_bin_rep.push_str(byte);
        }
    };

    // Split 8 character binary code to 6 character binary code in order: left -> right
    let s = char_bin_rep.split("").filter(|v| v.len() > 0).collect::<Vec<_>>();
    let g_c6 = s.chunks(6).collect::<Vec<_>>();

    // Get character assigned to binary number from base64 alphabet (binary number is from above section)
    let mut ready_encoded = String::new();
    for b_seq in g_c6.iter() {
        let mut action = |b_seq: String| {
            // let b_seq = b_seq.join(""); // 6 code binary is here
            let d_rep = u8::from_str_radix(&b_seq, 2).unwrap(); // decimal number which is under binary representation
            
            // Try to find character in base64 alphabet (depends on decimal number as index)
            let char_to_insert = ASCII_BASE64.iter()
                .find(|(id, _)| {
                    return *id == d_rep;
                });
    
            // When character has been found then put them otherwise insert base64 padding character "="
            match char_to_insert {
                Some((_, char)) => ready_encoded.push(*char),
                None => ready_encoded.push(ASCII_BASE64[64].1)
            }
        };

        if b_seq.len() == 6 { // simple join bytes
            let b_seq = b_seq.join(""); // joined collection of bytes
            action(b_seq);
        } else if b_seq.into_iter().any(|v| v.contains("1")) { // fill to 6 bytes and perform same action
            let zeros = {
                let zero_nums = 6 - b_seq.len() as u8;

                let mut i = 0;
                let mut z = String::new();

                while i < zero_nums {
                    z.push('0');
                    i+=1;
                };
                z
            };

            let mut to_edit = b_seq.join("");
            let _ = to_edit.push_str(&zeros);

            action(to_edit);
        }
    }

    // Insert padding at the end of base64 encoding result when condtion arrived (result of dviding chunks must be equal 0 because: ./because.txt) (maximum can be inserted two paddings)
    let insert_paddings = 4 - g_c6.len() % 4;
    if insert_paddings != 4 {
        let paddings: String = {
            let mut p = String::new();
            for _ in 0..insert_paddings {
                p.push('=');
            };
            p
        };
        ready_encoded.push_str(&paddings);
    }

    // Return result
    ready_encoded
}

fn decode(mut d_sen: String) -> String {
    
}

fn main() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();

    // println!("{}", 0b00000011u8);
    let encoded = encode(buff);
    println!("{encoded}")
}

#[test]
fn encoding_test() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();

    let encoded = encode(buff);
    println!("{encoded}")
}
