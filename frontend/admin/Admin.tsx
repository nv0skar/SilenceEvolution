// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import {
    createContext,
    createEffect,
    createResource,
    createSignal,
    on,
    Show,
    type Accessor,
    type Setter,
} from "solid-js";

import { A, type RouteSectionProps } from "@solidjs/router";

export const UserContext = createContext<{
    user: Accessor<User | undefined>;
    set_user: Setter<User | undefined>;
}>();

type User = {
    user_id: number;
    name: string;
    email: string;
    role: string | null;
};

const Logout = async () => {
    await fetch("/api/internal/logout");
    document.location = "/";
};

export default (props: RouteSectionProps) => {
    const [get_user, set_user] = createSignal<User | undefined>(undefined);

    // Load user data.
    const [user] = createResource(async (): Promise<User> => {
        const res = await fetch("/api/internal/whoami");

        if (!res.ok) console.clear();

        if (res.status === 200) {
            const data = (await res.json())[0];
            return data as User;
        } else {
            document.location = "/auth?redirect=/admin";
            throw new Error("User is not logged in.");
        }
    });

    createEffect(
        on(user, (user) => {
            if (user !== undefined) set_user(user!);
        }),
    );

    return (
        <>
            <div class="drawer lg:drawer-open">
                <input id="app-drawer" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content flex flex-col">
                    <nav>
                        <div class="navbar bg-base-100/50 backdrop-blur-xl shadow fixed left-0 z-10">
                            <div class="navbar-start">
                                <label
                                    for="app-drawer"
                                    data-tip="Open"
                                    class="btn btn-ghost lg:hidden"
                                >
                                    <span class="material-symbols-outlined">
                                        side_navigation
                                    </span>
                                </label>
                            </div>
                            <div class="navbar-center">
                                <A
                                    class="text-xl lg:text-2xl font-stretch-150% btn btn-ghost"
                                    href="/"
                                >
                                    Silence
                                </A>
                            </div>
                            <div class="navbar-end">
                                <Show when={get_user() !== undefined}>
                                    <button
                                        class="btn btn-ghost"
                                        popovertarget="user-dropdown"
                                        style="anchor-name:--user-anchor"
                                    >
                                        <div class="inline text-sm">
                                            <span class="text-sm">Hi, </span>
                                            <span class="dark:text-emerald-400 text-sm font-bold">
                                                {get_user()!.name}
                                            </span>
                                        </div>
                                    </button>
                                    <ul
                                        class="dropdown menu w-52 rounded-box bg-base-100/50 border-base-200 border backdrop-blur-2xl shadow-lg"
                                        popover
                                        id="user-dropdown"
                                        style="position-anchor:--user-anchor"
                                    >
                                        <li>
                                            <button onClick={Logout}>
                                                <span class="material-symbols-outlined text-xs">
                                                    logout
                                                </span>
                                                <span class="font-bold">
                                                    Logout
                                                </span>
                                            </button>
                                        </li>
                                    </ul>
                                </Show>
                            </div>
                        </div>
                    </nav>
                    <div class="absolute top-0 left-0 p-8 px-2 lg:pl-26 pt-24 w-screen h-screen z-0">
                        <Show when={get_user() !== undefined}>
                            <Show
                                when={get_user()!.role === "admin"}
                                fallback={
                                    <div class="flex justify-center items-center top-0 text-2xl text-center font-bold leading-relaxed">
                                        <div>
                                            <span>
                                                Hmmm... It seems you shouldn't
                                                be here {get_user()!.name}.{" "}
                                            </span>
                                            <br />
                                            <span class="text-error">
                                                This account doesn't have the
                                                required role for managing this{" "}
                                                <span class="font-black">
                                                    Silence
                                                </span>{" "}
                                                project.
                                            </span>
                                            <br />
                                            <span>
                                                HINT: access the database and
                                                add the{" "}
                                                <code class="bg-base-300 p-2 rounded-sm">
                                                    admin
                                                </code>{" "}
                                                role to the user id{" "}
                                                <code class="bg-base-300 p-2 rounded-sm">
                                                    {get_user()!.user_id}
                                                </code>
                                            </span>
                                        </div>
                                    </div>
                                }
                            >
                                <UserContext.Provider
                                    value={{
                                        user: get_user,
                                        set_user,
                                    }}
                                >
                                    {props.children}
                                </UserContext.Provider>
                            </Show>
                        </Show>
                    </div>
                </div>
                <div class="drawer-side is-drawer-close:overflow-visible">
                    <label
                        for="app-drawer"
                        aria-label="close sidebar"
                        class="drawer-overlay"
                    ></label>
                    <div class="flex h-full flex-col items-start fixed is-drawer-close:w-16 is-drawer-open:w-40">
                        <ul class="menu h-full bg-base-200 border-base-300 backdrop-blur shadow-xl rounded-box border m-2 p-2 gap-2 [&_li]:transition [&_li]:duration-200 [&_li]:hover:scale-105">
                            <li>
                                <A href="/endpoints">
                                    <span class="material-symbols-outlined scale-95">
                                        route
                                    </span>
                                    <span class="is-drawer-close:hidden">
                                        Endpoints
                                    </span>
                                </A>
                            </li>
                            <li class="bottom-0">
                                <A href="/users">
                                    <span class="material-symbols-outlined scale-95">
                                        group
                                    </span>
                                    <span class="is-drawer-close:hidden">
                                        Users
                                    </span>
                                </A>
                            </li>
                            <li>
                                <A href="/settings">
                                    <span class="material-symbols-outlined scale-95">
                                        settings
                                    </span>
                                    <span class="is-drawer-close:hidden">
                                        Settings
                                    </span>
                                </A>
                            </li>
                            <li class="absolute bottom-0 my-2">
                                <label for="app-drawer">
                                    <span class="material-symbols-outlined scale-95">
                                        side_navigation
                                    </span>
                                </label>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </>
    );
};
