from sqlalchemy import create_engine
from sqlalchemy.orm import Session, sessionmaker


class SyncDatabase:
    """
    While the whole application is asynchronous, in tests, it is more convenient
    to use synchronous database operations as they have less boilerplate
    (e.g. no need for every select to be awaited, and then get the scalared result)
    """

    session_scope: sessionmaker[Session]

    def __init__(self, url: str) -> None:
        engine = create_engine(url, echo=True)
        self.session_scope = sessionmaker(bind=engine)
