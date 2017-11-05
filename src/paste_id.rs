use std::fmt;
use std::borrow::Cow;

use uuid::Uuid;
use rocket::http::RawStr;
use rocket::request::FromParam;


pub struct PasteID<'a>(Cow<'a, str>);

impl<'a> PasteID<'a> {
    pub fn new() -> PasteID<'static> {
        let id = Uuid::new_v4().simple().to_string();
        PasteID(Cow::Owned(id))
    }
}

impl<'a> FromParam<'a> for PasteID<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<PasteID<'a>, &'a RawStr> {
        match Uuid::parse_str(param) {
            Ok(_id) => Ok(PasteID(Cow::Borrowed(param))),
            Err(err) => {
                println!("UUID ERROR: {:?}", err);
                Err(param)
            }
        }
    }
}

impl<'a> fmt::Display for PasteID<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
