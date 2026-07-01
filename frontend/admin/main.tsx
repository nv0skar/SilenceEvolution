// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import Admin from "./Admin.tsx";

import Endpoints from "./Endpoints.tsx";
import ManageEndpoint from "./ManageEndpoint.tsx";
import TestEndpoint from "./TestEndpoint.tsx";

import Users from "./Users.tsx";
import ManageUser from "./ManageUser.tsx";

import Config from "./Config.tsx";

import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

const root = document.getElementById("root");

render(
    () => (
        <Router root={Admin} base="/admin">
            <Route path="/*" component={Endpoints} />
            <Route path="/endpoints" component={Endpoints}>
                <Route path="/*" />
                <Route path="/:id">
                    <Route path="/modify" component={ManageEndpoint} />
                    <Route path="/test" component={TestEndpoint} />
                </Route>
                <Route path="/new" component={ManageEndpoint} />
            </Route>
            <Route path="/users" component={Users}>
                <Route path="/*" />
                <Route path="/:id" component={ManageUser} />
                <Route path="/new" component={ManageUser} />
            </Route>
            <Route path="/config" component={Config} />
        </Router>
    ),
    root!,
);
