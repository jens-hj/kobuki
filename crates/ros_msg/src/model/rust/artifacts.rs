use std::vec;

use crate::formatter::common::{build_padding_string, TAB_SIZE};


/** 
 * TODO: Remove the MsgModule
*/

pub trait Formattable {
    fn format(&self) -> String;
}


// A rust module for ros messages
pub struct Module {
    pub format_params: FormatParameters,
    pub name: String,
    pub dependencies: Vec<String>,
    pub functions: Vec<Struct>,
    pub modules: Vec<Module>,
    pub structs: Vec<Struct>
}

impl Module {
    pub fn default() -> Self {
        Self { 
            format_params: FormatParameters::default(),
            name: String::default(), 
            dependencies: vec![], 
            functions: vec![], 
            modules: vec![], 
            structs: vec![] 
        }
    }

    fn has_elements(&self) -> bool {
        let child_has_elements = self.modules.iter().map(|m| m.has_elements()).reduce(|a,b| a||b);
        match child_has_elements {
            Some(res) => !self.structs.is_empty() || res,
            None => !self.structs.is_empty() 
        }
    }
}

impl Formattable for Module {
    fn format(&self) -> String {
        
        let padding = build_padding_string(self.format_params.padding);
        
        let mut source = format!("{}pub mod {} {{\n", padding, self.name);

        for _struct in &self.structs {
            source = format!("{}{}\n", source, _struct.format());
        }

        for module in &self.modules {
            println!("{}", module.name);
            
            if module.has_elements()  {
                source = format!("{}{}{}", source, padding, module.format());
            }
        }
    
        source = format!("{}{}}}\n", source, padding);

        println!("{}", &self.name);
        if self.has_elements() {
            source
        } else {
            String::default()
        }
    }
}


pub struct FormatParameters {
    padding: usize
}

impl FormatParameters {
    pub fn default() -> Self {
        Self { padding: 0 }
    }

    pub fn new(padding: usize,) -> Self {
        Self { padding }
    }
}

pub struct StructMember {
    pub format: FormatParameters,

    pub _type: String,
    pub name: String
}

impl StructMember {
    pub fn default() -> Self {
        Self { 
            format: FormatParameters::default(),
            _type: String::default(),
            name: String::default() 
        }
    }
}

impl Formattable for StructMember {
    fn format(&self) -> String {
        
        let padding = build_padding_string(self.format.padding);
        let tab_size = build_padding_string(TAB_SIZE);

        format!("{}{}{}: {},\n", padding, tab_size, self.name, self._type)
    }
}


pub struct StructImplementation {

}
impl StructImplementation {
    pub fn default() -> Self {
        Self {  }
    }
}

impl Formattable for StructImplementation {
    fn format(&self) -> String {
        format!("")
    }
}

pub struct StructDerives {

}
impl StructDerives {
    pub fn default() -> Self {
        Self {  }
    }
}

impl Formattable for StructDerives {
    fn format(&self) -> String {
        format!("#[derives()]")
    }
}

pub struct Struct {
    pub format_params: FormatParameters,

    pub derives: StructDerives,
    pub name: String,
    pub members: Vec<StructMember>,
    pub implementation: StructImplementation
}

impl Struct {
    pub fn default() -> Self {
        Self { 
            format_params: FormatParameters::default(),
            derives: StructDerives::default(), 
            name: String::default(), 
            members: vec![], 
            implementation: StructImplementation::default() 
        }
    }
}

impl Formattable for Struct {
    fn format(&self) -> String {

        let mut buf = String::default();
        let padding = build_padding_string(self.format_params.padding);

        buf.push_str(format!("{}pub struct {} {{\n", padding, self.name).as_str());
        
        for member in &self.members {
            buf.push_str(member.format().as_str());
        }
    
        buf.push_str(format!("{}}}", padding).as_str());

        buf
    }
}