use bitvec::prelude::BitVec;

pub fn parity_check(bits: &BitVec) -> usize {
    bits.iter()
        .enumerate()
        .filter(|(_, bit)| **bit)
        .map(|(bit_index, _)| bit_index)
        .reduce(|bit_index_1, bit_index_2| bit_index_1 ^ bit_index_2)
        .unwrap_or(0)
}

pub fn parity_check_u8(bits: &BitVec<u8>) -> usize {
    bits.iter()
        .enumerate()
        .filter(|(_, bit)| **bit)
        .map(|(bit_index, _)| bit_index)
        .reduce(|bit_index_1, bit_index_2| bit_index_1 ^ bit_index_2)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
