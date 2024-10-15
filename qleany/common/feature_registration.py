from qleany.common.direct_access.common.database.interfaces.i_db_context import IDbContext
from qleany.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany.common.direct_access.common.repository.repository_messenger import IMessenger
from qleany.python_file_listing_feature.python_file_listing_controller import PythonFileListingController

def register_features(db_context: IDbContext, repository_factory: IRepositoryFactory, messenger: IMessenger):
    
    PythonFileListingController.initialize(db_context, repository_factory, messenger)