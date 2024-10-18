from qleany.common.direct_access.common.database.interfaces.i_db_context import (
    IDbContext,
)
from qleany.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany.common.direct_access.common.repository.repository_messenger import (
    IMessenger,
)
from qleany.python_file_listing_feature.dtos import (
    PythonFileListingDto,
    PythonFileListingResponseDto,
)
from qleany.python_file_listing_feature.python_file_listing_uow import (
    PythonFileListingUow,
)
from qleany.python_file_listing_feature.use_cases.list_common_base_files_uc import (
    ListCommonBaseFilesUc,
)
from qleany.python_file_listing_feature.use_cases.list_direct_access_files_uc import (
    ListDirectAccessFilesUc,
)
from qleany.python_file_listing_feature.use_cases.list_entity_files_uc import (
    ListEntityFilesUc,
)
from qleany.python_file_listing_feature.use_cases.list_feature_files_uc import (
    ListFeatureFilesUc,
)
from qleany.python_file_listing_feature.use_cases.list_persistence_files_uc import (
    ListPersistenceFilesUc,
)
from qleany.python_file_listing_feature.use_cases.list_ui_files_uc import ListUiFilesUc


class PythonFileListingController:
    _instance = None
    _db_context: IDbContext | None = None
    _repository_factory: IRepositoryFactory | None = None
    _messenger: IMessenger | None = None

    @classmethod
    def get_instance(cls):
        if cls._instance is None:
            if cls._db_context is None or cls._repository_factory is None:
                raise ValueError(
                    "RootController must be initialized with db_context and repository_factory first"
                )
            if cls._messenger is None:
                raise ValueError("Messenger must be initialized first")
            cls._instance = cls(
                cls._db_context, cls._repository_factory, cls._messenger
            )
        return cls._instance

    @classmethod
    def initialize(
        cls,
        db_context: IDbContext,
        repository_factory: IRepositoryFactory,
        messenger: IMessenger,
    ):
        if cls._instance is None:
            cls._db_context = db_context
            cls._repository_factory = repository_factory
            cls._messenger = messenger

    def __init__(
        self,
        db_context: IDbContext,
        repository_factory: IRepositoryFactory,
        messenger: IMessenger,
    ):
        if PythonFileListingController._instance is not None:
            raise Exception("This class is a singleton!")

        self._db_context = db_context
        self._repository_factory = repository_factory
        self._messenger = messenger

    def list_direct_access_files(
        self, dto: PythonFileListingDto
    ) -> PythonFileListingResponseDto:
        unit_of_work = PythonFileListingUow(self._db_context, self._repository_factory)  # type: ignore
        return ListDirectAccessFilesUc(unit_of_work).execute(dto)

    def list_entity_files(
        self, dto: PythonFileListingDto
    ) -> PythonFileListingResponseDto:
        unit_of_work = PythonFileListingUow(self._db_context, self._repository_factory)  # type: ignore
        return ListEntityFilesUc(unit_of_work).execute(dto)

    def list_feature_files(
        self, dto: PythonFileListingDto
    ) -> PythonFileListingResponseDto:
        unit_of_work = PythonFileListingUow(self._db_context, self._repository_factory)  # type: ignore
        return ListFeatureFilesUc(unit_of_work).execute(dto)

    def list_persistence_files(
        self, dto: PythonFileListingDto
    ) -> PythonFileListingResponseDto:
        unit_of_work = PythonFileListingUow(self._db_context, self._repository_factory)  # type: ignore
        return ListPersistenceFilesUc(unit_of_work).execute(dto)

    def list_common_base_files(
        self, dto: PythonFileListingDto
    ) -> PythonFileListingResponseDto:
        unit_of_work = PythonFileListingUow(self._db_context, self._repository_factory)  # type: ignore
        return ListCommonBaseFilesUc(unit_of_work).execute(dto)

    def list_ui_files(self, dto: PythonFileListingDto) -> PythonFileListingResponseDto:
        unit_of_work = PythonFileListingUow(self._db_context, self._repository_factory)  # type: ignore
        return ListUiFilesUc(unit_of_work).execute(dto)
