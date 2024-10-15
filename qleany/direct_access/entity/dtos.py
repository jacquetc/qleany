from dataclasses import dataclass
from typing import List

@dataclass(slots=True)
class EntityDto:
    id_: int
    only_for_heritage: bool
    fields: list[int]

@dataclass(slots=True)
class CreateEntitiesDto:
    entities: List[EntityDto]
    owner_id: int = 0
    position: int = -1

    def from_dtos(self, dtos: List[EntityDto]):
        return CreateEntitiesDto(
            entities=dtos,
            owner_id=self.owner_id,
            position=self.position
        )