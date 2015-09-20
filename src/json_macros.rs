/// Retrieve a value of type `serde_json::Value::$Var` from the given `$value: serde_json::Value`
/// located at `$path$.
///
/// Returns `Result<_, &str>`.
macro_rules! json_get {($Var:ident, $value:ident $(.$path:ident)+) => (
    $value.find_path(&[$(stringify!($path)),*])
          .ok_or(concat!("no such value: ", stringify!($value), $(".", stringify!($path)),*))
          .and_then(|val| match val {
              &::json::Value::$Var(ref var) => ::std::result::Result::Ok(var),
              _ => Err(concat!("incorrect value type, expected: ", stringify!($Var))),
          });
)}
