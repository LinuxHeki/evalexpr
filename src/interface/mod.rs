use Configuration;
use EmptyConfiguration;
use Error;
use FloatType;
use IntType;
use Node;
use token;
use tree;
use Value;
use value::TupleType;

/// Evaluate the given expression string.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval(string: &str) -> Result<Value, Error> {
    eval_with_configuration(string, &EmptyConfiguration)
}

/// Evaluate the given expression string with the given configuration.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut configuration = HashMapConfiguration::new();
/// configuration.insert_variable("one", 1);
/// configuration.insert_variable("two", 2);
/// configuration.insert_variable("three", 3);
/// assert_eq!(eval_with_configuration("one + two + three", &configuration), Ok(Value::from(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<Value, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval_with_configuration(configuration)
}

/// Build the operator tree for the given expression string.
///
/// The operator tree can later on be evaluated directly.
/// This saves runtime if a single expression should be evaluated multiple times, for example with differing configurations.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let precomputed = build_operator_tree("one + two + three").unwrap(); // Do proper error handling here
///
/// let mut configuration = HashMapConfiguration::new();
/// configuration.insert_variable("one", 1);
/// configuration.insert_variable("two", 2);
/// configuration.insert_variable("three", 3);
///
/// assert_eq!(precomputed.eval_with_configuration(&configuration), Ok(Value::from(6)));
///
/// configuration.insert_variable("three", 5);
/// assert_eq!(precomputed.eval_with_configuration(&configuration), Ok(Value::from(8)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn build_operator_tree(string: &str) -> Result<Node, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)
}

/// Evaluate the given expression string into a string.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string(string: &str) -> Result<String, Error> {
    match eval(string) {
        Ok(Value::String(string)) => Ok(string),
        Ok(value) => Err(Error::expected_string(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an integer.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int(string: &str) -> Result<IntType, Error> {
    match eval(string) {
        Ok(Value::Int(int)) => Ok(int),
        Ok(value) => Err(Error::expected_int(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float(string: &str) -> Result<FloatType, Error> {
    match eval(string) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(value) => Err(Error::expected_float(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a boolean.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean(string: &str) -> Result<bool, Error> {
    match eval(string) {
        Ok(Value::Boolean(boolean)) => Ok(boolean),
        Ok(value) => Err(Error::expected_boolean(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a tuple.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple(string: &str) -> Result<TupleType, Error> {
    match eval(string) {
        Ok(Value::Tuple(tuple)) => Ok(tuple),
        Ok(value) => Err(Error::expected_tuple(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a string with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<String, Error> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::String(string)) => Ok(string),
        Ok(value) => Err(Error::expected_string(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an integer with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<IntType, Error> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Int(int)) => Ok(int),
        Ok(value) => Err(Error::expected_int(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<FloatType, Error> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(value) => Err(Error::expected_float(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a boolean with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<bool, Error> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Boolean(boolean)) => Ok(boolean),
        Ok(value) => Err(Error::expected_boolean(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a tuple with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<TupleType, Error> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Tuple(tuple)) => Ok(tuple),
        Ok(value) => Err(Error::expected_tuple(value)),
        Err(error) => Err(error),
    }
}
