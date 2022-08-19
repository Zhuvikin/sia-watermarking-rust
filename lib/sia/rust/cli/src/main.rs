extern crate getopts;

use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::{env, fs};

use getopts::Options;
use image::ImageFormat;

use crate::timer::Elapsed;
use watermarking;

mod timer;

fn process(inputs: &Vec<String>, suffix_option: Option<String>, output_option: Option<String>) {
    match suffix_option {
        Some(suffix) => {
            let mut output_folder_path = Path::new("");
            if output_option.is_some() {
                let output = output_option.as_ref().unwrap();
                output_folder_path = Path::new(output.as_str());
                if !output_folder_path.is_dir() && output_folder_path.exists() {
                    println!("{} is not a folder", output_folder_path.to_str().unwrap());
                    std::process::exit(1);
                }
                fs::create_dir_all(output_folder_path).unwrap();
            }

            for input in inputs {
                let timer = Instant::now();
                let input_image = image::open(input).unwrap();

                let channels_count = input_image.color().channel_count();
                let watermarked = watermarking::watermark(input_image, 100.0, 1.0, 11);

                let input_path = Path::new(input);
                let filename = input_path.file_stem().unwrap();
                let extension = input_path.extension().unwrap();

                let mut output_path = format!(
                    "{}-{}.{}",
                    filename.to_str().unwrap(),
                    suffix,
                    extension.to_str().unwrap()
                )
                .to_owned();
                if output_option.is_some() {
                    let output_folder = output_folder_path.canonicalize();
                    output_path = format!(
                        "{}/{}",
                        output_folder.unwrap().to_str().unwrap(),
                        output_path
                    );
                }

                let mut output_file = File::create(output_path.as_str()).unwrap();
                watermarked
                    .write_to(
                        &mut output_file,
                        ImageFormat::from_path(&output_path).unwrap(),
                    )
                    .unwrap();

                println!(
                    "Watermarked {} ({} -> {} channels) in {}",
                    output_path,
                    channels_count,
                    watermarked.color().channel_count(),
                    Elapsed::from(&timer)
                );
            }
        }
        None => println!("No output image"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("s", "", "set watermarked images filenames suffix", "SUFFIX");
    opts.optopt("o", "", "set watermarked images output folder", "OUT");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let suffix = matches.opt_str("s");
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free
    } else {
        print_usage(&program, opts);
        return;
    };
    process(&input, suffix, output);
}
