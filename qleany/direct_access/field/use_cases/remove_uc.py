from typing import List, Sequence
from qleany.direct_access.field.i_field_uow import IFieldUow

class RemoveUc():
    def __init__(self, unit_of_work: IFieldUow):
        self._unit_of_work = unit_of_work

    def execute(self, entity_ids: Sequence[int]) -> Sequence[int]:
        
        self._validate(entity_ids)
        
        with self._unit_of_work as uow:
            removed_ids = uow.field_repository.remove(entity_ids)
            return removed_ids
    
    def _validate(self, entity_ids: Sequence[int]):
        # verify if exist
        with self._unit_of_work as uow:
            entities = uow.field_repository.get(entity_ids)
            if len(entities) != len(entity_ids):
                raise Exception("Some entities do not exist")