use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const DEFAULT_SERVER_URL: &str = "http://172.21.191.216:8080";
const TOKEN_STORAGE_KEY: &str = "blog_token";

#[derive(Debug, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[wasm_bindgen]
pub struct BlogApp {
    server_url: String,
    token: Option<String>,
}

#[derive(Debug, Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
struct CreatePostRequest {
    title: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct UpdatePostRequest {
    title: String,
    content: String,
}

#[wasm_bindgen]
impl BlogApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<BlogApp, JsValue> {
        let token = load_token_from_storage()?;

        Ok(Self {
            server_url: DEFAULT_SERVER_URL.to_owned(),
            token,
        })
    }

    #[wasm_bindgen]
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    #[wasm_bindgen]
    pub fn logout(&mut self) -> Result<(), JsValue> {
        self.token = None;
        remove_token_from_storage()
    }

    #[wasm_bindgen]
    pub async fn login(&mut self, username: String, password: String) -> Result<JsValue, JsValue> {
        if username.trim().is_empty() || password.trim().is_empty() {
            return Err(JsValue::from_str("username and password must not be empty"));
        }

        let url = format!("{}/api/auth/login", self.server_url);
        let request = LoginRequest { username, password };
        let response = Request::post(&url)
            .json(&request)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let response = check_response(response).await?;
        let auth_response: AuthResponse = response.json().await.map_err(js_error)?;
        save_token_to_storage(&auth_response.token)?;
        self.token = Some(auth_response.token.clone());
        serde_wasm_bindgen::to_value(&auth_response).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue> {
        if username.trim().is_empty() || email.trim().is_empty() || password.trim().is_empty() {
            return Err(JsValue::from_str(
                "username, email and password must not be empty",
            ));
        }

        let url = format!("{}/api/auth/register", self.server_url);

        let request = RegisterRequest {
            username,
            email,
            password,
        };

        let response = Request::post(&url)
            .json(&request)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let response = check_response(response).await?;
        let auth_response: AuthResponse = response.json().await.map_err(js_error)?;
        save_token_to_storage(&auth_response.token)?;
        self.token = Some(auth_response.token.clone());

        serde_wasm_bindgen::to_value(&auth_response).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn load_posts(&self) -> Result<JsValue, JsValue> {
        let url = format!("{}/api/posts", self.server_url);
        let response = Request::get(&url).send().await.map_err(js_error)?;
        let response = check_response(response).await?;
        let posts: Vec<Post> = response.json().await.map_err(js_error)?;
        serde_wasm_bindgen::to_value(&posts).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn create_post(&self, title: String, content: String) -> Result<JsValue, JsValue> {
        if title.trim().is_empty() || content.trim().is_empty() {
            return Err(JsValue::from_str("title and content must not be empty"));
        }

        let token = self.require_token()?;
        let url = format!("{}/api/posts", self.server_url);

        let request = CreatePostRequest { title, content };

        let response = Request::post(&url)
            .header("Authorization", &format!("Bearer {token}"))
            .json(&request)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let response = check_response(response).await?;

        let post: Post = response.json().await.map_err(js_error)?;

        serde_wasm_bindgen::to_value(&post).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
    ) -> Result<JsValue, JsValue> {
        if title.trim().is_empty() || content.trim().is_empty() {
            return Err(JsValue::from_str("title and content must not be empty"));
        }

        let token = self.require_token()?;
        let url = format!("{}/api/posts/{id}", self.server_url);

        let request = UpdatePostRequest { title, content };

        let response = Request::put(&url)
            .header("Authorization", &format!("Bearer {token}"))
            .json(&request)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let response = check_response(response).await?;

        let post: Post = response.json().await.map_err(js_error)?;

        serde_wasm_bindgen::to_value(&post).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn delete_post(&self, id: i64) -> Result<(), JsValue> {
        let token = self.require_token()?;
        let url = format!("{}/api/posts/{id}", self.server_url);

        let response = Request::delete(&url)
            .header("Authorization", &format!("Bearer {token}"))
            .send()
            .await
            .map_err(js_error)?;

        check_response(response).await?;

        Ok(())
    }

    fn require_token(&self) -> Result<&str, JsValue> {
        self.token
            .as_deref()
            .ok_or_else(|| JsValue::from_str("authentication required"))
    }
}

fn js_error<E: std::fmt::Display>(error: E) -> JsValue {
    JsValue::from_str(&error.to_string())
}

fn storage() -> Result<web_sys::Storage, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window is unavailable"))?;

    window
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("localStorage is unavailable"))
}

fn save_token_to_storage(token: &str) -> Result<(), JsValue> {
    storage()?.set_item(TOKEN_STORAGE_KEY, token)
}

fn load_token_from_storage() -> Result<Option<String>, JsValue> {
    storage()?.get_item(TOKEN_STORAGE_KEY)
}

fn remove_token_from_storage() -> Result<(), JsValue> {
    storage()?.remove_item(TOKEN_STORAGE_KEY)
}

async fn check_response(
    response: gloo_net::http::Response,
) -> Result<gloo_net::http::Response, JsValue> {
    if response.ok() {
        return Ok(response);
    }

    let status = response.status();

    let message = response
        .text()
        .await
        .unwrap_or_else(|_| "unknown server error".to_owned());

    Err(JsValue::from_str(&format!(
        "server returned {status}: {message}"
    )))
}
