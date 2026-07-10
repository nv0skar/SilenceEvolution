// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod admin;
pub mod console;
pub mod mysql_proxy;

use admin::*;
use console::*;
use mysql_proxy::*;

use crate::*;

use http_execute::mysql::*;

pub static APP_INTERNAL_ENDPOINTS: LazyLock<Endpoints> = LazyLock::new(|| {
    // NOTE: if the main and internal databases are the same, two connections to the same database are still opened as the app's cx cannot be accessed from here due to the app's initialization order.

    Endpoints::new_unchecked(CheapVec::from_iter([
        EndpointBuilder::default()
            .id("Whoami".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/whoami".into())
                .version("internal".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("SELECT users.user_id, users.name, users.email, roles.role FROM |users_target_table| as users LEFT JOIN |roles_target_table| as roles ON (roles.user_id = users.user_id) WHERE (users.user_id = |user_id|)".into()).with_behaviour(MySQLBehaviour::Unique).into()
                )))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Returns the current user.".into())
            .require_auth(true)
            .inject_auth_metadata(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("Bootstrap".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/bootstrap".into())
                .version("internal".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::Many { queries: CheapVec::from_iter(
                       [
                           MySQLQuery::new("INSERT INTO |roles_target_table| (user_id, role) SELECT users.user_id, 'admin' FROM |users_target_table| as users WHERE (SELECT COUNT(*) FROM |users_target_table|) = 1".into(), false, MySQLBehaviour::Permissive),
                           MySQLQuery::new("SELECT roles.role FROM |roles_target_table| as roles WHERE (roles.user_id = |user_id|);".into(), true, MySQLBehaviour::Unique)
                       ]
                   ) }.into()
                )))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("If there is only a single user, give it the admin role.".into())
            .require_auth(true)
            .inject_auth_metadata(true)
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("ListUsers".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/users".into())
                .version("internal/admin".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("SELECT users.user_id, users.name, users.email, roles.role, users.password FROM |users_target_table| as users LEFT JOIN |roles_target_table| as roles ON (roles.user_id = users.user_id)".into()).into()
                )))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Returns all users.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetUser".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/users/{id}".into())
                .version("internal/admin".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("SELECT users.user_id, users.name, users.email, roles.role, users.password FROM |users_target_table| as users LEFT JOIN |roles_target_table| as roles ON (roles.user_id = users.user_id) WHERE (users.user_id = {id})".into()).with_behaviour(MySQLBehaviour::Unique).into()
                )))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Returns a user given its id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("NewUser".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/users".into())
                .version("internal/admin".into())
                .method(HttpMethod::Post)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("INSERT INTO |users_target_table| (name, email, password) VALUES ({name}, {email}, {password}); INSERT INTO |roles_target_table| (user_id, role) SELECT users.user_id, {role} FROM |users_target_table| as users WHERE (users.email = {email});".into()).with_include(false).into()
                ))) // maybe `RETURNING` does not work with preparated statements?
                .body_params(CheapVec::from_vec(vec!["name".into(), "email".into(), "role".into(), "password".into()]))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Creates a new user without doing the signup flow (it won't generate a session token).".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetUser".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/users/{id}".into())
                .version("internal/admin".into())
                .method(HttpMethod::Put)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("UPDATE |users_target_table| SET name={name}, email={email}, password={password} WHERE (user_id = {id}); REPLACE INTO |roles_target_table| (user_id, role) VALUES ({id}, {role});".into()).with_include(false).into()
                )))
                .body_params(CheapVec::from_vec(vec!["name".into(), "email".into(), "role".into(), "password".into()]))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Modifies an existing user given its id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteUser".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/users/{id}".into())
                .version("internal/admin".into())
                .method(HttpMethod::Delete)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("DELETE FROM |users_target_table| WHERE (user_id = {id})".into()).with_include(false).into()
                )))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Deletes an user given its id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteRole".into())
            .database("internal".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("/users/{id}/role".into())
                .version("internal/admin".into())
                .method(HttpMethod::Delete)
                .execute(Arc::new(MySQLExecuteProxy::new(
                   MySQLQueryWrapper::new("DELETE FROM |roles_target_table| WHERE (user_id = {id})".into()).with_include(false).into()
                )))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Deletes the role of a given user id.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("ListEndpoints".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("endpoints".into())
                .version("internal".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(EndpointsManager))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Lists all endpoints of the Silence app.".into())
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetEndpoint".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("endpoints/{endpoint_id}".into())
                .version("internal".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(EndpointsManager))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Gets an endpoint.".into())
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("NewEndpoint".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("endpoints".into())
                .version("internal/admin".into())
                .method(HttpMethod::Post)
                .execute(Arc::new(EndpointsManager))
                .capture_all_params(true)
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Creates a new endpoint.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetEndpoint".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("endpoints/{endpoint_id}".into())
                .version("internal/admin".into())
                .method(HttpMethod::Put)
                .execute(Arc::new(EndpointsManager))
                .capture_all_params(true)
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Modifies an existing endpoint.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("DeleteEndpoint".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("endpoints/{id}".into())
                .version("internal/admin".into())
                .method(HttpMethod::Delete)
                .execute(Arc::new(EndpointsManager))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Deletes an existing endpoint.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("GetConfig".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("config".into())
                .version("internal/admin".into())
                .method(HttpMethod::Get)
                .execute(Arc::new(ConfigManager))
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Returns Silence app's settings.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("SetConfig".into())
            .target(Targets::HttpTarget(HttpTargetBuilder::default()
                .route("config".into())
                .version("internal/admin".into())
                .method(HttpMethod::Put)
                .execute(Arc::new(ConfigManager))
                .capture_all_params(true)
                .auto_generated(true)
                .build()
                .unwrap()
            ))
            .description("Sets Silence app's settings.".into())
            .require_auth(true)
            .allowed_roles(CheapVec::from_vec(vec!["admin".into()]))
            .build()
            .unwrap(),
        EndpointBuilder::default()
            .id("ConsoleStream".into())
            .target(Targets::SocketTarget(SocketTargetBuilder::default()
                .execute(Arc::new(ConsoleStream))
                .build()
                .unwrap()
            ))
            .description("Streams Silence's console log.".into())
            .build()
            .unwrap(),
    ]))
});

/// Checks whether a given endpoint is internal.
pub fn is_endpoint_internal(endpoint: &Endpoint) -> bool {
    let Targets::HttpTarget(http_target) = endpoint.target() else {
        return true;
    };

    let Some(_) = http_target.execute().to_owned().map(|execute| {
        execute
            .to_owned()
            .into_arc_any()
            .downcast::<MySQLExecute>()
            .ok()
            .is_some()
            || execute
                .to_owned()
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
