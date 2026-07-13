// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "@admin/Admin.tsx";
import { fetcher, UsersContext } from "@admin/users";

import Modal from "@admin/components/Modal";

import {
    children,
    createEffect,
    createResource,
    Index,
    on,
    Show,
    useContext,
} from "solid-js";

import { A, useNavigate, type RouteSectionProps } from "@solidjs/router";

export default (props: RouteSectionProps) => {
    const session_context = useContext(SessionContext);

    if (!session_context) throw new Error("Can't find user's context");

    const navigate = useNavigate();

    // Load users.
    const [users, { refetch }] = createResource(fetcher);

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
                    <div class="flex gap-2 self-end text-right ml-auto items-center *:rounded-2xl">
                        <button
                            class="btn text-sm self-end text-right ml-auto"
                            onClick={refetch}
                        >
                            <span class="material-symbols-outlined">
                                refresh
                            </span>
                        </button>
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
                <Show
                    when={!users.loading}
                    fallback={
                        <div class="flex w-full my-8 justify-center">
                            <span class="loading loading-spinner loading-xl"></span>
                        </div>
                    }
                >
                    <Modal parent_path="/users">
                        <UsersContext.Provider
                            value={{
                                users: users()!,
                                refetch: refetch,
                            }}
                        >
                            {props.children}
                        </UsersContext.Provider>
                    </Modal>

                    <div class="overflow-x-auto transition-all transition-discrete duration-500 starting:opacity-0 starting:scale-95">
                        <div class="table text-sm table-auto border-collapse my-6">
                            <div class="table-header-group border-b-2 border-b-base-300">
                                <div class="table-row font-bold bg-base-300 [&_div]:p-4 [&_div]:align-middle [&_div]:text-left [&_div]:btn [&_div]:btn-ghost [&_div]:rounded-none">
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
