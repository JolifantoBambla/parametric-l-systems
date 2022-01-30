"use strict";

const foo =
    `B(2)A(4,4)
     A(x,y): y<=3 -> A(x*2,x+y)
     A(x,y): y> 3 -> B(x)A(x/y,0)
     B(x)  : x< 1 -> C
     B(x)  : x>=1 -> B(x-1)`;

function parseSystemDefinition(definition) {
  // first pass
  // split productions in four parts
  // don't initialize functions in condition and transformation yet

  // gather all parameters in the system

  // second pass
  // initialize functions in condition and tranformation

  // return L-system
}

function findClosingParenthesis(str, pos = 0) {
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

class Module {
  constructor(name, args = [], isQuery = false) {
    this.name = name;
    this.args = args;
    this.isQuery = isQuery;
  }
  toString() {
    if (this.args.length) {
      return `${this.isQuery ? '?': ''}${this.name}(${this.args.join(',')})`;
    } else {
      return this.name;
    }
  }
  static fromDefinition (definition) {
    const parenOpenIndex = definition.indexOf('(');
    const args = parenOpenIndex > 0 ?
      definition.slice(parenOpenIndex + 1, findClosingParenthesis(definition, parenOpenIndex)).split(',') : [];
    const isQuery = definition[0] === '?';
    return new Module(definition[isQuery ? 1 : 0], args, isQuery);
  }
}

class LSystem {
  static #getProductionProbability (definition) {
    const parts = definition.split(';');
    if (parts.length === 2) {
      try {
        return parseFloat(parts[0]);
      } catch (e) {} // just swallow it...
    }
    return 1.0;
  }

  static #getProductionModuleSpecification (definition) {
    const parts = definition.split(':');
  }

  static #getProductionBody (definition) {
    const parts = definition.split('->');
    return parts[parts.length - 1];
  }

  static #splitProductionDefinition (definition) {
    const cleanDefinition = definition.replace(/\s/g, '');
    let semicolonIndex = cleanDefinition.indexOf(';');
    let colonIndex = cleanDefinition.indexOf(':');
    let arrowIndex = cleanDefinition.indexOf('->');

    console.assert(arrowIndex > 0);
    console.assert(semicolonIndex < colonIndex < arrowIndex);

    // probabilities & conditions are optional
    if (colonIndex < 0) colonIndex = arrowIndex;

    return {
      probability: semicolonIndex < 0 ? '' : cleanDefinition.slice(0, semicolonIndex),
      moduleSpecification: cleanDefinition.slice(semicolonIndex + 1, colonIndex),
      condition: cleanDefinition.slice(colonIndex + 1, arrowIndex),
      body: cleanDefinition.slice(arrowIndex + 2)
    };
  }

  static #parseModuleString (moduleString) {
    const modules = [];
    for (let i = 0; i < moduleString.length; ++i) {
      const parenOpenIndex = i + (moduleString[i] === '?' ? 2 : 1);
      const moduleEndIndex =
        (moduleString[parenOpenIndex] === '(' ? findClosingParenthesis(moduleString, parenOpenIndex) : i) + 1;
      modules.push(Module.fromDefinition(moduleString.slice(i, moduleEndIndex)));
      i = moduleEndIndex - 1;
    }
    return modules;
  }

  static #parseModuleSpecification (definition) {
    const lessThanIndex = definition.indexOf('<');
    const greaterThanIndex = definition.indexOf('>');
    return {
      predecessors: this.#parseModuleString(lessThanIndex < 0 ? '' : definition.slice(0, lessThanIndex)),
      successors: this.#parseModuleString(greaterThanIndex < 0 ? '' : definition.slice(greaterThanIndex + 1)),
      module: this.#parseModuleString(
        definition.slice(lessThanIndex + 1, greaterThanIndex < 0 ? definition.length : greaterThanIndex))
    }
  }

  static parseProductionDefinition (definition) {
    const {probability, moduleSpecification, condition, body} = this.#splitProductionDefinition(definition);

    console.log('new rule:', definition);
    console.log('probability', probability);
    console.log('moduleSpecification', moduleSpecification);
    console.log('condition', condition);
    console.log('body', body);

    const moduleSpec = this.#parseModuleSpecification(moduleSpecification);
    const variables = [
      ...moduleSpec.predecessors.flatMap(m => m.args),
      ...moduleSpec.module.flatMap(m => m.args),
      ...moduleSpec.successors.flatMap(m => m.args),
    ];
    console.assert(variables.length === new Set(variables).size,
      'Variables in module specification must be unique.');

    // todo: get parameters from condition & body (args of modules)
    // nope: they will have to be supplied together with the rest of the system

    const bodyModules = this.#parseModuleString(body);
    console.log(bodyModules.map(p => p.toString()));
    console.log(moduleSpec.predecessors.map(p => p.toString()).join(''));
    console.log(moduleSpec.module.map(p => p.toString()).join(''));
    console.log(moduleSpec.successors.map(p => p.toString()).join(''));
  }
}


