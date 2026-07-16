// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { type ResourceReturn } from "solid-js";

import { pipe, map } from "remeda";

export interface User {
    user_id?: number;
    name: string;
    email: string;
    role?: string;
    password: string;
}

export type UsersContext = ResourceReturn<Array<User>, any>;

export const fetcher = async (): Promise<Array<User>> => {
    const res = await fetch("/api/internal/admin/users");

    if (!res.ok) console.clear();

    if (res.status === 200) {
        let data = (await res.json()) as Array<User>;

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
};
