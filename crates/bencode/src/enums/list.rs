#[derive(Debug)]
pub enum BencodeValue {
    Int(i64),
    Str(String),
    List(Vec<BencodeValue>),
}
