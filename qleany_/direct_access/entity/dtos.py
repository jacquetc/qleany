from dataclasses import dataclass, field


@dataclass(slots=True)
class EntityDto:
    id_: int = 0
    name: str = ""
    only_for_heritage: bool = False
    fields: list[int] = field(default_factory=lambda: [])


@dataclass(slots=True)
class CreateEntitiesDto:
    entities: list[EntityDto] = field(default_factory=lambda: [])
    owner_id: int = 0
    position: int = -1

    def from_dtos(self, dtos: list[EntityDto]):
        return CreateEntitiesDto(
            entities=dtos, owner_id=self.owner_id, position=self.position
        )
