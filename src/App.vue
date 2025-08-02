<template>
  <el-container class="app-container">
    <!-- 顶部工具栏 -->
    <el-header class="app-header" height="60px">
      <div class="header-content">
        <div class="header-left">
          <h1 class="app-title">
            <el-icon><VideoPlay /></el-icon>
            剑网3改键工具
          </h1>
        </div>
        <div class="header-right">
          <el-dropdown @command="handleCommand" trigger="click">
            <el-button type="primary" :icon="Setting">
              设置 <el-icon class="el-icon--right"><ArrowDown /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="changeFolder" :icon="Folder">更改文件夹</el-dropdown-item>
                <el-dropdown-item command="help" :icon="QuestionFilled">使用帮助</el-dropdown-item>
                <el-dropdown-item command="about" :icon="InfoFilled">关于</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </div>
    </el-header>

    <!-- 主内容区 -->
    <el-main class="app-main">
      <el-row :gutter="20" class="main-row">
        <!-- 左侧：源账号配置 -->
        <el-col :span="8">
          <el-card class="config-card source-card" shadow="hover">
            <template #header>
              <div class="card-header">
                <el-icon class="header-icon"><Upload /></el-icon>
                <span>源账号配置</span>
              </div>
            </template>
            
            <el-form :model="sourceForm" label-position="top" class="config-form">
              <el-form-item 
                v-for="(label, index) in labels" 
                :key="index"
                :label="label"
                class="form-item"
              >
                <el-select 
                  v-model="sourceSelections[index]"
                  :placeholder="`选择${label}`"
                  @change="onSourceChange(index)"
                  class="form-select"
                  clearable
                >
                  <el-option
                    v-for="option in sourceOptions[index]"
                    :key="option"
                    :label="option"
                    :value="option"
                  />
                </el-select>
              </el-form-item>
              
              <el-button 
                type="primary" 
                :icon="DocumentAdd"
                @click="savePreset"
                :disabled="!canSavePreset"
                class="action-button"
                size="large"
              >
                保存预设
              </el-button>
            </el-form>
          </el-card>
        </el-col>

        <!-- 中间：目标账号配置 -->
        <el-col :span="8">
          <el-card class="config-card target-card" shadow="hover">
            <template #header>
              <div class="card-header">
                <el-icon class="header-icon"><Download /></el-icon>
                <span>目标账号配置</span>
              </div>
            </template>
            
            <el-form :model="targetForm" label-position="top" class="config-form">
              <el-form-item 
                v-for="(label, index) in labels" 
                :key="index"
                :label="label"
                class="form-item"
              >
                <el-select 
                  v-model="targetSelections[index]"
                  :placeholder="`选择${label}`"
                  @change="onTargetChange(index)"
                  class="form-select"
                  clearable
                >
                  <el-option
                    v-for="option in targetOptions[index]"
                    :key="option"
                    :label="option"
                    :value="option"
                  />
                </el-select>
              </el-form-item>
              
              <el-button 
                type="success" 
                :icon="Check"
                @click="executeKeyChange"
                :disabled="!canExecute"
                class="action-button"
                size="large"
              >
                执行改键
              </el-button>
            </el-form>
          </el-card>
        </el-col>

        <!-- 右侧：预设管理 -->
        <el-col :span="8">
          <el-card class="config-card preset-card" shadow="hover">
            <template #header>
              <div class="card-header">
                <el-icon class="header-icon"><Collection /></el-icon>
                <span>预设管理</span>
              </div>
            </template>
            
            <div class="preset-content">
              <el-alert
                title="双击加载预设，右键管理预设"
                type="info"
                :closable="false"
                show-icon
                class="preset-hint"
              />
              
              <div v-if="presetNames.length > 0" class="preset-list">
                <el-card 
                  v-for="name in presetNames" 
                  :key="name"
                  class="preset-item"
                  shadow="hover"
                  @dblclick="loadPreset(name)"
                  @contextmenu.prevent="showPresetMenu($event, name)"
                >
                  <div class="preset-item-content">
                    <el-icon class="preset-icon"><Document /></el-icon>
                    <span class="preset-name">{{ name }}</span>
                    <el-tag size="small" type="info">双击</el-tag>
                  </div>
                </el-card>
              </div>
              
              <el-empty v-else description="暂无预设" :image-size="80" />
            </div>
          </el-card>
        </el-col>
      </el-row>
    </el-main>

  </el-container>

  <!-- 右键菜单 -->
  <div 
    v-if="contextMenu.show"
    class="context-menu"
    :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
    @click.stop
  >
    <div class="context-menu-item" @click="handlePresetCommand('rename')">
      <el-icon><Edit /></el-icon>
      <span>重命名预设</span>
    </div>
    <div class="context-menu-divider"></div>
    <div class="context-menu-item danger-item" @click="handlePresetCommand('delete')">
      <el-icon><Delete /></el-icon>
      <span>删除预设</span>
    </div>
  </div>

  <!-- 遮罩层用于关闭右键菜单 -->
  <div 
    v-if="contextMenu.show"
    class="context-menu-overlay"
    @click="contextMenu.show = false"
    @contextmenu.prevent="contextMenu.show = false"
  ></div>

  <!-- 预设名称输入对话框 -->
  <el-dialog
    v-model="showPresetDialog"
    title="保存预设"
    width="400px"
    :before-close="cancelSavePreset"
  >
    <el-form :model="presetForm" label-position="top">
      <el-form-item label="预设名称">
        <el-input
          v-model="presetNameInput"
          placeholder="请输入预设名称"
          @keyup.enter="confirmSavePreset"
          ref="presetInputRef"
        />
      </el-form-item>
    </el-form>
    
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancelSavePreset">取消</el-button>
        <el-button type="primary" @click="confirmSavePreset" :disabled="!presetNameInput.trim()">
          保存
        </el-button>
      </span>
    </template>
  </el-dialog>

  <!-- 重命名预设对话框 -->
  <el-dialog
    v-model="showRenameDialog"
    title="重命名预设"
    width="400px"
    :before-close="cancelRenamePreset"
  >
    <el-form :model="renameForm" label-position="top">
      <el-form-item label="原名称">
        <el-input :value="renamePresetName" disabled />
      </el-form-item>
      <el-form-item label="新名称">
        <el-input
          v-model="renameNewName"
          placeholder="请输入新的预设名称"
          @keyup.enter="confirmRenamePreset"
          ref="renameInputRef"
        />
      </el-form-item>
    </el-form>
    
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancelRenamePreset">取消</el-button>
        <el-button type="primary" @click="confirmRenamePreset" :disabled="!renameNewName.trim()">
          重命名
        </el-button>
      </span>
    </template>
  </el-dialog>

  <!-- 帮助对话框 -->
  <el-dialog
    v-model="showHelpDialog"
    :title="helpTitle"
    width="600px"
    class="help-dialog"
  >
    <div class="help-content" v-html="helpContent"></div>
    
    <template #footer>
      <span class="dialog-footer">
        <el-button type="primary" @click="showHelpDialog = false">确定</el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { message, ask, confirm } from '@tauri-apps/api/dialog'

