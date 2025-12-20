use env_logger;
use log::{debug, info};
/* fus
*/
fn main() {
    env_logger::init();
    // test subject
    // test description1
    // test description2
    info!("hello");
    // asdf
    info!("bye");
    // test subject debug
    debug!("hello, debug!");
}
