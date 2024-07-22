extern crate logdoc;
use logdoc::app::Application;

fn main() {
    let result = Application::run();
    match result {
        Ok(_) => {}
        Err(e) => println!("Failed: {e:?}"),
    }
}
