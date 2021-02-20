use bitvector::BitVector;

#[derive(Copy, Clone, Debug)]
pub enum Trit {
    ZERO, ONE, TWO,
}

#[derive(Debug, Clone)]
pub struct TritVector {
    bitvector: BitVector,
    trit_len: usize,
}

impl TritVector {
    pub fn new(trits: usize) -> Self {
        let trit_len = trits;
        let bitvector = BitVector::new(trit_len * 2);
        Self {bitvector, trit_len}
    }
    
    pub fn get(&self, trit_index: usize) -> Trit {
        assert!(trit_index < self.trit_len);
        let bit_index = 2 * trit_index;

        match (self.bitvector.contains(bit_index), self.bitvector.contains(bit_index + 1)) {
            (false, false) => Trit::ZERO,
            (false, true) => Trit::ONE,
            (true, false) => Trit::TWO,
            (true, true) => panic!("Found illegal trit"),
        }
    }

    pub fn set(&mut self, trit_index: usize, trit: Trit) {
        assert!(trit_index < self.trit_len);
        let bit_index = 2 * trit_index;

        match trit {
            Trit::ZERO => {self.bitvector.remove(bit_index); self.bitvector.remove(bit_index + 1);}
            Trit::ONE => {self.bitvector.remove(bit_index); self.bitvector.insert(bit_index + 1);}
            Trit::TWO => {self.bitvector.insert(bit_index); self.bitvector.remove(bit_index + 1);}
        }
    }
}

impl PartialEq for TritVector {
    fn eq(&self, other: &TritVector) -> bool {
        if self.trit_len != other.trit_len {
            return false;
        }

        for i in 0..self.trit_len * 2 {
            if self.bitvector.contains(i) != other.bitvector.contains(i) {
                return false;
            }
        }

        return true;
    }
}

impl Eq for TritVector {}