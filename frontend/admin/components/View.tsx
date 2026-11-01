// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { type ParentProps, onMount } from "solid-js";

import { A, useLocation, useNavigate, useParams } from "@solidjs/router";

export default (props: { parent_path: string } & ParentProps) => {
    const navigate = useNavigate();

    const location = useLocation();

    const params = useParams();

    onMount(() => {
        scrollTo(0, 0);
    });

    return (
        <>
            <div class="overscroll-none antialiased w-max h-max transition duration-300 [&_input]:rounded-lg [&_input]:bg-base-200/75 [&_input]:backdrop-brightness-125 [&_input]:backdrop-blur-xs [&_textarea]:rounded-lg">
                <A
                    class="btn btn-circle bg-base-300/50 border-[0.5px] border-base-200 backdrop-blur-xs shadow-xs scale-90 hover:bg-base-200"
                    href={props.parent_path}
                    noScroll
                    replace={false}
                >
                    <span class="material-symbols-outlined">arrow_back</span>
                </A>

                <button
                    class="btn btn-circle tooltip tooltip-right bg-base-300/50 border-[0.5px] border-base-200 backdrop-blur-xs shadow-xs scale-90 hover:bg-base-200"
                    classList={{
                        hidden:
                            params["id"] === undefined ||
                            !props.parent_path.includes("endpoints"),
                    }}
                    onClick={() => {
                        navigate(
                            location.pathname.includes("modify")
                                ? `/endpoints/${params["id"]!}/test`
                                : `/endpoints/${params["id"]!}/modify`,
                            {
                                replace: false,
                                scroll: false,
                            },
                        );
                    }}
                    data-tip={
                        location.pathname.includes("modify")
                            ? "Test endpoint"
                            : "Modify endpoint"
                    }
                >
                    <span class="material-symbols-outlined">
                        {location.pathname.includes("modify")
                            ? "science"
                            : "data_object"}
                    </span>
                </button>
            </div>

            <div class="overflow-y-scroll min-h-screen [&>div]:transition-all [&>div]:transition-discrete [&>div]:duration-300 [&>div]:starting:opacity-0 [&>div]:starting:scale-90">
                {props.children}
            </div>
        </>
    );
};
