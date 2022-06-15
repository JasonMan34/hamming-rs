use bitvec::prelude::*;

fn fix_parity(bits: &mut BitVec, parity: usize) {
    let mut index = 1;
    while index <= parity {
        if index & parity != 0 {
            let value = bits[index];
            bits.set(index, !value);
        }

        index = index * 2;
    }
}

fn parity_check(bits: &BitVec) -> usize {
    bits.iter()
        .enumerate()
        .filter(|(_, bit)| **bit)
        .map(|(bit_index, _)| bit_index)
        .reduce(|bit_index_1, bit_index_2| bit_index_1 ^ bit_index_2)
        .unwrap_or(0)
}

// fn new_index_to_og_index(index: usize) -> usize {
//     index - ((index as f64).log2().ceil() as usize) - 1
// }

fn og_index_to_new_index(index: usize) -> usize {
    let index_list = [3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15];
    index_list[index]
}

pub fn encode(file: Vec<u8>, final_chunk_size: usize) -> Vec<u8> {
    if (final_chunk_size as f64).log2().fract() != 0.0 {
        panic!("final_chunk_size must be a power of 2");
    }

    let chunk_size = ((final_chunk_size as f64) - (final_chunk_size as f64).log2() - 1.0) as usize;

    let chunks = file.as_bits::<Lsb0>().chunks(chunk_size);
    let chunks_count = chunks.clone().count();
    let mut encoded_file: Vec<u8> = Vec::with_capacity(chunks_count * (final_chunk_size / 8));

    for (_, chunk) in chunks.enumerate() {
        let mut new_chunk: BitVec<usize, Lsb0> = BitVec::with_capacity(final_chunk_size);
        unsafe { new_chunk.set_len(final_chunk_size) }
        new_chunk.fill(false);

        for (bit_index, bit) in chunk.iter().enumerate() {
            new_chunk.set(og_index_to_new_index(bit_index), *bit);
        }

        let parity = parity_check(&new_chunk);
        fix_parity(&mut new_chunk, parity);

        for byte in new_chunk.chunks(8) {
            encoded_file.push(byte.load());
        }
    }

    encoded_file
}

pub fn encode_7_4(file: Vec<u8>) -> Vec<u8> {
    encode(file, 8)
}

pub fn run(file_path: String, output_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let og_file = std::fs::read_to_string(file_path)?;
    let encoded_file = encode_7_4(og_file.into_bytes());

    std::fs::write(output_file_path, encoded_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::og_index_to_new_index;

    #[test]
    fn test_og_index_to_new_index() {
        assert_eq!(og_index_to_new_index(0), 3);
        assert_eq!(og_index_to_new_index(1), 5);
        assert_eq!(og_index_to_new_index(2), 6);
        assert_eq!(og_index_to_new_index(3), 7);
        assert_eq!(og_index_to_new_index(4), 9);
        assert_eq!(og_index_to_new_index(5), 10);
    }
}
