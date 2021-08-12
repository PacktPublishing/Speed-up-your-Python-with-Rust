#!/usr/bin/env python
from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])
from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name="flitton-oasis-risk-modelling",
    version="0.1",
    rust_extensions=[RustExtension(
        ".flitton_oasis_risk_modelling.flitton_oasis_risk_modelling",
        path="Cargo.toml", binding=Binding.PyO3)],
    packages=["flitton_oasis_risk_modelling"],
    include_package_data=True,
    package_data={'': ['*.csv']},
    zip_safe=False,
)
