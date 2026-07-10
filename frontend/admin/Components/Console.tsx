// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { createSignal, Show } from "solid-js";

import { Portal } from "solid-js/web";

import Convert from "ansi-to-html";

export default () => {
    let console_stream: HTMLDivElement | undefined = undefined;
    let root_pseudocontainer: HTMLDivElement | undefined = undefined;

    const [error, set_error] = createSignal<boolean>();

    const socket = new WebSocket("/", "ConsoleStream");

    socket.onmessage = async (msg: MessageEvent<Blob>) => {
        let paragraph = document.createElement("p");

        const convert = new Convert();

        paragraph.innerHTML = convert.toHtml(
            new TextDecoder().decode(await msg.data.bytes()),
        );

        console_stream!.append(paragraph);

        console_stream!.lastElementChild!.scrollIntoView({
            behavior: "smooth",
            block: "end",
        });
    };

    socket.onerror = (_) => {
        set_error(true);
    };

    return (
        <>
            <Portal>
                <div class="overscroll-none antialiased fixed flex flex-col w-screen h-screen inset-0 z-50 pointer-events-none">
                    <div class="w-full flex justify-end">
                        <div
                            class="relative w-2 h-screen pointer-coarse:h-[90vh] min-h-[20vh] max-h-screen pointer-coarse:max-h-[90vh] scale-x-[500] opacity-0 resize-y overflow-hidden origin-bottom-right pointer-events-auto cursor-ns-resize [clip-path:inset(calc(100%-24px)_0_0_0)]"
                            ref={root_pseudocontainer}
                        ></div>
                    </div>

                    <div class="flex flex-col min-h-0 h-full w-full bg-white/20 dark:bg-black/20 opacity-90 hover:bg-white/50 pointer-coarse:bg-white/50 dark:hover:bg-black/50 dark:pointer-coarse:bg-black/50 hover:opacity-100 border-t-2 border-base-300 backdrop-blur-sm hover:backdrop-blur-xl backdrop-brightness-90 rounded-t-4xl transition duration-200 pointer-events-auto">
                        <div class="absolute top-1 left-1/2 -translate-x-1/2 -translate-y-4 w-32 h-1.25 bg-zinc-500 rounded-full pointer-coarse:hidden pointer-events-none"></div>

                        <div class="flex flex-col gap-2 h-full px-6 pt-4 text-sm font-mono">
                            <div class="flex items-center">
                                <div class="flex gap-3 self-start text-left mr-auto items-center">
                                    <div class="inline-grid *:[grid-area:1/1]">
                                        <div
                                            class="status status-success animate-ping"
                                            classList={{
                                                "status-error!": error(),
                                            }}
                                        ></div>
                                        <div
                                            class="status status-success"
                                            classList={{
                                                "status-error!": error(),
                                            }}
                                        ></div>
                                    </div>
                                    <span class="font-black tracking-wider">
                                        CONSOLE
                                    </span>
                                </div>
                                <div class="flex gap-3 self-end text-right ml-auto items-center">
                                    <button
                                        class="hover:scale-105 transition duration-200"
                                        onClick={() => {
                                            console_stream!.innerHTML = "";
                                        }}
                                    >
                                        CLEAR
                                    </button>
                                    <button
                                        class="align-middle hover:scale-105 transition duration-200"
                                        onClick={() => {
                                            root_pseudocontainer!.style.height =
                                                "100vh";
                                        }}
                                    >
                                        <span class="material-symbols-outlined">
                                            bottom_panel_close
                                        </span>
                                    </button>
                                    <button
                                        class="align-middle hover:scale-105 transition duration-200"
                                        onClick={() => {
                                            root_pseudocontainer!.style.height =
                                                "0vh";
                                        }}
                                    >
                                        <span class="material-symbols-outlined">
                                            expand_content
                                        </span>
                                    </button>
                                </div>
                            </div>

                            <div class="border-t border-t-base-300"></div>

                            <div class="overflow-y-auto mb-2 text-sm/5 text-black dark:text-white **:whitespace-pre-wrap **:wrap-break-word">
                                <Show when={error()}>
                                    <p class="text-error font-bold">
                                        Couldn't connect to socket.
                                    </p>
                                </Show>
                                <div ref={console_stream}></div>
                            </div>
                        </div>
                    </div>
                </div>
            </Portal>
        </>
    );
};
