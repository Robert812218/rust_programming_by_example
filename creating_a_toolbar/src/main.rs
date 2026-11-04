use gtk::{
	ContainerExt,
	SeparatorToolItem,
	Toolbar,
	ToolButton,
},
fn main() {
	const PLAY_STOCK: &str = "gtk-media-play";

	application.connect_startup(|application| {
		let toolbar = Toolbar::new();
		window.add(&toolbar);
	}
	let open_button = ToolButton::new_from_stock("gtk-open");
	toolbar.add(&open_button);
}
