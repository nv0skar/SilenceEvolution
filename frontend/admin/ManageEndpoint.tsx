// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { EndpointsContext } from "./Endpoints";

import { createSignal, Show, useContext } from "solid-js";

import {
    useNavigate,
    useParams,
    type RouteSectionProps,
} from "@solidjs/router";

export interface EndpointStruct {
    id: string;
    route: string;
    description: string | null;
    version: string | null;
    method: string;
    query_params: Array<string>;
    body_params: Array<string>;
    query: string | null;
    require_auth: boolean;
    allowed_roles: Array<string>;
    inject_user_id: boolean;
    auto_generated: boolean;
}

export default (_: RouteSectionProps) => {
    const endpoints_context = useContext(EndpointsContext);

    if (!endpoints_context) return <span></span>; // This component may be rendered twice, one of the renders will happen with endpoints' context undefined.

    const navigate = useNavigate();

    const [error, set_error] = createSignal<string | undefined>(undefined);

    const id = useParams()["id"];

    const endpoint_data =
        id !== undefined
            ? endpoints_context.endpoints_data.filter((endpoint_pair) => {
                  return endpoint_pair.endpoint.id == id;
              })[0]!
            : undefined;

    const submit_endpoint = async () => {
        const form = document.getElementById("form")! as HTMLFormElement;

        const form_data = Object.fromEntries(new FormData(form));

        const req = {
            id: form_data["id"]?.toString(),
            route: form_data["route"]?.toString(),
            description: form_data["description"]?.toString(),
            version: form_data["version"]?.toString(),
            method: form_data["method"]?.toString().toLowerCase(),
            query_params: Array.from(
                (form_data["query_params"]?.toString() as string)
                    .replaceAll(" ", "")
                    .split(",")
                    .filter((val) => val.length !== 0),
            ),
            body_params: Array.from(
                (form_data["body_params"]?.toString() as string)
                    .replaceAll(" ", "")
                    .split(",")
                    .filter((val) => val.length !== 0),
            ),
            query: form_data["query"]?.toString(),
            require_auth:
                form_data["require_auth"]?.toString() == "on" ? true : false,
            allowed_roles: Array.from(
                (form_data["allowed_roles"]?.toString() as string)
                    .replaceAll(" ", "")
                    .split(",")
                    .filter((val) => val.length !== 0),
            ),
            inject_user_id:
                form_data["inject_user_id"]?.toString() == "on" ? true : false,
            auto_generated:
                form_data["auto_generated"]?.toString() == "on" ? true : false,
        } as EndpointStruct;

        if (endpoint_data) {
            for (const field in req) {
                const value = req[field as keyof EndpointStruct];
                if (
                    value === undefined ||
                    (!Array.isArray(value) && (value as string).length === 0)
                )
                    delete req[field as keyof EndpointStruct];
            }
        }

        const res = await fetch(
            `/api/internal/admin/endpoints/${endpoint_data ? endpoint_data!.endpoint.id : ""}${(form_data["path"] ?? "".length !== 0) ? `?target_file=${form_data["path"]}` : ""}`,
            {
                method: endpoint_data ? "put" : "post",
                body: JSON.stringify(req),
            },
        );

        if (!res.ok) {
            console.clear();

            const data = (await res.json()) as {
                error: string;
            };

            set_error(data.error);

            return;
        }

        endpoints_context.refetch_endpoints();

        navigate("/endpoints", {
            replace: false,
            scroll: false,
        });
    };

    const delete_endpoint = async () => {
        const res = await fetch(
            `/api/internal/admin/endpoints/${endpoint_data!.endpoint.id}`,
            { method: "delete" },
        );

        if (!res.ok) console.clear();

        if (res.status === 200) {
            endpoints_context.refetch_endpoints();

            navigate("/endpoints", {
                replace: false,
                scroll: false,
            });
        } else {
            const data = (await res.json()) as {
                error: string;
            };

            const delete_button = document.getElementById("delete_endpoint")!;

            delete_button.innerText = "Retry";
            delete_button.setAttribute("data-confirmed", "false");

            set_error(data.error);
        }
    };

    return (
        <>
            <div class="grid gap-2 py-8">
                <div>
                    <h1 class="text-3xl text-center pt-4 font-bold">
                        {endpoint_data !== undefined
                            ? endpoint_data.endpoint.id
                            : "New endpoint"}
                    </h1>
                </div>
                <Show
                    when={
                        endpoint_data === undefined ||
                        endpoint_data.endpoint.query !== null ||
                        !endpoint_data.endpoint.version?.includes("internal")
                    }
                    fallback={
                        <span>
                            This endpoint can't be modified nor deleted.
                        </span>
                    }
                >
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
                                <span class="label">ID</span>
                                <input
                                    name="id"
                                    type="text"
                                    class="input peer"
                                    placeholder={
                                        endpoint_data
                                            ? endpoint_data.endpoint.id
                                            : "ID"
                                    }
                                    required={endpoint_data === undefined}
                                />
                                <p class="text-info pt-1 hidden peer-focus:block">
                                    An unique identifier for each endpoint.
                                </p>
                            </label>

                            <label>
                                <span class="label">Route</span>
                                <input
                                    name="route"
                                    type="text"
                                    class="input"
                                    value={
                                        endpoint_data
                                            ? endpoint_data.endpoint.route
                                            : ""
                                    }
                                    placeholder="Route"
                                    required={endpoint_data === undefined}
                                />
                            </label>

                            <label>
                                <span class="label">Method</span>
                                <input
                                    name="method"
                                    type="text"
                                    class="input peer"
                                    placeholder={
                                        endpoint_data
                                            ? endpoint_data.endpoint.method
                                            : "Method"
                                    }
                                    required={endpoint_data === undefined}
                                />
                                <p class="text-info pt-1 hidden peer-focus:block">
                                    HINT: available GET, POST, PUT and DELETE.
                                </p>
                            </label>

                            <label>
                                <span class="label">Body params</span>
                                <input
                                    name="body_params"
                                    type="text"
                                    class="input peer"
                                    value={
                                        endpoint_data
                                            ? (
                                                  endpoint_data.endpoint
                                                      .body_params ?? [""]
                                              ).join(", ")
                                            : ""
                                    }
                                    placeholder="Body params"
                                />
                                <p class="text-info pt-1 hidden peer-focus:block">
                                    Parameters sent in the request's body that
                                    will be accepted by the endpoint and could
                                    be inserted in the query. Parameters should
                                    be comma-separated.
                                </p>
                            </label>
                        </fieldset>

                        <label class="my-1">
                            <span class="label text-xs">Query</span>
                            <textarea
                                name="query"
                                class="input whitespace-normal w-full h-14"
                                value={
                                    endpoint_data
                                        ? (endpoint_data.endpoint.query ?? "")
                                        : ""
                                }
                                placeholder="Query"
                                required={endpoint_data === undefined}
                            />
                        </label>

                        <details class="collapse bg-base-100 border border-base-300 rounded-2xl mt-3 my-1">
                            <summary class="collapse-title font-semibold transition duration-200 hover:bg-base-200">
                                Extras
                            </summary>
                            <div class="collapse-content">
                                <fieldset class="fieldset [&_.input]:w-full lg:grid-cols-3 gap-3">
                                    <label>
                                        <span class="label">Version</span>
                                        <input
                                            name="version"
                                            type="text"
                                            class="input"
                                            value={!endpoint_data ? "v1" : ""}
                                            placeholder={
                                                endpoint_data
                                                    ? (endpoint_data.endpoint
                                                          .version ?? "—")
                                                    : "Version"
                                            }
                                        />
                                    </label>

                                    <label>
                                        <span class="label">Description</span>
                                        <input
                                            name="description"
                                            type="text"
                                            class="input"
                                            value={
                                                endpoint_data
                                                    ? (endpoint_data.endpoint
                                                          .description ?? "")
                                                    : ""
                                            }
                                            placeholder="Description"
                                        />
                                    </label>

                                    <label>
                                        <span class="label">Query params</span>
                                        <input
                                            name="query_params"
                                            type="text"
                                            class="input peer"
                                            value={
                                                endpoint_data
                                                    ? (
                                                          endpoint_data.endpoint
                                                              .query_params ?? [
                                                              "",
                                                          ]
                                                      ).join(", ")
                                                    : ""
                                            }
                                            placeholder="Query params"
                                        />
                                        <p class="text-info pt-1 hidden peer-focus:block">
                                            Query params that will be accepted
                                            by the endpoint and could be
                                            inserted in the query (SQL).
                                            Parameters should be
                                            comma-separated.
                                        </p>
                                    </label>

                                    <label>
                                        <span class="label">Allowed roles</span>
                                        <input
                                            name="allowed_roles"
                                            type="text"
                                            class="input"
                                            value={
                                                endpoint_data
                                                    ? (
                                                          endpoint_data.endpoint
                                                              .allowed_roles ?? [
                                                              "",
                                                          ]
                                                      ).join(", ")
                                                    : ""
                                            }
                                            placeholder="Allowed roles"
                                        />
                                    </label>

                                    <label>
                                        <span class="label">Path</span>
                                        <input
                                            name="path"
                                            type="text"
                                            class="input"
                                            placeholder={
                                                endpoint_data
                                                    ? (endpoint_data.path ??
                                                      "—")
                                                    : "Path"
                                            }
                                            readOnly={
                                                endpoint_data !== undefined
                                            }
                                        />
                                    </label>
                                </fieldset>

                                <fieldset class="fieldset [&_.input]:w-full text-sm lg:grid-cols-3 gap-3 my-1">
                                    <label class="align-middle">
                                        <span class="label mr-2">
                                            Requires auth
                                        </span>
                                        <input
                                            name="require_auth"
                                            type="checkbox"
                                            class="checkbox"
                                            checked={
                                                endpoint_data
                                                    ? endpoint_data.endpoint
                                                          .require_auth
                                                    : false
                                            }
                                        />
                                    </label>

                                    <label class="align-middle">
                                        <span class="label mr-2">
                                            Injects user id into query
                                        </span>
                                        <input
                                            name="inject_user_id"
                                            type="checkbox"
                                            class="checkbox"
                                            checked={
                                                endpoint_data
                                                    ? endpoint_data.endpoint
                                                          .inject_user_id
                                                    : false
                                            }
                                        />
                                    </label>
                                </fieldset>
                            </div>
                        </details>
                    </form>
                    <div class="flex gap-2 [&_button]:rounded-xl [&_button]:hover:shadow">
                        <button
                            id="delete_endpoint"
                            class="btn btn-active bg-red-600 text-white max-h-full"
                            classList={{
                                hidden: endpoint_data === undefined,
                            }}
                            data-confirmed={false}
                            onClick={async (event) => {
                                let button = event.currentTarget;

                                if (
                                    button.getAttribute("data-confirmed")! ===
                                    "true"
                                ) {
                                    await delete_endpoint();
                                } else {
                                    button.setAttribute(
                                        "data-confirmed",
                                        "true",
                                    );
                                    button.innerText = "Are you sure?";
                                }
                            }}
                        >
                            Delete endpoint
                        </button>
                        <button
                            id="submit"
                            type="submit"
                            class="btn text-black btn-success"
                            classList={{
                                "btn-disabled": endpoint_data === undefined,
                            }}
                            onClick={submit_endpoint}
                        >
                            {endpoint_data ? "Update" : "Create endpoint"}
                        </button>
                    </div>
                </Show>
            </div>
        </>
    );
};
