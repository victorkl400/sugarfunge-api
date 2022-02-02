use crate::state::*;
use serde::{Deserialize, Serialize};
// use serde_json::Value;
use actix_web::{
    web,
    Responder,
    HttpRequest,
    ResponseError,
    http::{header, StatusCode}
};
use awc::{self};

#[derive(Debug,Serialize, Deserialize)]
pub struct UserInfo {
    id: String,
    attributes: UserAtributes,
    email: String,
    #[serde(rename = "emailVerified", default)]
    email_verified: bool,
    username: String,
    #[serde(rename = "firstName", default)]
    first_name: String,
    #[serde(rename = "lastName", default)]
    last_name: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct UserAtributes {
    #[serde(rename = "user-seed", default)]
    user_seed: Box<[String]>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSeedOutput {
    seed: Option<String>,
}


pub async fn get_seed(
    _data: web::Data<AppState>,
    request: HttpRequest,
) -> Result<web::Json<UserSeedOutput>, web::Json<UserSeedOutput>> { 

    let req_headers = request.headers();
    let auth_header = req_headers.get("Authorization");
    let auth: &str = auth_header.unwrap().to_str().unwrap();

    let awc_client = awc::Client::new();

    let response = awc_client.get("http://0.0.0.0:8080/auth/realms/Sugarfunge/account")
        .append_header((header::ACCEPT, "application/json"), )
        .append_header((header::CONTENT_TYPE, "application/json"))
        .append_header((header::AUTHORIZATION, auth))
        .send()
        .await; 
        
        match response {
            Ok(mut response) => {
                
                match response.status() {
                    StatusCode::OK => {
                        let body_str: String = std::str::from_utf8(&response.body().await.unwrap()).unwrap().to_string();
                        let user_info: UserInfo = serde_json::from_str(&body_str).unwrap();

                        println!("{:?}", &user_info);
                        // println!("seed {:?}", user_info.attributes.user_seed.len());

                        if !user_info.attributes.user_seed.is_empty() {
                            let user_seed = user_info.attributes.user_seed[0].clone();
                            Ok(web::Json(
                                UserSeedOutput {
                                    seed: Some(user_seed)
                                }
                            ))
                        } else {
                            Ok(web::Json(
                                UserSeedOutput {
                                    seed: Some("".to_string())
                                }
                            ))
                        }
                    },
                    _ => Err(web::Json(
                        UserSeedOutput {
                            seed: None
                        }
                    ))
                }
            }
        Err(_) => Err(web::Json(
            UserSeedOutput {
                seed: None
            }
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct VerifySeedOutput {
    message: String,
}

pub async fn verify_seed(
    data: web::Data<AppState>,
    request: HttpRequest,
) -> impl Responder { 
    match get_seed(data, request).await {
        Ok(response) => {
            if !response.seed.clone().unwrap_or_default().is_empty() {
                web::Json(
                    VerifySeedOutput {
                        message: "user with seed".to_string()
                    }
                )
            } else {
                //TODO: add seed to user attributes
                web::Json(
                    VerifySeedOutput {
                        message: "user without seed".to_string()
                    }
                )
            }
        },
        Err(_) => {
            web::Json(
                VerifySeedOutput {
                    message: "Unknown Error".to_string()
                }
            )
        }
    }
}