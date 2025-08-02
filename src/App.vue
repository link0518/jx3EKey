<template>
  <div class="app">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <button class="toolbar-btn" @click="showSettingsMenu = !showSettingsMenu" title="设置">
          ⚙️ 设置
        </button>
        <button class="toolbar-btn" @click="showHelpMenu = !showHelpMenu" title="帮助">
          ❓ 帮助
        </button>
      </div>
    </div>

    <!-- 设置菜单 -->
    <div v-if="showSettingsMenu" class="dropdown-menu settings-menu">
      <div class="menu-item" @click="changeBaseFolder">
        📁 更改文件夹
      </div>
    </div>

    <!-- 帮助菜单 -->
    <div v-if="showHelpMenu" class="dropdown-menu help-menu">
      <div class="menu-item" @click="showUsageHelp">
        📖 使用帮助
      </div>
      <div class="menu-item" @click="showAbout">
        ℹ️ 关于
      </div>
    </div>

    <!-- 主内容区 - 紧凑布局 -->
    <main class="main">
      <div class="container">
        <!-- 左侧：源账号配置 -->
        <div class="panel source-panel">
          <div class="panel-header">
            <h2 class="panel-title">源账号配置</h2>
          </div>
          <div class="form-container">
            <div class="form-row" v-for="(label, index) in labels" :key="index">
              <label class="form-label">{{ label }}</label>
              <select 
                class="form-select" 
                v-model="sourceSelections[index]"
                @change="onSourceChange(index)"
                :size="1"
              >
                <option value="">选择{{ label }}</option>
                <option 
                  v-for="option in sourceOptions[index]" 
                  :key="option" 
                  :value="option"
                >
                  {{ option }}
                </option>
              </select>
            </div>
            <button 
              class="action-btn save-btn" 
              @click="savePreset"
              :disabled="!canSavePreset"
            >
              保存预设
            </button>
          </div>
        </div>

        <!-- 中间：目标账号配置 -->
        <div class="panel target-panel">
          <div class="panel-header">
            <h2 class="panel-title">目标账号配置</h2>
          </div>
          <div class="form-container">
            <div class="form-row" v-for="(label, index) in labels" :key="index">
              <label class="form-label">{{ label }}</label>
              <select 
                class="form-select" 
                v-model="targetSelections[index]"
                @change="onTargetChange(index)"
                :size="1"
              >
                <option value="">选择{{ label }}</option>
                <option 
                  v-for="option in targetOptions[index]" 
                  :key="option" 
                  :value="option"
                >
                  {{ option }}
                </option>
              </select>
            </div>
            <button 
              class="action-btn execute-btn" 
              @click="executeKeyChange"
              :disabled="!canExecute"
            >
              执行改键
            </button>
          </div>
        </div>

        <!-- 右侧：预设管理 -->
        <div class="panel preset-panel">
          <div class="panel-header">
            <h2 class="panel-title">预设管理</h2>
          </div>
          <div class="preset-container">
            <div class="preset-hint">双击加载，右键管理</div>
            <div class="preset-list" v-if="presetNames.length > 0">
              <div 
                v-for="name in presetNames" 
                :key="name"
                class="preset-item"
                @dblclick="loadPreset(name)"
                @contextmenu.prevent="showPresetMenu($event, name)"
              >
                <div class="preset-name">{{ name }}</div>
                <div class="preset-actions">
                  <span class="preset-action-hint">双击</span>
                </div>
              </div>
            </div>
            <div v-else class="preset-empty">
              <div class="empty-text">暂无预设</div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 右键菜单 -->
    <div 
      v-if="contextMenu.show" 
      class="context-menu"
      :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
    >
      <div class="context-menu-item" @click="renamePreset(contextMenu.presetName)">
        重命名预设
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item delete-item" @click="deletePreset(contextMenu.presetName)">
        删除预设
      </div>
    </div>

    <!-- 预设名称输入对话框 -->
    <div v-if="showPresetDialog" class="dialog-overlay">
      <div class="dialog">
        <h3>保存预设</h3>
        <div class="form-group">
          <label class="label">预设名称</label>
          <input 
            v-model="presetNameInput" 
            class="input" 
            placeholder="请输入预设名称"
            @keyup.enter="confirmSavePreset"
            ref="presetInput"
          />
        </div>
        <div class="dialog-buttons">
          <button class="btn btn-secondary" @click="cancelSavePreset">取消</button>
          <button class="btn btn-primary" @click="confirmSavePreset" :disabled="!presetNameInput.trim()">保存</button>
        </div>
      </div>
    </div>

    <!-- 重命名预设对话框 -->
    <div v-if="showRenameDialog" class="dialog-overlay">
      <div class="dialog">
        <h3>重命名预设</h3>
        <div class="form-group">
          <label class="label">原名称</label>
          <input 
            :value="renamePresetName" 
            class="input" 
            disabled
            style="background: var(--bg-accent); color: var(--fg-secondary);"
          />
        </div>
        <div class="form-group">
          <label class="label">新名称</label>
          <input 
            v-model="renameNewName" 
            class="input" 
            placeholder="请输入新的预设名称"
            @keyup.enter="confirmRenamePreset"
            id="renameInput"
          />
        </div>
        <div class="dialog-buttons">
          <button class="btn btn-secondary" @click="cancelRenamePreset">取消</button>
          <button class="btn btn-primary" @click="confirmRenamePreset" :disabled="!renameNewName.trim()">重命名</button>
        </div>
      </div>
    </div>

    <!-- Toast 信息提示 -->
    <div v-if="showToast" class="toast" :class="[`toast-${toastType}`]">
      <div class="toast-content">
        {{ toastMessage }}
      </div>
    </div>

    <!-- 自定义确认对话框 -->
    <div v-if="showConfirmDialog" class="dialog-overlay">
      <div class="dialog confirm-dialog">
        <h3>{{ confirmTitle }}</h3>
        <div class="confirm-message">{{ confirmMessage }}</div>
        <div class="dialog-buttons">
          <button class="btn btn-secondary" @click="handleConfirm(false)">取消</button>
          <button class="btn btn-danger" @click="handleConfirm(true)">确认</button>
        </div>
      </div>
    </div>

    <!-- 帮助对话框 -->
    <div v-if="showHelpDialog" class="dialog-overlay">
      <div class="dialog help-dialog">
        <h3>{{ helpTitle }}</h3>
        <div class="help-content" v-html="helpContent"></div>
        <div class="dialog-buttons">
          <button class="btn btn-primary" @click="showHelpDialog = false">确定</button>
        </div>
      </div>
    </div>

    <!-- 遮罩层 -->
    <div 
      v-if="contextMenu.show || showPresetDialog || showRenameDialog || showConfirmDialog || showHelpDialog || showSettingsMenu || showHelpMenu" 
      class="overlay"
      @click="contextMenu.show = false; showPresetDialog = false; showRenameDialog = false; showSettingsMenu = false; showHelpMenu = false"
    ></div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { message, ask, confirm } from '@tauri-apps/api/dialog'

