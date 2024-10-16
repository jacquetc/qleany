from qleany.common.entities.entity import Entity
from qleany.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany.python_file_listing_feature.i_python_file_listing_uow import (
    IPythonFileListingUow,
)
import stringcase


class ListFeatureFilesUc:
    def __init__(self, unit_of_work: IPythonFileListingUow):
        self._unit_of_work = unit_of_work

    def execute(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:

        self._validate(dto)

        files = [
            "common/feature_registration.py",
        ]

        with self._unit_of_work as uow:
            root = uow.root_repository.get([1])[0]
            features = list(uow.feature_repository.get(root.features))

            for feature in features:
                feature_name_snake = stringcase.snakecase(feature.name)
                files.append(f"{feature_name_snake}_feature/__init__.py")
                files.append(
                    f"{feature_name_snake}_feature/{feature_name_snake}_controller.py"
                )
                files.append(
                    f"{feature_name_snake}_feature/i_{feature_name_snake}_uow.py"
                )
                files.append(
                    f"{feature_name_snake}_feature/{feature_name_snake}_uow.py"
                )
                files.append(f"{feature_name_snake}_feature/dtos.py")
                files.append(f"{feature_name_snake}_feature/use_cases/__init__.py")

                # use cases

                # files.append(f"{feature_name_snake}_feature/use_cases/{use_case_name_snake}_uc.py")

        return PythonFileListingResponseDto(files=files)

    def _validate(self, dto: PythonFileListingDto):
        # verify if exist
        with self._unit_of_work as uow:
            if not uow.root_repository.exists(1):
                raise Exception("Root wasn't created")
