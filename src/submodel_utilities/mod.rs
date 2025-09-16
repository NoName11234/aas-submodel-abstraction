use aas_model_rs::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use aas_model_rs::structs::multi_language_property::MultiLanguageProperty;
use aas_model_rs::structs::property::Property;
use aas_model_rs::structs::reference::Reference;
use aas_model_rs::structs::submodel_element_collection::SubmodelElementCollection;
use aas_model_rs::traits::has_semantics::THasSemantics;

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