// 响应式数据
const labels = ['账号', '大区', '区服', '角色']
const basePath = ref('')
const sourceSelections = reactive(['', '', '', ''])
const targetSelections = reactive(['', '', '', ''])
const sourceOptions = reactive([[], [], [], []])
const targetOptions = reactive([[], [], [], []])
const presets = reactive({})
const contextMenu = reactive({
  show: false,
  x: 0,
  y: 0,
  presetName: ''
})
const showPresetDialog = ref(false)
const presetNameInput = ref('')
const showRenameDialog = ref(false)
const renamePresetName = ref('')
const renameNewName = ref('')
const showToast = ref(false)
const toastMessage = ref('')
const toastType = ref('info')
const showConfirmDialog = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const confirmResolve = ref(null)
const showSettingsMenu = ref(false)
const showHelpMenu = ref(false)
const showHelpDialog = ref(false)
const helpTitle = ref('')
const helpContent = ref('')


// 计算属性
const presetNames = computed(() => Object.keys(presets))
const canSavePreset = computed(() => basePath.value && sourceSelections.some(s => s))
const canExecute = computed(() => {
  const sourcePath = getSelectedPath(sourceSelections)
  const targetPath = getSelectedPath(targetSelections)
  return sourcePath && targetPath && sourcePath !== targetPath
})

// 生命周期
onMounted(async () => {
  await loadConfig()
  await loadPresets()
  if (!basePath.value) {
    await selectBaseFolder()
  } else {
    await updateOptions()
    // 恢复上次的源账号选择
    await restoreLastSourceSelections()
  }
})

// 方法
async function selectBaseFolder() {
  try {
    const selected = await open({
      directory: true,
      title: '选择游戏数据文件夹'
    })
    
    if (selected) {
      basePath.value = selected
      await saveConfig()
      await updateOptions()
      showToastMessage('文件夹设置成功', 'success')
    }
  } catch (error) {
    showToastMessage(`选择文件夹失败: ${error}`, 'error')
  }
}

async function updateOptions() {
  if (!basePath.value) return
  
  try {
    console.log('开始更新选项...')
    
    // 清空所有选项
    sourceOptions.forEach(arr => arr.splice(0))
    targetOptions.forEach(arr => arr.splice(0))
    
    // 更新第一级选项
    await updateLevelOptions(basePath.value, 0)
    
    console.log('选项更新完成')
  } catch (error) {
    console.error('更新选项失败:', error)
    showToastMessage(`更新选项失败: ${error}`, 'error')
  }
}

async function updateLevelOptions(path, level) {
  if (level >= labels.length) return
  
  try {
    const subdirs = await invoke('get_subdirectories', { path })
    sourceOptions[level].splice(0, sourceOptions[level].length, ...subdirs)
    targetOptions[level].splice(0, targetOptions[level].length, ...subdirs)
    
    // 不再递归调用，避免无限递归
    console.log(`第${level}级选项已更新，共${subdirs.length}个选项`)
  } catch (error) {
    console.error('获取子目录失败:', error)
  }
}



