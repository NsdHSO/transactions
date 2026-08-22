fn main() {
    // let mut buffer = String::new();
    // let stdin = stdin();

    let add_op = Operation::new("+".to_string(), Add);
    let multiply = Operation::new("*".to_string(), Multiply);
    println!("3 {} 5 = {}", add_op.symbol, add_op.evaluate(3.0, 5.0));
    println!("3 {} 5 = {}", multiply.symbol, multiply.evaluate(2.0, 5.0));
    let op_2 = Operator::Addition {
        lhs: Value(23.0),
        rhs: Value(2.0),
    };

    println!("Operation 2 > : 23.0 + 2.0 = {}", op_2.apply().evaluate())
    // let operation: Vec<Operation<_>> = vec![add_op, multiply];
}
// Traits  look remarkably like Java Interface/abstract classes,
// They define method signtures that type must implement.
// Drop trait provide cleanup functionality that seems equivalent to destructors.
//
//
// Structs are not classes are data Rust Data Structure
//
// You can't extend or modify their definition by creating children of the struct.
// A Rust struct is what it is.
//
// A traits are much more accuarately thought of as contracts.
//
// Polymorphism is build into the essence of classes in most modern OO Language. There is an
// inherent expectation that child classes are interchangeable with their parents and siblings.
//
// The Borrow checker prevents object patterns
// Borrow checker makes us think much more carfully about the movement and sharing of data
//
// Pulling it together: types are not object
//
// We may not call structs and enums objects, but they are collections of data.
// They allow collections of data and their 'impl' blocks allow us to group specific code with
// specific structures.21
//
// Misusing traits
// Traits exist to provide contracts that structs can fulfil, which adds expressiveness and
// flexibility to the language.
//
// We Can't have a collection of different Operation< T > types.
// Enums
//
// - carry and encapsulate their own data
// - allow many subtypes under umbrella of one supertype
// - make it simple to select behavior based on their type
// - Enums Have only ONE impl block
// - Each Variant is an independent type.
//

trait OperationType {
    fn calulate(&self, left: f64, right: f64) -> f64;
    fn precedence(&self) -> u8;
}

struct Add;
impl OperationType for Add {
    #[allow(dead_code)]
    fn calulate(&self, left: f64, right: f64) -> f64 {
        left + right
    }

    #[allow(dead_code)]
    fn precedence(&self) -> u8 {
        1
    }
}

struct Multiply;
impl OperationType for Multiply {
    fn calulate(&self, left: f64, right: f64) -> f64 {
        left * right
    }

    fn precedence(&self) -> u8 {
        2
    }
}
struct Operation<T: OperationType> {
    symbol: String,
    op_type: T,
}

impl<T: OperationType> Operation<T> {
    fn new(symbol: String, op_type: T) -> Self {
        Operation { symbol, op_type }
    }

    fn evaluate(&self, left: f64, right: f64) -> f64 {
        self.op_type.calulate(left, right)
    }

    #[allow(dead_code)]
    fn get_precedence(&self) -> u8 {
        self.op_type.precedence()
    }
}

struct Value(f64);

impl Value {
    fn evaluate(&self) -> f64 {
        self.0
    }
}

enum Operator {
    Addition { lhs: Value, rhs: Value },
    Substraction { lhs: Value, rhs: Value },
}

impl Operator {
    fn apply(&self) -> Value {
        let inner = match self {
            Operator::Addition { lhs, rhs } => lhs.evaluate() + rhs.evaluate(),
            Operator::Substraction { lhs, rhs } => lhs.evaluate() - rhs.evaluate(),
        };

        Value(inner)
    }

    fn symbol(&self) -> char {
        match self {
            Operator::Addition { .. } => '+',
            Operator::Substraction { .. } => '-',
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Addition { .. } | Operator::Substraction { .. } => 0,
        }
    }
}
