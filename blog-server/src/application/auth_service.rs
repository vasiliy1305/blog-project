use crate::data::user_repository::UserRepository;
use crate::domain::error::DomainError;
use crate::domain::user::{CreateUser, Login, Registration, User};
use crate::infrastructure::jwt::JwtService;

pub struct AuthService<R>
where
    R: UserRepository,
{
    jwt: JwtService,
    repository: R,
}

impl<R> AuthService<R>
where
    R: UserRepository,
{
    pub fn new(jwt: JwtService, repository: R) -> Self {
        AuthService { jwt, repository }
    }

    pub async fn register(&self, reg_data: &Registration) -> Result<AuthResponse, DomainError> {
        todo!()
    }

    pub async fn login(&self, login_info: Login) -> Result<AuthResponse, DomainError> {
        todo!()
    }
}

pub struct AuthResponse {
    pub user: User,
    pub token: String,
}
