"""
Pydantic model serialization utilities with ormsgpack integration.

This module provides utilities for creating serializable wrappers around custom types,
particularly for use with Pydantic models and ormsgpack serialization.
"""

from typing import Annotated, Type, TypeVar
from py03_pydantic_ormsgpack_experiment import (
    Person,
    create_nested_person,
)
from pydantic import BaseModel, PlainSerializer, PlainValidator
import ormsgpack


PersonWrapper = Annotated[
    Person,
    PlainValidator(Person.validate),
    PlainSerializer(lambda v: v.__dict__, return_type=dict),
]
"""Type alias for Person with serialization capabilities"""


class MySubModel(BaseModel):
    """
    A Pydantic model containing a PersonWrapper instance.

    Attributes:
        person: A Person object with validation and serialization capabilities
    """

    person: PersonWrapper


class MyModel(BaseModel):
    """
    The top-level Pydantic model for demonstrating nested serialization.

    Attributes:
        sub: A MySubModel instance
    """

    sub: MySubModel


def main():
    """
    Demonstrate serialization and deserialization with Pydantic and ormsgpack.

    Creates a nested Person object, wraps it in Pydantic models, then:
    1. Dumps the model to a dictionary
    2. Serializes with ormsgpack
    3. Deserializes back to Python objects
    4. Reconstructs the original Pydantic model

    Prints details about the objects and their types at each step.
    """

    nested_person: Person = create_nested_person(depth=3, max_children=2)

    nested_person.age = 10

    print(nested_person.__dict__)

    assert isinstance(nested_person, Person)

    my_model = MyModel(sub=MySubModel(person=nested_person))

    dump = my_model.model_dump()
    print(f"dump={dump}")
    assert isinstance(dump, dict)
    assert isinstance(dump["sub"], dict)
    assert isinstance(dump["sub"]["person"], dict)
    assert isinstance(dump["sub"]["person"]["children"], list)
    assert isinstance(dump["sub"]["person"]["children"][0], dict)

    packed = ormsgpack.packb(dump)
    assert isinstance(packed, bytes)

    unpacked: dict = ormsgpack.unpackb(packed)
    assert isinstance(unpacked, dict)

    parsed = MyModel(**unpacked)
    print(f"parsed={parsed}")

    assert parsed == my_model

    assert isinstance(parsed.sub, MySubModel)
    assert isinstance(parsed.sub.person, Person)
    assert isinstance(parsed.sub.person.children, list)
    assert isinstance(parsed.sub.person.children[0], Person)
