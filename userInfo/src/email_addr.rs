use validator::validate_email;

#[derive(Debug)]
pub enum Validate_Email {
    validEmail,
    invalidEmail,
}
pub fn valid_email(email: &String) -> Validate_Email {
    match validate_email(email) {
        true => return Validate_Email::validEmail,
        false => return Validate_Email::invalidEmail,
    }
}

#[derive(PartialEq, Debug)]
pub enum DuplicateEmail {
    Exist,
    Nonexist,
}
// pub fn is_unique_email(email: String) -> bool {
    
// }