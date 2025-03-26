use std::fs;


pub fn generate_sources(sources: Vec<&str>, output_path: &str) {

    for source_dir in sources {
        let paths = fs::read_dir(source_dir).unwrap();
        let mut out = String::default();
        for path in paths {
            let entry = path.unwrap();
            let file_name = entry.file_name().into_string().unwrap().clone();
            let source = fs::read_to_string(entry.path().as_os_str()).expect("Unable to read file");

            // generate source
            let source = generate_source(file_name.replace(".msg", ""), source);
            
            out = format!("{}\n\n{}", out, source);
        }

        fs::write(format!("{}/{}.rs", output_path, "types") , out).expect("Unable to write file");

    }
}

fn generate_source(name: String, input: String) -> String {
    let msg = tokenize_source(name, input).unwrap();
    // generate msg definitions
    let model = generate_data_model(msg);
    // generate source
    generate_source_from_model(model).unwrap()
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
        let tokens = line.split(" ").filter(|&elem|  { !elem.is_empty() }).collect::<Vec<&str>>();
        
        println!("{:?}", tokens);
        if tokens.len() < 2 {
            continue;
        }

        if tokens.get(0).unwrap().starts_with("#") {
            continue;
        }

        msg.tokens.push((
            String::from(*tokens.get(0).unwrap()),
            String::from(*tokens.get(1).unwrap()),
        ));
    }

    Some(msg)
}

#[derive(Debug, PartialEq, Clone)]
struct MsgMember {
    member_type: String,
    member_name: String,
}
impl MsgMember {
    fn default() -> Self {
        Self {
            member_type: "".to_string(),
            member_name: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct MsgModelDefinition {
    name: String,
    members: Vec<MsgMember>,
}

impl MsgModelDefinition {
    fn default() -> Self {
        MsgModelDefinition { name: String::default(), members: vec![] }
    }
}

fn generate_data_model(msg: Msg) -> MsgModelDefinition {
    let mut out = MsgModelDefinition::default();
    out.name = msg.title;

    for token in msg.tokens {
        println!("tokens: {:?}", token);
        let name = token.1;
        let m_type = token.0;
        
        let mut msg_mem = MsgMember::default();

        msg_mem.member_name = name;
        msg_mem.member_type = m_type;
        
        out.members.push(msg_mem);
    }


    out
}

#[derive(Debug, PartialEq)]
enum MsgErrorCode {
    ModelDefMissingName
}

// maps given type to primitive if it exists
fn map_type(m_type: String) -> String {

    let mut underlying_type = m_type.clone();

    let mut is_array= false; 
    
    if m_type.contains("[") {
        is_array = true;
        underlying_type = m_type.split("[").collect::<Vec<&str>>().get(0).unwrap().to_string();
    };
    

    let primitive_type = match underlying_type.as_str() {
        "float64" => "f64".to_string(),
        "float32" => "f32".to_string(),
        _ => underlying_type
    };
    
    if is_array {
        return format!("Vec<{}>", primitive_type)
    }

    primitive_type

}

fn generate_source_from_model(model_def: MsgModelDefinition) -> Result<String, MsgErrorCode> {

    if model_def.name.is_empty() { return Err(MsgErrorCode::ModelDefMissingName) } 
    let mut buf = String::default();
    
    buf.push_str(format!("struct {} {{\n", model_def.name).as_str());
    
    for member in model_def.members {
        buf.push_str(format!("    {}: {},\n", member.member_name, map_type(member.member_type)).as_str());
    }

    buf.push_str(format!("}}").as_str());
    
    Ok(buf)
}


#[cfg(test)]
mod source_generation_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_source_generator() {
        let input = MsgModelDefinition::default();
        assert_eq!(generate_source_from_model(input), Err(MsgErrorCode::ModelDefMissingName))
    }

    #[test]
    fn test_basic_types() {
        let mut input = MsgModelDefinition::default();
        input.name = String::from("Test");
        assert_eq!(generate_source_from_model(input).unwrap(), String::from("struct Test {\n}"))
    }

    #[test]
    fn test_basic_struct_generation() {
        let mut input = MsgModelDefinition::default();
        input.name = String::from("Test");
        
        let mut  msg_member = MsgMember::default();
        msg_member.member_name = "test_name".to_string();
        msg_member.member_type = "member_type".to_string();
        input.members.push(msg_member.clone());
        input.members.push(msg_member);
        assert_eq!(generate_source_from_model(input).unwrap(), String::from("struct Test {\n    test_name: member_type,\n    test_name: member_type,\n}"))
    }

}


#[cfg(test)]
mod data_model_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_source_tokenizer() {
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
mod map_type_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn map_typ_float64() {
        let out = map_type("float64".to_string());
        assert_eq!(out, "f64".to_string())
    }

    #[test]
    fn map_type_float64_array() {
        let out = map_type("float64[]".to_string());
        assert_eq!(out, "Vec<f64>".to_string())
    }
    #[test]
    fn map_type_type_array() {
        let out = map_type("Point32[]".to_string());
        assert_eq!(out, "Vec<Point32>".to_string())
    }
}

#[cfg(test)]
mod integration_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generates_vector3() {
        let vec3_definition = String::from("# This represents a vector in free space.\n# It is only meant to represent a direction. Therefore, it does not\n# make sense to apply a translation to it (e.g., when applying a \n# generic rigid transformation to a Vector3, tf2 will only apply the\n# rotation). If you want your data to be translatable too, use the\n# geometry_msgs/Point message instead.\n\nfloat64 x\nfloat64 y\nfloat64 z");
                    // tokenize msg descriptors
        let msg = tokenize_source("Vector3".to_string(), vec3_definition).unwrap();
        // generate msg definitions
        let model = generate_data_model(msg);
        
        println!("{:?}", model.members);
        // generate source
        let source = generate_source_from_model(model).unwrap();

        assert_eq!(source, String::from("struct Vector3 {\n    x: f64,\n    y: f64,\n    z: f64,\n}"))
    }

    #[test]
    fn generates_vector3_windows() {
        let vec3_definition = String::from("# This represents a vector in free space.\n# It is only meant to represent a direction. Therefore, it does not\r\n# make sense to apply a translation to it (e.g., when applying a \r\n# generic rigid transformation to a Vector3, tf2 will only apply the\r\n# rotation). If you want your data to be translatable too, use the\r\n# geometry_msgs/Point message instead.\r\n\r\nfloat64 x\r\nfloat64 y\r\nfloat64 z");
                    // tokenize msg descriptors
        let msg = tokenize_source("Vector3".to_string(), vec3_definition).unwrap();
        // generate msg definitions
        let model = generate_data_model(msg);
        
        println!("{:?}", model.members);
        // generate source
        let source = generate_source_from_model(model).unwrap();

        assert_eq!(source, String::from("struct Vector3 {\n    x: f64,\n    y: f64,\n    z: f64,\n}"))
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
}

