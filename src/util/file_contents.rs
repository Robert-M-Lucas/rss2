use std::path::Path;
use std::fs;
use human_bytes::human_bytes;

type ZipLengthType = u64;
const ZIP_LENGTH_SIZE: usize = size_of::<ZipLengthType>();

pub struct FileContents {
    contents: Vec<u8>,
    zip_length: usize,
}

impl FileContents {
    pub fn new(zipped_contents: Vec<u8>, binary_contents: Vec<u8>) -> Self {
        let mut final_contents = Vec::with_capacity(ZIP_LENGTH_SIZE + zipped_contents.len() + binary_contents.len());
        final_contents.extend_from_slice((zipped_contents.len() as ZipLengthType).to_le_bytes().as_ref());
        final_contents.extend_from_slice(&zipped_contents);
        final_contents.extend_from_slice(&binary_contents);
        FileContents {
            contents: final_contents,
            zip_length: zipped_contents.len()
        }
    }

    pub fn remove_binary(&mut self) {
        self.contents.truncate(ZIP_LENGTH_SIZE + self.zip_length);
    }

    pub fn replace_binary(&mut self, binary: &[u8]) {
        self.remove_binary();
        self.contents.extend_from_slice(binary);
    }

    pub fn print_stats(&self) {
        println!("Project zip size: {}", human_bytes(self.zip_length as f64));
        println!("Binary size: {}", human_bytes((self.contents.len() - self.zip_length - ZIP_LENGTH_SIZE) as f64));
        println!("Total size: {}", human_bytes(self.contents.len() as f64));
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Option<FileContents>, String> {
        if !path.as_ref().exists() { return Ok(None) }
        let contents = fs::read(&path).map_err(|e| format!("E07 Failed to read file: {}", e))?;
        if contents.len() < 8 {
            return Err(format!("Corrupted file: {:?} [E1]", path.as_ref()));
        }
        let zipped_len = ZipLengthType::from_le_bytes(contents[0..8].try_into().unwrap());
        if ((contents.len() - ZIP_LENGTH_SIZE) as ZipLengthType) < zipped_len {
            return Err(format!("Corrupted file: {:?} [E2]", path.as_ref()));
        }
        Ok(Some(FileContents {
            contents,
            zip_length: zipped_len as usize,
        }))
    }

    pub fn zipped_contents(&self) -> &[u8] {
        &self.contents[ZIP_LENGTH_SIZE..self.zip_length + ZIP_LENGTH_SIZE]
    }

    pub fn bin_contents(&self) -> &[u8] {
        &self.contents[self.zip_length + ZIP_LENGTH_SIZE..]
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        fs::write(path, &self.contents).map_err(|e| format!("E08 Failed to write file: {}", e))
    }
}