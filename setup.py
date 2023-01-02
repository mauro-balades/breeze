# -*- coding: utf-8 -*-
from setuptools import setup, find_packages

try:
    long_description = open("README.rst").read()
except IOError:
    long_description = ""

setup(
    name="breeze-build",
    version="0.1.2",
    description="Breeze is a light-weight, highly-customizable build tool for different languages such as C(++) and Java",
    license="MIT",
    entry_points={
        'console_scripts': [
            "breeze = src.breeze:main"
        ]
    },
    author_email='mauro.balades@tutanota.com',
    author="mauro-balades",
    install_requires=[],
    package_dir = {"": "src"},
    packages = find_packages(where="src"),
    long_description=long_description,
    classifiers=[
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Natural Language :: English',
        "Programming Language :: Python",
        "Programming Language :: Python :: 3.8",
    ]
)
