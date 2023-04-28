
#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String
}

impl ProjectInfo {
    pub fn from(t: (String, String)) -> ProjectInfo {
        Self {
            name: t.0,
            version: t.1
        }
    }
}
