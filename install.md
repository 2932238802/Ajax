src/
├── main.rs
│
├── ecs/
│   ├── mod.rs
│   ├── entity.rs
│   ├── component.rs
│   ├── storage.rs
│   ├── world.rs
│   └── query.rs
│
├── components/
│   ├── mod.rs
│   ├── health.rs
│   ├── position.rs
│   ├── inventory.rs
│   ├── hunger.rs
│   └── thirst.rs
│
├── resources/
│   ├── mod.rs
│   ├── time.rs
│   ├── map.rs
│   └── weather.rs
│
├── systems/
│   ├── mod.rs
│   ├── movement.rs
│   ├── combat.rs
│   ├── survival.rs
│   └── craft.rs
│
├── entities/
│   ├── mod.rs
│   ├── player.rs
│   └── creature.rs
│
├── commands/
│   ├── mod.rs
│   ├── command.rs
│   ├── parser.rs
│   └── executor.rs
│
└── game/
    ├── mod.rs
    ├── game.rs
    └── schedule.rs


<<<<<<< Updated upstream
- Fn 
    - Fn 是一个 trait，表示可以像函数一样调用的类型
- fn
    - fn 是函数指针类型，类似 C 的函数指针
    - 是一个具体的类型，大小固定（一个指针）
    - 可以直接存储和传递
    - 没有运行时开销
    - 例如:
        ```rust
        // 定义一个函数
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        // fn 类型的变量
        let f: fn(i32, i32) -> i32 = add;

        println!("{}", f(1, 2)); // 3

        // 不捕获环境的闭包也可以转为 fn
        let f: fn(i32) -> i32 = |x| x + 1;
        println!("{}", f(5)); // 6

        // 但捕获环境的闭包不能转为 fn
        let y = 10;
        let f: fn(i32) -> i32 = |x| x + y; // ❌ 编译错误
        ```
- Fn
    - 例如
        ```rust
        Fn      // 可以多次调用，不可变借用环境
        FnMut   // 可以多次调用，可变借用环境
        FnOnce  // 只能调用一次，获取环境所有权
        ```
        - 是一个 trait，不是具体类型
        - 可以表示任何可调用的东西：函数、闭包、实现了 Fn 的结构体
        - 大小不确定，通常需要 Box<dyn Fn(...)> 或泛型
        - 可以捕获环境
    - 
=======
>>>>>>> Stashed changes
