"use strict";

export const deterministic = {
  axiom: 'a',
  productions: [
    'a->ab',
    'b->ac'
  ],
  parameters: [],
  symbols: ['a', 'b', 'c'],
  results: [
    'ab',
    'abac',
    'abacabc',
    'abacabcabacc'
  ]
}

export const stochastic = {
  axiom: 'F',
  productions: [
    'F->F[+F]F[-F]F',
    'F->F[+F]F',
    'F->F[-F]F'
  ],
  parameters: [],
  symbols: ['F', '[', ']', '+', '-'],
  results: []
}

export const contextSensitive = {
  axiom: 'baaaaaaa',
  productions: [
    'b<a -> b',
    'b->a'
  ],
  parameters: [],
  symbols: ['a', 'b'],
  results: [
    'baaaaaaa',
    'abaaaaaa',
    'aabaaaaa',
    'aaabaaaa',
    'aaaabaaa',
    'aaaaabaa',
    'aaaaaaba'
  ]
}

export const parametric = {
  axiom: 'B(2)A(4,4)',
  productions: [
    'A(x,y): y<=3 -> A(x*2,x+y)',
    'A(x,y): y> 3 -> B(x)A(x/y,0)',
    'B(x)  : x< 1 -> C',
    'B(x)  : x>=1 -> B(x-1)'
  ],
  parameters: [],
  symbols: ['A(x,y)', 'B(x)', 'C'],
  results: [
    'B(1)B(4)A(1,0)',
    'B(0)B(3)A(2,1)',
    'CB(2)A(4,3)',
    'CB(1)A(8,7)',
    'CB(0)B(8)A(1.142,0)'
  ]
}

export const treeModel = {
  axiom: 'A(1,10)',
  productions: [
    'A(l,w) -> F(l,w)[&(a0)B(l*r2,w*wr)]/(d)A(l*r1,w*wr)',
    'B(l,w) -> F(l,w)[+(-a1)$C(l*r2,w*wr)]C(l*r1,w*wr)',
    'C(l,w) -> F(l,w)[+(a1)$B(l*r2,w*wr)]B(l*r1,w*wr)'
  ],
  parameters: ['r1', 'r2', 'a0', 'a1', 'd', 'wr'],
  symbols: ['A(l,w)', 'B(l,w)', 'C(l,w)', 'F', '[', ']', '+', '&', '/', '$'],
  results: []
};

export const queryParameters = {
  axiom: 'A',
  productions: [
    'A->F(1)?P(x,y)-A',
    'F(k)->F(k+1)'
  ],
  parameters: [],
  symbols: ['A', 'F(k)', 'P(x,y)', '-'],
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
