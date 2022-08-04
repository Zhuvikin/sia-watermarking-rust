use bitvec::prelude::*;
use ndarray::{Array2, ArrayBase, AssignElem, Ix2, ViewRepr};
use std::os::macos::raw::stat;

const NULL: u8 = 0xFF;

pub struct WatermarkToDwtCoefficientsWriter<'a> {
    offset_column: usize,
    offset_row: usize,
    domain: &'a mut ArrayBase<ViewRepr<&'a mut f64>, Ix2>,
    depth: f64,
}

pub struct WatermarkFromDwtCoefficientsReader<'a> {
    offset_column: usize,
    offset_row: usize,
    domain: &'a ArrayBase<ViewRepr<&'a mut f64>, Ix2>,
    depth: f64,
}

fn get_capacity(width: usize, height: usize) -> usize {
    ((width * height) as f64 / 8.).floor() as usize
}

impl<'a> WatermarkToDwtCoefficientsWriter<'a> {
    pub fn new(domain: &'a mut ArrayBase<ViewRepr<&'a mut f64>, Ix2>, depth: f64) -> Self {
        Self {
            offset_column: 0,
            offset_row: 0,
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

        let bit_vec: BitVec<u8, Lsb0> = BitVec::from_vec(bytes);
        bit_vec.iter().for_each(|bit| self.write_bit(*bit));
    }

    fn write_byte(&mut self, byte: u8) {
        self.write(vec![byte]);
    }

    fn write_bit(&mut self, bit: bool) {
        let reminder = if bit { 0.25 } else { -0.25 };

        let coefficient = self
            .domain
            .get_mut(Ix2(self.offset_row, self.offset_column))
            .unwrap(); // TODO: check availability

        let original_coefficient = coefficient.clone();

        coefficient
            .assign_elem(self.depth * ((*coefficient / self.depth).round() as f64 + reminder));

        println!(
            "write ({:?}) to ({:?}, {:?}) : {:?} -> {:?}",
            if bit { 1 } else { 0 },
            self.offset_column,
            self.offset_row,
            original_coefficient,
            coefficient
        );

        self.increment_offset();
    }

    fn increment_offset(&mut self) {
        let (width, _) = self.domain.dim();
        self.offset_column += 1;
        if self.offset_column == width {
            self.offset_row += 1;
            self.offset_column = 0;
        }
    }

    pub fn close(&mut self) {
        let (width, height) = self.domain.dim();
        if self.offset_row >= height {
            println!("full capacity is used");
            return;
        }

        println!("write NULL");
        self.write_byte(NULL);
        let bits_written = self.offset_row * width + self.offset_column;
        println!("{:?} bits written", bits_written)
    }
}

impl<'a> WatermarkFromDwtCoefficientsReader<'a> {
    pub fn new(domain: &'a ArrayBase<ViewRepr<&'a mut f64>, Ix2>, depth: f64) -> Self {
        Self {
            offset_column: 0,
            offset_row: 0,
            domain,
            depth,
        }
    }

    pub fn read(&mut self) -> Vec<u8> {
        let (width, height) = self.domain.dim();
        let capacity = get_capacity(width, height);

        let mut extracted = vec![];
        loop {
            if extracted.len() == capacity {
                println!("full capacity read");
                break;
            }
            let byte = self.read_next();
            if byte == NULL {
                println!("NULL read");
                break;
            }
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
        let (width, _) = self.domain.dim();
        let coefficient_option = self.domain.get(Ix2(self.offset_row, self.offset_column));
        match coefficient_option {
            Some(coefficient) => {
                self.increment_offset();
                if coefficient - self.depth * (coefficient / self.depth).round() >= 0 as f64 {
                    true
                } else {
                    false
                }
            }
            None => {
                panic!(
                    "offset ({:?}, {:?}) is out of bounds",
                    self.offset_column, self.offset_row
                )
            }
        }
    }

    fn increment_offset(&mut self) {
        let (width, _) = self.domain.dim();
        self.offset_column += 1;
        if self.offset_column == width {
            self.offset_row += 1;
            self.offset_column = 0;
        }
    }
}
