# Evaluator

**Source:** `src/evaluator/mod.rs` *(not yet implemented)*
**Entry point:** `evaluator::eval(program: typed_ast::TypedProgram) -> Result<Value, YoloscriptError>`

The evaluator is a tree-walking interpreter over the typed AST. See `docs/04-TASKS/epic-002-evaluator/` for the planned implementation work.

## Planned Design

### Runtime Values

```rust
enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Unit,
    Tuple(Vec<Value>),
    Array(Rc<RefCell<Vec<Value>>>),
    Struct { name: String, fields: HashMap<String, Value> },
    Enum   { name: String, variant: String, fields: HashMap<String, Value> },
    Function(FunctionValue),
    Closure(ClosureValue),
    Perhaps(Option<Box<Value>>),
    Result(std::result::Result<Box<Value>, Box<Value>>),
}
```

### Environment

A scope stack of `HashMap<String, Rc<RefCell<Value>>>`. `Rc<RefCell<T>>` makes mutable closure capture work naturally — closures share the same `Rc` as the outer scope.

### Control Flow Signals

`break`, `continue`, `return`, and `?` are handled via a signal enum returned alongside values rather than Rust exceptions:

```rust
enum Signal {
    Value(Value),
    Return(Value),
    Break(Value),       // carries the break value for `loop { break expr; }`
    Continue,
    PropagateErr(Value),
}
```

Each evaluation function returns `Result<Signal, RuntimeError>`. Signals propagate up the call stack until consumed by the appropriate construct.

### Built-ins

Built-in functions (`print`, `println`, `array_push`, `array_len`, etc.) are registered in the root environment before evaluation begins, implemented as a `Value::Builtin(fn(...) -> Value)` variant.
