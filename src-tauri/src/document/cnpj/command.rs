use crate::document::CNPJ;

#[tauri::command]
pub fn generate_cnpj() -> String {
    return CNPJ::generate();
}

#[tauri::command]
pub fn validate_cnpj(document: String) -> bool {
    return CNPJ::validate(document);
}