from typing import Sequence

from qleany.common.entities.root import Root
from qleany.direct_access.root.dtos import CreateRootsDto, RootDto
from qleany.direct_access.root.i_root_uow import IRootUow


class CreateUc:
    def __init__(self, unit_of_work: IRootUow):
        self._unit_of_work = unit_of_work

    def execute(self, create_dto: CreateRootsDto) -> Sequence[RootDto]:
        self.validate(create_dto)

        with self._unit_of_work as uow:
            entities_to_create = self._convert_dtos_to_entities(create_dto.entities)
            new_entities = uow.root_repository.create(entities_to_create)
            return self._convert_entities_to_dtos(new_entities)

    def validate(self, create_dto: CreateRootsDto):
        if not create_dto.entities:
            raise ValueError("No entities to create")

    def _convert_entity_to_dto(self, entity: Root) -> RootDto:
        return RootDto(
            id_=entity.id_,
            # global_id=entity.global_id,
            entities=entity.entities,
            features=entity.features,
        )

    def _convert_dto_to_entity(self, dto: RootDto) -> Root:
        return Root(
            id_=0,
            # global_id=dto.global_id,
            entities=dto.entities,
            features=dto.features,
        )

    def _convert_entities_to_dtos(self, entities: Sequence[Root]) -> list[RootDto]:
        return [self._convert_entity_to_dto(root) for root in entities]

    def _convert_dtos_to_entities(self, dtos: Sequence[RootDto]) -> list[Root]:
        return [self._convert_dto_to_entity(dto) for dto in dtos]
