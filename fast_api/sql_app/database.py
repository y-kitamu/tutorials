"""database.py

Author : Yusuke Kitamura
Create Date : 2021-12-26 06:37:58
Copyright (c) 2019- Yusuke Kitamura <ymyk6602@gmail.com>
"""
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

SQLITE_DATABASE_URL = "sqlite:///./sql_app.db"
engine = create_engine(SQLITE_DATABASE_URL, connect_args={"check_same_thread": False})
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()
