// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import Admin from "@admin/Admin.tsx";

import ListEndpoint from "@admin/endpoints/List.tsx";
import ModifyEndpoint from "@admin/endpoints/Modify.tsx";
import TestEndpoint from "@admin/endpoints/Test.tsx";

import ListUsers from "@admin/users/List.tsx";
import ModifyUser from "@admin/users/Modify.tsx";

import Config from "./Config.tsx";

import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

const root = document.getElementById("root");

render(
    () => (
        <Router root={Admin} base="/admin">
            <Route path="/*" component={ListEndpoint} />
            <Route path="/endpoints" component={ListEndpoint}>
                <Route path="/*" />
                <Route path="/:id">
                    <Route path="/modify" component={ModifyEndpoint} />
                    <Route path="/test" component={TestEndpoint} />
                </Route>
                <Route path="/new" component={ModifyEndpoint} />
            </Route>
            <Route path="/users" component={ListUsers}>
                <Route path="/*" />
                <Route path="/:id" component={ModifyUser} />
                <Route path="/new" component={ModifyUser} />
            </Route>
            <Route path="/config" component={Config} />
        </Router>
    ),
    root!,
);