async function onSourceChange(level) {
  console.log(`源账号选择变更: level=${level}, value=${sourceSelections[level]}`)
  
  // 清空后续级别的选择
  for (let i = level + 1; i < sourceSelections.length; i++) {
    sourceSelections[i] = ''
    sourceOptions[i].splice(0)
  }
  
  // 如果选择了当前级别，更新下一级选项
  if (sourceSelections[level] && level + 1 < labels.length) {
    const path = getSelectedPath(sourceSelections, level + 1)
    console.log(`准备更新下一级选项: path=${path}, nextLevel=${level + 1}`)
    
    try {
      const subdirs = await invoke('get_subdirectories', { path })
      console.log(`获取到${subdirs.length}个子目录:`, subdirs)
      
      sourceOptions[level + 1].splice(0, sourceOptions[level + 1].length, ...subdirs)
      targetOptions[level + 1].splice(0, targetOptions[level + 1].length, ...subdirs)
      
      // 自动选择第一个选项
      if (subdirs.length > 0) {
        sourceSelections[level + 1] = subdirs[0]
        console.log(`自动选择: ${labels[level + 1]} = ${subdirs[0]}`)
        
        // 递归更新下一级
        await autoUpdateNextLevel(level + 1)
      }
    } catch (error) {
      console.error('获取子目录失败:', error)
    }
  }
  
  console.log('源账号选择处理完成')
  
  // 保存当前选择状态
  await saveConfig()
}

async function autoUpdateNextLevel(level) {
  if (level + 1 >= labels.length) return
  
  const path = getSelectedPath(sourceSelections, level + 1)
  console.log(`自动更新下一级: path=${path}, nextLevel=${level + 1}`)
  
  try {
    const subdirs = await invoke('get_subdirectories', { path })
    console.log(`自动获取到${subdirs.length}个子目录:`, subdirs)
    
    sourceOptions[level + 1].splice(0, sourceOptions[level + 1].length, ...subdirs)
    targetOptions[level + 1].splice(0, targetOptions[level + 1].length, ...subdirs)
    
    // 如果有选项，继续自动选择
    if (subdirs.length > 0) {
      sourceSelections[level + 1] = subdirs[0]
      console.log(`自动选择: ${labels[level + 1]} = ${subdirs[0]}`)
      
      // 继续下一级
      await autoUpdateNextLevel(level + 1)
    }
  } catch (error) {
    console.error('自动更新失败:', error)
  }
}

async function onTargetChange(level) {
  console.log(`目标账号选择变更: level=${level}, value=${targetSelections[level]}`)
  
  // 清空后续级别的选择
  for (let i = level + 1; i < targetSelections.length; i++) {
    targetSelections[i] = ''
    targetOptions[i].splice(0)
  }
  
  // 如果选择了当前级别，更新下一级选项
  if (targetSelections[level] && level + 1 < labels.length) {
    const path = getSelectedPath(targetSelections, level + 1)
    console.log(`准备更新目标下一级选项: path=${path}, nextLevel=${level + 1}`)
    
    try {
      const subdirs = await invoke('get_subdirectories', { path })
      console.log(`获取到${subdirs.length}个子目录:`, subdirs)
      
      targetOptions[level + 1].splice(0, targetOptions[level + 1].length, ...subdirs)
      
      // 自动选择第一个选项
      if (subdirs.length > 0) {
        targetSelections[level + 1] = subdirs[0]
        console.log(`目标自动选择: ${labels[level + 1]} = ${subdirs[0]}`)
        
        // 递归更新下一级
        await autoUpdateTargetNextLevel(level + 1)
      }
    } catch (error) {
      console.error('获取目标子目录失败:', error)
    }
  }
  
  console.log('目标账号选择处理完成')
}

async function autoUpdateTargetNextLevel(level) {
  if (level + 1 >= labels.length) return
  
  const path = getSelectedPath(targetSelections, level + 1)
  console.log(`目标自动更新下一级: path=${path}, nextLevel=${level + 1}`)
  
  try {
    const subdirs = await invoke('get_subdirectories', { path })
    console.log(`目标自动获取到${subdirs.length}个子目录:`, subdirs)
    
    targetOptions[level + 1].splice(0, targetOptions[level + 1].length, ...subdirs)
    
    // 如果有选项，继续自动选择
    if (subdirs.length > 0) {
      targetSelections[level + 1] = subdirs[0]
      console.log(`目标自动选择: ${labels[level + 1]} = ${subdirs[0]}`)
      
      // 继续下一级
      await autoUpdateTargetNextLevel(level + 1)
    }
  } catch (error) {
    console.error('目标自动更新失败:', error)
  }
}

function getSelectedPath(selections, maxLevel = selections.length) {
  let path = basePath.value
  for (let i = 0; i < maxLevel && i < selections.length; i++) {
    if (selections[i]) {
      path += `/${selections[i]}`
    } else {
      break
    }
  }
  return path
}

function savePreset() {
  const timestamp = new Date().toLocaleString('zh-CN').replace(/[\/\s:]/g, '-')
  presetNameInput.value = `预设-${timestamp}`
  showPresetDialog.value = true
  
  // 等待DOM更新后聚焦输入框
  setTimeout(() => {
    const input = document.querySelector('.dialog input')
    if (input) {
      input.focus()
      // 使用多种方法确保文字被选中
      input.select()
      input.setSelectionRange(0, input.value.length)
    }
  }, 200)
}