class Symbol {
    constructor(definition) {
        const parts = definition.split(')')[0].split('(');
        console.assert(parts.length <= 2, `Invalid symbol definition: ${parts}`);
        this.name = parts[0];
        this.parameters = parts[1].split(',');
    }
    toString() {
        if (this.parameters.length) {
            return `${this.name}(${this.parameters.join(',')})`;
        }
        return this.name;
    }
}


class X {
    constructor(name, args = []) {
        this.name = name;
        this.args = args;
    }
    toString() {
        if (this.args.length) {
            return `${this.name}(${this.args.join(',')})`;
        }
        return `${this.name}`;
    }
}

class Result {
    constructor(definition, args =[], systemParameters = []) {
        this.definition = definition;
        let openingIndex = 0;
        const resultParts = [];
        while (openingIndex !== -1) {
            const last = openingIndex;
            openingIndex = definition.indexOf('(', openingIndex);
            if (openingIndex !== -1) {
                resultParts.push(definition.substr(last, openingIndex -  last + 1));

                const closingIndex = findClosingParenthesis(definition, openingIndex);

                const forms = definition.substr(openingIndex + 1, closingIndex - openingIndex - 1);

                if (!forms.includes('(')) {
                    resultParts.push(forms.split(','));
                } else {
                    // todo: handle functions...
                    console.log(forms);
                }

                openingIndex = closingIndex;
            } else {
                resultParts.push(definition.substr(last));
            }
        }
        this.formatString = `\`${resultParts.map(p => {
            if (typeof p === 'string' || p instanceof String) {
                return p
            } else {
                return `\${${p.join('},${')}}`;
            }
        }).join('')}\``;
        this.func = new Function(...args, ...systemParameters, `return ${this.formatString}`);


        const formatCode = [];
        for (let i = 0; i < definition.length; ++i) {
            const symbol = definition[i];
            if (i + 1 < definition.length && definition[i + 1] === '(') {
                const openingIndex = i + 2;
                const closingIndex = findClosingParenthesis(definition, openingIndex);
                formatCode.push(`new X('${symbol}', [${definition.substr(openingIndex, closingIndex - openingIndex)}])`);
                i = closingIndex;
            } else {
                formatCode.push(`new X('${symbol}')`);
            }
        }
        this.func2 = new Function(...args, ...systemParameters, `return [${formatCode.join(',')}]`);
    }
}

class Production {
    #condition;

    constructor(definition) {
        let parts = definition.replace(/\s/g, '').split(':');
        this.symbol = new Symbol(parts[0]);

        // todo: parse parameters from definition
        const systemParameters = [];

        parts = parts[1].split('->');
        this.conditionDefinition = parts[0];
        this.#condition = new Function(
            ...this.symbol.parameters, ...systemParameters, `return !!(${this.conditionDefinition})`);

