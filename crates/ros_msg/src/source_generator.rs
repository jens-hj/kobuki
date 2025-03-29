use std::{fs, path::Path, vec};

use crate::{formatter::{common::{build_padding_string, TAB_SIZE}, field_formatter::generate_rust_source_from_model}, model::msg::{MsgMember, MsgModelDefinition}};


struct MsgModule {
    name: String,
    modules: Vec<Self>,
    msg_structs: Vec<MsgModelDefinition>
}

impl MsgModule {
    fn default() -> Self {
        Self {
            name: "".to_string(), 
            modules: vec![],
            msg_structs: vec![]
        }
    }
}

struct ModuleGenerator {
    path: Vec<String> 
}

impl ModuleGenerator {

    fn default() -> Self {
        Self { path: vec![] }
    }

    pub fn read_directory_recursive(&mut self, input_path: &Path) -> MsgModule {
        // create a module
        let mut module = MsgModule::default();
        let name: String = input_path.file_name().unwrap().to_str().unwrap().into();
        
        self.path.push(name.clone());
        module.name = name;

        let paths = fs::read_dir(input_path).unwrap();
        // read all elements in the current directory
        for local_dir in paths.map(|p| p.unwrap() ) {
            
            let md = local_dir.metadata().unwrap();
            
            if md.is_dir() {

                let a_path = local_dir.path().to_owned();
                let path_string = a_path.to_str().unwrap();
                let out_path = Path::new(&path_string);
                
                module.modules.push(self.read_directory_recursive(out_path));
    
            } else if md.is_file() {
                // do things with files
                match process_file(local_dir.path().as_path()) {
                    Some(model) => module.msg_structs.push(model),
                    None => {}
                }
            }
    
        }
        self.path.pop();

        module
    }

}


fn process_file(file: &Path) -> Option<MsgModelDefinition> {
    match file.extension() {
        Some(x) => {
            if x == "msg" {
                Some(generate_msg_model(file))
            } else {
                None
            }
        }
        None => None
    }
}


fn generate_msg_model(path: &Path) -> MsgModelDefinition {
    
    let file_name: String= String::from(path.file_name().to_owned().unwrap().to_str().unwrap());

    let msg_source = fs::read_to_string(path.as_os_str()).expect("Unable to read file");

    let msg_name: String = file_name.replace(".msg", "");

    let msg = tokenize_source(msg_name, msg_source).unwrap();
    // generate msg definitions
    generate_data_model(msg)

}


fn src_to_module(input: &str) -> MsgModule {
    let mut mod_gen = ModuleGenerator::default();
    let path = Path::new(input);
    let out = mod_gen.read_directory_recursive(path);

    //println!("{}", out.name);
    //println!("{:?}", out.modules.iter().map(|a| a.name.to_string()).collect::<Vec<String>>());

    out

}

// accepts a director outputs a string with the source for the given libraries.
pub fn build_msg_libs(dir: &str, out_dir: &str) {

    let module = src_to_module(dir);
    
    // convert dir to path and generate the model of the message
    let (out, _) = build_rust_libs_recursive(0, &module);
    fs::write(format!("{}/{}.rs", out_dir, "ros_msg_defs") , out).expect("Unable to write file");
}


fn build_rust_libs_recursive(depth: usize, module: &MsgModule) -> (String, bool) {
    let mut working_depth = depth;
    let padding = build_padding_string(depth);

    let mut source = format!("{}pub mod {} {{\n", padding, module.name);

    let mut has_structs = !module.msg_structs.is_empty();


    if has_structs { working_depth = depth + TAB_SIZE }
    for msg_struct in &module.msg_structs {
        let struct_source = generate_rust_source_from_model(msg_struct, working_depth).unwrap();
        source = format!("{}{}\n", source, struct_source);
    }

    for module in &module.modules {
        let (module_source, child_has_structs) = build_rust_libs_recursive(working_depth, module);

        has_structs = child_has_structs || has_structs;

        if child_has_structs  {
            source = format!("{}{}{}", source, padding, module_source);
        }

    }

    source = format!("{}{}}}\n",source, padding);
    if has_structs {
        (source, has_structs )
    } else {
        ("".to_string(), has_structs)
    }
}


#[derive(Debug, PartialEq)]
struct Msg {
    title: String,
    tokens: Vec<(String, String)>,
}

impl Msg {
    fn default() -> Self {
        Msg {
            title: String::from(""),
            tokens: vec![],
        }
    }
}

