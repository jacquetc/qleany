from qleany.python_file_listing_feature.dtos import (
    ListDirectAccessFilesUcDto,
    ListDirectAccessFilesUcResponseDto,
)
from qleany.common.direct_access.entity.i_entity_repository import (
    IEntityRepository,
)
from qleany.common.persistence.repositories.interfaces.i_field_repository import (
    IFieldRepository,
)
from qleany.common.persistence.database.interfaces.i_db_context import IDbContext


class ListDirectAccessFilesUc:
    def __init__(
        self, entity_repository: IEntityRepository, field_repository: IFieldRepository
    ):
        self.entity_repository = entity_repository
        self.field_repository = field_repository

    def execute(self, db_context: IDbContext, dto: ListDirectAccessFilesUcDto):

        conn = db_context.get_connection()

        files = ["qleany/common/__init__.py"]

        if dto.preview:
            files = [f"preview/{file}" for file in files]

        return ListDirectAccessFilesUcResponseDto(files=files)
