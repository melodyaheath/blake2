use rfc7693::blake2b::hash;

fn main() {
    let mut message: Vec<u64> = Vec::new();
    
    message.push(0x00636261);
    for _ in 1..16 {
        message.push(0x00000000);
    }
    println!("m: {:X?}", message);

    let v: Vec<u64> = hash( &message, 3, 0 , 64);

    for x in v.iter() {
        for y in x.to_le_bytes().iter()  {
            print!("{:02X?} ", y);
        }
    }

    println!("\nThe message for '':");


    message = Vec::new();
    for _ in 0..16 {
        message.push(0x00000000);
    }

    let v: Vec<u64> = hash( &message, 0, 0 , 64);

    for x in v.iter() {
        for y in x.to_le_bytes().iter()  {
            print!("{:02X?} ", y);
        }
    }

}