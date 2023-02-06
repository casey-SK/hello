
fn main() {

    let input_stream = create_input_data();
    
}


fn create_input_data() -> [[u8; 64]; 8] {
    // "hello, world!\n" in a bunch of chunks of between 1 and 3 bytes, random. 
    let input_line = "Hello, World!\n".as_bytes();
    println!("{:?}", input_line);

    let minus_1 = [b'\0'; 63];
    let minus_2 = [b'\0'; 62];
    let minus_3 = [b'\0'; 61];


    // Manually create random data
    let in_1 = [&input_line[..1], minus_1.as_slice()].concat();
    let in_2 = [&input_line[1..3], minus_2.as_slice()].concat();
    let in_3 = [&input_line[3..4], minus_1.as_slice()].concat();
    let in_4 = [&input_line[4..7], minus_3.as_slice()].concat();
    let in_5 = [&input_line[7..9], minus_2.as_slice()].concat();
    let in_6 = [&input_line[9..11], minus_2.as_slice()].concat();
    let in_7 = [&input_line[11..12], minus_1.as_slice()].concat();
    let in_8 = [&input_line[12..14], minus_2.as_slice()].concat();

    let input_stream = [
        in_1.as_slice(), 
        in_2.as_slice(), 
        in_3.as_slice(), 
        in_4.as_slice(), 
        in_5.as_slice(), 
        in_6.as_slice(), 
        in_7.as_slice(), 
        in_8.as_slice(), 
        ];
    
    return  input_stream;
}

//async fn read_until(serial: &mut CdcAcmClass<Driver<T>>, sep: u8) -> Result<[u8; 64], Disconnected> {
//    
//    let mut found_sep = false;
//    let mut out_buffer = [0u8; 64];
//    let mut in_buffer = [0u8; 64];
//    let mut out_idx: usize = 0;
//    let mut extra_vals
//    
//    while !found_sep {
//        let in_idx = serial.read_packet(&mut in_buffer).await?;
//        let in_window = &in_buffer[..in_idx];
//        
//        for i in in_window {
//            out_idx += 1;
//            if i == &sep {
//
//            }
//        }
//    }
//
//    unimplemented!()
//}