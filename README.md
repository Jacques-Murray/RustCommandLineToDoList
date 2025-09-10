# Command-Line To-Do List in Rust
This is a simple command-line to-do list application written in Rust. It allows you to add tasks, mark them as complete, and view all tasks.

## Concepts Covered
- **Structs and Impls:** Organising data and behaviour.
- **HashMap:** Storing key-value data for the to-do items.
- **File I/O:** Reading from and writing to a file (`db.txt`) to persist tasks.
- **Error Handling:** Using `Result` and `Option` types for robust error management.
- **Command-Line Argument Parsing:** Reading user input from the terminal.
- **Ownership and Borrowing:** Rust's core memory management principles.

## How to Run
1. **Compile the program:**
   ```bash
   rustc src/main.rs -o todo
   ```
2. **Add a new task:**
    ```bash
    ./todo add "Buy milk"
    ```
3. **Add another task:**
    ```bash
    ./todo add "Read a book"
    ```
4. **List all tasks:**
    ```bash
    ./todo list
    ```
    *Output:*
    ```bash
    --- To-Do List ---
    [ ] Buy milk
    [ ] Read a book
    ------------------
    ```
5. **Complete a task:**
    ```bash
    ./todo complete "Buy milk"
    ```
6. **List tasks again:**
    ```bash
    ./todo list
    ```
    *Output:*
    ```bash
    --- To-Do List ---
    [x] Buy milk
    [ ] Read a book
    ------------------
    ```