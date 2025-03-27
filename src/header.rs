use std::fs::File;
use std::io::prelude::*;

pub struct CHeader {
    name: String,
    data: Vec<u8>,
    width: u32,
    height: u32,
    channels: u32,
    static_attr: bool,
    const_attr: bool,
    data_type: String,
    str_stream: String,
    output_path: String,
    write_hex: bool,
}

impl CHeader {
    pub fn new(
        name: String,
        data: Vec<u8>,
        width: u32,
        height: u32,
        channels: u32,
        static_attr: bool,
        const_attr: bool,
        data_type: String,
        output_path: String,
        write_hex: bool,
    ) -> Self {
        // If no name for the variable is given, use the name of the output file
        let var_name = match name.chars().count() {
            0 => output_path.clone(),
            _ => name,
        };

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
                self.str_stream.push_str("\n");
            }
        }
        self.str_stream.push_str("};\n");
    }

    /// Write a single data element to the header file in hexadecimal format
    fn write_data_element_hex(&mut self, hex: u8) {
        self.str_stream.push_str("0x");
        self.str_stream.push_str(&format!("{:02x}", hex));
    }

    /// Write a single data element to the header file in decimal format
    fn write_data_element_dec(&mut self, dec: u8) {
        if dec < 10 {
            self.str_stream.push_str(&format!("{:1}", dec));
        } else if dec < 100 {
            self.str_stream.push_str(&format!("{:1}", dec));
        } else {
            self.str_stream.push_str(&format!("{:3}", dec));
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
        let mut file = File::create(format!("{output_path}.h", output_path = self.output_path))?;
        file.write_all(self.str_stream.as_bytes())?;
        Ok(())
    }
}
