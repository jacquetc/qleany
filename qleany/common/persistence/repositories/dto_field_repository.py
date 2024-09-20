from qleany.common.persistence.repositories.interfaces.i_dto_field_repository import IDtoFieldRepository
from qleany.common.entities.dto_field import DtoField

class DtoFieldRepository(IDtoFieldRepository):

    def __init__(self, ):
        pass

    def get(self, ids: list[int]) -> list[DtoField]:
        pass

    def get_all(self) -> list[DtoField]:
        pass

    def get_all_ids(self) -> list[int]:
        pass

    def create(self, dto_fields: list[DtoField]) -> list[DtoField]:
        pass

    def update(self, dto_fields: list[DtoField]) -> list[DtoField]:
        pass

    def delete(self, ids: list[int]) -> list[int]:
        pass