from abc import ABC, abstractmethod
from pathlib import Path
from typing import Any

class IImporter(ABC):

    @abstractmethod
    def load_file(self, file_path: Path):
        pass
    
    @abstractmethod
    def get_json(self) -> Any:
        pass