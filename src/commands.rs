use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::args;
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

pub fn read_png(path: &PathBuf) -> Result<Png, &'static str> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(..) => return Err("Open file error"),
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    Png::try_from(buffer.as_ref())
}

pub fn write_png(path: &PathBuf, png: Png) -> Result<(), ()> {
    let mut file = match fs::File::create(path) {
        Ok(file) => file,
        Err(..) => return Err(()),
    };
    file.write_all(png.as_bytes().as_ref()).unwrap();
    Ok(())
}

pub fn encode(subopt: &args::EncodeStruct) {
    println!("Encoding {}", subopt.path.to_str().unwrap());
    let mut png_image = read_png(&subopt.path).unwrap();
    let chunk_type = match ChunkType::from_str(&subopt.chunk_type){
        Ok(chunk_type) => chunk_type,
        Err(..) => {
            println!("Please use a valid chunk type");
            println!("Note: the third character must be a capital letter");
            panic!("Invalid chunk type");
        },
    };
    let data: Vec<u8> = subopt.message
        .bytes()
        .collect();
    let secret_chunk = Chunk::new(chunk_type, data);
    png_image.append_chunk(secret_chunk);
    println!("Inserted secret message");
    let default = PathBuf::from_str("output.png").unwrap();
    let destination = match &subopt.dest {
        Some(path) => path,
        None => &default,
    };
    write_png(&destination, png_image).unwrap();
}

pub fn decode(subopt: &args::DecodeStruct) {
    println!("Decoding {}", subopt.path.to_str().unwrap());
    let png_image = read_png(&subopt.path).unwrap();
    if let Some(target_chunk) = png_image.chunk_by_type(&subopt.chunk_type) {
        println!("The message is: {}", target_chunk);
    } else {
        println!("No hidden message found for the chunk type {}", subopt.chunk_type);
    }
}

pub fn remove(subopt: &args::RemoveStruct) {
    println!("Removing a chunk type {} in {}", subopt.chunk_type, subopt.path.to_str().unwrap());
    let mut png_image = read_png(&subopt.path).unwrap();
    png_image.remove_chunk(&subopt.chunk_type).unwrap();
    write_png(&subopt.path, png_image).unwrap();
}

pub fn print(subopt: &args::PrintStruct) {
    let png_image = read_png(&subopt.path).unwrap();
    println!("{}", png_image)
}
