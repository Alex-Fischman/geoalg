use skulpin::app::*;
use skulpin::skia_safe::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Object {
	fn update(&mut self, _args: &AppUpdateArgs) {}
	fn draw(&mut self, _args: &mut AppDrawArgs) {}
}

pub type Wrapped<T> = Rc<RefCell<T>>;

pub fn wrap<T>(t: T) -> Wrapped<T> {
	Rc::new(RefCell::new(t))
}

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
