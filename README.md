# rm-rs - Linux `rm` Command for Windows

<p align="center">
  <img src="https://img.shields.io/badge/Rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
  <img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="Windows Compatible">
</p>

`rm-rs` 是一个为 Windows 系统实现的 Linux `rm` 命令工具，专为熟悉 Linux 命令的用户设计。它完美复刻了 Linux 中 `rm` 命令的行为模式，让您在 Windows 环境中也能使用熟悉的文件删除操作。

> ⚠️ **注意**：Windows PowerShell 自带 `rm` 命令（作为 `Remove-Item` 的别名）。

## ✨ 功能特性

- 🔄 完美模拟 Linux `rm` 命令行为
- 🔧 支持所有主要参数：`-f`, `-r`, `-d`, `-v`, `-i`, `-I`
- 📁 自动处理只读文件和目录
- ✅ 交互式删除确认（支持单文件提示和批量提示）
- 📊 详细模式显示操作过程
- 🛡️ 安全防护机制防止误删


## 🛠️ 使用说明

```bash
rm [OPTIONS] <FILE>...
```

### 选项说明
| 参数 | 简写 | 说明 |
|------|------|------|
| `--force` | `-f` | 忽略不存在的文件，不显示错误信息 |
| `--interactive` | `-i` | 每次删除前都要求确认 |
| `--I` | `-I` | 删除超过3个文件或递归删除前提示一次 |
| `--recursive` | `-r`, `-R` | 递归删除目录及其内容 |
| `--dir` | `-d` | 删除空目录 |
| `--verbose` | `-v` | 显示详细操作信息 |

### 使用示例

1. **删除单个文件**：
```bash
rm file.txt
```

2. **强制删除文件（忽略不存在文件）**：
```bash
rm -f *.log
```

3. **递归删除目录及其内容**：
```bash
rm -r node_modules
```

4. **交互式删除（每个文件确认）**：
```bash
rm -i *.tmp
```

5. **删除空目录**：
```bash
rm -d empty_dir
```

6. **详细模式（显示操作过程）**：
```bash
rm -rv old_project
```

## ⚖️ 与 PowerShell `rm` 的区别

| 特性 | `rm-rs` | PowerShell `rm` |
|------|---------|----------------|
| 命令名称 | `rm` | `rm` |
| 行为模式 | 严格遵循 Linux `rm` | PowerShell `Remove-Item` |
| 参数兼容性 | 支持 Linux `rm` 所有参数 | 使用 PowerShell 参数格式 |
| 只读处理 | 自动清除只读属性 | 需要额外参数处理只读文件 |
| 交互模式 | 支持 `-i` 和 `-I` | 使用 `-Confirm` |
| 递归删除 | `-r` 或 `-R` | `-Recurse` |
| 空目录删除 | `-d` 选项 | 自动删除空目录 |

## ⚠️ 重要注意事项

1. **递归删除风险**：
   - `rm -r` 会永久删除目录及其所有内容
   - 使用前请确认目标路径是否正确
   - 建议先在重要目录使用 `-i` 交互模式

2. **文件恢复**：
   - 删除的文件不会进入回收站
   - 删除操作不可逆
   - 重要文件建议先备份

3. **权限要求**：
   - 需要管理员权限才能删除系统保护文件
   - 普通文件不需要额外权限

## 📥 安装指南

### 前置要求
- 安装 [Rust](https://www.rust-lang.org/tools/install) 工具链

### 安装步骤
打开终端（CMD 或 PowerShell）

1. 克隆仓库：
```bash
git clone https://github.com/starwindv/rm-for-windows
```
```bash
cd rm-for-windows/rm_for_windows
```

2. 构建项目：
```bash
cargo build --release
```

3. 使用工具：
将`target/release/rm.exe`移动到您喜欢的位置并添加到环境变量中，重启shell以使用类似Linux的`rm`命令

## 📜 开源许可

本项目采用 [MIT 许可证](./LICENSE)发布。

## 👥 贡献指南

欢迎提交 Issue 和 Pull Request！
---

**温馨提示**：操作文件删除命令时请务必谨慎，错误的删除操作可能导致数据永久丢失！💾🚫
