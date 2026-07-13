// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "@admin/Admin.tsx";
import {
    type Endpoint,
    type EndpointByFile,
    EndpointsContext,
} from "@admin/endpoints";

import Modal from "@admin/components/Modal.tsx";

import {
    children,
    createEffect,
    createResource,
    createSignal,
    Index,
    on,
    Show,
    useContext,
} from "solid-js";

import { A, useNavigate, type RouteSectionProps } from "@solidjs/router";

import { filter, pipe, sortBy } from "remeda";

import { loadGrammar } from "@arborium/arborium";

export default (props: RouteSectionProps) => {
    const session_context = useContext(SessionContext);

    if (!session_context) throw new Error("Can't find user's context");

    const navigate = useNavigate();

    // Full data.
    const [get_full_data, set_full_data] = createSignal<boolean>(false);

    // Sort state.
    const [sort, set_sort] = createSignal<{
        id: keyof Endpoint;
        order: "asc" | "desc";
    }>({ id: "id", order: "asc" });

    // Load endpoints.
    const [endpoints, { refetch }] = createResource(
        async (): Promise<Array<EndpointByFile>> => {
            const res = await fetch("/api/internal/endpoints");

            if (!res.ok) console.clear();

            if (res.status === 200) {
                const data = (await res.json()) as Array<
                    Array<string | Array<Endpoint>>
                >;

                let endpoints: Array<EndpointByFile> = new Array();

                for (const endpoints_path of data) {
                    for (const endpoint of endpoints_path[1] as Array<Endpoint>) {
                        // Instantiate endpoint in order to define a fixed order.
                        endpoints.push({
                            path: endpoints_path[0] as string | null,
                            endpoint: {
                                id: endpoint.id,
                                database: endpoint.database,
                                route: endpoint.route,
                                version: endpoint.version,
                                method: endpoint.method.toUpperCase(),
                                execute: endpoint.execute,
                                query_params: endpoint.query_params,
                                body_params: endpoint.body_params,
                                description: endpoint.description,
                                require_auth: endpoint.require_auth,
                                allowed_roles: endpoint.allowed_roles,
                                inject_auth_metadata:
                                    endpoint.inject_auth_metadata,
                                auto_generated: endpoint.auto_generated,
                            },
                        } as EndpointByFile);
                    }
                }

                return endpoints;
            } else {
                return new Array();
            }
        },
    );

    const [endpoints_list, set_endpoints_list] = createSignal<
        Array<EndpointByFile>
    >(new Array());

    // Load SQL grammar.
    const [sql_grammar] = createResource(async () => {
        return (await loadGrammar("sql"))!;
    });

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

    createEffect(() => {
        if (endpoints() !== undefined)
            set_endpoints_list(
                pipe(
                    endpoints()!,
                    sortBy([
                        (endpoint_by_file) =>
                            endpoint_by_file.endpoint[sort().id]!,
                        sort().order,
                    ]),
                ),
            );
    });

    createEffect(() => {
        if (endpoints !== undefined)
            set_endpoints_list(
                pipe(
                    endpoints()!,
                    filter((endpoint_by_file) => {
                        return (
                            get_full_data() ||
                            (endpoint_by_file.endpoint.execute! !== null &&
                                !endpoint_by_file.endpoint.version?.includes(
                                    "internal",
                                ))
                        );
                    }),
                    sortBy([
                        (endpoint_by_file) =>
                            endpoint_by_file.endpoint[sort().id]!,
                        sort().order,
                    ]),
                ),
            );
    });

    createEffect(() =>
        set_endpoints_list(
            pipe(
                endpoints_list(),
                sortBy([
                    (endpoint_by_file) => endpoint_by_file.endpoint[sort().id]!,
                    sort().order,
                ]),
            ),
        ),
    );

    return (
        <>
            <div>
                <div class="flex items-center w-full">
                    <h1 class="text-2xl lg:text-4xl font-bold">Endpoints</h1>
                    <div class="flex gap-2 self-end text-right ml-auto items-center *:rounded-2xl">
                        <A
                            class="btn text-sm self-end text-right ml-auto"
                            href="/endpoints/new"
                        >
                            <span class="material-symbols-outlined lg:hidden!">
                                add
                            </span>
                            <span class="not-lg:hidden">
                                Create new endpoint
                            </span>
                        </A>
                        <button
                            class="btn text-sm self-end text-right ml-auto"
                            onClick={refetch}
                        >
                            <span class="material-symbols-outlined">
                                refresh
                            </span>
                        </button>
                        <div
                            class="py-2 btn btn-ghost bg-base-200 border border-base-300 flex items-baseline-last gap-2"
                            onClick={(_) => {
                                set_full_data(!get_full_data());
                            }}
                        >
                            <span
                                class="text-xs text-blue-500 font-semibold self-center"
                                classList={{
                                    "text-red-500": get_full_data(),
                                }}
                            >
                                {get_full_data() ? "Full data" : "Reduced data"}
                            </span>
                            <input
                                id="table-mode"
                                type="checkbox"
                                class="toggle"
                                checked={get_full_data()}
                            />
                        </div>
                    </div>
                </div>
                <Show
                    when={!endpoints.loading}
                    fallback={
                        <div class="flex w-full my-8 justify-center">
                            <span class="loading loading-spinner loading-xl"></span>
                        </div>
                    }
                >
                    <Modal parent_path="/endpoints">
                        <EndpointsContext.Provider
                            value={{
                                endpoints_data: endpoints()!,
                                refetch_endpoints: refetch,
                            }}
                        >
                            {props.children}
                        </EndpointsContext.Provider>
                    </Modal>

                    <div class="overflow-x-auto transition-all transition-discrete duration-500 starting:opacity-0 starting:scale-95">
                        <div class="table text-sm table-auto border-collapse my-6">
                            <div class="table-header-group border-b-2 border-b-base-300">
                                <div class="table-row font-bold bg-base-300 [&_div]:w-auto [&_div]:p-4 [&_div]:align-middle [&_div]:text-left [&_div]:btn [&_div]:btn-ghost [&_div]:rounded-none">
                                    <div class="table-cell rounded-tl-xl!"></div>
                                    <div
                                        class="table-cell"
                                        onClick={(_) => {
                                            set_sort({
                                                id: "id",
                                                order: "asc",
                                            });
                                        }}
                                    >
                                        ID
                                    </div>
                                    <div
                                        class="table-cell"
                                        onClick={(_) => {
                                            set_sort({
                                                id: "route",
                                                order: "asc",
                                            });
                                        }}
                                    >
                                        Route
                                    </div>
                                    <div
                                        class="table-cell"
                                        classList={{
                                            "hidden!": !get_full_data(),
                                        }}
                                        onClick={(_) => {
                                            set_sort({
                                                id: "description",
                                                order: "asc",
                                            });
                                        }}
                                    >
                                        Description
                                    </div>

                                    <div class="table-cell">Version</div>
                                    <div class="table-cell">Method</div>
                                    <div
                                        class="table-cell"
                                        classList={{
                                            "hidden!": !get_full_data(),
                                        }}
                                    >
                                        Query params
                                    </div>
                                    <div class="table-cell">Body params</div>
                                    <div
                                        class="table-cell"
                                        classList={{
                                            "hidden!": !get_full_data(),
                                        }}
                                    >
                                        Queries
                                    </div>
                                    <div class="table-cell">Requires auth</div>
                                    <div
                                        class="table-cell"
                                        classList={{
                                            "rounded-tr-xl!": !get_full_data(),
                                        }}
                                    >
                                        Allowed roles
                                    </div>
                                    <div
                                        class="table-cell"
                                        classList={{
                                            "hidden!": !get_full_data(),
                                        }}
                                    >
                                        Injects auth metadata
                                    </div>
                                    <div
                                        class="table-cell"
                                        classList={{
                                            "hidden!": !get_full_data(),
                                        }}
                                    >
                                        Auto-generated
                                    </div>
                                    <div
                                        class="table-cell rounded-tr-xl!"
                                        classList={{
                                            "hidden!": !get_full_data(),
                                        }}
                                    >
                                        Path
                                    </div>
                                </div>
                            </div>
                            <div class="table-row-group [&>div:nth-of-type(even)]:bg-base-200 [&>div]:hover:bg-base-300/80! [&>div]:transition [&>div]:duration-200 [&>div]:hover:scale-101 [&>div]:hover:cursor-pointer">
                                <Index each={endpoints_list()}>
                                    {(endpoint, ix) => (
                                        <>
                                            <div
                                                class="table-row border-b border-b-base-300 [&_span]:text-xs [&_span]:lg:text-sm [&_div]:size-auto [&_div]:p-2 [&_div]:align-middle"
                                                // @ts-ignore
                                                popovertarget={`endpoint-row-${ix}-dropdown`}
                                                style={`anchor-name:--endpoint-row-${ix}`}
                                                onPointerDown={(event) => {
                                                    const target =
                                                        event.currentTarget;

                                                    const any_open_popover =
                                                        document.querySelector(
                                                            ".dropdown:popover-open",
                                                        );
                                                    if (
                                                        any_open_popover !==
                                                        null
                                                    ) {
                                                        target.setAttribute(
                                                            "block_popover",
                                                            "true",
                                                        );
                                                    }
                                                }}
                                                onClick={(event) => {
                                                    const target =
                                                        event.currentTarget;

                                                    const block_popover =
                                                        target.getAttribute(
                                                            "block_popover",
                                                        ) === "true";

                                                    target.removeAttribute(
                                                        "block_popover",
                                                    );

                                                    if (block_popover) return;

                                                    const dropdown =
                                                        target.nextSibling as HTMLUListElement;

                                                    dropdown.togglePopover();
                                                }}
                                            >
                                                <div class="table-cell font-light pl-8! align-middle">
                                                    {ix}
                                                </div>
                                                <div class="table-cell font-black">
                                                    {endpoint().endpoint.id}
                                                </div>
                                                <div class="table-cell font-mono">
                                                    {endpoint().endpoint.route}
                                                </div>
                                                <div
                                                    class="table-cell"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                    }}
                                                >
                                                    {endpoint().endpoint
                                                        .description ?? "—"}
                                                </div>
                                                <div class="table-cell font-mono">
                                                    {
                                                        endpoint().endpoint
                                                            .version
                                                    }
                                                </div>
                                                <div
                                                    class="table-cell text-cyan-500"
                                                    classList={{
                                                        "text-green-500!":
                                                            endpoint()
                                                                .endpoint.method.toLowerCase()
                                                                .includes(
                                                                    "get",
                                                                ),
                                                        "text-red-500!":
                                                            endpoint()
                                                                .endpoint.method.toLowerCase()
                                                                .includes(
                                                                    "delete",
                                                                ),
                                                    }}
                                                >
                                                    {endpoint().endpoint.method}
                                                </div>
                                                <div
                                                    class="table-cell font-mono"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                    }}
                                                >
                                                    {endpoint().endpoint
                                                        .query_params
                                                        ? endpoint().endpoint.query_params.join(
                                                              ", ",
                                                          )
                                                        : "—"}
                                                </div>
                                                <div class="table-cell font-mono">
                                                    {endpoint().endpoint
                                                        .body_params
                                                        ? endpoint().endpoint.body_params.join(
                                                              ", ",
                                                          )
                                                        : "—"}
                                                </div>
                                                <div
                                                    class="table-cell"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                    }}
                                                >
                                                    {
                                                        // @ts-ignore
                                                        (): Element => {
                                                            const [
                                                                formatted_sql,
                                                                set_formatted_sql,
                                                            ] =
                                                                createSignal<string>(
                                                                    "—",
                                                                );

                                                            createEffect(
                                                                async () => {
                                                                    set_formatted_sql(
                                                                        await sql_grammar()?.highlight(
                                                                            endpoint()
                                                                                .endpoint
                                                                                .execute !==
                                                                                null
                                                                                ? endpoint()
                                                                                      .endpoint.execute!.queries!.map(
                                                                                          (
                                                                                              query,
                                                                                          ) =>
                                                                                              query.query,
                                                                                      )
                                                                                      .join(
                                                                                          ";\n",
                                                                                      )
                                                                                : "—",
                                                                        )!,
                                                                    );
                                                                },
                                                            );

                                                            return (
                                                                <>
                                                                    <Show
                                                                        when={
                                                                            !sql_grammar.loading &&
                                                                            formatted_sql() !==
                                                                                undefined
                                                                        }
                                                                    >
                                                                        <span
                                                                            class="font-mono"
                                                                            innerHTML={
                                                                                formatted_sql() as string
                                                                            }
                                                                        ></span>
                                                                    </Show>
                                                                </>
                                                            ) as Element;
                                                        }
                                                    }
                                                </div>
                                                <div
                                                    class="table-cell"
                                                    classList={{
                                                        "text-success":
                                                            endpoint().endpoint
                                                                .require_auth,
                                                        "text-error":
                                                            !endpoint().endpoint
                                                                .require_auth,
                                                    }}
                                                >
                                                    {endpoint().endpoint
                                                        .require_auth
                                                        ? "Yes"
                                                        : "No"}
                                                </div>
                                                <div class="table-cell font-mono">
                                                    {endpoint().endpoint
                                                        .allowed_roles
                                                        ? endpoint().endpoint.allowed_roles.join(
                                                              ", ",
                                                          )
                                                        : "—"}
                                                </div>
                                                <div
                                                    class="table-cell"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                        "text-success":
                                                            endpoint().endpoint
                                                                .inject_auth_metadata,
                                                        "text-error":
                                                            !endpoint().endpoint
                                                                .inject_auth_metadata,
                                                    }}
                                                >
                                                    {endpoint().endpoint
                                                        .inject_auth_metadata
                                                        ? "Yes"
                                                        : "No"}
                                                </div>
                                                <div
                                                    class="table-cell"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                        "text-success":
                                                            endpoint().endpoint
                                                                .auto_generated,
                                                        "text-error":
                                                            !endpoint().endpoint
                                                                .auto_generated,
                                                    }}
                                                >
                                                    {endpoint().endpoint
                                                        .auto_generated
                                                        ? "Yes"
                                                        : "No"}
                                                </div>
                                                <div
                                                    class="table-cell font-mono pr-4!"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                    }}
                                                >
                                                    {endpoint().path ?? "—"}
                                                </div>
                                            </div>
                                            <ul
                                                id={`endpoint-row-${ix}-dropdown`}
                                                class="dropdown menu w-1/4 rounded-box bg-base-200/25 border-base-300 border backdrop-blur-sm backdrop-brightness-110 shadow-lg opacity-0 [&:popover-open]:opacity-100 starting:opacity-0 transition-all transition-discrete duration-200"
                                                classList={{
                                                    hidden:
                                                        resolved_children() !==
                                                        undefined,
                                                }}
                                                style={`position-anchor:--endpoint-row-${ix}; inset: auto; align-self: anchor-center; justify-self: anchor-left; margin: 0.5rem;`}
                                                onClick={(event) =>
                                                    (
                                                        event.currentTarget as HTMLUListElement
                                                    ).togglePopover()
                                                }
                                                onMouseLeave={(event) =>
                                                    (
                                                        event.currentTarget as HTMLUListElement
                                                    ).togglePopover()
                                                }
                                                popover
                                            >
                                                <li>
                                                    <button
                                                        onClick={(_) => {
                                                            navigate(
                                                                `/endpoints/${endpoint().endpoint.id}/modify`,
                                                                {
                                                                    replace: false,
                                                                    scroll: false,
                                                                },
                                                            );
                                                        }}
                                                    >
                                                        <span class="material-symbols-outlined scale-90">
                                                            data_object
                                                        </span>
                                                        <span class="font-bold">
                                                            Modify
                                                        </span>
                                                    </button>
                                                    <button
                                                        onClick={(_) => {
                                                            navigate(
                                                                `/endpoints/${endpoint().endpoint.id}/test`,
                                                                {
                                                                    replace: false,
                                                                    scroll: false,
                                                                },
                                                            );
                                                        }}
                                                    >
                                                        <span class="material-symbols-outlined scale-90">
                                                            science
                                                        </span>
                                                        <span class="font-bold">
                                                            Test
                                                        </span>
                                                    </button>
                                                </li>
                                            </ul>
                                        </>
                                    )}
                                </Index>
                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </>
    );
};
