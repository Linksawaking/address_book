//! # Entry
//!
//! Entries contain information about a specific person (name, surname, number).



#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn create_entry() {
        let name = "James";
        let surname = "Smith";
        let number = "+44712345678";

        let ad = AddressEntry::new(name, surname, number);
    }
}

/// Stucture holding the data of the entry.
#[derive(PartialEq, Debug)]
pub struct AddressEntry<'a> {
    pub name: &'a str,
    pub surname: &'a str,
    pub number: &'a str,
}


impl<'a> AddressEntry<'a> {
    /// Instanciates a new AddressEntry structure.
    pub fn new<'b>(name: &'b str, surname: &'b str, number: &'b str) -> AddressEntry<'b>{
        AddressEntry {
            name,
            surname,
            number,
        }
    }

}
