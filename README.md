## ZCopy allows for seamless file copying in the terminal

### Description
This package builds on [zoxide](https://github.com/ajeetdsouza/zoxide) and ``cp`` shell command (``copy`` for Windows) to easily move files in your system.
Copy files or directories to any directory you've already been to

### Dependencies
 - [zoxide](https://github.com/ajeetdsouza/zoxide)


NOTE: Support has been added for ZCopy on Windows but it is currently untested on Windows

### To install

- Install [zoxide](https://github.com/ajeetdsouza/zoxide)
- Install [cargo](https://github.com/rust-lang/cargo) with ``curl https://sh.rustup.rs -sSf | sh``

- Run ``cargo install zcopy``

- Run ``zcp --version`` to verify installation

### Installing manually

- Install [zoxide](https://github.com/ajeetdsouza/zoxide)
- Install [cargo](https://github.com/rust-lang/cargo)
- clone the repository at [zmove](https://github.com/korkie9/zmove)
- Build with ``cargo build --release`` and find executable in release folder


### How to use

- To move a single file or directory: ``zcp <file name> <target directory>``

- To move multiple files or directories: ``zcp *.<extension if file has one> <target directory>``

- To move multiple files or directories: ``zcp <path (optional)>/*.<extension if file has one> <target directory>``

- To move all files or directories: ``zcp <path (optional)>/*.* <target directory>``


Eg. If you have a directory called foobar and you want to move example.txt into it, you could run ``zcp example.txt foobar`` or even just ``zcp example.txt bar`` and full directory path will be inferred giving that the user has visited it before or if foobar exists in current directory.

Eg. If you would like to move multiple files to another directory, run ``zcp *.txt bar`` or ``zcp foo/*.txt bar`` or ``zcp ./* bar``


### Contribute
- Feel free to raise issues and make pull requests at ``https://github.com/korkie9/zcopy``


### Related projects
- [zmove](https://crates.io/crates/zcopy)
