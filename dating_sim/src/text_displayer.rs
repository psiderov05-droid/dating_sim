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

    pub fn display_text(&self)
    {
        if is_key_down(KeyCode::Space)
        {
            draw_text(self.text[0].text(), 100.0,100.0, 50.0, BLUE);
        }
    }

}