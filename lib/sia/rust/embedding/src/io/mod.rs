mod steganography;

use crate::io::steganography::{
    WatermarkFromDwtCoefficientsReader, WatermarkToDwtCoefficientsWriter,
};
use crate::utils::slice2d_as_nd_array;
use bitvec::bitvec;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use ndarray::{Array2, ArrayBase, ArrayViewMut, Ix2, ViewRepr};
use std::io::BufWriter;

pub fn write<'a>(
    domain: &'a mut ArrayBase<ViewRepr<&'a mut f64>, Ix2>,
    bytes: Vec<u8>,
    depth: f64,
) {
    let mut writer = WatermarkToDwtCoefficientsWriter::new(domain, depth);
    writer.write(bytes);
    writer.close();
}

pub fn read<'a>(domain: &'a ArrayBase<ViewRepr<&'a mut f64>, Ix2>, depth: f64) -> Vec<u8> {
    let mut reader = WatermarkFromDwtCoefficientsReader::new(domain, depth);
    reader.read()
}

#[test]
fn read_test() {
    let mut domain = slice2d_as_nd_array(vec![
        vec![873.5000, 906.5000, 1104.875, 944.6250],
        vec![976.7500, 925.5000, 1032.125, 1078.250],
        vec![1028.125, 1057.000, 1051.875, 998.6250],
        vec![985.1250, 978.6250, 991.1250, 1023.750],
    ]);
    let depth = 10.0;

    println!("original domain: {:?}", domain);
    let bytes = vec![123];
    write(&mut domain.view_mut(), bytes, depth);
    println!("domain with byte written: {:?}", domain);

    let read = read(&domain.view_mut(), depth);
    println!("read bytes: {:?}", read);
}
