use crate::pipeline::{PiperError, Value, ValueType};

use super::Operator;

#[derive(Clone, Debug)]
pub struct PlusOperator;

impl Operator for PlusOperator {
    fn get_output_type(&self, argument_types: &[ValueType]) -> Result<ValueType, PiperError> {
        if argument_types.len() != 2 {
            return Err(PiperError::ArityError(
                "+".to_string(),
                argument_types.len(),
            ));
        }
        match argument_types {
            [ValueType::Int, ValueType::Int] => Ok(ValueType::Int),
            [ValueType::Int, ValueType::Long] => Ok(ValueType::Long),
            [ValueType::Int, ValueType::Float] => Ok(ValueType::Double),
            [ValueType::Int, ValueType::Double] => Ok(ValueType::Double),

            [ValueType::Long, ValueType::Int] => Ok(ValueType::Long),
            [ValueType::Long, ValueType::Long] => Ok(ValueType::Long),
            [ValueType::Long, ValueType::Float] => Ok(ValueType::Double),
            [ValueType::Long, ValueType::Double] => Ok(ValueType::Double),

            [ValueType::Float, ValueType::Int] => Ok(ValueType::Double),
            [ValueType::Float, ValueType::Long] => Ok(ValueType::Double),
            [ValueType::Float, ValueType::Float] => Ok(ValueType::Float),
            [ValueType::Float, ValueType::Double] => Ok(ValueType::Double),

            [ValueType::Double, ValueType::Int] => Ok(ValueType::Double),
            [ValueType::Double, ValueType::Long] => Ok(ValueType::Double),
            [ValueType::Double, ValueType::Float] => Ok(ValueType::Double),
            [ValueType::Double, ValueType::Double] => Ok(ValueType::Double),

            [ValueType::String, ValueType::String] => Ok(ValueType::String),

            // All other combinations are invalid
            [a, b] => Err(PiperError::TypeMismatch(
                stringify!($op).to_string(),
                *a,
                *b,
            ))?,

            // Shouldn't reach here
            _ => unreachable!("Unknown error."),
        }
    }

    fn eval(&self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            return Value::Error(PiperError::ArityError("+".to_string(), arguments.len()));
        }

        match arguments.as_slice() {
            // Float + Non-Float always promote to Double
            [Value::Int(a), Value::Int(b)] => (a + b).into(),
            [Value::Int(a), Value::Long(b)] => (a.clone() as i64 + b).into(),
            [Value::Int(a), Value::Float(b)] => (a.clone() as f64 + b.clone() as f64).into(),
            [Value::Int(a), Value::Double(b)] => (a.clone() as f64 + b).into(),

            [Value::Long(a), Value::Int(b)] => (a + b.clone() as i64).into(),
            [Value::Long(a), Value::Long(b)] => (a + b).into(),
            [Value::Long(a), Value::Float(b)] => (a.clone() as f64 + b.clone() as f64).into(),
            [Value::Long(a), Value::Double(b)] => (a.clone() as f64 + b).into(),

            [Value::Float(a), Value::Int(b)] => (a.clone() as f64 + b.clone() as f64).into(),
            [Value::Float(a), Value::Long(b)] => (a.clone() as f64 + b.clone() as f64).into(),
            [Value::Float(a), Value::Float(b)] => (a + b).into(),
            [Value::Float(a), Value::Double(b)] => (a.clone() as f64 + b.clone() as f64).into(),

            [Value::Double(a), Value::Int(b)] => (a + b.clone() as f64).into(),
            [Value::Double(a), Value::Long(b)] => (a + b.clone() as f64).into(),
            [Value::Double(a), Value::Float(b)] => (a + b.clone() as f64).into(),
            [Value::Double(a), Value::Double(b)] => (a + b.clone() as f64).into(),

            // String concat
            [Value::String(a), Value::String(b)] => (format!("{}{}", a, b)).into(),

            // All other combinations are invalid
            [a, b] => Value::Error(PiperError::TypeMismatch(
                "+".to_string(),
                a.value_type(),
                b.value_type(),
            )),

            // Shouldn't reach here
            _ => unreachable!("Unknown error."),
        }
    }

    fn dump(&self, arguments: Vec<String>) -> String {
        format!("({} + {})", arguments[0], arguments[1])
    }
}

