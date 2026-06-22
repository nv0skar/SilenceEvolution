// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import Admin from "./Admin.tsx";

import Endpoints from "./Endpoints.tsx";
import Endpoint from "./Endpoint.tsx";

import Settings from "./Settings.tsx";
import Users from "./Users.tsx";

import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

const root = document.getElementById("root");

render(
    () => (
        <Router root={Admin} base="/admin">
            <Route path="/*" component={Endpoints} />
            <Route path="/endpoints" component={Endpoints}>
                <Route path="/*" />
                <Route path="/:id" component={Endpoint} />
                <Route path="/new" component={Endpoint} />
            </Route>
            <Route path="/users" component={Users} />
            <Route path="/settings" component={Settings} />
        </Router>
    ),
    root!,
);
