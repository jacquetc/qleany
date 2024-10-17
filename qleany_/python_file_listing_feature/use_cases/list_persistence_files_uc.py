from qleany_.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany_.python_file_listing_feature.i_python_file_listing_uow import (
    IPythonFileListingUow,
)


class ListPersistenceFilesUc:
    def __init__(self, unit_of_work: IPythonFileListingUow):
        self._unit_of_work = unit_of_work

    def execute(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:
        files = [
            "common/direct_access/common/__init__.py",
            "common/direct_access/common/database/__init__.py",
            "common/direct_access/common/database/db_table_creator.py",
            "common/direct_access/common/database/interfaces/i_db_connection.py",
            "common/direct_access/common/database/interfaces/i_db_context.py",
            "common/direct_access/common/database/interfaces/i_db_table_group.py",
            "common/direct_access/common/database/many_to_many_ordered_associator.py",
            "common/direct_access/common/database/many_to_many_unordered_associator.py",
            "common/direct_access/common/database/one_to_many_ordered_associator.py",
            "common/direct_access/common/database/one_to_many_unordered_associator.py",
            "common/direct_access/common/database/one_to_one_associator.py",
            "common/direct_access/common/database/sqlite_db_connection.py",
            "common/direct_access/common/database/sqlite_db_context.py",
            "common/direct_access/common/database/sqlite_db_table_group.py",
            "common/direct_access/common/repository/__init__.py",
            "common/direct_access/common/repository/repository_factory.py",
            "common/direct_access/common/repository/repository_messenger.py",
        ]

        # keep the already existing files
        if dto.existing:
            for file in files:
                pass

        return PythonFileListingResponseDto(files=files)
