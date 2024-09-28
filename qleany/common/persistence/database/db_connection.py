from sqlalchemy.orm import Session

from qleany.common.persistence.database.interfaces.i_db_connection import (
    IDbConnection,
)


class DbConnection(IDbConnection):
    def __init__(self, session: Session):
        self._session = session

    def connection(self) -> Session:
        return self._session

    def commit(self):
        self._session.commit()
