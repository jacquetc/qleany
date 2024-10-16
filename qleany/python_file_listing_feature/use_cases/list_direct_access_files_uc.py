from qleany.common.entities.entity import Entity
from qleany.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany.python_file_listing_feature.i_python_file_listing_uow import (
    IPythonFileListingUow,
)
import stringcase


class ListDirectAccessFilesUc:
    def __init__(self, unit_of_work: IPythonFileListingUow):
        self._unit_of_work = unit_of_work

    def execute(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:

        self._validate(dto)

        files = ["direct_access/__init__.py"]

        entities: list[Entity] = []

        with self._unit_of_work as uow:
            root = uow.root_repository.get([1])[0]
            entities = list(uow.entity_repository.get(root.entities))

        for entity in entities:
            entity_name_snake = stringcase.snakecase(entity.name)
            files.append(f"direct_access/{entity_name_snake}/__init__.py")
            files.append(
                f"direct_access/{entity_name_snake}/{entity_name_snake}_controller.py"
            )
            files.append(
                f"direct_access/{entity_name_snake}/i_{entity_name_snake}_uow.py"
            )
            files.append(
                f"direct_access/{entity_name_snake}/{entity_name_snake}_uow.py"
            )
            files.append(f"direct_access/{entity_name_snake}/dtos.py")
            files.append(f"direct_access/{entity_name_snake}/use_cases/__init__.py")
            files.append(f"direct_access/{entity_name_snake}/use_cases/create_uc.py")
            files.append(f"direct_access/{entity_name_snake}/use_cases/get_uc.py")
            files.append(f"direct_access/{entity_name_snake}/use_cases/remove_uc.py")
            files.append(f"direct_access/{entity_name_snake}/use_cases/update_uc.py")
            files.append(f"direct_access/common/{entity_name_snake}/__init__.py")
            files.append(
                f"direct_access/common/{entity_name_snake}/i_{entity_name_snake}_repository.py"
            )
            files.append(
                f"direct_access/common/{entity_name_snake}/{entity_name_snake}_repository.py"
            )
            files.append(
                f"direct_access/common/{entity_name_snake}/i_{entity_name_snake}_db_table_group.py"
            )
            files.append(
                f"direct_access/common/{entity_name_snake}/{entity_name_snake}_db_table_group.py"
            )

        return PythonFileListingResponseDto(files=files)

    def _validate(self, dto: PythonFileListingDto):
        # verify if exist
        with self._unit_of_work as uow:
            if not uow.root_repository.exists(1):
                raise Exception("Root wasn't created")
