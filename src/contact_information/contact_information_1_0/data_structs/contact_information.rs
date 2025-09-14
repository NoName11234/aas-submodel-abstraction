use std::{collections::HashMap};

use crate::contact_information::contact_information_1_0::data_structs::{email::Email, fax::Fax, ip_communication::{self, IpCommunication}, phone::Phone};

/// A struct representing a submodel element collection of type contact information which contains information on how to
/// contact the manufacturer or an authorised service provider, e.g. when a maintenance service is required.
#[derive(PartialEq, Clone)]
pub struct ContactInformation {
    role_of_contact_person: Option<RoleOfContactPerson>,
    national_code: HashMap<String, String>,
    languages: Vec<Language>,
    time_zone: Option<String>,
    city_town: HashMap<String, String>,
    company: HashMap<String, String>,
    department: HashMap<String, String>,
    phone: Option<Phone>,
    fax: Option<Fax>,
    email: Option<Email>,
    ip_communication: Vec<IpCommunication>,
    street: HashMap<String, String>,
    zipcode: HashMap<String, String>,
    pobox: HashMap<String, String>,
    zip_code_of_pobox: HashMap<String, String>,
    state_county: HashMap<String, String>,
    name_of_contact: HashMap<String, String>,
    first_name: HashMap<String, String>,
    middle_names: HashMap<String, String>,
    title: HashMap<String, String>,
    academic_title: HashMap<String, String>,
    further_details_of_contact: HashMap<String, String>,
    address_of_aditional_link: Option<String>
}

impl ContactInformation {
    /// Creates a new empty instance of the struct.
    pub fn new() -> ContactInformation {
        ContactInformation {
            role_of_contact_person: None,
            national_code: HashMap::new(),
            languages: Vec::new(),
            time_zone: None,
            city_town: HashMap::new(),
            company: HashMap::new(),
            department: HashMap::new(),
            phone: None,
            fax: None,
            email: None,
            ip_communication: Vec::new(),
            street: HashMap::new(),
            zipcode: HashMap::new(),
            pobox: HashMap::new(),
            zip_code_of_pobox: HashMap::new(),
            state_county: HashMap::new(),
            name_of_contact: HashMap::new(),
            first_name: HashMap::new(),
            middle_names: HashMap::new(),
            title: HashMap::new(),
            academic_title: HashMap::new(),
            further_details_of_contact: HashMap::new(),
            address_of_aditional_link: None
        }
    }

    pub fn set_role_of_contact_person(&mut self, role_of_contact_person: RoleOfContactPerson) {
        self.role_of_contact_person = Some(role_of_contact_person);
    }

    pub fn get_role_of_contact_person(&self) -> Option<&RoleOfContactPerson> {
        self.role_of_contact_person.as_ref()
    }

    pub fn set_national_code(&mut self, national_code: HashMap<String, String>) {
        self.national_code = national_code;
    }

    pub fn get_national_code(&self) -> &HashMap<String, String> {
        &self.national_code
    }

    pub fn add_national_code(&mut self, language: String, national_code: String){
        self.national_code.insert(language, national_code);
    }

    pub fn remove_national_code(&mut self, language: &String) -> Option<String> {
        self.national_code.remove(language)
    }

    pub fn set_languages(&mut self, languages: Vec<Language>) {
        self.languages = languages;
    }

    pub fn get_languages(&self) -> &Vec<Language> {
        &self.languages
    }

    pub fn add_language(&mut self, language: Language) {
        self.languages.push(language);
    }

    pub fn remove_language(&mut self, index: usize) -> Language {
        self.languages.remove(index)
    }

    pub fn set_time_zone(&mut self, time_zone: String) {
        self.time_zone = Some(time_zone);
    }

    pub fn get_time_zone(&self) -> Option<&String> {
        self.time_zone.as_ref()
    }

    pub fn set_city_town(&mut self, city_town: HashMap<String, String>) {
        self.city_town = city_town;
    }

    pub fn get_city_town(&self) -> &HashMap<String, String> {
        &self.city_town
    }

    pub fn add_city_town(&mut self, language: String, city_town: String) {
        self.city_town.insert(language, city_town);
    }

    pub fn remove_city_town(&mut self, language: &String) -> Option<String> {
        self.city_town.remove(language)
    }

    pub fn set_company(&mut self, company: HashMap<String, String>) {
        self.company = company;
    }

    pub fn get_company(&self) -> &HashMap<String, String> {
        &self.company
    }

    pub fn add_company(&mut self, language: String, company: String) {
        self.company.insert(language, company);
    }

