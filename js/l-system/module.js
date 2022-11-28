"use strict";

import {findClosingParenthesis} from "../util/string.js";

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

export class Symbol extends Clone {
    #name;
    #parameters;

    constructor({name, parameters = []}) {
        super();
        if (!name.length) {
            throw Error('symbol name must not be empty');
        }
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
        return `${this.name}(${this.parameters.map(_ => '_')})`;
    }

    toSerializable() {
        return { name: this.name, parameters: this.parameters };
    }

    toString(ignoreEmptyParameters = false) {
        if (ignoreEmptyParameters && !this.parameters.length) {
            return `${this.name}`;
        } else {
            return `${this.name}(${this.parameters})`;
        }
    }

    static fromString(str) {
        if (str.includes('(')) {
            const openIdx = str.indexOf('(');
            return new Symbol({
                name: str.slice(0, openIdx),
                parameters: str.slice(openIdx + 1, findClosingParenthesis(str, openIdx)).split(','),
            });
        } else {
            return new Symbol({name: str});
        }
    }
}

export class Module extends Clone {
    #name;
    #parameters;
    #isQuery;

    constructor({name, parameters = []}) {
        super();
        if (!name.length) {
            throw Error('module name must not be empty');
        }
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
        return `${this.name}(${this.parameters.map(_ => '_')})`;
    }

