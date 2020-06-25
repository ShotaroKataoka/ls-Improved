import setuptools
from os import path
import re

with open("README.md", "r") as fh:
    long_description = fh.read()

package_name = 'ls-Improved'
main_directory = 'ls_improved'
root_dir = path.abspath(path.dirname(__file__))
with open(path.join(root_dir, main_directory, '__init__.py')) as f:
    init_text = f.read()
    version = re.search(r'__version__\s*=\s*[\'\"](.+?)[\'\"]', init_text).group(1)
    license = re.search(r'__license__\s*=\s*[\'\"](.+?)[\'\"]', init_text).group(1)
    author = re.search(r'__author__\s*=\s*[\'\"](.+?)[\'\"]', init_text).group(1)
    author_email = re.search(r'__author_email__\s*=\s*[\'\"](.+?)[\'\"]', init_text).group(1)
    url = re.search(r'__url__\s*=\s*[\'\"](.+?)[\'\"]', init_text).group(1)

assert version
assert license
assert author
assert author_email
assert url

setuptools.setup(
    name=package_name,
    version=version,
    author=author,
    author_email=author_email,
    description="show directory description when ls.",
    long_description=long_description,
    keywords='ls, filer, command, mkdir, utils',
    long_description_content_type="text/markdown",
    url=url,
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python",
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 2.7',
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        ],
    license=license,
    entry_points = {
        'console_scripts': [
            'lsi = ls_improved.lsi:main',
            'mkdiri = ls_improved.mkdiri:main',
            ],
        },
    python_requires='>=2.7',
)
