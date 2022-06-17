use bitvec::field::BitField;
use bitvec::prelude::AsBits;
use bitvec::prelude::BitVec;
use bitvec::prelude::Lsb0;
use shared::parity_check;

fn fix_parity(bits: &mut BitVec<u8>, parity: usize) {
    if parity != 0 {
        let value = bits[parity];
        bits.set(parity, !value);
    }
}

pub fn decode(file: Vec<u8>) -> Vec<u8> {
    let last_chunk_bitcount = file[7];
    let chunk_size = file[8];
    let file = &file[9..];

    let decoded_chunk_size = ((chunk_size as f64) - (chunk_size as f64).log2() - 1.0) as usize;

    let chunks = file.as_bits::<Lsb0>().chunks(chunk_size as usize);
    let chunks_count = chunks.clone().count();
    let mut decoded_bitvec: BitVec<u8, Lsb0> = BitVec::with_capacity(
        ((chunks_count - 1) * decoded_chunk_size + (last_chunk_bitcount as usize)) / 8,
    );

    for (chunk_index, chunk) in chunks.enumerate() {
        println!("Chunk #{} is: {}", chunk_index + 1, chunk);
        let mut chunk_vec = BitVec::from(chunk);
        let parity = parity_check(&chunk_vec);
        fix_parity(&mut chunk_vec, parity);

        for (bit_index, bit) in chunk.iter().enumerate().skip(3) {
            if (bit_index as f64).log2().fract() != 0.0 {
                decoded_bitvec.push(*bit);
            }
        }
    }

    for _ in 0..last_chunk_bitcount {
        decoded_bitvec.pop();
    }

    let mut decoded_file: Vec<u8> = Vec::with_capacity(decoded_bitvec.capacity());
    for byte in decoded_bitvec.chunks(8) {
        decoded_file.push(byte.load());
    }

    decoded_file
}

pub fn run(file_in: String, file_out: String) -> Result<(), Box<dyn std::error::Error>> {
    let og_file = std::fs::read(file_in)?;
    // let encoded_file = encode_7_4(og_file.into_bytes());
    let decoded_file = decode(og_file);

    std::fs::write(file_out, decoded_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
