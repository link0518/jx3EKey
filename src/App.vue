<template>
  <el-container class="app-container">
    <!-- 自定义标题栏 -->
    <div class="custom-titlebar">
      <div class="titlebar-content">
        <div class="titlebar-title" data-tauri-drag-region>
          <img src="/fox.svg" alt="狐狸" class="titlebar-icon" />
          <span>毛毛狐改键工具</span>
        </div>
        <div class="titlebar-controls">
          <button class="titlebar-button minimize-btn" @click.stop="minimizeWindow">
            <el-icon><Minus /></el-icon>
          </button>
          <el-dropdown @command="handleCommand" trigger="click" placement="bottom-end">
            <button class="titlebar-button settings-btn" @click.stop>
              <el-icon><Setting /></el-icon>
            </button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="changeFolder" :icon="Folder">更改文件夹</el-dropdown-item>
                <el-dropdown-item command="help" :icon="QuestionFilled">使用帮助</el-dropdown-item>
                <el-dropdown-item command="about" :icon="InfoFilled">关于</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          <button class="titlebar-button close-btn" @click.stop="closeWindow">
            <el-icon><Close /></el-icon>
          </button>
        </div>
      </div>
    </div>
    
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
    width="680px"
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
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { appWindow } from '@tauri-apps/api/window'

// Element Plus 组件和图标
import { 
  Setting, Folder, QuestionFilled, InfoFilled,
  Upload, Download, Collection, DocumentAdd, Check, Document,
  Edit, Delete, Minus, Close
} from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'

// ==================== 常量定义 ====================
const SELECTION_LABELS = ['账号', '大区', '区服', '角色']
const DIALOG_FOCUS_DELAY = 200

// ==================== 响应式数据 ====================
// 基础配置
const basePath = ref('')
const labels = SELECTION_LABELS

// 选择器数据
const sourceSelections = reactive(['', '', '', ''])
const targetSelections = reactive(['', '', '', ''])
const sourceOptions = reactive([[], [], [], []])
const targetOptions = reactive([[], [], [], []])

// 预设管理
const presets = reactive({})

// 表单数据（Element Plus 需要）
const sourceForm = reactive({})
const targetForm = reactive({})
const presetForm = reactive({})
const renameForm = reactive({})

// ==================== UI 状态管理 ====================
// 右键菜单
const contextMenu = reactive({
  show: false,
  x: 0,
  y: 0,
  presetName: ''
})

// 对话框状态
const showPresetDialog = ref(false)
const presetNameInput = ref('')
const showRenameDialog = ref(false)
const renamePresetName = ref('')
const renameNewName = ref('')
const showHelpDialog = ref(false)
const helpTitle = ref('')
const helpContent = ref('')

// DOM 引用
const presetInputRef = ref()
const renameInputRef = ref()

// ==================== 计算属性 ====================
const presetNames = computed(() => Object.keys(presets))

const canSavePreset = computed(() => 
  basePath.value && sourceSelections.some(selection => selection)
)

const canExecute = computed(() => {
  const sourcePath = getSelectedPath(sourceSelections)
  const targetPath = getSelectedPath(targetSelections)
  return sourcePath && targetPath && sourcePath !== targetPath
})

// ==================== 工具函数 ====================
/**
 * 根据选择构建文件路径
 * @param {Array} selections - 选择数组
 * @param {number} maxLevel - 最大层级
 * @returns {string} 构建的路径
 */
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

/**
 * 显示消息提示
 * @param {string} message - 消息内容
 * @param {string} type - 消息类型
 */
function showToastMessage(message, type = 'info') {
  const messageType = ['warning', 'error', 'success'].includes(type) ? type : 'info'
  ElMessage({
    message,
    type: messageType,
    duration: 3000,
    showClose: true
  })
}

/**
 * 显示确认对话框
 * @param {string} title - 标题
 * @param {string} message - 消息内容
 * @returns {Promise<boolean>} 用户确认结果
 */
function showConfirm(title, message) {
  return ElMessageBox.confirm(message, title, {
    confirmButtonText: '确认',
    cancelButtonText: '取消',
    type: 'warning',
    dangerouslyUseHTMLString: true
  }).then(() => true).catch(() => false)
}

