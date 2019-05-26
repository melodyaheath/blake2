pub static INDEX_FOR_MIX: &'static [u32] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3,
    11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4,
    7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8,
    9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13,
    2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9,
    12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11,
    13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10,
    6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5,
    10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0,
];


const BLAKE2S_IV:  [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
    0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19
];

fn mix(local_vector: &mut Vec<u32>, a: usize, b: usize, c: usize, d: usize, x:  u32, y: u32){
    let (rotate_a, rotate_b, rotate_c, rotate_d) = (16, 12,  8,  7);

    wrapping_add!(local_vector[a], local_vector[a] , local_vector[b] , x);
    local_vector[d] =  (local_vector[d] ^ local_vector[a] ).rotate_right(rotate_a);

    wrapping_add!(local_vector[c], local_vector[c] , local_vector[d] , 0);
    local_vector[b] =  (local_vector[b] ^ local_vector[c]).rotate_right(rotate_b);

    wrapping_add!(local_vector[a], local_vector[a] , local_vector[b] , y);
    local_vector[d] =  (local_vector[d] ^ local_vector[a] ).rotate_right(rotate_c);

    wrapping_add!(local_vector[c], local_vector[c] , local_vector[d] , 0);
    local_vector[b] =  (local_vector[b] ^ local_vector[c]).rotate_right(rotate_d);
}

fn compression(state_vector:&mut Vec<u32> , message: &Vec<u32>, t: u64, is_final_block: bool) {
    let mut local_vector: Vec<u32> = Vec::new();
    local_vector.clone_from(state_vector);
    local_vector.extend_from_slice(&BLAKE2S_IV[..]);

    local_vector[12] ^= t as u32;
    local_vector[13] ^= 0;

    if is_final_block {
        local_vector[14] = !local_vector[14] ;
    }

    for i in 0..10  {

        let index: Vec<u32> = (INDEX_FOR_MIX[(i * 16 .. i*16 + 16)]).to_vec();

        mix(&mut local_vector, 0, 4, 8, 12,  message[index[ 0] as usize], message[index[ 1] as usize]);
        mix(&mut local_vector, 1, 5,  9, 13, message[index[ 2] as usize], message[index[ 3] as usize] );
        mix(&mut local_vector, 2, 6, 10, 14, message[index[ 4] as usize], message[index[ 5] as usize] );
        mix(&mut local_vector, 3, 7, 11, 15, message[index[ 6] as usize], message[index[ 7] as usize] );
        mix(&mut local_vector, 0, 5, 10, 15, message[index[ 8] as usize], message[index[ 9] as usize] );
        mix(&mut local_vector, 1, 6, 11, 12, message[index[10] as usize], message[index[11] as usize] ); 
        mix(&mut local_vector, 2, 7,  8, 13, message[index[12] as usize], message[index[13] as usize] );
        mix(&mut local_vector, 3, 4,  9, 14, message[index[14] as usize], message[index[15] as usize] );
    }

    for i in 0..8 {
        state_vector[i] = state_vector[i] ^ local_vector[i] ^ local_vector[i + 8];
    }
}

pub fn hash( message: &Vec<u32>, offset: usize, kk: u32, nn: u32) -> Vec<u32>{
    let mut state_vector: Vec<u32> = Vec::new();
    state_vector.extend_from_slice(&BLAKE2S_IV[..]);

    state_vector[0] = state_vector[0] ^ 0x01010000 ^ (kk << 8) ^ nn;

    if kk == 0 {
        compression(&mut state_vector, &message, offset as u64, true);
    }
    else {
        compression(&mut state_vector, &message, offset as u64 + 32, true);
    }
    return state_vector;
}