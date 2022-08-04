use ndarray::{ArrayBase, Ix2, ViewRepr};

use crate::get_test_matrix;
use crate::io::steganography::{
    WatermarkFromDwtCoefficientsReader, WatermarkToDwtCoefficientsWriter,
};
use crate::utils::slice2d_as_nd_array;

mod steganography;

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
fn io_write_and_read_test_with_1_byte_capacity() {
    let mut domain = slice2d_as_nd_array(vec![
        vec![873.5000, 906.5000, 1104.875, 944.6250],
        vec![976.7500, 925.5000, 1032.125, 1078.250],
        vec![1028.125, 1057.000, 1051.875, 998.6250],
        vec![985.1250, 978.6250, 991.1250, 1023.750],
    ]);
    let depth = 10.0;

    println!("original domain: {:?}", domain);
    let bytes = vec![17];
    write(&mut domain.view_mut(), bytes.clone(), depth);
    println!("domain with byte written: {:?}", domain);

    let read = read(&domain.view_mut(), depth);
    println!("read bytes: {:?}", read.clone());
    assert_eq!(bytes, read);
}

#[test]
fn io_write_and_read_test_with_many_bytes_capacity() {
    let mut domain = slice2d_as_nd_array(get_test_matrix());
    let depth = 10.0;

    println!("original domain: {:?}", domain);
    let bytes = vec![231, 73];
    write(&mut domain.view_mut(), bytes.clone(), depth);
    println!("domain with byte written: {:?}", domain);

    let read = read(&domain.view_mut(), depth);
    println!("read bytes: {:?}", read.clone());
    assert_eq!(bytes, read);
}
