"use strict";

import {findClosingParenthesis} from "../util/string";

class ToSerializable {
    toSerializable() {
        return {};
    }
}

/**
 * A base class for serializable classes
 */
class Clone extends ToSerializable {
    clone() {
        return new this.constructor(JSON.parse(JSON.stringify(this.toSerializable())));
    }
}

export class Parameter {
    #name;
    #initialValue;
}

export class Symbol extends Clone {
    #name;
    #parameters;

    constructor({name, parameters}) {
        super();
        this.#name = name;
        this.#parameters = parameters;
    }

    get name() {
        return this.#name;
    }

    get parameters() {
        return this.#parameters;
    }

    equals(other) {
        return this.name === other.name && this.parameters.length === other.parameters.length;
    }

    toGenericString() {
        return `${this.name}(${this.parameters.map(_ => '')})`;
    }

    toSerializable() {
        return { name: this.name, parameters: this.parameters };
    }

    createModule(values) {
        console.assert(values.length === this.parameters.length);
        return new Module({
            name: this.name,
            parameters: values
        });
    }

    static fromString(str) {
        const openIdx = str.indexOf('(');
        return new Symbol({
            name: str.split(0, openIdx),
            parameters: openIdx > 0 ? str.slice(openIdx + 1, findClosingParenthesis(str, openIdx)).split(',') : []
        });
    }
}

export class Module extends Clone {
    #name;
    #parameters;
    #isQuery;

    constructor({name, parameters}) {
        super();
        this.#name = name;
        this.#parameters = parameters;
        this.#isQuery = name[0] === '?';

        if (this.#isQuery) {
            throw Error("query modules are not supported yet");
        }
    }

    get name() {
        return this.#name;
    }

    get parameters() {
        return this.#parameters;
    }

    get isQuery() {
        return this.#isQuery;
    }

    toGenericString() {
        return `${this.name}(${this.parameters.map(_ => '')})`;
    }

    toString() {
        return `${this.name}(${this.parameters})`;
    }

    toSerializable() {
        return { name: this.#name, parameters: this.parameters };
    }

    static fromString(str) {
        const symbol = Symbol.fromString(str);
        return new Module({
            name: symbol.name,
            parameters: symbol.parameters.map(Number),
        });
    }
}

class LSystemState extends Clone {
    #axiom;
    parameters;

    constructor({axiom, parameters}) {
        super();
        this.#axiom = axiom;
        this.parameters = parameters;
    }

    get axiom() {
        return this.#axiom;
    }

    set axiom(value) {
        this.#axiom = value;
    }

    toSerializable() {
        return {
            axiom: this.#axiom.map(m => m.toSerializable()),
            parameters: this.parameters,
        }
    }

    toString() {
        return this.#axiom.map(m => m.toString()).join('')
    }
}

class Operation {
    #probability;
    #func;

    constructor(probability, func) {
        this.#probability = probability;
        this.#func = func;
    }

    get probability() {
        return this.#probability;
    }

    apply(module, state) {
        return this.#func(Module.prototype.constructor, ...module.parameters, state);
    }
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

    /**
     * A list of productions and their probabilities to be applied, sorted by their probabilities.
     */
    #operations;

    constructor({symbol, predecessors, successors, condition = undefined, operations}) {
        this.#symbol = symbol;
        this.#predecessors = predecessors;
        this.#successors = successors;
        this.#condition = condition;
        this.#operations = operations;
    }

    #chooseOperation() {
        const probability = Math.random();
        for (const op of this.#operations) {
            if (probability <= op.probability) {
                return op;
            }
        }
    }

    apply(module, state) {
        return this.#chooseOperation().apply(module, state);
    }

    condition(module, systemState) {
        this.#condition ? this.#condition(...module.parameters, ...systemState.parameters) : true;
    }

    rank() {
        return this.#predecessors.length + this.#successors.length + (this.#condition ? 1 : 0);
    }
}

export class LSystemParser {
    #probabilitySeparator;
    #modulePredecessorSeparator;
    #moduleSuccessorSeparator;
    #conditionSeparator;
    #operationSeparator;


