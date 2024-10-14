from qleany.common.direct_access.common.database.db_table_group import DbTableGroup
from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.entities.entity import Entity


class EntityDbTableGroup(DbTableGroup):
    def __init__(self, db_connection: IDbConnection):
        super().__init__(Entity, db_connection.connection())