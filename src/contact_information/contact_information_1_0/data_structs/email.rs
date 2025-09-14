use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub struct Email {
    email_address: String,
    public_key: HashMap<String, String>,
    type_of_email_address: Option<TypeOfEmailAddress>,
    type_of_public_key: HashMap<String, String>
}

impl Email {
    pub fn new(email_address: String) -> Email {
        Email {
            email_address,
            public_key: HashMap::new(),
            type_of_email_address: None,
            type_of_public_key: HashMap::new()
        }
    }

    pub fn set_email_address(&mut self, email_address: String) {
        self.email_address = email_address;
    }

    pub fn get_email_address(&self) -> &String {
        &self.email_address
    }

    pub fn set_public_key(&mut self, public_key: HashMap<String, String>) {
        self.public_key = public_key;
    }

    pub fn get_public_key(&self) -> &HashMap<String, String> {
        &self.public_key
    }

    pub fn add_public_key(&mut self, language: String, public_key: String) {
        self.public_key.insert(language, public_key);
    }

    pub fn remove_public_key(&mut self, language: String) -> Option<String> {
        self.public_key.remove(&language)
    }

    pub fn set_type_of_email_address(&mut self, type_of_email_address: TypeOfEmailAddress) {
        self.type_of_email_address = Some(type_of_email_address);
    }

    pub fn get_type_of_email_address(&self) -> Option<&TypeOfEmailAddress> {
        self.type_of_email_address.as_ref()
    }

    pub fn set_type_of_public_key(&mut self, type_of_public_key: HashMap<String, String>) {
        self.type_of_public_key = type_of_public_key;
    }

    pub fn get_type_of_public_key(&self) -> &HashMap<String, String> {
        &self.type_of_public_key
    }

    pub fn add_type_of_public_key(&mut self, language: String, type_of_public_key: String) {
        self.type_of_public_key.insert(language, type_of_public_key);
    }

    pub fn remove_type_of_public_key(&mut self, language: String) -> Option<String> {
        self.type_of_public_key.remove(&language)
    }
}

#[derive(PartialEq, Clone)]
pub enum TypeOfEmailAddress {
    Office,
    Secretary,
    Substitute,
    Home
}

impl TypeOfEmailAddress {
    pub fn get_semantic_id(&self) -> String {
        match self {
            TypeOfEmailAddress::Office => String::from("0173-1#07-AAS754#001"),
            TypeOfEmailAddress::Secretary => String::from("0173-1#07-AAS756#001"),
            TypeOfEmailAddress::Substitute => String::from("0173-1#07-AAS757#001"),
            TypeOfEmailAddress::Home => String::from("0173-1#07-AAS758#001")
        }
    }

    pub fn from_semantic_id(semantic_id: &String) -> Option<TypeOfEmailAddress> {
        match semantic_id.as_str() {
            "0173-1#07-AAS754#001" => Some(TypeOfEmailAddress::Office),
            "0173-1#07-AAS756#001" => Some(TypeOfEmailAddress::Secretary),
            "0173-1#07-AAS757#001" => Some(TypeOfEmailAddress::Substitute),
            "0173-1#07-AAS758#001" => Some(TypeOfEmailAddress::Home),
            _ => None
        }
    }
}