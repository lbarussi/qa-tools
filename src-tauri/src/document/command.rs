use crate::document::CPF;

#[tauri::command]
pub fn generate_cpf() -> String {
    println!("{}", CPF::generate_document());
    return CPF::generate_document();
}

#[tauri::command]
pub fn validate_cpf(document: String) -> bool {
    return CPF::validate_document(document);
}