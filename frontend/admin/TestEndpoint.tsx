// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { EndpointsContext } from "./Endpoints";

import {
    createEffect,
    createResource,
    createSignal,
    Index,
    Show,
    useContext,
} from "solid-js";

import { createStore } from "solid-js/store";

import { useParams, type RouteSectionProps } from "@solidjs/router";

import { pipe, map, fromEntries, filter, join } from "remeda";

import { loadGrammar } from "@arborium/arborium";

export interface EndpointSchema {
    base_route: string;
    route_params: Array<string>;
    query_params: Array<string>;
    body_params: Array<string>;
}

export default (_: RouteSectionProps) => {
    const endpoints_context = useContext(EndpointsContext);

    if (!endpoints_context) return <span></span>; // This component may be rendered twice, one of the renders will happen with endpoints' context undefined.

    const [error, set_error] = createSignal<string | undefined>(undefined);

    const [response, set_response] = createSignal<
        | {
              time: string;
              body: string;
          }
        | undefined
    >(undefined);

    const [authorization, set_authorization] = createSignal<string | undefined>(
        localStorage.getItem("test_authorization") ?? undefined,
    );

    const [req_body, set_req_body] = createSignal<string | undefined>(
        undefined,
    );

    const id = useParams()["id"];

    let route_params_element: HTMLFieldSetElement | undefined = undefined;
    let query_params_element: HTMLFieldSetElement | undefined = undefined;
    let req_body_highlighted_element: HTMLDivElement | undefined = undefined;
    let req_body_element: HTMLTextAreaElement | undefined = undefined;
    let response_body_element: HTMLDivElement | undefined = undefined;

    const endpoint_data =
        id !== undefined
            ? endpoints_context.endpoints_data.filter((endpoint_pair) => {
                  return endpoint_pair.endpoint.id == id;
              })[0]!
            : undefined;

    const [endpoint_schema, set_endpoint_schema] = createStore<{
        state: EndpointSchema | undefined;
    }>({
        state:
            endpoint_data !== undefined
                ? ({
                      base_route: `/${pipe(
                          `/api/${endpoint_data!.endpoint.version}/${endpoint_data!.endpoint.route}`.split(
                              "/",
                          ),
                          filter((str) => str.length !== 0),
                          join("/"),
                      )}`,
                      route_params: pipe(
                          endpoint_data!.endpoint.route
                              .matchAll(/\{(\w+)\}/g)
                              .toArray(),
                          map((match) => match[1]),
                      ),
                      query_params:
                          endpoint_data!.endpoint.query_params ?? new Array(),
                      body_params:
                          endpoint_data!.endpoint.body_params ?? new Array(),
                  } as EndpointSchema)
                : undefined,
    });

    const send_req = async () => {
        // Extract all route params.
        if (route_params_element === undefined) return;
        const route_params_children = (
            route_params_element! as HTMLFieldSetElement
        ).children;

        const route_params = pipe(
            Array.from(route_params_children),
            map((element) => {
                const input = element.querySelector("input")!;
                return {
                    name: input.name,
                    value: input.value,
                };
            }),
        );

        // Extract all query params.
        if (query_params_element === undefined) return;
        const query_params_children = (
            query_params_element! as HTMLFieldSetElement
        ).children;

        const query_params = pipe(
            Array.from(query_params_children),
            map((element) => {
                const field_name = element.getElementsByClassName(
                    "field_name",
                )[0]! as HTMLInputElement;
                const field_value = element.getElementsByClassName(
                    "field_value",
                )[0]! as HTMLInputElement;

                return {
                    name: field_name.value,
                    value: field_value.value,
                };
            }),
        );

        // Build the route.
        let base_route = endpoint_schema.state!.base_route;

        for (const route_param of route_params) {
            base_route = base_route.replace(
                `{${route_param.name}}`,
                route_param.value,
            );
        }

        if (query_params.length !== 0) {
            base_route = base_route.concat("?");

            base_route = base_route.concat(
                pipe(
                    query_params,
                    map(({ name, value }) => {
                        return `${name}=${value}`;
                    }),
                    join("&"),
                ),
            );
        }

        const res = await fetch(base_route, {
            method: endpoint_data!.endpoint.method,
            headers: {
                Authorization: localStorage.getItem("test_authorization") ?? "",
            },
            body:
                endpoint_data!.endpoint.method.toLowerCase() === "get"
                    ? null
                    : req_body_element!.value,
            credentials: "omit",
        });

        const body = await res.json();

        if ("token" in body) {
            localStorage.setItem("test_authorization", body["token"]);
            set_authorization(body["token"]);
        }

        if (!res.ok) {
            if (res.status === 401) {
                localStorage.removeItem("test_authorization");
                set_authorization(undefined);
            }

            set_error(body["error"]);
        }

        set_response({
            time: res.headers.get("Date")!,
            body: JSON.stringify(body),
        });
    };

    // Load JSON grammar.
    const [json_grammar] = createResource(async () => {
        return (await loadGrammar("json"))!;
    });

    const [default_body_skeleton] = createResource(async () => {
        if (endpoint_schema.state !== undefined) {
            let skeleton = pipe(
                endpoint_schema.state!.body_params,
                map((param) => [param, null] as const),
                fromEntries,
            );

            return JSON.stringify(skeleton).replaceAll('"', "");
        } else {
            return "{}";
        }
    });

    createEffect(async () => {
        if (endpoint_data !== undefined) {
            req_body_highlighted_element!.innerHTML =
                await json_grammar()?.highlight(
                    req_body() ?? default_body_skeleton()!,
                )!;
        }
    });

    createEffect(async () => {
        if (response() !== undefined) {
            response_body_element!.innerHTML = await json_grammar()?.highlight(
                response()!.body,
            )!;
        }
    });

    return (
        <>
            <div class="grid gap-2 py-8">
                <div>
                    <h1 class="text-3xl text-center pt-4 font-bold">
                        {endpoint_data !== undefined
                            ? `🧪 ${endpoint_data.endpoint.id}`
                            : "🧪 Test endpoint"}
                    </h1>
                </div>
                <Show
                    when={endpoint_data !== undefined}
                    fallback={<span>The endpoint {id} doesn't exist.</span>}
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
                        <fieldset class="fieldset flex gap-1">
                            <input
                                name="method"
                                type="text"
                                class="input p-0 inline-20 text-center font-bold w-min"
                                value={endpoint_data!.endpoint.method}
                                readOnly
                            />
                            <input
                                name="route"
                                type="text"
                                class="input w-full"
                                value={endpoint_schema.state!.base_route}
                                readOnly
                            />
                        </fieldset>

                        <details
                            class="collapse bg-base-100/20 border border-base-300 rounded-2xl mt-3 my-1"
                            classList={{
                                hidden:
                                    endpoint_schema.state!.route_params
                                        .length === 0,
                            }}
                            open
                        >
                            <summary class="collapse-title font-semibold transition duration-200 hover:bg-base-200">
                                Route params
                            </summary>
                            <div class="collapse-content">
                                <fieldset
                                    class="fieldset [&_.input]:w-full lg:grid-cols-3 gap-3"
                                    ref={route_params_element}
                                >
                                    <Index
                                        each={
                                            endpoint_schema.state!.route_params
                                        }
                                    >
                                        {(param) => {
                                            return (
                                                <>
                                                    <label>
                                                        <span class="label">
                                                            {param()}
                                                        </span>
                                                        <input
                                                            name={param()}
                                                            type="text"
                                                            class="input"
                                                            placeholder="—"
                                                            required
                                                        />
                                                    </label>
                                                </>
                                            );
                                        }}
                                    </Index>
                                </fieldset>
                            </div>
                        </details>

                        <details
                            class="collapse bg-base-100/20 border border-base-300 rounded-2xl mt-3 my-1"
                            open
                        >
                            <summary class="collapse-title font-semibold transition duration-200 hover:bg-base-200">
                                Query params
                            </summary>
                            <div class="collapse-content">
                                <fieldset
                                    id="query_params"
                                    class="fieldset [&_.input]:w-full lg:grid-cols-3 gap-3 empty:hidden"
                                    ref={query_params_element}
                                >
                                    <Index
                                        each={
                                            endpoint_schema.state!.query_params
                                        }
                                    >
                                        {(param, _) => {
                                            return (
                                                <>
                                                    <label>
                                                        <div class="flex">
                                                            <input
                                                                type="text"
                                                                class="field_name label outline-0"
                                                                placeholder="Query param name"
                                                                value={param()}
                                                            />
                                                            <div
                                                                class="self-end text-right ml-auto opacity-75 scale-75 transition duration-200 hover:opacity-50"
                                                                onClick={(
                                                                    event,
                                                                ) => {
                                                                    const parent =
                                                                        event
                                                                            .currentTarget
                                                                            .parentElement!
                                                                            .parentElement;

                                                                    parent!.remove();
                                                                }}
                                                            >
                                                                <span class="material-symbols-outlined">
                                                                    delete
                                                                </span>
                                                            </div>
                                                        </div>
                                                        <input
                                                            type="text"
                                                            class="field_value input"
                                                            placeholder="—"
                                                        />
                                                    </label>
                                                </>
                                            );
                                        }}
                                    </Index>
                                </fieldset>
                                <div
                                    class="btn text-sm mt-2 text-right"
                                    onClick={(_) => {
                                        let query_params = Array.from(
                                            endpoint_schema.state!.query_params,
                                        );

                                        query_params.push("");

                                        set_endpoint_schema(
                                            "state",
                                            "query_params",
                                            query_params,
                                        );
                                    }}
                                >
                                    <span class="material-symbols-outlined lg:hidden!">
                                        add
                                    </span>
                                    <span class="not-lg:hidden">
                                        Add query param
                                    </span>
                                </div>
                            </div>
                        </details>

                        <label class="my-1">
                            <span class="label text-xs">Body</span>
                            <div class="overflow-auto overscroll-contain">
                                <div class="grid grid-cols-1 box-border min-w-0 font-mono **:text-sm **:leading-6 overflow-hidden">
                                    <div
                                        class="bg-base-100/25 p-3 border border-base-300 col-start-1 row-start-1 rounded-2xl w-full h-full inset-0 pointer-events-none whitespace-pre-wrap wrap-break-word z-10 min-w-0 overflow-hidden"
                                        ref={req_body_highlighted_element}
                                    ></div>
                                    <textarea
                                        id="body"
                                        name="body"
                                        class="bg-transparent p-3 border text-transparent col-start-1 row-start-1 whitespace-pre-wrap w-full min-h-14 not-focus:text-transparent z-20 min-w-0 outline-0"
                                        spellcheck="false"
                                        onKeyDown={(event) => {
                                            let target = event.currentTarget;

                                            if (event.key === "Tab") {
                                                event.preventDefault();

                                                let value = target.value,
                                                    start =
                                                        target.selectionStart,
                                                    end = target.selectionEnd;
                                                target.value =
                                                    value.substring(0, start) +
                                                    "\t" +
                                                    value.substring(end);
                                                target.selectionStart =
                                                    target.selectionEnd =
                                                        start + 1;
                                            }
                                        }}
                                        value={default_body_skeleton()}
                                        placeholder="Body"
                                        onInput={async (event) => {
                                            set_req_body(
                                                event.currentTarget.value,
                                            );
                                        }}
                                        required={endpoint_data === undefined}
                                        ref={req_body_element}
                                    />
                                </div>
                            </div>
                        </label>
                    </form>
                    <div class="flex gap-1 my-2 text-xs font-bold">
                        <span>Authorization header: </span>

                        {authorization() !== undefined ? (
                            <span class="text-green-500">
                                {authorization()!}
                            </span>
                        ) : (
                            <span class="text-red-500">no</span>
                        )}
                    </div>
                    <div class="flex gap-2 [&_button]:rounded-xl [&_button]:hover:shadow">
                        <button
                            id="submit"
                            type="submit"
                            class="btn text-black btn-success"
                            classList={{
                                "btn-disabled":
                                    endpoint_schema.state!.route_params
                                        .length !== 0,
                            }}
                            onClick={send_req}
                        >
                            Send
                        </button>
                    </div>
                    <Show when={response() !== undefined}>
                        <div class="bg-base-100/25 border border-base-300 backdrop-blur shadow-xl rounded-box my-2 p-2">
                            <span class="text-xs">
                                Response at {response()!.time}
                            </span>
                            <p
                                class="text-sm font-semibold"
                                ref={response_body_element}
                            ></p>
                        </div>
                    </Show>
                </Show>
            </div>
        </>
    );
};
