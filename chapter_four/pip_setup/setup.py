import setuptools
from setuptools import find_packages
import pathlib


with open("README.md", "r") as fh:
    long_description = fh.read()

with open(str(pathlib.Path(__file__).parent.absolute()) + "/monolithcaching/version.py", "r") as fh:
    version = fh.read().split("=")[1].replace("'", "")

# pushing another build
directives = {
    'language_level': 3,
    'always_allow_keywords': True
}

setuptools.setup(
    name="monolithcaching",
    version=version,
    author="Maxwell Flitton",
    author_email="maxwell@monolithai.com",
    description="Python package for monolithcaching",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/MonolithAILtd/caching",
    install_requires=[
        "redis>=3.3.8",
        "boto3>=1.9.243",
        "botocore>=1.11.1"
    ],
    packages=find_packages(exclude=("tests",)),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'caching-hello = monolithcaching.console_commands.hello:print_logo',
        ],
    }
)
