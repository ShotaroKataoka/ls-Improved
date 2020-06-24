import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="ls-Improved-ShotaroKataoka",
    version="0.2.1-beta",
    author="ShotaroKataoka",
    author_email="shotaro.kata@gmail.com",
    description="show directory description when ls.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/ShotaroKataoka/ls-Improved",
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        ],
    entry_points = {
        'console_scripts': [
            'lsi = ls_improved.lsi:main',
            'mkdiri = ls_improved.mkdiri:main',
            ],
        },
    python_requires='>=2.7',
)
