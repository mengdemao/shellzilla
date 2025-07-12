import { createApp } from "vue";
import App from "./App.vue";

// createApp(App).mount("#app");

// import { createApp } from 'vue'
// import App from './App.vue'
// import './index.css'

// 配置 Monaco Editor 的 worker 路径
self.MonacoEnvironment = {
  getWorkerUrl: function (moduleId, label) {
    if (label === "json") {
      return "./json.worker.js";
    }
    if (label === "css" || label === "scss" || label === "less") {
      return "./css.worker.js";
    }
    if (label === "html" || label === "handlebars" || label === "razor") {
      return "./html.worker.js";
    }
    if (label === "typescript" || label === "javascript") {
      return "./ts.worker.js";
    }
    return "./editor.worker.js";
  },
};

createApp(App).mount("#app");
