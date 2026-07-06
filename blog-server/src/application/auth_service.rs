use std::collections::HashMap;

use crate::domain::{self, user::User};

pub struct AuthService {
    users: HashMap<u64, User>,
}


impl AuthService {
    
}