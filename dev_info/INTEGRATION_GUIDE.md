# RTLauncher - 功能集成说明

## 已集成的功能

本项目已成功集成以下Minecraft启动器核心功能:

### 1. 登录功能 (`api/login.rs`)
- **功能**: 微软账户OAuth登录
- **Tauri命令**: `get_code`
- **说明**: 
  - 在本地40323端口启动HTTP服务器
  - 打开浏览器进行微软账户授权
  - 获取OAuth授权码用于后续登录

**前端调用示例**:
```typescript
import { getAuthCode } from "@/lib/tauri-commands";

const result = await getAuthCode();
console.log(result.code); // 授权码
```

### 2. 游戏下载功能 (`module/download/`)

#### 2.1 获取版本清单
- **Tauri命令**: `get_version_manifest`
- **功能**: 获取所有可用的Minecraft版本列表

**前端调用示例**:
```typescript
import { getVersionManifest } from "@/lib/tauri-commands";

const manifest = await getVersionManifest();
console.log(manifest.latest.release); // 最新正式版
console.log(manifest.versions); // 所有版本列表
```

#### 2.2 下载游戏文件
- **Tauri命令**: `dwl_version_manifest`
- **功能**: 
  - 下载指定版本的客户端JAR
  - 下载所需的库文件(libraries)
  - 下载游戏资源文件(assets)
  - 下载并解压natives文件
  - SHA1校验确保文件完整性

**前端调用示例**:
```typescript
import { downloadVersion } from "@/lib/tauri-commands";

// 从版本清单中获取版本URL
const versionUrl = "https://piston-meta.mojang.com/v1/packages/.../1.21.4.json";
const result = await downloadVersion(versionUrl);
```

#### 2.3 文件路径管理 (`paths.rs`)
- 跨平台路径支持(Windows/macOS/Linux)
- 自动创建必要的目录结构
- 默认路径:
  - Windows: `C:\.minecraft`
  - macOS: `~/Library/Application Support/minecraft`
  - Linux: `~/.minecraft`

#### 2.4 解压功能 (`decompression.rs`)
- 自动解压natives库文件
- 根据操作系统和CPU架构过滤文件
- 支持32位/64位系统

### 3. 游戏启动功能 (`module/start_game/`)

- **Tauri命令**: `stg`
- **功能**:
  - 自动检测系统已安装的Java
  - 生成完整的JVM启动参数
  - 配置classpath和natives路径
  - 启动Minecraft游戏进程

**前端调用示例**:
```typescript
import { startGame } from "@/lib/tauri-commands";

const result = await startGame({
  startup_parameter: "-Xmx2G -Xms1G", // 内存参数
  version_id: "1.21.4",
  java_version: "17", // 需要的Java版本
  asset_index_id: "17", // 资源索引ID
  username: "Player",
});
```

### 4. 工具模块 (`utils/`)

#### 4.1 HTTP请求工具 (`request.rs`)
- 封装reqwest进行HTTP请求
- 用于下载游戏文件和获取API数据

#### 4.2 Java路径检测 (`get_java_path.rs`)
- 自动检测系统中安装的Java
- 支持多平台Java路径查找
- 检查JAVA_HOME环境变量
- 扫描常见安装目录

## 项目结构

```
src-tauri/
├── src/
│   ├── lib.rs                    # 主入口,注册所有Tauri命令
│   ├── main.rs                   # 程序启动入口
│   ├── api/
│   │   ├── mod.rs
│   │   └── login.rs              # 登录功能
│   ├── module/
│   │   ├── mod.rs
│   │   ├── download/
│   │   │   ├── mod.rs
│   │   │   ├── dwl_main.rs       # 下载主逻辑
│   │   │   ├── decompression.rs  # 解压功能
│   │   │   └── paths.rs          # 路径管理
│   │   └── start_game/
│   │       ├── mod.rs
│   │       └── stg_main.rs       # 启动游戏
│   └── utils/
│       ├── mod.rs
│       ├── request.rs            # HTTP请求工具
│       └── get_java_path.rs      # Java检测
└── Cargo.toml                    # Rust依赖配置

RTL-WebUI/
├── lib/
│   ├── tauri-types.ts            # TypeScript类型定义
│   └── tauri-commands.ts         # Tauri命令封装
└── app/
    └── launcher/
        └── page.tsx              # 启动器示例页面
```

## 依赖说明

### Rust依赖 (Cargo.toml)
```toml
reqwest = "0.12.12"      # HTTP客户端
tokio = "1.43.0"         # 异步运行时
futures = "0.3.31"       # 异步流处理
sha1 = "0.10.6"          # SHA1校验
zip = "2.2.2"            # ZIP解压
os_info = "3.9.2"        # 系统信息
walkdir = "2.5.0"        # 目录遍历
url = "2.5.4"            # URL解析
open = "5.3.2"           # 打开浏览器
dirs = "5.0"             # 跨平台目录
```

## 使用步骤

### 1. 构建项目
```bash
cd /Users/chuqi/RTLauncher
cargo build --manifest-path=src-tauri/Cargo.toml
```

### 2. 开发运行
```bash
# 启动Tauri开发服务器
pnpm tauri dev

# 或者分别启动
cd RTL-WebUI
pnpm dev

# 另一个终端
cd src-tauri
cargo run
```

### 3. 访问示例页面
打开浏览器访问示例启动器页面: `/launcher`

## 注意事项

1. **路径配置**: 
   - 默认游戏文件保存在 `.minecraft` 目录
   - 可在 `paths.rs` 中修改默认路径

2. **Java要求**:
   - 不同Minecraft版本需要不同Java版本
   - 1.17+需要Java 17
   - 1.12-1.16需要Java 8

3. **网络要求**:
   - 下载功能需要访问Mojang服务器
   - 建议使用稳定的网络连接

4. **系统权限**:
   - 需要文件读写权限
   - 登录功能需要绑定本地端口40323

## 下一步开发建议

1. **账户系统完善**:
   - 实现完整的微软账户登录流程
   - 存储和管理用户Token
   - 支持多账户切换

2. **UI优化**:
   - 添加下载进度显示
   - 实时日志输出
   - 游戏运行状态监控

3. **功能扩展**:
   - Mod管理
   - 材质包/资源包管理
   - 服务器列表管理
   - 游戏设置管理

4. **性能优化**:
   - 断点续传
   - 并发下载优化
   - 缓存管理

## 技术栈

- **后端**: Rust + Tauri 2.0
- **前端**: Next.js 14 + React + TypeScript
- **UI**: Shadcn/ui + Tailwind CSS
- **异步**: Tokio
- **HTTP**: Reqwest

## 许可证

本项目基于原RTLauncher项目移植,遵循GPL-3.0许可证。
