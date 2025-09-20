use std::string::String;
use std::ops::Add;
use aas_model_rs::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use aas_model_rs::enumerations::key_type::KeyType;
use aas_model_rs::enumerations::reference_type::ReferenceType;
use aas_model_rs::structs::key::Key;
use aas_model_rs::structs::reference::Reference;
use aas_model_rs::structs::submodel::Submodel;
use aas_model_rs::structs::submodel_element_collection::SubmodelElementCollection;
use aas_model_rs::traits::has_semantics::THasSemantics;
use aas_model_rs::traits::referable::TReferable;
use crate::contact_information::contact_information_1_0::data_structs::contact_information::ContactInformation;
use crate::contact_information::contact_information_1_0::data_structs::email::Email;
use crate::contact_information::contact_information_1_0::data_structs::fax::Fax;
use crate::contact_information::contact_information_1_0::data_structs::ip_communication::IpCommunication;
use crate::contact_information::contact_information_1_0::data_structs::phone::Phone;
use crate::contact_information::contact_information_1_0::template_submodel::get_contact_information_1_0;
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::{ACADEMIC_TITLE, CITY_TOWN, COMPANY, CONTACT_INFORMATION_ADDRESS_OF_ADDITIONAL_LINK, CONTACT_INFORMATION_ID, DEPARTMENT, FIRST_NAME, FURTHER_DETAILS_OF_CONTACT, LANGUAGE, MIDDLE_NAMES, NAME_OF_CONTACT, NATIONAL_CODE, PO_BOX, ROLE_OF_CONTACT_PERSON, STATE_COUNTY, STREET, TIMEZONE, TITLE, ZIPCODE, ZIPCODE_OF_PO_BOX};
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::email::EMAIL_ID;
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::fax::FAX_ID;
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::ip_communication::IP_COMMUNICATION_ID;
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::phone::PHONE_ID;
use crate::submodel_utilities::{convert_hashmap_to_multi_language_text_type_list, get_mutable_mlp, get_mutable_property, get_mutable_smc};

pub struct ContactInformationWriter1_0 {
    submodel: Submodel
}

impl ContactInformationWriter1_0 {
    pub fn new() -> ContactInformationWriter1_0 {
        ContactInformationWriter1_0 {
            submodel: get_contact_information_1_0()
        }
    }

    pub fn get_submodel(&self) -> Submodel {
        self.submodel.clone()
    }

