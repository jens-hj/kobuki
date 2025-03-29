#[derive(Debug, PartialEq, Clone)]
pub struct MsgMember {
    pub member_type: String,
    pub member_name: String,
}

impl MsgMember {
    pub fn default() -> Self {
        Self {
            member_type: "".to_string(),
            member_name: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MsgModelDefinition {
    pub name: String,
    pub members: Vec<MsgMember>,
}

impl MsgModelDefinition {
    pub fn default() -> Self {
        MsgModelDefinition { name: String::default(), members: vec![] }
    }
}
