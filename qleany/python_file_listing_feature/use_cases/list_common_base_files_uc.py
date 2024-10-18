from qleany.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany.python_file_listing_feature.i_python_file_listing_uow import (
    IPythonFileListingUow,
)


class ListCommonBaseFilesUc:
    def __init__(self, unit_of_work: IPythonFileListingUow):
        self._unit_of_work = unit_of_work

    def execute(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:
        files = ["common/__init__.py"]

        # with self._unit_of_work as uow:

        return PythonFileListingResponseDto(files=files)
