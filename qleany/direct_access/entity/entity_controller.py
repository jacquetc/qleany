

from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory


class EntityController:

    def __init__(self, repository_factory: IRepositoryFactory):
        self._repository_factory = repository_factory