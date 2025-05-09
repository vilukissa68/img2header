use std::any::TypeId;
use std::fmt::{Display, LowerHex};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct CHeader<T> {
    name: String,
    data: Vec<T>,
    width: u32,
    height: u32,
    channels: u32,
    static_attr: bool,
    const_attr: bool,
    data_type: String,
    str_stream: String,
    output_path: PathBuf,
    write_hex: bool,
    link_section: String,
}

impl<T> CHeader<T>
where
    T: Display + Copy + Into<i64> + LowerHex + PartialOrd + 'static,
{
    pub fn new(
        name: String,
        data: Vec<T>,
        width: u32,
        height: u32,
        channels: u32,
        static_attr: bool,
        const_attr: bool,
        data_type: String,
        output_path: impl AsRef<Path>,
        write_hex: bool,
        link_section: String,
    ) -> Self {
        let output_path = output_path.as_ref().to_path_buf();

        // If no name for the variable is given, use the name of the output file
        let var_name = match name.chars().count() {
            0 => output_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            _ => name,
        };

        println!("Var name: {}", var_name);
        Self {
            name: var_name,
            data,
            width,
            height,
            channels,
            static_attr,
            const_attr,
            data_type,
            str_stream: String::new(),
            output_path,
            write_hex,
            link_section,
        }
    }

    /// Write the front guard of the header file
    fn write_front_guard(&mut self) {
        self.str_stream
            .push_str(&format!("#ifndef {name}_H\n", name = self.name));
        self.str_stream
            .push_str(&format!("#define {name}_H\n\n", name = self.name));
    }

    /// Write the back guard of the header file
    fn write_back_guard(&mut self) {
        self.str_stream
            .push_str(&format!("#endif // {name}_H\n", name = self.name));
    }

    /// Write the defines for the width, height, channels, and size of the data
    fn write_defines(&mut self) {
        // Include integer types
        self.str_stream.push_str("#include <stdint.h>\n");
        self.str_stream
            .push_str(&format!("#define WIDTH {width}\n", width = self.width));
        self.str_stream
            .push_str(&format!("#define HEIGHT {height}\n", height = self.height));
        self.str_stream.push_str(&format!(
            "#define CHANNELS {channels}\n",
            channels = self.channels
        ));
        self.str_stream.push_str(&format!(
            "#define SIZE {total_size}\n",
            total_size = self.width * self.height * self.channels
        ));
    }

    /// Write the data to the header file
    fn write_data(&mut self) {
        self.str_stream.push_str(&format!(
            "// size of data: {total_size}\n",
            total_size = self.width * self.height * self.channels
        ));

        if self.link_section != "" {
            self.str_stream.push_str(&format!(
                "__attribute__((section(\"{section}\")))\n",
                section = self.link_section
            ));
        }

        if self.static_attr {
            self.str_stream.push_str("static ");
        }
        if self.const_attr {
            self.str_stream.push_str("const ");
        }
        self.str_stream.push_str(&format!(
            "{data_type} data[SIZE] = {{\n",
            data_type = self.data_type
        ));
        for i in 0..self.width {
            for j in 0..self.height {
                for k in 0..self.channels {
                    if self.write_hex {
                        self.write_data_element_hex(
                            self.data[((i * self.height + j) * self.channels + k) as usize],
                        );
                    } else {
                        self.write_data_element_dec(
                            self.data[((i * self.height + j) * self.channels + k) as usize],
                        );
                    }
                    self.str_stream.push_str(", ");
                }
                self.str_stream.push('\n');
            }
        }
        self.str_stream.push_str("};\n");
    }

    /// Write a single data element to the header file in hexadecimal format
    fn write_data_element_hex(&mut self, value: T) {
        self.str_stream.push_str("0x");
        let type_id = TypeId::of::<T>();
        if type_id == TypeId::of::<i8>() || type_id == TypeId::of::<u8>() {
            self.str_stream
                .push_str(&format!("{:02x}", value.into() as u8));
        } else if type_id == TypeId::of::<i16>() || type_id == TypeId::of::<u16>() {
            self.str_stream
                .push_str(&format!("{:04x}", value.into() as u16));
        } else if type_id == TypeId::of::<i32>() || type_id == TypeId::of::<u32>() {
            self.str_stream
                .push_str(&format!("{:08x}", value.into() as u32));
        } else {
            self.str_stream.push_str(&format!("{:x}", value));
        }
    }

    /// Write a single data element to the header file in decimal format
    fn write_data_element_dec(&mut self, value: T) {
        let as_i64 = value.into();
        if as_i64 < 10 {
            self.str_stream.push_str(&format!("{:1}", as_i64));
        } else if as_i64 < 100 {
            self.str_stream.push_str(&format!("{:2}", as_i64));
        } else {
            self.str_stream.push_str(&format!("{:3}", as_i64));
        }
    }

    /// Write the header file to the string stream
    pub fn write_header(&mut self) {
        self.write_front_guard();
        self.write_defines();
        self.write_data();
        self.write_back_guard();
    }

    /// Write contents of the string stream to a file
    pub fn write_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create(self.output_path.clone())?;
        file.write_all(self.str_stream.as_bytes())?;
        Ok(())
    }
}