fn tokenize_source(name: String, source: String) -> Option<Msg> {
    if source.is_empty() {
        return None;
    };

    let mut msg = Msg::default();

    msg.title = name;

    // split source up by lines
    let lines = source.lines();

    for line in lines {        
        match handle_token_line(line) {
            Some(token) => msg.tokens.push(token),
            None => continue
        }
    }

    Some(msg)
}


fn handle_token_line(line: &str) -> Option<(String, String)> {
    // remove tabs
    let formatted_line = line.replace("\t", " ");
    let tokens = formatted_line.split(" ").filter(|&elem|  { !elem.is_empty() }).collect::<Vec<&str>>();
    
    // println!("{:?}", tokens);
    
    if tokens.len() < 2 {
        return None;
    }

    if tokens.get(0).unwrap().starts_with("#") {
        return  None;
    }


    let mut value = tokens.get(1).unwrap().to_string();
    
    value = value.chars()

    .take_while(|&ch| ch != '=')
    .collect::<String>();
    
    Some((
        String::from(*tokens.get(0).unwrap()),
        value,
    ))
}



fn generate_data_model(msg: Msg) -> MsgModelDefinition {
    let mut out = MsgModelDefinition::default();
    out.name = msg.title;

    for token in msg.tokens {
        // println!("tokens: {:?}", token);
        let name = token.1;
        let m_type = token.0;
        
        let mut msg_mem = MsgMember::default();

        msg_mem.member_name = name;
        msg_mem.member_type = m_type;
        
        out.members.push(msg_mem);
    }


    out
}



#[cfg(test)]
mod generate_data_model {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_generate_data_model() {
        let input = Msg::default();
        assert_eq!(generate_data_model(input), MsgModelDefinition::default())
    }

    #[test]
    fn test_basic_types() {
        let mut input = Msg::default();
        input.title = "test".to_string();
        let name = "name".to_string();
        let m_type = "float32".to_string();
        input.tokens.push((m_type.clone(), name.clone()));

        let mut result = MsgModelDefinition::default();
        
        let mut member = MsgMember::default();
        member.member_name = name.clone();
        member.member_type = m_type.clone();

        result.members.push(member);
        result.name = "test".to_string();


        assert_eq!(generate_data_model(input), result)
    }

}

#[cfg(test)]
mod module_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generate_modules() {
        
        let model = MsgModule::default();

        let (out, _) = build_rust_libs_recursive(0, &model);

        assert_eq!("", out);
    }

    #[test]
    fn generate_modules_with_struct() {
        
        let mut model = MsgModule::default();
        let mut msg = MsgModelDefinition::default();
        msg.name = "Test".to_string();
        model.msg_structs.push(msg);
        let (out, _) = build_rust_libs_recursive(0, &model);

        assert_eq!("pub mod  {\n    pub struct Test {\n    }\n}\n", out);
    }

    #[test]
    fn nested_generate_modules_with_child() {
        let mut model = MsgModule::default();
        model.name = "parent".to_string();
        let mut child_model = MsgModule::default();
        child_model.name = "child".to_string();
        let mut msg = MsgModelDefinition::default();
        msg.name = "Test".to_string();
        child_model.msg_structs.push(msg);
        model.modules.push(child_model);
        let (out, _) = build_rust_libs_recursive(0, &model);
        let  expected = "pub mod parent {\npub mod child {\n    pub struct Test {\n    }\n}\n}\n";
        assert_eq!(expected, out);
    }
    #[test]
    fn nested_generate_modules_with_siblings() {
        let mut model = MsgModule::default();
        model.name = "parent".to_string();
        let mut child_model = MsgModule::default();
        child_model.name = "child".to_string();
        let mut msg = MsgModelDefinition::default();
        msg.name = "Test".to_string();
        child_model.msg_structs.push(msg);
        model.modules.push(child_model);

        let mut child_model2 = MsgModule::default();
        child_model2.name = "child2".to_string();
        let mut msg2 = MsgModelDefinition::default();
        msg2.name = "Test2".to_string();
        child_model2.msg_structs.push(msg2);

        model.modules.push(child_model2);

        let (out, _) = build_rust_libs_recursive(0, &model);
        let  expected = "pub mod parent {\npub mod child {\n    pub struct Test {\n    }\n}\npub mod child2 {\n    pub struct Test2 {\n    }\n}\n}\n";

        assert_eq!(expected, out);
    }
}

#[cfg(test)]
mod integration_tests {

