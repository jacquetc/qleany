from dataclasses import dataclass


@dataclass(slots=True)
class PythonFileListingDto:
    existing: bool = False
    sub_group: str = ""
    manifest_path: str = ""


@dataclass(slots=True)
class PythonFileListingResponseDto:
    files: list[str]