// ==================== 生命周期 ====================
onMounted(async () => {
  try {
    // 加载所有应用数据
    await loadAppData()
    
    // 初始化应用状态
    if (!basePath.value) {
      await selectBaseFolder()
    } else {
      await updateOptions()
      await restoreLastSourceSelections()
    }
  } catch (error) {
    console.error('应用初始化失败:', error)
    showToastMessage('应用初始化失败，请重试', 'error')
  }
})

// ==================== 文件夹管理 ====================
/**
 * 选择游戏数据文件夹
 */
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
    console.error('选择文件夹失败:', error)
    showToastMessage(`选择文件夹失败: ${error}`, 'error')
  }
}

// ==================== 选项管理 ====================
/**
 * 更新所有选项
 */
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

/**
 * 更新指定层级的选项
 * @param {string} path - 目录路径
 * @param {number} level - 层级索引
 */
async function updateLevelOptions(path, level) {
  if (level >= labels.length) return
  
  try {
    const subdirs = await invoke('get_subdirectories', { path })
    
    // 更新源和目标选项
    sourceOptions[level].splice(0, sourceOptions[level].length, ...subdirs)
    targetOptions[level].splice(0, targetOptions[level].length, ...subdirs)
    
    console.log(`第${level}级选项已更新，共${subdirs.length}个选项`)
  } catch (error) {
    console.error(`获取第${level}级子目录失败:`, error)
  }
}



// ==================== 选择器事件处理 ====================
/**
 * 处理源账号选择变更
 * @param {number} level - 变更的层级
 */
async function onSourceChange(level) {
  console.log(`源账号选择变更: level=${level}, value=${sourceSelections[level]}`)
  
  // 清空后续级别的选择和选项
  clearSubsequentLevels(sourceSelections, sourceOptions, level)
  
  // 如果当前级别有选择，更新下一级选项
  if (sourceSelections[level] && level + 1 < labels.length) {
    await updateNextLevelOptions(sourceSelections, sourceOptions, targetOptions, level)
  }
  
  // 保存配置
  await saveConfig()
  console.log('源账号选择处理完成')
}

/**
 * 处理目标账号选择变更
 * @param {number} level - 变更的层级
 */
async function onTargetChange(level) {
  console.log(`目标账号选择变更: level=${level}, value=${targetSelections[level]}`)
  
  // 清空后续级别的选择和选项
  clearSubsequentLevels(targetSelections, targetOptions, level)
  
  // 如果当前级别有选择，更新下一级选项
  if (targetSelections[level] && level + 1 < labels.length) {
    await updateNextLevelOptions(targetSelections, [targetOptions], null, level)
  }
  
  console.log('目标账号选择处理完成')
}

/**
 * 清空指定层级之后的所有选择和选项
 * @param {Array} selections - 选择数组
 * @param {Array} options - 选项数组
 * @param {number} level - 起始层级
 */
function clearSubsequentLevels(selections, options, level) {
  for (let i = level + 1; i < selections.length; i++) {
    selections[i] = ''
    if (options[i]) {
      options[i].splice(0)
    }
  }
}

/**
 * 更新下一级选项并自动选择
 * @param {Array} selections - 选择数组
 * @param {Array} sourceOpts - 源选项数组
 * @param {Array} targetOpts - 目标选项数组
 * @param {number} level - 当前层级
 */
async function updateNextLevelOptions(selections, sourceOpts, targetOpts, level) {
  const nextLevel = level + 1
  const path = getSelectedPath(selections, nextLevel)
  
  console.log(`准备更新第${nextLevel}级选项: path=${path}`)
  
  try {
    const subdirs = await invoke('get_subdirectories', { path })
    console.log(`获取到${subdirs.length}个子目录:`, subdirs)
    
    // 更新选项
    sourceOpts[nextLevel].splice(0, sourceOpts[nextLevel].length, ...subdirs)
    if (targetOpts && targetOpts[nextLevel]) {
      targetOpts[nextLevel].splice(0, targetOpts[nextLevel].length, ...subdirs)
    }
    
    // 自动选择第一个选项
    if (subdirs.length > 0) {
      selections[nextLevel] = subdirs[0]
      console.log(`自动选择: ${labels[nextLevel]} = ${subdirs[0]}`)
      
      // 递归更新下一级
      if (nextLevel + 1 < labels.length) {
        await updateNextLevelOptions(selections, sourceOpts, targetOpts, nextLevel)
      }
    }
  } catch (error) {
    console.error(`获取第${nextLevel}级子目录失败:`, error)
  }
}



// ==================== 预设管理 ====================
/**
 * 开始保存预设流程
 */
