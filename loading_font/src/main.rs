										     fn create_texture_from_text<'a>(texture_creator: &'a 
       TextureCreator<WindowContext>,
       font: &sdl2::ttf::Font,
       text: &str,
       r: u8, g: u8, b: u8,
       ) -> Option<Texture<'a>> {
         if let Ok(surface) = font.render(text)
           .blended(Color::RGB(r, g, b)) {
          texture_creator.create_texture_from_surface(&surface).ok()
         } else {
               None
           }
       }

			let rendered_text = create_texture_from_text(&texture_creator,
     &font, "test", 255, 255, 255).expect("Cannot render text");
     canvas.copy(&rendered_text, None, Some(Rect::new(width as i32 - 
        40, 0, 40, 30)))
    .expect("Couldn't copy text");

    fn display_game_information<'a>(tetris: &Tetris,
       canvas: &mut Canvas<Window>,
       texture_creator: &'a TextureCreator<WindowContext>,
       font: &sdl2::ttf::Font,
       start_x_point: i32) {
     let score_text = format!("Score: {}", tetris.score);
     let lines_sent_text = format!("Lines sent: {}", tetris.nb_lines);
     let level_text = format!("Level: {}", tetris.current_level);

     let score = create_texture_from_text(&texture_creator, &font,
        &score_text, 255, 255, 255)
        .expect("Cannot render text");
     let lines_sent = create_texture_from_text(&texture_creator, &font,
        &lines_sent_text, 255, 255, 255)
        .expect("Cannot render text");
     let level = create_texture_from_text(&texture_creator, &font,
        &level_text, 255, 255, 255)
        .expect("Cannot render text");
     
     canvas.copy(&score, None, get_rect_from_text(&score_text, 
       start_x_point, 90))
          .expect("Couldn't copy text");
    canvas.copy(&lines_sent, None, get_rect_from_text(&score_text,
       start_x_point, 125))
          .expect("Couldn't copy text");
    canvas.copy(&level, None, get_rect_from_text(&score_text, 
       start_x_point, 160))
          .expect("Couldn't copy text");
    }

fn main() {
	let font = ttf_context.load_font("assets/lucida.ttf", 128).expect("Coudln't load the front");
	font.set_style(sdl2::ttf::STYLE_BOLD);
}
