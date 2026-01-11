use env_logger;
use log;
use log::{debug, info, trace};
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
    // use scope identifier
    // description
    log::warn!("this is scoped identifier message");
    debug!("hello trace");
}
