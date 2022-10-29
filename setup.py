# -*- coding: utf-8 -*-
from setuptools import setup, find_packages

try:
    long_description = open("README.rst").read()
except IOError:
    long_description = ""

setup(
    name="breeze",
    version="0.1.0",
    description="Breeze is a light-weight, highly-customizable build tool for different languages such as C(++) and Java",
    license="MIT",
    author="mauro-balades",
    packages=find_packages(),
    install_requires=[],
    long_description=long_description,
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Python :: 3.8",
    ]
)
