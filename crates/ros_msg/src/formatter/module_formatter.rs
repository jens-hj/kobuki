use crate::model::{msg::artifacts::Package, rust::artifacts::Module};

use super::msg_formatter::msg_to_struct;

pub fn package_to_module(package: &Package) -> Module {
    let mut module = Module::default();
    module.name = package.name.clone();
    
    for msg in &package.messages {
        let _struct = msg_to_struct(msg, 0);
        module.structs.push(_struct);
    }

    for child_package in &package.packages {
        module.modules.push(package_to_module(child_package));
    }

    module
}




/*
#[cfg(test)]
mod module_tests {
    use crate::model::msg::artifacts::Msg;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generate_modules() {
        
        let package = Package::default();

        let (out, _) = build_rust_libs_recursive(0, &package);

        assert_eq!("", out);
    }

    #[test]
    fn generate_modules_with_struct() {
        
        let mut package = Package::default();
        let mut msg = Msg::default();
        msg.name = "Test".to_string();
        package.messages.push(msg);
        let (out, _) = build_rust_libs_recursive(0, &package);

        assert_eq!("pub mod  {\n    pub struct Test {\n    }\n}\n", out);
    }

    #[test]
    fn nested_generate_modules_with_child() {
        let mut package = Package::default();
        package.name = "parent".to_string();
        let mut child_package = Package::default();
        child_package.name = "child".to_string();
        let mut msg = Msg::default();
        msg.name = "Test".to_string();
        child_package.messages.push(msg);
        package.packages.push(child_package);
        let (out, _) = build_rust_libs_recursive(0, &package);
        let  expected = "pub mod parent {\npub mod child {\n    pub struct Test {\n    }\n}\n}\n";
        assert_eq!(expected, out);
    }
    #[test]
    fn nested_generate_modules_with_siblings() {
        let mut package = Package::default();
        package.name = "parent".to_string();
        let mut child_package = Package::default();
        child_package.name = "child".to_string();
        let mut msg = Msg::default();
        msg.name = "Test".to_string();
        child_package.messages.push(msg);
        package.packages.push(child_package);

        let mut child_package2 = Package::default();
        child_package2.name = "child2".to_string();
        let mut msg2 = Msg::default();
        msg2.name = "Test2".to_string();
        child_package2.messages.push(msg2);

        package.packages.push(child_package2);

        let (out, _) = build_rust_libs_recursive(0, &package);
        let  expected = "pub mod parent {\npub mod child {\n    pub struct Test {\n    }\n}\npub mod child2 {\n    pub struct Test2 {\n    }\n}\n}\n";

        assert_eq!(expected, out);
    }
}
 */