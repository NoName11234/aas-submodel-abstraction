use aas_model_rs::enumerations::data_type_def_xsd::DataTypeDefXsd;
use aas_model_rs::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use aas_model_rs::enumerations::key_type::KeyType;
use aas_model_rs::enumerations::modelling_kind::ModellingKind;
use aas_model_rs::enumerations::qualifier_kind::QualifierKind;
use aas_model_rs::enumerations::reference_type::ReferenceType;
use aas_model_rs::structs::administrative_information::AdministrativeInformation;
use aas_model_rs::structs::key::Key;
use aas_model_rs::structs::multi_language_property::MultiLanguageProperty;
use aas_model_rs::structs::multi_language_text_type::MultiLanguageTextType;
use aas_model_rs::structs::property::Property;
use aas_model_rs::structs::qualifier::Qualifier;
use aas_model_rs::structs::reference::Reference;
use aas_model_rs::structs::submodel::Submodel;
use aas_model_rs::structs::submodel_element_collection::SubmodelElementCollection;
use aas_model_rs::traits::has_kind::THasKind;
use aas_model_rs::traits::has_semantics::THasSemantics;
use aas_model_rs::traits::identifiable::TIdentifiable;
use aas_model_rs::traits::qualifiable::TQualifiable;
use aas_model_rs::traits::referable::TReferable;
use crate::contact_information::contact_information_1_0::data_structs::email::Email;
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::{ACADEMIC_TITLE, CITY_TOWN, COMPANY, CONTACT_INFORMATION_ADDRESS_OF_ADDITIONAL_LINK, CONTACT_INFORMATION_ID, DEPARTMENT, FIRST_NAME, FURTHER_DETAILS_OF_CONTACT, LANGUAGE, MIDDLE_NAMES, NAME_OF_CONTACT, NATIONAL_CODE, PO_BOX, ROLE_OF_CONTACT_PERSON, STATE_COUNTY, STREET, TIMEZONE, TITLE, ZIPCODE, ZIPCODE_OF_PO_BOX};
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::email::{EMAIL_ADDRESS, EMAIL_ID, PUBLIC_KEY, TYPE_OF_EMAIL_ADDRESS, TYPE_OF_PUBLIC_KEY};
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::fax::{FAX_ID, FAX_NUMBER, TYPE_OF_FAX_NUMBER};
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::ip_communication::{IP_COMMUNICATION_ADDRESS_OF_ADDITIONAL_LINK, IP_COMMUNICATION_AVAILABLE_TIME, IP_COMMUNICATION_ID, TYPE_OF_COMMUNICATION};
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::contact_information::phone::{PHONE_AVAILABLE_TIME, PHONE_ID, TELEPHONE_NUMBER, TYPE_OF_TELEPHONE};
use crate::contact_information::contact_information_1_0::template_submodel::semantic_ids::SUBMODEL_SEMANTIC_ID;

pub mod semantic_ids {
    pub const SUBMODEL_SEMANTIC_ID: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations";

    pub mod contact_information {
        pub const CONTACT_INFORMATION_ID: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation";
        pub const ROLE_OF_CONTACT_PERSON: &str = "0173-1#02-AAO204#003";
        pub const NATIONAL_CODE: &str = "0173-1#02-AAO134#002";
        pub const LANGUAGE: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/Language";
        pub const TIMEZONE: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/TimeZone";
        pub const CITY_TOWN: &str = "0173-1#02-AAO132#002";
        pub const COMPANY: &str = "0173-1#02-AAW001#001";
        pub const DEPARTMENT: &str = "0173-1#02-AAO127#003";
        pub const STREET: &str = "0173-1#02-AAO128#002";
        pub const ZIPCODE: &str = "0173-1#02-AAO129#002";
        pub const PO_BOX: &str = "0173-1#02-AAO130#002";
        pub const ZIPCODE_OF_PO_BOX: &str = "0173-1#02-AAO131#002";
        pub const STATE_COUNTY: &str = "0173-1#02-AAO133#002";
        pub const NAME_OF_CONTACT: &str = "0173-1#02-AAO205#002";
        pub const FIRST_NAME: &str = "0173-1#02-AAO206#002";
        pub const MIDDLE_NAMES: &str = "0173-1#02-AAO207#002";
        pub const TITLE: &str = "0173-1#02-AAO208#003";
        pub const ACADEMIC_TITLE: &str = "0173-1#02-AAO209#003";
        pub const FURTHER_DETAILS_OF_CONTACT: &str = "0173-1#02-AAO210#002";
        pub const CONTACT_INFORMATION_ADDRESS_OF_ADDITIONAL_LINK: &str = "0173-1#02-AAQ326#002";

