# Shellzilla 🦀

![Languages](https://img.shields.io/badge/Vue-56.3%25-4FC08D?style=flat-square)
![Languages](https://img.shields.io/badge/Rust-16.3%25-CE422B?style=flat-square)
![Languages](https://img.shields.io/badge/JavaScript-15.4%25-F7DF1E?style=flat-square)

串口调试助手 - 一个功能强大的跨平台串口调试工具

## 📋 项目简介

Shellzilla 是一个使用 **Vue + Rust + Tauri** 构建的现代化串口调试助手，为开发者提供便捷的串口通信、调试和数据分析功能。

## 🌟 主要特性

- 🔧 **易用的界面** - 基于 Vue 构建的现代化用户界面
- ⚡ **高性能** - 采用 Rust 后端确保性能和稳定性
- 🖥️ **跨平台支持** - 基于 Tauri 框架，支持 Windows、macOS、Linux
- 📊 **数据可视化** - 实时数据展示和分析

## 🛠️ 技术栈

- **前端**：Vue 3 (56.3%)
- **后端**：Rust (16.3%)
- **脚本**：JavaScript (15.4%)
- **样式**：CSS (9.8%)
- **标签**：HTML (2.2%)
- **框架**：Tauri（桌面应用框架）

## 📦 快速开始

### 前置要求

- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/tools/install) (最新版本)
- [Yarn](https://yarnpkg.com/) 或 NPM

### 安装与运行

```shell
# 1. 克隆项目
git clone https://github.com/mengdemao/shellzilla.git
cd shellzilla

# 2. 安装依赖
yarn install

# 3. 开发模式运行
yarn tauri dev

# 4. 构建生产版本
yarn tauri build

# 5. 更新依赖（可选）
yarn upgrade --latest
```

## 📖 使用指南

### 基本操作

1. **连接设备** - 选择串口并配置波特率、数据位等参数
2. **发送数据** - 输入或粘贴数据进行发送
3. **接收数据** - 实时显示接收到的数据
4. **数据分析** - 查看数据统计和时间轴信息

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

## 📝 许可证

请查看 [LICENSE](./LICENSE) 文件了解详情。

## 👨‍💻 作者

- [mengdemao](https://github.com/mengdemao)

## 📧 联系方式

如有问题或建议，欢迎提交 Issue 或 Discussion。

---

**Made with ❤️ by Shellzilla Community**
