use crate::api_fn;
use lemmy_api_common::person::{Login, LoginResponse};

api_fn!(login, Login, LoginResponse, Post, "user/login");
