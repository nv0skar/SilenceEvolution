// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import Auth from "./Auth.tsx";

import Login from "./Login.tsx";
import Signup from "./Signup.tsx";

import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

const root = document.getElementById("root");

render(
    () => (
        <Router root={Auth} base="/auth">
            <Route path="/*" component={Login} />
            <Route path="/signup" component={Signup} />
        </Router>
    ),
    root!,
);
