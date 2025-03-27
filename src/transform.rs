use image::DynamicImage;

pub fn to_gray(image: &DynamicImage) -> DynamicImage {
    image.grayscale()
}

pub fn resize(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Nearest)
}