    constructor({
        probabilitySeparator = ';',
        modulePredecessorSeparator = '<',
        moduleSuccessorSeparator = '>',
        conditionSeparator = ':',
        operationSeparator = '->'
    }) {
        this.#probabilitySeparator = probabilitySeparator;
        this.#modulePredecessorSeparator = modulePredecessorSeparator;
        this.#moduleSuccessorSeparator = moduleSuccessorSeparator;
        this.#conditionSeparator = conditionSeparator;
        this.#operationSeparator = operationSeparator;
    }

    parseLSystem({alphabet = undefined, parameters = {}, productions, axiom}) {
        // two modules with the same name but different numbers of args are allowed!
        // module names are arbitrary, but the following symbols are not allowed:
        //  ? ( ) < > : ; -
        // if no alphabet is given it is parsed from the axiom and the productions in the following way
        //  - symbols for which productions exist are first extracted from the productions
        //  -

        const productionSpecs = productions.map(this.#splitProductionDefinition)
            .reduce((symbolsToProdSpecs, p) => {
                const symbol = p.moduleSpecification.module.toGenericString();
                if (!symbolsToProdSpecs[symbol]) {
                    symbolsToProdSpecs[symbol] = [];
                }
                symbolsToProdSpecs[symbol].push(p);
                return symbolsToProdSpecs;
            }, {});

        const symbols = [];
        if (!alphabet) {
            // todo: parse alphabet from productions & axiom
            for (const key of Object.keys(productionSpecs)) {
                symbols.push(productionSpecs[key][0].moduleSpecification.module);
            }

        } else {
            symbols.push(...alphabet.map(Symbol.fromString))
        }

        // sort by most specific to the least specific
        symbols.sort((a, b) => {
            if (a.name === b.name) {
                return b.parameters.length - a.parameters.length;
            }
            return b.name.length - a.name.length;
        });

        // at this point the alphabet must be complete, so we can parse the axiom & production specs
        return new LSystem({
            symbols,
            parameters,
            axiom: this.#parseAxiom(axiom, symbols),
            productions: this.#processProductionSpecs(productionSpecs, symbols, parameters),
        })
    }

    #processProductionSpecs(productionSpecs, symbols, parameters) {
        return Object.keys(productionSpecs).reduce((productions, symbolName) => {
            // todo: allow for productions without condition!
            const specs = productionSpecs[symbolName].map(({probability, moduleSpecification, condition, body}) => {
                return {
                    // todo: parse probability
                    probability,
                    symbol: moduleSpecification.module,
                    predecessors: this.#parseProductionPreOrSuccessors(moduleSpecification.predecessors, symbols),
                    successors: this.#parseProductionPreOrSuccessors(moduleSpecification.successors, symbols),
                    condition: this.#parseProductionCondition(condition, moduleSpecification.module.parameters, parameters),
                    body: this.#parseProductionBody(body, moduleSpecification.module.parameters, parameters, symbols),
                };
            });

            // todo: group by predecessors & successors & condition and construct productions
            const processed = new Set();
            for (const i in specs) {
                if (processed.has(i)) {
                    continue;
                }

                const spec = specs[i];
                const operations = [];
                for (const s of specs) {
                    if (s.predecessors.length !== spec.predecessors.length ||
                        s.successors.length !== spec.successors.length //||
                        // todo: compare conditions
                        //new Set(Object.keys(s.condition.requiredSystemParameters) )
                    ) {
                        continue;
                    }
                    let candidate = true;
                    for (const j in s.predecessors) {
                        if (!s.predecessors[i].equals(spec.predecessors[i])) {
                            candidate = false;
                            break;
                        }
                    }
                    if (!candidate) {
                        continue;
                    }
                    for (const j in s.successors) {
                        if (!s.successors[i].equals(spec.successors[i])) {
                            candidate = false;
                            break;
                        }
                    }
                    if (!candidate) {
                        continue;
                    }
                    operations.push(s);
                }

                // todo: make sameO
                const ops = [];

                // todo: if any operations have no probability, they need to be assigned (1-sumOfProbs)/numNoProb

                let offset = 0.0;
                for (const i in operations) {
                    const op = operations[i];
                    if (i === operations.length - 1) {
                        ops.push(new Operation(1.0, op.body));
                    } else {
                        ops.push(new Operation(offset + op.probability, op.body));
                        offset += op.probability;
                    }
                }

                if (!productions[symbolName]) {
                    productions[symbolName] = [];
                }
                productions[symbolName].push(new Production({
                    symbol: spec.symbol,
                    predecessors: spec.predecessors,
                    successors: spec.successors,
                    condition: spec.condition.func,
                    operations: ops,
                }));

                processed.add(i);
            }

            return productions;
        }, {});
    }

