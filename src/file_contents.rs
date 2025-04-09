use std::path::Path;
use std::fs;
use human_bytes::human_bytes;

pub struct FileContents {
    contents: Vec<u8>,
    zip_length: usize,
}

impl FileContents {
    pub fn new(zipped_contents: Vec<u8>, binary_contents: Vec<u8>) -> Self {
        let mut final_contents = Vec::with_capacity(8 + zipped_contents.len() + binary_contents.len());
        final_contents.extend_from_slice((zipped_contents.len() as u64).to_le_bytes().as_ref());
        final_contents.extend_from_slice(&zipped_contents);
        final_contents.extend_from_slice(&binary_contents);
        FileContents {
            contents: final_contents,
            zip_length: zipped_contents.len()
        }
    }

    pub fn print_stats(&self) {
        println!("Project zip size: {}", human_bytes(self.zip_length as f64));
        println!("Binary size: {}", human_bytes((self.contents.len() - self.zip_length - 8) as f64));
        println!("Total size: {}", human_bytes(self.contents.len() as f64));
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Option<FileContents>, String> {
        if !path.as_ref().exists() { return Ok(None) }
        let contents = fs::read(&path).map_err(|e| format!("E07 Failed to read file: {}", e))?;
        if contents.len() < 8 {
            return Err(format!("Corrupted file: {:?} [E1]", path.as_ref()));
        }
        let zipped_len = u64::from_le_bytes(contents[0..8].try_into().unwrap());
        if (contents.len() as u64 - 8) < zipped_len {
            return Err(format!("Corrupted file: {:?} [E2]", path.as_ref()));
        }
        Ok(Some(FileContents {
            contents,
            zip_length: zipped_len as usize,
        }))
    }

    pub fn zipped_contents(&self) -> &[u8] {
        &self.contents[8..self.zip_length + 8]
    }

    pub fn bin_contents(&self) -> &[u8] {
        &self.contents[self.zip_length + 8..]
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        fs::write(path, &self.contents).map_err(|e| format!("E08 Failed to write file: {}", e))
    }
}