        this.result = new Result(parts[1], this.symbol.parameters, systemParameters);
    }

    condition(args, systemParameters) {
        return this.#condition(...args, ...systemParameters);
    }

    apply(args, systemParameters) {
        return this.result.func(args, systemParameters);
    }

    toString() {
        return `${this.symbol}: ${this.conditionDefinition}$ -> ${this.result}`;
    }
}

function next(axiom, productionsMap, systemParameters) {
    let nextIteration = '';
    for (let i = 0; i < axiom.length; ++i) {
        const symbol = axiom[i];
        let parameters = [];
        if (i !== axiom.length - 1 && axiom[i+1] === '(') {
            parameters = axiom
                .substr(i+2,axiom.indexOf(')', i+2) - (i + 2))
                .split(',')
                .map(p => parseFloat(p));
        }
        let appliedRule = false;
        if (productionsMap[symbol]) {
            const rules = productionsMap[symbol][parameters.length];
            if (rules) {
                for (const r of rules) {
                    if (r.condition(parameters, systemParameters)) {
                        nextIteration += r.apply(...parameters, ...systemParameters);
                        appliedRule = true;
                        break;
                    }
                }
            }
        }
        if (!appliedRule) {
            nextIteration += symbol;
            if (parameters.length) {
                nextIteration += axiom.substr(i+1,axiom.indexOf(')', i+2) - (i + 1))
            }
        }
        if (parameters.length) {
            i += parameters.join(',').length + 2;
        }
    }
    return nextIteration;
}

function next2(axiom, productionsMap, systemParameters) {
    let nextIteration = [];
    nextIteration = axiom.flatMap(s => {
        const symbol = s.name;
        const parameters = s.args;
        if (productionsMap[symbol]) {
            const rules = productionsMap[symbol][parameters.length];
            if (rules) {
                for (const r of rules) {
                    if (r.condition(parameters, systemParameters)) {
                        return r.result.func2(...parameters, ...systemParameters);
                    }
                }
            }
        }
        return s;
    });
    return nextIteration;
}



// assumptions: no function name is longer than one character!

function test() {
    const lines = foo.split('\n');
    const axiom = lines[0];
    const productions = lines.slice(1).map(p => new Production(p));
    console.log('lines', lines);
    console.log('axiom', axiom);
    console.log('productions', productions);
    const productionsMap = productions.reduce((m, p) => {
        if (!(p.symbol.name in m)) {
            m[p.symbol.name] = {};
        }
        if (!m[p.symbol.name][p.symbol.parameters.length]) {
            m[p.symbol.name][p.symbol.parameters.length] = [];
        }
        m[p.symbol.name][p.symbol.parameters.length].push(p);
        return m;
    }, {});

    const numIterations = 1000;
    const systemParameters = [];

    let start = performance.now();

    let currentAxiom = [new X('B',[2]), new X('A', [4,4])];
    for (let i = 0; i < numIterations; ++i) {
        currentAxiom = next2(currentAxiom, productionsMap, systemParameters);
        //console.log('next', currentAxiom.map(s => s.toString()).join(''));
    }
    console.log('obj', performance.now() - start);

    // looks like the string implementation is much faster...
    start = performance.now();
    currentAxiom = axiom;
    for (let i = 0; i < numIterations; ++i) {
        currentAxiom = next(currentAxiom, productionsMap, systemParameters);
        //console.log('next', currentAxiom);
    }
    console.log(currentAxiom);
    console.log('string', performance.now() - start);

    const systems = [
      deterministic,
      stochastic,
      contextSensitive,
      parametric,
      treeModel,
      queryParameters,
      {
        productions: ['ab<c>def -> a', 'a(b,c)>?P(x,y)->a']
      }
    ];
    for (const s of systems) {
      for (const p of s.productions) {
        LSystem.parseProductionDefinition(p);
      }
    }
    console.log('test', 'x+Math.cos(y)/34.5f'.matchAll(validVariableNameRegex));
}
