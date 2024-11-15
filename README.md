# skywalker2088
吸血鬼幸存者类型游戏，宇宙科幻废土，玩家驾驶飞船在宇宙中漫游，打怪收集装备升级飞船，研究科技，探索文明的遗迹。

______
# 0. 简介
- 游戏类型：吸血鬼幸存者like
- 游戏背景：玩家是一名太空巡警，在一次巡逻任务中发现了一艘海盗船，在追捕过程中意外地被一个黑洞吸入，进入到一个宇宙遗迹中，这个宇宙的编号是2088，
        为了回到自己所在的宇宙空间，玩家需要驾驶太空飞船打怪、搜集装备、研究科技、探索文明的遗迹，在这个过程中玩家逐渐揭开这个宇宙文明毁灭的
        真相，他需要尽快想办法回到自己所在的宇宙，将2088的案例带给宇宙科学家们，也许他们有办法避免自己宇宙的文明走向崩坏。

______
# 1. 玩法设计
1. 能量收集：太空飞船拥有将太空矿石转化成能量的转化内核，也能吸收转化太空怪物身上的能量晶核，这些能量为飞船提供源源不断的能源供给和升级所需的能量；
2. 飞船升级：机甲类怪物的内核和部件可以作为飞船改造升级的材料，升级改造的方向主要有引擎升级（更快的移动速度）、武器升级、防御升级……升级后的飞船可以应对更加强大的怪物；
3. 科技研究：通过对文明遗迹的探索，玩家可以收集科技点，解锁科技树，使太空飞船拥有更加强大的技能，比如激光射线、引力波探测器、无人机集群、超能粒子炮……
4. 文明遗迹：随着等级提升，玩家可以探索更加高级且危险的文明遗迹，从而获取更加强大的内核，更多的能量和科技点；
5. 美术风格：太空、科幻、废土、文明遗迹。

## 1.1 操作
1. 移动：鼠标左键为飞船导航（TODO：键盘方向键和手柄摇杆支持）
2. 暂停/恢复：键盘空格键（TODO：手柄Start按键）
3. 返回主菜单：键盘ESC（todo：手柄return返回按键）
4. 全屏/窗口切换：F11键盘按键（todo：手柄设置菜单）
5. 缩放：鼠标滚轮/PageUp = +、PageDown = -（Todo：手柄左右Trigger缩放）

______
# 2. 关卡设计
关卡主要分为：1.剧情类关卡；2.遗迹探索类关卡；3.无限深渊关卡。
## 2.1 剧情关卡
todo……
## 2.2 遗迹
todo……
## 2.3 无限深渊
todo……

______
# 3. 开发记录
## 3.1 已实现列表
- [x] 使用`cargo generate thebevyflock/bevy_new_2d`生成的2d游戏模板作为[游戏基础开发框架](https://github.com/TheBevyFlock/bevy_new_2d)；
- [x] 从[ASCII Space Shooter](https://github.com/JamesHDuffield/ascii-rust)开源项目Copy核心逻辑代码，并优化结构；
- [x] 暂停游戏
- [x] WASM支持
- [x] 敌人AI实现
- [x] 游戏音效
- [x] 使用[bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader)优化资源加载
- [x] 使用[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)优化音效播放逻辑和性能
- [x] 支持全屏/窗口切换和鼠标滚轮缩放（2024年11月15日）
- [x] 通过csv来配置数值和多语言（先支持中英双语）

## 3.2 TODO List
- [ ] 支持手柄操作输入（我喜欢用手柄玩，没有手柄可以用键鼠，考虑使用[leafwing-input-manager](https://github.com/leafwing-studios/leafwing-input-manager)插件）
- [ ] 多语言支持（先弄中英文双语，考虑使用[fluent](https://github.com/kgv/bevy_fluent)插件）
- [ ] [LDTK](https://github.com/Trouv/bevy_ecs_ldtk)软件编辑关卡
- [ ] 关卡载入
- [ ] 关卡切换
- [ ] 玩家定点和敌人随机点出生
- [ ] 利用物理引擎进行碰撞检测
- [ ] 玩家出生、护盾、爆炸、水波等精灵动画
- [ ] 游戏UI
- [ ] 游戏存档、读档、自动保存(考虑使用[bevy_pkv](https://github.com/johanhelsing/bevy_pkv)插件开发)
- [ ] 本地多人模式

# 4. 开发指南
## 4.1 游戏引擎：[Bevy](https://bevyengine.org/)

## 4.2 游戏模板
- 使用`cargo generate thebevyflock/bevy_new_2d`生成的2d游戏模板
- 从[ASCII Space Shooter](https://github.com/JamesHDuffield/ascii-rust)开源项目Copy核心逻辑代码

## 4.3 编译优化
- 使用[Sccache](https://github.com/mozilla/sccache)作为编译缓存工具，安装命令：`cargo install sccache --locked`

## 4.4 常用命令列表
- Rust格式化： `cargo fmt`
- clippy提交检查：`cargo clippy --locked --workspace --all-targets --all-features -- --deny warnings`
- 本地开发运行：`cargo run`