        pub mod phone {
            pub const PHONE_ID: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/Phone";
            pub const TELEPHONE_NUMBER: &str = "0173-1#02-AAO136#002";
            pub const TYPE_OF_TELEPHONE: &str = "0173-1#02-AAO137#003";
            pub const PHONE_AVAILABLE_TIME: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/AvailableTime/";
        }

        pub mod fax {
            pub const FAX_ID: &str = "0173-1#02-AAQ834#005";
            pub const FAX_NUMBER: &str = "0173-1#02-AAO195#002";
            pub const TYPE_OF_FAX_NUMBER: &str = "0173-1#02-AAO196#003";
        }

        pub mod email {
            pub const EMAIL_ID: &str = "0173-1#02-AAQ836#005";
            pub const EMAIL_ADDRESS: &str = "0173-1#02-AAO198#002";
            pub const PUBLIC_KEY: &str = "0173-1#02-AAO200#002";
            pub const TYPE_OF_EMAIL_ADDRESS: &str = "0173-1#02-AAO199#003";
            pub const TYPE_OF_PUBLIC_KEY: &str = "0173-1#02-AAO201#002";
        }

        pub mod ip_communication {
            pub const IP_COMMUNICATION_ID: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/IPCommunication";
            pub const IP_COMMUNICATION_ADDRESS_OF_ADDITIONAL_LINK: &str = "0173-1#02-AAQ326#002";
            pub const TYPE_OF_COMMUNICATION: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/IPCommunication/TypeOfCommunication";
            pub const IP_COMMUNICATION_AVAILABLE_TIME: &str = "https://admin-shell.io/zvei/nameplate/1/0/ContactInformations/ContactInformation/AvailableTime/";
        }
    }
}


