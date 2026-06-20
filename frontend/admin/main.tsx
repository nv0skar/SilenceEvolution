// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import Admin from "./Admin.tsx";

import { render } from "solid-js/web";
import { Router } from "@solidjs/router";

const root = document.getElementById("root");

render(() => <Router root={Admin} base="/admin" />, root!);
