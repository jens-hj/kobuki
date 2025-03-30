#[derive(Debug, PartialEq)]
pub struct MsgDTO {
    pub title: String,
    pub tokens: Vec<(String, String)>,
}

impl MsgDTO {
    pub fn default() -> Self {
        Self {
            title: String::default(),
            tokens: vec![],
        }
    }
}

/**
 *  New
 **/
 #[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub value: String
}

impl Field {
    pub fn default() -> Self {
        Self { 
            name: String::default(), 
            value: String::default()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constant {
}

impl Constant {
    pub fn default() -> Self {
        Self {  }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Msg {
    pub name: String,
    pub fields: Vec<Field>,
    pub constants: Vec<Constant>
}

impl Msg {
    pub fn default() -> Self {
        Self {
            name: String::default(), 
            fields: vec![], 
            constants: vec![] 
        }
    }
}

pub struct Package {
    pub name: String,
    pub packages: Vec<Package>,
    pub messages: Vec<Msg> 
}

impl Package {
    pub fn default() -> Self {
        Self { 
            name: String::default(), 
            packages: vec![], 
            messages: vec![] 
        }
    }

    pub fn build_default_message() -> Msg {
        let mut header = Msg::default();
        header.name = "Header".to_string();

        let mut timestamp_field = Field::default();

        timestamp_field.name = "time_stamp".to_string();
        timestamp_field.value = "time".to_string();

        header.fields.push(timestamp_field);

        let mut frame_id_field =Field::default();

        frame_id_field.name = "frame_id".to_string();
        frame_id_field.value = "string".to_string();

        //string frame_id
        //builtin_interfaces/Time stamp

        header.fields.push(frame_id_field);
        header

    }
    
} 