// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import {
    createContext,
    createEffect,
    createResource,
    createSignal,
    on,
    Show,
    type Setter,
} from "solid-js";

import { A, useSearchParams, type RouteSectionProps } from "@solidjs/router";
export const Context = createContext<Context>();

type Context = {
    setTitle: Setter<string>;
    setError: Setter<string | undefined>;
    redirect: string;
};

export default (props: RouteSectionProps) => {
    // Check whether a session already exists.
    const [session_exists] = createResource(async () => {
        const res = await fetch("/api/internal/whoami");

        return res.status === 200;
    });

    createEffect(
        on(session_exists, (session_exists) => {
            if (session_exists) document.location = "/";
        }),
    );

    const [title, setTitle] = createSignal<string>("Login");

    const [error, setError] = createSignal<string | undefined>(undefined);

    const [params, _] = useSearchParams();

    const redirect = (params["redirect"] ?? "/") as string;

    return (
        <>
            <nav>
                <div class="navbar bg-base-100/50 backdrop-blur-sm shadow fixed">
                    <div class="navbar-start"></div>
                    <div class="navbar-center">
                        <A
                            class="text-2xl font-stretch-150% btn btn-ghost"
                            href="/"
                        >
                            Silence
                        </A>
                    </div>
                    <div class="navbar-end"></div>
                </div>
            </nav>
            <div class="flex h-screen justify-center items-center">
                <div>
                    <Context.Provider value={{ setTitle, setError, redirect }}>
                        <h1 class="my-6 text-4xl font-extrabold text-center">
                            {title()}
                        </h1>
                        <div class="bg-base-200/50 border-base-300 backdrop-blur shadow-xl rounded-box w-xs border p-4">
                            <Show when={error() != undefined}>
                                <div
                                    class="bg-red-800 border-red-400 backdrop-blur shadow-xl rounded-box my-2 p-2 text-center cursor-pointer"
                                    onClick={() => setError(undefined)}
                                >
                                    <p class="text-sm font-semibold">
                                        An error has occurred.&nbsp;
                                        {error()!}
                                    </p>
                                </div>
                            </Show>
                            {props.children}
                        </div>
                    </Context.Provider>
                </div>
            </div>
        </>
    );
};
