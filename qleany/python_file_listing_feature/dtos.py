from dataclasses import dataclass

@dataclass(slots=True)
class PythonFileListingDto:
    existing: bool = False
    sub_group: str = ""
    
@dataclass(slots=True)
class PythonFileListingResponseDto:
    files: list[str]