# ArdanLabs Rust

Finished 15_1_14 28:32

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