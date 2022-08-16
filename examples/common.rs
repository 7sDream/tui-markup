use std::path::Path;

use tui_markup::{compile, generator::Generator};

pub fn compile_file<G: Generator<'static> + Default, P: AsRef<Path>>(p: P) -> G::Output {
    let s = String::from_utf8(std::fs::read(p.as_ref()).unwrap()).unwrap();
    let s = Box::leak::<'static>(Box::new(s)).as_str();
    compile::<G>(s).unwrap()
}