async function confirmSavePreset() {
  try {
    const name = presetNameInput.value.trim()
    if (!name) return
    
    if (presets[name]) {
      const confirmed = await showConfirm('确认覆盖', `预设 '${name}' 已存在，是否覆盖？`)
      if (!confirmed) return
    }
    
    presets[name] = [basePath.value, ...sourceSelections]
    await savePresets()
    
    showPresetDialog.value = false
    presetNameInput.value = ''
    
    showToastMessage(`预设 '${name}' 已保存`, 'success')
  } catch (error) {
    console.error('保存预设失败:', error)
    showToastMessage(`保存预设失败: ${error}`, 'error')
  }
}

function cancelSavePreset() {
  showPresetDialog.value = false
  presetNameInput.value = ''
}

// 重命名预设相关函数
function renamePreset(name) {
  contextMenu.show = false
  renamePresetName.value = name
  renameNewName.value = name
  showRenameDialog.value = true
  
  // 等待DOM更新后聚焦输入框
  setTimeout(() => {
    const input = document.querySelector('#renameInput')
    if (input) {
      input.focus()
      // 使用多种方法确保文字被选中
      input.select()
      input.setSelectionRange(0, input.value.length)
    }
  }, 200)
}

async function confirmRenamePreset() {
  try {
    const oldName = renamePresetName.value
    const newName = renameNewName.value.trim()
    
    if (!newName) return
    
    if (newName === oldName) {
      showRenameDialog.value = false
      renamePresetName.value = ''
      renameNewName.value = ''
      return
    }
    
    if (presets[newName]) {
      const confirmed = await showConfirm('确认覆盖', `预设 '${newName}' 已存在，是否覆盖？`)
      if (!confirmed) return
    }
    
    // 复制预设数据到新名称
    presets[newName] = presets[oldName]
    // 删除旧名称
    delete presets[oldName]
    
    await savePresets()
    
    showRenameDialog.value = false
    renamePresetName.value = ''
    renameNewName.value = ''
    
    showToastMessage(`预设已重命名为 '${newName}'`, 'success')
  } catch (error) {
    console.error('重命名预设失败:', error)
    showToastMessage(`重命名预设失败: ${error}`, 'error')
  }
}

function cancelRenamePreset() {
  showRenameDialog.value = false
  renamePresetName.value = ''
  renameNewName.value = ''
}

// Toast 信息提示函数
function showToastMessage(message, type = 'info') {
  toastMessage.value = message
  toastType.value = type
  showToast.value = true
  
  // 3秒后自动隐藏
  setTimeout(() => {
    showToast.value = false
  }, 3000)
}

// 自定义确认对话框函数
function showConfirm(title, message) {
  return new Promise((resolve) => {
    confirmTitle.value = title
    confirmMessage.value = message
    confirmResolve.value = resolve
    showConfirmDialog.value = true
  })
}

function handleConfirm(result) {
  showConfirmDialog.value = false
  if (confirmResolve.value) {
    confirmResolve.value(result)
    confirmResolve.value = null
  }
}

// 设置和帮助相关函数
async function changeBaseFolder() {
  showSettingsMenu.value = false
  await selectBaseFolder()
}

function showUsageHelp() {
  showHelpMenu.value = false
  helpTitle.value = '使用帮助'
  helpContent.value = `
    <div class="help-section">
      <h4>🎯 使用说明</h4>
      <p>把exe文件放入到文件夹中执行，执行后会生成“last_path.json”和“presets.json”文件</p>
    </div>
    
    <div class="help-section">
      <h4>📋 使用步骤</h4>
      <ol>
        <li><strong>选择文件夹：</strong>首次打开需要选择游戏数据文件夹（通常在"盘符:/SeasunGame/Game/JX3/bin/zhcn_hd/userdata"目录）</li>
        <li><strong>选择源配置：</strong>左侧选择要复制的角色键位</li>
        <li><strong>选择目标：</strong>中间选择要覆盖的角色位置</li>
        <li><strong>执行改键：</strong>点击"执行改键"完成复制</li>
        <li><strong>快捷使用：</strong>登录目标账号到角色选择界面，改键后进入游戏即可生效</li>
      </ol>
    </div>
    
    <div class="help-section">
      <h4>💾 预设功能</h4>
      <ul>
        <li><strong>保存：</strong>保存常用配置为预设</li>
        <li><strong>加载：</strong>双击预设名称快速加载</li>
        <li><strong>管理：</strong>右键预设可重命名或删除</li>
      </ul>
    </div>
    
    <div class="help-section">
      <h4>⚠️ 注意事项</h4>
      <ul>
        <li>改键前请确保关闭源账号同步设置</li>
        <li>建议备份重要配置</li>
      </ul>
    </div>
  `
  showHelpDialog.value = true
}

function showAbout() {
  showHelpMenu.value = false
  helpTitle.value = '关于'
  helpContent.value = `
    <div class="about-content">
      <div class="app-info">
        <h4>🎮 剑网3改键工具</h4>
        <p><strong>版本：</strong>3.0.0 Tauri Edition</p>
        <p><strong>作者：</strong>by 咕涌</p>
      </div>
      
      <div class="features">
        <h4>✨ 主要特性</h4>
        <ul>
          <li>🚀 现代化界面设计</li>
          <li>⚡ 高性能桌面应用</li>
          <li>🔧 智能预设管理</li>
          <li>🛡️ 安全的文件操作</li>
        </ul>
      </div>
      
      <div class="developer">
        <h4>👨‍💻 技术栈</h4>
        <p>Vue 3 + Tauri + Rust</p>
        <p>现代化跨平台桌面应用</p>
      </div>
      
      <div class="copyright">
        <p style="text-align: center; margin-top: 16px; color: #718096; font-size: 12px;">
          © 2025 剑网3改键工具. All rights reserved.
        </p>
      </div>
    </div>
  `
  showHelpDialog.value = true
}

