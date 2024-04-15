/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>,
 * Gleb Krylov <gleb_cry@mail.ru>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["*.html", "./src/**/*.rs", "../cicero-app/src/**/*.rs"],
    },
    theme: {
        extend: {
            fontFamily: {
                'inter': ["Inter"]
            }
        },
    },
    plugins: [
        require('tailwind-scrollbar'),
    ],
}