# SUMMARY

[关于](README.md)

- [基础数据类型](fundamental/index.md)
    - [整数类型](fundamental/integer.md)
    - [浮点类型](fundamental/floating-point.md)
    - [布尔类型](fundamental/bool.md)
    - [字符 char](fundamental/char.md)
    - [数组 array](fundamental/array.md)
    - [元组 tuple](fundamental/tuple.md)
    - [unit](fundamental/unit.md)
    - [切片 slice](fundamental/slice.md)
    - [字符串切片 str](fundamental/str.md)
    - [指针 pointer](fundamental/pointer.md)
    - [类型转换](fundamental/cast.md)
- [结构体 Struct](struct/index.md)
    - [定义方法 impl](struct/impl.md)
    - [声明相关的常量 const](struct/constant.md)
    - [内存布局 memory layout](struct/layout.md)
    - [泛型 Generics](struct/generics.md)
    - [内部可变性 Interior Mutability](struct/interior-mutability.md)
- [枚举与模式匹配 Enums and Patterns](enum/index.md)
    - [定义方法 impl](enum/impl.md)
    - [Option](enum/option.md)
    - [Result](enum/result.md)
    - [Patterns](enum/patterns.md)
- [表达式 Expression](expression/index.md)
    - [控制流 Flow control](expression/flow-control.md)
- [库与模块 crates and modules](crates/index.md)
    - [文档与注释](crates/doc.md)
    - [rustup工具](crates/rustup.md)
    - [Cargo 基础](crates/guide.md)
    - [属性 attr](crates/attributes.md)
    - [配置](crates/config.md)
    - [包引用](crates/references.md)
    - [构建脚本 build.rs](crates/build.md)
    - [工具](crates/tools.md)
    - [条件编译](crates/cfg.md)
    - [跨平台 Cross Platform](crates/cross-platform.md)
    - [交叉编译 Cross Compilation](crates/cross-complation.md)
    - [静态编译 Static Compilation](crates/static-compilation.md)
- [所有权 Ownership](ownership/index.md)
    - [所有权 Ownership](ownership/ownership.md)
    - [转移所有权](ownership/move.md)
    - [不需要转移所有权](ownership/copy-type.md)
    - [使用 Rc 与 Arc 共享所有权](ownership/rc-arc.md)
- [引用与生命周期 Reference and Lifetime](reference/index.md)
    - [对值的引用](reference/ref-to-value.md)
    - [引用的操作](reference/ref-ops.md)
    - [引用的安全性](reference/ref-safety.md)
    - [生命周期](reference/lifetime.md)
    - [标注生命周期 Lifetime marker](reference/lifetime-marker.md)
    - [标注泛型生命周期](reference/generic-lifetime-marker.md)
    - [static 静态生命周期](reference/lifetime-static.md)
    - [Non-lexical lifetimes NLL](reference/nll.md)
- [接口 Trait](trait/index.md)
    - [Trait](trait/trait.md)
    - [Derive: 自动继承常见的 trait](trait/derive.md)
    - [关联类型 Associated Types](trait/associated-types.md)
    - [关联常量 Associated Consts](trait/associated-consts.md)
    - [静态派发与动态派发 Static Dispatch and Dynamic Dispatch](trait/dispatch.md)
    - [虚表 vtable](trait/vtable.md)
    - [Trait Objects](trait/trait-object.md)
    - [泛型 Generics](trait/generics.md)
    - [隐藏名称 Name Hiding](trait/name-hiding.md)
- [操作符重载 Operator Overloading](ops/index.md)
    - [算术与比特位操作符 Arithmetic and bitwise operators](ops/arith-bit-ops.md)
    - [Range](ops/range.md)
    - [索引 Index 与 IndexMut](ops/index-index-mut.md)
    - [相等与比较 Eq and Ord](ops/eq-ord.md)
- [迭代器 Iterators](iterator/index.md)
    - [Iterator 与 IntoIterator](iterator/iterator-into-iterator.md)
    - [ExactSizeIterator](iterator/exact-size-iterator.md)
    - [DoubleEndedIterator](iterator/double-ended-iterator.md)
    - [FromIterator](iterator/from-iterator.md)
    - [Extend](iterator/extend.md)
    - [求和 Sum](iterator/sum.md)
    - [乘积 Product](iterator/product.md)
    - [迭代器的适配器 Adapters](iterator/adapters.md)
    - [使用 Iterator](iterator/consuming-iterators.md)
