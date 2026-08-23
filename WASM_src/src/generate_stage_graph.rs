use crate::generate_stage::{GenerateStage};
use base64::prelude::*;
use std::io::Cursor;
use image::{DynamicImage, ImageOutputFormat};

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{LazyLock, Mutex};

static GRAPH_MAGIC: LazyLock<Mutex<HashMap<(i32, i32), String>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

impl GenerateStage
{
    pub fn graph_file_to_imag(txt: &str) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
    {
        match BASE64_STANDARD.decode(&txt)
        {
            Ok(bytes) => match image::load_from_memory(&bytes)
            {
                Ok(image) => image.to_rgba8(),
                Err(_) => image::ImageBuffer::new(0, 0),
            },
            Err(_) => image::ImageBuffer::new(0, 0),
        }
    }

    pub fn graph_imag_to_file(img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, default_txt: String) -> String
    {
        let mut buffer = Cursor::new(Vec::new());
        match DynamicImage::ImageRgba8(img).write_to(&mut buffer, ImageOutputFormat::Png)
        {
            Ok(_) => BASE64_STANDARD.encode(buffer.into_inner()),
            Err(_) => default_txt,
        }
    }

    pub fn graph_create_magic(width: i32, height: i32) -> String
    {
        let mut map = GRAPH_MAGIC.lock().unwrap();
        match map.entry((width, height))
        {
            Entry::Occupied(item) => {
                item.get().clone()
            },
            Entry::Vacant(item) => {
                let mut img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(width as u32, height as u32);
                for (x, y, pixel) in img.enumerate_pixels_mut()
                {
                    if (x + y) % 2 == 0
                    {
                        *pixel = image::Rgba([128, 128, 128, 255]);
                    }
                    else
                    {
                        *pixel = image::Rgba([127, 127, 127, 255]);
                    }
                }
                let img_txt = GenerateStage::graph_imag_to_file(img, "X".to_string());
                item.insert(img_txt.to_string());
                img_txt
            }
        }
    }

    pub fn graph_is_magic(img_txt: &str) -> bool
    {
        let map = GRAPH_MAGIC.lock().unwrap();
        if map.values().any(|item| item == img_txt)
        {
            true
        }
        else
        {
            /*let img = GenerateStage::graph_file_to_imag(img_txt);
            for (x, y, pixel) in img.enumerate_pixels()
            {
                if (x + y) % 2 == 0
                {
                    if pixel.0[0] != 255 { return false; }
                    if pixel.0[1] != 255 { return false; }
                    if pixel.0[2] != 255 { return false; }
                }
                else
                {
                    if pixel.0[0] != 0 { return false; }
                    if pixel.0[1] != 0 { return false; }
                    if pixel.0[2] != 0 { return false; }
                }
            }
            true*/
            false
        }
    }

    pub fn graph_mask_1(imag_txt: String, mask_txt: &str, mask_color_r: i32, mask_color_g: i32, mask_color_b: i32) -> String
    {
        let mut imag_data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = GenerateStage::graph_file_to_imag(&imag_txt);
        let mask_data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = GenerateStage::graph_file_to_imag(&mask_txt);
        let mask_color_r_8 = mask_color_r as u8;
        let mask_color_g_8 = mask_color_g as u8;
        let mask_color_b_8 = mask_color_b as u8;

        if (imag_data.len() > 0) && (mask_data.len() > 0)
        {
            let imag_raw: &mut [u8] = &mut imag_data;
            let mask_raw: &[u8] = &mask_data;

            for i in (0..imag_raw.len()).step_by(4)
            {
                if mask_raw[i + 1] > 0
                {
                    if mask_raw[i + 1] == 255
                    {
                        imag_raw[i + 0] = mask_color_r_8;
                        imag_raw[i + 1] = mask_color_g_8;
                        imag_raw[i + 2] = mask_color_b_8;
                    }
                    else
                    {
                        let v = mask_raw[i + 1] as i32;
                        imag_raw[i + 0] = (((mask_color_r * v) + ((imag_raw[i + 0] as i32) * (255 - v))) / 255) as u8;
                        imag_raw[i + 1] = (((mask_color_g * v) + ((imag_raw[i + 1] as i32) * (255 - v))) / 255) as u8;
                        imag_raw[i + 2] = (((mask_color_b * v) + ((imag_raw[i + 2] as i32) * (255 - v))) / 255) as u8;
                        
                    }
                }
            }

            GenerateStage::graph_imag_to_file(imag_data, imag_txt)
        }
        else
        {
            imag_txt
        }
    }

    pub fn graph_mask_3(imag_txt: String, mask_txt: &str, refe_txt: &str) -> String
    {
        let mut imag_data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = GenerateStage::graph_file_to_imag(&imag_txt);
        let mask_data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = GenerateStage::graph_file_to_imag(&mask_txt);
        let refe_data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = GenerateStage::graph_file_to_imag(&refe_txt);

        if (imag_data.len() > 0) && (mask_data.len() > 0) && (refe_data.len() > 0)
        {
            let imag_raw: &mut [u8] = &mut imag_data;
            let mask_raw: &[u8] = &mask_data;
            let refe_raw: &[u8] = &refe_data;

            for i in (0..imag_raw.len()).step_by(4)
            {
                if mask_raw[i + 1] < 255
                {
                    if mask_raw[i + 1] == 0
                    {
                        imag_raw[i + 0] = refe_raw[i + 0];
                        imag_raw[i + 1] = refe_raw[i + 1];
                        imag_raw[i + 2] = refe_raw[i + 2];
                    }
                    else
                    {
                        let v = mask_raw[i + 1] as i32;
                        imag_raw[i + 0] = ((((imag_raw[i + 0] as i32) * v) + ((refe_raw[i + 0] as i32) * (255 - v))) / 255) as u8;
                        imag_raw[i + 1] = ((((imag_raw[i + 1] as i32) * v) + ((refe_raw[i + 1] as i32) * (255 - v))) / 255) as u8;
                        imag_raw[i + 2] = ((((imag_raw[i + 2] as i32) * v) + ((refe_raw[i + 2] as i32) * (255 - v))) / 255) as u8;
                    }
                }
            }

            GenerateStage::graph_imag_to_file(imag_data, imag_txt)
        }
        else
        {
            imag_txt
        }
    }
}