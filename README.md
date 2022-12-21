# parametric-l-systems
An implementation of parametric L-systems for the Fractals course at TU Wien

Check it out [TODO]().

# Build

TODO: install rust & wasm-pack

```bash
wasm-pack build --target web
```

# Run
TODO: Install and configure WebGPU capable browser

Open `index.html` in a browser that supports WebGPU.

# Scene file format

```
{
  "systems": { ... },
  "scene": { ... }
}
```

```
{
  "definition": {
    "alphabet": [ ... ]
  }
}
```


# Specification

## L-system
An L-system consists of a list of modules, a list of productions, a list of parameters, an axiom, and an interpreter.
The axiom (see [Axiom](#axiom)) is a list of evaluated modules which are transformed by the L-system's productions (see [Production](#production)) to generate a new list of evaluated modules.
If no production can be applied to a module, the identity production is applied, i.e. the module is simply copied to the output list.
If the generated list of evaluated modules contains query modules (see [Module](#module)), the L-system's interpreter is used to evaluate the states of all query modules after the productions have been applied. 

## Interpreter
An L-system's interpreter interprets lists of evaluated modules and tracks a system's state consisting of one or more named state variables.
These state variables are used to replace the variables of query modules of a list of evaluated modules produced by processing an axiom using the L-system's production.

## Parameter
A parameter is uniquely defined by its name, which must be a valid variable name in the JavaScript language.

## Module
A module is uniquely defined by a symbol and a number of arguments.
Symbols are named using single-character names.
The exception are query modules which are prefixed by a `?` (question mark) and must have at least one argument.
Arguments of a module are specified in a comma-separated list within parentheses (inbetween `(` and `)`).
If a module has no arguments the parentheses are optional.
The argument names of non-query modules hold no meaning in the context of modules but only in the context of productions (see [Production](#production)).
The argument names of query modules must name variables within the L-system's state.
E.g.:
```
// Since none of the following modules have the same number of arguments, they may all exist in the same L-system:

// A symbol with no arguments:
A

// ... it is equivalent to
A()

// A symbol with two arguments:
A(x,y)

// ... it is equivalent to
A(foo,bar)

// A query module for querying the variable x from the L-system's state:
?A(x)
```

## Axiom
An axiom is a list of evaluated modules, i.e. symbols with explicit values for their parameters.
E.g.:
```
// An axiom consisting of one module A with the value 1 as its single parameter and a module B with values 2 and 3 as its to parameters:
A(1)B(2,3)
```

## Production
A production specifies a rule for transforming a single evaluated module (see [Axiom](#axiom)) in an axiom to zero or more evaluated modules.
A production consists of four parts:
1. A probability for the production to be applied.
2. A specification of the module that is transformed by the production.
3. A condition that has to be fulfilled for the production to be applied.
4. A list of module forms generating evaluated modules.

These four parts are separated by the keywords `;`, `:`, and `->` in that order, i.e.:
`<probability> ; <module specification> : <condition> -> <module form list>`
White spaces hold no meaning and may or may not be inserted for readability.

### Probability
The probability of a production including the separating keyword `;` is optional and follows a uniform distribution of all productions with the same module specification.
Probabilties must be positive floating point numbers less than or equal to 1.
The sum of all probabilities of productions with the same module specification within an L-system must be less than or equal to 1.
If the sum of all probabilities of productions that can be applied to an evaluated module is less than 1, they are rescaled to the range `[0,1]` before choosing a production.
This may happen at run-time if two or more productions have the same module specification and different conditions, which are all satisfied.
E.g.:
```
// Given that there is only one production with the unique module specification <module specification A>, the folllowing three ways of specifying probabilities are all equivalent:
<module specification A> : ...
; <module specification A> : ...
1.0; <module specification A> : ...

// explicit probabilities for two productions for the unique module specification <module specification B>
0.45; <module specification B> : ...
0.55; <module specification B> : ...

// implicit probabilities for two productions for the unique module specification <module specification C> of 0.5 each
<module specification C> : ...
<module specification C> : ...
```

### Module Specification
A module specification consists of a module (see [Module](#module)) and optional predecessors and successors separated by the keywords `<` and `>`.
Argument names within a module specification must be unique accross all modules within the specification.
They are defined within the whole scope of the production and may be used within the production's condition and/or its list of module forms.
An argument sharing its name with one of the L-system's parameters shadow this parameter's name in the production's scope.
The argument names of a query module must name variables in the L-system's state.
E.g.: 
```
// The module A with no arguments may be transformed by the production.
A : ... -> ...

// The module A with two arguments may be transformed by the production.
// Its arguments x and y may be used in other parts of the production.
A(x,y) : ... -> ...

// If the module B with no arguments is preceeded by two modules A with no arguments and succeeded by a module C with two arguments, it may be transformed by the production.
// The arguments x and y may be used in other parts of the production.
AA<B>C(x, y) : ... -> ...

// If the module A with two arguments is preceeded by a module B with one argument, it may be transformed by the production.
// All arguments x, y, and z may be used in other parts of the production.
B(x) < A(y,z) : ... -> ...

// The query module A with two arguments x and y may be transformed by the production.
?A(x,y) : ... -> ...
```

### Condition
Specifying a condition for a production optional.
A condition must be empty or a valid JavaScript expression evaluating to a generalized boolean.
If no condition is specified or the specification is empty, the condition is true by default.
A production's condition may use all variables defined in the production's module specification, as well as all  parameters of the L-system.

If an evaluated module satisfies both the module specification and the condition of more than one of an L-system's productions, the rules for probabilities of productions (see [Probability](#probability)) apply.

E.g.:
```
// The following conditions are equivent and default to true:
... -> ...
... : -> ...

// A module A with two arguments may be transformed by the production if its first argument is smaller than its second one.
A(x,y) : x < y -> ...

// A module A with one argument may be transformed by the production if its argument is smaller than the L-system's parameter y.
A(x) : x < y -> ...

// A condition may use all functions defined in the global scope:
A(x) : x < foo(x) -> ...
```

### List of module forms
A production's list of module forms can either be empty or consist of one or multiple module forms.
A module form consists of a symbol name satisfying the rules of modules (see [Module](#module)) and an optional comma-separated list of argument forms.
An argument form must be a valid JavaScript expression evaluating to a value.
All variables defined in the scope of the production, i.e. variables defined by the production's module specification and the L-system's parameters, may be used in argument forms.
The only exception are the argument forms of query modules, which are evaluated by querying the L-system's state after all modules of the axiom have been processed.
E.g.:
```
// The module A with one argument is transformed to a module A with its argument incremented by one.
A(x) -> A(x+1)

// The module A is transformed to a query module B with three parameters x, y, and z.
A -> ?B(x,y,z)

// The module A is transformed to a module B with one argument set to the value of the L-system's paramter x.
A -> B(x)
```

## Examples
```
// axiom
B(2)A(4,4)

// productions
A(x,y): y<=3 -> A(x*2,x+y) 
A(x,y): y>3 -> B(x)A(x/y,0)
B(x) : x<1 -> C
B(x) : x>=1 -> B(x-1)

// Results
B(2)A(4,4)
B(1)B(4)A(1,0)
B(0)B(3)A(2,1)
CB(2)A(4,3)
CB(1)A(8,7)
CB(0)B(8)A(1.142,0)
```
