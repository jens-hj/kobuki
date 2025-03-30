use std::{fs, path::Path};

use crate::{formatter::module_formatter::package_to_module, model::{msg::artifacts::Package, rust::artifacts::Formattable}, parser::msg::parse_package::PackageGenerator};


fn dir_to_package(input: &str) -> Package {
    let path = Path::new(input);
    let mut mod_gen = PackageGenerator::default();
    mod_gen.read_directory_recursive(path)
}

// accepts a director outputs a string with the source for the given libraries.
pub fn build_msg_libs(dir: &str, out_dir: &str) {
    let mut package = dir_to_package(dir);
    // this needs to go into a std_msgs package
    package.messages.push(Package::build_default_message());
    let module = package_to_module(&package);
    let source = module.format();
    fs::write(format!("{}/{}.rs", out_dir, "ros_msg_defs") , source).expect("Unable to write file");
}

