pub mod headers;
pub mod builder;
pub mod field_zone;

#[derive(Debug)]
pub struct DocumentDecoded{
    pub headers: headers::Headers,
    pub message: field_zone::FieldZone,
}