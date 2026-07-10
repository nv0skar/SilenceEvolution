// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "./Admin.tsx";
import { type UserStruct } from "./User.tsx";

import {
    children,
    createContext,
    createEffect,
    createResource,
    Index,
    on,
    Show,
    useContext,
} from "solid-js";

import { A, useNavigate, type RouteSectionProps } from "@solidjs/router";

import { pipe, map } from "remeda";

export const UsersContext = createContext<{
    users_data: Array<UserStruct>;
    refetch_users: Function;
}>();

export default (props: RouteSectionProps) => {
    const session_context = useContext(SessionContext);

    if (!session_context) throw new Error("Can't find user's context");

    const navigate = useNavigate();

    // Load users.
    const [users, { refetch }] = createResource(
        async (): Promise<Array<UserStruct>> => {
            const res = await fetch("/api/internal/admin/users");

            if (!res.ok) console.clear();

            if (res.status === 200) {
                let data = (await res.json()) as Array<UserStruct>;

                data = pipe(
                    data,
                    map((user) => {
                        if (user.role?.length === 0) delete user.role;
                        return user;
                    }),
                );

                return data;
            } else {
                return [];
            }
        },
    );

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
                    <h1 class="text-4xl font-bold">Users</h1>
                    <div class="flex self-end text-right ml-auto items-center">
                        <A
                            class="btn text-sm self-end text-right ml-auto"
                            href="/users/new"
                        >
                            <span class="material-symbols-outlined lg:hidden!">
                                add
                            </span>
                            <span class="not-lg:hidden">Create new user</span>
                        </A>
                    </div>
                </div>
                <Show when={!users.loading}>
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
                                navigate("/users", {
                                    replace: false,
                                    scroll: false,
                                })
                            }
                        >
                            <div
                                id="modal"
                                class="relative lg:m-32 w-full h-fit px-4 bg-base-100/75 backdrop-blur-xs border border-base-300 rounded-2xl  shadow-lg transition duration-300"
                                classList={{
                                    "opacity-0 scale-75":
                                        resolved_children() === undefined,
                                }}
                                onClick={(event) => {
                                    event.stopPropagation();
                                }}
                            >
                                <A
                                    class="absolute top-0 right-0 m-2 btn btn-circle bg-base-300/50 border-[0.5px] border-base-200 backdrop-blur-xs shadow-xs scale-90 hover:bg-base-200 z-50"
                                    href="/users"
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
                                        <UsersContext.Provider
                                            value={{
                                                users_data: users()!,
                                                refetch_users: refetch,
                                            }}
                                        >
                                            {props.children}
                                        </UsersContext.Provider>
                                    </Show>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="overflow-x-auto">
                        <div class="table text-sm table-auto border-collapse my-6">
                            <div class="table-header-group border-b-2 border-b-base-300">
                                <div class="table-row font-bold bg-base-200 [&_div]:p-4 [&_div]:align-middle [&_div]:text-left [&_div]:btn [&_div]:btn-ghost [&_div]:rounded-none">
                                    <div class="table-cell rounded-tl-xl! pl-8!">
                                        ID
                                    </div>
                                    <div class="table-cell">Name</div>
                                    <div class="table-cell">Email</div>
                                    <div class="table-cell">Role</div>
                                    <div class="table-cell rounded-tr-xl!">
                                        Password
                                    </div>
                                </div>
                            </div>
                            <div class="table-row-group [&>div]:even:bg-base-200 [&>div]:hover:bg-base-300 [&>div]:transition [&>div]:duration-200 [&>div]:hover:scale-101 [&>div]:hover:cursor-pointer">
                                <Index
                                    each={users()?.sort((user_1, user_2) => {
                                        return user_1.name.localeCompare(
                                            user_2.name,
                                        );
                                    })}
                                >
                                    {(user, _) => (
                                        <div
                                            class="table-row border-b border-b-base-300 [&_span]:text-xs [&_span]:lg:text-sm [&_div]:size-auto [&_div]:p-2 [&_div]:align-middle"
                                            onClick={() =>
                                                navigate(
                                                    `/users/${user().user_id}`,
                                                    {
                                                        replace: false,
                                                        scroll: false,
                                                    },
                                                )
                                            }
                                        >
                                            <div class="table-cell font-light pl-8!">
                                                {user().user_id}
                                            </div>
                                            <div class="table-cell">
                                                {user().name}
                                            </div>
                                            <div class="table-cell">
                                                {user().email}
                                            </div>
                                            <div class="table-cell">
                                                {user().role ?? "—"}
                                            </div>
                                            <div class="table-cell [&_span]:hidden after:content-['******'] after:text-base-content hover:[&_span]:block hover:after:hidden pr-4!">
                                                <span>{user().password}</span>
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
