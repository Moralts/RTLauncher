### ✅ 已完成的功能
1. **登录系统** - 微软账户OAuth登录
2. **游戏下载** - 完整的Minecraft版本下载(客户端、库文件、资源文件)
3. **游戏启动** - 自动配置JVM参数并启动游戏
4. **工具模块** - HTTP请求、Java检测等

前端调用Tauri命令

// 1. 登录
import { getAuthCode } from "@/lib/tauri-commands";
const loginResult = await getAuthCode();

// 2. 获取版本列表
import { getVersionManifest } from "@/lib/tauri-commands";
const versions = await getVersionManifest();

// 3. 下载游戏
import { downloadVersion } from "@/lib/tauri-commands";
await downloadVersion(versionUrl);

// 4. 启动游戏
import { startGame } from "@/lib/tauri-commands";
await startGame({
  startup_parameter: "-Xmx2G -Xms1G",
  version_id: "1.21.4",
  java_version: "17",
  asset_index_id: "17",
  username: "Player"
});

RTLauncher/
├── src-tauri/              # Rust后端
│   └── src/
│       ├── api/            # 登录API
│       ├── module/
│       │   ├── download/   # 下载模块
│       │   └── start_game/ # 启动模块
│       └── utils/          # 工具函数
│
├── RTL-WebUI/              # Next.js前端
│   ├── app/
│   │   └── launcher/       # 启动器页面
│   └── lib/
│       ├── tauri-commands.ts  # Tauri命令封装
│       └── tauri-types.ts     # TypeScript类型
│
├── INTEGRATION_GUIDE.md   # 详细集成文档
└── QUICKSTART.md          # 本文件

登录实现方法
- 在本地启动HTTP服务器(端口40323)
- 打开浏览器进行微软OAuth授权
- 返回授权码供后续使用

下载
- 支持下载任意Minecraft版本
- 自动下载客户端JAR、库文件、资源文件
- SHA1校验确保文件完整性
- 自动解压natives库

启动功能
- 自动检测系统Java安装
- 生成完整的JVM启动参数
- 配置classpath和natives路径
- 启动游戏进程

配置说明

### 游戏文件路径
默认路径位于 `src-tauri/src/module/download/paths.rs`:

- **Windows**: `C:\.minecraft`
- **macOS**: `~/Library/Application Support/RTLaincher`
- **Linux**: `~/.minecraft`

可根据需要修改 `MinecraftPaths::new()` 方法。

### 内存配置
在启动游戏时配置JVM参数:

startup_parameter: "-Xmx4G -Xms2G"  // 最大4G,最小2G

### Java版本
不同Minecraft版本需要不同的Java版本:
- MC 1.17+: Java 17+
- MC 1.12-1.16: Java 8

## 故障排除

### 1. 端口占用
如果40323端口被占用,登录功能会失败。解决方法:
```bash
# macOS/Linux
lsof -ti:40323 | xargs kill -9

# Windows
netstat -ano | findstr :40323
taskkill /PID <进程ID> /F
```

### 2. Java未找到
确保系统已安装Java并设置JAVA_HOME环境变量。

### 3. 下载失败
- 检查网络连接
- 确保能访问Mojang服务器
- 查看终端日志获取详细错误信息

## 下一步开发

查看 `INTEGRATION_GUIDE.md` 了解:
- 完整的API文档
- 详细的代码结构说明
- 功能扩展建议
- 性能优化方案

## 技术支持

遇到问题? 
1. 查看终端日志输出
2. 检查 `INTEGRATION_GUIDE.md`
3. 查看原项目文档

## 许可证

GPL-3.0 License
