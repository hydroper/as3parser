/// Escapes XML special characters.
pub fn escape_xml(characters: &str) -> String {
    let escaped = htmlentity::entity::encode(characters.as_ref(), &htmlentity::entity::EncodeType::NamedOrHex, &htmlentity::entity::CharacterSet::SpecialChars);
    String::from_utf8_lossy(&escaped.bytes().into_owned()).into_owned()
}

/// Unescapes XML entities conforming to HTML entities.
pub fn unescape_xml(input: &str) -> String {
    let unescaped = htmlentity::entity::decode(input.as_ref());
    String::from_utf8_lossy(&unescaped.bytes().into_owned()).into_owned()
}