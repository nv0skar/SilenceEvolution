// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

export interface ConfigStruct {
    listening_addr: string;
    serve_static_files: boolean;
    internal_params: {
        user_id: string;
        users_target_table: string;
        sessions_target_table: string;
        roles_target_table: string;
    };
    skip_endpoints_ids: Array<string>;
    database_conn: {
        host: string;
        username: string;
        password: string;
        db: string;
    };
}
