use byteorder::{NativeEndian, ReadBytesExt};
use gl;
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::raw::c_void;
use std::path::PathBuf;

fn image_load(filename: &PathBuf) -> Result<image::DynamicImage, std::io::Error> {
    Ok(image::open(filename).expect(format!("Failed to load texture {filename:?}").as_str()))
}

fn load_raw(filename: &PathBuf) -> Result<image::DynamicImage, std::io::Error> {
    let mut buffer = BufReader::new(
        File::open(filename).expect(format!("Failed to load raw texture {filename:?}").as_str()),
    );

    let width = buffer.read_u32::<NativeEndian>()?;
    let height = buffer.read_u32::<NativeEndian>()?;
    let depth = buffer.read_i32::<NativeEndian>()?;

    println!("Got {width} {height} {depth}");

    let dyn_image = match depth {
        3 => {
            let mut img = image::RgbImage::new(width, height);

            buffer.read_exact(img.as_mut())?;

            image::DynamicImage::ImageRgb8(img)
        }
        4 => {
            let mut img = image::RgbaImage::new(width, height);

            buffer.read_exact(img.as_mut())?;

            image::DynamicImage::ImageRgba8(img)
        }
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")),
    };

    Ok(dyn_image)
}

fn load_texture(
    img: &image::DynamicImage,
    wrap: bool,
    _filter: u32,
) -> Result<u32, std::io::Error> {
    let data = img.as_bytes();
    let mut texture = 0;
    let (format, pxtype) = match img.color() {
        image::ColorType::Rgb8 => (gl::RGB, gl::UNSIGNED_BYTE),
        image::ColorType::Rgba8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        image::ColorType::Rgb16 => (gl::RGB, gl::UNSIGNED_SHORT),
        image::ColorType::Rgba16 => (gl::RGBA, gl::UNSIGNED_SHORT),
        _ => {
            panic!("unknown bullshit!");
        }
    };
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        if wrap {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        } else {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            pxtype,
            data.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    Ok(texture)
}

fn load_sprite(img: &image::DynamicImage) -> Result<(u32, u32, u32), std::io::Error> {
    let data = img.as_bytes();
    let mut texture = 0;
    let (format, pxtype) = match img.color() {
        image::ColorType::Rgb8 => (gl::RGB, gl::UNSIGNED_BYTE),
        image::ColorType::Rgba8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        image::ColorType::Rgb16 => (gl::RGB, gl::UNSIGNED_SHORT),
        image::ColorType::Rgba16 => (gl::RGBA, gl::UNSIGNED_SHORT),
        _ => {
            panic!("unknown bullshit!");
        }
    };
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            pxtype,
            data.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Ok((texture, img.width(), img.height()))
}

pub fn get_texture(filename: &PathBuf, wrap: bool, filter: u32) -> Result<u32, std::io::Error> {
    let rgba = image_load(filename)?;

    return load_texture(&rgba, wrap, filter);
}

pub fn get_sprite(filename: &PathBuf) -> Result<(u32, u32, u32), std::io::Error> {
    let rgba = if filename.extension().unwrap() == "raw" {
        load_raw(filename)
    } else {
        image_load(filename)
    }?;

    return load_sprite(&rgba);
}
