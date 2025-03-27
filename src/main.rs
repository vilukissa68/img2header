use clap::Parser;
use image::{DynamicImage, GenericImageView};

mod header;
pub use crate::header::CHeader;
mod transform;
pub use crate::transform::{resize, to_gray};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Path to file to convert
    #[arg(short, long)]
    file: String,
    /// Output type of data
    #[arg(short, long, default_value = "uint8_t")]
    data_type: Option<String>,
    /// Whether to make the data static
    #[arg(long, default_value = "false")]
    static_attr: bool,
    /// Whether to make the data const
    #[arg(long, default_value = "false")]
    const_attr: bool,
    /// Desired output width
    #[arg(long, default_value = "0")]
    width: u32,
    /// Desired output height
    #[arg(long, default_value = "0")]
    height: u32,
    /// Output as grayscale
    #[arg(long, default_value = "false")]
    grayscale: bool,
    /// Path for output
    #[arg(short, long, default_value = "output.h")]
    output: String,
    /// Name of the output variable
    #[arg(short, long, default_value = "")]
    name: String,
    /// Write output data in hexadicimal format
    #[arg(long, default_value = "false")]
    hex: bool,
}

fn open_image(file: &str) -> DynamicImage {
    let image = image::open(file).unwrap();
    image
}

fn parse_transformation(img: DynamicImage, args: &Args) -> DynamicImage {
    let mut img = img;

    let target_width = match args.width {
        0 => img.width(),
        _ => args.width,
    };

    let target_height = match args.height {
        0 => img.height(),
        _ => args.height,
    };

    img = img.resize(
        target_width,
        target_height,
        image::imageops::FilterType::Nearest,
    );

    if args.grayscale {
        img = img.grayscale();
    }

    img
}

fn main() {
    let args = Args::parse();
    let img = open_image(&args.file);

    let img = parse_transformation(img, &args);

    let mut header = CHeader::new(
        args.name,
        img.to_rgb8()
            .pixels()
            .map(|p| vec![p[0], p[1], p[2]])
            .flatten()
            .collect(),
        img.dimensions().0,
        img.dimensions().1,
        img.color().channel_count() as u32,
        args.static_attr,
        args.const_attr,
        args.data_type.unwrap_or("uint8_t".to_string()),
        args.output,
        args.hex,
    );
    header.write_header();
    let _ = header.write_to_file();
    println!("Done!");
}