async function loadPreset(name) {
  try {
    if (!presets[name]) return
    
    console.log(`加载预设: ${name}`)
    const [savedBasePath, ...selections] = presets[name]
    
    // 设置基础路径
    basePath.value = savedBasePath
    
    // 清空当前选择
    sourceSelections.forEach((_, i) => sourceSelections[i] = '')
    
    // 重新初始化选项
    await updateOptions()
    
    // 逐级设置选择并更新选项
    for (let i = 0; i < selections.length && i < sourceSelections.length; i++) {
      if (selections[i]) {
        sourceSelections[i] = selections[i]
        
        // 更新下一级选项
        if (i + 1 < labels.length) {
          const path = getSelectedPath(sourceSelections, i + 1)
          try {
            const subdirs = await invoke('get_subdirectories', { path })
            sourceOptions[i + 1].splice(0, sourceOptions[i + 1].length, ...subdirs)
            targetOptions[i + 1].splice(0, targetOptions[i + 1].length, ...subdirs)
          } catch (error) {
            console.error(`更新第${i + 1}级选项失败:`, error)
            break
          }
        }
      }
    }
    
    await saveConfig()
    console.log('预设加载完成')
  } catch (error) {
    console.error('加载预设失败:', error)
    showToastMessage(`加载预设失败: ${error}`, 'error')
  }
}

function showPresetMenu(event, presetName) {
  contextMenu.show = true
  contextMenu.x = event.clientX
  contextMenu.y = event.clientY
  contextMenu.presetName = presetName
}

async function deletePreset(name) {
  contextMenu.show = false
  
  try {
    const confirmed = await showConfirm('确认删除', `确定要删除预设 '${name}' 吗？\n\n此操作无法撤销。`)
    
    if (confirmed) {
      delete presets[name]
      await savePresets()
      showToastMessage(`预设 '${name}' 已删除`, 'success')
    }
  } catch (error) {
    showToastMessage(`删除预设失败: ${error}`, 'error')
  }
}

async function executeKeyChange() {
  const sourcePath = getSelectedPath(sourceSelections)
  const targetPath = getSelectedPath(targetSelections)
  
  if (!sourcePath || !targetPath) {
    showToastMessage('请确保源路径和目标路径都已选择', 'warning')
    return
  }
  
  if (sourcePath === targetPath) {
    showToastMessage('源路径和目标路径不能相同', 'warning')
    return
  }
  
  try {
    const sourceName = sourceSelections.filter(s => s).join(' → ')
    const targetName = targetSelections.filter(s => s).join(' → ')
    
    const confirmed = await showConfirm(
      '确认改键',
      `确认要执行改键操作吗？\n\n源配置: ${sourceName}\n目标位置: ${targetName}\n\n此操作将覆盖目标位置的现有配置`
    )
    
    if (!confirmed) return
    
    await invoke('copy_directory', { source: sourcePath, target: targetPath })
    showToastMessage('键位配置已成功复制到目标位置！建议重启游戏以确保配置生效', 'success')
  } catch (error) {
    showToastMessage(`改键操作失败: ${error}`, 'error')
  }
}

async function saveConfig() {
  try {
    await invoke('save_config', {
      config: {
        base_path: basePath.value,
        last_left_path: getSelectedPath(sourceSelections),
        last_source_selections: [...sourceSelections]
      }
    })
  } catch (error) {
    console.error('保存配置失败:', error)
  }
}

async function loadConfig() {
  try {
    const config = await invoke('load_config')
    console.log('加载的配置:', config)
    
    basePath.value = config.base_path || ''
    
    // 恢复上次的源账号选择
    if (config.last_source_selections && config.last_source_selections.length > 0) {
      console.log('恢复源账号选择:', config.last_source_selections)
      for (let i = 0; i < config.last_source_selections.length && i < sourceSelections.length; i++) {
        sourceSelections[i] = config.last_source_selections[i] || ''
      }
      console.log('恢复后的sourceSelections:', [...sourceSelections])
    } else {
      console.log('没有找到上次的源账号选择')
    }
  } catch (error) {
    console.error('加载配置失败:', error)
  }
}

async function savePresets() {
  try {
    await invoke('save_presets', { presets })
  } catch (error) {
    console.error('保存预设失败:', error)
  }
}

async function loadPresets() {
  try {
    const loadedPresets = await invoke('load_presets')
    Object.assign(presets, loadedPresets)
  } catch (error) {
    console.error('加载预设失败:', error)
  }
}

