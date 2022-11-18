"use strict";

export class Parameter {
    #name;
    #initialValue;
}

export class Symbol {
    #name;
    #arguments;

    constructor(name, args) {
        this.#name = name;
        this.#arguments = args;
    }

    get name() {
        return this.#name;
    }

    get arguments() {
        return this.#arguments;
    }

    equals(other) {
        return this.name === other.name && this.arguments.length === other.arguments.length;
    }

    toGenericString() {
        return `${this.name}(${this.arguments.map(_ => '')})`;
    }
}

export class Module {
    #symbol;
    #values;
    #isQuery;

    constructor(symbol, values, isQuery = false) {
        this.#symbol = symbol;
        this.#values = values;
        this.#isQuery = isQuery;
    }

    get name() {
        return this.#symbol.name;
    }

    get arguments() {
        return this.#symbol.arguments;
    }

    get values() {
        return this.#values;
    }

    get isQuery() {
        return this.#isQuery;
    }

    clone() {
        return new Module(this.#symbol, this.values, this.#isQuery);
    }
}

class LSystemState {
    axiom;
    parameters;
}

class Production {
    /**
     * The symbol this production operates on.
     */
    #symbol;

    /**
     * A list of symbols that have to precede the symbol.
     */
    #predecessors;

    /**
     * A list of symbols that have to succeed the symbol.
     */
    #successors;

    /**
     * A condition that has to be satisfied for this production to get applied.
     */
    #condition;

    // list of tuples / structs (prob, func)
    /**
     * A list of productions and their probabilities to be applied, sorted by their probabilities.
     */
    #operations;

    apply(module, state) {
        // todo: if more than one operation: select based on probability

        // todo: replace module
        return module.clone();
    }

    // todo: create condition
    condition(parameters, state) {
        this.#condition ? this.#condition(...parameters, ...state) : true;
    }

    rank() {
        return this.#predecessors.length + this.#successors.length + (this.#condition ? 1 : 0);
    }
}

export class LSystem {
    #symbols;
    #parameters;
    #axiom;
    #productions;

    constructor(symbols, parameters, axiom, productions) {
        this.#symbols = symbols;
        this.#parameters = parameters;
        this.#axiom = axiom;

        this.#productions = {};
        for (const p of productions) {
            const symbol = p.symbol().toGenericString();
            if (!this.#productions[symbol]) {
                this.#productions[symbol] = [];
            }
            this.#productions[symbol].push(p);
        }
    }

    evaluate(modules, state) {
        const result = [];
        for (const i in modules) {
            const m = modules[i];
            const p = this.#findProduction(m, state);
            result.push(p ? p.apply(m) : m.clone());
        }
        return result;
    }

    #getProductionsForSymbol(s) {
        return this.#productions[s.toGenericString()];
    }

    #findProduction(i, modules, state) {
        const module = modules[i];
        const candidates = this.#getProductionsForSymbol(module.symbol);
        if (!candidates) {
            return null;
        }
        candidates.sort((a, b) => b.rank() - a.rank());
        for (const c of candidates) {
            let matches = true;
            for (const j in c.predecessors) {
                if (i - j - 1 < 0) {
                    matches = false;
                    break;
                }
                if (!c.predecessors[c.predecessors.length - j - 1].equals(modules[i - j - 1].symbol)) {
                    matches = false;
                    break;
                }
            }
            if (matches) {
                for (const j in c.successors) {
                    if (i + j < modules.length) {
                        matches = false;
                        break;
                    }
                    if (!c.successors[j].equals(modules[i + j].symbol)) {
                        matches = false;
                        break;
                    }
                }
            }
            // todo: figure out what this needs (current module params & state, i.e., system params, I think)
            if (matches && c.condition(state, parameters)) {
                return c;
            }
        }
        return null;
    }

    // todo: refactor from other file
    static parseFromDefinition(definition) {}
}

export class LSystemEvaluator {
    #lSystem;
    #state;

    constructor(lSystem, state) {
        this.#lSystem = lSystem;
        this.#state = state;
    }

    next() {
        // todo: update state
        this.#lSystem.evaluate(this.#state.axiom, this.#state.parameters);
    }

    static parseFromDefinition(definition) {}
}
