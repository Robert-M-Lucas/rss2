use std::borrow::Cow;
use std::path::Path;
use std::fs;
use human_bytes::human_bytes;

type LengthType = u64;
const LENGTH_TYPE_SIZE: usize = size_of::<LengthType>();

pub struct FileContents {
    contents: Vec<u8>,
    triple_length: usize,
    zip_length: usize,
}

impl FileContents {
    pub fn new(zipped_contents: Vec<u8>, binary_contents: Vec<u8>, triple: &str) -> Self {
        let mut final_contents = Vec::with_capacity(LENGTH_TYPE_SIZE + zipped_contents.len() + binary_contents.len());
        final_contents.extend_from_slice((zipped_contents.len() as LengthType).to_le_bytes().as_ref()); // zipped len
        final_contents.extend_from_slice(&zipped_contents); // zipped
        final_contents.extend_from_slice((triple.as_bytes().len() as LengthType).to_le_bytes().as_ref()); // triple len
        final_contents.extend(triple.as_bytes()); // triple
        final_contents.extend_from_slice(&binary_contents); // binary
        FileContents {
            contents: final_contents,
            triple_length: triple.as_bytes().len(),
            zip_length: zipped_contents.len()
        }
    }

    pub fn remove_binary(&mut self) {
        self.contents.truncate(LENGTH_TYPE_SIZE + self.zip_length + LENGTH_TYPE_SIZE + self.triple_length);
    }

    pub fn replace_binary(&mut self, triple: &str, binary: &[u8]) {
        self.contents.truncate(LENGTH_TYPE_SIZE + self.zip_length);
        self.triple_length = triple.as_bytes().len();
        self.contents.extend_from_slice((triple.as_bytes().len() as LengthType).to_le_bytes().as_ref()); // triple len
        self.contents.extend(triple.as_bytes()); // triple
        self.contents.extend_from_slice(binary); // binary
    }

    pub fn print_stats(&self) {
        println!("Project zip size: {}", human_bytes(self.zip_length as f64));
        println!("Target triple indicator size: {}", human_bytes(self.triple_length as f64));
        println!("Binary size: {}", human_bytes((self.contents.len() - self.zip_length - LENGTH_TYPE_SIZE) as f64));
        println!("Total size: {}", human_bytes(self.contents.len() as f64));
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Option<FileContents>, String> {
        if !path.as_ref().exists() { return Ok(None) }
        let contents = fs::read(&path).map_err(|e| format!("E07 Failed to read file: {}", e))?;
        if contents.len() < 8 {
            return Err(format!("Corrupted file: {:?} [E1]", path.as_ref()));
        }
        let zipped_len = LengthType::from_le_bytes(contents[0..8].try_into().unwrap()) as usize;
        // Check for zipped length + target triple length indicator
        if (contents.len() - LENGTH_TYPE_SIZE) < zipped_len + LENGTH_TYPE_SIZE {
            return Err(format!("Corrupted file: {:?} [E2]", path.as_ref()));
        }
        let triple_len = LengthType::from_le_bytes(contents[LENGTH_TYPE_SIZE + zipped_len..LENGTH_TYPE_SIZE + zipped_len + LENGTH_TYPE_SIZE].try_into().unwrap()) as usize;
        if (contents.len() - LENGTH_TYPE_SIZE - zipped_len - LENGTH_TYPE_SIZE) < triple_len {
            return Err(format!("Corrupted file: {:?} [E46]", path.as_ref()));
        }
        Ok(Some(FileContents {
            contents,
            triple_length: triple_len,
            zip_length: zipped_len,
        }))
    }

    pub fn zipped_contents(&self) -> &[u8] {
        &self.contents[LENGTH_TYPE_SIZE..LENGTH_TYPE_SIZE + self.zip_length]
    }

    pub fn target_triple(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.contents[LENGTH_TYPE_SIZE + self.zip_length + LENGTH_TYPE_SIZE..LENGTH_TYPE_SIZE + self.zip_length + LENGTH_TYPE_SIZE + self.triple_length])
    }

    pub fn bin_contents(&self) -> &[u8] {
        &self.contents[LENGTH_TYPE_SIZE + self.zip_length + LENGTH_TYPE_SIZE + self.triple_length..]
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        fs::write(path, &self.contents).map_err(|e| format!("E08 Failed to write file: {}", e))
    }
}