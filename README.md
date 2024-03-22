# ArdanLabs Rust

Finished 17_2_1

## Learnings

- The `String` is designed for manipulation and edits. The `&str` is designed to be "view" into the original `String`.
    - If you want to modify the "string," use the `String` data type.

- Rust will compile things in parallel. Splitting your codebase into separate libraries might be beneficial.

- There is a quite important difference between `.iter()` and `into_iter()` functions.
    - The `iter()` passes your vector into the iterator as a _reference_. This means you can still use the vector AFTER consuming the iterator.
    - The `into_iter()` consumes the original vector. This means that you will NOT be able to use the original vector after consuming the iterator.

- You can `use` within a function block. It looks like the following.

  ```rust
  pub fn hash_password(password: &str) -> String {
      use sha2::{Digest};
      let mut hasher = sha2::Sha256::new();
      hasher.update(password);

      return format!("{:X}", hasher.finalize());
  }
  ```

  Aside from the possible _name collision avoidance_, I'm unsure what could be the benefit of using `use` in blocks rather than at the top of the file.

- One has to **be mindful of using _paths_ in libraries**.
    - The **"absolute" path, like `foo.json` will point to the library root directory, not the main application root directory**.

- Threads are good for CPU heavy operations.
    - **Thread creation is slow**.

- To "wait" for the thread, use the `join` function.
    - Interesting name.

- To move data into the thread, you ought to use the `move` keyword.

  ```rust
  let mut thread_handles = Vec::new();
  
  for i in 0..5 {
      // Notice the `move` keyword here.
      let thread_handle = std::thread::spawn(move || {
          hello_thread(i)
      });
      thread_handles.push(thread_handle);
  }
  ```

- Threads could be named. This is quite useful for debugging.
    - To get the thread name, use the `thread::current().name()` function, like so
    ```rust
    fn my_thread() {
        println!("Hello from a thread named {}", thread::current().name().unwrap());
    }
    ```
- You can create **_scoped_ threads**. These are **handy for retaining ownership of variables from the outer scope**.

  ```rust
  struct Person {
      first_name: String,
  }

  fn example() {
      let person = Person {
          first_name: "Wojciech".to_string()
      };

      let print_name = || {
          println!("First name is: {}", &person.first_name)
      };

      thread::scope(|scope| {
          // I do not have to `join` here. It is done automatically!
          scope.spawn(print_name);
      });

      println!("I still have the ownership of: {}", person.first_name);
  }
  ```

  If I did not use the `thread::scope`, I would need to _move_ the ownership of the `person` into the closure.
  Doing that would prevent me from using the `person.first_name` AFTER the thread finished, since that variable is now dropped.