- [常用的 Traits](common-traits/index.md)
    - [Drop](common-traits/drop.md)
    - [Default](common-traits/default.md)
    - [Write](common-traits/write.md)
    - [Debug 与 Display](common-traits/debug-display.md)
    - [FromStr 与 ToString](common-traits/from-str-to-string.md)
    - [Clone 与 Copy](common-traits/clone-copy.md)
    - [ToOwned](common-traits/to-owned.md)
    - [From 与 Into](common-traits/from-into.md)
    - [TryFrom 与 TryInto](common-traits/try-from-try-into.md)
    - [AsRef 与 AsMut](common-traits/as-ref-as-mut.md)
    - [Deref 与 DerefMut](common-traits/deref-deref-mut.md)
    - [Borrow 与 BorrowMut](common-traits/borrow-borrow-mut.md)
    - [Send 与 Sync](common-traits/send-sync.md)
    - [Sized](common-traits/sized.md)
    - [Hash 与 Hasher](common-traits/hash-hasher.md)
    - [反射 Any](common-traits/any.md)
- [函数与闭包 Closure](closure/index.md)
    - [函数 function](closure/function.md)
    - [泛型函数 Generics](closure/generics.md)
    - [重载 Overloading](closure/function-overloading.md)
    - [闭包 Closure](closure/closure.md)
    - [Fn, FnMut 与 FnOnce](closure/fn-mut-once.md)
    - [回调函数](closure/callback.md)
    - [函数声名示例](closure/declaration.md)
- [容器 Collections](collections/index.md)
    - [列表 Vector](collections/vector.md)
    - [字符串 String](collections/string.md)
    - [VecDeque](collections/deque.md)
    - [LinkedList](collections/linked-list.md)
    - [BinaryHeap](collections/binary-heap.md)
    - [HashMap & HashSet](collections/hash-map.md)
    - [BTreeMap & BTreeSet](collections/btree-map.md)
    - [BitVector](collections/bit-vector.md)
- [错误处理 Error handling](error-handling/index.md)
    - [Panic](error-handling/panic.md)
    - [Panic Hook](error-handling/panic-hook.md)
    - [Result](error-handling/result.md)
    - [Option](error-handling/option.md)
    - [Error trait](error-handling/error-trait.md)
    - [自定义错误类型](error-handling/custom.md)
    - [thiserror 库](error-handling/thiserror.md)
    - [anyhow 库](error-handling/anyhow.md)
- [指针与内存管理](mem/index.md)
    - [原始指针 Raw pointer](mem/raw-pointer.md)
    - [胖指针 Fat pointer](mem/fat-pointer.md)
    - [智能指针 Smart pointers](mem/smart-pointers/index.md)
        - [使用 Box 分配堆内存](mem/smart-pointers/box.md)
        - [Rc 与 Weak](mem/smart-pointers/rc.md)
        - [RefCell 与内部可变性 Interior Mutability](mem/refcell.md)
        - [循环引用 Reference Cycle](mem/smart-pointers/ref-cycle.md)
    - [指针汇总 Summary of pointers](mem/pointer-summary.md)
    - [NonNull](mem/nonnull.md)
    - [Unique](mem/unique.md)
    - [PhantomData](mem/phantom.md)
    - [写时复制 Cow](mem/cow.md)
    - [Cell](mem/cell.md)
    - [OnceCell](mem/once-cell.md)
    - [LazyCell](mem/lazy-cell.md)
    - [UnsafeCell](mem/unsafe-cell.md)
    - [Pin 以及 Unpin](mem/pin.md)
    - [ManuallyDrop](mem/manually-drop.md)
    - [forget](mem/forget.md)
    - [自引用](mem/ref-to-self.md)
    - [transmute](mem/transmute.md)
    - [内存布局 Data Layout](mem/data-layout.md)
    - [初始化内存 Initialize Memory](mem/init.md)
    - [mem 模块其它功能](mem/mem-module.md)
    - [内存分配器](mem/allocator.md)
    - [jemalloc](mem/jemalloc.md)
    - [自定义内存分配器](mem/customize-allocator.md)
    - [内存模型 Memory Model](mem/memory-model.md)
    - [工具](mem/tools.md)
