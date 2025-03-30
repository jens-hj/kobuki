use crate::model::{msg::artifacts::Msg, rust::artifacts::{FormatParameters, Struct}};

use super::field_formatter::field_to_member;

pub fn msg_to_struct(msg: &Msg, depth: usize) -> Struct {
 
    let mut _struct = Struct::default();

    let format = FormatParameters::new(depth);
    _struct.name = msg.name.clone();

    _struct.format_params = format;
    
    for field in &msg.fields {
        _struct.members.push(field_to_member(field));
    }

    _struct
}
