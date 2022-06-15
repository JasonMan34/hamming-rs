use bitvec::field::BitField;
use bitvec::prelude::AsBits;
use bitvec::prelude::BitVec;
use bitvec::prelude::Lsb0;
use shared::parity_check_u8;

fn fix_parity(bits: &mut BitVec<u8>, parity: usize) {
    if parity != 0 {
        let value = bits[parity];
        bits.set(parity, !value);
    }
}

fn new_index_to_og_index(index: usize) -> usize {
    println!("index is {}", index);
    index - ((index as f64).log2().ceil() as usize) - 1
}

pub fn decode(file: Vec<u8>) -> Vec<u8> {
    let padded_bits_count = file[7];
    let chunk_size = file[8] as usize;
    let file = &file[9..];

    let decoded_chunk_size = ((chunk_size as f64) - (chunk_size as f64).log2() - 1.0) as usize;

    let chunks = file.as_bits::<Lsb0>().chunks(chunk_size);
    let chunks_count = chunks.clone().count();
    let mut decoded_bitvec: BitVec<usize, Lsb0> =
        BitVec::with_capacity(chunks_count * decoded_chunk_size / 8 - (padded_bits_count as usize));

    for (chunk_index, mut chunk) in chunks.enumerate() {
        println!("Chunk #{} is: {}", chunk_index + 1, chunk);
        let mut chunk_vec = BitVec::from(chunk);
        let parity = parity_check_u8(&chunk_vec);
        fix_parity(&mut chunk_vec, parity);

        let mut decoded_chunk: BitVec<usize, Lsb0> = BitVec::with_capacity(decoded_chunk_size);

        for (bit_index, bit) in chunk.iter().enumerate().skip(3) {
            if (bit_index as f64).log2().fract() != 0.0 {
                decoded_chunk.push(*bit);
            }
        }

        for bit in decoded_chunk.iter() {
            decoded_bitvec.push(*bit);
        }
    }

    let mut decoded_file: Vec<u8> = Vec::with_capacity(chunks_count * decoded_chunk_size / 8);
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
