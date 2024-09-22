from dataclasses import dataclass


@dataclass(slots=True)
class ListCommonBaseFilesUcDto:
    preview: bool


@dataclass(slots=True)
class ListCommonBaseFilesUcResponseDto:
    files: list[str]