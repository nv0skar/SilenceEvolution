// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "./Admin";

import {
    children,
    createEffect,
    createResource,
    createSignal,
    on,
    Show,
    useContext,
} from "solid-js";

import { type RouteSectionProps } from "@solidjs/router";
export interface ConfigStruct {
    listening_addr: string;
    serve_static_files: boolean;
    internal_params: {
        user_id: string;
        users_target_table: string;
        sessions_target_table: string;
        roles_target_table: string;
    };
    skip_endpoints_ids: Array<string>;
    database_conn: {
        host: string;
        username: string;
        password: string;
        db: string;
    };
}

export default (props: RouteSectionProps) => {
    const session_context = useContext(SessionContext);

    if (!session_context) throw new Error("Can't find user's context");

    const [error, set_error] = createSignal<string | undefined>(undefined);

    // Load config.
    const [config, { refetch }] = createResource(
        async (): Promise<ConfigStruct> => {
            const res = await fetch("/api/internal/admin/config");

            if (!res.ok) console.clear();

            if (res.status === 200) {
                const data = (await res.json()) as ConfigStruct;

                return data;
            } else {
                return {} as ConfigStruct;
            }
        },
    );

    const update_config = async () => {
        const form = document.getElementById("form")! as HTMLFormElement;

        const form_data = Object.fromEntries(new FormData(form));

        const req = {
            listening_addr: form_data["listening_addr"]
                ?.toString()
                .toLowerCase(),
            serve_static_files:
                form_data["serve_static_files"]?.toString() == "on"
                    ? true
                    : false,
            internal_params: {
                user_id: form_data["user_id_row"]?.toString().toLowerCase(),
                users_target_table: form_data["users_target_table"]
                    ?.toString()
                    .toLowerCase(),
                sessions_target_table: form_data["sessions_target_table"]
                    ?.toString()
                    .toLowerCase(),
                roles_target_table: form_data["roles_target_table"]
                    ?.toString()
                    .toLowerCase(),
            },
            skip_endpoints_ids: Array.from(
                (form_data["skip_endpoints_ids"]?.toString() as string)
                    .replaceAll(" ", "")
                    .split(",")
                    .filter((val) => val.length !== 0),
            ),
            database_conn: {
                host: form_data["db_host"]?.toString().toLowerCase(),
                username: form_data["db_username"]?.toString().toLowerCase(),
                password: form_data["db_password"]?.toString().toLowerCase(),
                db: form_data["db_name"]?.toString().toLowerCase(),
            },
        } as ConfigStruct;

        for (const field in req) {
            const value = req[field as keyof ConfigStruct];
            if (
                value === undefined ||
                (!Array.isArray(value) && (value as string).length === 0)
            )
                delete req[field as keyof ConfigStruct];
        }

        const res = await fetch(`/api/internal/admin/config`, {
            method: "put",
            body: JSON.stringify(req),
        });

        if (!res.ok) {
            console.clear();

            const data = (await res.json()) as {
                error: string;
            };

            set_error(data.error);

            return;
        }

        refetch();
    };

    const resolved_children = children(() => props.children);

    createEffect(
        on(resolved_children, (resolved_children) => {
            if (resolved_children !== undefined) {
                document.body.style.overflow = "hidden";
            } else {
                document.body.style.overflow = "";
            }
        }),
    );

    return (
        <>
            <div>
                <div class="flex items-center w-full">
                    <h1 class="text-4xl font-bold">Config</h1>
                </div>
                <Show when={!config.loading}>
                    <div class="grid gap gap-2 py-8">
                        <div class="bg-base-200 backdrop-blur shadow-xl rounded-box my-2 p-2 text-center cursor-pointer">
                            <p class="font-semibold">
                                Some changes may require a manual server
                                restart.
                            </p>
                        </div>
                        <Show when={error() != undefined}>
                            <div
                                class="bg-red-800 border-red-400 backdrop-blur shadow-xl rounded-box my-2 p-2 text-center cursor-pointer"
                                onClick={() => set_error(undefined)}
                            >
                                <p class="text-sm font-semibold">
                                    An error has occurred. {error()!}
                                </p>
                            </div>
                        </Show>
                        <form
                            id="form"
                            class="[&_span]:mb-1"
                            onInput={(event) => {
                                set_error(undefined);

                                let form = event.currentTarget;
                                let submit = document.getElementById("submit");

                                if (form.checkValidity()) {
                                    submit?.classList.remove("btn-disabled");
                                } else submit?.classList.add("btn-disabled");
                            }}
                        >
                            <fieldset class="fieldset [&_.input]:w-full lg:grid-cols-2 gap-3">
                                <label>
                                    <span class="label">
                                        Server listening address
                                    </span>
                                    <input
                                        name="name"
                                        type="text"
                                        class="input"
                                        placeholder={config()?.listening_addr}
                                        value={config()?.listening_addr}
                                    />
                                </label>

                                <label>
                                    <span class="label">
                                        Skip endpoints IDs
                                    </span>
                                    <input
                                        name="skip_endpoints_ids"
                                        type="text"
                                        class="input"
                                        value={
                                            config()?.skip_endpoints_ids
                                                ? (
                                                      config()
                                                          ?.skip_endpoints_ids ?? [
                                                          "",
                                                      ]
                                                  ).join(", ")
                                                : ""
                                        }
                                        placeholder="—"
                                    />
                                </label>
                            </fieldset>

                            <details class="collapse bg-base-100 border border-base-300 rounded-2xl mt-3 my-1">
                                <summary class="collapse-title font-semibold transition duration-200 hover:bg-base-200">
                                    Internal parameters
                                </summary>
                                <div class="collapse-content">
                                    <fieldset class="fieldset [&_.input]:w-full lg:grid-cols-3 gap-3">
                                        <label>
                                            <span class="label">
                                                User ID row
                                            </span>
                                            <input
                                                name="user_id_row"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.internal_params
                                                        .user_id
                                                }
                                                value={
                                                    config()?.internal_params
                                                        .user_id
                                                }
                                            />
                                        </label>

                                        <label>
                                            <span class="label">
                                                User's target table
                                            </span>
                                            <input
                                                name="users_target_table"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.internal_params
                                                        .users_target_table
                                                }
                                                value={
                                                    config()?.internal_params
                                                        .users_target_table
                                                }
                                            />
                                        </label>

                                        <label>
                                            <span class="label">
                                                Session's target table
                                            </span>
                                            <input
                                                name="sessions_target_table"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.internal_params
                                                        .sessions_target_table
                                                }
                                                value={
                                                    config()?.internal_params
                                                        .sessions_target_table
                                                }
                                            />
                                        </label>

                                        <label>
                                            <span class="label">
                                                Role's target table
                                            </span>
                                            <input
                                                name="roles_target_table"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.internal_params
                                                        .roles_target_table
                                                }
                                                value={
                                                    config()?.internal_params
                                                        .roles_target_table
                                                }
                                            />
                                        </label>
                                    </fieldset>
                                </div>
                            </details>

                            <details class="collapse bg-base-100 border border-base-300 rounded-2xl mt-3 my-1">
                                <summary class="collapse-title font-semibold transition duration-200 hover:bg-base-200">
                                    Database
                                </summary>
                                <div class="collapse-content">
                                    <fieldset class="fieldset [&_.input]:w-full lg:grid-cols-3 gap-3">
                                        <label>
                                            <span class="label">Host</span>
                                            <input
                                                name="db_host"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.database_conn.host
                                                }
                                                value={
                                                    config()?.database_conn.host
                                                }
                                            />
                                        </label>

                                        <label>
                                            <span class="label">Name</span>
                                            <input
                                                name="db_name"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.database_conn.db
                                                }
                                                value={
                                                    config()?.database_conn.db
                                                }
                                            />
                                        </label>

                                        <label>
                                            <span class="label">Username</span>
                                            <input
                                                name="db_username"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.database_conn
                                                        .username
                                                }
                                                value={
                                                    config()?.database_conn
                                                        .username
                                                }
                                            />
                                        </label>

                                        <label>
                                            <span class="label">Password</span>
                                            <input
                                                name="db_password"
                                                type="text"
                                                class="input"
                                                placeholder={
                                                    config()?.database_conn
                                                        .password
                                                }
                                                value={
                                                    config()?.database_conn
                                                        .password
                                                }
                                            />
                                        </label>
                                    </fieldset>
                                </div>
                            </details>

                            <label class="align-middle">
                                <span class="label mr-2 py-4">
                                    Serve static files
                                </span>
                                <input
                                    name="serve_static_files"
                                    type="checkbox"
                                    class="checkbox"
                                    checked={config()?.serve_static_files}
                                />
                            </label>
                        </form>
                        <div class="flex gap-2 [&_button]:rounded-xl [&_button]:hover:shadow">
                            <button
                                id="submit"
                                type="submit"
                                class="btn btn-disabled"
                                onClick={update_config}
                            >
                                Update
                            </button>
                        </div>
                    </div>
                </Show>
            </div>
        </>
    );
};
