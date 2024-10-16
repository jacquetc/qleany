from dataclasses import dataclass, field
from typing import List

@dataclass(slots=True)
class RootDto:
    id_: int = 0
    # global_: int
    entities: list[int] = field(default_factory= lambda: [])  
    features: list[int] = field(default_factory= lambda: [])  

@dataclass(slots=True)
class CreateRootsDto:
    entities: List[RootDto] = field(default_factory= lambda: [])  

    def from_dtos(self, dtos: List[RootDto]):
        return CreateRootsDto(
            entities=dtos
        )