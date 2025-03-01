# Cargo Workspace

随着项目开发的深入，库 crate 持续增大，你会希望将其进一步拆分成多个库 crate。

Workspace（工作空间）是一组共享同一个 `Cargo.lock` 和输出目录的包，用以帮助管理多个协同开发的包。

Workspace 例子：

- 1 个 Binary crate：`main`
- 2 个 Library Crate：`add_one` 和 `add_two`

```bash
❯ mkdir add && cd add
❯ touch Cargo.toml
❯ cargo new adder
❯ cargo new add_one --lib
❯ cargo new add_two --lib
❯ tree .
.
|-- Cargo.lock              -- 用于锁定所有依赖的具体版本
|-- Cargo.toml              -- 整个项目的配置文件
|-- README.md               -- 描述项目的目的、用法等信息
|-- add_one                 -- 库 crate add_one
|   |-- Cargo.toml          -- 库的元数据和依赖项
|   `-- src                 
|       `-- lib.rs          -- 库的源代码文件
|-- add_two                 -- 库 crate add_two
|   |-- Cargo.toml          -- 库的元数据和依赖项
|   `-- src
|       `-- lib.rs          -- 库的源代码文件
`-- adder                   -- 二进制 crate
    |-- Cargo.toml          -- 二进制 crate 的元数据和依赖项
    `-- src
        `-- main.rs         -- 二进制程序的入口文件
```

工作空间在顶级目录有一个 target 目录，adder 并没有自己的 target 目录。构建结果位于 add/target 而不是 add/adder/target。通过共享一个 target 目录，工作空间可以避免其他 crate 重复构建。

## 依赖外部包

工作空间只在根目录有一个 `Cargo.lock`，而不是在每一个 crate 目录都有 `Cargo.lock`，这确保了所有的 crate 都使用完全相同版本的依赖。

> 如果在 `Cargo.toml` 和 `add_one/Cargo.toml` 中都增加 `rand` crate，则 Cargo 会将其都解析为同一版本并记录到唯一的 `Cargo.lock` 中。

Cargo 只保证在语义版本（Semantic Versioning）规则范围内的兼容性：

1. `add_one` crate 依赖 rand 0.9.0，`adder` crate 依赖 rand 0.9.0（或更高 0.9.1 ）。根据语义版本规则，0.9.0 与 0.9.1 是兼容的，因此这两个 crate 都会使用 0.9.1 。
2. `add_one` crate 依赖 rand 0.9.0，`add_two` crate 依赖 rand 0.8.0。根据语义版本规则，0.9.0 与 0.8.0 是不兼容的，因此 Cargo 会为每个 crate 使用不同版本的 rand 。
