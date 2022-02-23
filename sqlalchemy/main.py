"""main.py

Author : Yusuke Kitamura
Create Date : 2021-12-26 20:23:14
Copyright (c) 2019- Yusuke Kitamura <ymyk6602@gmail.com>
"""
from sqlalchemy import create_engine

engine = create_engine("sqlite+pysqlite:///:memory:", echo=True, future=True)
