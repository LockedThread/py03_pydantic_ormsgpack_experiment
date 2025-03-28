
# PyO3-Pydantic-ormsgpack Integration Example

This project demonstrates how to create a seamless integration between Rust and Python, combining the performance benefits of Rust with Python's rich data validation ecosystem. Specifically, it shows how to expose Rust structs to Python via PyO3 and make them fully compatible with Pydantic models and ormsgpack serialization.

## Features

- 🦀 Rust data structures exposed to Python via PyO3
- 🔍 Pydantic validation and type checking with Rust struct references
- 📦 Efficient serialization with ormsgpack
- 🔄 Complete round-trip serialization (Rust → Python → bytes → Python → Rust)

## How It Works

This example demonstrates a powerful pattern for Python/Rust interoperability:

1. **Rust Side (PyO3)**:
   - Defines a `Person` struct with attributes and methods
   - Implements a `Wrapper` trait providing dictionary conversion functionality
   - Exposes the class and utility functions to Python

2. **Python Side (Pydantic)**:
   - Creates a `PersonWrapper` annotated type that combines:
     - The Rust `Person` class as the base type
     - [`PlainValidator`](https://docs.pydantic.dev/latest/concepts/validators/#field-plain-validator) using the `Person.validate` method
     - [`PlainSerializer`](https://docs.pydantic.dev/latest/api/functional_serializers/#pydantic.functional_serializers.PlainSerializer) using the `__dict__` method
   - Wraps the type in Pydantic models for validation

3. **Serialization Flow**:
   - Rust objects are created and passed to Python
   - Pydantic models validate and wrap these objects
   - ormsgpack serializes to binary format
   - Deserialization reconstructs the complete object hierarchy
   - The nested Person struct is fully preserved, and equal to the original struct

## Installation

```bash
# Clone the repository
git clone git@github.com:LockedThread/py03_pydantic_ormsgpack_experiment.git
cd py03_pydantic_ormsgpack_experiment
# Install Python dependencies
pip install pydantic ormsgpack

cargo run
```

## Usage Example

```python
from pydantic import BaseModel, PlainSerializer, PlainValidator
from typing import Annotated
import ormsgpack
from py03_pydantic_ormsgpack_experiment import Person, create_nested_person

# Create a type annotation for Person with serialization capabilities
PersonWrapper = Annotated[
    Person,
    PlainValidator(Person.validate),
    PlainSerializer(lambda v: v.__dict__, return_type=dict),
]

# Use the wrapper in a Pydantic model
class UserModel(BaseModel):
    person: PersonWrapper

# Create a Person instance from Rust
person = create_nested_person(depth=2, max_children=3)

# Validate using Pydantic
user = UserModel(person=person)

# Serialize to bytes with ormsgpack
serialized = ormsgpack.packb(user.model_dump())

# Deserialize back to a valid model
deserialized = ormsgpack.unpackb(serialized)
restored_user = UserModel(**deserialized)

# The nested Person structure is fully preserved!
assert isinstance(restored_user.person, Person)
assert isinstance(restored_user.person.children[0], Person)
```

## Technical Details

### Rust-Python Bridge

The key to making this work is the `Wrapper` trait in Rust:

```rust
pub trait Wrapper {
    fn to_dict_with_py<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>>;
    fn to_dict(&self) -> PyResult<Py<PyDict>>;
    fn from_dict(dict: &Bound<'_, PyDict>) -> PyResult<Self> where Self: Sized;
    fn validate(value: &Bound<'_, PyAny>) -> PyResult<Self> where Self: Sized;
}
```

This trait enables bidirectional conversion between Rust structs and Python dictionaries, which is then exposed to Python via PyO3's [`#[pymethods]`](https://pyo3.rs/main/doc/pyo3/attr.pymethods).

### Pydantic Integration

On the Python side, we use Pydantic's [`Annotated`](https://docs.python.org/3/library/typing.html#typing.Annotated) types with:

1. [`PlainValidator`](https://docs.pydantic.dev/latest/concepts/validators/#field-plain-validator) - Converts from Python objects to the Rust type
2. [`PlainSerializer`](https://docs.pydantic.dev/latest/api/functional_serializers/#pydantic.functional_serializers.PlainSerializer) - Converts from the Rust type to Python dictionaries

This enables seamless integration with Pydantic's validation system while preserving the type information.

## Use Cases

This pattern is particularly useful for:

- High-performance data processing pipelines
- Applications needing both speed and type safety
- APIs that handle complex nested data structures
- Projects transitioning from Python to Rust incrementally

## Benefits

- **Type Safety**: Ensure data consistency with Pydantic's validation
- **Serialization**: Efficiently transmit data with ormsgpack's compact format
- **Developer Experience**: Maintain Python's ease of use while getting Rust's benefits

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
