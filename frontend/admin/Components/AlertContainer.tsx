// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { Show, type Accessor, type Setter } from "solid-js";

export interface AlertStruct {
    value: string;
    is_error?: boolean;
}

export default (props: {
    get_alert: Accessor<AlertStruct | undefined>;
    set_alert: Setter<AlertStruct | undefined>;
}) => {
    return (
        <>
            <div
                class="flex flex-col bg-base-300/50 min-h-12 border border-base-300 text-info text-center justify-center items-center backdrop-blur-xs shadow-xl rounded-box my-2 p-2 cursor-pointer transition transition-discrete duration-500"
                classList={{
                    "hidden opacity-0 pointer-events-none":
                        props.get_alert() === undefined,
                    "bg-red-800 border-red-500 text-white":
                        props.get_alert()?.is_error,
                }}
                onClick={() => props.set_alert(undefined)}
            >
                <Show when={props.get_alert()?.is_error}>
                    <span class="text-sm font-semibold">
                        An error has occurred.{" "}
                    </span>
                </Show>
                <span class="text-sm font-semibold">
                    {props.get_alert()?.value}
                </span>
            </div>
        </>
    );
};
