print("Hello, world! from python")

import random
from py03_pydantic_ormsgpack_experiment import Person, new_person, create_nested_person as create_nested_person_rs, create_random_person as create_random_person_rs
from pydantic import BaseModel
import ormsgpack

class MyModel(BaseModel):
    person: Person

    class Config:
        arbitrary_types_allowed = True
        json_encoders = {
            Person: lambda p: p.to_dict()
        }

def main():
        # Create a deeply nested person structure
    #nested_person = create_nested_person(depth=10, max_children=12)
    nested_person = create_nested_person_rs(depth=7, max_children=12)
    #print(f"Created nested person with name: {nested_person}")
    
    # You can also use it with your model
    my_model = MyModel(person=nested_person)
    #print("my_model", my_model)
    #print("my_model.model_dump()", my_model.model_dump())
    
    # Optionally, print some stats about the nested structure
    def count_descendants(person):
        if not hasattr(person, "children") or not person.children:
            return 0
        count = len(person.children)
        for child in person.children:
            count += count_descendants(child)
        return count
    
    total_descendants = count_descendants(nested_person)
    print(f"Total number of descendants: {total_descendants}")

    
    packed = ormsgpack.packb(my_model.model_dump(), option=ormsgpack.OPT_SERIALIZE_PYDANTIC)
    print("packed", packed)

    unpacked: Person = ormsgpack.unpackb(packed)
    print("unpacked", unpacked)
