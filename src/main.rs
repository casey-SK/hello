//#![no_std]

use heapless::Vec;

fn main() {

    let input = create_input_data();
    let ((data, num), (_, _)) = read_until(input, b'\n');

    println!("{:?}", &data[..num])
}

/// returns a tuple of two tuples
///     Tuple 1 - contains the output buffer until the seperator,
///     Tuple 2 - contains the extra bits from after the seperator that can be further used as needed.
/// 
///     TODO: manage the case where you don't find the seperator before reaching 64 bytes, so now the 
///     output needs to span multiple buffers (how many buffers????)
fn read_until(input: [([u8; 64], usize); 8], sep: u8) -> (([u8; 64], usize), ([u8; 64], usize))  {
    
    let mut found_sep = false;
    let mut i: usize = 0;
    let mut output: Vec<u8, 64> = Vec::new();
    let mut extras: Vec<u8, 64> = Vec::new();
    let mut n1: usize = 0;
    let mut n2: usize = 0;

    while !found_sep {
        let input_buffer = input[i].0;
        let input_size = input[i].1;
        let input_window = &input_buffer[..input_size];
        if input_window.contains(&sep) {
            let mut count: usize = 0;
            for i in input_window{
                if i == &sep { 
                    output.extend_from_slice(&input_window[..(count + 1)]).unwrap();
                    extras.extend_from_slice(&input_window[(count + 1)..]).unwrap();
                    n1 = n1 + input_size;
                    n2 = input_size - count;
                    found_sep = true;
                } else {
                    count += 1;
                }
            }
        } else {
            output.extend_from_slice(input_window).unwrap();
            n1 = n1 + input_size
        }
        i += 1;
    }
    
    let mut out = [0u8; 64];
    let mut idx: usize = 0;
    for i in output {
        out[idx] = i;
        idx = idx + 1;
    }

    let mut ext = [0u8; 64]; // extras
    let mut idx: usize = 0;
    for i in extras {
        ext[idx] = i;
        idx = idx + 1;
    }

    return ((out, n1), (ext, n2));

}



fn create_input_data() -> [([u8; 64], usize); 8] {
    // "hello, world!\n" in a bunch of chunks of between 1 and 3 bytes, random. 
    let input_line = "Hello, World!\n".as_bytes();

    println!("{:?}", input_line);

    let minus_1 = [b'\0'; 63];
    let minus_2 = [b'\0'; 62];
    let minus_3 = [b'\0'; 61];


    // Manually create random data
    let in_1: [u8; 64] = [&input_line[..1], minus_1.as_slice()].concat().try_into().unwrap();
    let in_2: [u8; 64] = [&input_line[1..3], minus_2.as_slice()].concat().try_into().unwrap();
    let in_3: [u8; 64] = [&input_line[3..4], minus_1.as_slice()].concat().try_into().unwrap();
    let in_4: [u8; 64] = [&input_line[4..7], minus_3.as_slice()].concat().try_into().unwrap();
    let in_5: [u8; 64] = [&input_line[7..9], minus_2.as_slice()].concat().try_into().unwrap();
    let in_6: [u8; 64] = [&input_line[9..11], minus_2.as_slice()].concat().try_into().unwrap();
    let in_7: [u8; 64] = [&input_line[11..12], minus_1.as_slice()].concat().try_into().unwrap();
    let in_8: [u8; 64] = [&input_line[12..14], minus_2.as_slice()].concat().try_into().unwrap();



    let input_stream = [
        (in_1, 1usize),
        (in_2, 2usize),
        (in_3, 1usize),
        (in_4, 3usize),
        (in_5, 2usize),
        (in_6, 2usize),
        (in_7, 1usize),
        (in_8, 2usize),
        ];
    
    return  input_stream;
}

