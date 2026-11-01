// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

export interface Session {
    user_id: number;
    name: string;
    email: string;
    role: string | null;
}

export const fetcher = async (): Promise<Session | null> => {
    // Check whether the server is listening on a loopback address.
    const loopback_req = await fetch("/api/internal/loopback");

    if (!loopback_req.ok) console.clear();

    const is_loopback = (await loopback_req.json()) as {
        loopback: boolean;
    };

    if (is_loopback.loopback) {
        console.log(
            "Login as guest as the server is listening on a loopback address.",
        );
        return null;
    }

    const whoami = await fetch("/api/internal/whoami");

    if (!whoami.ok) console.clear();

    if (whoami.status === 200) {
        const data = await whoami.json();

        if ((data as Session).role !== "admin") {
            const res = await fetch("/api/internal/bootstrap");

            const maybe_new_role = (await res.json()) as {
                role: string | null;
            };

            data.role = maybe_new_role.role;
        }

        return data as Session;
    } else {
        document.location.replace("/auth?redirect=/admin");
        throw new Error("User is not logged in.");
    }
};

export const logout = async () => {
    await fetch("/api/internal/logout");
    document.location = "/";
};
