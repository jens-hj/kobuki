use std::{fs, path::Path};

use crate::model::msg::artifacts::{Field, Msg, MsgDTO};

pub struct MsgParser;

impl MsgParser {
    pub fn default() -> Self {
        Self
    }

    pub fn parse(&self, data: RosMsgFileMetaData, source: String) -> Msg {
        let msg = tokenize_source(data.name, source).unwrap();
        generate_data_model(msg)
    }
}

pub enum MsgType {
    MSG
}

pub struct RosMsgFileMetaData {
    pub name: String,
    pub file_type: Option<MsgType>,
}

impl RosMsgFileMetaData {
    pub fn default() -> Self {
        Self { 
            file_type: None,
            name: String::default() 
        }
    }
}

pub fn get_metadata(file: &Path) -> RosMsgFileMetaData {
    let mut data = RosMsgFileMetaData::default();

    let file_name: String = String::from(file.file_name().to_owned().unwrap().to_str().unwrap());
    let msg_name: String = file_name.replace(".msg", "");
    
    data.name = msg_name;

    data.file_type = match file.extension() {
        Some(x) => {
            match x.to_str().unwrap() {
                "msg" => Some(MsgType::MSG),  
                _ =>  None
            }
        }
        None => None
    };


    data
}


pub fn read_file(file: &Path) -> String {

    fs::read_to_string(file.as_os_str()).expect("Unable to read file")
}

fn handle_token_line(line: &str) -> Option<(String, String)> {
    // remove tabs
    let formatted_line = line.replace("\t", " ");
    let tokens = formatted_line.split(" ").filter(|&elem|  { !elem.is_empty() }).collect::<Vec<&str>>();

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

fn tokenize_source(name: String, source: String) -> Option<MsgDTO> {
    if source.is_empty() {
        return None;
    };

    let mut msg = MsgDTO::default();

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

fn generate_data_model(msg: MsgDTO) -> Msg {
    let mut out = Msg::default();
    out.name = msg.title;

    for token in msg.tokens {
        let name = token.1;
        let m_type = token.0;

        let mut msg_mem = Field::default();

        msg_mem.name = name;
        msg_mem.value= m_type;
        
        out.fields.push(msg_mem);
    }

    out
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

    use crate::model::msg::artifacts::MsgDTO;

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

        let result = MsgDTO {
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

        let result = MsgDTO {
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

        let result = MsgDTO {
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

        let expected = MsgDTO {
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

        let expected = MsgDTO {
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

/*
TODO: test bring the tests back

#[cfg(test)]
mod integration_tests {

    use crate::formatter::field_formatter::msg_model_to_rust_source;

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
        let source = msg_model_to_rust_source(&model, 0).unwrap();

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
        let source = msg_model_to_rust_source(&model, 0).unwrap();

        assert_eq!(source, String::from("pub struct Vector3 {\n    x: f64,\n    y: f64,\n    z: f64,\n}"))
    }
}




#[cfg(test)]
mod generate_data_model {

    use crate::model::msg::artifacts::{MsgDTO, Field, Msg};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty_generate_data_model() {
        let input = MsgDTO::default();
        assert_eq!(generate_data_model(input), Msg::default())
    }

    #[test]
    fn test_basic_types() {
        let mut input = MsgDTO::default();
        input.title = "test".to_string();
        let name = "name".to_string();
        let m_type = "float32".to_string();
        input.tokens.push((m_type.clone(), name.clone()));

        let mut result = Msg::default();
        
        let mut member = Field::default();
        member.name = name.clone();
        member.value = m_type.clone();

        result.fields.push(member);
        result.name = "test".to_string();


        assert_eq!(generate_data_model(input), result)
    }

}


 */