use std::io;
use std::str;
use std::time::{ Instant };
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

/// Decoding base64 to utf-8
fn decode(mut d_sen: String) -> String {
    // Remove paddings
    d_sen = d_sen.replace("=", "");

    // Create binary chain in string
    let mut as_6bites = String::new();
    let d_s_s = d_sen.split("").filter(|v| v.len() > 0).collect::<Vec<_>>();
    
    for char in d_s_s {
        let char_num_b64alph = ASCII_BASE64.iter()
            .find(|(_, c)| c.to_string() == char)
            .unwrap().0;
        let char_binary = format!("{char_num_b64alph:06b}");
        as_6bites.push_str(&char_binary)
    };

    // Create 8 characters chunks
    let b8_chunks = as_6bites
        .split("")
        .filter(|v| v.len() > 0)
        .collect::<Vec<_>>();
    let b8_chunks = b8_chunks.chunks(8)
        .collect::<Vec<_>>();

    // Decode characters from base64 to utf-8 again
    let mut result = String::new();
    for chunk in b8_chunks {
        // Decoding operation = obtain utf-8 character clasified under base64 character
        let mut op = |chunk: String| {
            let chunk_byte = u8::from_str_radix(&chunk, 2).unwrap();
            let ch = String::from_utf8(vec![chunk_byte]).unwrap();
            result.push_str(&ch)
        };

        // Perform specific operation to meeted condition
        if chunk.len() == 8 {
            // Merge as string chain
            let chunk = chunk.join("");
            
            // Decode operation
            op(chunk)
        } else {
            // Obtain set to refill chain tail with lacking zeros (0)
            let lacking_zeros = {
                let mut res: Vec<&str> = vec![];
                let c = 8 - chunk.len();
                for _ in 0..c {
                    res.push("0");
                }
                res
            };

            // Merge as string chain which has got 8 characters (at the tail are 0)
            let chunk = vec![chunk.to_vec(), lacking_zeros].concat().join("");

            // Decode operation
            op(chunk);
        } 
        // (else) if consist from 0's and is not equal to 8 bytes ignore forth operation
    }

    result
}

fn main() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();

    // Measure time required to perform both: encoding and decoding
    let whole_cycle = Instant::now();

    // Encoding
    // Measure time required to perform encoding
    let encoding_t = Instant::now();
    let encoded = encode(buff);
    println!("Encoded: {encoded}");
    println!("Encoding operation takes: {}ns", encoding_t.elapsed().as_nanos());

    // Decoding
    // Measure time required to perform decoding
    let decoding_t = Instant::now();
    let decoded = decode(encoded);
    println!("Decoded: {decoded}");
    println!("Decoding operation takes: {}ns", decoding_t.elapsed().as_nanos());

    println!("Whole circle takes: {}ns", whole_cycle.elapsed().as_nanos());
}

#[test]
fn encoding_test() {
    let ex = String::from("Example to encode");

    // Encoding
    let encoded = encode(ex);
    println!("Encoded: {encoded}")
}

#[test]
fn decoding_test() {
    let ex = String::from("Example to encode and decode");

    // Must be firstly encoded to check decoding
    let e_ = encode(ex);

    // Decoding prior encoded
    let decoded = decode(e_);
    println!("Decoded: {decoded}");
}
