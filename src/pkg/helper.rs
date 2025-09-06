// use std::collections::HashMap;
// use validator::ValidationErrors;

// pub fn validation_errors_to_map(errors: &ValidationErrors) -> HashMap<String, String> {
//     let mut map = HashMap::new();

//     for (field, errors) in errors.field_errors().iter() {
//         if let Some(first_error) = errors.first() {
//             let message = first_error
//                 .message
//                 .clone()
//                 .unwrap_or_else(|| std::borrow::Cow::Borrowed("Invalid value"));
//             map.insert(field.to_string(), message.to_string());
//         }
//     }

//     map
// }