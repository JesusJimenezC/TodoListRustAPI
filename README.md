# Rust Todo-List API

This is a simple API for managing a todo-list built in Rust. The API provides endpoints to create, retrieve, update, and delete tasks in the todo-list.

## Features

- Create new tasks with a title and description.
- Retrieve all tasks or get a specific task by ID.
- Update task details such as title, description, and completion status.
- Delete tasks by ID.

## Technologies Used

- Rust programming language.
- Actix-Web framework for building the API.
- Diesel ORM for interacting with the SQLite database.
- JSON format for data exchange.

## Setup and Usage

1. Install Rust programming language and cargo build system.
2. Clone this repository.
3. Configure the database connection in the `config.toml` file.
4. Run database migrations to set up the required schema.
5. Build and run the API using `cargo run`.
6. Access the API endpoints using a tool like curl or a REST client.

## API Endpoints

- `GET /tasks`: Retrieve all tasks.
- `GET /tasks/{task_id}`: Retrieve a specific task by ID.
- `POST /tasks`: Create a new task.
- `PUT /tasks/{task_id}`: Update an existing task by ID.
- `DELETE /tasks/{task_id}`: Delete a task by ID.
- `GET /lists`: Retrieve all lists.
- `GET /lists/{list_id}`: Retrieve a specific task by ID.
- `POST /lists`: Create a new task.
- `PUT /lists/{list_id}`: Update an existing task by ID.
- `DELETE /lists/{list_id}`: Delete a task by ID.

  > You will see in the code a configuration to get all the keys from the data request as camelCase instead snake_case.

For more details on the request and response formats, please refer to the API documentation provided in the codebase.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute the code as per the license terms.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## Acknowledgements

This project was built using various open-source libraries and resources. We acknowledge and thank the authors and contributors of those projects for their valuable work.

## Contact

For any inquiries or questions, please contact [jesus.jimenezc.mx@gmail.com].

Happy coding!
