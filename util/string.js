"use strict";

export function findClosingParenthesis(str, pos = 0) {
    let depth = 1;
    for (let i = pos + 1; i < str.length; ++i) {
        if (str[i] === ')' && --depth === 0) {
            return i;
        } else if (str[i] === '(') {
            depth++;
        }
    }
    return -1;
}
