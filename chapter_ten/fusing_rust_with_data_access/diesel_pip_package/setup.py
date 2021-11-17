#!/usr/bin/env python
from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])
from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name="rust-database-cloning",
    version="0.1",
    rust_extensions=[RustExtension(
        ".rust_db_cloning.rust_db_cloning",
        path="Cargo.toml", binding=Binding.PyO3)],
    packages=["rust_db_cloning"],
    zip_safe=False,
)
