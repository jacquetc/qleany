from dataclasses import dataclass, field
from typing import List


@dataclass(slots=True)
class FieldDto:
    id_: int = 0
    name: str = ""
    type_: str = ""
    entity: int | None = None
    is_nullable: bool = False
    is_primary_key: bool = False
    is_list: bool = False
    is_single: bool = False
    strong: bool = False
    ordered: bool = False
    list_model: bool = False
    list_model_displayed_field: str = ""


@dataclass(slots=True)
class CreateFieldsDto:
    entities: List[FieldDto] = field(default_factory=lambda: [])
    owner_id: int = 0
    position: int = -1

    def from_dtos(self, dtos: List[FieldDto]):
        return CreateFieldsDto(
            entities=dtos, owner_id=self.owner_id, position=self.position
        )
