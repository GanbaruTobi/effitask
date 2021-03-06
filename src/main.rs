#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate log;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod add;
mod agenda;
mod application;
mod date;
mod done;
mod edit;
mod flag;
mod inbox;
mod logger;
mod search;
mod tasks;
mod widgets;

fn main() {
    setup_panic!();

    if ::std::env::args().nth(1) == Some("usage".to_owned()) {
        usage(&::std::env::args().nth(0).unwrap());

        ::std::process::exit(0);
    }

    crate::application::Widget::run(()).unwrap();
}

fn usage(program: &str) {
    let path = ::std::path::Path::new(&program);

    println!("    {}", path.file_name().unwrap().to_str().unwrap());
    println!("      Launch focus graphical interface");
}
