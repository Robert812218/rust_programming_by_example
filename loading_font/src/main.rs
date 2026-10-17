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

fn main() {
	let font = ttf_context.load_font("assets/lucida.ttf", 128).expect("Coudln't load the front");
	font.set_style(sdl2::ttf::STYLE_BOLD);
}