macro_rules! binary_math_op {
    ($name:ident, $op_name:tt, $op:tt) => {
        #[derive(Clone, Debug)]
        pub struct $name;

        impl Operator for $name {
            fn get_output_type(&self, argument_types: &[ValueType]) -> Result<ValueType, PiperError> {
                if argument_types.len() != 2 {
                    return Err(PiperError::ArityError("+".to_string(), argument_types.len()));
                }
                match argument_types {
                    [ValueType::Int, ValueType::Int] => Ok(ValueType::Int),
                    [ValueType::Int, ValueType::Long] => Ok(ValueType::Long),
                    [ValueType::Int, ValueType::Float] => Ok(ValueType::Double),
                    [ValueType::Int, ValueType::Double] => Ok(ValueType::Double),

                    [ValueType::Long, ValueType::Int] => Ok(ValueType::Long),
                    [ValueType::Long, ValueType::Long] => Ok(ValueType::Long),
                    [ValueType::Long, ValueType::Float] => Ok(ValueType::Double),
                    [ValueType::Long, ValueType::Double] => Ok(ValueType::Double),

                    [ValueType::Float, ValueType::Int] => Ok(ValueType::Double),
                    [ValueType::Float, ValueType::Long] => Ok(ValueType::Double),
                    [ValueType::Float, ValueType::Float] => Ok(ValueType::Float),
                    [ValueType::Float, ValueType::Double] => Ok(ValueType::Double),

                    [ValueType::Double, ValueType::Int] => Ok(ValueType::Double),
                    [ValueType::Double, ValueType::Long] => Ok(ValueType::Double),
                    [ValueType::Double, ValueType::Float] => Ok(ValueType::Double),
                    [ValueType::Double, ValueType::Double] => Ok(ValueType::Double),

                    // All other combinations are invalid
                    [a, b] => Err(PiperError::TypeMismatch(
                        stringify!($op).to_string(),
                        *a,
                        *b,
                    ))?,

                    // Shouldn't reach here
                    _ => unreachable!("Unknown error."),
                }
            }

            fn eval(&self, arguments: Vec<Value>) -> Value {
                if arguments.len() != 2 {
                    return Value::Error(PiperError::ArityError("+".to_string(), arguments.len()));
                }

                match arguments.as_slice() {
                    [Value::Int(a), Value::Int(b)] => (a $op b).into(),
                    [Value::Int(a), Value::Long(b)] => (a.clone() as i64 $op b).into(),
                    [Value::Int(a), Value::Float(b)] => (a.clone() as f64 $op b.clone() as f64).into(),
                    [Value::Int(a), Value::Double(b)] => (a.clone() as f64 $op b).into(),

                    [Value::Long(a), Value::Int(b)] => (a $op b.clone() as i64).into(),
                    [Value::Long(a), Value::Long(b)] => (a $op b).into(),
                    [Value::Long(a), Value::Float(b)] => (a.clone() as f64 $op b.clone() as f64).into(),
                    [Value::Long(a), Value::Double(b)] => (a.clone() as f64 $op b).into(),

                    [Value::Float(a), Value::Int(b)] => (a.clone() as f64 $op b.clone() as f64).into(),
                    [Value::Float(a), Value::Long(b)] => (a.clone() as f64 $op b.clone() as f64).into(),
                    [Value::Float(a), Value::Float(b)] => (a $op b).into(),
                    [Value::Float(a), Value::Double(b)] => (a.clone() as f64 $op b.clone() as f64).into(),

                    [Value::Double(a), Value::Int(b)] => (a $op b.clone() as f64).into(),
                    [Value::Double(a), Value::Long(b)] => (a $op b.clone() as f64).into(),
                    [Value::Double(a), Value::Float(b)] => (a $op b.clone() as f64).into(),
                    [Value::Double(a), Value::Double(b)] => (a $op b.clone() as f64).into(),

                    // Null + Null = Null
                    [Value::Null, Value::Null] => Value::Null,

                    // All other combinations are invalid
                    [a, b] => Value::Error(PiperError::TypeMismatch(
                        stringify!($op).to_string(),
                        a.value_type(),
                        b.value_type(),
                    )),

                    // Shouldn't reach here
                    _ => unreachable!("Unknown error."),
                }
            }

            fn dump(&self, arguments: Vec<String>) -> String {
                format!("({} {} {})", arguments[0], stringify!($op_name), arguments[1])
            }
        }
    };
}

binary_math_op!(MinusOperator, -, -);
binary_math_op!(MultiplyOperator, *, *);
binary_math_op!(DivideOperator, /, /);
