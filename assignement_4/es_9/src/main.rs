#![allow(dead_code, unused_imports)]
pub mod actions;
pub mod permission;
pub mod role;
pub mod testing;
use std::collections::HashMap;

use permission::Permission;
use role::Role;
use actions::Actions;

trait Auth{
    fn check_permission(&self, action:&str, permission_type:&Permission) -> bool;
    fn can_write(&self, string:&str) -> bool;
    fn can_read(&self, string:&str) -> bool;
    fn can_execute(&self, string:&str) -> bool;
}
#[derive(Debug, PartialEq, Eq)]
pub struct User{
    pub name:String,
    pub role:Role,
    pub actions: Vec<Actions>
}

impl Auth for User{
    fn check_permission(&self, action:&str, permission_type:&Permission) -> bool {
        let possible_actions:Vec<&Actions> = self.actions
                                        .iter()
                                        .filter(|doable_action| doable_action.action.to_string() == action)
                                        .filter(|action| *action.permissions.get(permission_type).unwrap() == true)
                                        .collect();
        if possible_actions.len() == 0{
            return false;
        }
        return true;
    }

    fn can_write(&self, string:&str) -> bool {
        self.check_permission(&string, &Permission::WRITE)
    }
    fn can_read(&self, string:&str) -> bool {
        self.check_permission(&string, &Permission::READ)
    }
    fn can_execute(&self, string:&str) -> bool {
        self.check_permission(&string, &Permission::EXECUTE)
    }
    
}
impl Default for User{
    fn default() -> Self {
        Self { name: "Guest".to_string(), role: Role::GUEST, actions: Vec::new()}
    }
}
impl User{
    fn change_role(&mut self, role:Role) -> Result<(), String>{
        match self.role{
            Role::ADMIN => {self.role = role; Ok(())},
            Role::USER => {
                match role{
                    Role::ADMIN => return Err("User {self.name} tried to get role Admin.".to_string()),
                    _ => self.role = role
                };
                Ok(())
            },
            Role::GUEST => {
                match role{
                    Role::GUEST => {self.role = role; Ok(())},
                    _ => Err("Guest {self.name} tried to get role {role}.".to_string())
                }
            }
        }
    }
}

pub fn sudo_change_permissions(user: &mut User, string:String, permission:Permission, value:bool){
    match permission{
        Permission::READ => {
            if !user.can_read(string.as_str()){
                let action = user.actions.iter_mut().find(|action| action.action == string);
                match action {
                    None => user.actions.push(Actions::new(string, value, false, false)),
                    Some(a) => {
                        a.permissions.insert(Permission::READ, true);
                    }
                };
            }
        },
        Permission::WRITE => {
            if !user.can_write(string.as_str()){
                let action = user.actions.iter_mut().find(|action| action.action == string);
                match action {
                    None => user.actions.push(Actions::new(string, false, value, false)),
                    Some(a) => {
                        a.permissions.insert(Permission::WRITE, true);
                    }
                };
            }
        },
        Permission::EXECUTE => {
            if !user.can_execute(string.as_str()){
                let action = user.actions.iter_mut().find(|action| action.action == string);
                match action {
                    None => user.actions.push(Actions::new(string, false, false, value)),
                    Some(a) => {
                        a.permissions.insert(Permission::EXECUTE, true);
                    }
                };
            }
        }
    }
}
fn main() {
    let mut user = create_guest_for_testing_sudo();
    sudo_change_permissions(&mut user, "no permission".to_string(), Permission::WRITE, true);
    println!(" VALORE: {}", *user.actions[0]
                            .permissions
                            .get(&Permission::WRITE)
                            .expect("Key WRITE always present in action."))
}

fn create_guest_for_testing_sudo() -> User {
    User {
        name: "test sudo".to_string(),
        role: Role::GUEST,
        actions: vec! [Actions{
            action: "no permission".to_string(),
            permissions: HashMap::from([
                (Permission::READ, false),
                (Permission::WRITE, false),
                (Permission::EXECUTE, false)
            ])
        }]
    }
}