    use crate::formatter::field_formatter::generate_rust_source_from_model;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generates_vector3() {
        let vec3_definition = String::from("# This represents a vector in free space.\n# It is only meant to represent a direction. Therefore, it does not\n# make sense to apply a translation to it (e.g., when applying a \n# generic rigid transformation to a Vector3, tf2 will only apply the\n# rotation). If you want your data to be translatable too, use the\n# geometry_msgs/Point message instead.\n\nfloat64 x\nfloat64 y\nfloat64 z");
                    // tokenize msg descriptors
        let msg = tokenize_source("Vector3".to_string(), vec3_definition).unwrap();
        // generate msg definitions
        let model = generate_data_model(msg);

        // generate source
        let source = generate_rust_source_from_model(&model, 0).unwrap();

        assert_eq!(source, String::from("pub struct Vector3 {\n    x: f64,\n    y: f64,\n    z: f64,\n}"))
    }

    #[test]
    fn generates_vector3_windows() {
        let vec3_definition = String::from("# This represents a vector in free space.\n# It is only meant to represent a direction. Therefore, it does not\r\n# make sense to apply a translation to it (e.g., when applying a \r\n# generic rigid transformation to a Vector3, tf2 will only apply the\r\n# rotation). If you want your data to be translatable too, use the\r\n# geometry_msgs/Point message instead.\r\n\r\nfloat64 x\r\nfloat64 y\r\nfloat64 z");
                    // tokenize msg descriptors
        let msg = tokenize_source("Vector3".to_string(), vec3_definition).unwrap();
        // generate msg definitions
        let model = generate_data_model(msg);

        // generate source
        let source = generate_rust_source_from_model(&model, 0).unwrap();

        assert_eq!(source, String::from("pub struct Vector3 {\n    x: f64,\n    y: f64,\n    z: f64,\n}"))
    }
}



#[cfg(test)]
mod handle_token_line {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn returns_none_on_empty_line() {
        assert_eq!(handle_token_line(""), None);
    }

    
    #[test]
    fn handles_line_with_assignment() {
        let expected = Some(("uint8".to_string(), "BOX_Z".to_string()));
        assert_eq!(expected, handle_token_line("uint8 BOX_Z=2"));
    }
}