    pub fn add_contact_information(&mut self, contact_information: &ContactInformation) -> Result<(), String> {
        let key = Key::new(KeyType::GlobalReference, String::from(CONTACT_INFORMATION_ID));
        let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

        let optional_contact_information_smc = get_mutable_smc(self.submodel.get_mut_submodel_elements(), &semantic_id);

        if optional_contact_information_smc.is_some() {
            let (_, contact_information_smc) = optional_contact_information_smc.unwrap();
            let mut contact_information_smc = contact_information_smc.clone();

            //map role of contact person property
            let optional_role_of_contact_person = contact_information.get_role_of_contact_person();

            let key = Key::new(KeyType::GlobalReference, String::from(ROLE_OF_CONTACT_PERSON));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_property = get_mutable_property(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_property.is_some() {
                let (index, property) = optional_property.unwrap();

                if optional_role_of_contact_person.is_some() {
                    property.set_value(optional_role_of_contact_person.unwrap().get_semantic_id());
                } else {
                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template property role of contact person."));
            }

            //map national code mlp
            let national_code = contact_information.get_national_code();

            let key = Key::new(KeyType::GlobalReference, String::from(NATIONAL_CODE));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_mlp.is_some() {
                let (index, mlp) = optional_mlp.unwrap();

                if national_code.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    mlp.set_value(convert_hashmap_to_multi_language_text_type_list(national_code));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property national code."));
            }

            //map language properties
            let languages = contact_information.get_languages();

            let key = Key::new(KeyType::GlobalReference, String::from(LANGUAGE));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_template_property = get_mutable_property(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_template_property.is_some() {
                let (index, template_property) = optional_template_property.unwrap();
                let template_property = template_property.clone();

                for language_value in languages {
                    let mut language_property = template_property.clone();

                    language_property.set_value(language_value.get_language().clone());
                    language_property.set_id_short(language_value.get_short_id().clone());

                    contact_information_smc.get_mut_value().push(SubmodelElement::Property(language_property));
                }

                contact_information_smc.get_mut_value().remove(index);
            } else {
                return Err(String::from("Unable to find template multilanguage property language."));
            }

            //map time zone property
            let optional_time_zone = contact_information.get_time_zone();

            let key = Key::new(KeyType::GlobalReference, String::from(TIMEZONE));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_time_zone_property = get_mutable_property(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_time_zone_property.is_some() {
                let (index, time_zone_property) = optional_time_zone_property.unwrap();

                if optional_time_zone.is_some() {
                    time_zone_property.set_value(optional_time_zone.unwrap().clone());
                } else {
                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template property time zone."));
            }

            //map city town mlp
            let city_town_hashmap = contact_information.get_city_town();

            let key = Key::new(KeyType::GlobalReference, String::from(CITY_TOWN));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_city_town_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_city_town_mlp.is_some() {
                let (index, city_town_mlp) = optional_city_town_mlp.unwrap();

                if city_town_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    city_town_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(city_town_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property city town."));
            }

            //map company mlp
            let company_hashmap = contact_information.get_company();

            let key = Key::new(KeyType::GlobalReference, String::from(COMPANY));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_company_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_company_mlp.is_some() {
                let (index, company_mlp) = optional_company_mlp.unwrap();

                if company_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    company_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(company_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property company."));
            }

            //map department mlp
            let department_hashmap = contact_information.get_department();

            let key = Key::new(KeyType::GlobalReference, String::from(DEPARTMENT));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_department_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_department_mlp.is_some() {
                let (index, department_mlp) = optional_department_mlp.unwrap();

                if department_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    department_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(department_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property department."));
            }

            //map phone smc
            let phone_values = contact_information.get_phone();

            let key = Key::new(KeyType::GlobalReference, String::from(PHONE_ID));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_phone_smc = get_mutable_smc(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_phone_smc.is_some() {
                let (index, phone_smc) = optional_phone_smc.unwrap();

                if phone_values.is_some() {
                    let result = Self::map_phone_smc(phone_smc, phone_values.unwrap());

                    if result.is_err() {
                        let error_message = String::from("Error in submodel element collection \"")
                            .add("Phone")
                            .add("\": ")
                            .add(result.err().unwrap().as_str());

                        return Err(error_message);
                    }
                } else {
                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template submodel element collection phone."));
            }

            //map fax smc
            let fax_values = contact_information.get_fax();

            let key = Key::new(KeyType::GlobalReference, String::from(FAX_ID));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_fax_smc = get_mutable_smc(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_fax_smc.is_some() {
                let (index, fax_smc) = optional_fax_smc.unwrap();

                if fax_values.is_some() {
                    let result = Self::map_fax_smc(fax_smc, fax_values.unwrap());

                    if result.is_err() {
                        let error_message = String::from("Error in submodel element collection \"")
                            .add("Fax")
                            .add("\": ")
                            .add(result.err().unwrap().as_str());

                        return Err(error_message);
                    }
                } else {
                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template submodel element collection fax."));
            }

            //map email smc
            let email_values = contact_information.get_email();

            let key = Key::new(KeyType::GlobalReference, String::from(EMAIL_ID));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_email_smc = get_mutable_smc(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_email_smc.is_some() {
                let (index, email_smc) = optional_email_smc.unwrap();

                if email_values.is_some() {
                    let result = Self::map_email_smc(email_smc, email_values.unwrap());

                    if result.is_err() {
                        let error_message = String::from("Error in submodel element collection \"")
                            .add("Email")
                            .add("\": ")
                            .add(result.err().unwrap().as_str());

                        return Err(error_message);
                    }
                } else {
                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template submodel element collection email."));
            }

            //map ip communication smcs
            let ip_communication_value_list = contact_information.get_ip_communication();

            let key = Key::new(KeyType::GlobalReference, String::from(IP_COMMUNICATION_ID));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_ip_communication_template_smc = get_mutable_smc(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_ip_communication_template_smc.is_some() {
                let (index, ip_communication_template_smc) = optional_ip_communication_template_smc.unwrap();
                let ip_communication_template_smc = ip_communication_template_smc.clone();

                if ip_communication_value_list.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    for ip_communication_value in ip_communication_value_list {
                        let mut ip_communication_smc = ip_communication_template_smc.clone();

                        let result = Self::map_ip_communication(&mut ip_communication_smc, ip_communication_value);

                        if result.is_err() {
                            let error_message = String::from("Error in submodel element collection \"")
                                .add(ip_communication_value.get_id_short().as_str())
                                .add("\": ")
                                .add(result.err().unwrap().as_str());

                            return Err(error_message);
                        }

                        contact_information_smc.get_mut_value().push(SubmodelElement::SubmodelElementCollection(ip_communication_smc));
                    }

                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template submodel element collection ip communication."));
            }

            //map street mlp
            let street_hashmap = contact_information.get_street();

            let key = Key::new(KeyType::GlobalReference, String::from(STREET));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_street_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_street_mlp.is_some() {
                let (index, street_mlp) = optional_street_mlp.unwrap();

                if street_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    street_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(street_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property street."));
            }

            //map zipcode mlp
            let zipcode_hashmap = contact_information.get_zipcode();

            let key = Key::new(KeyType::GlobalReference, String::from(ZIPCODE));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_zipcode_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_zipcode_mlp.is_some() {
                let (index, zipcode_mlp) = optional_zipcode_mlp.unwrap();

                if zipcode_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    zipcode_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(zipcode_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property zipcode."));
            }

            //map po box mlp
            let po_box_hashmap = contact_information.get_pobox();

            let key = Key::new(KeyType::GlobalReference, String::from(PO_BOX));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_po_box_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_po_box_mlp.is_some() {
                let (index, po_box_mlp) = optional_po_box_mlp.unwrap();

                if po_box_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    po_box_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(po_box_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property po box."));
            }

            //map zipcode of po pox mlp
            let zipcode_of_po_box_hashmap = contact_information.get_zipcode_of_pobox();

            let key = Key::new(KeyType::GlobalReference, String::from(ZIPCODE_OF_PO_BOX));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let  optional_zipcode_of_po_box_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_zipcode_of_po_box_mlp.is_some() {
                let (index, zipcode_of_po_box_mlp) = optional_zipcode_of_po_box_mlp.unwrap();

                if zipcode_of_po_box_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    zipcode_of_po_box_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(zipcode_of_po_box_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property zipcode of po box."));
            }

            //map state county mlp
            let state_county_hashmap = contact_information.get_state_county();

            let key = Key::new(KeyType::GlobalReference, String::from(STATE_COUNTY));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_state_county_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_state_county_mlp.is_some() {
                let (index, state_county_mlp) = optional_state_county_mlp.unwrap();

                if state_county_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    state_county_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(state_county_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property state county."));
            }

            //map name of contact mlp
            let name_of_contact_hashmap = contact_information.get_name_of_contact();

            let key = Key::new(KeyType::GlobalReference, String::from(NAME_OF_CONTACT));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_name_of_contact_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_name_of_contact_mlp.is_some() {
                let (index, name_of_contact_mlp) = optional_name_of_contact_mlp.unwrap();

                if name_of_contact_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    name_of_contact_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(name_of_contact_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property name of contact."));
            }

            //map first name mlp
            let first_name_hashmap = contact_information.get_first_name();

            let key = Key::new(KeyType::GlobalReference, String::from(FIRST_NAME));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_first_name_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_first_name_mlp.is_some() {
                let (index, first_name_mlp) = optional_first_name_mlp.unwrap();

                if first_name_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    first_name_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(first_name_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property first name."));
            }

            //map middle names mlp
            let middle_names_hashmap = contact_information.get_middle_names();

            let key = Key::new(KeyType::GlobalReference, String::from(MIDDLE_NAMES));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_middle_names_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_middle_names_mlp.is_some() {
                let (index, middle_names_mlp) = optional_middle_names_mlp.unwrap();

                if middle_names_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    middle_names_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(middle_names_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property middle names."));
            }

            //map title mlp
            let title_hashmap = contact_information.get_title();

            let key = Key::new(KeyType::GlobalReference, String::from(TITLE));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_title_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_title_mlp.is_some() {
                let (index, title_mlp) = optional_title_mlp.unwrap();

                if title_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    title_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(title_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property title."));
            }

            //map academic title mlp
            let academic_title_hashmap = contact_information.get_academic_title();

            let key = Key::new(KeyType::GlobalReference, String::from(ACADEMIC_TITLE));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_academic_title_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_academic_title_mlp.is_some() {
                let (index, academic_title_mlp) = optional_academic_title_mlp.unwrap();

                if academic_title_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    academic_title_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(academic_title_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property academic title."));
            }

            //map further details of contact mlp
            let further_details_of_contact_hashmap = contact_information.get_further_details_of_contact();

            let key = Key::new(KeyType::GlobalReference, String::from(FURTHER_DETAILS_OF_CONTACT));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_further_details_of_contact_mlp = get_mutable_mlp(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_further_details_of_contact_mlp.is_some() {
                let (index, further_details_of_contact_mlp) = optional_further_details_of_contact_mlp.unwrap();

                if further_details_of_contact_hashmap.is_empty() {
                    contact_information_smc.get_mut_value().remove(index);
                } else {
                    further_details_of_contact_mlp.set_value(convert_hashmap_to_multi_language_text_type_list(further_details_of_contact_hashmap));
                }
            } else {
                return Err(String::from("Unable to find template multilanguage property further details of contact."));
            }

            //map address of additional link property
            let address_of_additional_link_value = contact_information.get_address_of_aditional_link();

            let key = Key::new(KeyType::GlobalReference, String::from(CONTACT_INFORMATION_ADDRESS_OF_ADDITIONAL_LINK));
            let semantic_id = Reference::new(ReferenceType::ExternalReference, vec![key]);

            let optional_address_additional_link_property = get_mutable_property(contact_information_smc.get_mut_value(), &semantic_id);

            if optional_address_additional_link_property.is_some() {
                let (index, address_additional_link_property) = optional_address_additional_link_property.unwrap();

                if address_of_additional_link_value.is_some() {
                    address_additional_link_property.set_value(address_of_additional_link_value.unwrap().clone());
                } else {
                    contact_information_smc.get_mut_value().remove(index);
                }
            } else {
                return Err(String::from("Unable to find template property address of additional link."));
            }

            self.submodel.add_submodel_element(SubmodelElement::SubmodelElementCollection(contact_information_smc));

            Ok(())
        } else {
            Err(String::from("Failed to add contact information"))
        }
    }

    pub fn add_list_of_contact_information(&mut self, contact_information: &Vec<ContactInformation>) -> Result<(), String> {
        for single_contact_information in contact_information {
            let result =self.add_contact_information(single_contact_information);

            if result.is_err() {
                let error_message = String::from("Error in submodel element collection \"")
                    .add(single_contact_information.get_id_short().as_str())
                    .add("\": ")
                    .add(result.err().unwrap().as_str());
                return Err(error_message);
            }
        }

        Ok(())
    }

    fn map_phone_smc(template_phone_smc: &mut SubmodelElementCollection, phone_values: &Phone) -> Result<(), String> {

    }

    fn map_fax_smc(template_fax_smc: &mut SubmodelElementCollection, fax_values: &Fax) -> Result<(), String> {

    }

    fn map_email_smc(template_email_smc: &mut SubmodelElementCollection, email_values: &Email) -> Result<(), String> {

    }

    fn map_ip_communication(template_ip_communication_smc: &mut SubmodelElementCollection, ip_communication_values: &IpCommunication) -> Result<(), String> {

    }
}