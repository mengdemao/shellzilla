<template>
  <div class="editor-container">
    <div class="toolbar">
      <select v-model="selectedLanguage" @change="changeLanguage">
        <option value="javascript">JavaScript</option>
        <option value="typescript">TypeScript</option>
        <option value="html">HTML</option>
        <option value="css">CSS</option>
        <option value="python">Python</option>
        <option value="json">JSON</option>
      </select>

      <button @click="formatCode" title="格式化代码">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          viewBox="0 0 16 16"
        >
          <path
            d="M10.5 15a.5.5 0 0 1-.5-.5V2H9v12.5a.5.5 0 0 1-1 0V9H7a4 4 0 1 1 0-8h5.5a.5.5 0 0 1 0 1H11v12.5a.5.5 0 0 1-.5.5z"
          />
        </svg>
      </button>

      <button @click="copyCode" title="复制代码">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          viewBox="0 0 16 16"
        >
          <path
            d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"
          />
          <path
            d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"
          />
        </svg>
      </button>

      <button @click="toggleTheme" title="切换主题">
        {{ darkMode ? "☀️ 亮色" : "🌙 暗色" }}
      </button>
    </div>

    <div ref="monacoContainer" class="monaco-editor"></div>
  </div>
</template>

<script>
import * as monaco from "monaco-editor";
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";

export default {
  name: "MonacoEditor",
  props: {
    modelValue: {
      type: String,
      default:
        '// 在这里编写代码...\nfunction hello() {\n  console.log("Hello, Tauri!");\n}',
    },
  },
  emits: ["update:modelValue"],
  setup(props, { emit }) {
    const monacoContainer = ref(null);
    const editor = ref(null);
    const selectedLanguage = ref("javascript");
    const darkMode = ref(false);

    // 初始化编辑器
    const initEditor = () => {
      if (!monacoContainer.value) return;

      // 销毁现有编辑器实例
      if (editor.value) {
        editor.value.dispose();
      }

      // 创建新编辑器实例
      editor.value = monaco.editor.create(monacoContainer.value, {
        value: props.modelValue,
        language: selectedLanguage.value,
        theme: darkMode.value ? "vs-dark" : "vs",
        automaticLayout: true,
        minimap: { enabled: true },
        fontSize: 14,
        lineNumbers: "on",
        roundedSelection: false,
        scrollBeyondLastLine: false,
        readOnly: false,
        folding: true,
        lineDecorationsWidth: 10,
        contextmenu: true,
      });

      // 监听内容变化
      editor.value.onDidChangeModelContent(() => {
        const value = editor.value.getValue();
        emit("update:modelValue", value);
      });
    };

    // 更改语言
    const changeLanguage = () => {
      if (editor.value) {
        monaco.editor.setModelLanguage(
          editor.value.getModel(),
          selectedLanguage.value,
        );
      }
    };

    // 格式化代码
    const formatCode = async () => {
      if (editor.value) {
        try {
          await editor.value.getAction("editor.action.formatDocument").run();
        } catch (error) {
          console.error("格式化失败:", error);
        }
      }
    };

    // 复制代码
    const copyCode = async () => {
      if (editor.value) {
        const code = editor.value.getValue();
        try {
          await writeText(code);
          alert("代码已复制到剪贴板！");
        } catch (error) {
          console.error("复制失败:", error);
        }
      }
    };

    // 切换主题
    const toggleTheme = () => {
      darkMode.value = !darkMode.value;
      monaco.editor.setTheme(darkMode.value ? "vs-dark" : "vs");
    };

    // 生命周期钩子
    onMounted(() => {
      // 延迟初始化以确保容器已渲染
      setTimeout(initEditor, 100);
    });

    onBeforeUnmount(() => {
      if (editor.value) {
        editor.value.dispose();
      }
    });

    // 监听模型值变化
    watch(
      () => props.modelValue,
      (newValue) => {
        if (editor.value && editor.value.getValue() !== newValue) {
          editor.value.setValue(newValue);
        }
      },
    );

    return {
      monacoContainer,
      selectedLanguage,
      darkMode,
      changeLanguage,
      formatCode,
      copyCode,
      toggleTheme,
    };
  },
};
</script>

<style scoped>
.editor-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  border-radius: 8px;
  overflow: hidden;
  background: #f5f5f5;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.toolbar {
  display: flex;
  gap: 10px;
  padding: 10px 15px;
  background: #f0f0f0;
  border-bottom: 1px solid #ddd;
}

.toolbar select,
.toolbar button {
  padding: 6px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  background: white;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.toolbar button {
  display: flex;
  align-items: center;
  gap: 5px;
}

.toolbar select:hover,
.toolbar button:hover {
  background: #e9e9e9;
  border-color: #ccc;
}

.monaco-editor {
  flex: 1;
  min-height: 400px;
  border-radius: 0 0 8px 8px;
  overflow: hidden;
}

.dark-mode .editor-container {
  background: #2d2d2d;
}

.dark-mode .toolbar {
  background: #333;
  border-bottom: 1px solid #444;
}
</style>
