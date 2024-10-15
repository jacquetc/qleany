from dataclasses import dataclass, field
from typing import List

@dataclass(slots=True)
class FeatureDto:
    id_: int
    name: str
    description: str
    # use_cases: list[int] = field(default_factory= lambda: [])  

@dataclass(slots=True)
class CreateFeaturesDto:
    entities: List[FeatureDto] = field(default_factory= lambda: [])  
    owner_id: int = 0
    position: int = -1

    def from_dtos(self, dtos: List[FeatureDto]):
        return CreateFeaturesDto(
            entities=dtos,
            owner_id=self.owner_id,
            position=self.position
        )