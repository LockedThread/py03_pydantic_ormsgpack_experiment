from typing import Dict, List, Any

class Person:
    name: str
    age: int
    children: List["Person"]

    def __dict__(self) -> Dict[str, Any]: ...
    def __repr__(self) -> str: ...
    def add_child(self, child: "Person") -> None: ...
    @staticmethod
    def from_dict(dict_data: Dict[str, Any]) -> "Person": ...
    @staticmethod
    def validate(value: Any) -> "Person": ...

def new_person(name: str, age: int) -> Person: ...
def create_random_person() -> Person: ...
def create_nested_person(depth: int, max_children: int) -> Person: ...
