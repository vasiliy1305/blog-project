use crate::data::user_repository::UserRepository;
use crate::domain::error::DomainError;
use crate::domain::user::{CreateUser, Login, Registration, User};
use crate::infrastructure::jwt::JwtService;
use crate::infrastructure::password::{self, Password};

pub struct AuthService<R, P>
where
    R: UserRepository,
    P: Password,
{
    jwt: JwtService,
    repository: R,
    password: P
}

impl<R, P> AuthService<R, P>
where
    R: UserRepository,
    P: Password,
{
    pub fn new(jwt: JwtService, repository: R, password: P) -> Self {
        AuthService { jwt, repository, password }
    }

    pub async fn register(&self, reg_data: &Registration) -> Result<AuthResponse, DomainError> {


       match self.repository.find_by_username(&reg_data.username).await? {
           Some(user) => Err(DomainError::UserAlreadyExists(user.username)),
           None => {
            let hash = P::hash_password(&reg_data.password)?;
            let user = &self.repository.create(
                &CreateUser { username: reg_data.username, 
                    email: reg_data.email, 
                    password_hash: hash })
                    .await?;

        let  token = &self.jwt.generate_token(user.i, username) ;


            todo!()
           }
       }

        
    }

    pub async fn login(&self, login_info: Login) -> Result<AuthResponse, DomainError> {
        todo!()
    }
}

pub struct AuthResponse {
    pub user: User,
    pub token: String,
}