// Element Plus 图标导入
import { 
  Setting, ArrowDown, Folder, QuestionFilled, InfoFilled,
  Upload, Download, Collection, DocumentAdd, Check, Document,
  Edit, Delete, VideoPlay
} from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'

// 响应式数据
const labels = ['账号', '大区', '区服', '角色']
const basePath = ref('')
const sourceSelections = reactive(['', '', '', ''])
const targetSelections = reactive(['', '', '', ''])
const sourceOptions = reactive([[], [], [], []])
const targetOptions = reactive([[], [], [], []])
const presets = reactive({})

// Element Plus 表单数据
const sourceForm = reactive({})
const targetForm = reactive({})
const presetForm = reactive({})
const renameForm = reactive({})

// 对话框和菜单状态
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
const showHelpDialog = ref(false)
const showHelpMenu = ref(false)
const helpTitle = ref('')
const helpContent = ref('')

// 引用
const presetInputRef = ref()
const renameInputRef = ref()

// 工具函数
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

function savePreset() {
  const timestamp = new Date().toLocaleString('zh-CN').replace(/[\/\s:]/g, '-')
  presetNameInput.value = `预设-${timestamp}`
  showPresetDialog.value = true
  
  // 等待DOM更新后聚焦输入框
  setTimeout(() => {
    if (presetInputRef.value) {
      presetInputRef.value.focus()
      presetInputRef.value.select()
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
  renamePresetName.value = name
  renameNewName.value = name
  showRenameDialog.value = true
  
  // 等待DOM更新后聚焦输入框
  setTimeout(() => {
    if (renameInputRef.value) {
      renameInputRef.value.focus()
      renameInputRef.value.select()
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

// Element Plus 消息提示函数
function showToastMessage(message, type = 'info') {
  const messageType = type === 'warning' ? 'warning' : type === 'error' ? 'error' : type === 'success' ? 'success' : 'info'
  ElMessage({
    message,
    type: messageType,
    duration: 3000,
    showClose: true
  })
}

// Element Plus 确认对话框函数
function showConfirm(title, message) {
  return ElMessageBox.confirm(message, title, {
    confirmButtonText: '确认',
    cancelButtonText: '取消',
    type: 'warning',
    dangerouslyUseHTMLString: true
  }).then(() => true).catch(() => false)
}

// Element Plus 命令处理
function handleCommand(command) {
  switch (command) {
    case 'changeFolder':
      selectBaseFolder()
      break
    case 'help':
      showUsageHelp()
      break
    case 'about':
      showAbout()
      break
  }
}

function handlePresetCommand(command) {
  const presetName = contextMenu.presetName
  contextMenu.show = false
  
  switch (command) {
    case 'rename':
      renamePreset(presetName)
      break
    case 'delete':
      deletePreset(presetName)
      break
  }
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
  event.preventDefault()
  event.stopPropagation()
  
  // 关闭之前的菜单
  contextMenu.show = false
  
  // 使用 nextTick 确保菜单位置正确
  setTimeout(() => {
    contextMenu.x = event.clientX
    contextMenu.y = event.clientY
    contextMenu.presetName = presetName
    contextMenu.show = true
  }, 10)
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
/* Element Plus 主题样式 */
.app-container {
  min-height: 100vh;
  height: 800px;
  background: #f5f7fa;
}

.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
}

.app-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-right {
  display: flex;
  align-items: center;
}

.app-main {
  padding: 20px 24px 20px 24px;
  background: #f5f7fa;
  overflow: hidden;
}

.main-row {
  height: calc(100vh - 40px);
  overflow: hidden;
}

.config-card {
  height: 100%;
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.config-card :deep(.el-card__header) {
  background: linear-gradient(135deg, #f8f9ff 0%, #f0f2ff 100%);
  border-bottom: 1px solid #e4e7ed;
  padding: 18px 22px;
}

.source-card :deep(.el-card__header) {
  background: linear-gradient(135deg, #e3f2fd 0%, #f3e5f5 100%);
}

.target-card :deep(.el-card__header) {
  background: linear-gradient(135deg, #e8f5e8 0%, #f1f8e9 100%);
}

.preset-card :deep(.el-card__header) {
  background: linear-gradient(135deg, #fff3e0 0%, #fce4ec 100%);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 600;
  font-size: 16px;
  color: #303133;
}

.header-icon {
  font-size: 20px;
  color: #409eff;
}

.config-form {
  padding: 8px 0 0 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.form-item {
  margin-bottom: 12px;
  flex-shrink: 0;
}

.form-item :deep(.el-form-item__label) {
  font-weight: 600;
  color: #606266;
  margin-bottom: 8px;
}

.form-select {
  width: 100%;
}

.action-button {
  width: 100%;
  margin-top: auto;
  margin-bottom: 12px;
  height: 40px;
  font-size: 14px;
  font-weight: 600;
  border-radius: 8px;
  flex-shrink: 0;
}

.preset-content {
  padding: 20px 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.preset-hint {
  margin-bottom: 16px;
  border-radius: 8px;
  flex-shrink: 0;
}

.preset-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-height: 0;
  padding-right: 6px;
}

.preset-item {
  cursor: pointer;
  transition: all 0.3s ease;
  border-radius: 8px;
  border: 1px solid #e4e7ed;
}

.preset-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  border-color: #409eff;
}

.preset-item-content {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
}

.preset-icon {
  color: #909399;
  font-size: 16px;
}

.preset-name {
  flex: 1;
  font-weight: 500;
  color: #303133;
}

/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 9999;
  min-width: 140px;
  padding: 4px 0;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  font-size: 14px;
  color: #606266;
}

.context-menu-item:hover {
  background-color: #f5f7fa;
}

.context-menu-item.danger-item {
  color: #f56c6c;
}

.context-menu-item.danger-item:hover {
  background-color: #fef0f0;
}

.context-menu-divider {
  height: 1px;
  background-color: #e4e7ed;
  margin: 4px 0;
}

.context-menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9998;
}

/* 帮助对话框样式 */
.help-dialog :deep(.el-dialog__body) {
  max-height: 60vh;
  overflow-y: auto;
}

.help-content {
  line-height: 1.6;
  color: #606266;
}

.help-content h4 {
  color: #303133;
  margin: 16px 0 8px 0;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.help-content p, .help-content li {
  margin-bottom: 8px;
}

.help-content ol, .help-content ul {
  padding-left: 20px;
  margin: 8px 0;
}

.help-content strong {
  color: #303133;
  font-weight: 600;
}

.about-content .app-info {
  text-align: center;
  margin-bottom: 20px;
  padding: 20px;
  background: linear-gradient(135deg, #f0f2ff 0%, #f8f9ff 100%);
  border-radius: 12px;
  border: 1px solid #e4e7ed;
}

.about-content .app-info h4 {
  font-size: 20px;
  font-weight: 700;
  color: #303133;
  margin-bottom: 12px;
  justify-content: center;
}

.features, .developer {
  margin-bottom: 20px;
}

.features ul {
  padding-left: 20px;
}

.features li, .developer p {
  margin-bottom: 6px;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .main-row :deep(.el-col) {
    margin-bottom: 20px;
  }
  
  .main-row {
    height: auto;
  }
  
  .config-card {
    height: auto;
  }
}

@media (max-width: 768px) {
  .app-main {
    padding: 10px;
  }
  
  .header-content {
    padding: 0 10px;
  }
  
  .app-title {
    font-size: 16px;
  }
  
  .config-form {
    padding: 10px 0;
  }
  
  .preset-content {
    padding: 10px 0;
  }
}
</style>
/* 滚动条样式
优化 */
.preset-list::-webkit-scrollbar {
  width: 6px;
}

.preset-list::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 2px;
}

.preset-list::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 2px;
}

.preset-list::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* 确保卡片内容不会溢出 */
.config-card :deep(.el-card__body) {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 12px 20px 16px 20px;
}

/* 表单项标签样式优化 */
.form-item :deep(.el-form-item__label) {
  font-weight: 600;
  color: #606266;
  margin-bottom: 6px;
  font-size: 13px;
}

/* 选择框样式优化 */
.form-select :deep(.el-input__inner) {
  font-size: 14px;
  height: 36px;
}

/* 空状态样式优化 */
.preset-content :deep(.el-empty) {
  padding: 20px 0;
}

.preset-content :deep(.el-empty__description) {
  font-size: 13px;
  color: #909399;
}