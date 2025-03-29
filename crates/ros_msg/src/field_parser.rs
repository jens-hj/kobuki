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
