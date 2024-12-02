# Learning Async Programming with Tokio

Tokio is the go-to choice for async programming for rust. it -

* Abstracts away low-level concurrency details, so you don't need to worry about thread management.
* Enables you to write asynchronous code that can run concurrently with other tasks.
* Allows you to handle multiple tasks at the same time, improving performance and responsiveness.

### [Hello Tokio](./hello_tokio/src/main.rs)
This is the entry point of the program. It defines several asynchronous functions:

* `hello`: Prints "hello" to the console.
* `test2`: Prints "second task" to the console.
* `test1`: Prints "first task" to the console.
* `counter`: A counter that prints numbers from 0 to 9, yielding control back to the event loop with `tokio::task::yield_now().await;`.

The `main` function is marked with the `#[tokio::main]` attribute, which indicates that it's the entry point of the program. It calls each of these functions asynchronously:

* `hello().await;`
* `join!(test1(), test2());` - joins two tasks together and waits for both to complete.
* `let _ = join!(spawn(counter()), spawn(counter()));` - creates two new tasks that run concurrently, and waits for them both to complete using another `join!` macro.

### [Blocking Tasks](./blocking_tasks/src/main.rs)
Let's say we have a couple tasks. Task 1 takes 100ms to complete and the other task takes 200ms. We want to spawn both tasks and delegate the time/resource consuming tasks to available threads.

* `delayed_task` function simulates the above scenario. It takes two arguments
  * `task`: the number of task
  * `time`: the amount of time the thread will be occupied in ms


We're using tokio's `spawn_blocking` function to move over all the time/resource consuming task. This function lives under `tokio::task` namespace.
