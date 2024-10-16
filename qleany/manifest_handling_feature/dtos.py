from dataclasses import dataclass

@dataclass(slots=True)
class LoadManifestDto:
    file_path: str
    
@dataclass(slots=True)
class SaveManifestDto:
    file_path: str
