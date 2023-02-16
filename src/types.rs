#[derive(Debug)]
pub struct ListItem {
    pub id: i32,
    pub name: String,
}

pub trait Listable {
    fn get_name(&self) -> String;
    fn get_id(&self) -> String;
}

impl Listable for ListItem {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}