function savePreset() {
  const timestamp = new Date().toLocaleString('zh-CN').replace(/[\/\s:]/g, '-')
  presetNameInput.value = `预设-${timestamp}`
  showPresetDialog.value = true
  
  // 聚焦输入框
  focusInput(presetInputRef)
}

/**
 * 确认保存预设
 */
async function confirmSavePreset() {
  try {
    const name = presetNameInput.value.trim()
    if (!name) return
    
    // 检查是否需要覆盖
    if (presets[name]) {
      const confirmed = await showConfirm('确认覆盖', `预设 '${name}' 已存在，是否覆盖？`)
      if (!confirmed) return
    }
    
    // 保存预设数据
    presets[name] = [basePath.value, ...sourceSelections]
    await savePresets()
    
    // 关闭对话框并清空输入
    closePresetDialog()
    showToastMessage(`预设 '${name}' 已保存`, 'success')
  } catch (error) {
    console.error('保存预设失败:', error)
    showToastMessage(`保存预设失败: ${error}`, 'error')
  }
}

/**
 * 取消保存预设
 */
function cancelSavePreset() {
  closePresetDialog()
}

/**
 * 关闭预设对话框
 */
function closePresetDialog() {
  showPresetDialog.value = false
  presetNameInput.value = ''
}

/**
 * 开始重命名预设
 * @param {string} name - 预设名称
 */
function renamePreset(name) {
  renamePresetName.value = name
  renameNewName.value = name
  showRenameDialog.value = true
  
  // 聚焦输入框
  focusInput(renameInputRef)
}

/**
 * 确认重命名预设
 */
async function confirmRenamePreset() {
  try {
    const oldName = renamePresetName.value
    const newName = renameNewName.value.trim()
    
    if (!newName) return
    
    // 如果名称没有变化，直接关闭
    if (newName === oldName) {
      closeRenameDialog()
      return
    }
    
    // 检查新名称是否已存在
    if (presets[newName]) {
      const confirmed = await showConfirm('确认覆盖', `预设 '${newName}' 已存在，是否覆盖？`)
      if (!confirmed) return
    }
    
    // 执行重命名
    presets[newName] = presets[oldName]
    delete presets[oldName]
    await savePresets()
    
    closeRenameDialog()
    showToastMessage(`预设已重命名为 '${newName}'`, 'success')
  } catch (error) {
    console.error('重命名预设失败:', error)
    showToastMessage(`重命名预设失败: ${error}`, 'error')
  }
}

/**
 * 取消重命名预设
 */
function cancelRenamePreset() {
  closeRenameDialog()
}

/**
 * 关闭重命名对话框
 */
function closeRenameDialog() {
  showRenameDialog.value = false
  renamePresetName.value = ''
  renameNewName.value = ''
}

/**
 * 聚焦输入框
 * @param {Ref} inputRef - 输入框引用
 */
function focusInput(inputRef) {
  nextTick(() => {
    setTimeout(() => {
      if (inputRef.value) {
        inputRef.value.focus()
        inputRef.value.select()
      }
    }, DIALOG_FOCUS_DELAY)
  })
}

// ==================== 命令处理 ====================
/**
 * 处理下拉菜单命令
 * @param {string} command - 命令类型
 */
function handleCommand(command) {
  const commandHandlers = {
    changeFolder: selectBaseFolder,
    help: showUsageHelp,
    about: showAbout
  }
  
  const handler = commandHandlers[command]
  if (handler) {
    handler()
  }
}

/**
 * 处理预设右键菜单命令
 * @param {string} command - 命令类型
 */
function handlePresetCommand(command) {
  const presetName = contextMenu.presetName
  contextMenu.show = false
  
  const commandHandlers = {
    rename: () => renamePreset(presetName),
    delete: () => deletePreset(presetName)
  }
  
  const handler = commandHandlers[command]
  if (handler) {
    handler()
  }
}

// ==================== 帮助和关于 ====================
/**
 * 显示使用帮助
 */
