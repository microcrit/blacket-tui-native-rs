use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

use crate::api::types::{requests, responses::GenericResponse};
use crate::utils::cookie;
use crate::consts::api::{get_endpoint, Endpoints};

#[derive(Debug)]
pub struct NeedsCodeError;

#[derive(Debug)]
pub struct GenericError;

#[derive(Debug)]
pub enum LoginResponseError {
    NeedsCode(NeedsCodeError),
    Generic(GenericError),
}

pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,

    pub token: Option<String>,
}

impl User {
    pub fn new() -> Self {
        User {
            username: None,
            password: None,
            token: None,
        }
    }
    
    pub fn new_credentials(username: &str, password: &str) -> Self {
        User {
            username: Some(username.to_string()),
            password: Some(password.to_string()),
            token: None,
        }
    }
    
    pub fn login(&mut self, username: Option<&str>, password: Option<&str>) -> Result<String, LoginResponseError> {
        let username = match username {
            Some(username) => username,
            None => self.username.as_ref().unwrap(),
        };
        let password = match password {
            Some(password) => password,
            None => self.password.as_ref().unwrap(),
        };
        let login_endpoint = get_endpoint(Endpoints::UserLogin);
        let client = Client::new();
    
        let code = None;
        let data = requests::UserRequest {
            username: username.to_string(),
            password: password.to_string(),
            code
        };
        let response = client.post(login_endpoint)
            .json(&data)
            .send()
            .unwrap();
        let response_headers: HeaderMap = response.headers().clone();
        let response_data: GenericResponse = serde_json::from_str(&response.text().unwrap()).unwrap();
    
        match response_data.reason {
            Some(reason) => {
                if reason == "You must specify a code." {
                    return Err(LoginResponseError::NeedsCode(NeedsCodeError));
                } else if reason != "" {
                    return Err(LoginResponseError::Generic(GenericError));
                } else {
                    response_headers.keys().for_each(|key| {
                        println!("{}: {}", key, response_headers.get(key).unwrap().to_str().unwrap());
                    });
                    let token = response_headers.get("Set-Cookie").unwrap().to_str().unwrap();
                    let token = cookie::parse_cookie(token);
                    let token = token.get("token").unwrap().to_string();
                    self.token = Some(token.clone());
                    self.username = Some(username.to_string());
                    return Ok(token);
                }
            },
            None => {
                let token = response_headers.get("Set-Cookie").unwrap().to_str().unwrap();
                let token = cookie::parse_cookie(token);
                let token = token.get("token").unwrap().to_string();
                self.token = Some(token.clone());
                self.username = Some(username.to_string());
                return Ok(token);
            }
        }
    }

    pub fn logout(&self) -> Result<(), String> {
        let logout_endpoint = get_endpoint(Endpoints::UserLogout);
        let client = Client::new();
        let response = client.post(logout_endpoint)
            .header("Cookie", format!("token={}", self.token.as_ref().unwrap()))
            .send()
            .unwrap();
        if response.status().as_u16() != 302 {
            return Err("Failed to logout".to_string());
        }
        Ok(())
    }
}