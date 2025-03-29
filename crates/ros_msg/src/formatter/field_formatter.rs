use crate::{common::errors_codes::MsgErrorCode, model::msg::MsgModelDefinition};

use super::common::{build_padding_string, TAB_SIZE};



pub fn generate_rust_source_from_model(model_def: &MsgModelDefinition, depth: usize) -> Result<String, MsgErrorCode> {
    let padding = build_padding_string(depth);
    let tab_padding = build_padding_string(TAB_SIZE);

    if model_def.name.is_empty() { return Err(MsgErrorCode::ModelDefMissingName) }
    
    let mut buf = String::default();
    
    buf.push_str(format!("{}pub struct {} {{\n", padding, model_def.name).as_str());
    
    for member in &model_def.members {
        let member_type = member.member_type.clone();
        buf.push_str(format!("{}{}{}: {},\n", padding, tab_padding, map_name(member.member_name.clone()), map_type(member_type)).as_str());
    }

    buf.push_str(format!("{}}}", padding).as_str());
    
    Ok(buf)
}


// maps given type to primitive if it exists
fn map_type(m_type: String) -> String {
    let mut out = String:: default();
    let mut underlying_type = m_type.clone();

    let mut is_array= false; 
    
    // TODO improve to include length
    if m_type.contains("[") {
        is_array = true;
        underlying_type = m_type.split("[").collect::<Vec<&str>>().get(0).unwrap().to_string();
    };
    

    let primitive_type = match underlying_type.as_str() {
        "float64" => "f64".to_string(),
        "float32" => "f32".to_string(),
        "int8" => "i8".to_string(),
        "int16" => "i16".to_string(),
        "int32" => "i32".to_string(),
        "uint8" | "byte" | "char" => "u8".to_string(),
        "uint32" => "u32".to_string(),
        "uint64" => "u64".to_string(),
        "string" => "String".to_string(),
        "time" => "usize".to_string(),
        //TODO: figure out what to do with headers
        "Header" => "usize".to_string(),
        
        _ => underlying_type
    };

    out = primitive_type;

    if is_array {
        out = format!("Vec<{}>", out)
    }

    // remove slash qualified
    out = out.replace("/", "::");

    out

}

fn map_name(name: String) -> String {
    match name.as_str() {
        "type"  => "_type".to_string(),
        _ => name
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
mod generate_rust_source_from_model {

    use crate::model::msg::MsgMember;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_source_generator() {
        let input = MsgModelDefinition::default();
        assert_eq!(generate_rust_source_from_model(&input, 0), Err(MsgErrorCode::ModelDefMissingName))
    }

    #[test]
    fn test_basic_types() {
        let mut input = MsgModelDefinition::default();
        input.name = String::from("Test");
        assert_eq!(generate_rust_source_from_model(&input, 0).unwrap(), String::from("pub struct Test {\n}"))
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
        assert_eq!(generate_rust_source_from_model(&input, 0).unwrap(), String::from("pub struct Test {\n    test_name: member_type,\n    test_name: member_type,\n}"))
    }

    #[test]
    fn test_basic_struct_generation_with_tab_2() {
        let mut input = MsgModelDefinition::default();
        input.name = String::from("Test");
        
        let mut  msg_member = MsgMember::default();
        msg_member.member_name = "test_name".to_string();
        msg_member.member_type = "member_type".to_string();
        input.members.push(msg_member.clone());
        input.members.push(msg_member);
        assert_eq!(generate_rust_source_from_model(&input, 2).unwrap(), String::from("  pub struct Test {\n      test_name: member_type,\n      test_name: member_type,\n  }"))
    }

    
    #[test]
    fn test_basic_struct_generation_with_tab_4() {
        let mut input = MsgModelDefinition::default();
        input.name = String::from("Test");
        
        let mut  msg_member = MsgMember::default();
        msg_member.member_name = "test_name".to_string();
        msg_member.member_type = "member_type".to_string();
        input.members.push(msg_member.clone());
        input.members.push(msg_member);
        assert_eq!(generate_rust_source_from_model(&input, 4).unwrap(), String::from("    pub struct Test {\n        test_name: member_type,\n        test_name: member_type,\n    }"))
    }

}