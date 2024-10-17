from qleany_.common.direct_access.common.database.interfaces.i_db_context import (
    IDbContext,
)
from qleany_.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany_.common.direct_access.common.repository.repository_messenger import (
    IMessenger,
)
from qleany_.manifest_handling_feature.dtos import LoadManifestDto
from qleany_.manifest_handling_feature.manifest_handling_uow import ManifestHandlingUow
from qleany_.manifest_handling_feature.use_cases.load_uc import LoadUc
from qleany_.manifest_handling_feature.yaml_importer import YamlImporter


class ManifestHandlingController:
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
        if ManifestHandlingController._instance is not None:
            raise Exception("This class is a singleton!")

        self._db_context = db_context
        self._repository_factory = repository_factory
        self._messenger = messenger

    def load_manifest(self, dto: LoadManifestDto):
        unit_of_work = ManifestHandlingUow(self._db_context, self._repository_factory)  # type: ignore
        yaml_importer = YamlImporter()
        return LoadUc(unit_of_work, yaml_importer).execute(dto)
