use std::collections::HashMap;
use crate::permission::Permission;

#[derive(Debug, PartialEq, Eq)]
pub struct Actions{
    pub action:String,
    pub permissions: HashMap<Permission, bool>
}
impl Default for Actions{
    fn default() -> Self {
        let permissions = HashMap::from([
                (Permission::READ, false),
                (Permission::WRITE, false),
                (Permission::EXECUTE, false),
            ]);
        Self {action: String::new(), permissions}
    }
}
impl Actions{
    pub fn new(action: String, read:bool, write:bool, execute:bool) -> Self{
        let permissions = HashMap::from([
            (Permission::READ, read),
            (Permission::WRITE, write),
            (Permission::EXECUTE, execute),
        ]);
        Self {action, permissions}
    }
}