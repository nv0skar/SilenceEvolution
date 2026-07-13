// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { createContext } from "solid-js";

export type EndpointByFile = {
    path: string | null;
    endpoint: Endpoint;
};

export interface Endpoint {
    id?: string;
    database?: string;
    route: string;
    version?: string;
    method: string;
    execute?: {
        query?: string;
        queries?: Array<{
            query: string;
            include?: boolean;
            behaviour?: string;
        }>;
    };
    query_params: Array<string>;
    body_params: Array<string>;
    description?: string;
    require_auth: boolean;
    inject_auth_metadata: boolean;
    allowed_roles: Array<string>;
    auto_generated: boolean;
}

export const EndpointsContext = createContext<{
    endpoints_data: Array<{
        path: string | null;
        endpoint: Endpoint;
    }>;
    refetch_endpoints: Function;
}>();
