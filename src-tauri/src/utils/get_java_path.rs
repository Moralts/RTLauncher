/*
RTLauncher, a third-party Minecraft launcher built with the newest
technology and provides innovative funtionalities
Copyright (C) 2025 lutouna

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::env;

pub fn get_java_path() -> Vec<String> {
    let mut java_paths = Vec::new();

    // 检查环境变量 JAVA_HOME
    if let Ok(java_home) = env::var("JAVA_HOME") {
        java_paths.push(java_home);
    }

    // 检查常见的 Java 安装路径
    #[cfg(target_os = "windows")]
    {
        let program_files = vec![
            env::var("ProgramFiles").unwrap_or_default(),
            env::var("ProgramFiles(x86)").unwrap_or_default(),
        ];
        
        for pf in program_files {
            if !pf.is_empty() {
                let java_dir = PathBuf::from(&pf).join("Java");
                if java_dir.exists() {
                    if let Ok(entries) = std::fs::read_dir(java_dir) {
                        for entry in entries.flatten() {
                            if entry.path().is_dir() {
                                java_paths.push(entry.path().to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mac_java_paths = vec![
            "/Library/Java/JavaVirtualMachines",
            "/System/Library/Java/JavaVirtualMachines",
        ];
        
        for path in mac_java_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let jdk_path = entry.path().join("Contents").join("Home");
                    if jdk_path.exists() {
                        java_paths.push(jdk_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let linux_java_paths = vec![
            "/usr/lib/jvm",
            "/usr/java",
            "/opt/java",
        ];
        
        for path in linux_java_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        java_paths.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    java_paths
}
