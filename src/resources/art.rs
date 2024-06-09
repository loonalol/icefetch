use std::time::{SystemTime, UNIX_EPOCH};

#[warn(unused_imports)]
use termimage::ops::{guess_format, load_image, write_ansi_truecolor};
use std::io::{self, stdout};
#[allow(unused_imports)]
use std::path::PathBuf;
use term_size;
use image::{DynamicImage, imageops::FilterType};
use std::fs;
#[allow(dead_code)]
pub fn resize_to_terminal(image: DynamicImage) -> DynamicImage {
    if let Some((width, height)) = term_size::dimensions() {
        let img_size = crate::config::image();
        let (width, height) = (width as u32, height as u32 * img_size); 
        image.resize(width, height, FilterType::Lanczos3)
    } else {
        image
    }
}
#[allow(dead_code)]
pub fn display_art() -> io::Result<()> {
    let image_dir = "src/images";
    let mut image_paths = Vec::new();
    if let Ok(paths) = fs::read_dir(image_dir) {
        for path in paths {
            if let Ok(entry) = path {
#[allow(unused_variables)]
                if let Some(file_name) = entry.file_name().to_str() {
                    image_paths.push(entry.path());
                }
            }
        }
    }
    if image_paths.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No image files found"));
    }
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let random_index = (seed % image_paths.len() as u64) as usize;
    let selected_image_path = &image_paths[random_index];

    let image_reference = &(selected_image_path.to_string_lossy().to_string(), selected_image_path.clone());
    let format = guess_format(image_reference).expect("Failed to guess image format");
    let image = load_image(image_reference, format).expect("Failed to load image");
    let resized_image = resize_to_terminal(image);
    write_ansi_truecolor(&mut stdout(), &resized_image);
    Ok(())
}