    #parseProductionCondition(conditionDefinition, moduleParameters, systemParameters) {
        const requiredSystemParameters = [];
        for (const p of Object.keys(systemParameters)) {
            if (conditionDefinition.includes(p)) {
                requiredSystemParameters.push(p);
            }
        }
        const systemParamsString = `{${
            requiredSystemParameters.map(p => `${p}=${systemParameters[p]}`)
        }`;
        return {
            func: new Function(
                ...moduleParameters,
                systemParamsString,
                conditionDefinition,
            ),
            requiredSystemParameters,
        };
    }

    #parseProductionBody(bodyDefinition, moduleParameters, systemParameters, symbols) {
        const producedSymbols = this.#parseProductionPreOrSuccessors(bodyDefinition, symbols);
        const body = producedSymbols.map(s => {
            return `new ctor(${s.name},[${s.parameters}])`
        });
        return new Function(
            'ctor',
            ...moduleParameters,
            `{${Object.keys(systemParameters).map(k =>`${k}=${systemParameters[k]}`)})}}`,
            `return [${body}];`
        );
    }

    // todo: same as parseAxiom except for Symbol instead of Module -> combine and add if statement
    #parseProductionPreOrSuccessors(moduleDefinitions, symbols) {
        const parsedSymbols = [];
        const originalAxiom = `${moduleDefinitions}`;
        while (moduleDefinitions.length) {
            let found = false;
            for (const s of symbols) {
                if (moduleDefinitions.indexOf(s.name) === 0) {
                    const openIdx = moduleDefinitions.indexOf('(');
                    const closeIdx = findClosingParenthesis(moduleDefinitions, openIdx);
                    if (openIdx === s.name.length) {
                        const moduleDefinition = moduleDefinitions.slice(0, closeIdx + 1);
                        if (s.equals(Symbol.fromString(moduleDefinition))) {
                            found = true;
                            parsedSymbols.push(Symbol.fromString(moduleDefinition));
                            moduleDefinitions = moduleDefinitions.split(moduleDefinition)[1];
                        }
                    } else if (s.parameters.length === 0) {
                        found = true;
                        parsedSymbols.push(new Symbol({
                            name: s.name,
                            parameters: [],
                        }));
                        moduleDefinitions = moduleDefinitions.split(s.name)[1];
                    }
                }
                if (found) {
                    break;
                }
            }
            if (!found) {
                throw Error(`Incomplete alphabet ${symbols.map(s => s.toString())} for modules ${originalAxiom}`);
            }
        }
        return parsedSymbols;
    }

    #parseAxiom(axiom, symbols) {
        const parsedAxiom = [];
        const originalAxiom = `${axiom}`;
        while (axiom.length) {
            let found = false;
            for (const s of symbols) {
                if (axiom.indexOf(s.name) === 0) {
                    const openIdx = axiom.indexOf('(');
                    const closeIdx = findClosingParenthesis(axiom, openIdx);
                    if (openIdx === s.name.length) {
                        const moduleDefinition = axiom.slice(0, closeIdx + 1);
                        if (s.equals(Symbol.fromString(moduleDefinition))) {
                            found = true;
                            parsedAxiom.push(Module.fromString(moduleDefinition));
                            axiom = axiom.split(moduleDefinition)[1];
                        }
                    } else if (s.parameters.length === 0) {
                        found = true;
                        parsedAxiom.push(new Module({
                            name: s.name,
                            parameters: [],
                        }));
                        axiom = axiom.split(s.name)[1];
                    }
                }
                if (found) {
                    break;
                }
            }
            if (!found) {
                throw Error(`Incomplete alphabet ${symbols.map(s => s.toString())} for axiom ${originalAxiom}`);
            }
        }
        return parsedAxiom;
    }

    #splitModuleSpecification(definition) {
        const predSeparator = definition.indexOf('<');
        const succSeparator = definition.indexOf('>');
        return {
            predecessors: predSeparator < 0 ? '' : definition.slice(0, predSeparator),
            successors: succSeparator < 0 ? '' : definition.slice(succSeparator + 1),
            module: Symbol.fromString(
                definition.slice(predSeparator + 1, succSeparator < 0 ? definition.length : succSeparator)
            )
        }
    }

    #splitProductionDefinition(definition) {
        const cleanDefinition = definition.replace(/\s/g, '');
        let probSepIdx = cleanDefinition.indexOf(this.#probabilitySeparator);
        let condSepIdx = cleanDefinition.indexOf(this.#conditionSeparator);
        let opSepIdx = cleanDefinition.indexOf(this.#operationSeparator);

        console.assert(opSepIdx > 0);
        console.assert(probSepIdx < condSepIdx < opSepIdx);

        // probabilities & conditions are optional
        if (condSepIdx < 0) condSepIdx = opSepIdx;

        return {
            probability: probSepIdx < 0 ? '' : cleanDefinition.slice(0, probSepIdx),
            moduleSpecification: this.#splitModuleSpecification(cleanDefinition.slice(probSepIdx + 1, condSepIdx)),
            condition: cleanDefinition.slice(condSepIdx + 1, opSepIdx),
            body: cleanDefinition.slice(opSepIdx + 2)
        };
    }
}

export class LSystem {
    #symbols;
    #parameters;
    #axiom;
    #productions;

    constructor({symbols, parameters, axiom, productions}) {
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

    evaluate(systemState) {
        const result = [];
        for (const i in systemState.axiom) {
            const m = systemState.axiom[i];
            const p = this.#findProduction(i, systemState);
            result.push(p ? p.apply(m, systemState) : m.clone());
        }
        systemState.axiom(result);
        return systemState;
    }

    #getProductionsForModule(m) {
        return this.#productions[m.toGenericString()];
    }

    #findProduction(i, systemState) {
        const modules = systemState.axiom;
        const module = modules[i];
        const candidates = this.#getProductionsForModule(module);
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
            if (matches && c.condition(module, systemState)) {
                return c;
            }
        }
        return null;
    }

    get parameters() {
        return JSON.parse(JSON.stringify(this.#parameters));
    }



    // todo: refactor from other file
    static parseFromDefinition({alphabet = undefined, parameters = [], productions, axiom}) {

    }
}

export class LSystemEvaluator {
    #lSystem;
    #state;
    #previousStates;

    constructor(lSystem) {
        this.#lSystem = lSystem;
        this.#state = new LSystemState({ axiom: lSystem.axiom, parameters: lSystem.parameters });
        this.#previousStates = [];
    }

    next() {
        this.#previousStates = this.#state.clone();
        this.state = this.#lSystem.evaluate(this.#state);
        return this.state.toString();
    }

    nth(i) {
        if (this.#previousStates.length > i) {
            return this.#previousStates[i].toString;
        }
        while (i !== this.#previousStates.length) {
            this.next();
        }
        return this.#state.toString();
    }

    static parseFromDefinition({parameters, production, axiom}) {

    }
}

export function testLSystems() {
    const deterministic = {
        axiom: 'a',
        productions: [
            'a->ab',
            'b->ac'
        ],
        parameters: {},
        symbols: ['a', 'b', 'c'],
        results: [
            'ab',
            'abac',
            'abacabc',
            'abacabcabacc'
        ]
    }

    const deterministicLSystem = LSystem.parseFromDefinition(deterministic);
}
