fn startup_handler(application: &Application) {
	let window = ApplicationWindow::new(&application);
	window.set_title("Rusic");
	window.show()
}

fn main() {
	|application| {
		let window = ApplicationWindow::new(&application);
		window.set_title("Rustic");
		window.show();
	}
	application.connect_startup(startup_handler);
	application.run(&env::args().collect::<Vec<_>>());

	window.connect_delete_event(|_, _| {
		Inhibit(true)
	})
	window.connect_delete_event(|_, _| {
		Inhibit(false)
	})
}
