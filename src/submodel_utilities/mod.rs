use std::collections::HashMap;
use aas_model_rs::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use aas_model_rs::structs::multi_language_property::MultiLanguageProperty;
use aas_model_rs::structs::multi_language_text_type::MultiLanguageTextType;
use aas_model_rs::structs::property::Property;
use aas_model_rs::structs::reference::Reference;
use aas_model_rs::structs::submodel_element_collection::SubmodelElementCollection;
use aas_model_rs::traits::has_semantics::THasSemantics;

pub fn convert_hashmap_to_multi_language_text_type_list(data: &HashMap<String, String>) -> Vec<MultiLanguageTextType> {
    let mut multi_language_text_type_list: Vec<MultiLanguageTextType> = Vec::new();

    for key in data.keys() {
        multi_language_text_type_list.push(MultiLanguageTextType::new(key.clone(), data.get(key).unwrap().clone()));
    }

    multi_language_text_type_list
}

pub fn get_property<'a>(submodel_elements: &'a Vec<SubmodelElement>, semantic_id: &Reference) -> Option<&'a Property> {
    for submodel_element in submodel_elements {
        match submodel_element {
            SubmodelElement::Property(property) => {
                if property.get_semantic_id() == Some(semantic_id) {
                    return Some(property);
                }
            },
            _ => {}
        }
    }

    None
}

pub fn get_mutable_property<'a>(submodel_elements: &'a mut Vec<SubmodelElement>, semantic_id: &Reference) -> Option<&'a mut Property> {
    for submodel_element in submodel_elements {
        match submodel_element {
            SubmodelElement::Property(property) => {
                if property.get_semantic_id() == Some(semantic_id) {
                    return Some(property);
                }
            },
            _ => {}
        }
    }

    None
}

pub fn get_mlp<'a>(submodel_elements: &'a Vec<SubmodelElement>, semantic_id: &Reference) -> Option<&'a MultiLanguageProperty> {
    for submodel_element in submodel_elements {
        match submodel_element {
            SubmodelElement::MultiLanguageProperty(mlp) => {
                if mlp.get_semantic_id() == Some(semantic_id) {
                    return Some(mlp);
                }
            },
            _ => {}
        }
    }

    None
}

pub fn get_mutable_mlp<'a>(submodel_elements: &'a mut Vec<SubmodelElement>, semantic_id: &Reference) -> Option<&'a mut MultiLanguageProperty> {
    for submodel_element in submodel_elements {
        match submodel_element {
            SubmodelElement::MultiLanguageProperty(mlp) => {
                if mlp.get_semantic_id() == Some(semantic_id) {
                    return Some(mlp);
                }
            },
            _ => {}
        }
    }

    None
}

pub fn get_smc<'a>(submodel_elements: &'a Vec<SubmodelElement>, semantic_id: &Reference) -> Option<&'a SubmodelElementCollection> {
    for submodel_element in submodel_elements {
        match submodel_element {
            SubmodelElement::SubmodelElementCollection(smc) => {
                if smc.get_semantic_id() == Some(semantic_id) {
                    return Some(smc);
                }
            },
            _ => {}
        }
    }

    None
}

pub fn get_mutable_smc<'a>(submodel_elements: &'a mut Vec<SubmodelElement>, semantic_id: &Reference) -> Option<&'a mut SubmodelElementCollection> {
    for submodel_element in submodel_elements {
        match submodel_element {
            SubmodelElement::SubmodelElementCollection(smc) => {
                if smc.get_semantic_id() == Some(semantic_id) {
                    return Some(smc);
                }
            },
            _ => {}
        }
    }

    None
}