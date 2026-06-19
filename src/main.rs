// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use silence::*;

use silence::internal_endpoints::mysql_proxy::*;

use waveless_commons::*;
use waveless_executor::*;

use execute::mysql::*;
use logging::*;
use runtime::handle_main;

use server::serve;

use std::net::SocketAddr;

use rustyrosetta::*;

use anyhow::{Result, anyhow};
use clap::Parser;
use compact_str::*;
use iocraft::prelude::*;
use mimalloc::MiMalloc;
use tower::util::BoxCloneService;
use tracing::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser)]
#[command(
    name = "silence",
    version,
    about = "An educational framework for deploying APIs based on a MySQL schema and web applications (Waveless' wrapper).",
    propagate_version = true
)]
struct SilenceCLI {
    /// Whether to enable debug mode in the compiler.
    #[arg(short = 'D', long = "debug", default_value_t = false)]
    debug: bool,

    /// Whether to show all the loaded endpoints on start.
    #[arg(
        short = 'd',
        long = "display_endpoints",
        default_value_t = false,
        help = "Whether to show all the loaded endpoints on start."
    )]
    display_endpoints_on_start: bool,

    /// Server listening address.
    #[arg(short = 'l', long = "listen", help = "Server listening address.")]
    addr: Option<SocketAddr>,
}

fn main() -> Result<()> {
    handle_main(try_main)
}

async fn try_main() -> Result<ResultContext> {
    let cli = SilenceCLI::parse();

    // Setup logging.
    subscribe_logging(cli.debug)?;

    // Loads Silence's app's context.
    let app_cx = AppCx::from_workspace().await?;

    match app_cx {
        Some(_app_cx) => {
            _app_cx.set_global_cx().await?;

            if cli.display_endpoints_on_start {
                let runtime_build = RuntimeCx::acquire().build().read().await;

                let mut endpoints = runtime_build.endpoints().inner().to_owned();

                // The Waveless' internal endpoints are added here temporarily to allow them to be shown in the table, as they are only injected when building the router.
                endpoints.append(&mut CheapVec::<endpoint::Endpoint>::from_vec(
                    INTERNAL_ENDPOINTS
                        .iter()
                        .map(|(_, endpoint)| endpoint)
                        .cloned()
                        .collect(),
                ));

                macro_rules! print_bool {
                    ($cond: expr) => {
                        if $cond {
                            element! {
                                Text(content: "Yes", color: Color::Green)
                            }
                        } else {
                            element! {
                                Text(content: "No", color: Color::Red)
                            }
                        }
                    };
                }

                element! {
                    View(
                        margin_top: 1,
                        margin_bottom: 1,
                        width: terminal_size::terminal_size().ok_or(anyhow!("Cannot get the terminal size."))?.0.0,
                        flex_direction: FlexDirection::Column,
                        border_style: BorderStyle::Round,
                        border_color: Color::DarkBlue,
                    ) {
                        View(border_style: BorderStyle::Single, border_edges: Edges::Bottom, border_color: Color::Grey) {
                            View(width: 200pct, justify_content: JustifyContent::End, padding_right: 2) {
                                Text(content: "Id", weight: Weight::Bold, decoration: TextDecoration::Underline)
                            }
                            #(
                                {
                                    const COLUMNS: &[&str] = &["Method", "Route", "Query", "Requires auth", "Injects user id", "Allowed Roles", "Auto Generated"];

                                    COLUMNS.iter().map(|column| element! {
                                        View(width: 250pct) {
                                            Text(content: *column, weight: Weight::Bold, decoration: TextDecoration::Underline)
                                        }
                                    })
                                }
                            )
                        }

                        #(endpoints.iter().enumerate().map(|(i, endpoint)| element! {
                            View(background_color: if i % 2 == 0 { None } else { Some(Color::DarkGrey) }) {
                                View(width: 200pct, justify_content: JustifyContent::End, padding_right: 2) {
                                    Text(content: endpoint.id().to_string(), weight: Weight::Bold)
                                }
                                View(width: 250pct) {
                                    Text(content: endpoint.method().to_string())
                                }
                                View(width: 250pct) {
                                    Text(content: format!("api/{}{}", endpoint.version().to_owned().map(|path| format!("{}/", path.trim_matches('/'))).unwrap_or_default(), endpoint.route().to_string().trim_matches('/')))
                                }
                                View(width: 250pct) {
                                    #(
                                        if let Some(execute) = endpoint
                                            .execute()
                                            .to_owned()
                                            .map(|execute| {
                                                execute
                                                    .into_arc_any()
                                                    .downcast::<MySQLExecute>()
                                                    .ok()
                                            })
                                            .flatten()
                                        {
                                            element! {
                                                View {
                                                    Text(content: execute.query().to_string())
                                                }
                                            }
                                        } else if let Some(execute) = endpoint
                                            .execute()
                                            .to_owned()
                                            .map(|execute| {
                                                execute
                                                    .into_arc_any()
                                                    .downcast::<MySQLExecuteProxy>()
                                                    .ok()
                                            })
                                            .flatten() {
                                            element! {
                                                View {
                                                    Text(content: execute.query().to_string(), color: Color::Blue)
                                                    Text(content: "(runtime parameters injected)", color: Color::DarkRed)
                                                }
                                            }
                                        } else {
                                            element! {
                                                View {
                                                        Text(content: "Internal", color: Color::Blue)
                                                }
                                            }
                                        }
                                    )
                                }
                                View(width: 250pct) {
                                    #(print_bool!(*endpoint.require_auth()))
                                }
                                View(width: 250pct) {
                                    #(print_bool!(*endpoint.inject_user_id()))
                                }
                                View(width: 250pct) {
                                    Text(content: if !endpoint.allowed_roles().is_empty() {
                                        endpoint.allowed_roles().iter().map(|role| role.to_string()).collect::<Vec<_>>().join(", ")
                                    } else {
                                        "Any".to_string()
                                    }, color: if !endpoint.allowed_roles().is_empty() {
                                        Color::Green
                                    } else {
                                        Color::Red
                                    })
                                }
                                View(width: 250pct) {
                                    #(print_bool!(*endpoint.auto_generated()))
                                }
                            }
                        }))
                    }
                }.print();
            }

            let runtime_executor = RuntimeCx::acquire()
                .build()
                .read()
                .await
                .executor()
                .to_owned();

            serve(
                match runtime_executor.listening_addr() {
                    Some(addr) => Some(addr.to_owned()),
                    None => cli.addr,
                },
                BoxCloneService::new(StaticService),
            )
            .await?;
        }
        None => {
            info!("Setup mode.")
        }
    }

    Ok("".to_compact_string())
}
