from qleany_.common.direct_access.common.database.interfaces.i_db_context import IDbContext
from qleany_.common.direct_access.common.repository.repository_factory import IRepositoryFactory
from qleany_.common.direct_access.common.repository.repository_messenger import IMessenger
from qleany_.manifest_handling_feature.manifest_handling_controller import ManifestHandlingController
from qleany_.python_file_listing_feature.python_file_listing_controller import PythonFileListingController

def register_features(db_context: IDbContext, repository_factory: IRepositoryFactory, messenger: IMessenger):
    
    PythonFileListingController.initialize(db_context, repository_factory, messenger)
    ManifestHandlingController.initialize(db_context, repository_factory, messenger)