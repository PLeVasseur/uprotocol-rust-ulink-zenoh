pub mod sdk {
    pub use uprotocol_sdk::*;
}

pub mod example {
    use crate::sdk::uprotocol::Uuid as uproto_Uuid;

    fn ex1() {
        let _uuuuu = uproto_Uuid::default();
    }
}