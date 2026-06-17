// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod manage_endpoints;
pub mod mysql_proxy;

use manage_endpoints::*;
use mysql_proxy::*;

use crate::*;

use execute::mysql::*;

pub static INTERNAL_ENDPOINTS: LazyLock<Endpoints> = LazyLock::new(|| {
    Endpoints::new(CheapVec::from_vec(vec![
        EndpointBuilder::default()
            .id("Whoami".to_compact_string())
            .route("/whoami".to_compact_string())
            .version("internal".to_compact_string())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "SELECT * FROM |users_target_table| WHERE (|user_id_row| = |user_id|)"
                    .to_compact_string(),
            )))
            .description("Returns the current user.".to_compact_string())
            .require_auth(true)
            .inject_user_id(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("AmIAdmin".to_compact_string())
            .route("/verify".to_compact_string())
            .version("internal".to_compact_string())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "SELECT (roles.role = 'admin') as 'is_admin' FROM |roles_target_table| as roles WHERE (|user_id_row| = |user_id|)"
                    .to_compact_string(),
            )))
            .description("Verify whether the current user role is admin.".to_compact_string())
            .require_auth(true)
            .inject_user_id(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("ListEndpoints".to_compact_string())
            .route("endpoints".to_compact_string())
            .version("internal".to_compact_string())
            .method(HttpMethod::Get)
            .execute(Arc::new(ManageEndpoints))
            .description("Lists all endpoints of the Silence app.".to_compact_string())
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetEndpoint".to_compact_string())
            .route("endpoints/{endpoint_id}".to_compact_string())
            .version("internal".to_compact_string())
            .method(HttpMethod::Get)
            .execute(Arc::new(ManageEndpoints))
            .description("Gets an endpoint.".to_compact_string())
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("NewEndpoint".to_compact_string())
            .route("endpoints".to_compact_string())
            .version("internal/admin".to_compact_string())
            .method(HttpMethod::Post)
            .description("Creates a new endpoint.".to_compact_string())
            .execute(Arc::new(ManageEndpoints))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetEndpoint".to_compact_string())
            .route("endpoints/{endpoint_id}".to_compact_string())
            .version("internal/admin".to_compact_string())
            .method(HttpMethod::Put)
            .execute(Arc::new(ManageEndpoints))
            .description("Modifies an existing endpoint.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteEndpoint".to_compact_string())
            .route("endpoints/{id}".to_compact_string())
            .version("internal/admin".to_compact_string())
            .method(HttpMethod::Delete)
            .description("Deletes an existing endpoint.".to_compact_string())
            .execute(Arc::new(ManageEndpoints))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetConfig".to_compact_string())
            .route("config".to_compact_string())
            .version("internal/admin".to_compact_string())
            .method(HttpMethod::Get)
            .description("Returns Silence app's settings.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetConfig".to_compact_string())
            .route("config".to_compact_string())
            .version("internal/admin".to_compact_string())
            .method(HttpMethod::Put)
            .description("Sets Silence app's settings.".to_compact_string())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".to_compact_string()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
    ]))
});

/// Checks whether a given endpoint is internal.
pub fn is_endpoint_internal(endpoint: &Endpoint) -> bool {
    let Some(_) = endpoint.execute().to_owned().map(|execute| {
        execute
            .to_owned()
            .into_arc_any()
            .downcast::<MySQLExecute>()
            .ok()
            .is_some()
            || execute
                .into_arc_any()
                .downcast::<MySQLExecuteProxy>()
                .ok()
                .is_some()
    }) else {
        // It's trivial that if the endpoint doesn't have a MySQL executor it's because it's internal.
        return true;
    };

    if let Some(_) = INTERNAL_ENDPOINTS
        .inner()
        .iter()
        .find(|_endpoint| *_endpoint.id() == endpoint.id())
    {
        return true;
    }

    return false;
}
