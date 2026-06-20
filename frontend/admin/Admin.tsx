// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

import { A, type RouteSectionProps } from "@solidjs/router";

export default (props: RouteSectionProps) => {
    return (
        <>
            <div class="drawer lg:drawer-open">
                <input id="app-drawer" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content flex flex-col">
                    <nav>
                        <div class="navbar bg-base-100 shadow">
                            <div class="navbar-left">
                                <label
                                    for="app-drawer"
                                    class="btn drawer-button btn-ghost"
                                >
                                    <span class="material-symbols-outlined">
                                        menu
                                    </span>
                                </label>
                            </div>
                            <div class="navbar-center">
                                <A class="text-xl btn btn-ghost" href="/">
                                    Silence
                                </A>
                            </div>
                        </div>
                    </nav>
                </div>
                <div class="drawer-side is-drawer-close:overflow-visible">
                    <label
                        for="app-drawer"
                        aria-label="close sidebar"
                        class="drawer-overlay"
                    ></label>
                    {props.children}
                    <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64">
                        <ul class="menu bg-base-200 min-h-full p-2">
                            <li>
                                <button
                                    class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                                    data-tip="Homepage"
                                >
                                    <span class="material-symbols-outlined">
                                        side_navigation
                                    </span>
                                    <span class="is-drawer-close:hidden">
                                        Homepage
                                    </span>
                                </button>
                            </li>
                            <li>
                                <button
                                    class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                                    data-tip="Settings"
                                >
                                    <span class="material-symbols-outlined">
                                        side_navigation
                                    </span>
                                    <span class="is-drawer-close:hidden">
                                        Settings
                                    </span>
                                </button>
                            </li>
                            <li class="bottom-0">
                                <label
                                    for="app-drawer"
                                    class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                                >
                                    <span class="material-symbols-outlined">
                                        side_navigation
                                    </span>
                                </label>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </>
    );
};
