//! # Address book
//!
//! Create address book to store people's phone numbers

pub mod entry;

use entry::AddressEntry;

#[cfg(test)]
mod address_tests {
    use super::*;

    fn setup_test_address_book<'a>() -> AddressBook<'a> {
        let mut ad = AddressBook::new("Tom");
        ad.add_entry("James", "Smith", "+44712345678");
        ad.add_entry("James", "Smith", "+44111111111");

        ad.add_entry("James", "Dean", "+44715555655");
        ad.add_entry("Bob", "Smith", "+4471321655");
        ad.add_entry("Jenny", "Wolken", "+44222222222");
        ad.add_entry("Jenny", "Wolken", "+44333333333");
        ad
    }

    #[test]
    fn create_address_book_test() {
        let ad = AddressBook::new("Tom");
    }

    #[test]
    fn add_entry_test() {
        let name = "James";
        let surname = "Smith";
        let number = "+44712345678";

        let mut ad = AddressBook::new("Tom");
        ad.add_entry(name, surname, number);
    }

    #[test]
    fn add_entry_two_times() {
        let name = "James";
        let surname = "Smith";
        let number = "+44712345678";

        let mut ad = AddressBook::new("Tom");
        ad.add_entry(name, surname, number);
        let length1 = ad.get_entries().len();
        ad.add_entry(name, surname, number);
        let length2 = ad.get_entries().len();
        assert_eq!(length1, length2);
    }

    #[test]
    fn remove_entries_3_methods() {
        let mut ad = setup_test_address_book();

        assert!(!ad.remove_entries_name("Bip"));
        assert!(!ad.remove_entries_name_surname("Bip", "Bop"));
        assert!(!ad.remove_entry("Bip", "Bop", "0000"));
        assert!(ad.remove_entry("James", "Smith", "+44111111111"));
        assert!(ad.remove_entries_name_surname("Jenny", "Wolken"));
        assert!(ad.remove_entries_name("James"));
        assert_eq!(ad.get_entries().len(), 1);
    }

    #[test]
    fn search() {
        let ad = setup_test_address_book();
        //searches
        let james_search = ad.search_name("James");
        let smith_search = ad.search_surname("Smith");
        let num_search = ad.search_number("+44111111111");

        // entries supposed to be found
        let james = AddressEntry::new("James", "Smith", "+44111111111");
        let smith = AddressEntry::new("Bob", "Smith", "+4471321655");
        let num = AddressEntry::new("James", "Smith", "+44111111111");

        assert!(james_search.contains(&&james));
        assert!(smith_search.contains(&&smith));
        assert!(num_search.contains(&&num));

    }
}

/// Structure that represents the address book. All actions on the entries are
/// performed from here.
pub struct AddressBook<'a> {
    owner: &'a str,
    entries: Vec<AddressEntry<'a>>,
}

impl<'a> AddressBook<'a> {
    /// Instanciates a new AddressBook structure. It only needs the name of the owner.
    pub fn new(owner: &str) -> AddressBook {
        AddressBook {
            owner: owner,
            entries: Vec::new(),
        }
    }
    /// Creates a new AddressEntry and adds it to the existing ones.
    /// If the entry already exists, it doesn't add it, and return false, else it returns true.
    pub fn add_entry(&mut self, name: &'a str, surname: &'a str, number: &'a str) -> bool{
        let new_entry = AddressEntry::new(name, surname, number);
        if !self.entries.contains(&new_entry) {
            self.entries.push(new_entry);
            return true;
        }
        else {
            return false;
        }
    }

    /// Removes a specific entry in the address book. returns true if something has been removed.
    pub fn remove_entries_name(&mut self, name: &'a str) -> bool {
        let len_before = self.entries.len();
        self.entries.retain(|e| !((e.name == name)));
        return len_before!=self.entries.len();
    }

    /// Removes a specific entry in the address book. returns true if something has been removed.
    pub fn remove_entries_name_surname(&mut self, name: &'a str, surname: &'a str) -> bool {
        let len_before = self.entries.len();
        self.entries.retain(|e| !((e.name == name) && (e.surname==surname)));
        return len_before!=self.entries.len();
    }

    /// Removes a specific entry in the address book. returns true if something has been removed.
    pub fn remove_entry(&mut self, name: &'a str, surname: &'a str, number: &'a str) -> bool {
        let len_before = self.entries.len();
        self.entries.retain(|e| !((e.name == name) && (e.surname==surname) && (e.number==number)));
        return len_before!=self.entries.len();
    }

    /// Returns an immutable reference to the entries.
    pub fn get_entries(&self) -> &Vec<AddressEntry>{
        &self.entries
    }

    /// Search entries by their name. Returns a collection of AddressEntry.
    pub fn search_name(&self, name: &str) -> Vec<&AddressEntry> {
        self.entries.iter().filter(|e| e.name == name).collect()
    }

    /// Search entries by their name. Returns a collection of AddressEntry.
    pub fn search_surname(&self, surname: &str) -> Vec<&AddressEntry> {
        self.entries.iter().filter(|e| e.surname == surname).collect()
    }

    /// Search entries by their name. Returns a collection of AddressEntry.
    pub fn search_number(&self, number: &str) -> Vec<&AddressEntry> {
        self.entries.iter().filter(|e| e.number == number).collect()
    }

}

impl<'a> std::fmt::Display for AddressBook<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:#?})", self.owner, self.entries)
    }
}
