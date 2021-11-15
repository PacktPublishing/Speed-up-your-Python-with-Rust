from flask import _app_ctx_stack
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, scoped_session
from config import GlobalParams


class DbEngine:

    def __init__(self) -> None:
        params = GlobalParams()
        self.base = declarative_base()
        self.engine = create_engine(params.get("DB_URL"),
                                    echo=True,
                                    pool_recycle=3600,
                                    pool_size=2,
                                    max_overflow=1,
                                    connect_args={
                                        'connect_timeout': 5
                                    })
        self.session = scoped_session(sessionmaker(
            bind=self.engine
        ), scopefunc=_app_ctx_stack)
        self.url = params.get("DB_URL")


dal = DbEngine()