    pub fn remove_company(&mut self, language: &String) -> Option<String> {
        self.company.remove(language)
    }

    pub fn set_department(&mut self, department: HashMap<String, String>) {
        self.department = department;
    }

    pub fn get_department(&self) -> &HashMap<String, String> {
        &self.department
    }

    pub fn add_department(&mut self, language: String, department: String) {
        self.department.insert(language, department);
    }

    pub fn remove_department(&mut self, language: &String) -> Option<String> {
        self.department.remove(language)
    }

    pub fn set_phone(&mut self, phone: Phone) {
        self.phone = Some(phone);
    }

    pub fn get_phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    pub fn set_fax(&mut self, fax: Fax) {
        self.fax = Some(fax);
    }

    pub fn get_fax(&self) -> Option<&Fax> {
        self.fax.as_ref()
    }

    pub fn set_email(&mut self, email: Email) {
        self.email = Some(email);
    }

    pub fn get_email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    pub fn set_ip_communication(&mut self, ip_communication: Vec<IpCommunication>) {
        self.ip_communication = ip_communication;
    }

    pub fn get_ip_communication(&self) -> &Vec<IpCommunication> {
        &self.ip_communication
    }

    pub fn add_ip_communication(&mut self, ip_communication: IpCommunication) {
        self.ip_communication.push(ip_communication);
    }

    pub fn remove_ip_communication(&mut self, index: usize) -> IpCommunication {
        self.ip_communication.remove(index)
    }

    pub fn set_street(&mut self, street: HashMap<String, String>) {
        self.street = street;
    }

    pub fn get_street(&self) -> &HashMap<String, String> {
        &self.street
    }

    pub fn add_street(&mut self, language: String, street: String) {
        self.street.insert(language, street);
    }

    pub fn remove_street(&mut self, language: &String) -> Option<String> {
        self.street.remove(language)
    }

    pub fn set_zipcode(&mut self, zipcode: HashMap<String, String>) {
        self.zipcode = zipcode;
    }

    pub fn get_zipcode(&self) -> &HashMap<String, String> {
        &self.zipcode
    }

    pub fn add_zipcode(&mut self, language: String, zipcode: String) {
        self.zipcode.insert(language, zipcode);
    }

    pub fn remove_zipcode(&mut self, language: &String) -> Option<String> {
        self.zipcode.remove(language)
    }

    pub fn set_pobox(&mut self, pobox: HashMap<String, String>) {
        self.pobox = pobox;
    }

    pub fn get_pobox(&self) -> &HashMap<String, String> {
        &self.pobox
    }

    pub fn add_pobox(&mut self, language: String, pobox: String) {
        self.pobox.insert(language, pobox);
    }

    pub fn remove_pobox(&mut self, language: &String) -> Option<String> {
        self.pobox.remove(language)
    }

    pub fn set_zipcode_of_pobox(&mut self, zipcode_of_pobox: HashMap<String, String>) {
        self.zip_code_of_pobox = zipcode_of_pobox;
    }

    pub fn get_zipcode_of_pobox(&self) -> &HashMap<String, String> {
        &self.zip_code_of_pobox
    }

    pub fn add_zipcode_of_pobox(&mut self, language: String, zipcode_of_pobox: String) {
        self.zip_code_of_pobox.insert(language, zipcode_of_pobox);
    }

    pub fn remove_zipcode_of_pobox(&mut self, language: &String) -> Option<String> {
        self.zip_code_of_pobox.remove(language)
    }

    pub fn set_state_county(&mut self, state_county: HashMap<String, String>) {
        self.state_county = state_county;
    }

    pub fn get_state_county(&self) -> &HashMap<String, String> {
        &self.state_county
    }

    pub fn add_state_county(&mut self, language: String, state_county: String) {
        self.state_county.insert(language, state_county);
    }

    pub fn remove_state_county(&mut self, language: &String) -> Option<String> {
        self.state_county.remove(language)
    }

    pub fn set_name_of_contact(&mut self, name_of_contact: HashMap<String, String>) {
        self.name_of_contact = name_of_contact;
    }

    pub fn get_name_of_contact(&self) -> &HashMap<String, String> {
        &self.name_of_contact
    }

    pub fn add_name_of_contact(&mut self, language: String, name_of_contact: String) {
        self.name_of_contact.insert(language, name_of_contact);
    }

    pub fn remove_name_of_contact(&mut self, language: &String) -> Option<String> {
        self.name_of_contact.remove(language)
    }

    pub fn set_first_name(&mut self, first_name: HashMap<String, String>) {
        self.first_name = first_name;
    }

