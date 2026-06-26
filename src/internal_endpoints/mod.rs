// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod admin;
pub mod mysql_proxy;

use admin::*;
use mysql_proxy::*;

use crate::*;

use execute::mysql::*;

pub static APP_INTERNAL_ENDPOINTS: LazyLock<Endpoints> = LazyLock::new(|| {
    Endpoints::new_unchecked(CheapVec::from_vec(vec![
        EndpointBuilder::default()
            .id("Whoami".into())
            .route("/whoami".into())
            .version("internal".into())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "SELECT users.|user_id_row|, users.name, users.email, roles.role FROM |users_target_table| as users LEFT JOIN |roles_target_table| as roles ON (roles.|user_id_row| = users.|user_id_row|) WHERE (users.|user_id_row| = |user_id|)"
                    .into(),
            )))
            .description("Returns the current user.".into())
            .require_auth(true)
            .inject_user_id(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("Bootstrap".into())
            .route("/bootstrap".into())
            .version("internal".into())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecuteProxy::new_many(
                 CheapVec::from_vec(vec!["INSERT INTO |roles_target_table| (|user_id_row|, role) SELECT users.|user_id_row|, 'admin' FROM |users_target_table| as users WHERE (SELECT COUNT(*) FROM |users_target_table|) = 1;".into(), "SELECT roles.role FROM |roles_target_table| as roles WHERE (roles.|user_id_row| = |user_id|)".into()]
            ))))
            .description("If there is only a single user, give it the admin role.".into())
            .require_auth(true)
            .inject_user_id(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("ListUsers".into())
            .route("/users".into())
            .version("internal/admin".into())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "SELECT users.|user_id_row|, users.name, users.email, roles.role, users.password FROM |users_target_table| as users LEFT JOIN |roles_target_table| as roles ON (roles.|user_id_row| = users.|user_id_row|)"
                    .into(),
            )))
            .description("Returns all users.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetUser".into())
            .route("/users/{user_id}".into())
            .version("internal/admin".into())
            .method(HttpMethod::Get)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "SELECT users.|user_id_row|, users.name, users.email, roles.role, users.password FROM |users_target_table| as users LEFT JOIN |roles_target_table| as roles ON (roles.|user_id_row| = users.|user_id_row|) WHERE (users.|user_id_row| = {user_id})"
                    .into(),
            )))
            .description("Returns a user given its id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("NewUser".into())
            .route("/users".into())
            .version("internal/admin".into())
            .method(HttpMethod::Post)
            .execute(Arc::new(MySQLExecuteProxy::new_many(
                CheapVec::from_vec(vec!["INSERT INTO |users_target_table| (name, email, password) VALUES ({name}, {email}, {password})".into(), "INSERT INTO |roles_target_table| (|user_id_row|, role) SELECT users.|user_id_row|, {role} FROM |users_target_table| as users WHERE (users.email = {email})".into()])
            ))) // maybe `RETURNING` does not work with preparated statements?
            .description("Creates a new user without doing the signup flow (it won't generate a session token).".into())
            .body_params(CheapVec::from_vec(vec!["name".into(), "email".into(), "role".into(), "password".into()]))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetUser".into())
            .route("/users/{user_id}".into())
            .version("internal/admin".into())
            .method(HttpMethod::Put)
            .execute(Arc::new(MySQLExecuteProxy::new_many(
                 CheapVec::from_vec(vec!["UPDATE |users_target_table| SET name={name}, email={email}, password={password} WHERE (|user_id_row| = {user_id})".into(), "REPLACE INTO |roles_target_table| (|user_id_row|, role) VALUES ({user_id}, {role})".into()])
            )))
            .description("Modifies an existing user given its id.".into())
            .body_params(CheapVec::from_vec(vec!["name".into(), "email".into(), "role".into(), "password".into()]))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteUser".into())
            .route("/users/{user_id}".into())
            .version("internal/admin".into())
            .method(HttpMethod::Delete)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "DELETE FROM |users_target_table| WHERE (|user_id_row| = {user_id});".into(),
            )))
            .description("Deletes an user given its id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteRole".into())
            .route("/users/{user_id}/role".into())
            .version("internal/admin".into())
            .method(HttpMethod::Delete)
            .execute(Arc::new(MySQLExecuteProxy::new(
                "DELETE FROM |roles_target_table| WHERE (|user_id_row| = {user_id});".into(),
            )))
            .description("Deletes the role of a given user id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("ListEndpoints".into())
            .route("endpoints".into())
            .version("internal".into())
            .method(HttpMethod::Get)
            .execute(Arc::new(EndpointsManager))
            .description("Lists all endpoints of the Silence app.".into())
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetEndpoint".into())
            .route("endpoints/{endpoint_id}".into())
            .version("internal".into())
            .method(HttpMethod::Get)
            .execute(Arc::new(EndpointsManager))
            .description("Gets an endpoint.".into())
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("NewEndpoint".into())
            .route("endpoints".into())
            .version("internal/admin".into())
            .method(HttpMethod::Post)
            .description("Creates a new endpoint.".into())
            .execute(Arc::new(EndpointsManager))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetEndpoint".into())
            .route("endpoints/{endpoint_id}".into())
            .version("internal/admin".into())
            .method(HttpMethod::Put)
            .execute(Arc::new(EndpointsManager))
            .description("Modifies an existing endpoint.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .capture_all_params(true)
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteEndpoint".into())
            .route("endpoints/{id}".into())
            .version("internal/admin".into())
            .method(HttpMethod::Delete)
            .description("Deletes an existing endpoint.".into())
            .execute(Arc::new(EndpointsManager))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetConfig".into())
            .route("config".into())
            .version("internal/admin".into())
            .method(HttpMethod::Get)
            .description("Returns Silence app's settings.".into())
            .execute(Arc::new(ConfigManager))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .auto_generated(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetConfig".into())
            .route("config".into())
            .version("internal/admin".into())
            .method(HttpMethod::Put)
            .description("Sets Silence app's settings.".into())
            .execute(Arc::new(ConfigManager))
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
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

    if let Some(_) = APP_INTERNAL_ENDPOINTS
        .inner()
        .iter()
        .find(|_endpoint| *_endpoint.id() == endpoint.id())
    {
        return true;
    }

    return false;
}
