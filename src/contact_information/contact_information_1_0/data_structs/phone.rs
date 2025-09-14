use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub struct Phone {
    telephone_number: HashMap<String, String>,
    type_of_phone: Option<TypeOfTelephone>,
    available_time: HashMap<String, String>
}

impl Phone {
    pub fn new() -> Phone {
        Phone {
            telephone_number: HashMap::new(),
            type_of_phone: None,
            available_time: HashMap::new()
        }
    }

    pub fn set_telephone_number(&mut self, phone_number: HashMap<String, String>) {
        self.telephone_number = phone_number;
    }

    pub fn get_telephone_number(&self) -> &HashMap<String, String> {
        &self.telephone_number
    }

    pub fn add_telephone_number(&mut self, language: String, telephone_number: String) {
        self.telephone_number.insert(language, telephone_number);
    }

    pub fn remove_telephone_number(&mut self, language: &String) {
        self.telephone_number.remove(language);
    }

    pub fn set_type_of_phone(&mut self, type_of_telephone: TypeOfTelephone) {
        self.type_of_phone = Some(type_of_telephone);
    }

    pub fn get_type_of_phone(&self) -> Option<&TypeOfTelephone> {
        self.type_of_phone.as_ref()
    }

    pub fn set_available_time(&mut self, available_time: HashMap<String, String>) {
        self.available_time = available_time;
    }

    pub fn get_available_time(&self) -> &HashMap<String, String> {
        &self.available_time
    }

    pub fn add_available_time(&mut self, language: String, available_time: String) {
        self.available_time.insert(language, available_time);
    }

    pub fn remove_available_time(&mut self, language: &String) {
        self.available_time.remove(language);
    }
}

#[derive(PartialEq, Clone)]
pub enum TypeOfTelephone {
    Office,
    OfficeMobile,
    Secretary,
    Substitute,
    Home,
    PrivateMobile
}

impl TypeOfTelephone {
    pub fn get_semantic_id(&self) -> String {
        match self {
            TypeOfTelephone::Office => String::from("0173-1#07-AAS754#001"),
            TypeOfTelephone::OfficeMobile => String::from("0173-1#07-AAS755#001"),
            TypeOfTelephone::Secretary => String::from("0173-1#07-AAS756#001"),
            TypeOfTelephone::Substitute => String::from("0173-1#07-AAS757#001"),
            TypeOfTelephone::Home => String::from("0173-1#07-AAS758#001"),
            TypeOfTelephone::PrivateMobile => String::from("0173-1#07-AAS759#001")
        }
    }

    pub fn from_semantic_id(semantic_id: &String) -> Option<TypeOfTelephone> {
        match semantic_id.as_str() {
            "0173-1#07-AAS754#001" => Some(TypeOfTelephone::Office),
            "0173-1#07-AAS755#001" => Some(TypeOfTelephone::OfficeMobile),
            "0173-1#07-AAS756#001" => Some(TypeOfTelephone::Secretary),
            "0173-1#07-AAS757#001" => Some(TypeOfTelephone::Substitute),
            "0173-1#07-AAS758#001" => Some(TypeOfTelephone::Home),
            "0173-1#07-AAS759#001" => Some(TypeOfTelephone::PrivateMobile),
            _ => None
        }
    }
}