// 恢复上次的源账号选择
async function restoreLastSourceSelections() {
  try {
    console.log('开始恢复上次源账号选择:', sourceSelections)
    
    // 逐级恢复选择并更新下级选项
    for (let i = 0; i < sourceSelections.length; i++) {
      if (sourceSelections[i]) {
        console.log(`恢复第${i}级选择: ${labels[i]} = ${sourceSelections[i]}`)
        
        // 更新下一级选项
        if (i + 1 < labels.length) {
          const path = getSelectedPath(sourceSelections, i + 1)
          console.log(`获取第${i + 1}级选项，路径: ${path}`)
          
          try {
            const subdirs = await invoke('get_subdirectories', { path })
            console.log(`第${i + 1}级选项:`, subdirs)
            
            sourceOptions[i + 1].splice(0, sourceOptions[i + 1].length, ...subdirs)
            targetOptions[i + 1].splice(0, targetOptions[i + 1].length, ...subdirs)
          } catch (error) {
            console.error(`恢复第${i + 1}级选项失败:`, error)
            // 如果某一级失败，清空后续选择
            for (let j = i + 1; j < sourceSelections.length; j++) {
              sourceSelections[j] = ''
            }
            break
          }
        }
      } else {
        console.log(`第${i}级选择为空，停止恢复`)
        break
      }
    }
    
    console.log('上次源账号选择恢复完成')
  } catch (error) {
    console.error('恢复上次选择失败:', error)
  }
}
</script>

<style scoped>
/* 现代化设计系统 */
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #f7fafc;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', sans-serif;
  overflow: hidden;
}

.main {
  flex: 1;
  padding: 16px 32px 20px;
  overflow: hidden;
}

.container {
  display: flex;
  flex-direction: row;
  gap: 28px;
  height: 100%;
  max-width: 1120px;
  margin: 0 auto;
}

.panel {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  min-height: 0;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.source-panel {
  flex: 0.85;
  margin-left: 8px;
}

.target-panel {
  flex: 0.85;
}

.preset-panel {
  flex: 0.55;
  margin-right: 8px;
}

.panel:hover {
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.12);
  border-color: rgba(102, 126, 234, 0.2);
}

.panel-header {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, rgba(255, 255, 255, 0.08) 100%);
  padding: 18px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  display: flex;
  justify-content: center;
  align-items: center;
  flex-shrink: 0;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  position: relative;
}

.panel-header::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 20px;
  right: 20px;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(0, 0, 0, 0.1), transparent);
}

.panel-title {
  font-size: 15px;
  font-weight: 600;
  color: #2d3748;
  margin: 0;
  letter-spacing: -0.01em;
  text-align: center;
  position: relative;
}

/* 移除panel-icon样式 */

.form-container {
  flex: 1;
  padding: 18px 18px 16px;
  display: flex;
  flex-direction: column;
  gap: 13px;
  overflow-y: auto;
  min-height: 0;
}

.form-row {
  display: flex;
  flex-direction: column;
  gap: 5px;
  flex-shrink: 0;
}

.form-label {
  font-size: 12px;
  font-weight: 500;
  color: #4a5568;
  margin-bottom: 5px;
  letter-spacing: 0.02em;
}

.form-select {
  padding: 11px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  font-size: 13px;
  background: #ffffff;
  color: #2d3748;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
  min-height: 42px;
  height: 42px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  font-weight: 500;
  appearance: none;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 12px center;
  background-repeat: no-repeat;
  background-size: 15px;
  padding-right: 38px;
  padding-left: 16px;
}

.form-select:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1), 0 4px 6px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.form-select:hover {
  border-color: #667eea;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.15);
}

.form-select::selection {
  background: rgba(102, 126, 234, 0.3);
  color: #2d3748;
}

.form-select::-moz-selection {
  background: rgba(102, 126, 234, 0.3);
  color: #2d3748;
}

.action-btn {
  margin-top: 10px;
  margin-bottom: 0;
  padding: 11px 18px;
  border: none;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 42px;
  flex-shrink: 0;
  letter-spacing: 0.02em;
  user-select: none;
  -webkit-user-select: none;
  position: relative;
  overflow: hidden;
}

.action-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.action-btn:hover::before {
  left: 100%;
}

.action-btn:active {
  transform: scale(0.96);
  transition-duration: var(--duration-immediate);
}

.save-btn {
  background: linear-gradient(135deg, #4299e1 0%, #3182ce 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(66, 153, 225, 0.4);
}

.save-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, #3182ce 0%, #2c5282 100%);
  box-shadow: 0 4px 15px rgba(66, 153, 225, 0.4);
  transform: translateY(-1px);
}

.execute-btn {
  background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(72, 187, 120, 0.4);
}

.execute-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, #38a169 0%, #2f855a 100%);
  box-shadow: 0 4px 15px rgba(72, 187, 120, 0.4);
  transform: translateY(-1px);
}

.action-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
  transform: none;
}

.preset-container {
  flex: 1;
  padding: 18px 14px 16px;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.preset-hint {
  font-size: 11px;
  color: #718096;
  text-align: center;
  margin-bottom: 16px;
  padding: 9px 14px;
  background: linear-gradient(135deg, rgba(247, 250, 252, 0.9) 0%, rgba(237, 242, 247, 0.8) 100%);
  border-radius: 8px;
  flex-shrink: 0;
  font-weight: 500;
  border: 1px solid rgba(226, 232, 240, 0.5);
}

.preset-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
  overflow-y: auto;
  padding-right: 4px;
}

.preset-item {
  padding: 12px 16px;
  background: linear-gradient(135deg, #ffffff 0%, #fafbfc 100%);
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  justify-content: space-between;
  align-items: center;
  user-select: none;
  -webkit-user-select: none;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.06);
  position: relative;
  overflow: hidden;
}

