from dataclasses import dataclass

@dataclass(slots=True)
class EntityDto:
    id_: int
    only_for_heritage: bool
    fields: list[int]

@dataclass(slots=True)
class CreateEntityDto:
    only_for_heritage: bool
    fields: list[int]
