from typing import Sequence, cast
from qleany.common.direct_access.common.database.sqlite_db_table_group import SqliteDbTableGroup
from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.direct_access.root.i_root_db_table_group import IRootDbTableGroup
from qleany.common.entities.entity_enums import RelationshipInfo
from qleany.common.entities.root import Root


class RootDbTableGroup(IRootDbTableGroup):
    def __init__(self, db_connection: IDbConnection):
        super().__init__()
        self._sqlite_db_table_group = SqliteDbTableGroup(Root, db_connection.connection())

    def get(self, ids: Sequence[int]) -> Sequence[Root]:
        return cast(Sequence[Root], self._sqlite_db_table_group.get(ids))
    
    def get_all(self) -> Sequence[Root]:

        return cast(Sequence[Root], self._sqlite_db_table_group.get_all())
    
    def get_all_ids(self) -> Sequence[int]:
        return self._sqlite_db_table_group.get_all_ids()
    
    def create(self, entities: Sequence[Root]) -> Sequence[Root]:
        return cast(Sequence[Root], self._sqlite_db_table_group.create(entities))
    
    def update(self, entities: Sequence[Root]) -> Sequence[Root]:
        return cast(Sequence[Root], self._sqlite_db_table_group.update(entities))
    
    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        return self._sqlite_db_table_group.remove(ids)
    
    def clear(self):
        self._sqlite_db_table_group.clear()

    def exists(self, id_: int) -> bool:
        return self._sqlite_db_table_group.exists(id_)

    def get_left_ids(self,relationship: RelationshipInfo, right_id: int) -> Sequence[int]:
        return self._sqlite_db_table_group.get_left_ids(relationship, right_id)
    
    def get_right_ids(self, relationship: RelationshipInfo, left_id: int) -> Sequence[int]:
        return self._sqlite_db_table_group.get_right_ids(relationship, left_id)
    
    