.preset-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 3px;
  height: 100%;
  background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
  opacity: 0;
  transition: opacity 0.25s ease;
}

.preset-item:hover {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.04) 0%, rgba(118, 75, 162, 0.02) 100%);
  border-color: rgba(102, 126, 234, 0.25);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.12);
}

.preset-item:hover::before {
  opacity: 1;
}

.preset-item:active {
  transform: scale(0.98);
  transition-duration: var(--duration-immediate);
}

.preset-name {
  font-size: 13px;
  font-weight: 500;
  color: #2d3748;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.preset-action-hint {
  font-size: 11px;
  color: #a0aec0;
  opacity: 0;
  transition: opacity 0.2s ease;
  font-weight: 500;
}

.preset-item:hover .preset-action-hint {
  opacity: 1;
}

.preset-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #a0aec0;
  gap: 12px;
  padding: 40px 20px;
  background: linear-gradient(135deg, rgba(247, 250, 252, 0.5) 0%, rgba(237, 242, 247, 0.3) 100%);
  border-radius: 12px;
  border: 2px dashed #e2e8f0;
}

/* 移除empty-icon样式 */

.empty-text {
  font-size: 13px;
  font-weight: 500;
}

/* 移除面板特色边框 */

/* 加载动画 */
@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

.form-select:focus::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  animation: shimmer 1.5s infinite;
  pointer-events: none;
}

/* 滚动条优化 */
.preset-list::-webkit-scrollbar {
  width: 6px;
}

.preset-list::-webkit-scrollbar-track {
  background: var(--bg-accent);
  border-radius: var(--radius-sm);
}

.preset-list::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: var(--radius-sm);
}

.preset-list::-webkit-scrollbar-thumb:hover {
  background: var(--fg-tertiary);
}

/* 面板进入动画 */
@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.panel {
  animation: slideInUp 0.4s ease-out;
}

.source-panel {
  animation-delay: 0.1s;
}

.target-panel {
  animation-delay: 0.2s;
}

.preset-panel {
  animation-delay: 0.3s;
}

.context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  padding: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  font-size: 14px;
  backdrop-filter: blur(20px);
  min-width: 160px;
}

.context-menu-item {
  display: flex;
  align-items: center;
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #2d3748;
  font-weight: 500;
  user-select: none;
  -webkit-user-select: none;
}

.context-menu-item:hover {
  background: rgba(102, 126, 234, 0.08);
}

.context-menu-item.delete-item:hover {
  background: rgba(245, 87, 108, 0.08);
  color: #f5576c;
}

.context-menu-divider {
  height: 1px;
  background: #e2e8f0;
  margin: 4px 0;
}

/* 移除menu-icon样式 */

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 999;
  backdrop-filter: blur(2px);
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--bg-overlay);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1001;
  animation: fadeIn var(--duration-moderate) var(--timing-ease-out);
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from { 
    opacity: 0;
    transform: translateY(20px) scale(0.95);
  }
  to { 
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.dialog {
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid #e2e8f0;
  border-radius: 16px;
  padding: 32px;
  min-width: 480px;
  max-width: 90vw;
  box-shadow: 0 20px 25px rgba(0, 0, 0, 0.15);
  animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(20px);
}

.dialog h3 {
  margin: 0 0 20px 0;
  color: #2d3748;
  font-size: 18px;
  font-weight: 600;
  text-align: center;
}

.label {
  font-size: 13px;
  font-weight: 500;
  color: #4a5568;
  margin-bottom: 8px;
  display: block;
}

.input {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  font-size: 14px;
  background: #ffffff;
  color: #2d3748;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', sans-serif;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  font-weight: 500;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.05);
  letter-spacing: -0.022em;
  font-weight: var(--font-weight-regular);
}

.input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1), 0 4px 6px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.input::placeholder {
  color: #a0aec0;
  font-weight: 400;
}

.input:disabled {
  cursor: not-allowed;
  background: #f7fafc;
  color: #a0aec0;
}

.input::selection {
  background: rgba(102, 126, 234, 0.3);
  color: #2d3748;
}

.input::-moz-selection {
  background: rgba(102, 126, 234, 0.3);
  color: #2d3748;
}

.btn {
  padding: var(--spacing-2) var(--spacing-4);
  border: none;
  border-radius: var(--radius-lg);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--transition-base);
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 36px;
  font-family: var(--font-family);
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
}

.btn-primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #5a67d8 0%, #6b46c1 100%);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
  transform: translateY(-2px);
}

