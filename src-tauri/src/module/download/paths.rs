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

use walkdir;

// 路径管理
pub struct MinecraftPaths {
    pub base_dir: std::path::PathBuf,
    pub versions_dir: std::path::PathBuf,
    pub libraries_dir: std::path::PathBuf,
    pub assets_dir: std::path::PathBuf,
}

impl MinecraftPaths {
    pub fn new() -> Self {
        // 游戏文件保存路径 - 根据操作系统设置默认路径
        let base_dir = if cfg!(target_os = "windows") {
            std::path::PathBuf::from("C:\\.minecraft")
        } else if cfg!(target_os = "macos") {
            dirs::home_dir()
                .unwrap_or_default()
                .join("Library")
                .join("Application Support")
                .join("RTLauncher")
        } else {
            // Linux
            dirs::home_dir().unwrap_or_default().join(".minecraft")
        };

        Self {
            versions_dir: base_dir.join("versions"),
            libraries_dir: base_dir.join("libraries"),
            assets_dir: base_dir.join("assets"),
            base_dir,
        }
    }

    pub fn get_version_dir(&self, version_id: &str) -> std::path::PathBuf {
        self.versions_dir.join(version_id)
    }

    pub fn get_natives_dir(&self, version_id: &str) -> std::path::PathBuf {
        self.get_version_dir(version_id)
            .join(format!("{}-natives", version_id))
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.base_dir)?;
        std::fs::create_dir_all(&self.versions_dir)?;
        std::fs::create_dir_all(&self.libraries_dir)?;
        std::fs::create_dir_all(&self.assets_dir)?;
        Ok(())
    }

    // 获取绝对路径-公共方法
    pub fn get_absolute_path(&self, path: std::path::PathBuf) -> String {
        path.canonicalize()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .trim_start_matches(r"\\?\")
            .to_string()
    }

    // 获取libraries目录下所有jar文件的路径
    pub fn get_libraries_classpath(&self) -> Vec<String> {
        walkdir::WalkDir::new(&self.libraries_dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "jar"))
            .map(|entry| self.get_absolute_path(entry.path().to_path_buf()))
            .collect()
    }
}
