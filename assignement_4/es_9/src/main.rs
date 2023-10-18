
pub mod actions;
pub mod permission;
pub mod role;
// pub mod testing;
use permission::Permission;
use role::Role;
use actions::Actions;

trait Auth{
    fn check_permission(&self, action:&str, permission_type:&Permission) -> bool;
    fn can_write(&self, string:&str) -> bool;
    fn can_read(&self, string:&str) -> bool;
    fn can_execute(&self, string:&str) -> bool;
}
struct User{
    name:String,
    role:Role,
    actions: Vec<Actions>
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

fn sudo_change_permissions(mut user: User, string:String, permission:Permission, value:bool){
    match permission{
        Permission::READ => {
            if !user.can_read(string.as_str()){
                user.actions.push(Actions::new(string, value, false, false));
            }
        },
        // todo: what am i supposed to do???
        _ => {}
    }
}
fn main() {
    println!("Hello, world!");
}
