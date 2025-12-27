use macroquad::prelude::*;
mod chunk;
use chunk::Chunk_;
mod chunk_handaler;
use chunk_handaler::ChunkHandaler;
mod text_displayer;
use text_displayer::textDsiplayer;

#[macroquad::main("Dating Sim")]
async fn main() {
    let message:&str = "Welcome to the Game!";
    let mut time_passed:f64 = 0.0;
    let mut opacity:f64 = 0.0;
    let mut handler:ChunkHandaler<> = chunk_handaler::ChunkHandaler::new();
    handler.load_text_from_file();
    let displayer:textDsiplayer = textDsiplayer::new(handler.list_of_chunks);

    /* 
    loop {
        clear_background(BLACK);

        draw_texture(&background_texture, 0.0,0.0, WHITE);
        draw_texture(&player_texture, 100.0, 100.0, WHITE);
        macroquad::text::draw_text("My text", 17.0, 50.0, 50.0, PINK);
        
        time_passed += get_frame_time();

        let chars_to_show = (time_passed * 8.0) as usize; // Type speed
        let visible_text = &message[0..chars_to_show.min(message.len())];

        opacity = f32::min(opacity + 0.02, 1.0);

        let color = Color::new(1.0, 1.0, 1.0, opacity);

        draw_text(visible_text, 20.0, 80.0, 40.0, color);
        
        next_frame().await;
    } 
    */

    loop 
    {
        clear_background(BLACK);

        displayer.display_text();

        next_frame().await;
    }
}