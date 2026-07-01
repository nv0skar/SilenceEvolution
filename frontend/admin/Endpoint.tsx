// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

export interface EndpointStruct {
    id: string;
    route: string;
    description: string | null;
    version: string | null;
    method: string;
    query_params: Array<string>;
    body_params: Array<string>;
    query: string | null;
    require_auth: boolean;
    allowed_roles: Array<string>;
    inject_user_id: boolean;
    auto_generated: boolean;
}
