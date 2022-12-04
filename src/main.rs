mod args;
use args::Args;
use std::{io::BufReader, fs::File, convert::TryInto, vec};
use image::{io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::FilterType::Triangle};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height:u32, name: String) -> Self {
        let buffer_capacity =  height*width*4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors>{
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors>{
    let args = Args::new();
    let (image_1, im_format_1) = find_iamge_from_path(args.image_1);
    let (image_2, im_format_2) = find_iamge_from_path(args.image_2);
    
    if im_format_1 != im_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output_image = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);
    output_image.set_data(combined_data)?;

    image::save_buffer_with_format(output_image.name, &output_image.data, output_image.width, output_image.height, image::ColorType::Rgba8, im_format_1).unwrap();
    Ok(())
}

fn find_iamge_from_path(path: String) -> (DynamicImage, ImageFormat){
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format:ImageFormat  = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimesnion(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 {dim_1} else {dim_2};
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage){
    let (width, height) = get_smallest_dimesnion(image_1.dimensions(), image_2.dimensions());
    print!("width: {}, height: {}\n",width, height);

    if image_2.dimensions() == (width, height){
        image_1.resize_exact(width, height, Triangle);
    }
    else {
        image_2.resize_exact(width, height, Triangle);
    }
    return (image_1, image_2);
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1 : Vec<u8>, vec_2 : Vec<u8>) -> Vec<u8> {
    // if vec_1.len() ==5, vec![0u8; vec_1.len()] = [0,0,0,0,0]
    let mut combined_data = vec![0u8; vec_1.len()];
    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i+3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i+3));
        }
        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8>{
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index is out of bounds")
        };
        rgba.push(val);
    }
    rgba
}