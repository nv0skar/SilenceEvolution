// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use waveless_commons::{
    endpoint::{Endpoints, HttpMethod},
    *,
};

use execute::mysql::*;

pub static INTERNAL_ENDPOINTS: LazyLock<Endpoints> = LazyLock::new(|| {
    Endpoints::new(CheapVec::from_vec(vec![
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("Whoami".to_compact_string())
            .route("/whoami".to_compact_string())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecute::new(
                "SELECT * FROM Usuarios WHERE (usuarioId = |user_id|)".to_compact_string(),
            )))
            .description("Returns the current user.".to_compact_string())
            .require_auth(true)
            .inject_user_id(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("ListEndpoints".to_compact_string())
            .route("/".to_compact_string())
            .method(HttpMethod::Get)
            .description("Lists all endpoints of the Silence app.".to_compact_string())
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("GetEndpoint".to_compact_string())
            .route("/manage/endpoints/{id}".to_compact_string())
            .method(HttpMethod::Get)
            .description("Gets an endpoint.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("NewEndpoint".to_compact_string())
            .route("/manage/endpoints".to_compact_string())
            .method(HttpMethod::Post)
            .description("Creates a new endpoint.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("SetEndpoint".to_compact_string())
            .route("/manage/endpoints/{id}".to_compact_string())
            .method(HttpMethod::Put)
            .description("Modifies an existing endpoint.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("DeleteEndpoint".to_compact_string())
            .route("/manage/endpoints/{id}".to_compact_string())
            .method(HttpMethod::Delete)
            .description("Deletes an existing endpoint.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("GetConfig".to_compact_string())
            .route("/manage/config".to_compact_string())
            .method(HttpMethod::Get)
            .description("Returns Silence app's settings.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        waveless_commons::endpoint::EndpointBuilder::default()
            .id("SetConfig".to_compact_string())
            .route("/manage/config".to_compact_string())
            .method(HttpMethod::Post)
            .description("Sets Silence app's settings.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
    ]))
});
