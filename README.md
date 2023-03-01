# Scrum master tools (Backend)
This repository holds the source code for the API of Scrum Master Tools

## Setup
To setup this project you have to build the docker-compose file that builds up a mongodb container for the database
I will now write the steps to build the project
> NOTE: You have to provide the enviroment varialbes `MONGODB_ROOT_USERNAME` and `MONGODB_ROOT_PASSWORD` before running the project.

### Build and run production
```console
$ cargo build --release
$ docker-compose up
$ ./target/release/smt_backend
```

### Build and run development
```console
$ docker-compose up
$ cargo run
```

## Changelog
See [Changelog](./CHANGELOG.md)

## Todo list
See [Todo List](./TODO.md)