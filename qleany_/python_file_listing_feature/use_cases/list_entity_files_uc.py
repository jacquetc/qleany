import stringcase

from qleany_.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany_.python_file_listing_feature.i_python_file_listing_uow import (
    IPythonFileListingUow,
)


class ListEntityFilesUc:
    def __init__(self, unit_of_work: IPythonFileListingUow):
        self._unit_of_work = unit_of_work

    def execute(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:
        self._validate(dto)

        files = [
            "common/entities/__init__.py",
            "common/entities/entity_enums.py",
            "common/entities/i_entity.py",
        ]

        with self._unit_of_work as uow:
            root = uow.root_repository.get([1])[0]
            entities = list(uow.entity_repository.get(root.entities))

        for entity in entities:
            entity_name_snake = stringcase.snakecase(entity.name)
            files.append(f"common/entities/{entity_name_snake}.py")

        return PythonFileListingResponseDto(files=files)

    def _validate(self, dto: PythonFileListingDto):
        # verify if exist
        with self._unit_of_work as uow:
            if not uow.root_repository.exists(1):
                raise Exception("Root wasn't created")