    pub fn get_first_name(&self) -> &HashMap<String, String> {
        &self.first_name
    }

    pub fn add_first_name(&mut self, language: String, first_name: String) {
        self.first_name.insert(language, first_name);
    }

    pub fn remove_first_name(&mut self, language: &String) -> Option<String> {
        self.first_name.remove(language)
    }

    pub fn set_middle_names(&mut self, middle_names: HashMap<String, String>) {
        self.middle_names = middle_names;
    }

    pub fn get_middle_names(&self) -> &HashMap<String, String> {
        &self.middle_names
    }

    pub fn add_middle_names(&mut self, language: String, middle_names: String) {
        self.middle_names.insert(language, middle_names);
    }

    pub fn remove_middle_names(&mut self, language: &String) -> Option<String> {
        self.middle_names.remove(language)
    }

    pub fn set_title(&mut self, title: HashMap<String, String>) {
        self.title = title;
    }

    pub fn get_title(&self) -> &HashMap<String, String> {
        &self.title
    }

    pub fn add_title(&mut self, language: String, title: String) {
        self.title.insert(language, title);
    }

    pub fn remove_title(&mut self, language: &String) -> Option<String> {
        self.title.remove(language)
    }

    pub fn set_academic_title(&mut self, academic_title: HashMap<String, String>) {
        self.academic_title = academic_title;
    }

    pub fn get_academic_title(&self) -> &HashMap<String, String> {
        &self.academic_title
    }

    pub fn add_academic_title(&mut self, language: String, academic_title: String) {
        self.academic_title.insert(language, academic_title);
    }

    pub fn remove_academic_title(&mut self, language: &String) -> Option<String> {
        self.academic_title.remove(language)
    }

    pub fn set_further_details_of_contact(&mut self, further_details_of_contact: HashMap<String, String>) {
        self.further_details_of_contact = further_details_of_contact;
    }

    pub fn get_further_details_of_contact(&self) -> &HashMap<String, String> {
        &self.further_details_of_contact
    }

    pub fn add_further_details_of_contact(&mut self, language: String, further_details_of_contact: String) {
        self.further_details_of_contact.insert(language, further_details_of_contact);
    }

    pub fn remove_further_details_of_contact(&mut self, language: &String) -> Option<String> {
        self.further_details_of_contact.remove(language)
    }

    pub fn set_address_of_aditional_link(&mut self, address_of_aditional_link: String) {
        self.address_of_aditional_link = Some(address_of_aditional_link);
    }

    pub fn get_address_of_aditional_link(&self) -> Option<&String> {
        self.address_of_aditional_link.as_ref()
    }
}

///The function of a contact person in a process.
#[derive(PartialEq, Clone)]
pub enum RoleOfContactPerson {
    AdministrativeContact,
    CommercialContact,
    OtherContact,
    HazardousGoodsContact,
    TechnicalContact
}

impl RoleOfContactPerson {
    pub fn get_semantic_id(&self) -> String {
        match self {
            RoleOfContactPerson::AdministrativeContact => String::from("0173-1#07-AAS927#001"),
            RoleOfContactPerson::CommercialContact => String::from("0173-1#07-AAS928#001"),
            RoleOfContactPerson::OtherContact => String::from("0173-1#07-AAS929#001"),
            RoleOfContactPerson::HazardousGoodsContact => String::from("0173-1#07-AAS930#001"),
            RoleOfContactPerson::TechnicalContact => String::from("0173-1#07-AAS931#001")
        }
    }

    pub fn from_semantic_id(semantic_id: &String) -> Option<RoleOfContactPerson> {
        match semantic_id.as_str() {
            "173-1#07-AAS927#001" => Some(RoleOfContactPerson::AdministrativeContact),
            "0173-1#07-AAS928#001" => Some(RoleOfContactPerson::CommercialContact),
            "0173-1#07-AAS929#001" => Some(RoleOfContactPerson::OtherContact),
            "0173-1#07-AAS930#001" => Some(RoleOfContactPerson::HazardousGoodsContact),
            "0173-1#07-AAS931#001" => Some(RoleOfContactPerson::TechnicalContact),
            _ => None
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Language {
    short_id: String,
    language: String
}

impl Language {
    pub fn new(short_id: String, language: String) -> Language {
        Language { 
            short_id,
            language
        }
    }

    pub fn set_short_id(&mut self, short_id: String) {
        self.short_id = short_id;
    }

    pub fn get_short_id(&self) -> &String {
        &self.short_id
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }
}