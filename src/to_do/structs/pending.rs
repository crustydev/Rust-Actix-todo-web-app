use super::base::Base;
use super::traits::create::Create;
use super::traits::edit::Edit;
use super::traits::get::Get;
use super::traits::delete::Delete;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");
        return Pending {
            super_struct: base
        }
    }
}

impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
impl Delete for Pending {}


#[cfg(test)]
mod pending_test {
    use super::Pending;

    #[test]
    fn new() {
        let title: String = String::from("Do Laundry");
        let expected_title: String = String::from("Do Laundry");
        let expected_status: String = String::from("pending");

        let pending:Pending = Pending::new(&title);

        assert_eq!(expected_title, pending.super_struct.title);
        assert_eq!(expected_status, pending.super_struct.status);
    }
}