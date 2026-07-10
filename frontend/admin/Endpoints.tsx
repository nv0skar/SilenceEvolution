// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "./Admin.tsx";
import { type EndpointStruct } from "./Endpoint.tsx";

import {
    children,
    createContext,
    createEffect,
    createResource,
    createSignal,
    Index,
    on,
    Show,
    useContext,
} from "solid-js";

import {
    A,
    useLocation,
    useNavigate,
    useParams,
    type RouteSectionProps,
} from "@solidjs/router";

import { loadGrammar } from "@arborium/arborium";

export const EndpointsContext = createContext<{
    endpoints_data: Array<{
        path: string | null;
        endpoint: EndpointStruct;
    }>;
    refetch_endpoints: Function;
}>();
export default (props: RouteSectionProps) => {
    const session_context = useContext(SessionContext);

    if (!session_context) throw new Error("Can't find user's context");

    const location = useLocation();

    const navigate = useNavigate();

    const params = useParams();

    // Full data.
    const [get_full_data, set_full_data] = createSignal<boolean>(false);

    // Load endpoints.
    const [endpoints, { refetch }] = createResource(
        async (): Promise<
            Array<{
                path: string | null;
                endpoint: EndpointStruct;
            }>
        > => {
            const res = await fetch("/api/internal/endpoints");

            if (!res.ok) console.clear();

            if (res.status === 200) {
                const data = (await res.json()) as Array<
                    Array<string | Array<EndpointStruct>>
                >;

                let endpoints: Array<{
                    path: string | null;
                    endpoint: EndpointStruct;
                }> = new Array();

                for (const endpoints_path of data) {
                    for (const endpoint of endpoints_path[1] as Array<EndpointStruct>) {
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
                            } as EndpointStruct,
                        });
                    }
                }

                return endpoints;
            } else {
                return [];
            }
        },
    );

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

    return (
        <>
            <div>
                <div class="flex items-center w-full">
                    <h1 class="text-4xl font-bold">Endpoints</h1>
                    <div class="flex self-end text-right ml-auto items-center">
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
                        <div
                            class="mx-2 px-2 py-2 btn btn-ghost bg-base-200 border border-base-300 rounded-2xl flex items-baseline-last gap-2"
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
                <Show when={!endpoints.loading}>
                    <div
                        class="fixed top-0 left-0 w-screen h-screen p-4 z-20 backdrop-blur-md backdrop-brightness-90 transition duration-300"
                        classList={{
                            "opacity-0 pointer-events-none":
                                resolved_children() === undefined,
                        }}
                    >
                        <div
                            class="flex h-screen justify-center items-center"
                            onClick={() =>
                                navigate("/endpoints", {
                                    replace: false,
                                    scroll: false,
                                })
                            }
                        >
                            <div
                                id="modal"
                                class="relative lg:m-32 w-full min-h-fit max-h-screen px-4 bg-base-100/75 backdrop-blur-xs border-base-300 rounded-2xl border shadow-lg transition duration-300"
                                classList={{
                                    "opacity-0 scale-75":
                                        resolved_children() === undefined,
                                }}
                                onClick={(event) => {
                                    event.stopPropagation();
                                }}
                            >
                                <div class="absolute top-0 right-0 m-2 z-50">
                                    <button
                                        class="btn btn-circle tooltip tooltip-left bg-base-300/50 border-[0.5px] border-base-200 backdrop-blur-xs shadow-xs scale-90 hover:bg-base-200"
                                        classList={{
                                            hidden: params["id"] === undefined,
                                        }}
                                        onClick={() => {
                                            navigate(
                                                location.pathname.includes(
                                                    "modify",
                                                )
                                                    ? `/endpoints/${params["id"]!}/test`
                                                    : `/endpoints/${params["id"]!}/modify`,
                                                {
                                                    replace: false,
                                                    scroll: false,
                                                },
                                            );
                                        }}
                                        data-tip={
                                            location.pathname.includes("modify")
                                                ? "Test endpoint"
                                                : "Modify endpoint"
                                        }
                                    >
                                        <span class="material-symbols-outlined">
                                            {location.pathname.includes(
                                                "modify",
                                            )
                                                ? "science"
                                                : "data_object"}
                                        </span>
                                    </button>
                                    <A
                                        class="btn btn-circle bg-base-300/50 border-[0.5px] border-base-200 backdrop-blur-xs shadow-xs scale-90 hover:bg-base-200"
                                        href="/endpoints"
                                        noScroll
                                        replace={false}
                                    >
                                        <span class="material-symbols-outlined">
                                            close
                                        </span>
                                    </A>
                                </div>

                                <div class="mx-2 my-0 max-h-[70vh] overflow-y-scroll scrollbar-none">
                                    <Show
                                        when={resolved_children() !== undefined}
                                    >
                                        <EndpointsContext.Provider
                                            value={{
                                                endpoints_data: endpoints()!,
                                                refetch_endpoints: refetch,
                                            }}
                                        >
                                            {props.children}
                                        </EndpointsContext.Provider>
                                    </Show>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-x-auto">
                        <div class="table text-sm table-auto border-collapse my-6">
                            <div class="table-header-group border-b-2 border-b-base-300">
                                <div class="table-row font-bold bg-base-200 [&_div]:w-auto [&_div]:p-4 [&_div]:align-middle [&_div]:text-left [&_div]:btn [&_div]:btn-ghost [&_div]:rounded-none">
                                    <div class="table-cell rounded-tl-xl!"></div>
                                    <div class="table-cell">ID</div>
                                    <div class="table-cell">Route</div>

                                    <div
                                        class="table-cell"
                                        classList={{
                                            "hidden!": !get_full_data(),
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
                                <Index
                                    each={endpoints()?.sort(
                                        (endpoint_pair_1, endpoint_pair_2) => {
                                            return endpoint_pair_1.endpoint.id!.localeCompare(
                                                endpoint_pair_2.endpoint.id!,
                                            );
                                        },
                                    )}
                                >
                                    {(endpoint, ix) => (
                                        <Show
                                            when={
                                                get_full_data() ||
                                                (endpoint().endpoint
                                                    .execute! !== null &&
                                                    !endpoint().endpoint.version?.includes(
                                                        "internal",
                                                    ))
                                            }
                                        >
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
                                                                .inject_auth_metadata,
                                                        "text-error":
                                                            !endpoint().endpoint
                                                                .inject_auth_metadata,
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
                                        </Show>
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
