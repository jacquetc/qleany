from qleany.common.direct_access.common.database.interfaces.i_db_context import (
    IDbContext,
)
from qleany.common.direct_access.common.repository.repository_factory import (
    IRepositoryFactory,
)
from qleany.common.direct_access.common.repository.repository_messenger import (
    IMessenger,
)
from qleany.direct_access.entity.entity_controller import EntityController
from qleany.direct_access.feature.feature_controller import FeatureController
from qleany.direct_access.field.field_controller import FieldController
from qleany.direct_access.root.root_controller import RootController


def register_controllers(
    db_context: IDbContext,
    repository_factory: IRepositoryFactory,
    messenger: IMessenger,
):
    RootController.initialize(db_context, repository_factory)
    EntityController.initialize(db_context, repository_factory)
    FieldController.initialize(db_context, repository_factory)
    FeatureController.initialize(db_context, repository_factory)
