from typing import List, Sequence
from qleany.direct_access.root.i_root_uow import IRootUow

class RemoveUc():
    def __init__(self, unit_of_work: IRootUow):
        self._unit_of_work = unit_of_work

    def execute(self, entity_ids: Sequence[int]) -> Sequence[int]:
        
        self._validate(entity_ids)
        
        with self._unit_of_work as uow:
            removed_ids = uow.root_repository.remove(entity_ids)
            return removed_ids
    
    def _validate(self, entity_ids: Sequence[int]):
        # verify if exist
        with self._unit_of_work as uow:
            entities = uow.root_repository.get(entity_ids)
            if len(entities) != len(entity_ids):
                raise Exception("Some entities do not exist")