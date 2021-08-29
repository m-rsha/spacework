# `spacework` - a workspace manager

## Warning: This project is nowhere near complete

`spacework` is hoping to become a simple workspace manager similar to `cargo`
without the dependency management. It will mostly help organize directories and
track your previous commands to help you remember what you were working on last.
It also will allow you to run simple build commands from it, along with
defining your own custom commands in `toml` files.

My main motivation for building it is to be able to organize and track my work.

# Where it is

`spacework` isn't quite ready yet, but it's slowly and steadily making progress!

# Where it's going

I'm still figuring out how I want things to work in the end, but I'm planning on
having functionality similar to the following:

```sh
# Create a new workspace in C++
$ spacework new -l cpp hello_socks
Created `spacework` directory: /home/marsha/spacework/cpp/hello_socks

# It comes with a `main.cpp` file with a "hello, world!" greeting like `cargo`.
$ tree ~/spacework/cpp/hello_socks/
/home/marsha/spacework/cpp/hello_socks/
├── bin
└── src
    └── main.cpp

# It writes to a `.spacework_history` file to keep track of projects.
# I'm thinking of storing information in sqlite instead, and
# reading it with a `spacework history` subcommand.
$ cat ~/.spacework_history
2021-08-11@13:59:08: Hello hello, world!
2021-08-11@13:59:09: Created /home/marsha/spacework/cpp/hello_socks

# It should have build and run subcommands to choose between only compiling
# your code, or compiling and run it all at once.
$ cd ~/spacework/cpp/hello_socks
$ spacework run
Hello, world!
```
