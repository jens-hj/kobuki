use std::{fs, path::Path};

use crate::model::msg::artifacts::Package;

use super::parse_msg::{get_metadata, read_file, MsgParser, MsgType};


pub struct PackageGenerator {
    pub path: Vec<String> 
}

impl PackageGenerator {
    pub fn default() -> Self {
        Self { path: vec![] }
    }

    pub fn read_directory_recursive(&mut self, input_path: &Path) -> Package {
        // create a module
        let mut package = Package::default();
        
        let name: String = input_path.file_name().unwrap().to_str().unwrap().into();
        self.path.push(name.clone());


        //println!("{}", name);

        package.name = name;

        let paths = fs::read_dir(input_path).unwrap();

        // read all elements in the current directory
        for local_dir in paths.map(|p| p.unwrap() ) {
            let md = local_dir.metadata().unwrap();

            if md.is_dir() {
                let a_path = local_dir.path().to_owned();
                let path_string = a_path.to_str().unwrap();
                let out_path = Path::new(&path_string);
                
                package.packages.push(self.read_directory_recursive(out_path));
    
            } else if md.is_file() {
                let path: std::path::PathBuf = local_dir.path();
                let data = get_metadata(path.as_path());

                match &data.file_type {
                    Some(msg_type) => {
                        match msg_type {
                            MsgType::MSG => {
                                let source: String = read_file(path.as_path());
                                let msg_parser: MsgParser = MsgParser::default();
                                package.messages.push(msg_parser.parse(data, source));
                            }
                        };
                    }, 
                    _ => continue
                } 
            }
        }

        self.path.pop();

        package
    }

}
