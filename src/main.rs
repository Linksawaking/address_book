use address_book::AddressBook;

fn main() {
    let mut ad = AddressBook::new("Tom");
    ad.add_entry("James", "Smith", "+44712345678");
    ad.add_entry("James", "Smith", "+44111111111");
    println!("{}", ad);
    ad.add_entry("James", "Dean", "+44715555655");
    ad.add_entry("Bob", "Smith", "+4471321655");
    ad.add_entry("Jenny", "Wolken", "+44222222222");
    ad.add_entry("Jenny", "Wolken", "+44333333333");
    println!("{}", ad);
}
