/** @type {import('tailwindcss').Config} */
window.default = {
    content: {
        relative: true,
        files: ["*.html", "./frontend/**/*.tsx"],
    },
    theme: {
        extend: {},
    },
    plugins: [
        require('daisyui'),
    ],
};
