use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub struct IpCommunication {
    id_short: String,
    address_of_additional_link: String,
    type_of_communication: Option<String>,
    available_time: HashMap<String, String>
}

impl IpCommunication {
    pub fn new(id_short: String, address_of_additional_link: String) -> IpCommunication {
        IpCommunication {
            id_short,
            address_of_additional_link,
            type_of_communication: None,
            available_time: HashMap::new()
        }
    }

    pub fn set_id_short(&mut self, id_short: String) {
        self.id_short = id_short;
    }

    pub fn get_id_short(&self) -> &String {
        &self.id_short
    }

    pub fn set_address_of_additional_link(&mut self, address_of_additional_link: String) {
        self.address_of_additional_link = address_of_additional_link;
    }

    pub fn get_address_of_additional_link(&self) -> &String {
        &self.address_of_additional_link
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

    pub fn remove_available_time(&mut self, language: &String) -> Option<String> {
        self.available_time.remove(language)
    }
}