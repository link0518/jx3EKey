<div align="center">

# 🦊 毛毛狐改键工具

*剑网3键位配置管理工具 - 现代化Tauri版本*

[![Version](https://img.shields.io/badge/version-3.0.0-blue.svg)](https://github.com/link0518/jx3EKey.git)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-1.5.0-orange.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.3.0-brightgreen.svg)](https://vuejs.org/)

</div>

## ✨ 特性

- 🚀 **现代化界面** - 简洁美观的用户界面设计
- ⚡ **高性能应用** - Tauri + Rust 提供原生级别的性能
- 🔧 **智能预设管理** - 保存和管理常用键位配置
- 🛡️ **安全操作** - 多重确认机制，防止误操作
- 📱 **响应式设计** - 适配不同屏幕尺寸
- 🎯 **简单易用** - 四步完成键位复制

## 🎮 使用说明

### 快速开始

1. **选择游戏目录** - 首次使用需选择游戏数据文件夹
   ```
   盘符:/SeasunGame/Game/JX3/bin/zhcn_hd/userdata
   ```

2. **配置源账号** - 左侧选择要复制键位的角色配置

3. **选择目标位置** - 中间选择要应用键位的目标角色

4. **执行改键** - 点击"执行改键"完成键位复制

### 预设管理

- **保存预设**: 配置好源账号后，点击"保存预设"
- **加载预设**: 双击预设名称快速加载配置
- **管理预设**: 右键预设可重命名或删除

### 注意事项

⚠️ **重要提醒**
- 改键前请确保关闭源账号的云同步设置
- 建议在操作前备份重要的键位配置
- 登录目标账号到角色选择界面，改键后进入游戏即可生效

## 🛠️ 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| **Vue 3** | 3.3.0 | 前端框架 |
| **Tauri** | 1.5.0 | 桌面应用框架 |
| **Rust** | Latest | 后端逻辑 |
| **Element Plus** | 2.10.5 | UI组件库 |
| **Vite** | 4.4.0 | 构建工具 |

## 🚀 开发环境

### 环境要求

- **Node.js** >= 16.0.0
- **Rust** >= 1.60.0
- **Git** (用于版本控制)

### 快速开始

```bash
# 克隆项目
git clone https://github.com/link0518/jx3EKey.git
cd jx3EKey

# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

### 构建发布

```bash
# 构建生产版本
npm run build

# 预览构建结果
npm run preview
```

## 📁 项目结构

```
jx3-key-manager/
├── 📁 public/                 # 静态资源
│   ├── 🦊 fox.svg            # 狐狸图标
│   └── 🖼️ favicon.ico        # 网站图标
├── 📁 src/                    # 前端源码
│   ├── 🎨 App.vue            # 主应用组件
│   ├── ⚡ main.js            # 应用入口
│   └── 🎨 style.css          # 现代化样式
├── 📁 src-tauri/             # Tauri后端
│   ├── 📁 src/
│   │   └── 🦀 main.rs        # Rust后端逻辑
│   ├── 📁 icons/             # 应用图标
│   ├── ⚙️ Cargo.toml         # Rust依赖
│   └── ⚙️ tauri.conf.json    # Tauri配置
├── 📄 package.json           # 项目配置
├── ⚙️ vite.config.js         # Vite配置
└── 📖 README.md              # 项目说明
```

## 🔧 开发指南

### 添加新功能

1. **前端功能**: 在 `src/App.vue` 中实现UI和逻辑
2. **后端API**: 在 `src-tauri/src/main.rs` 中添加Rust函数
3. **样式修改**: 在 `src/style.css` 中调整样式

### 调试技巧

- **前端调试**: 使用浏览器开发者工具
- **后端调试**: 查看终端的Rust日志输出
- **Tauri调试**: 使用 `npm run dev` 查看详细日志

### 代码规范

- 使用 **Vue 3 Composition API**
- 遵循 **现代化设计** 的UI规范
- Rust代码遵循 **标准格式化** 规范

## 🤝 贡献指南

欢迎提交Issue和Pull Request！

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- 感谢 [Tauri](https://tauri.app/) 提供的优秀桌面应用框架
- 感谢 [Vue.js](https://vuejs.org/) 团队的出色工作
- 感谢 [Element Plus](https://element-plus.org/) 提供的UI组件

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐ Star 支持一下！**

Made with ❤️ by 咕涌

</div>

