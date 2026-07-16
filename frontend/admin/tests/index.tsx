// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { type ResourceReturn } from "solid-js";

export type TestByFile = {
    path: string | null;
    test: Test;
};

export interface Test {
    name?: string;

    target_endpoint_id?: string;

    description?: string;

    [k: string]: any;
}

export type TestsContext = ResourceReturn<Array<TestByFile>, any>;

export const fetcher = async (): Promise<Array<TestByFile>> => {
    const res = await fetch("/api/internal/admin/tests");

    if (!res.ok) console.clear();

    if (res.status === 200) {
        const data = (await res.json()) as Array<Array<string | Array<Test>>>;

        let tests: Array<TestByFile> = new Array();

        for (const tests_path of data) {
            for (let test of tests_path[1] as Array<Test>) {
                tests.push({
                    path: tests_path[0] as string | null,
                    test,
                });
            }
        }

        return tests;
    } else {
        return new Array();
    }
};
