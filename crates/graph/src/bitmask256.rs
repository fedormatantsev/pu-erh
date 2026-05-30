/// 256-bit occupancy mask for radix trie inner nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitmask256([u64; 4]);

impl Bitmask256 {
    pub const EMPTY: Self = Self([0; 4]);

    #[inline]
    pub fn test(&self, index: u8) -> bool {
        let word = (index as usize) >> 6;
        let bit = (index as u64) & 63;
        (self.0[word] >> bit) & 1 == 1
    }

    #[inline]
    pub fn set(&mut self, index: u8) {
        let word = (index as usize) >> 6;
        let bit = (index as u64) & 63;
        self.0[word] |= 1 << bit;
    }

    #[inline]
    pub fn clear(&mut self, index: u8) {
        let word = (index as usize) >> 6;
        let bit = (index as u64) & 63;
        self.0[word] &= !(1 << bit);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == [0; 4]
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.0.iter().map(|word| word.count_ones() as usize).sum()
    }

    /// Number of set bits strictly below `index`.
    #[inline]
    pub fn rank(&self, index: u8) -> usize {
        let word = (index as usize) >> 6;
        let bit = (index as u64) & 63;
        let low_mask = if bit == 0 {
            0
        } else {
            (1u64 << bit) - 1
        };
        let mut count = (self.0[word] & low_mask).count_ones() as usize;
        for w in &self.0[..word] {
            count += w.count_ones() as usize;
        }
        count
    }

    /// Highest set bit index, if any.
    pub fn last_set(&self) -> Option<u8> {
        for word_idx in (0..4).rev() {
            let word = self.0[word_idx];
            if word != 0 {
                let bit = 63 - word.leading_zeros();
                return Some((word_idx << 6) as u8 + bit as u8);
            }
        }
        None
    }

    pub fn iter_set_bits(&self) -> impl Iterator<Item = u8> + '_ {
        (0u8..=255).filter(|&index| self.test(index))
    }

    pub fn iter_set_bits_rev(&self) -> impl DoubleEndedIterator<Item = u8> + '_ {
        (0u8..=255).rev().filter(|&index| self.test(index))
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmask256;

    #[test]
    fn rank_and_last_set() {
        let mut mask = Bitmask256::EMPTY;
        mask.set(0);
        mask.set(255);
        assert!(mask.test(0));
        assert!(mask.test(255));
        assert!(!mask.test(1));
        assert_eq!(mask.count(), 2);
        assert_eq!(mask.rank(0), 0);
        assert_eq!(mask.rank(255), 1);
        assert_eq!(mask.last_set(), Some(255));
    }
}
