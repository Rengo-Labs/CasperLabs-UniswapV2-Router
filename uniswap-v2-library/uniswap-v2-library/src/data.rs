use common::ContractHash;

// Accepts a Contract Hash and converts it into a simple String Hash without hex(0x)|(contract-)
pub fn make_hash(contract_hash: &ContractHash) -> String {
    let formatted_hash = contract_hash.to_formatted_string();
    let splitted_hash = formatted_hash.split('-');
    let vec = splitted_hash.collect::<Vec<&str>>();
    vec[1].into()
}

// Accepts array of hashes and concats them without hex(0x)|(contract-)
pub fn encode_packed(args: &[&String]) -> String {
    let mut encoded_hash: String = "".into();
    for i in args {
        let hash = i;
        encoded_hash.push_str(hash);
    }
    encoded_hash
}
