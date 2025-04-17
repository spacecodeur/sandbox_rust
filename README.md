# Rust Sandbox

This repository is your playground for experimenting with advanced Rust concepts. It is designed as a sandbox environment where I can safely explore new ideas, test code, and sharpen your Rust skills.

## Overview

Inside this repository you'll find a collection of experiments and learning projects in Rust. It serves as a personal laboratory to dive deep into the language while experimenting with different patterns, libraries, and programming paradigms.

This repository uses Rust’s built-in [examples](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples) system to organize small, self-contained programs focused on specific concepts like smart pointers, ownership, concurrency, and more.

### Docker-Based Commands

To streamline the development process, several handy commands are available. These commands are built on top of Docker, ensuring that you always work with a fully installed and configured Rust environment.

### Starting the Development Environment

To launch a Docker container with the complete Rust environment, run:

`./commands.sh app/container/build`

This command will build and start a container configured with all necessary Rust tooling.

### Accessing the Container Shell

Once your container is up and running, you can enter the container to execute commands inside the environment with:

`./commands.sh app/container/shell`

This allows you to work interactively inside the container, ensuring consistency and isolation from your host system.

You can also connect to the running container with the plugin [remote explorer](https://marketplace.visualstudio.com/items/?itemName=ms-vscode-remote.remote-containers).

## Getting Started

0. Clone the repository

```bash
git clone <repository_url>
cd <repository_directory>
```

1. Start the container

Run the following command to build the rust container :

`./commands.sh app/container/build.sh`

(tips: you would like have autocompletion when you run `commands.sh <autocomplete plz with tab>` ? exec `source ./commands.sh` before !)

2. Access the interactive shell

Once the container is running, enter it via:

`./commands.sh app/container/shell`

3. Start experimenting

Now that you’re inside the container, you can compile your Rust projects, run tests, and try out advanced Rust patterns with all required tooling at your disposal.

### Running Examples

This project makes use of Cargo's example system (examples/ directory) to demonstrate specific Rust concepts in isolation.

To run a specific example, use the following command pattern:

`./commands.sh fc/app/examples/run <example_name>`

For instance, if you want to run the smartpointers examples, execute:

`./commands.sh fc/app/examples/run smartpointers`

`fc` stands for `from container`, that means commands begin by `fc` can be run from the host machine or the container.

These examples are designed to be small, focused, and easy to extend. Feel free to explore them or add your own under the `examples/` folder.

### Running Tests

- unit tests : `./commads.sh fc/app/examples/tests/unit`
- unit tests with code coverage : `fc/app/examples/tests/unit-cov`

## Contributing

Feel free to fork this repository and suggest improvements or share your experiments by creating a pull request. This sandbox is meant to evolve as you explore new Rust features and techniques.

### License

This project is open source and available under the MIT License.