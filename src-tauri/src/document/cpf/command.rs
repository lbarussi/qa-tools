use crate::document::CPF;

#[tauri::command]
pub fn generate_cpf() -> String {
    return CPF::generate();
}

#[tauri::command]
pub fn validate_cpf(document: String) -> bool {
    return CPF::validate(document);
}