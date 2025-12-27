use crate::chunk::Chunk_;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct ChunkHandaler {
    pub list_of_chunks: Vec<Chunk_>,
}

impl ChunkHandaler {
    pub fn new() -> Self {
        Self {
            list_of_chunks: Vec::new(),
        }
    }

    pub fn load_text_from_file(&mut self) -> io::Result<()> {
        let file = File::open("/home/runerat/Desktop/Rust/dating_sim_reloaded/Text/dialog.txt")?;
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result?;
            let line_content: Vec<&str> = line.split('*').collect();

            let text = line_content[0].to_string();
            let name_of_char = line_content[1].to_string();
            let has_choice = line_content[2] != "No";
            let choice_screen_name = line_content[3].to_string();

            let new_chunk = Chunk_::new(
                text,
                name_of_char,
                has_choice,
                choice_screen_name,
            );

            self.list_of_chunks.push(new_chunk);
        }

        Ok(())
    }
}
