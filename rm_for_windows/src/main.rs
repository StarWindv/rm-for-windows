use anyhow::{anyhow, Context, Result};
use clap::{App, Arg};
use dialoguer::Confirm;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 配置结构体
struct Config {
    force: bool,
    interactive: String, // "none", "each", "once"
    recursive: bool,
    dir: bool,
    verbose: bool,
}

/// 递归删除目录
fn remove_dir_recursive(path: &Path, config: &Config) -> Result<()> {
    for entry in WalkDir::new(path)
        .contents_first(true) // 先处理内容再处理目录
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            remove_file_force(path, config)?;
        } else if path.is_dir() {
            remove_dir_force(path, config)?;
        }
    }
    remove_dir_force(path, config)
}

/// 强制删除文件（处理只读属性）
fn remove_file_force(path: &Path, config: &Config) -> Result<()> {
    if config.verbose {
        println!("removed '{}'", path.display());
    }

    // 处理只读属性
    if let Ok(metadata) = fs::metadata(path) {
        let mut perms = metadata.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            fs::set_permissions(path, perms)?;
        }
    }

    fs::remove_file(path).with_context(|| format!("failed to remove file '{}'", path.display()))
}

/// 强制删除目录（处理只读属性）
fn remove_dir_force(path: &Path, config: &Config) -> Result<()> {
    if config.verbose {
        println!("removed directory '{}'", path.display());
    }

    // 处理只读属性
    if let Ok(metadata) = fs::metadata(path) {
        let mut perms = metadata.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            fs::set_permissions(path, perms)?;
        }
    }

    fs::remove_dir(path).with_context(|| format!("failed to remove directory '{}'", path.display()))
}

/// 删除单个路径（文件或目录）
fn delete_path(path: &Path, config: &Config) -> Result<()> {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_e) if config.force => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    if metadata.is_file() {
        remove_file_force(path, config)
    } else if metadata.is_dir() {
        if config.recursive {
            remove_dir_recursive(path, config)
        } else if config.dir {
            remove_dir_force(path, config)
        } else {
            Err(anyhow!("cannot remove '{}': Is a directory", path.display()))
        }
    } else {
        Err(anyhow!("unsupported file type: {}", path.display()))
    }
}

/// 统计需要删除的项目数量
fn count_deletions(paths: &[PathBuf], config: &Config) -> Result<usize> {
    let mut count = 0;
    for path in paths {
        if !path.exists() {
            if !config.force {
                return Err(anyhow!("cannot remove '{}': No such file or directory", path.display()));
            }
            continue;
        }

        if path.is_file() {
            count += 1;
        } else if path.is_dir() {
            if config.recursive {
                count += WalkDir::new(path).into_iter().count();
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn main() -> Result<()> {
    let matches = App::new("rm-rs")
        .version("0.0.1")
        .author("Author: \x1b[96m星灿长风v(StarWindv)\x1b[0m")
        .about("\x1b[95mLinux rm command for Windows\x1b[0m")
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("Ignore nonexistent files and arguments, never prompt"),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .help("Prompt before every removal"),
        )
        .arg(
            Arg::with_name("I")
                .short("I")
                .help("Prompt once before removing more than three files, or when removing recursively"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .aliases(&["R", "recursive"])
                .help("Remove directories and their contents recursively"),
        )
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .help("Remove empty directories"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Explain what is being done"),
        )
        .arg(
            Arg::with_name("FILE")
                .multiple(true)
                .required(true)
                .help("Files or directories to remove"),
        )
        .get_matches();

    // 构建配置
    let config = Config {
        force: matches.is_present("force"),
        interactive: match (matches.is_present("interactive"), matches.is_present("I")) {
            (true, _) => "each".to_string(),
            (_, true) => "once".to_string(),
            _ => "none".to_string(),
        },
        recursive: matches.is_present("recursive"),
        dir: matches.is_present("dir"),
        verbose: matches.is_present("verbose"),
    };

    // 获取路径参数
    let paths: Vec<PathBuf> = matches
        .values_of("FILE")
        .unwrap()
        .map(PathBuf::from)
        .collect();

    // 处理 -I 参数（一次性提示）
    if config.interactive == "once" {
        let deletion_count = count_deletions(&paths, &config)?;
        if deletion_count > 3 || config.recursive {
            let prompt = format!(
                "Remove {} files and directories recursively?",
                deletion_count
            );
            if !Confirm::new().with_prompt(prompt).interact()? {
                return Ok(());
            }
        }
    }

    // 处理每个路径
    for path in &paths {
        if !path.exists() {
            if !config.force {
                return Err(anyhow!("cannot remove '{}': No such file or directory", path.display()));
            }
            continue;
        }

        // 处理 -i 参数（每个项目提示）
        if config.interactive == "each" {
            let prompt = format!("remove '{}'?", path.display());
            if !Confirm::new().with_prompt(prompt).interact()? {
                continue;
            }
        }

        // 执行删除
        if let Err(e) = delete_path(path, &config) {
            if !config.force {
                return Err(e);
            }
        }
    }

    Ok(())
}
