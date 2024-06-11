pub mod cpf;
pub mod cnpj;

pub use cpf::CPF;
pub use cnpj::CNPJ;

pub use cpf::command::generate_cpf;
pub use cpf::command::validate_cpf;

pub use cnpj::command::generate_cnpj;
pub use cnpj::command::validate_cnpj;