.btn-secondary {
  background: #ffffff;
  color: #4a5568;
  border: 1px solid #e2e8f0;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.btn-secondary:hover:not(:disabled) {
  background: #f7fafc;
  border-color: #667eea;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.btn-danger {
  background: linear-gradient(135deg, #f5576c 0%, #e53e3e 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(245, 87, 108, 0.4);
}

.btn-danger:hover:not(:disabled) {
  background: linear-gradient(135deg, #e53e3e 0%, #c53030 100%);
  box-shadow: 0 6px 20px rgba(245, 87, 108, 0.6);
  transform: translateY(-2px);
}

.confirm-dialog {
  max-width: 500px;
}

.confirm-message {
  font-size: 14px;
  color: #4a5568;
  line-height: 1.6;
  margin-bottom: 24px;
  white-space: pre-line;
}

/* 工具栏样式 */
.toolbar {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  padding: 8px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: relative;
  z-index: 100;
}

.toolbar-left {
  display: flex;
  gap: 8px;
}

.toolbar-btn {
  padding: 6px 10px;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 7px;
  font-size: 11px;
  font-weight: 500;
  color: #4a5568;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 3px;
}

.toolbar-btn:hover {
  background: rgba(255, 255, 255, 1);
  border-color: rgba(102, 126, 234, 0.25);
  color: #2d3748;
  box-shadow: 0 2px 6px rgba(102, 126, 234, 0.15);
}

/* 下拉菜单样式 */
.dropdown-menu {
  position: absolute;
  top: 45px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 12px;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  min-width: 140px;
  overflow: hidden;
}

.settings-menu {
  left: 16px;
}

.help-menu {
  left: 80px;
}

.menu-item {
  padding: 9px 14px;
  font-size: 12px;
  font-weight: 500;
  color: #4a5568;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 7px;
}

.menu-item:hover {
  background: rgba(102, 126, 234, 0.08);
  color: #2d3748;
}

.menu-item:not(:last-child) {
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

/* 帮助对话框样式 */
.help-dialog {
  max-width: 520px;
  max-height: 85vh;
  overflow-y: auto;
}

.help-content {
  font-size: 13px;
  color: #4a5568;
  line-height: 1.5;
  margin-bottom: 20px;
}

.help-section {
  margin-bottom: 18px;
}

.help-section h4 {
  font-size: 14px;
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.help-section p, .help-section li {
  margin-bottom: 4px;
  font-size: 13px;
}

.help-section ol, .help-section ul {
  padding-left: 18px;
  margin-top: 4px;
}

.help-section strong {
  color: #2d3748;
  font-weight: 600;
}

.about-content .app-info {
  text-align: center;
  margin-bottom: 18px;
  padding: 16px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.03) 100%);
  border-radius: 12px;
}

.about-content .app-info h4 {
  font-size: 18px;
  font-weight: 700;
  color: #2d3748;
  margin-bottom: 10px;
}

.about-content .app-info p {
  font-size: 13px;
  margin-bottom: 4px;
}

.features, .developer {
  margin-bottom: 16px;
}

.features h4, .developer h4 {
  font-size: 14px;
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.features ul {
  padding-left: 18px;
}

.features li, .developer p {
  font-size: 13px;
  margin-bottom: 4px;
}

/* Toast 样式 */
.toast {
  position: fixed;
  top: 24px;
  right: 24px;
  z-index: 2000;
  padding: 16px 20px;
  border-radius: 12px;
  backdrop-filter: blur(20px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  font-size: 14px;
  font-weight: 500;
  animation: slideInRight 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  max-width: 400px;
  word-wrap: break-word;
}

.toast-success {
  background: rgba(52, 199, 89, 0.9);
  color: white;
  border: 1px solid rgba(52, 199, 89, 0.3);
}

.toast-error {
  background: rgba(255, 59, 48, 0.9);
  color: white;
  border: 1px solid rgba(255, 59, 48, 0.3);
}

.toast-warning {
  background: rgba(255, 149, 0, 0.9);
  color: white;
  border: 1px solid rgba(255, 149, 0, 0.3);
}

.toast-info {
  background: rgba(0, 122, 255, 0.9);
  color: white;
  border: 1px solid rgba(0, 122, 255, 0.3);
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(100%);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.dialog-buttons {
  display: flex;
  gap: 12px;
  justify-content: center;
  margin-top: 20px;
}

.form-group {
  margin-bottom: var(--spacing-20);
}

.form-group:last-of-type {
  margin-bottom: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .container {
    flex-direction: column;
  }
  
  .panel {
    flex: none;
    height: 200px;
  }
  
  .preset-panel {
    height: 300px;
  }
}

@media (max-width: 900px) {
  .main {
    padding: var(--spacing-3);
  }
  
  .container {
    gap: var(--spacing-3);
  }
  
  .panel-header {
    padding: var(--spacing-2) var(--spacing-3);
  }
  
  .form-container, .preset-container {
    padding: var(--spacing-3);
  }
  
  .panel-title {
    font-size: var(--font-size-base);
  }
  
  .dialog {
    min-width: 320px;
    margin: var(--spacing-4);
    padding: var(--spacing-6);
  }
}

/* 高度优化 */
@media (max-height: 600px) {
  .main {
    padding: var(--spacing-2);
  }
  
  .form-container, .preset-container {
    padding: var(--spacing-2);
  }
  
  .form-row {
    gap: var(--spacing-1);
  }
  
  .form-select {
    min-height: 32px;
    height: 32px;
    padding: var(--spacing-1) var(--spacing-2);
    font-size: var(--font-size-xs);
  }
  
  .action-btn {
    min-height: 32px;
    padding: var(--spacing-1) var(--spacing-3);
    font-size: var(--font-size-xs);
  }
  
  .panel-header {
    padding: var(--spacing-2);
  }
  
  .panel-title {
    font-size: var(--font-size-sm);
  }
}
</style>