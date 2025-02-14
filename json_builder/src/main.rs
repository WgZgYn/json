use indexmap::IndexMap;

type JsonMap = IndexMap<String, Value>;

#[derive(Clone, Debug)]
enum Value {
    JsonObject(JsonMap),
    JsonArray(Vec<Value>),
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}

struct ObjectBuilder {
    value: JsonMap,
}

impl ObjectBuilder {
    fn new() -> ObjectBuilder {
        ObjectBuilder {
            value: JsonMap::new(),
        }
    }

    fn add(mut self, key: &str, value: impl IntoValue) -> Self {
        self.value.insert(key.to_owned(), value.into_value());
        self
    }

    fn build(self) -> Value {
        Value::JsonObject(self.value)
    }
}

struct ArrayBuilder {
    value: Vec<Value>,
}

impl ArrayBuilder {
    fn new() -> ArrayBuilder {
        ArrayBuilder { value: Vec::new() }
    }

    fn add(mut self, value: impl IntoValue) -> Self {
        self.value.push(value.into_value());
        self
    }

    fn build(self) -> Value {
        Value::JsonArray(self.value)
    }
}

impl Value {
    fn object() -> ObjectBuilder {
        ObjectBuilder::new()
    }
    fn array() -> ArrayBuilder {
        ArrayBuilder::new()
    }
    fn null() -> Value {
        Value::Null
    }
}

trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for ObjectBuilder {
    fn into_value(self) -> Value {
        self.build()
    }
}

impl IntoValue for ArrayBuilder {
    fn into_value(self) -> Value {
        self.build()
    }
}

impl IntoValue for Vec<Value> {
    fn into_value(self) -> Value {
        Value::JsonArray(self)
    }
}

impl IntoValue for JsonMap {
    fn into_value(self) -> Value {
        Value::JsonObject(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Number(self)
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Boolean(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_string())
    }
}

fn main() {
    let v2 = Value::array()
        .add(Value::Number(0.0))
        .add(
            Value::object()
                .add("1", "1")
                .add("2", "2")
                .add("into_value", 5.)
                .build(),
        )
        .build();

    let v3 = Value::object()
        .add("Null", Value::null())
        .add("content", "ok")
        .add("array", v2)
        .add(
            "object",
            Value::object()
                .add("Hello", 1.23)
                .add("World!", true)
                .build(),
        )
        .build();

    println!("{:#?}", v3);
}
