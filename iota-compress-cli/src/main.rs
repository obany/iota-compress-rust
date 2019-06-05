extern crate iota_compress;
extern crate rand;
extern crate time;
extern crate std;

use iota_compress::compress;
use iota_compress::decompress;
use rand::Rng;
use time::PreciseTime;
use std::str;

const PACKET_SIZE: usize = 2673;

fn main() {
    let num_iterations: u32 = 100;
    let mut total_compress: i64 = 0;
    let mut total_decompress: i64 = 0;
    let mut total_compressed_length: u32 = 0;

    for i in 0..num_iterations {
        //let trytes = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
        //let trytes = "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";
        //let trytes = generate_random();

        let trytes_buffer = generate_random();

        let start_compress = PreciseTime::now();
        let compressed = compress(&trytes_buffer);
        let compress_duration = start_compress.to(PreciseTime::now()).num_nanoseconds().unwrap_or(0);

        let compressed_len = compressed.len() as u32;

        total_compress += compress_duration;
        total_compressed_length += compressed_len;

        let start_decompress = PreciseTime::now();
        let decompressed = decompress(&compressed);
        let decompress_duration = start_decompress.to(PreciseTime::now()).num_nanoseconds().unwrap_or(0);

        total_decompress += decompress_duration;

        println!("{} Compress {} ns, Decompress {} ns, Compressed Size {}", i, compress_duration, decompress_duration, compressed_len);

        println!("matches {}", str::from_utf8(&trytes_buffer).unwrap() == str::from_utf8(&decompressed).unwrap());
    }

    println!("Average Compress {} ns, Average Decompress {} ns, Average Compressed Size {} bytes", 
      total_compress / i64::from(num_iterations),
      total_decompress / i64::from(num_iterations),
      total_compressed_length / num_iterations
    );
}

fn generate_random() -> Vec<u8> {
    let tryte_alphabet: Vec<u8> = vec![57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90];

    let mut output: Vec<u8> = Vec::new();

    for _i in 0..PACKET_SIZE {
        let num = rand::thread_rng().gen_range(0, tryte_alphabet.len());
        output.push(tryte_alphabet[num]);
    }

    output
}