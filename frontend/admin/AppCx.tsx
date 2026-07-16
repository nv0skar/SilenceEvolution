// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import * as session from "@admin/Session";
import * as endpoints from "@admin/endpoints";
import * as tests from "@admin/tests";
import * as users from "@admin/users";
import * as config from "@admin/Config";

import {
    createContext,
    createResource,
    getOwner,
    runWithOwner,
    type ParentProps,
    type ResourceReturn,
} from "solid-js";

export const RESOURCES_FETCHERS: Record<
    keyof ResourceTypes,
    () => Promise<any>
> = {
    session: session.fetcher,
    endpoints: endpoints.fetcher,
    tests: tests.fetcher,
    users: users.fetcher,
    config: config.fetcher,
};

export interface ResourceTypes {
    session: session.Session;
    endpoints: Array<endpoints.EndpointByFile>;
    tests: Array<tests.TestByFile>;
    users: Array<users.User>;
    config: config.ConfigStruct;
}

const AppCx = createContext<{
    get_resource: <K extends keyof ResourceTypes>(
        resource: K,
    ) => ResourceReturn<ResourceTypes[K]>;
}>();

export const AppCxProvider = (props: ParentProps) => {
    const owner = getOwner();

    const cache: Record<string, ResourceReturn<any>> = {};

    const get_or_init = (
        key: keyof typeof RESOURCES_FETCHERS,
        fetcher: () => Promise<any>,
    ): ResourceReturn<any> => {
        if (cache[key] === undefined) {
            const resource = runWithOwner(owner, () => createResource(fetcher));

            cache[key] = resource!;
        }

        return cache[key];
    };

    const get_resource = (resource: keyof typeof RESOURCES_FETCHERS) => {
        return get_or_init(resource, RESOURCES_FETCHERS[resource]!);
    };

    return (
        <>
            <AppCx.Provider value={{ get_resource }}>
                {props.children}
            </AppCx.Provider>
        </>
    );
};

export default AppCx;
