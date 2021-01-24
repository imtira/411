/*
 * Copyright (C) 2020 github.com/t1ra
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod error;
use error::Too;

mod crc;

use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let args_len = match args.size_hint().1 {
        Some(len) => len,
        None => return Err(Box::new(error::ArgsError { reason: Too::Many })),
    };

    if args_len == 1 {
        return Err(Box::new(error::ArgsError { reason: Too::Few }));
    }

    for file in args.skip(1) {
        let output_file = {
            let file_name = file
                .split('.')
                .filter(|chunk| *chunk != "ppm")
                .collect::<String>()
                + ".png";

            File::create(Path::new(&file_name))?
        };

        let ref mut w = std::io::BufWriter::new(output_file);
        let mut encoder = png::Encoder::new(w, 64, 48);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let mut png_img: Vec<u8> = Vec::new();

        for yuv_chunk in fs::read(Path::new(&file))?
            .chunks(6)
            .collect::<Vec<&[u8]>>()
        {
            png_img.extend(yuv_to_rgb(yuv_chunk));
        }

        writer.write_image_data(&png_img)?;
    }

    Ok(())
}

fn yuv_to_rgb<'a>(chunk: &'a [u8]) -> Vec<u8> {
    let d = chunk[4] - 128;
    let e = chunk[5] - 128;

    let mut out: Vec<u8> = Vec::new();

    for y in &chunk[0..3] {
        let c = y - 16;
        // R
        out.push((((298i32 * c as i32) + (409i32 * e as i32) + 128) >> 8) as u8);
        // G
        out.push((((298i32 * c as i32) - (100i32 * d as i32) - (208i32 * e as i32) + 128) >> 8) as u8);
        // B
        out.push((((298i32 * c as i32) + (516i32 * d as i32) + 128) >> 8) as u8);
    };

    out
}

/*
enum ChunkType {
    IHDR,
    IDAT,
    IEND,
}

trait WriteChunk {
    fn write_chunk(&mut self, chunk_type: ChunkType, data: &[u8]) -> Result<(), Box<dyn Error>>;
}

impl WriteChunk for File {
    fn write_chunk(&mut self, chunk_type: ChunkType, data: &[u8]) -> Result<(), Box<dyn Error>> {
        match chunk_type {
            // An IHDR chunk directly follows the magic.
            ChunkType::IHDR => {
                self.write(&[
                    // The chunk length, only counting the chunk data, not the rest of the chunk.
                    13,
                    // The chunk type. Here it's "IHDR". This is written as ascii characters.
                    73, 72, 68, 82,

                    // The chunk data.
                    // 4 bytes for the width
                    0, 0, 0, 64,
                    // 4 bytes for the height
                    0, 0, 0, 32,
                    // 1 byte for bit depth
                    16,
                    // 1 byte for color type
                    // Here it's 2 (color used).
                    2,
                    // 1 byte for compression method
                    // 0 is the only supported value, deflate/inflate + 32k sliding window.
                    0,
                    // 1 byte  for the filter method
                    // 0 is the only supported value, adaptive filtering with five filter types.
                    0,
                    // and 1 byte for the interlace method
                    0,

                    // chunk CRC (Cyclic Redundancy Check).
                    // It includes the type and data, but not length.
                    // Since this chunk is all static, we can hardcode this.
                    125, 111, 53, 144,
                ])?;
            }
            ChunkType::IDAT => {

            },
            ChunkType::IEND => {
                self.write(&[
                    // chunk length
                    0,
                    // chunk type (IEND)
                    73, 69, 78, 68,
                    // chunk data. IEND has none.
                    // chunk CRC
                    174, 66, 96, 130,
                ])?;
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let args_len = match args.size_hint().1 {
        Some(len) => len,
        None => return Err(Box::new(error::ArgsError { reason: Too::Many })),
    };

    if args_len == 1 {
        return Err(Box::new(error::ArgsError { reason: Too::Few }));
    }

    for file in args.skip(1) {
        let file_name = file
            .split('.')
            .filter(|chunk| *chunk != "ppm")
            .collect::<String>() + ".png";
        let mut png_output = File::create(file_name)?;
        // Write the PNG magic
        png_output.write(&[137, 80, 78, 71, 13, 10, 26, 10])?;

        // Write the IHDR header
        png_output.write_chunk(ChunkType::IHDR, &[])?;

        // Write chunk data.

        // Write the IEND header.
        png_output.write_chunk(ChunkType::IEND, &[])?;
    }

    Ok(())
}
*/
