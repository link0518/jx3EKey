# 剑网3改键工具 - Tauri版本

这是剑网3改键工具的现代化Tauri重构版本，使用Vue 3 + Tauri技术栈开发。

## 特性

- 🎨 现代化苹果风格界面设计
- ⚡ 基于Tauri的高性能桌面应用
- 🔧 智能预设管理系统
- 🚀 快速键位配置复制
- 🛡️ 安全的文件操作保护
- 📱 响应式界面设计

## 技术栈

- **前端**: Vue 3 + Vite
- **后端**: Rust + Tauri
- **样式**: 原生CSS（苹果设计语言）
- **状态管理**: Pinia

## 开发环境要求

- Node.js 16+
- Rust 1.60+
- Tauri CLI

## 安装依赖

```bash
# 安装前端依赖
npm install

# 安装Tauri CLI（如果还没有安装）
npm install -g @tauri-apps/cli
```

## 开发

```bash
# 启动开发服务器
npm run tauri dev
```

## 构建

```bash
# 构建生产版本
npm run tauri build
```

## 项目结构

```
tauri-app/
├── src/                    # Vue前端源码
│   ├── App.vue            # 主应用组件
│   ├── main.js            # 应用入口
│   └── style.css          # 苹果风格样式
├── src-tauri/             # Tauri后端源码
│   ├── src/
│   │   └── main.rs        # Rust后端逻辑
│   ├── Cargo.toml         # Rust依赖配置
│   └── tauri.conf.json    # Tauri配置
├── package.json           # Node.js依赖
└── vite.config.js         # Vite配置
```

## 功能说明

### 源账号配置
- 选择要复制的键位配置源
- 支持四级目录结构：账号 → 大区 → 区服 → 角色
- 可保存为预设以便快速重用

### 目标账号配置
- 选择键位配置的目标位置
- 同样支持四级目录结构
- 执行改键操作前会进行安全检查

### 预设管理
- 保存常用的源配置为预设
- 双击预设名称快速加载配置
- 右键预设名称可删除预设
- 预设数据自动保存到本地

### 安全特性
- 操作前确认对话框
- 路径冲突检测
- 文件存在性验证
- 错误处理和用户提示

## 与原版对比

| 特性 | 原版(Tkinter) | Tauri版 |
|------|---------------|---------|
| 界面技术 | Python Tkinter | Vue 3 + CSS |
| 性能 | 中等 | 高性能 |
| 界面美观度 | 良好 | 优秀 |
| 跨平台 | 需要Python环境 | 原生应用 |
| 安装包大小 | 需要Python运行时 | 独立可执行文件 |
| 开发维护 | Python生态 | 现代Web技术栈 |

## 开发说明

### 添加新功能
1. 前端逻辑在 `src/App.vue` 中实现
2. 后端API在 `src-tauri/src/main.rs` 中添加
3. 样式修改在 `src/style.css` 中进行

### 调试
- 前端调试：浏览器开发者工具
- 后端调试：Rust日志输出
- Tauri调试：`tauri dev` 命令行输出

## 许可证

与原版保持一致