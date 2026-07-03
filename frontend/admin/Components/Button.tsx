// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

export const confirm_btn = (callback: Function) => {
    return async (event: MouseEvent) => {
        let button = event.currentTarget! as HTMLButtonElement;

        if (button.getAttribute("data-confirmed")! === "true") {
            await callback();
        } else {
            let current_text = button.innerHTML;

            setTimeout(() => {
                button.setAttribute("data-confirmed", "false");
                button.classList.remove("animate-pulse");
                button.innerText = current_text;
            }, 5000);

            button.setAttribute("data-confirmed", "true");
            button.classList.add("animate-pulse");
            button.innerText = `${current_text} (confirm)`;
        }
    };
};
