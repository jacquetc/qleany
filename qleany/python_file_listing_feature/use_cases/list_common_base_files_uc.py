from qleany.python_file_listing_feature.dtos import (
    ListCommonBaseFilesUcDto,
    ListCommonBaseFilesUcResponseDto,
)


class ListCommonBaseFilesUc:
    def __init__(self):
        pass

    def execute(self, db_context: IDbContext, dto: ListCommonBaseFilesUcDto):

        files = ["qleany/common/__init__.py"]

        if dto.preview:
            files = [f"preview/{file}" for file in files]

        return ListCommonBaseFilesUcResponseDto(files=files)
