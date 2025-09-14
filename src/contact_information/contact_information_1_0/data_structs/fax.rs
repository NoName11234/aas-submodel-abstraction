use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub struct Fax {
    fax_number: HashMap<String, String>,
    type_of_fax_number: Option<TypeOfFaxNumber>
}

impl Fax {
    pub fn new() -> Fax {
        Fax {
            fax_number: HashMap::new(),
            type_of_fax_number: None
        }
    }

    pub fn set_fax_number(&mut self, fax_number: HashMap<String, String>) {
        self.fax_number = fax_number;
    }

    pub fn get_fax_number(&self) -> &HashMap<String, String> {
        &self.fax_number
    }

    pub fn add_fax_number(&mut self, language: String, fax_number: String) {
        self.fax_number.insert(language, fax_number);
    }

    pub fn remove_fax_number(&mut self, language: &String) -> Option<String> {
        self.fax_number.remove(language)
    }

    pub fn set_type_of_fax_number(&mut self, fax_number: TypeOfFaxNumber) {
        self.type_of_fax_number = Some(fax_number);
    }

    pub fn get_type_of_fax_number(&self) -> Option<&TypeOfFaxNumber> {
        self.type_of_fax_number.as_ref()
    }
}

#[derive(PartialEq, Clone)]
pub enum TypeOfFaxNumber {
    Office,
    Secretary,
    Home
}

impl TypeOfFaxNumber {
    pub fn get_semantic_id(&self) -> String {
        match self {
            TypeOfFaxNumber::Office => String::from("0173-1#07-AAS754#001"),
            TypeOfFaxNumber::Secretary => String::from("0173-1#07-AAS756#001"),
            TypeOfFaxNumber::Home => String::from("0173-1#07-AAS758#001"),
        }
    }

    pub fn from_semantic_id(semantic_id: String) -> Option<TypeOfFaxNumber> {
        match semantic_id.as_str() {
            "0173-1#07-AAS754#001" => Some(TypeOfFaxNumber::Office),
            "0173-1#07-AAS756#001" => Some(TypeOfFaxNumber::Secretary),
            "0173-1#07-AAS758#001" => Some(TypeOfFaxNumber::Home),
            _ => None
        }
    }
}