- [并发编程](concurrency/index.md)
    - [并发编程模型 concurrency models](concurrency/concurrent-programming-model.md)
    - [多进程编程](concurrency/multi-processing.md)
    - [多线程与线程池](concurrency/multi-threads.md)
    - [消息传递 message passing](concurrency/message-passing.md)
    - [使用 Channel 传递消息](concurrency/channel.md)
    - [共享内存 Shared memory](concurrency/shared-memory.md)
    - [Send and Sync](concurrency/send-and-sync.md)
    - [线程间共享可变更的状态 Mutable State](concurrency/shared-mutable-state/index.md)
        - [互斥琐 Mutex 与 MutexGuard](concurrency/shared-mutable-state/mutex.md)
        - [在用户空间实现的快速锁 futex](concurrency/shared-mutable-state/futex.md)
        - [读写锁 RwLock 与 RwLockGuards](concurrency/shared-mutable-state/rwlock.md)
        - [OnceLock](concurrency/shared-mutable-state/once_lock.md)
        - [LazyLock](concurrency/shared-mutable-state/lazy-lock.md)
        - [Barrier](concurrency/shared-mutable-state/barrier.md)
        - [条件变量 CondVar](concurrency/shared-mutable-state/condvar.md)
        - [Memory Order](concurrency/shared-mutable-state/memory-order.md)
        - [原子操作 Atomic](concurrency/shared-mutable-state/atomic.md)
        - [Arc 与 Weak](concurrency/shared-mutable-state/arc.md)
        - [全局变量 Global Variables](concurrency/shared-mutable-state/global-variables.md)
        - [Thread Local Storage](concurrency/shared-mutable-state/thread-local-storage.md)
    - [参考资料](concurrency/references.md)
- [异步 async](async/index.md)
    - [第一个 async 程序](async/async-dance.md)
    - [理解 Futures 与 Tasks](async/futures-and-tasks.md)
    - [理解 async/await](async/async-await.md)
    - [生命周期与内存固定 Lifetimes and Pinning](async/lifetimes-pinning.md)
    - [标准库中的 future 模块](async/future-module.md)
    - [标准库中的 task 模块](async/task-module.md)
    - [futures 库](async/futures/index.md)
        - [Future 的执行者 Executors](async/futures/executors.md)
        - [管理任务 Tasks](async/futures/tasks.md)
        - [一次执行多个 Futures](async/futures/multi-futures.md)
        - [使用 channel 传递信息](async/futures/channel.md)
        - [异步 IO](async/futures/io.md)
        - [Stream 与迭代器](async/futures/stream-and-iterator.md)
        - [同步原语 Synchronization Primitives](async/futures/synchronization.md)
    - [mio 库](async/mio.md)
    - [tokio 库](async/tokio.md)
    - [async-std 库](async/async-std.md)
    - [smol 库](async/smol.md)
    - [如何调试 debug](async/debug.md)
    - [IO uring](async/io-uring.md)
- [宏 Macro](macro/index.md)
    - [宏 Macro](macro/macro.md)
    - [宏示例](macro/macro-example.md)
    - [过程宏 proc-macro](macro/proc-macro.md)
    - [syn 库](macro/syn.md)
- [不安全的代码 Unsafe code](unsafe/index.md)
- [FFI](ffi/index.md)
    - [c ffi](ffi/c.md)
    - [与C语言兼容的基础数据类型](ffi/raw-types.md)
    - [与C语言兼容的结构体](ffi/c-struct.md)
    - [c string](ffi/c-string.md)
    - [声明外部函数以及变量](ffi/declare-function.md)
    - [使用外部库中的函数](ffi/function-from-libraries.md)
    - [使用 CC 编译C代码并链接到内部](ffi/cc.md)
    - [自动生成语言绑定](ffi/binding.md)
- [测试 Test](test/index.md)
    - [文档测试 Doc Test](test/doc-test.md)
    - [单元测试 Unit Test](test/unit-test.md)
    - [集成测试 Integrated Test](test/integerated-test.md)
    - [代码覆盖率 Code Coverage](test/code-coverage.md)
- [调试与性能优化](perf/index.md)
    - [调试 Debug](perf/debug.md)
    - [gdb](perf/gdb.md)
    - [sanitizers](perf/sanitizers.md)
    - [valgrind](perf/valgrind.md)
    - [Benchmark](perf/benchmark/index.md)
        - [divan 库](perf/benchmark/divan.md)
        - [criterian 库](perf/benchmark/criterian.md)
    - [perf-tools](perf/perf-tools.md)
    - [Link Time Optimization: lto](perf/lto.md)
    - [Profile-guided Optimization: pgo](perf/pgo.md)
