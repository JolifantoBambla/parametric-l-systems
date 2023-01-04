# User Interface

## Input File Editor

Allows users to upload, edit, and save input files for this tool.
Additionally, the buttons `Test` and `Render` allow users to run test iterations of the L-systems, or render the Scene defined in the current
input file (see [Input File Format](#input-file-format)) respectively.
The editor is initialized with a default input file.

## Test Output

Shows the output of test iterations run for L-systems defined in the current input file.
By default, 3 iterations are run for each instance of each L-system (see [L-systems](#l-systems) and [Instaces](#instances)).

## Viewer

A 3D rendering of the scene defined in the last input file passed to the renderer via clicking the `Render` button.
The camera orbits around the center of the scene and can be controlled via the mouse: to change the camera's orientation click the left mouse button and move the mouse.
The mouse wheel controls the camera's zoom level.
The active iteration of each L-system instance in the scene can be controlled by a corresponding slider in the user interface.
Requires WebGPU to be supported by the browser.

## Documentation

Shows the tool's documentation (this text).

# Input file format

L-systems and Scenes are defined in JSON files.
On the top level they define three properties:
- **L-systems**: a collection of named L-system definitions.
- **Scene**: a definition of a 3D scene to render in the viewer tab. This property is only required for rendering the scene, i.e., clicking the `Test` button works even if this property is missing.
- **Resources**: a collection of resources, e.g., external meshes. This property is completely optional if no external resources are used.

Example:
```json
{
  "lSystems": { ... },
  "scene": { ... },    // only required for rendering L-systems
  "resources": { ... } // optional
}
```

## Common properties

### Transform
Several objects in an input file may specify a `"transform"` property that defines their relation to their embedding space.
A transform is given as an array of 12 floats, the elements of a 4x4 column-major matrix.
All transform properties are optional and default to the identity.

### Material

TODO

## L-Systems

Each L-system in the file is uniquely defined by its name which is used as a key in the `"lSystems"` object of the input file.
In the following example, a single L-system with the unique name `tree` is defined:
```json
{
  "lSystems": {
    "tree": { ... } // contains the L-system's definition
  },
  ...
}
```

Each L-system specifies the following properties:
- **Definition**: The actual definition of the L-system consisting of an alphabet, a set of productions, a collection of parameters, and an axiom.
- **Instances**: A collection of instances of this L-system specifying a number of iterations as well as parameter overrides to use when evaluating the L-system.
- **Transform**: A 4x4 column major matrix defining the transformations to apply to transform the object(s) created by a 3D turtle to the L-system's space (see [3D turtle's orientation](#orientation)).
- **Primitives**: A collection of primitives the 3D turtle may use during the interpretation of an evaluated L-system instance. Each primitive must map to a resource in the input file's `"resources"` property.

```json
{ 
  "definition": { ... },
  "instances": { ... },
  "transform": [ ... ],  // optional
  "primitives": [ ... ]  // optional; if primitives are defined, they must name a resurce in the input file's resources property
}
```

### Definition

The actual L-system consisting of the following properties:
- **Alphabet**: A collection of modules that may occur in either the axiom or a production. Each module is uniquely defined by a name and an implicit number of parameters (e.g., `Foo(a,b,c)` and `Foo(x,y,z)` are equivalent in the context of the L-system).
- **Parameters**: A collection of global, immutable parameters that may occur in either the axiom or a production. Parameter names must be valid identifiers in the JavaScript language and their values must be numbers (they are coerced to the JavaScript `Number` type). Parameter values may be overriden by an [instance](#instances) before they are evaluated.
- **Productions**: A collection of productions to transform modules in the L-systems axiom. Productions may only use modules defined in the L-system's alphabet and parameters defined by the module they replace, the module's environment, i.e., predecessors or successors of the module they replace, or by the L-system itself.
- **Axiom**: An initial string of modules that must occur in the L-system's alphabet. Parameters of the axiom's modules must be either numbers or name one of the L-system's parameters.

The following example shows an L-system with a single module in its alphabet: `A(x,y)`. Its name is `A` and its number of parameters is 2.
The L-system's single parameter `b` defaults to the value `23.5`.
The L-system has four productions that all replace an instance of the module `A(x,y)`.
The first two productions do not require the module to appear in a specific environment, and they have the same condition, `x < y`, as well as the same probability of being selected, `0.5`.
While the first one specifies this probability explicitly, the second production's probability is implicitly set to `(1 - sum_explicit_p) / num_implicit_p) = (1 - 0.5) / 1 = 0.5`, where `sum_explicit_p` is the sum of all probabilities explicitly given for all productions with the same condition and environment, and `num_implicit_p` is the number of productions not specifying a probability with the same condition and environment.
The third production replaces an instance of `A(x,y)` if there are other instances of `A(x,y)` before and after it, and the condition `x0 + y0 < x1 + x2 && x2 + y2 < b`, where `x0`, `x1`, `x2`, `y0`, `y1`, `y2` are the parameters of the three `A(x,y)` instances, and `b` is the L-system's global parameter `b`, evaluates to `true`.
The fourth production simply replaces an instance of `A(x,y)` if there are other instances of `A(x,y)` before and after it without any further condition.
For more detailed information see the [L-system syntax specification](#l-system-syntax).

```json
{ 
  "definition": {
    "alphabet": [ "A(x,y)" ],
    "parameters": {
      "b": 23.5
    },
    "productions": [
      "0.5; A(x,y): x < y -> A(x*2, Math.sqrt(y))",
      "A(x,y): x < y -> A(x / 2, b)",
      "A(x0,y0) < A(x1,y1) > A(x2,y2): x0 + y0 < x1 + x2 && x2 + y2 < b -> A(1,2)",
      "A(x0,y0) < A(x1,y1) > A(x2,y2) -> A(2,1)"
    ],
    "axiom": "A(100,b)A(100,b)A(100,b)"
  },
  ...
}
```

### Instances

L-system instances are used to evaluate an L-system. They must specify an integer number of iterations and may define the following properties:
- **Parameters**: A collection of overrides of the L-system's parameters.
- **Materials**: A collection of materials to use when interpreting this L-system instance (see [3D turtle's materials](#materials)).
- **Start material**: An index into the instance's collection of materials. Defaults to 0. This only has an effect if materials are defined.
- **Test iterations**: By default, only 3 iterations are evaluated for each instance during testing. If `"unlimitedTestIterations"` is explicitly set to `true`, the instance's number of iterations specified by its `"iterations"` property are evaluated instead.

The following two examples are equivalent:

```json
{
  ...,
  "instances": {
    "iterations": 10,
    "parameters": {},                 // optional; overrides the system's parameters
    "materials": [ ... ],             // optional
    "startMaterial": 0,               // optional; only has an effect if materials are defined
    "unlimitedTestIterations": false  // optional; defaults to false
  },
  ...
}
```
```json
{
  ...,
  "instances": {
    "iterations": 10
  },
  ...
}
```

### Primitives

TODO: implement and document

```json
{ 
  ...,
  "primitives": [ ... ]       // optional; if primitives are defined, they must name a resurce in the input file's resources property
}
```

## Scene

The input file's `"scene"` property defines a 3D scene to render 

```json
{
  ...
  "scene": {
    "camera": {},
    "lights": {},
    "objects": {}
  },
  ...
}
```

### Camera

### Lights

### Objects

## Resources

### Wavefront OBJ

# Turtle Graphics

L-systems are interpreted by a 3D turtle graphics system.
The turtle records transform matri

## Orientation
The 3d turtle graphics implementation uses a right-handed coordinate system with the head axis being `(0,0,-1)` and the up axis being `(0,1,0)`.
The L-system's transform is used to transform

## Materials

## Turtle state

## Commands

| Command                      | Description                                                                                                  |
|------------------------------|--------------------------------------------------------------------------------------------------------------|
| `F(l=1,w=DEFAULT_WIDTH)`     | Moves the turtle forward, i.e., along its head axis, by `l` and draws a linesegment with diameter `w`.       |
| `f(l=1)`                     | Moves the turtle forward, i.e., along its head axis, by `l`.                                                 |
| `!(w)`                       | Sets the turtle's default diameter for line segments (`DEFAULT_WIDTH`) to `w`. The paramter `w` is required. |
| `+(a=90)`                    | Rotates the turtle clockwise around its up axis by `a` degrees (yaw).                                        |
| `-(a=90)`                    | Rotates the turtle counterclockwise around its up axis by `a` degrees (yaw).                                 |
| `&(a=90)`                    | Rotates the turtle clockwise around its right axis by `a` degrees (pitch).                                   |
| `^(a=90)`                    | Rotates the turtle counterclockwise around its right axis by `a` degrees (pitch).                            |
| `/(a=90)`                    | Rotates the turtle clockwise around its head axis by `a` degrees (roll).                                     |
| `\(a=90)`                    | Rotates the turtle counterclockwise around its head axis by `a` degrees (roll).                              |
| `&vert;`                     | Rotates the turtle around its up axis by 180 degrees (yaw).                                                  |
| `[`                          | Pushes the turtle's current state onto a stack.                                                              |
| `]`                          | Pops the turtle's last state from a stack.                                                                   |
| `%`                          | Ignores all further commands until the turtle's last state is retrieved from the stack.                      |
| `&grave;(i=MATERIAL_ID + 1)` | Sets the turtle's material index to `i`, or the maximum material index if `i` is larger than the maximum material index. |

# L-System Syntax

TODO: describe how productions are chosen (rank!)
During evaluation, for each module in a string of modules, exactly one production from all productions


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
