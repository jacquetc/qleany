from typing import Sequence

from qleany.common.direct_access.common.database.interfaces.i_db_context import (
    IDbContext,
)
from qleany.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany.direct_access.field.dtos import CreateFieldsDto, FieldDto
from qleany.direct_access.field.field_uow import FieldUow
from qleany.direct_access.field.use_cases.create_uc import CreateUc
from qleany.direct_access.field.use_cases.get_uc import GetUc
from qleany.direct_access.field.use_cases.remove_uc import RemoveUc
from qleany.direct_access.field.use_cases.update_uc import UpdateUc


class FieldController:
    _instance = None
    _db_context: IDbContext | None = None
    _repository_factory: IRepositoryFactory | None = None

    @classmethod
    def get_instance(cls):
        if cls._instance is None:
            if cls._db_context is None or cls._repository_factory is None:
                raise ValueError(
                    "RootController must be initialized with db_context and repository_factory first"
                )
            cls._instance = cls(cls._db_context, cls._repository_factory)
        return cls._instance

    @classmethod
    def initialize(cls, db_context: IDbContext, repository_factory: IRepositoryFactory):
        if cls._instance is None:
            cls._db_context = db_context
            cls._repository_factory = repository_factory

    def __init__(self, db_context: IDbContext, repository_factory: IRepositoryFactory):
        if FieldController._instance is not None:
            raise Exception("This class is a singleton!")

        self._db_context = db_context
        self._repository_factory = repository_factory

    def create(self, dto: CreateFieldsDto) -> Sequence[FieldDto]:
        unit_of_work = FieldUow(self._db_context, self._repository_factory)  # type: ignore
        return CreateUc(unit_of_work).execute(dto)

    def get(self, ids: Sequence[int]) -> Sequence[FieldDto]:
        unit_of_work = FieldUow(self._db_context, self._repository_factory)  # type: ignore
        return GetUc(unit_of_work).execute(ids)

    def update(self, update_dtos: Sequence[FieldDto]) -> Sequence[FieldDto]:
        unit_of_work = FieldUow(self._db_context, self._repository_factory)  # type: ignore
        return UpdateUc(unit_of_work).execute(update_dtos)

    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        unit_of_work = FieldUow(self._db_context, self._repository_factory)  # type: ignore
        return RemoveUc(unit_of_work).execute(ids)
