// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { UserContext } from "./Admin";

import {
    createContext,
    createEffect,
    createResource,
    createSignal,
    on,
    Show,
    useContext,
    type Accessor,
    type Setter,
} from "solid-js";

import { A, type RouteSectionProps } from "@solidjs/router";

export default (_: RouteSectionProps) => {
    const user_context = useContext(UserContext);

    if (!user_context) throw new Error("Can't find user's context");

    return (
        <>
            <h1>Settings</h1>
        </>
    );
};
