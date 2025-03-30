use crate::model::{msg::artifacts::Field, rust::artifacts::StructMember};



pub fn field_to_member(field: &Field) -> StructMember {
    let mut member = StructMember::default();

    member.name = map_name(&field.name);
    member._type = map_type(&field.value);

    member
}

/*
#[cfg(test)]
mod msg_model_to_rust_source_tests {
    use crate::model::msg::artifacts::Field;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_source_generator() {
        let input = Msg::default();
        assert_eq!(msg_model_to_rust_source(&input, 0), Err(MsgErrorCode::ModelDefMissingName))
    }

    #[test]
    fn test_basic_types() {
        let mut input = Msg::default();
        input.name = String::from("Test");
        assert_eq!(msg_model_to_rust_source(&input, 0).unwrap(), String::from("pub struct Test {\n}"))
    }

    #[test]
    fn test_basic_struct_generation() {
        let mut input = Msg::default();
        input.name = String::from("Test");
        
        let mut  msg_member = Field::default();
        msg_member.name = "test_name".to_string();
        msg_member.value = "member_type".to_string();
        input.fields.push(msg_member.clone());
        input.fields.push(msg_member);
        assert_eq!(msg_model_to_rust_source(&input, 0).unwrap(), String::from("pub struct Test {\n    test_name: member_type,\n    test_name: member_type,\n}"))
    }

    #[test]
    fn test_basic_struct_generation_with_tab_2() {
        let mut input = Msg::default();
        input.name = String::from("Test");
        
        let mut  msg_member = Field::default();
        msg_member.name = "test_name".to_string();
        msg_member.value = "member_type".to_string();
        input.fields.push(msg_member.clone());
        input.fields.push(msg_member);
        assert_eq!(msg_model_to_rust_source(&input, 2).unwrap(), String::from("  pub struct Test {\n      test_name: member_type,\n      test_name: member_type,\n  }"))
    }

    
    #[test]
    fn test_basic_struct_generation_with_tab_4() {
        let mut input = Msg::default();
        input.name = String::from("Test");
        
        let mut  msg_member = Field::default();
        msg_member.name = "test_name".to_string();
        msg_member.value = "member_type".to_string();
        input.fields.push(msg_member.clone());
        input.fields.push(msg_member);
        assert_eq!(msg_model_to_rust_source(&input, 4).unwrap(), String::from("    pub struct Test {\n        test_name: member_type,\n        test_name: member_type,\n    }"))
    }

}
 */

// maps given type to primitive if it exists
fn map_type(m_type: &String) -> String {
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
        "uint16" => "u16".to_string(),
        "uint32" => "u32".to_string(),
        "uint64" => "u64".to_string(),
        "string" => "String".to_string(),

        //TODO: how to handle times
        "time" => "usize".to_string(),
        "duration" => "usize".to_string(),
        
        _ => underlying_type
    };

    let mut out = primitive_type;

    if is_array {
        out = format!("Vec<{}>", out)
    }

    // remove slash qualified
    out = out.replace("/", "::");

    out

}

fn map_name(name: &String) -> String {
    match name.as_str() {
        "type"  => "_type".to_string(),
        _ => name.clone()
    }
}



#[cfg(test)]
mod map_type_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn map_typ_float64() {
        let input = "float64".to_string();
        let out = map_type(&input);
        assert_eq!(out, "f64".to_string())
    }

    #[test]
    fn map_type_float64_array() {
        let input = "float64[]".to_string();
        let out = map_type(&input);
        assert_eq!(out, "Vec<f64>".to_string())
    }
    #[test]
    fn map_type_type_array() {
        let input = "Point32[]".to_string();
        let out = map_type(&input);
        assert_eq!(out, "Vec<Point32>".to_string())
    }
}

