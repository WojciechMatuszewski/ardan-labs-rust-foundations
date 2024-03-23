# ArdanLabs Rust

24_2_8

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

- When using _global state_ across threads, use `Atomic` operations or `Mutexes`.
    - Otherwise, you will expose yourself to a world of pain and misery with race conditions everywhere.
    - `Mutexes` can **create "traffic jam" if you are not fast enough with releasing the lock**.
        - `Atomics` do not suffer from this issue as they implement different mechanism than "locking."
        - Keep in mind that `Atomics` are for _simple_ data structures, like integers or strings. You will not be able to use `Atomic` for vectors and structures.

- The **read-write mutex is a good alternative to the "vanilla" mutex IN SOME SITUATIONS**.
    - The _read-write mutex_ will only lock when you want to write to a variable.
        - This means that there is less congestion when you read frequently, but write occasionally.

  ```rust

  use std::sync::RwLock;
  use once_cell::sync::Lazy;

  // Notice the lazy initialization here.
  static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| {
      return RwLock::new(build_users());
  });

  fn build_users() -> Vec<String> {
      return vec![
          "Alice".to_string(),
          "Bob".to_string(),
      ];
  }

  fn read_line() -> String {
      let mut input = String::new();

      std::io::stdin().read_line(&mut input).unwrap();

      return input.trim().to_string();
  }


  fn main() {
      std::thread::spawn(|| {
          loop {
              println!("Current users (in a thread)");
              let users = USERS.read().unwrap();
              println!("{users:?}");
              std::thread::sleep(std::time::Duration::from_secs(3));
          }
      });

      loop {
          println!("Enter a name to add the user list (or q to quit)");


          let input = read_line();

          if input == "q" {
              break;
          }

          let mut lock = USERS.write().unwrap();
          lock.push(input);
          // Lock will be an automatically released when we go out of scope here (so the next iteration of the loop).
      }
  }
  ```


- When using _mutexes_ you might "lock yourself out" of the variable.
    - Imagine having an infinite loop after acquiring the lock. No other code will ever be able to get the lock unless you "free" the lock.
    - **Keep in mind that the lock is dropped when you move to another scope**!

- In addition to "locking yourself out," one might "poison the lock." This occurs when the thread holding the lock crashed.

  ```rust
  use std::sync::Mutex;

  static MY_SHARED: Mutex<i32> = Mutex::new(3);

  fn posioner() {
      let mut lock = MY_SHARED.lock().unwrap();
      *lock += 1;
      panic!("Strike")
  }

  fn main() {
      let handle = std::thread::spawn(posioner);
      println!("Trying to return from the thread");
      println!("{:?}", handle.join());

      // The mutex is "poisoned"
      let lock = MY_SHARED.lock();
      println!("{lock:?}");
  }
  ```