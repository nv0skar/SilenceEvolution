// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { UserContext } from "./Admin";
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

import { A, useNavigate, type RouteSectionProps } from "@solidjs/router";

import { loadGrammar } from "@arborium/arborium";

export const EndpointsContext = createContext<{
    endpoints_data: Array<{
        path: string | null;
        endpoint: EndpointStruct;
    }>;
    refetch_endpoints: Function;
}>();

export default (props: RouteSectionProps) => {
    const user_context = useContext(UserContext);

    if (!user_context) throw new Error("Can't find user's context");

    const navigate = useNavigate();

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
                                method: endpoint.method,
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

    document.addEventListener("keydown", (event) => {
        if (event.key === "Escape")
            navigate("/endpoints", {
                replace: false,
                scroll: false,
            });
    });

    const render_field = (field: Accessor<any>, ix: number): Element => {
        switch (typeof field()) {
            case "string": {
                if ((field() as string).length === 0)
                    return (<span>—</span>) as Element;
                else if (ix === 7) {
                    const [formatted_sql, set_formatted_sql] =
                        createSignal<string>("—");

                    createEffect(async () => {
                        set_formatted_sql(
                            await sql_grammar()?.highlight(field())!,
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
                            {field()}
                        </span>
                    ) as Element;
            }
            case "boolean": {
                if (field())
                    return (<span class="text-success">Yes</span>) as Element;
                else return (<span class="text-error">No</span>) as Element;
            }
            default: {
                return (
                    <span>
                        {field() ? (field() as Array<string>).join(", ") : "—"}
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
                    <A
                        class="btn text-sm self-end text-right ml-auto"
                        href="new"
                    >
                        Create new endpoint
                    </A>
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
                                class="relative lg:m-32 w-full h-fit px-4 bg-base-200 border-base-300 rounded-2xl border shadow-lg transition duration-300"
                                classList={{
                                    "opacity- 0 scale-75":
                                        resolved_children() === undefined,
                                }}
                                onClick={(event) => {
                                    event.stopPropagation();
                                }}
                            >
                                <A
                                    class="absolute top-0 right-0 m-2 btn btn-circle bg-base-300/50 backdrop-blur-2xs shadow-2xs scale-90 z-10"
                                    href="/endpoints"
                                    noScroll
                                    replace={false}
                                >
                                    <span class="material-symbols-outlined">
                                        close
                                    </span>
                                </A>
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
                        <div class="text-sm table-auto border-collapse my-6">
                            <div class="table-header-group border-b-2 border-b-base-300">
                                <div class="table-row font-bold bg-base-200 [&_div]:p-2 [&_div]:align-middle">
                                    <div class="table-cell"></div>
                                    <div class="table-cell">ID</div>
                                    <div class="table-cell">Route</div>
                                    <div class="table-cell">Description</div>
                                    <div class="table-cell">Version</div>
                                    <div class="table-cell">Method</div>
                                    <div class="table-cell">Query params</div>
                                    <div class="table-cell">Body params</div>
                                    <div class="table-cell">Query</div>
                                    <div class="table-cell">Requires auth</div>
                                    <div class="table-cell">Allowed roles</div>
                                    <div class="table-cell">
                                        Injects user id
                                    </div>
                                    <div class="table-cell">Auto-generated</div>
                                    <div class="table-cell">Path</div>
                                </div>
                            </div>
                            <div class="table-row-group [&>div]:hover:bg-base-300 [&>div]:transition [&>div]:duration-200 [&>div]:hover:scale-101 [&>div]:shadow-xl [&>div]:hover:cursor-pointer">
                                <Index each={endpoints()}>
                                    {(endpoint, ix) => (
                                        <div
                                            class="table-row border-b border-b-base-300 [&_span]:text-xs [&_span]:lg:text-sm [&_div]:size-3 [&_div]:p-0.5 [&_div]:lg:p-1 [&_div]:align-middle"
                                            classList={{
                                                "bg-base-200": ix % 2 === 1,
                                            }}
                                            onClick={() =>
                                                navigate(
                                                    `/endpoints/${endpoint().endpoint.id}`,
                                                    {
                                                        replace: false,
                                                        scroll: false,
                                                    },
                                                )
                                            }
                                        >
                                            <div class="table-cell font-light pl-8!">
                                                {ix}
                                            </div>
                                            <Index
                                                each={Object.values(
                                                    endpoint()
                                                        .endpoint as EndpointStruct,
                                                )}
                                            >
                                                {(field, ix) => (
                                                    <div class="table-cell">
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
                                                )}
                                            </Index>
                                            <div class="table-cell font-light">
                                                {endpoint().path ?? "—"}
                                            </div>
                                        </div>
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