function showUsageHelp() {
  helpTitle.value = '使用帮助'
  helpContent.value = `
    <div class="help-section">
      <h4>🎯 使用说明</h4>
      <p>把exe文件放入到文件夹中执行，执行后会生成“app_data.json”文件</p>
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

/**
 * 显示关于对话框
 */
function showAbout() {
  helpTitle.value = '关于'
  helpContent.value = generateAboutContent()
  showHelpDialog.value = true
}

/**
 * 生成关于内容
 * @returns {string} HTML格式的关于内容
 */
function generateAboutContent() {
  return `
    <div class="about-container">
      <div class="app-header">
        <div class="app-icon">🦊</div>
        <div class="app-info">
          <h3>毛毛狐改键工具</h3>
          <p class="version">v3.0.0 Tauri Edition</p>
        </div>
      </div>
      
      <div class="feature-grid">
        <div class="feature-item">
          <div class="feature-icon">🚀</div>
          <span>现代化界面</span>
        </div>
        <div class="feature-item">
          <div class="feature-icon">⚡</div>
          <span>高性能应用</span>
        </div>
        <div class="feature-item">
          <div class="feature-icon">🔧</div>
          <span>智能预设管理</span>
        </div>
        <div class="feature-item">
          <div class="feature-icon">🛡️</div>
          <span>安全操作</span>
        </div>
      </div>
      
      <div class="tech-stack">
        <p class="tech-title">技术栈</p>
        <p class="tech-desc">Vue 3 + Tauri + Rust + Element Plus</p>
      </div>
      
      <div class="description">
        <p>专为剑网3玩家打造的键位配置管理工具，支持快速复制角色键位配置，让你轻松管理多个角色的按键设置。</p>
      </div>
      
      <div class="author-info">
        <p>开发者：咕涌</p>
        <p class="copyright">© 2025 All rights reserved</p>
      </div>
    </div>
  `
}

/**
 * 加载预设配置
 * @param {string} name - 预设名称
 */
async function loadPreset(name) {
  try {
    if (!presets[name]) {
      showToastMessage('预设不存在', 'warning')
      return
    }
    
    console.log(`加载预设: ${name}`)
    const [savedBasePath, ...selections] = presets[name]
    
    // 设置基础路径
    basePath.value = savedBasePath
    
    // 清空当前选择
    sourceSelections.forEach((_, i) => sourceSelections[i] = '')
    
    // 重新初始化选项
    await updateOptions()
    
    // 逐级恢复选择
    await restorePresetSelections(selections)
    
    await saveConfig()
    console.log('预设加载完成')
    showToastMessage(`预设 '${name}' 加载成功`, 'success')
  } catch (error) {
    console.error('加载预设失败:', error)
    showToastMessage(`加载预设失败: ${error}`, 'error')
  }
}

/**
 * 恢复预设的选择配置
 * @param {Array} selections - 预设的选择数组
 */
async function restorePresetSelections(selections) {
  for (let i = 0; i < selections.length && i < sourceSelections.length; i++) {
    if (!selections[i]) continue
    
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

/**
 * 显示预设右键菜单
 * @param {Event} event - 鼠标事件
 * @param {string} presetName - 预设名称
 */
function showPresetMenu(event, presetName) {
  event.preventDefault()
  event.stopPropagation()
  
  // 关闭之前的菜单
  contextMenu.show = false
  
  // 延迟显示确保位置正确
  nextTick(() => {
    setTimeout(() => {
      contextMenu.x = event.clientX
      contextMenu.y = event.clientY
      contextMenu.presetName = presetName
      contextMenu.show = true
    }, 10)
  })
}

/**
 * 删除预设
 * @param {string} name - 预设名称
 */
async function deletePreset(name) {
  contextMenu.show = false
  
  try {
    const confirmed = await showConfirm(
      '确认删除', 
      `确定要删除预设 '${name}' 吗？\n\n此操作无法撤销。`
    )
    
    if (confirmed) {
      delete presets[name]
      await savePresets()
      showToastMessage(`预设 '${name}' 已删除`, 'success')
    }
  } catch (error) {
    console.error('删除预设失败:', error)
    showToastMessage(`删除预设失败: ${error}`, 'error')
  }
}

// ==================== 核心功能 ====================
/**
 * 执行改键操作
 */
async function executeKeyChange() {
  const sourcePath = getSelectedPath(sourceSelections)
  const targetPath = getSelectedPath(targetSelections)
  
  // 验证路径
  if (!validatePaths(sourcePath, targetPath)) return
  
  try {
    // 构建确认信息
    const sourceName = sourceSelections.filter(s => s).join(' → ')
    const targetName = targetSelections.filter(s => s).join(' → ')
    
    const confirmed = await showConfirm(
      '确认改键',
      `确认要执行改键操作吗？\n\n源配置: ${sourceName}\n目标位置: ${targetName}\n\n此操作将覆盖目标位置的现有配置`
    )
    
    if (!confirmed) return
    
    // 执行复制操作
    await invoke('copy_directory', { source: sourcePath, target: targetPath })
    showToastMessage('键位配置已成功复制到目标位置！建议重启游戏以确保配置生效', 'success')
  } catch (error) {
    console.error('改键操作失败:', error)
    showToastMessage(`改键操作失败: ${error}`, 'error')
  }
}

/**
 * 验证源路径和目标路径
 * @param {string} sourcePath - 源路径
 * @param {string} targetPath - 目标路径
 * @returns {boolean} 验证结果
 */
function validatePaths(sourcePath, targetPath) {
  if (!sourcePath || !targetPath) {
    showToastMessage('请确保源路径和目标路径都已选择', 'warning')
    return false
  }
  
  if (sourcePath === targetPath) {
    showToastMessage('源路径和目标路径不能相同', 'warning')
    return false
  }
  
  return true
}

// ==================== 数据持久化 ====================
/**
 * 保存所有应用数据到 app_data.json
 */
async function saveAppData() {
  try {
    const appData = {
      config: {
        base_path: basePath.value,
        last_left_path: getSelectedPath(sourceSelections),
        last_source_selections: [...sourceSelections]
      },
      presets: { ...presets },
      version: '3.0.0',
      last_updated: new Date().toISOString()
    }
    
    await invoke('save_app_data', { data: appData })
    console.log('应用数据保存成功')
  } catch (error) {
    console.error('保存应用数据失败:', error)
  }
}

/**
 * 从 app_data.json 加载所有应用数据
 */
async function loadAppData() {
  try {
    const appData = await invoke('load_app_data')
    console.log('加载的应用数据:', appData)
    
    // 加载配置
    if (appData.config) {
      basePath.value = appData.config.base_path || ''
      
      // 恢复源账号选择
      if (appData.config.last_source_selections?.length > 0) {
        console.log('恢复源账号选择:', appData.config.last_source_selections)
        
        for (let i = 0; i < Math.min(appData.config.last_source_selections.length, sourceSelections.length); i++) {
          sourceSelections[i] = appData.config.last_source_selections[i] || ''
        }
        
        console.log('恢复后的sourceSelections:', [...sourceSelections])
      }
    }
    
    // 加载预设
    if (appData.presets) {
      Object.assign(presets, appData.presets)
      console.log('预设数据加载完成，共', Object.keys(presets).length, '个预设')
    }
    
    // 显示版本信息（如果需要）
    if (appData.version) {
      console.log('数据文件版本:', appData.version)
    }
    
  } catch (error) {
    console.error('加载应用数据失败:', error)
    // 如果加载失败，尝试创建默认数据文件
    await saveAppData()
  }
}

/**
 * 保存应用配置（兼容性函数）
 */
async function saveConfig() {
  await saveAppData()
}

/**
 * 加载应用配置（兼容性函数）
 */
async function loadConfig() {
  await loadAppData()
}

/**
 * 保存预设数据（兼容性函数）
 */
async function savePresets() {
  await saveAppData()
}

/**
 * 加载预设数据（兼容性函数）
 */
async function loadPresets() {
  // 预设数据已在 loadAppData 中加载，这里不需要额外操作
}

/**
 * 恢复上次的源账号选择
 */
async function restoreLastSourceSelections() {
  try {
    console.log('开始恢复上次源账号选择:', sourceSelections)
    
    // 逐级恢复选择并更新选项
    for (let i = 0; i < sourceSelections.length; i++) {
      if (!sourceSelections[i]) {
        console.log(`第${i}级选择为空，停止恢复`)
        break
      }
      
      console.log(`恢复第${i}级选择: ${labels[i]} = ${sourceSelections[i]}`)
      
      // 更新下一级选项
      if (i + 1 < labels.length) {
        const success = await restoreNextLevelOptions(i)
        if (!success) break
      }
    }
    
    console.log('上次源账号选择恢复完成')
  } catch (error) {
    console.error('恢复上次选择失败:', error)
  }
}

/**
 * 恢复下一级选项
 * @param {number} currentLevel - 当前层级
 * @returns {boolean} 是否成功
 */
async function restoreNextLevelOptions(currentLevel) {
  const nextLevel = currentLevel + 1
  const path = getSelectedPath(sourceSelections, nextLevel)
  
  console.log(`获取第${nextLevel}级选项，路径: ${path}`)
  
  try {
    const subdirs = await invoke('get_subdirectories', { path })
    console.log(`第${nextLevel}级选项:`, subdirs)
    
    // 更新选项
    sourceOptions[nextLevel].splice(0, sourceOptions[nextLevel].length, ...subdirs)
    targetOptions[nextLevel].splice(0, targetOptions[nextLevel].length, ...subdirs)
    
    return true
  } catch (error) {
    console.error(`恢复第${nextLevel}级选项失败:`, error)
    
    // 清空后续选择
    for (let j = nextLevel; j < sourceSelections.length; j++) {
      sourceSelections[j] = ''
    }
    
    return false
  }
}

// ==================== 窗口控制 ====================
/**
 * 最小化窗口
 */
async function minimizeWindow() {
  try {
    await appWindow.minimize()
    console.log('窗口最小化成功')
  } catch (error) {
    console.error('最小化窗口失败:', error)
    showToastMessage('最小化窗口失败', 'error')
  }
}

/**
 * 关闭窗口
 */
async function closeWindow() {
  try {
    await appWindow.close()
  } catch (error) {
    console.error('关闭窗口失败:', error)
    showToastMessage('关闭窗口失败', 'error')
  }
}
</script>

<style scoped>
/* 自定义标题栏样式 */
.custom-titlebar {
  height: 32px;
  width: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  user-select: none;
  -webkit-user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}

.titlebar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  height: 100%;
  padding: 0 16px;
}

.titlebar-title {
  display: flex;
  align-items: center;
  gap: 6px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  flex: 1; /* 占据剩余空间作为拖拽区域 */
  height: 100%;
  cursor: default;
}

.titlebar-icon {
  width: 20px;
  height: 20px;
  display: inline-block;
  margin-right: 6px;
  vertical-align: middle;
}

.titlebar-controls {
  display: flex;
  align-items: center;
  height: 100%;
  pointer-events: auto; /* 确保按钮可以接收点击事件 */
}

.titlebar-button {
  width: 46px;
  height: 32px;
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
  font-size: 12px;
  pointer-events: auto; /* 确保按钮可以接收点击事件 */
  z-index: 1001; /* 确保按钮在拖拽区域之上 */
}

.titlebar-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.titlebar-button.close-btn:hover {
  background-color: #e81123;
}

.titlebar-button:active {
  background-color: rgba(255, 255, 255, 0.2);
}

.titlebar-button.close-btn:active {
  background-color: #c50e1f;
}

.titlebar-button.settings-btn:hover {
  background-color: rgba(255, 255, 255, 0.15);
}

/* 标题栏下拉菜单样式 */
.titlebar-controls .el-dropdown {
  height: 100%;
}

.titlebar-controls .el-dropdown .titlebar-button {
  width: 46px;
  height: 32px;
}

/* Element Plus 主题样式 */
.app-container {
  min-height: 100vh;
  height: 800px;
  background: #f5f7fa;
  padding-top: 52px; /* 为固定标题栏留出空间 */
}



.app-main {
  padding: 20px 24px;
  background: #f5f7fa;
  overflow: hidden;
}

.main-row {
  height: calc(100vh - 64px); /* 减去标题栏32px + padding-top 32px */
  overflow: hidden;
}

.config-card {
  height: 457.65px;
  max-height: 457.65px; /* 设置最大高度防止延长 */
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
  padding: 0px 0 8px 0; /* 减小顶部padding，把提示框往上移 */
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
  height: calc(457.65px - 50px); /* 减去header和padding的高度 */
}

.preset-hint {
  margin-top: -4px; /* 添加负的顶部margin，继续上移 */
  margin-bottom: 10px;
  border-radius: 6px;
  flex-shrink: 0;
}

.preset-hint :deep(.el-alert__content) {
  font-size: 15px; /* 缩小提示文字 */
  padding: 0;
}

.preset-hint :deep(.el-alert__title) {
  font-size: 14px; /* 缩小标题文字 */
  margin: 0;
}

.preset-hint :deep(.el-alert) {
  padding: 6px 8px; /* 缩小提示框内边距 */
}

.preset-list {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  gap: 4px; /* 进一步减小预设项目之间的间距 */
  min-height: 0;
  max-height: calc(100% - 70px); /* 减去提示框的高度 */
  margin-top: 8px; /* 把预设列表往下移8px，为悬停效果留出空间 */
  padding-right: 6px;
}

.preset-item {
  cursor: pointer;
  transition: all 0.3s ease;
  border-radius: 4px;
  border: 1px solid #e4e7ed;
  flex-shrink: 0; /* 防止被压缩 */
  margin-top: 4px;
  min-height: 28px; /* 设置更小的最小高度 */
}

.preset-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  border-color: #409eff;
}

.preset-item-content {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px; /* 进一步减小内边距 */
  min-height: 24px; /* 设置更小的内容高度 */
}

.preset-icon {
  color: #909399;
  font-size: 12px; /* 进一步减小图标尺寸 */
}

.preset-name {
  flex: 1;
  font-weight: 500;
  color: #303133;
  font-size: 12px; /* 进一步减小文字尺寸 */
  line-height: 1.2; /* 减小行高 */
}

.preset-item-content .el-tag {
  font-size: 10px; /* 缩小标签文字 */
  height: 18px; /* 缩小标签高度 */
  line-height: 16px;
  padding: 0 4px; /* 缩小标签内边距 */
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
  max-height: 70vh;
  overflow-y: auto;
  padding: var(--spacing-8);
}

.help-dialog :deep(.el-dialog__header) {
  padding: var(--spacing-20) var(--spacing-24) var(--spacing-16);
  border-bottom: 1px solid var(--separator);
}

.help-dialog :deep(.el-dialog__title) {
  font-size: var(--font-size-title3);
  font-weight: var(--font-weight-semibold);
  color: var(--fg-primary);
}

/* 帮助页面样式 */
.help-container {
  padding: var(--spacing-8);
}

.help-step {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-16);
  margin-bottom: var(--spacing-24);
  padding: var(--spacing-16);
  background: var(--bg-tertiary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-primary);
  transition: all var(--duration-quick) var(--timing-ease-out);
}

.help-step:hover {
  background: var(--bg-secondary);
  box-shadow: var(--shadow-1);
  transform: translateY(-1px);
}

.step-number {
  width: 32px;
  height: 32px;
  background: var(--blue);
  color: white;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: var(--font-weight-semibold);
  font-size: var(--font-size-callout);
  flex-shrink: 0;
  box-shadow: var(--shadow-1);
}

.step-content {
  flex: 1;
}

.step-content h4 {
  margin: 0 0 var(--spacing-8) 0;
  font-size: var(--font-size-headline);
  font-weight: var(--font-weight-semibold);
  color: var(--fg-primary);
  letter-spacing: -0.022em;
}

.step-content p {
  margin: 0 0 var(--spacing-8) 0;
  color: var(--fg-secondary);
  font-size: var(--font-size-subheadline);
  line-height: 1.5;
}

.step-content code {
  display: block;
  background: var(--bg-quaternary);
  padding: var(--spacing-8) var(--spacing-12);
  border-radius: var(--radius-sm);
  font-family: var(--font-family-mono);
  font-size: var(--font-size-footnote);
  color: var(--blue);
  border: 1px solid var(--border-secondary);
  margin-top: var(--spacing-8);
}

.help-tips {
  margin-top: var(--spacing-32);
  padding: var(--spacing-20);
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-primary);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

.tip-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-8) 0;
  font-size: var(--font-size-subheadline);
  color: var(--fg-secondary);
}

.tip-item:not(:last-child) {
  border-bottom: 1px solid var(--separator);
  margin-bottom: var(--spacing-8);
  padding-bottom: var(--spacing-16);
}

.tip-item strong {
  color: var(--fg-primary);
  font-weight: var(--font-weight-semibold);
}

/* 关于页面样式 */
.about-container {
  padding: var(--spacing-16);
  text-align: center;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-16);
  margin-bottom: var(--spacing-32);
  padding: var(--spacing-24);
  background: var(--bg-elevated);
  border-radius: var(--radius-2xl);
  border: 1px solid var(--border-primary);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

.app-icon {
  font-size: 48px;
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.1));
}

.app-info h3 {
  margin: 0 0 var(--spacing-4) 0;
  font-size: var(--font-size-title2);
  font-weight: var(--font-weight-bold);
  color: var(--fg-primary);
  letter-spacing: -0.022em;
}

.version {
  margin: 0;
  font-size: var(--font-size-footnote);
  color: var(--fg-tertiary);
  font-weight: var(--font-weight-medium);
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-16);
  margin-bottom: var(--spacing-32);
}

.feature-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-8);
  padding: var(--spacing-20);
  background: var(--bg-tertiary);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-primary);
  transition: all var(--duration-quick) var(--timing-ease-out);
}

.feature-item:hover {
  background: var(--bg-secondary);
  box-shadow: var(--shadow-2);
  transform: translateY(-2px);
}

.feature-icon {
  font-size: 24px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
}

.feature-item span {
  font-size: var(--font-size-footnote);
  font-weight: var(--font-weight-medium);
  color: var(--fg-secondary);
}

.tech-stack {
  margin-bottom: var(--spacing-32);
  padding: var(--spacing-20);
  background: var(--bg-tertiary);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-primary);
}

.tech-title {
  margin: 0 0 var(--spacing-8) 0;
  font-size: var(--font-size-subheadline);
  font-weight: var(--font-weight-semibold);
  color: var(--fg-primary);
}

.tech-desc {
  margin: 0;
  font-size: var(--font-size-footnote);
  color: var(--fg-secondary);
  font-weight: var(--font-weight-medium);
}

.author-info {
  padding: var(--spacing-16);
  border-top: 1px solid var(--separator);
}

.author-info p {
  margin: var(--spacing-4) 0;
  font-size: var(--font-size-footnote);
  color: var(--fg-secondary);
}

.copyright {
  font-size: var(--font-size-caption1) !important;
  color: var(--fg-tertiary) !important;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .main-row :deep(.el-col) {
    margin-bottom: 20px;
  }
  
  .main-row {
    height: calc(100vh - 64px); /* 减去标题栏和padding */
  }
  
  .config-card {
    height: 457.65px;
    max-height: 457.65px;
  }
  
  .preset-card {
    height: 457.65px !important;
    max-height: 457.65px !important;
  }
}

@media (max-width: 768px) {
  .app-main {
    padding: 10px;
  }
  
  .config-form {
    padding: 10px 0;
  }
  
  /* 帮助和关于页面响应式 */
  .feature-grid {
    grid-template-columns: 1fr;
  }
  
  .app-header {
    flex-direction: column;
    text-align: center;
  }
  
  .help-step {
    flex-direction: column;
    text-align: center;
  }
  
  .step-number {
    align-self: center;
  }
}
  
  .preset-content {
    padding: 10px 0;
  }
  
  /* 滚动条样式优化 */
  .preset-list::-webkit-scrollbar {
    width: 8px;
  }

  .preset-list::-webkit-scrollbar-track {
    background: #f1f1f1;
    border-radius: 4px;
  }

  .preset-list::-webkit-scrollbar-thumb {
    background: #409eff;
    border-radius: 4px;
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
</style>
/* 关于对话框
样式 */
.about-container {
  text-align: center;
  padding: 20px;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 24px;
  gap: 16px;
}

.app-icon {
  font-size: 48px;
  line-height: 1;
}

.app-info h3 {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
  color: #303133;
}

.app-info .version {
  margin: 0;
  font-size: 14px;
  color: #909399;
  font-weight: 500;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
  margin: 24px 0;
  padding: 0 20px;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e9ecef;
}

.feature-icon {
  font-size: 20px;
  line-height: 1;
}

.feature-item span {
  font-size: 14px;
  font-weight: 500;
  color: #495057;
}

.tech-stack {
  margin: 24px 0;
  padding: 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  color: white;
}

.tech-title {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
}

.tech-desc {
  margin: 0;
  font-size: 14px;
  opacity: 0.9;
}

.description {
  margin: 24px 0;
  padding: 16px;
  background: #f0f9ff;
  border-radius: 8px;
  border-left: 4px solid #3b82f6;
}

.description p {
  margin: 0;
  font-size: 14px;
  line-height: 1.6;
  color: #374151;
}

.author-info {
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid #e5e7eb;
}

.author-info p {
  margin: 4px 0;
  font-size: 14px;
  color: #6b7280;
}

.copyright {
  font-size: 12px !important;
  opacity: 0.7;
}

/* 帮助对话框样式 */
.help-content {
  line-height: 1.6;
}

.help-section {
  margin-bottom: 24px;
}

.help-section h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
}

.help-section p {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: #606266;
  line-height: 1.6;
}

.help-section ol,
.help-section ul {
  margin: 0;
  padding-left: 20px;
}

.help-section li {
  margin-bottom: 8px;
  font-size: 14px;
  color: #606266;
  line-height: 1.6;
}

.help-section code {
  background: #f5f5f5;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  color: #e74c3c;
}