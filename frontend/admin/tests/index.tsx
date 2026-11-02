// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { type Session } from "@admin/Session";

import { type ResourceReturn } from "solid-js";

export type TestByFile = {
    path: string | null;
    test: Test;
};

export interface Test {
    name?: string;

    target_endpoint_id?: string;

    description?: string;

    [k: string]: any;
}

export type TestsContext = ResourceReturn<Array<TestByFile>, any>;

export const fetcher = async (): Promise<Array<TestByFile>> => {
    const res = await fetch("/api/internal/admin/tests");

    if (res.status === 200) {
        const data = (await res.json()) as Array<Array<string | Array<Test>>>;

        let tests: Array<TestByFile> = new Array();

        for (const tests_path of data) {
            for (let test of tests_path[1] as Array<Test>) {
                tests.push({
                    path: tests_path[0] as string | null,
                    test,
                });
            }
        }

        return tests;
    } else {
        return new Array();
    }
};

export const tests_session_fetcher = async (
    _?: boolean,
    info?: {
        refetching?: {
            user_id?: number;
            token?: string;
        };
    },
) => {
    const user_id = info?.refetching?.user_id;

    let token = info?.refetching?.token;

    if (user_id !== undefined) localStorage.removeItem("test_authorization"); // Clear the local storage, as the token is not valid.

    if (token === undefined)
        token = localStorage.getItem("test_authorization") ?? undefined;

    if (!user_id && !token) return null;

    if (user_id !== undefined && token === undefined) {
        const res = await fetch(
            `/api/internal/admin/users/session/issue/${user_id!}`,
            {
                method: "get",
            },
        );

        if (res.status === 200) {
            const body = (await res.json()) as {
                token: string;
            };

            token = body.token;
            localStorage.setItem("test_authorization", body.token);
        } else {
            return null;
        }
    }

    console.log(user_id, token);

    const whoami = await fetch("/api/internal/whoami", {
        headers: {
            Authorization: token,
        },
    });

    if (whoami.status === 200) {
        const data = await whoami.json();

        return data as Session;
    } else {
        return null;
    }
};
