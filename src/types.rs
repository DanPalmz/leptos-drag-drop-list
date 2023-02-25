#[derive(Debug, Clone)]
pub struct ListItem {
    pub id: i32,
    pub name: String,
    pub status: ListItemStatus,
}

#[derive(Debug, Clone)]
pub enum ListItemStatus {
    Complete,
    Incomplete,
}

impl Default for ListItem {
    fn default() -> ListItem {
        ListItem {
            id: 99,
            name: "New list item".to_string(),
            status: ListItemStatus::Incomplete,
        }
    }
}

pub trait Listable {
    fn get_name(&self) -> String;
    fn get_id(&self) -> i32;
}

impl<'a, T> Listable for &'a T
where
    T: Listable,
{
    fn get_id(&self) -> i32 {
        (*self).get_id()
    }
    fn get_name(&self) -> String {
        (*self).get_name()
    }
}

// impl Listable for &ListItem {
//     fn get_id(&self) -> String {
//         self.id.to_string()
//     }
//     fn get_name(&self) -> String {
//         self.name.to_string()
//     }
// }

impl Listable for ListItem {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum DragState {
    Normal,
    DraggedOver,
    Dragging,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DragListState {
    Normal,
    Dragging {
        item_id: i32,
        target_id: Option<i32>,
    },
    Dropped {
        item_id: i32,
        target_id: i32,
    },
}
