from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name='predictables_rs',
    version='0.1',
    rust_extensions=[RustExtension('my_package.my_module')],
    packages=['predictables_rs'],
    zip_safe=False,
)