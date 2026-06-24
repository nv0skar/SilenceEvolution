// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { SessionContext } from "./Admin";

import { useContext } from "solid-js";

import { type RouteSectionProps } from "@solidjs/router";

export default (_: RouteSectionProps) => {
    const user_context = useContext(SessionContext);

    if (!user_context) throw new Error("Can't find user's context");

    return (
        <>
            <h1>Settings</h1>
        </>
    );
};
