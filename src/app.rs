use skulpin::app::*;
use skulpin::skia_safe::*;

pub trait Object {
	fn update(&mut self, _args: &AppUpdateArgs) {}
	fn draw(&mut self, _args: &mut AppDrawArgs) {}
}

pub type Wrapped<T> = std::rc::Rc<std::cell::RefCell<T>>;

pub struct App {
	objects: Vec<Wrapped<dyn Object>>,
}

impl App {
	pub fn new(objects: Vec<Wrapped<dyn Object>>) -> App {
		App { objects }
	}
}

impl AppHandler for App {
	fn update(&mut self, mut args: AppUpdateArgs) {
		for object in &mut self.objects {
			object.borrow_mut().update(&mut args);
		}
	}

	fn draw(&mut self, mut args: AppDrawArgs) {
		args.canvas.clear(Color::WHITE);
		for object in &mut self.objects {
			object.borrow_mut().draw(&mut args);
		}
	}

	fn fatal_error(&mut self, _error: &AppError) {}
}

pub fn new_paint(color: Color) -> Paint {
	let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 0.0), None);
	paint.set_color(color);
	paint
}
