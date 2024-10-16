
import json
from pathlib import Path
from typing import Any
from qleany.manifest_handling_feature.i_importer import IImporter
from ruamel.yaml import YAML 

class YamlImporter(IImporter):
    
    def load_file(self, file_path: Path):
        with open(file_path, "r") as file:
            yaml: YAML = YAML(typ='safe', pure=True)
            self.data = yaml.load(file)
        self._validate_schema()
            
    def _validate_schema(self):
        # check manifest version
        if self.data.get('schema', {}).get('version', 0) != 2:
            raise ValueError("Invalid manifest version, need version 2")
            
    def get_json(self) -> Any:
        # dump yaml to json
        json_manifest = json.loads(json.dumps(self.data))
        return json_manifest