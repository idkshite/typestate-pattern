use anyhow::{anyhow, Context, Result};
use regex::Regex;

#[derive(Debug)]
struct Email(String);
/*#[derive(Debug)]
struct Password(String);*/


impl Email {
    fn is_valid_email(email: &String) -> bool{
    let regex = Regex::new(r"[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+").unwrap();
    if regex.is_match(email) {
      return true;
    }

    false
    }
    fn parse(email: String) -> Result<Self>{
        if Self::is_valid_email(&email){
            return Ok(Self(email))
        }

        Err(anyhow!("Invalid Email Address {:?}", email)).context("provided by userId: 123456")


    }
    fn to_str(self) -> String{
        self.0
    }
}


fn main() {

    let email = Email::parse("lolman@.com".to_string());
    match email {
        Ok(email) => {
            println!("Hello, {}", email.to_str());
        }
        Err(error) => {
            eprint!("{:?}", error);
        }
    }

}
