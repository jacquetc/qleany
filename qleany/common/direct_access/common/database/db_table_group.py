import sqlite3
from typing import Sequence

from qleany.common.entities.i_entity import IEntity

class DbTableGroup:

    entity_dict: dict[int, IEntity] = dict()
    next_free_id: int = 0

    def __init__(self, entity_type: type[IEntity], db_connection: sqlite3.Connection):
        self.entity_type = entity_type
        self.db_connection = db_connection

    def get(self, ids: list[int]) -> Sequence[IEntity]:
        # This method should return a list of entities from the database with the given ids.
        self.entity_dict = {id_: self.entity_type() for id_ in ids}
        return list(self.entity_dict.values())

    def get_all(self) -> Sequence[IEntity]:
        # This method should return a list of all entities from the database.
        return list(self.entity_dict.values())

    def get_all_ids(self) -> Sequence[int]:
        # This method should return a list of all entity ids from the database.
        return list(self.entity_dict.keys())

    def create(self, entities: list[IEntity]) -> Sequence[IEntity]:
        # This method should create the given entities in the database and return a list of the created entities.
        for entity in entities:
            entity.id_ = self.next_free_id
            self.entity_dict[self.next_free_id] = entity
            self.next_free_id += 1

        return entities

    def update(self, entities: list[IEntity]) -> Sequence[IEntity]:
        # This method should update the given entities in the database and return a list of the updated entities.
        for entity in entities:
            self.entity_dict[entity.id_] = entity

        return entities

    def remove(self, ids: list[int]) -> Sequence[int]:
        # This method should remove the entities with the given ids from the database.
        for id_ in ids:
            if id_ in self.entity_dict:
                del self.entity_dict[id_]

        return ids

    def clear(self):
        # This method should remove all entities from the database.
        self.entity_dict.clear()
        self.next_free_id = 0

    def get_right_ids(self, left_entity: str, field_name: str, left_ids: list[int]) -> list[int]:
        # This method should return a list of right entity ids that are related to the left entity ids.
        pass