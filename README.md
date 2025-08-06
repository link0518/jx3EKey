# 毛毛狐改键工具 v3.0.0

<div align="center">
  <img src="public/fox.svg" alt="毛毛狐改键工具" width="120" height="120">
  
  <h3>专为剑网3玩家打造的键位配置管理工具</h3>
  
  [![Tauri](https://img.shields.io/badge/Tauri-2.0.0-blue.svg)](https://tauri.app/)
  [![Vue](https://img.shields.io/badge/Vue-3.4.19-green.svg)](https://vuejs.org/)
  [![Element Plus](https://img.shields.io/badge/Element%20Plus-2.5.6-blue.svg)](https://element-plus.org/)
  [![Rust](https://img.shields.io/badge/Rust-1.60+-orange.svg)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
</div>

## ✨ 功能特色

- 🚀 **现代化界面** - 基于 Vue 3 + Element Plus 的美观 UI
- ⚡ **高性能应用** - Tauri 2.0 提供原生性能体验
- 🔧 **智能预设管理** - 保存和管理常用键位配置
- 🛡️ **安全操作** - 安全的文件复制和备份机制
- 📱 **响应式设计** - 适配不同屏幕尺寸
- 🎯 **一键改键** - 快速复制角色键位配置

## 🛠️ 技术栈

| 技术 | 版本 | 说明 |
|------|------|------|
| **前端框架** | Vue 3.4.19 | 渐进式 JavaScript 框架 |
| **UI 组件库** | Element Plus 2.5.6 | Vue 3 组件库 |
| **构建工具** | Vite 5.1.4 | 下一代前端构建工具 |
| **桌面框架** | Tauri 2.0.0 | 跨平台桌面应用框架 |
| **后端语言** | Rust 1.60+ | 系统级编程语言 |
| **状态管理** | Vue 3 Composition API | 响应式状态管理 |

## 📦 安装使用

### 方式一：直接下载（推荐）
1. 从 [Releases](../../releases) 页面下载最新版本
2. 解压到任意文件夹
3. 双击 `jx3-key-manager.exe` 运行

### 方式二：从源码构建
```bash
# 克隆项目
git clone https://github.com/link0518/jx3EKey.git
cd jx3EKey

# 安装前端依赖
npm install

# 开发模式运行
npm run dev

# 构建生产版本
npm run build
```

## 🎮 使用指南

### 初次使用
1. **选择游戏数据文件夹**
   - 首次打开会提示选择文件夹
   - 通常位于：`盘符:/SeasunGame/Game/JX3/bin/zhcn_hd/userdata`

2. **配置源账号**
   - 在左侧选择要复制的角色键位
   - 依次选择：账号 → 大区 → 区服 → 角色

3. **配置目标账号**
   - 在中间选择要覆盖的角色位置
   - 同样依次选择各级配置

4. **执行改键**
   - 点击"执行改键"按钮
   - 确认操作后完成键位复制

### 预设管理
- **保存预设**：配置好源账号后点击"保存预设"
- **加载预设**：双击预设名称快速加载配置
- **管理预设**：右键预设可重命名或删除

### 注意事项
⚠️ **重要提醒**
- 改键前请确保关闭源账号的云同步设置
- 建议在角色选择界面进行改键操作
- 改键后进入游戏即可生效
- 建议备份重要的键位配置

## 🏗️ 项目结构

```
jx3-key-manager/
├── src/                    # 前端源码
│   ├── App.vue            # 主应用组件
│   ├── main.js            # 应用入口
│   └── style.css          # 全局样式
├── src-tauri/             # Tauri 后端
│   ├── src/
│   │   └── main.rs        # Rust 主程序
│   ├── capabilities/      # 权限配置
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 配置
├── public/                # 静态资源
├── package.json           # Node.js 依赖配置
└── README.md             # 项目说明
```

## 🔧 开发指南

### 环境要求
- Node.js 16+ 
- Rust 1.60+
- 操作系统：Windows 10+

### 开发命令
```bash
# 安装依赖
npm install

# 开发模式（热重载）
npm run dev

# 构建应用
npm run build

# 仅构建前端
npm run build:web

# 检查代码
npm run lint
```

### 调试技巧
- 使用浏览器开发者工具调试前端
- 使用 `console.log` 在 Rust 端输出调试信息
- 查看 `target/debug` 目录下的日志文件

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 提交 Pull Request

## 📝 更新日志

### v3.0.0 (2025-01-06)
- 🎉 **重大更新**：升级到 Tauri 2.0
- ⬆️ **技术栈升级**：
  - Vue 3.3.0 → 3.4.19
  - Element Plus 2.10.5 → 2.5.6
  - Vite 4.4.0 → 5.1.4
  - Tauri 1.5.0 → 2.0.0
- 🐛 **问题修复**：
  - 修复窗口控制功能
  - 修复目标账号下拉框渲染问题
  - 优化权限配置和 API 调用
- ✨ **功能改进**：
  - 更好的错误处理
  - 优化的用户界面
  - 提升的性能表现

### v2.x.x
- 基础功能实现
- 预设管理系统
- 文件操作优化

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Tauri](https://tauri.app/) - 优秀的桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库
- [Rust](https://www.rust-lang.org/) - 系统编程语言

## 📞 联系方式

- 开发者：咕涌
- 项目地址：[GitHub](https://github.com/link0518/jx3EKey.git)
- 问题反馈：[Issues](../../issues)

---

<div align="center">
  <p>如果这个工具对你有帮助，请给个 ⭐ Star 支持一下！</p>
  <p>Made with ❤️ for JX3 Players</p>
</div>