"use strict";

const foo =
    `B(2)A(4,4)
     A(x,y): y<=3 -> A(x*2,x+y)
     A(x,y): y> 3 -> B(x)A(x/y,0)
     B(x)  : x< 1 -> C
     B(x)  : x>=1 -> B(x-1)`;

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

class Condition {
    constructor(definition) {
        this.definition = definition;
    }
}

class Result {
    constructor(definition) {
        this.definition = definition;
    }
}

class Production {
    constructor(definition) {
        let parts = definition.replace(/\s/g, '').split(':');
        this.symbol = new Symbol(parts[0]);
        this.condition = new Condition(parts[1].split('->')[0]);
        this.result = new Result(parts[1].split('->')[1]);
    }
    toString() {
        return `${this.symbol}: ${this.condition}$ -> ${this.result}`;
    }
}

function test() {
    const lines = foo.split('\n');
    const axiom = lines[0];
    const productions = lines.slice(1);
    console.log('lines', lines);
    console.log('axiom', axiom);
    console.log('productions', productions);
    console.log('productions', productions.map(p => new Production(p)));
}
