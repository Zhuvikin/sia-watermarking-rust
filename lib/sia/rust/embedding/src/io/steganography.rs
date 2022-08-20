use bitvec::prelude::*;
use ndarray::{ArrayBase, AssignElem, Ix2, ViewRepr};

const MAX_CAPACITY: u16 = 0xFFFF;

pub struct WatermarkToDwtCoefficientsWriter<'a> {
    offset: usize,
    domain: &'a mut ArrayBase<ViewRepr<&'a mut f64>, Ix2>,
    depth: f64,
}

pub struct WatermarkFromDwtCoefficientsReader<'a> {
    offset: usize,
    domain: &'a ArrayBase<ViewRepr<&'a mut f64>, Ix2>,
    depth: f64,
}

pub fn get_capacity(width: usize, height: usize) -> usize {
    ((width * height) as f64 / 8.).floor() as usize
}

fn get_column_and_row(offset: usize, width: usize, height: usize) -> (usize, usize) {
    let column = offset % width;
    let row = (offset as f64 / height as f64).floor() as usize;
    (column, row)
}

impl<'a> WatermarkToDwtCoefficientsWriter<'a> {
    pub fn new(domain: &'a mut ArrayBase<ViewRepr<&'a mut f64>, Ix2>, depth: f64) -> Self {
        Self {
            offset: 0,
            domain,
            depth,
        }
    }

    pub fn write(&mut self, bytes: Vec<u8>) {
        let (width, height) = self.domain.dim();
        let capacity = get_capacity(width, height);
        if capacity < bytes.len() {
            panic!(
                "capacity of {:?} bits is less than data to be embedded ({:?} bits)",
                capacity * 8,
                bytes.len() * 8
            )
        }

        // write data to subsequent bytes
        let bit_vec: BitVec<u8, Lsb0> = BitVec::from_vec(bytes);
        bit_vec.iter().for_each(|bit| self.write_bit(*bit));
    }

    fn write_byte_at(&mut self, byte: u8, offset: usize) {
        self.offset = offset;
        self.write(vec![byte]);
    }

    fn write_bit(&mut self, bit: bool) {
        let reminder = if bit { 0.25 } else { -0.25 };
        let (width, height) = self.domain.dim();
        let (column, row) = get_column_and_row(self.offset, width, height);

        let coefficient = self.domain.get_mut(Ix2(row, column)).unwrap();
        //let original_coefficient = coefficient.clone();

        coefficient
            .assign_elem(self.depth * ((*coefficient / self.depth).round() as f64 + reminder));

        //println!(
        //    "write ({:?}) to ({:?}, {:?}) : {:?} -> {:?}",
        //    if bit { 1 } else { 0 },
        //    column,
        //    row,
        //    original_coefficient,
        //    coefficient
        //);

        self.offset += 1;
    }

    pub fn close(&mut self) {
        //let bytes_written: u16 = (self.offset as f64 / 8.0).ceil() as u8 - 1;
        //println!("{:?} bytes are written", bytes_written);
        //
        //if bytes_written  > MAX_CAPACITY {
        //    panic!(
        //        "data length exceeds maximum allowed capacity of {:?}",
        //        bytes_written
        //    )
        //}
        //
        //// write length of the data to first byte
        //self.write_byte_at(bytes_written, 0);
        //println!("data length header is written");
    }
}

impl<'a> WatermarkFromDwtCoefficientsReader<'a> {
    pub fn new(domain: &'a ArrayBase<ViewRepr<&'a mut f64>, Ix2>, depth: f64) -> Self {
        Self {
            offset: 0,
            domain,
            depth,
        }
    }

    pub fn read(&mut self) -> Vec<u8> {
        let (width, height) = self.domain.dim();
        let data_length = get_capacity(width, height);

        let mut extracted = vec![];
        loop {
            if extracted.len() >= data_length as usize {
                break;
            }
            let byte = self.read_next();
            extracted.push(byte);
        }
        extracted
    }

    fn read_next(&mut self) -> u8 {
        let mut bits = bitvec![];
        (0..8).for_each(|_| bits.push(self.read_bit()));
        let vec = bits.into_vec();
        vec[0] as u8
    }

    fn read_bit(&mut self) -> bool {
        let (width, height) = self.domain.dim();
        let (column, row) = get_column_and_row(self.offset, width, height);

        let coefficient_option = self.domain.get(Ix2(row, column));
        match coefficient_option {
            Some(coefficient) => {
                self.offset += 1;
                if coefficient - self.depth * (coefficient / self.depth).round() >= 0 as f64 {
                    true
                } else {
                    false
                }
            }
            None => {
                panic!("offset ({:?}, {:?}) is out of bounds", column, row)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_capacity_test() {
        assert_eq!(32, get_capacity(16, 16));
        assert_eq!(72, get_capacity(24, 24));
        assert_eq!(512, get_capacity(64, 64));
    }
}