- [常用工具](tools/index.md)
    - [sccache](tools/sccache.md)
    - [clippy](tools/clippy.md)
    - [rust analyzer](tools/rust-analyzer.md)
- [网络编程 network programming](network/index.md)
    - [HTTP 请求: reqwest 库](network/reqwest.md)
    - [Web 服务: hyper 库](network/hyper.md)
    - [基于 Actor 模型实现的 Web 服务: actix-web 库](network/actix-web.md)
    - [websocket 库: tokio-tungstenite](network/websocket.md)
    - [openssl 的原生实现: rustls 库](network/rustls.md)
    - [QUIC 协议: quinn 库](network/quinn.md)
    - [gRPC 服务: tonic 库](network/tonic.md)
- [WebAssembly](wasm/index.md)
    - [WebAssembly 系统接口 wasi](wasm/wasi.md)
    - [基于 LLVM 的编译器 emscripten](wasm/emscripten.md)
    - [编译器与工具链 binaryen](wasm/binaryen.md)
    - [wasmtime 运行时](wasm/wasmtime.md)
    - [wasmer 运行时](wasm/wasmer.md)
    - [WasmEdge 运行时](wasm/WasmEdge.md)
    - [反编译与调试](wasm/disassemble-and-debug.md)
- [web programming](web/index.md)
    - [web-sys 库](web/web-sys.md)
    - [wasm-bindgen 库](web/wasm-bindgen.md)
    - [wasm-pack 工具](web/wasm-pack.md)
    - [trunk 工具](web/trunk.md)
    - [yew 库](web/yew/index.md)
    - [wgpu 库](web/wgpu.md)
    - [zu 库](web/zu.md)
    - [其它工具](web/tools.md)
- [标准库](std/index.md)
    - [格式化输出 format](std/format.md)
    - [IO](std/io.md)
    - [num](std/num.md)
    - [Hint](std/hint.md)
- [设计模式](design-patterns/index.md)
    - [创建型 Creational](design-patterns/creational/index.md)
        - [简单工厂模式 Simple factory](design-patterns/creational/simple-factory.md)
        - [工厂方法模式 Factory method](design-patterns/creational/factory-method.md)
        - [抽象工厂模式 Abstract factory](design-patterns/creational/abstract-factory.md)
        - [建造者模式 Builder](design-patterns/creational/builder.md)
        - [原型模式 Prototype](design-patterns/creational/prototype.md)
        - [单例模式 Singleton](design-patterns/creational/singleton.md)
    - [结构型 Structural](design-patterns/structural/index.md)
        - [适配器模式 Adapter](design-patterns/structural/adapter.md)
        - [桥梁模式 Bridge](design-patterns/structural/bridge.md)
        - [组合模式 Composite](design-patterns/structural/composite.md)
        - [装饰器模式 Decorator](design-patterns/structural/decorator.md)
        - [门面模式 Facade](design-patterns/structural/facade.md)
        - [享元模式 Flyweight](design-patterns/structural/flyweight.md)
        - [代理模式 Proxy](design-patterns/structural/proxy.md)
    - [行为型 Behavioral](design-patterns/behavioral/index.md)
        - [责任链模式 Chain of Responsibilities](design-patterns/behavioral/chain-of-responsibilities.md)
        - [命令行模式 Command](design-patterns/behavioral/command.md)
        - [迭代器模式 Iterator](design-patterns/behavioral/iterator.md)
        - [中介者模式 Mediator](design-patterns/behavioral/mediator.md)
        - [备忘录模式 Memento](design-patterns/behavioral/memento.md)
        - [观察者模式 Observer](design-patterns/behavioral/observer.md)
        - [访问者模式 Visitor](design-patterns/behavioral/visitor.md)
        - [策略模式 Strategy](design-patterns/behavioral/strategy.md)
        - [装态模式 State](design-patterns/behavioral/state.md)
        - [模板方法模式 Template Method](design-patterns/behavioral/template-method.md)
- [数据结构与算法](algs.md)
- [新特性 unstable features](unstable/index.md)
    - [portable simd](unsafe/portable-simd.md)
- [第三方库 crates.io](crate.io/index.md)
    - [时间 Time](crate.io/time.md)
    - [数据序列化 serde](crate.io/serde.md)
- [参考资料](ref.md)
