pub trait Document {
    fn generate_document(&self) -> String;
    fn validate_document(&self, document: String) -> bool;
}