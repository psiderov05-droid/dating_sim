use std::ops::Index;

use macroquad::prelude::*;
use crate::chunk::Chunk_;

pub struct textDsiplayer
{
    text:Vec<Chunk_>
}
impl textDsiplayer
{
    pub fn new(text:Vec<Chunk_>) -> textDsiplayer
    {
        textDsiplayer
        {
            text
        }
    }

    pub async fn display_text(&self)
    {
        let mut counter:usize = 0;
        let line_distance:Option<f32> = Some(1.5);

        loop 
        {
            clear_background(BLACK);

            if is_key_down(KeyCode::Space)
            {
                let fixed_text = self.text[counter].text().replace("\\n", "\n");
                draw_multiline_text(&fixed_text, 50.0, 50.0, 40.0,line_distance,RED);
            }
            if is_key_released(KeyCode::Space)
            {
                counter += 1;
            }
            next_frame().await;
        }
        
    }

}