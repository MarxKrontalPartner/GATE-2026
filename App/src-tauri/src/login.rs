use serde::Deserialize;
use serde_json::{json, Value};
use tauri::State;
use tokio::sync::Mutex;

const API: &str = "http://127.0.0.1:8000/api";

#[derive(Deserialize)]
struct TokenPair {
    access: String,
    refresh: String,
}

#[derive(Deserialize)]
struct Refreshed {
    access: String,
}

#[derive(Default)]
struct Tokens {
    access: Option<String>,
    refresh: Option<String>,
}

#[derive(Default)]
pub struct AuthState {
    tokens: Mutex<Tokens>,
    client: reqwest::Client,
}

impl AuthState {
    pub async fn login(&self, username: &str, password: &str) -> Result<(), String> {
        let res = self
            .client
            .post(format!("{API}/token/"))
            .json(&json!({ "username": username, "password": password }))
            .send()
            .await
            .map_err(|_| "Server not reachable".to_string())?;

        match res.status() {
            s if s.is_success() => {}
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err("Wrong username or password".into())
            }
            reqwest::StatusCode::BAD_REQUEST => {
                return Err("Username and password are required".into())
            }
            s => return Err(format!("Login failed: {s}")),
        }

        let pair: TokenPair = res.json().await.map_err(|e| e.to_string())?;
        *self.tokens.lock().await = Tokens {
            access: Some(pair.access),
            refresh: Some(pair.refresh),
        };
        Ok(())
    }

    pub async fn logout(&self) {
        *self.tokens.lock().await = Tokens::default();
    }

    pub async fn is_logged_in(&self) -> bool {
        self.tokens.lock().await.refresh.is_some()
    }

    async fn refresh(&self) -> Result<String, String> {
        let refresh = self
            .tokens
            .lock()
            .await
            .refresh
            .clone()
            .ok_or("NOT_LOGGED_IN")?;

        let res = self
            .client
            .post(format!("{API}/token/refresh/"))
            .json(&json!({ "refresh": refresh }))
            .send()
            .await
            .map_err(|_| "Server not reachable".to_string())?;

        if !res.status().is_success() {
            self.logout().await;
            return Err("NOT_LOGGED_IN".into());
        }

        let data: Refreshed = res.json().await.map_err(|e| e.to_string())?;
        self.tokens.lock().await.access = Some(data.access.clone());
        Ok(data.access)
    }

    pub async fn api(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value, String> {
        let method = reqwest::Method::from_bytes(method.to_uppercase().as_bytes())
            .map_err(|e| e.to_string())?;

        let cached = self.tokens.lock().await.access.clone();
        let mut token = match cached {
            Some(t) => t,
            None => self.refresh().await?,
        };

        for attempt in 0..2 {
            let mut req = self
                .client
                .request(method.clone(), format!("{API}{path}"))
                .bearer_auth(&token);
            if let Some(b) = &body {
                req = req.json(b);
            }
            let res = req
                .send()
                .await
                .map_err(|_| "Server not reachable".to_string())?;
            let status = res.status();

            if status == reqwest::StatusCode::UNAUTHORIZED {
                if attempt == 0 {
                    token = self.refresh().await?;
                    continue;
                }
                self.logout().await;
                return Err("NOT_LOGGED_IN".into());
            }
            if !status.is_success() {
                return Err(format!("{status}: {}", res.text().await.unwrap_or_default()));
            }
            if status == reqwest::StatusCode::NO_CONTENT {
                return Ok(Value::Null);
            }
            return res.json().await.map_err(|e| e.to_string());
        }
        Err("NOT_LOGGED_IN".into())
    }
}

#[tauri::command]
pub async fn login(
    state: State<'_, AuthState>,
    username: String,
    password: String,
) -> Result<(), String> {
    state.login(&username, &password).await
}

#[tauri::command]
pub async fn logout(state: State<'_, AuthState>) -> Result<(), String> {
    state.logout().await;
    Ok(())
}

#[tauri::command]
pub async fn is_logged_in(state: State<'_, AuthState>) -> Result<bool, String> {
    Ok(state.is_logged_in().await)
}

#[tauri::command]
pub async fn api(
    state: State<'_, AuthState>,
    method: String,
    path: String,
    body: Option<Value>,
) -> Result<Value, String> {
    state.api(&method, &path, body).await
}