#[cfg(test)]
mod tokenizer_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_source_tokenizer() {
        let source = String::from("");
        assert_eq!(tokenize_source("".to_string(), source), None)
    }

    #[test]
    fn test_tokenizer() {
        let source = String::from(
            "# This expresses velocity in free space broken into its linear and angular parts.\nVector3  linear\nVector3  angular",
        );

        let result = Msg {
            title: "".to_string(),
            tokens: vec![
                ("Vector3".to_string(), "linear".to_string()),
                ("Vector3".to_string(), "angular".to_string()),
            ],
        };

        assert_eq!(tokenize_source("".to_string(), source), Some(result))
    }

    #[test]
    fn test_tokenizer_with_double_spaced_comments() {
        let source = String::from(
            "# This  expresses velocity in free space broken into its linear and angular parts.\nVector3  linear\nVector3  angular",
        );

        let result = Msg {
            title: "".to_string(),
            tokens: vec![
                ("Vector3".to_string(), "linear".to_string()),
                ("Vector3".to_string(), "angular".to_string()),
            ],
        };

        assert_eq!(tokenize_source("".to_string(), source), Some(result))
    }

    #[test]
    fn test_tokenizer_with_single_spaced_comments() {
        let source = String::from(
            "# This represents a vector in free space.\n# It is only meant to represent a direction. Therefore, it does not\n# make sense to apply a translation to it (e.g., when applying a \n# generic rigid transformation to a Vector3, tf2 will only apply the\n# rotation). If you want your data to be translatable too, use the\n# geometry_msgs/Point message instead.\n\nfloat64 x\nfloat64 y\nfloat64 z"
        );

        let result = Msg {
            title: "".to_string(),
            tokens: vec![
                ("float64".to_string(), "x".to_string()),
                ("float64".to_string(), "y".to_string()),
                ("float64".to_string(), "z".to_string()),
            ],
        };

        assert_eq!(tokenize_source("".to_string(), source), Some(result))
    }

    #[test]
    fn test_tokenizer_with_goal_status() {
        let source = String::from(
"GoalID goal_id
uint8 status
uint8 PENDING         = 0   # The goal has yet to be processed by the action server
uint8 ACTIVE          = 1   # The goal is currently being processed by the action server
uint8 PREEMPTED       = 2   # The goal received a cancel request after it started executing
                            #   and has since completed its execution (Terminal State)
uint8 SUCCEEDED       = 3   # The goal was achieved successfully by the action server (Terminal State)
uint8 ABORTED         = 4   # The goal was aborted during execution by the action server due
                            #    to some failure (Terminal State)
uint8 REJECTED        = 5   # The goal was rejected by the action server without being processed,
                            #    because the goal was unattainable or invalid (Terminal State)
uint8 PREEMPTING      = 6   # The goal received a cancel request after it started executing
                            #    and has not yet completed execution
uint8 RECALLING       = 7   # The goal received a cancel request before it started executing,
                            #    but the action server has not yet confirmed that the goal is canceled
uint8 RECALLED        = 8   # The goal received a cancel request before it started executing
                            #    and was successfully cancelled (Terminal State)
uint8 LOST            = 9   # An action client can determine that a goal is LOST. This should not be
                            #    sent over the wire by an action server

#Allow for the user to associate a string with GoalStatus for debugging
string text

");

        let expected = Msg {
            title: "".to_string(),
            tokens: vec![
                ("GoalID".to_string(), "goal_id".to_string()),
                ("uint8".to_string(), "status".to_string()),
                ("uint8".to_string(), "PENDING".to_string()),
                ("uint8".to_string(), "ACTIVE".to_string()),
                ("uint8".to_string(), "PREEMPTED".to_string()),
                ("uint8".to_string(), "SUCCEEDED".to_string()),
                ("uint8".to_string(), "ABORTED".to_string()), 
                ("uint8".to_string(), "REJECTED".to_string()),   
                ("uint8".to_string(), "PREEMPTING".to_string()),  
                ("uint8".to_string(), "RECALLING".to_string()),
                ("uint8".to_string(), "RECALLED".to_string()),
                ("uint8".to_string(), "LOST".to_string()),
                ("string".to_string(), "text".to_string()),
            ],
        };
        let result = tokenize_source("".to_string(), source);
        assert_eq!(result, Some(expected))
    }

    
    #[test]
    fn test_tokenizer_with_solid_primitve() {
        let source = String::from(
"# Define box, sphere, cylinder, cone 
# All shapes are defined to have their bounding boxes centered around 0,0,0.

uint8 BOX=1
uint8 SPHERE=2
uint8 CYLINDER=3
uint8 CONE=4

# The type of the shape
uint8 type


# The dimensions of the shape
float64[] dimensions

# The meaning of the shape dimensions: each constant defines the index in the 'dimensions' array

# For the BOX type, the X, Y, and Z dimensions are the length of the corresponding
# sides of the box.
uint8 BOX_X=0
uint8 BOX_Y=1
uint8 BOX_Z=2


# For the SPHERE type, only one component is used, and it gives the radius of
# the sphere.
uint8 SPHERE_RADIUS=0


# For the CYLINDER and CONE types, the center line is oriented along
# the Z axis.  Therefore the CYLINDER_HEIGHT (CONE_HEIGHT) component
# of dimensions gives the height of the cylinder (cone).  The
# CYLINDER_RADIUS (CONE_RADIUS) component of dimensions gives the
# radius of the base of the cylinder (cone).  Cone and cylinder
# primitives are defined to be circular. The tip of the cone is
# pointing up, along +Z axis.

uint8 CYLINDER_HEIGHT=0
uint8 CYLINDER_RADIUS=1

uint8 CONE_HEIGHT=0
uint8 CONE_RADIUS=1
");

        let expected = Msg {
            title: "".to_string(),
            tokens: vec![
                ("uint8".to_string(), "BOX".to_string()),
                ("uint8".to_string(), "SPHERE".to_string()),
                ("uint8".to_string(), "CYLINDER".to_string()),
                ("uint8".to_string(), "CONE".to_string()),
                ("uint8".to_string(), "type".to_string()),
                ("float64[]".to_string(),  "dimensions".to_string()),
                ("uint8".to_string(), "BOX_X".to_string()),
                ("uint8".to_string(), "BOX_Y".to_string()),
                ("uint8".to_string(), "BOX_Z".to_string()),
                ("uint8".to_string(), "SPHERE_RADIUS".to_string()),
                ("uint8".to_string(), "CYLINDER_HEIGHT".to_string()),
                ("uint8".to_string(), "CYLINDER_RADIUS".to_string()),
                ("uint8".to_string(), "CONE_HEIGHT".to_string()),
                ("uint8".to_string(), "CONE_RADIUS".to_string()),
            ],
        };
        let result = tokenize_source("".to_string(), source);
        assert_eq!(result, Some(expected))
    }
}

