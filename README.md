# ats-intc

This is the driver of ats-intc(Asynchronous task scheduler and Interrupt controller)

### Driver

The Driver is the `AtsIntc` structure. It is strictly follow the designation of [ATS-INTC register space](https://ats-intc.github.io/docs/ats-intc/01register-space/). When you need to use the `ATS-INTC` in a specific process, you must create a new `AtsIntc` instance according the process id. 

```rust
/// The basic address of the ATS-INTC
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The ATS-INTC driver of kernel
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);
/// The ATS-INTC driver of process1
static ATSINTC1: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR + 1 * 0x1000);
```

### Task & TaskRef

#### Task

The `Task` structure is shown below. The fut can be any object that implemented the `Future` trait, maybe a process, a thread or a coroutine. 

Task must be created by the `Task::new` function, which will apply a space in the heap by using `Arc` smart pointer. However, we forget it by using `Task::as_ref` function and return the `TaskRef` instance. After doing these, we can store the `TaskRef` instance into the `ATS-INTC`.

When you fetch a `TaskRef` from the `ATS-INTC`, you must use the `Task::from_ref` function to convert it into `Arc<Task>`. So you can utilize the ownership of Rust, once the task has been done, it will be dropped automatically, once the task is blocked, you must use the `Task::as_ref` function again to prevent being recycled. **We have deal the ownership by encapsule the `Task::from_ref` and `Task::as_ref` into the `TaskRef::poll` function. So you don't need to worry about the ownership.**

```Rust
#[repr(C)]
pub struct Task {
    pub state: AtomicU32,
    pub priority: AtomicU32,
    pub atsintc: &'static AtsIntc,
    pub task_type: TaskType,
    pub fut: AtomicCell<Pin<Box<dyn Future<Output = i32> + 'static + Send + Sync>>>,
}
```

#### TaskRef

This structure is the raw pointer of the `Task` in the heap. It is only implemented the `Clone` traits, so it will be influenced by the ownership of Rust. It must be used carefully, otherwise it can cause some unexpected consequence.

You can get a valid `TaskRef` in two ways:

1. Using the `Task::new` function, you can use it immediately or push it into the `ATS-INTC`.
2. Fetching a valid `TaskRef` from the `ATS-INTC`.

**Warn: Don't use the clone function of `TaskRef` to do something else. What you need to do is using its `poll` function.**

Besides, you can get a virtual `TaskRef` by using the `TaskRef::virt_task`, it is only a pointer that has no data on the heap, so it is unsafe. **It is only used in the test case.**

### Waker

We implement a customized waker instead of some existed waker.

In the `TaskRef::poll` function, when a `TaskRef` is being polled, it will firstly create a waker by using `waker::from_task` function. This behavior will record a replica `TaskRef` into the waker. When the `Task`'s poll result is ready, the waker will do nothing and be dropped automatically. Once the result is pending, the `wake` function of the waker will be called at some time. A replica `TaskRef` will be pushed into the `ATS-INTC`.

### PAC

This IO registers are specificed by `svd`, if you want to modify the specification, you must install the required toolchain with follow commands:

```bash
cargo install svd2rust
cargo install form
```

Then you should create a new Peripheral access crate, for example:

```bash
cargo new xxx-pac --lib
```

The `cargo.toml` must has the follow dependencies:

```rust
[dependencies]
critical-section = { version = "1.0", optional = true }
cortex-m = "0.7.6"
cortex-m-rt = { version = "0.7.3", optional = true }
vcell = "0.1.2"

[features]
rt = ["cortex-m-rt/device"]
```

After you have done the above operations, you can modify the svd file (The svd file should not under the `src` directory). After finishing modifying the svd file, you can use the `genpac.py` script to generate the hareware pac crate. 

```bash
python3 $(target_svd_filename)
```
