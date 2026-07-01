// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "./Admin";
import { type EndpointStruct } from "./Endpoint";

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
    type Accessor,
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

const REDUCED_FIELDS: Array<keyof EndpointStruct> = [
    "id",
    "version",
    "route",
    "method",
    "body_params",
    "allowed_roles",
    "require_auth",
];

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
                                route: endpoint.route,
                                description: endpoint.description,
                                version: endpoint.version,
                                method: endpoint.method.toUpperCase(),
                                query_params: endpoint.query_params,
                                body_params: endpoint.body_params,
                                query: endpoint.query,
                                require_auth: endpoint.require_auth,
                                allowed_roles: endpoint.allowed_roles,
                                inject_user_id: endpoint.inject_user_id,
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

    const render_field = (
        field: Accessor<[string, any]>,
        ix: number,
    ): Element => {
        const [_, value] = field();

        switch (typeof value) {
            case "string": {
                if ((value as string).length === 0)
                    return (<span>—</span>) as Element;
                else if (ix === 7) {
                    const [formatted_sql, set_formatted_sql] =
                        createSignal<string>("—");

                    createEffect(async () => {
                        set_formatted_sql(
                            await sql_grammar()?.highlight(value)!,
                        );
                    });

                    return (
                        <Show
                            when={
                                !sql_grammar.loading &&
                                formatted_sql() !== undefined
                            }
                        >
                            <span innerHTML={formatted_sql() as string}></span>
                        </Show>
                    ) as Element;
                } else
                    return (
                        <span
                            classList={{
                                "font-bold": ix === 0,
                            }}
                        >
                            {value as string}
                        </span>
                    ) as Element;
            }
            case "boolean": {
                if (value)
                    return (<span class="text-success">Yes</span>) as Element;
                else return (<span class="text-error">No</span>) as Element;
            }
            default: {
                return (
                    <span>
                        {value ? (value as Array<string>).join(", ") : "—"}
                    </span>
                ) as Element;
            }
        }
    };

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
                        class="fixed top-0 left-0 w-screen h-screen p-4 z-20 bg-base-100/5 backdrop-blur-md transition duration-300"
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
                                class="relative lg:m-32 w-full h-fit px-4 bg-base-200/10 backdrop-blur-xs border-base-300 rounded-2xl border shadow-lg transition duration-300"
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
                                        class="btn btn-circle bg-base-300/50 border-base-200 backdrop-blur-xs shadow-2xs scale-90"
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
                                        class="btn btn-circle bg-base-300/50 border-base-200 backdrop-blur-xs shadow-2xs scale-90"
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
                                <div class="table-row font-bold bg-base-200 [&_div]:w-auto [&_div]:p-2 [&_div]:align-middle [&_div]:text-left [&_div]:btn [&_div]:btn-ghost [&_div]:rounded-none">
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
                                        Query
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
                                        Injects user id
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
                            <div class="table-row-group [&>div]:even:bg-base-200 [&>div]:hover:bg-base-300 [&>div]:transition [&>div]:duration-200 [&>div]:hover:scale-101 [&>div]:hover:cursor-pointer">
                                <Index
                                    each={endpoints()?.sort(
                                        (endpoint_pair_1, endpoint_pair_2) => {
                                            return endpoint_pair_1.endpoint.id.localeCompare(
                                                endpoint_pair_2.endpoint.id,
                                            );
                                        },
                                    )}
                                >
                                    {(endpoint, ix) => (
                                        <Show
                                            when={
                                                get_full_data() ||
                                                (endpoint().endpoint.query! !==
                                                    null &&
                                                    !endpoint().endpoint.version?.includes(
                                                        "internal",
                                                    ))
                                            }
                                        >
                                            <div
                                                class="table-row border-b border-b-base-300 [&_span]:text-xs [&_span]:lg:text-sm [&_div]:size-auto [&_div]:p-0.5 [&_div]:lg:p-1 [&_div]:align-middle"
                                                // @ts-ignore
                                                popovertarget={`endpoint-row-${ix}-dropdown`}
                                                style={`anchor-name:--endpoint-row-${ix}`}
                                                onClick={(event) => {
                                                    const dropdown = event
                                                        .currentTarget
                                                        .lastChild as HTMLUListElement;

                                                    dropdown.togglePopover();
                                                }}
                                            >
                                                <div class="table-cell font-light pl-8! align-middle">
                                                    {ix}
                                                </div>
                                                <Index
                                                    each={Object.entries(
                                                        endpoint()
                                                            .endpoint as EndpointStruct,
                                                    )}
                                                >
                                                    {(field, ix) => (
                                                        <>
                                                            <div
                                                                class="table-cell"
                                                                classList={{
                                                                    "hidden!":
                                                                        !REDUCED_FIELDS.includes(
                                                                            field()[0] as keyof EndpointStruct,
                                                                        ) &&
                                                                        !get_full_data(),
                                                                }}
                                                            >
                                                                {
                                                                    // @ts-ignore
                                                                    (): Element => {
                                                                        return render_field(
                                                                            field,
                                                                            ix,
                                                                        );
                                                                    }
                                                                }
                                                            </div>
                                                        </>
                                                    )}
                                                </Index>
                                                <div
                                                    class="table-cell font-light pr-4!"
                                                    classList={{
                                                        "hidden!":
                                                            !get_full_data(),
                                                    }}
                                                >
                                                    {endpoint().path ?? "—"}
                                                </div>

                                                <ul
                                                    id={`endpoint-row-${ix}-dropdown`}
                                                    class="dropdown dropdown-start menu w-52 rounded-box bg-base-100/25 border-base-200 border backdrop-blur-sm shadow-lg transition duration-200"
                                                    classList={{
                                                        hidden:
                                                            resolved_children() !==
                                                            undefined,
                                                    }}
                                                    style={`position-anchor:--endpoint-row-${ix}`}
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
                                            </div>
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
