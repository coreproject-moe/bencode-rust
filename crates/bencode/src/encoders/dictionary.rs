use crate::encoders::integer::encode_integer;
use crate::encoders::list::encode_list;
use crate::encoders::string::encode_string;
use crate::enums::dictionary::DictionaryValue;

use std::collections::BTreeMap;

pub fn encode_dict(data: BTreeMap<String, DictionaryValue>) -> Result<String, &'static str> {
    let mut encoded = String::from("d");

    for (key, value) in data.into_iter() {
        let item_str = match value {
            DictionaryValue::Int(n) => encode_integer(n)?,
            DictionaryValue::Str(s) => encode_string(&s)?,
            DictionaryValue::List(l) => encode_list(l)?,
            DictionaryValue::Dict(d) => encode_dict(d)?,
        };
    }

    encoded.push('e');
    Ok(encoded)
}
