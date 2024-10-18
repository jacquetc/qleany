from abc import ABC, abstractmethod
from datetime import datetime
from typing import Callable


class MessageDTO:
    def __init__(self, source: str, message: str, data: dict):
        self.source = source
        self.message = message
        self.data = data
        self.timestamp = datetime.now()

    def __repr__(self):
        return f"MessageDTO(source={self.source}, message={self.message}, data={self.data}, timestamp={self.timestamp})"


class IMessenger(ABC):
    @abstractmethod
    def notify(self, source: str, message: str, data: dict):
        pass

    @abstractmethod
    def _notify_listeners(self, dto: MessageDTO):
        pass

    @abstractmethod
    def subscribe(self, owner: object, listener: Callable):
        pass

    @abstractmethod
    def unsubscribe(self, listener: Callable):
        pass

    @abstractmethod
    def remove_listeners_by_owner(self, owner: object):
        pass


class Messenger(IMessenger):
    def __init__(self):
        self._listeners = []
        self._history = []

    def notify(self, source: str, message: str, data: dict):
        dto = MessageDTO(source, message, data)
        self._notify_listeners(dto)

    def _notify_listeners(self, dto: MessageDTO):
        for owner, listener in self._listeners:
            listener(dto)

    def subscribe(self, owner: object, listener: Callable):
        if (owner, listener) not in self._listeners:
            self._listeners.append((owner, listener))

    def unsubscribe(self, listener: Callable):
        self._listeners = [
            (owner, lst) for owner, lst in self._listeners if lst != listener
        ]

    def remove_listeners_by_owner(self, owner: object):
        self._listeners = [(own, lst) for own, lst in self._listeners if own != owner]
