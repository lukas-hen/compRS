use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

pub struct BitWriter<W: Write> {
    writer: W,
    buffer: u8,
    buffer_len: u8, // Number of bits currently in the buffer
}

impl<W: Write> BitWriter<W> {
    /// Create a new `BitWriter` that writes to the given file.
    pub fn new(writer: W) -> Self {
        Self {
            writer: writer,
            buffer: 0,
            buffer_len: 0,
        }
    }

    /// Write the least significant `num_bits` of `value` to the output.
    pub fn write_bits(&mut self, value: u32, num_bits: u8) -> Result<(), Box<dyn Error>> {
        let mut bits_to_write = num_bits;
        let mut value = value
            & if num_bits < 32 {
                (1 << num_bits) - 1
            } else {
                u32::MAX
            };

        while bits_to_write > 0 {
            let space_left = 8 - self.buffer_len;
            let bits_to_copy = bits_to_write.min(space_left);

            // Copy the top `bits_to_copy` bits into the buffer
            self.buffer |=
                ((value >> (bits_to_write - bits_to_copy)) as u8) << (space_left - bits_to_copy);

            bits_to_write -= bits_to_copy;
            self.buffer_len += bits_to_copy;

            // If the buffer is full, write it to the file
            if self.buffer_len == 8 {
                self.flush_buffer()?;
            }

            // Clear the written bits from `value`
            value &= (1 << bits_to_write) - 1;
        }

        Ok(())
    }

    /// Flush the remaining bits in the buffer to the file, padding with zeros if necessary.
    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        if self.buffer_len > 0 {
            self.flush_buffer()?;
        }
        self.writer.flush()?;
        Ok(())
    }

    /// Internal method to flush the buffer (only if it's full or on `flush` call).
    fn flush_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.write_all(&[self.buffer])?;
        self.buffer = 0;
        self.buffer_len = 0;
        Ok(())
    }
}

impl<W: Write> Drop for BitWriter<W> {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            eprintln!("Failed to flush BitWriter: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn bit_writer_test() {
        let file_path = "test_output.bin";
        let file = File::create(file_path).unwrap();
        let mut writer = BitWriter::new(file);

        // Write some values using varying bit lengths
        writer.write_bits(0b101, 3).unwrap();
        writer.write_bits(0b11111111, 8).unwrap();
        writer.write_bits(0b1, 1).unwrap();
        writer.write_bits(0b10, 2).unwrap();
        writer.flush().unwrap();

        let written_data = fs::read(file_path).unwrap();
        assert_eq!(written_data, vec![0b10111111, 0b11111000]);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn bit_write_long_test() {
        let file_path = "test_output2.bin";
        let file = File::create(file_path).unwrap();
        let mut writer = BitWriter::new(file);

        // Write ONE long 32 bit seq.
        writer
            .write_bits(0b10100010101001001110001010101000, 32)
            .unwrap();
        writer.flush().unwrap();

        let written_data = fs::read(file_path).unwrap();
        assert_eq!(
            written_data,
            vec![0b10100010, 0b10100100, 0b11100010, 0b10101000]
        );

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

        #[test]
    fn bit_write_multi_buffer_test() {
        let file_path = "test_output3.bin";
        let file = File::create(file_path).unwrap();
        let mut writer = BitWriter::new(file);

        writer
            .write_bits(0b10100010101001001110001010101000, 32)
            .unwrap();
        writer
            .write_bits(0b10100010101001001110001010101001, 32)
            .unwrap();
        writer
            .write_bits(0b00000111, 3)
            .unwrap();
        writer
            .write_bits(0b00000101, 3)
            .unwrap();
        writer
            .write_bits(0b00000010, 2)
            .unwrap();
        writer.flush().unwrap();

        let written_data = fs::read(file_path).unwrap();
        assert_eq!(
            written_data[0..4],
            vec![0b10100010, 0b10100100, 0b11100010, 0b10101000]
        );
        assert_eq!(
            written_data[4..8],
            vec![0b10100010, 0b10100100, 0b11100010, 0b10101001]
        );
        println!("{:#08b}", written_data[8]);
        assert_eq!(
            written_data[8],
            0b11110110,
        );
        
        // Clean up
        fs::remove_file(file_path).unwrap();
    }
}
