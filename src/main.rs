use clap::Parser;
use image::{DynamicImage, GenericImageView};

mod header;
pub use crate::header::CHeader;
mod transform;
pub use crate::transform::{resize, to_gray};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Output type of data
    #[arg(short, long, default_value = "uint8_t")]
    data_type: String,
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
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    /// Add alpha channel
    #[arg(short, long, default_value = "false")]
    alpha_channel: bool,
    /// Path to file to convert
    file: String,
}

fn open_image(file: &str) -> DynamicImage {
    image::open(file).expect("Failed to open image")
}

fn parse_transformation(img: DynamicImage, args: &Args) -> DynamicImage {
    let target_width = match args.width {
        0 => img.width(),
        _ => args.width,
    };

    let target_height = match args.height {
        0 => img.height(),
        _ => args.height,
    };

    let resized = img.resize(
        target_width,
        target_height,
        image::imageops::FilterType::Nearest,
    );

    if args.grayscale {
        resized.grayscale()
    } else {
        resized
    }
}

fn extract_pixel_data(img: &DynamicImage, grayscale: bool, alpha: bool) -> Vec<u8> {
    if grayscale {
        img.to_luma8().into_raw()
    } else if alpha {
        img.to_rgba8().into_raw()
    } else {
        img.to_rgb8().into_raw()
    }
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Opening image {}", args.file);
    }

    let mut img = open_image(&args.file);

    if args.verbose {
        println!(
            "Image: {}x{}x{}",
            img.dimensions().0,
            img.dimensions().1,
            img.color().channel_count()
        );
        println!("Transforming image...");
    }

    img = parse_transformation(img, &args);

    let width = img.width();
    let height = img.height();
    let channels = match (args.grayscale, args.alpha_channel) {
        (true, _) => 1,
        (false, false) => 3,
        (false, true) => 4,
    };

    let data = extract_pixel_data(&img, args.grayscale, args.alpha_channel);

    // Print verbose information
    if args.verbose {
        println!(
            "Image: {}x{}x{}",
            img.dimensions().0,
            img.dimensions().1,
            channels,
        );
        println!("Creating header file...");
    }

    let mut header = CHeader::new(
        args.name,
        data,
        width,
        height,
        channels,
        args,
        args.static_attr,
        args.const_attr,
        args.data_type.clone(),
        args.output.clone(),
        args.hex,
    );
    header.write_header();
    let _ = header.write_to_file();
    if args.verbose {
        println!("Header file written to {}", args.output);
    }
    println!("Done!");
}
