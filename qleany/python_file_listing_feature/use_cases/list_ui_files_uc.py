from qleany.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany.python_file_listing_feature.i_python_file_listing_uow import (
    IPythonFileListingUow,
)


class ListUiFilesUc:
    def __init__(self, unit_of_work: IPythonFileListingUow):
        self._unit_of_work = unit_of_work

    def execute(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:
        files = ["ui/__init__.py", "ui/cli/__init__.py", "ui/cli/cli.py"]

        return PythonFileListingResponseDto(files=files)
