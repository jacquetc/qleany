from typing import Sequence, cast
from qleany.common.direct_access.common.database.sqlite_db_table_group import SqliteDbTableGroup
from qleany.common.direct_access.common.database.interfaces.i_db_connection import IDbConnection
from qleany.common.direct_access.feature.i_feature_db_table_group import IFeatureDbTableGroup
from qleany.common.entities.entity_enums import RelationshipInfo
from qleany.common.entities.feature import Feature


class FeatureDbTableGroup(IFeatureDbTableGroup):
    def __init__(self, db_connection: IDbConnection):
        super().__init__()
        self._sqlite_db_table_group = SqliteDbTableGroup(Feature, db_connection.connection())

    def get(self, ids: Sequence[int]) -> Sequence[Feature]:
        return cast(Sequence[Feature], self._sqlite_db_table_group.get(ids))
    
    def get_all(self) -> Sequence[Feature]:

        return cast(Sequence[Feature], self._sqlite_db_table_group.get_all())
    
    def get_all_ids(self) -> Sequence[int]:
        return self._sqlite_db_table_group.get_all_ids()
    
    def create(self, entities: Sequence[Feature]) -> Sequence[Feature]:
        return cast(Sequence[Feature], self._sqlite_db_table_group.create(entities))
    
    def update(self, entities: Sequence[Feature]) -> Sequence[Feature]:
        return cast(Sequence[Feature], self._sqlite_db_table_group.update(entities))
    
    def remove(self, ids: Sequence[int]) -> Sequence[int]:
        return self._sqlite_db_table_group.remove(ids)
    
    def clear(self):
        self._sqlite_db_table_group.clear()

    def get_left_ids(self,relationship: RelationshipInfo, right_id: int) -> Sequence[int]:
        return self._sqlite_db_table_group.get_left_ids(relationship, right_id)
    
    def get_right_ids(self, relationship: RelationshipInfo, left_id: int) -> Sequence[int]:
        return self._sqlite_db_table_group.get_right_ids(relationship, left_id)
    
    