pub fn get_contact_information_1_0() -> Submodel {
    let mut submodel = Submodel::new(String::from("https://admin-shell.io/idta/aas/ContactInformation/1/0"));

    //set submodel data
    submodel.set_id_short(String::from("ContactInformation"));
    let key = Key::new(
        KeyType::Submodel,
        String::from(SUBMODEL_SEMANTIC_ID),
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    submodel.set_semantic_id(semantic_id);
    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("The Submodel \"ContactInformations\" is the collection for various contact information."),
    )];
    submodel.set_description(description);
    submodel.set_kind(ModellingKind::Template);
    let mut administration = AdministrativeInformation::new();
    administration.set_version(String::from("1"));
    administration.set_revision(String::from("0"));
    submodel.set_administration(administration);

    //set submodel elements
    let mut elements_of_contact_information_smc: Vec<SubmodelElement> = Vec::new();

    //create contact information element
    let mut contact_information_smc = SubmodelElementCollection::new();

    contact_information_smc.set_id_short(String::from("ContactInformation"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(CONTACT_INFORMATION_ID),
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    contact_information_smc.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("The SMC \"ContactInformation\" contains information on how to contact the manufacturer or an authorised service provider, e.g. when a maintenance service is required")
    )];
    contact_information_smc.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("OneToMany"));
    contact_information_smc.add_qualifier(qualifier);

    //create role of contact person element
    let mut role_of_contact_person_property = Property::new(DataTypeDefXsd::String);

    role_of_contact_person_property.set_id_short(String::from("RoleOfContactPerson"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(ROLE_OF_CONTACT_PERSON)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    role_of_contact_person_property.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("function of a contact person in a process")
    )];
    role_of_contact_person_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    role_of_contact_person_property.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::Property(role_of_contact_person_property));

    //create national code element
    let mut national_code_mlp = MultiLanguageProperty::new();

    national_code_mlp.set_id_short(String::from("NationalCode"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(NATIONAL_CODE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    national_code_mlp.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("code of a country")
    )];
    national_code_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    national_code_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(national_code_mlp));

    //create language element
    let mut language_property = Property::new(DataTypeDefXsd::String);

    language_property.set_id_short(String::from("Language"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(LANGUAGE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    language_property.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("Available language")
    )];
    language_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToMany"));
    language_property.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::Property(language_property));

    //create time zone element
    let mut timezone_property = Property::new(DataTypeDefXsd::String);

    timezone_property.set_id_short(String::from("TimeZone"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TIMEZONE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    timezone_property.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("offsets from Coordinated Universal Time (UTC)")
    )];
    timezone_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    timezone_property.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::Property(timezone_property));

    //create city town element
    let mut city_town_mlp = MultiLanguageProperty::new();

    city_town_mlp.set_id_short(String::from("CityTown"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(CITY_TOWN)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    city_town_mlp.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("town or city")
    )];
    city_town_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    city_town_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(city_town_mlp));

    //create company element
    let mut company_mlp = MultiLanguageProperty::new();

    company_mlp.set_id_short(String::from("Company"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(COMPANY)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    company_mlp.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("name of the company")
    )];
    company_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    company_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(company_mlp));

    //create department element
    let mut department_mlp = MultiLanguageProperty::new();

    department_mlp.set_id_short(String::from("Department"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(DEPARTMENT)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    department_mlp.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("administrative section within an organisation where a business partner is located")
    )];
    department_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    department_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(department_mlp));

    //create phone element
    let mut phone_smc = SubmodelElementCollection::new();

    phone_smc.set_id_short(String::from("Phone"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(PHONE_ID)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    phone_smc.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("Phone number including type")
        )
    ];
    phone_smc.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    phone_smc.add_qualifier(qualifier);

    //create phone elements
    let mut phone_elements: Vec<SubmodelElement> = Vec::new();

    //create telephone number element
    let mut telephone_number_mlp = MultiLanguageProperty::new();

    telephone_number_mlp.set_id_short(String::from("TelephoneNumber"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TELEPHONE_NUMBER)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    telephone_number_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("complete telephone number to be called to reach a business partner")
        )
    ];
    telephone_number_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("One"));
    telephone_number_mlp.add_qualifier(qualifier);

    phone_elements.push(SubmodelElement::MultiLanguageProperty(telephone_number_mlp));

    //create type of telephone element
    let mut type_of_telephone_property = Property::new(DataTypeDefXsd::String);

    type_of_telephone_property.set_id_short(String::from("TypeOfTelephone"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TYPE_OF_TELEPHONE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    type_of_telephone_property.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("characterization of a telephone according to its location or usage")
        )
    ];
    type_of_telephone_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    type_of_telephone_property.add_qualifier(qualifier);

    phone_elements.push(SubmodelElement::Property(type_of_telephone_property));

    //create available time element
    let mut available_time_mlp = MultiLanguageProperty::new();

    available_time_mlp.set_id_short(String::from("AvailableTime"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(PHONE_AVAILABLE_TIME)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    available_time_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("Specification of the available time window")
        )
    ];
    available_time_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    available_time_mlp.add_qualifier(qualifier);

    phone_elements.push(SubmodelElement::MultiLanguageProperty(available_time_mlp));

    phone_smc.set_value(phone_elements);

    elements_of_contact_information_smc.push(SubmodelElement::SubmodelElementCollection(phone_smc));

    //create fax element
    let mut fax_smc = SubmodelElementCollection::new();

    fax_smc.set_id_short(String::from("Fax"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(FAX_ID)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    fax_smc.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("Fax number including type")
        )
    ];
    fax_smc.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    fax_smc.add_qualifier(qualifier);

    //create fax elements
    let mut fax_elements: Vec<SubmodelElement> = Vec::new();

    //create fax number element
    let mut fax_number_mlp = MultiLanguageProperty::new();

    fax_number_mlp.set_id_short(String::from("FaxNumber"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(FAX_NUMBER)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    fax_number_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("complete telephone number to be called to reach a business partner's fax machine")
        )
    ];
    fax_number_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("One"));
    fax_number_mlp.add_qualifier(qualifier);

    fax_elements.push(SubmodelElement::MultiLanguageProperty(fax_number_mlp));

    //create type of fax number element
    let mut type_of_fax_number_property = Property::new(DataTypeDefXsd::String);

    type_of_fax_number_property.set_id_short(String::from("TypeOfFaxNumber"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TYPE_OF_FAX_NUMBER)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    type_of_fax_number_property.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("characterization of the fax according its location or usage")
        )
    ];
    type_of_fax_number_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    type_of_fax_number_property.add_qualifier(qualifier);

    fax_elements.push(SubmodelElement::Property(type_of_fax_number_property));

    fax_smc.set_value(fax_elements);
    elements_of_contact_information_smc.push(SubmodelElement::SubmodelElementCollection(fax_smc));

    //create email element
    let mut email_smc = SubmodelElementCollection::new();

    email_smc.set_id_short(String::from("Email"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(EMAIL_ID)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    email_smc.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("E-mail address and encryption method")
        )
    ];
    email_smc.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    email_smc.add_qualifier(qualifier);

    //create email elements
    let mut email_elements: Vec<SubmodelElement> = Vec::new();

    //create email address element
    let mut email_address_property = Property::new(DataTypeDefXsd::String);

    email_address_property.set_id_short(String::from("EmailAddress"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(EMAIL_ADDRESS)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    email_address_property.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("electronic mail address of a business partner")
        )
    ];
    email_address_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("One"));
    email_address_property.add_qualifier(qualifier);

    email_elements.push(SubmodelElement::Property(email_address_property));

    //create public key element
    let mut public_key_mlp = MultiLanguageProperty::new();

    public_key_mlp.set_id_short(String::from("PublicKey"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(PUBLIC_KEY)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    public_key_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("public part of an unsymmetrical key pair to sign or encrypt text or messages")
        )
    ];
    public_key_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    public_key_mlp.add_qualifier(qualifier);

    email_elements.push(SubmodelElement::MultiLanguageProperty(public_key_mlp));

    //create type of email address element
    let mut type_of_email_address_property = Property::new(DataTypeDefXsd::String);

    type_of_email_address_property.set_id_short(String::from("TypeOfEmailAddress"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TYPE_OF_EMAIL_ADDRESS)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    type_of_email_address_property.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("characterization of an e-mail address according to its location or usage")
    )];
    type_of_email_address_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    type_of_email_address_property.add_qualifier(qualifier);

    email_elements.push(SubmodelElement::Property(type_of_email_address_property));

    //create type of public key element
    let mut type_of_public_key_mlp = MultiLanguageProperty::new();

    type_of_public_key_mlp.set_id_short(String::from("TypeOfPublicKey"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TYPE_OF_PUBLIC_KEY)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    type_of_public_key_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("characterization of a public key according to its encryption process")
        )
    ];
    type_of_public_key_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    type_of_public_key_mlp.add_qualifier(qualifier);

    email_elements.push(SubmodelElement::MultiLanguageProperty(type_of_public_key_mlp));

    email_smc.set_value(email_elements);
    elements_of_contact_information_smc.push(SubmodelElement::SubmodelElementCollection(email_smc));

    //create ip communication element
    let mut ip_communication_smc = SubmodelElementCollection::new();

    ip_communication_smc.set_id_short(String::from("IPCommunication{00}"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(IP_COMMUNICATION_ID)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    ip_communication_smc.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("IP-based communication channels, e.g. chat or video call")
        )
    ];
    ip_communication_smc.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToMany"));
    ip_communication_smc.add_qualifier(qualifier);

    //create ip communication elements
    let mut ip_communication_elements: Vec<SubmodelElement> = Vec::new();

    //create address of additional link element
    let mut address_of_additional_link_property = Property::new(DataTypeDefXsd::String);

    address_of_additional_link_property.set_id_short(String::from("AddressOfAdditionalLink"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(IP_COMMUNICATION_ADDRESS_OF_ADDITIONAL_LINK)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    address_of_additional_link_property.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("web site address where information about the product or contact is given")
        )
    ];
    address_of_additional_link_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("One"));
    address_of_additional_link_property.add_qualifier(qualifier);

    ip_communication_elements.push(SubmodelElement::Property(address_of_additional_link_property));

    //create type of communication
    let mut type_of_communication_property = Property::new(DataTypeDefXsd::String);

    type_of_communication_property.set_id_short(String::from("TypeOfCommunication"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TYPE_OF_COMMUNICATION)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    type_of_communication_property.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("characterization of an IP-based communication channel")
        )
    ];
    type_of_communication_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    type_of_communication_property.add_qualifier(qualifier);

    ip_communication_elements.push(SubmodelElement::Property(type_of_communication_property));

    //create available time
    let mut available_time_mlp = MultiLanguageProperty::new();

    available_time_mlp.set_id_short(String::from("AvailableTime"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(IP_COMMUNICATION_AVAILABLE_TIME)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    available_time_mlp.set_semantic_id(semantic_id);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    available_time_mlp.add_qualifier(qualifier);

    ip_communication_elements.push(SubmodelElement::MultiLanguageProperty(available_time_mlp));

    ip_communication_smc.set_value(ip_communication_elements);
    elements_of_contact_information_smc.push(SubmodelElement::SubmodelElementCollection(ip_communication_smc));

    //create street element
    let mut street_mlp = MultiLanguageProperty::new();

    street_mlp.set_id_short(String::from("Street"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(STREET)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    street_mlp.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("street name and house number")
    )];
    street_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    street_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(street_mlp));

    //create zipcode element
    let mut zipcode_mlp = MultiLanguageProperty::new();

    zipcode_mlp.set_id_short(String::from("Zipcode"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(ZIPCODE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    zipcode_mlp.set_semantic_id(semantic_id);

    let description = vec![MultiLanguageTextType::new(
        String::from("en"),
        String::from("ZIP code of address")
    )];
    zipcode_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    zipcode_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(zipcode_mlp));

    //create po box element
    let mut po_box_mlp = MultiLanguageProperty::new();

    po_box_mlp.set_id_short(String::from("POBox"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(PO_BOX)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    po_box_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("P.O. box number")
        )
    ];
    po_box_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    po_box_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(po_box_mlp));

    //create zipcode of po box element
    let mut zipcode_of_po_box_mlp = MultiLanguageProperty::new();

    zipcode_of_po_box_mlp.set_id_short(String::from("ZipCodeOfPOBox"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(ZIPCODE_OF_PO_BOX)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    zipcode_of_po_box_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("ZIP code of P.O. box address")
        )
    ];
    zipcode_of_po_box_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    zipcode_of_po_box_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(zipcode_of_po_box_mlp));

    //create state county element
    let mut state_county_mlp = MultiLanguageProperty::new();

    state_county_mlp.set_id_short(String::from("StateCounty"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(STATE_COUNTY)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    state_county_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("federal state a part of a state")
        )
    ];
    state_county_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    state_county_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(state_county_mlp));

    //create name of contact element
    let mut name_of_contact_mlp = MultiLanguageProperty::new();

    name_of_contact_mlp.set_id_short(String::from("NameOfContact"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(NAME_OF_CONTACT)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    name_of_contact_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("surname of a contact person")
        )
    ];
    name_of_contact_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    name_of_contact_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(name_of_contact_mlp));

    //create first name element
    let mut first_name_mlp = MultiLanguageProperty::new();

    first_name_mlp.set_id_short(String::from("FirstName"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(FIRST_NAME)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    first_name_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("first name of a contact person")
        )
    ];
    first_name_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    first_name_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(first_name_mlp));

    //create middle names element
    let mut middle_names_mlp = MultiLanguageProperty::new();

    middle_names_mlp.set_id_short(String::from("MiddleNames"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(MIDDLE_NAMES)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    middle_names_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("middle names of contact person")
        )
    ];
    middle_names_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    middle_names_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(middle_names_mlp));

    //create title element
    let mut title_mlp = MultiLanguageProperty::new();

    title_mlp.set_id_short(String::from("Title"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(TITLE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    title_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("common, formal, religious, or other title preceding a contact person's name")
        )
    ];
    title_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    title_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(title_mlp));

    //create academic title element
    let mut academic_title_mlp = MultiLanguageProperty::new();

    academic_title_mlp.set_id_short(String::from("AcademicTitle"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(ACADEMIC_TITLE)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    academic_title_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("academic title preceding a contact person's name")
        )
    ];
    academic_title_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    academic_title_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(academic_title_mlp));

    //create further details of contact element
    let mut further_details_of_contact_mlp = MultiLanguageProperty::new();

    further_details_of_contact_mlp.set_id_short(String::from("FurtherDetailsOfContact"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(FURTHER_DETAILS_OF_CONTACT)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    further_details_of_contact_mlp.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("additional information of the contact person")
        )
    ];
    further_details_of_contact_mlp.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    further_details_of_contact_mlp.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::MultiLanguageProperty(further_details_of_contact_mlp));

    //create address of additional link element
    let mut address_of_additional_link_property = Property::new(DataTypeDefXsd::String);

    address_of_additional_link_property.set_id_short(String::from("AddressOfAdditionalLink"));

    let key = Key::new(
        KeyType::GlobalReference,
        String::from(CONTACT_INFORMATION_ADDRESS_OF_ADDITIONAL_LINK)
    );
    let semantic_id = Reference::new(ReferenceType::ModelReference, vec![key]);
    address_of_additional_link_property.set_semantic_id(semantic_id);

    let description = vec![
        MultiLanguageTextType::new(
            String::from("en"),
            String::from("web site address where information about the product or contact is given")
        )
    ];
    address_of_additional_link_property.set_description(description);

    let mut qualifier = Qualifier::new(String::from("Multiplicity"), DataTypeDefXsd::String);
    qualifier.set_kind(QualifierKind::ConceptQualifier);
    qualifier.set_value(String::from("ZeroToOne"));
    address_of_additional_link_property.add_qualifier(qualifier);

    elements_of_contact_information_smc.push(SubmodelElement::Property(address_of_additional_link_property));

    contact_information_smc.set_value(elements_of_contact_information_smc);
    submodel.add_submodel_element(SubmodelElement::SubmodelElementCollection(contact_information_smc));

    submodel
}
