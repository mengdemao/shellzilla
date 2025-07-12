<template>
  <div :class="{ 'dark-mode': darkMode }">
    <header>
      <h1>Tauri + Vue.js + Monaco Editor</h1>
      <p>功能完整的代码编辑器桌面应用</p>
    </header>

    <main>
      <div class="editor-wrapper">
        <MonacoEditor v-model="code" />
      </div>

      <div class="preview">
        <h3>代码预览</h3>
        <pre>{{ code }}</pre>
      </div>
    </main>

    <footer>
      <button @click="toggleTheme">
        {{ darkMode ? "切换到亮色主题" : "切换到暗色主题" }}
      </button>
      <p>使用 Tauri、Vue.js 和 Monaco Editor 构建</p>
    </footer>
  </div>
</template>

<script>
import MonacoEditor from "./components/editor.vue";
import { ref } from "vue";

export default {
  components: {
    MonacoEditor,
  },
  setup() {
    const code = ref(`function calculateSum(a, b) {
  // 计算两个数字的和
  return a + b;
}

console.log('3 + 5 =', calculateSum(3, 5));`);

    const darkMode = ref(false);

    const toggleTheme = () => {
      darkMode.value = !darkMode.value;
      document.documentElement.style.backgroundColor = darkMode.value
        ? "#1e1e1e"
        : "#f5f5f7";
    };

    return {
      code,
      darkMode,
      toggleTheme,
    };
  },
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body,
html,
#app {
  height: 100%;
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}

#app {
  display: flex;
  flex-direction: column;
  background: #f5f5f7;
  transition: background-color 0.3s;
}

.dark-mode {
  background: #1e1e1e;
  color: #e0e0e0;
}

header {
  text-align: center;
  padding: 20px;
  background: linear-gradient(135deg, #6a11cb 0%, #2575fc 100%);
  color: white;
}

header h1 {
  font-size: 2.2rem;
  margin-bottom: 10px;
}

main {
  display: flex;
  flex: 1;
  padding: 20px;
  gap: 20px;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.editor-wrapper {
  flex: 3;
  min-width: 0;
}

.preview {
  flex: 1;
  background: white;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  overflow: auto;
}

.dark-mode .preview {
  background: #2d2d2d;
  color: #d4d4d4;
}

.preview h3 {
  margin-bottom: 10px;
  padding-bottom: 8px;
  border-bottom: 1px solid #eee;
}

.dark-mode .preview h3 {
  border-bottom: 1px solid #444;
}

.preview pre {
  white-space: pre-wrap;
  font-family: "Fira Code", monospace;
  line-height: 1.5;
  font-size: 14px;
}

footer {
  padding: 15px;
  text-align: center;
  background: #f0f0f0;
  border-top: 1px solid #ddd;
}

.dark-mode footer {
  background: #252526;
  border-top: 1px solid #444;
}

footer button {
  padding: 8px 16px;
  background: #4a6cf7;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  margin-bottom: 10px;
  transition: background 0.3s;
}

footer button:hover {
  background: #3a5ae0;
}
</style>