    toString(ignoreEmptyParameters = false) {
        if (ignoreEmptyParameters && !this.parameters.length) {
            return `${this.name}`;
        } else {
            return `${this.name}(${this.parameters})`;
        }
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

    constructor({axiom = [], parameters = {}}) {
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
        return this.#axiom.map(m => m.toString(true)).join('')
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

    apply(module, systemState) {
        return this.#func(Module.prototype.constructor, ...module.parameters, systemState.parameters);
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

    apply(module, systemState) {
        return this.#chooseOperation().apply(module, systemState);
    }

    condition(module, systemState) {
        return this.#condition ? this.#condition(...module.parameters, systemState.parameters) : true;
    }

    rank() {
        return this.#predecessors.length + this.#successors.length + (this.#condition ? 1 : 0);
    }

    get predecessors() {
        return this.#predecessors;
    }

    get successors() {
        return this.#successors;
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

        const productionSpecs = productions.map(p => this.#splitProductionDefinition(p))
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
            axiom: LSystemParser.#parseAxiom(axiom, symbols),
            productions: this.#processProductionSpecs(productionSpecs, symbols, parameters),
        })
    }

    static #conditionEquals(a, b) {
        let condA = `${a.condition.conditionDefinition}`;
        let condB = `${b.condition.conditionDefinition}`;
        for (const i in b.parameters) {
            condA = condA.replace(b.parameters[i], a.parameters[i]);
            condB = condB.replace(b.parameters[i], a.parameters[i]);
        }
        return condA === condB;
    }

    #processProductionSpecs(productionSpecs, symbols, parameters) {
        return Object.keys(productionSpecs).reduce((productions, symbolName) => {
            const specs = productionSpecs[symbolName].map(({probability, moduleSpecification, condition, body}) => {
                return {
                    probability,
                    symbol: moduleSpecification.module,
                    predecessors: LSystemParser.#parseProductionPreOrSuccessors(moduleSpecification.predecessors, symbols),
                    successors: LSystemParser.#parseProductionPreOrSuccessors(moduleSpecification.successors, symbols),
                    condition: this.#parseProductionCondition(condition, moduleSpecification.module.parameters, parameters),
                    body: this.#parseProductionBody(body, moduleSpecification.module.parameters, parameters, symbols),
                };
            });

            const processed = new Set();
            for (const i in specs) {
                if (processed.has(i)) {
                    continue;
                }

                const spec = specs[i];
                const operations = [];
                for (let j = i; j < specs.length; ++j) {
                    const s = specs[j];
                    if (s.predecessors.length !== spec.predecessors.length ||
                        s.successors.length !== spec.successors.length ||
                        !LSystemParser.#conditionEquals(spec, s)
                    ) {
                        continue;
                    }
                    let candidate = true;
                    for (const l in s.predecessors) {
                        if (!s.predecessors[l].equals(spec.predecessors[l])) {
                            candidate = false;
                            break;
                        }
                    }
                    if (!candidate) {
                        continue;
                    }
                    for (const l in s.successors) {
                        if (!s.successors[l].equals(spec.successors[l])) {
                            candidate = false;
                            break;
                        }
                    }
                    if (!candidate) {
                        continue;
                    }
                    operations.push(s);
                    processed.add(j);
                }

                const numNoProbability = operations.reduce((sum, c) => sum + (c.probability ? 0 : 1), 0);
                const sumProbability = operations.reduce((sum, c) => sum + (c.probability || 0), 0);
                const defaultProbability = (1.0 - sumProbability) / numNoProbability;

                // todo: make sameO
                const ops = [];
                let offset = 0.0;
                for (const i in operations) {
                    const op = operations[i];
                    if (i === operations.length - 1) {
                        ops.push(new Operation(1.0, op.body));
                    } else {
                        const probability = op.probability || defaultProbability;
                        ops.push(new Operation(offset + probability, op.body));
                        offset += probability;
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
        const funcArgs = [...moduleParameters];
        if (Object.keys(requiredSystemParameters).length) {
            funcArgs.push(systemParamsString);
        }
        return {
            func: conditionDefinition.length ? new Function(
                ...funcArgs,
                `return !!(${conditionDefinition})`,
            ) : null,
            conditionDefinition,
        };
    }

    #parseProductionBody(bodyDefinition, moduleParameters, systemParameters, symbols) {
        const producedSymbols = LSystemParser.#parseProductionPreOrSuccessors(bodyDefinition, symbols);
        const body = producedSymbols.map(s => {
            return `new ctor({ name: '${s.name}', parameters: [${s.parameters}] })`
        });
        const funcArgs = ['ctor', ...moduleParameters];
        if (Object.keys(systemParameters).length) {
            funcArgs.push(`{${Object.keys(systemParameters).map(k =>`${k}=${systemParameters[k]}`)}}`);
        }
        return new Function(
            ...funcArgs,
            `return [${body}];`
        );
    }

    // todo: same as parseAxiom except for Symbol instead of Module -> combine and add if statement
    static #parseProductionPreOrSuccessors(moduleDefinitions, symbols) {
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
                            moduleDefinitions = moduleDefinitions.slice(moduleDefinition.length);
                        }
                    } else if (s.parameters.length === 0) {
                        found = true;
                        parsedSymbols.push(new Symbol({
                            name: s.name,
                            parameters: [],
                        }));
                        moduleDefinitions = moduleDefinitions.slice(s.name.length);
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

    static #parseAxiom(axiom, symbols) {
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
                            axiom = axiom.slice(moduleDefinition.length);
                        }
                    } else if (s.parameters.length === 0) {
                        found = true;
                        parsedAxiom.push(new Module({
                            name: s.name,
                            parameters: [],
                        }));
                        axiom = axiom.slice(s.name.length);
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
        const predSeparator = definition.indexOf(this.#modulePredecessorSeparator);
        const succSeparator = definition.indexOf(this.#moduleSuccessorSeparator);
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
            probability: probSepIdx < 0 ? null : Number(cleanDefinition.slice(0, probSepIdx)),
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
        this.#productions = productions;
    }

    evaluate(systemState) {
        const result = [];
        for (const i in systemState.axiom) {
            const m = systemState.axiom[i];
            const p = this.#findProduction(i, systemState);
            result.push(p ? p.apply(m, systemState) : m.clone());
        }
        systemState.axiom = result.flat();
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
                if (!c.predecessors[c.predecessors.length - j - 1].equals(modules[i - j - 1])) {
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
                    if (!c.successors[j].equals(modules[i + j])) {
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

    get axiom() {
        return this.#axiom.map(m => m.clone());
    }

    get parameters() {
        return JSON.parse(JSON.stringify(this.#parameters));
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

    // todo: this is an infinite loop?
    nth(i) {
        if (this.#previousStates.length > i) {
            return this.#previousStates[i].toString;
        }
        while (i !== this.#previousStates.length) {
            this.next();
        }
        return this.#state.toString();
    }
}

function runLSystem(definition) {
    const lSystem = new LSystemParser({}).parseLSystem(definition);
    const evaluator = new LSystemEvaluator(lSystem);
    console.log(definition);
    console.log(lSystem);
    for (const r of definition.results) {
        const result = evaluator.next();
        console.log(r);
        console.log(result);
    }
    if (!definition.results.length) {
        for (let i = 0; i < 5; ++i) {
            console.log(evaluator.next());
        }
    }
    return lSystem;
}

export function testLSystems() {
    const deterministic = {
        axiom: 'a',
        productions: [
            'a->ab',
            'b->ac'
        ],
        parameters: {},
        alphabet: ['a', 'b', 'c'],
        results: [
            'ab',
            'abac',
            'abacabc',
            'abacabcabacc'
        ]
    }

    const stochastic = {
        axiom: 'F',
        productions: [
            'F->F[+F]F[-F]F',
            'F->F[+F]F',
            'F->F[-F]F'
        ],
        parameters: {},
        alphabet: ['F', '[', ']', '+', '-'],
        results: []
    }

    const contextSensitive = {
        axiom: 'baaaaaaa',
        productions: [
            'b<a -> b',
            'b->a'
        ],
        parameters: {},
        alphabet: ['a', 'b'],
        results: [
            'abaaaaaa',
            'aabaaaaa',
            'aaabaaaa',
            'aaaabaaa',
            'aaaaabaa',
            'aaaaaaba'
        ]
    }

    const parametric = {
        axiom: 'B(2)A(4,4)',
        productions: [
            'A(x,y): y<=3 -> A(x*2,x+y)',
            'A(x,y): y> 3 -> B(x)A(x/y,0)',
            'B(x)  : x< 1 -> C',
            'B(x)  : x>=1 -> B(x-1)'
        ],
        parameters: {},
        alphabet: ['A(x,y)', 'B(x)', 'C'],
        results: [
            'B(1)B(4)A(1,0)',
            'B(0)B(3)A(2,1)',
            'CB(2)A(4,3)',
            'CB(1)A(8,7)',
            'CB(0)B(8)A(1.142,0)'
        ]
    }

    const treeModel = {
        axiom: 'A(1,10)',
        productions: [
            'A(l,w) -> F(l,w)[&(a0)B(l*r2,w*wr)]/(d)A(l*r1,w*wr)',
            'B(l,w) -> F(l,w)[+(-a1)$C(l*r2,w*wr)]C(l*r1,w*wr)',
            'C(l,w) -> F(l,w)[+(a1)$B(l*r2,w*wr)]B(l*r1,w*wr)'
        ],
        parameters: {r1: 7.0, r2: 7.1, a0: 7.2, a1: 7.3, d: 7.4, wr: 7.5},
        alphabet: ['A(l,w)', 'B(l,w)', 'C(l,w)', 'F(l,w)', '[', ']', '+(a)', '&(a)', '/(d)', '$'],
        results: []
    };

    const queryParameters = {
        axiom: 'A',
        productions: [
            'A->F(1)?P(x,y)-A',
            'F(k)->F(k+1)'
        ],
        parameters: {},
        alphabet: ['A', 'F(k)', '?P(x,y)', '-'],
        results: [
            'F(1)?P(0,1)-A',
            'F(2)?P(0,2)-F(1)?P(1,2)-A',
            'F(3)?P(0,3)-F(1)?P(2,3)-F(1)?P(2,2)-A'
        ],
        interpreter: {
            'F': (k, ctx) => {
                const result = {...ctx};
                const distance = k * ctx.dir;
                if (ctx.axis === 'x') {
                    result.x += distance;
                } else {
                    result.y += distance;
                }
                return result;
            }
        }
    };

    const definitions = [
        deterministic,
        stochastic,
        contextSensitive,
        parametric,
        treeModel,
        queryParameters
    ];
    const systems = [];
    for (const d of definitions) {
        systems.push(runLSystem(d));
    }
    return systems;
}
