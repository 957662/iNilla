//! **Settings**: iNilla's own switches, grouped for a thumb. Game settings (graphics details,
//! sound volumes, key bindings, the interface options) stay in the stock options windows — the
//! Game menu is one tap away on the radial.

use bevy_egui::egui;

use benilla_world::build_id::BuildId;
use inilla_core::pad::{Layer, PadBinding, PadButton, PadProfile, Special};
use inilla_core::settings::Quality;
use inilla_core::touch::default_layout;

use super::{hint, row, Overlay, Panel, SettingsTab};
use crate::inilla::data::DataState;
use crate::inilla::hud::GOLD;
use crate::inilla::pad::PadHud;
use crate::inilla::Inilla;

pub(super) fn show(
    ui: &mut egui::Ui,
    overlay: &mut Overlay,
    inilla: &mut Inilla,
    data: &DataState,
    pad: &PadHud,
    build: &BuildId,
) {
    ui.horizontal_wrapped(|ui| {
        for (tab, label) in [
            (SettingsTab::General, "通用"),
            (SettingsTab::Touch, "触屏"),
            (SettingsTab::Controller, "手柄"),
            (SettingsTab::Audio, "音频"),
            (SettingsTab::Debug, "调试"),
            (SettingsTab::About, "关于"),
        ] {
            if ui
                .add(egui::Button::selectable(
                    overlay.tab == tab,
                    egui::RichText::new(label).size(18.0),
                ))
                .clicked()
            {
                overlay.tab = tab;
            }
        }
    });
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| match overlay.tab {
        SettingsTab::General => general(ui, inilla),
        SettingsTab::Touch => touch(ui, overlay, inilla),
        SettingsTab::Controller => controller(ui, inilla, pad),
        SettingsTab::Audio => audio(ui, inilla),
        SettingsTab::Debug => debug(ui, overlay, inilla),
        SettingsTab::About => about(ui, inilla, data, build),
    });
}

fn general(ui: &mut egui::Ui, k: &mut Inilla) {
    let s = &mut k.settings;
    row(ui, "画面", |ui| {
        for q in Quality::ALL {
            ui.radio_value(&mut s.quality, q, q.label());
        }
    });
    hint(ui, "省电模式限制 30 帧并缩短视野；高画质按屏幕刷新率运行；自定义则保留原版视频选项。");
    row(ui, "渲染分辨率", |ui| {
        ui.add(
            egui::Slider::new(&mut s.render_scale, 0.5..=1.0)
                .step_by(0.05)
                .custom_formatter(|v, _| format!("{:.0} %", v * 100.0)),
        );
    });
    hint(ui, "3D 世界按此比例渲染，界面保持清晰。手机推荐 80%。");
    let mut auto = s.ui_scale == 0.0;
    row(ui, "Interface size", |ui| {
        if ui.checkbox(&mut auto, "自动").changed() {
            s.ui_scale = if auto { 0.0 } else { 1.25 };
        }
        if !auto {
            ui.add(egui::Slider::new(&mut s.ui_scale, 0.64..=1.5).step_by(0.01));
        }
    });
    hint(
        ui,
        "手机上自动值为 1.25：技能按钮接近指尖大小，任务与商人窗口不遮挡动作条。",
    );
    ui.checkbox(&mut s.keep_screen_on, "游玩时保持屏幕常亮");
    ui.checkbox(
        &mut s.auto_keyboard,
        "聚焦输入框时显示键盘",
    );
    ui.checkbox(&mut s.haptics, "震动反馈");
    ui.checkbox(
        &mut s.compact_stock_ui,
        "精简原版界面（隐藏动作条狮鹫）",
    );
    ui.checkbox(
        &mut s.island_margin,
        "窗口避开灵动岛",
    );
    hint(ui, "游戏窗口在灵动岛（或刘海）边界停止，无论在哪一侧；世界仍铺满屏幕，菜单按钮位于灵动岛旁。");
}

fn touch(ui: &mut egui::Ui, overlay: &mut Overlay, k: &mut Inilla) {
    let l = &mut k.layout;
    ui.checkbox(&mut l.enabled, "显示触屏控制");
    ui.checkbox(
        &mut k.settings.hide_touch_with_controller,
        "使用手柄时隐藏",
    );
    row(ui, "大小", |ui| {
        ui.add(egui::Slider::new(&mut l.hud_scale, 0.6..=1.6).step_by(0.05));
    });
    row(ui, "不透明度", |ui| {
        ui.add(egui::Slider::new(&mut l.opacity, 0.15..=1.0).step_by(0.05));
    });
    row(ui, "视图", |ui| {
        let views = l.views.clone();
        for v in views {
            ui.radio_value(&mut l.active_view, v.clone(), v);
        }
    });
    hint(ui, "Views swap the buttons around the jump button: Combat shows action slots, Explore travel verbs, Social chat verbs, Minimal only the essentials. The view button (its icon shows the view) cycles them in play.");
    ui.horizontal(|ui| {
        if ui
            .add(
                egui::Button::new(egui::RichText::new("编辑布局…").color(GOLD))
                    .min_size(egui::vec2(160.0, 44.0)),
            )
            .clicked()
        {
            overlay.panel = Some(Panel::Editor);
        }
        if ui.button("重置布局").clicked() {
            let enabled = l.enabled;
            *l = default_layout();
            l.enabled = enabled;
        }
    });
    ui.separator();
    ui.label(egui::RichText::new("手感").strong());
    let f = &mut l.feel;
    row(ui, "视角灵敏度", |ui| {
        ui.add(egui::Slider::new(&mut f.look_sensitivity, 0.3..=4.0).step_by(0.1));
    });
    ui.checkbox(&mut f.invert_y, "反转上下视角");
    ui.checkbox(
        &mut f.drag_turns_character,
        "拖动时角色随镜头转向",
    );
    row(ui, "长按（右键）", |ui| {
        ui.add(egui::Slider::new(&mut f.long_press_ms, 250..=900).suffix(" ms"));
    });
    row(ui, "双击窗口", |ui| {
        ui.add(egui::Slider::new(&mut f.double_tap_ms, 150..=500).suffix(" ms"));
    });
    row(ui, "摇杆大小", |ui| {
        ui.add(egui::Slider::new(&mut f.stick_radius, 36.0..=90.0).suffix(" pt"));
    });
    row(ui, "左摇杆左右", |ui| {
        ui.radio_value(&mut f.stick_steers, false, "平移");
        ui.radio_value(&mut f.stick_steers, true, "转向");
    });
    hint(ui, "转向会带动角色和镜头，类似 A、D 键；平移为侧移，类似 Q、E 键。前后移动两种方式相同。");
    hint(ui, "长按跳跃呼出轮盘：滑到内圈选视图、外圈选窗口后松开。长按其他按钮可查看功能，松开不执行。");
    hint(ui, "在世界中：点击=选中，双击或长按=交互/攻击，拖动=转视角，双指=缩放。在窗口中：点击=左键，长按=右键，拖动=移动物品，双指=滚动。");
}

/// Every binding a controller button can take, for the picker.
fn choices() -> Vec<PadBinding> {
    let mut v = vec![
        PadBinding::None,
        PadBinding::Special(Special::Interact),
        PadBinding::Special(Special::RadialMenu),
        PadBinding::Special(Special::VirtualCursor),
        PadBinding::Special(Special::InillaMenu),
        PadBinding::Special(Special::ToggleTouchHud),
    ];
    v.extend(crate::inilla::synth::command_names().map(|(n, _)| PadBinding::cmd(n)));
    v
}

fn controller(ui: &mut egui::Ui, k: &mut Inilla, pad: &PadHud) {
    match &pad.name {
        Some(n) => ui.label(
            egui::RichText::new(format!("已连接：{n}")).color(egui::Color32::LIGHT_GREEN),
        ),
        None => ui.label(
            "未连接手柄。请在 iOS 设置 → 蓝牙中配对，或连接 Backbone。",
        ),
    };
    let p = &mut k.pad;
    row(ui, "摇杆死区", |ui| {
        ui.add(egui::Slider::new(&mut p.stick_deadzone, 0.05..=0.4).step_by(0.01));
    });
    row(ui, "扳机阈值", |ui| {
        ui.add(egui::Slider::new(&mut p.trigger_threshold, 0.1..=0.9).step_by(0.05));
    });
    row(ui, "转向速度", |ui| {
        ui.add(egui::Slider::new(&mut p.look_yaw_speed, 60.0..=480.0).suffix(" °/s"));
    });
    row(ui, "上下视角速度", |ui| {
        ui.add(egui::Slider::new(&mut p.look_pitch_speed, 30.0..=300.0).suffix(" °/s"));
    });
    row(ui, "响应曲线", |ui| {
        ui.add(egui::Slider::new(&mut p.look_curve, 1.0..=3.0).step_by(0.1));
    });
    ui.checkbox(&mut p.invert_y, "反转上下视角");
    ui.checkbox(
        &mut k.settings.controller_hotkeys,
        "在动作条上显示手柄按键",
    );
    ui.checkbox(
        &mut k.settings.controller_bar,
        "在屏幕底部显示手柄栏",
    );
    hint(
        ui,
        "Each action button shows what presses it (X, LT A, RT ↑) while the controller is in use, \
         and holding a trigger lights up the buttons it switches to. LT uses the bottom-left bar, \
         RT the bottom-right, LT+RT the right bar.",
    );
    let p = &mut k.pad;
    row(ui, "光标速度", |ui| {
        ui.add(egui::Slider::new(&mut p.cursor_speed, 300.0..=2000.0).suffix(" pt/s"));
    });
    ui.checkbox(
        &mut p.swap_confirm,
        "B 确认 / A 取消（任天堂布局）",
    );
    ui.checkbox(
        &mut p.auto_cursor,
        "拾取、商人、任务窗口自动进入光标模式",
    );
    ui.separator();
    ui.label(egui::RichText::new("按键映射").strong());
    hint(ui, "Hold LT, RT or both to switch the face buttons and d-pad to another layer — 32 bindings without leaving the sticks. By default LT is the bottom-left action bar, RT the bottom-right, LT+RT the right bar: drag spells onto those bars as on desktop.");
    let all = choices();
    let glyphs = pad.glyphs;
    egui::Grid::new("pad-map")
        .striped(true)
        .min_col_width(120.0)
        .show(ui, |ui| {
            ui.label("");
            for l in Layer::ALL {
                ui.label(egui::RichText::new(l.label()).strong().color(GOLD));
            }
            ui.end_row();
            let buttons = [
                PadButton::A,
                PadButton::B,
                PadButton::X,
                PadButton::Y,
                PadButton::Up,
                PadButton::Right,
                PadButton::Down,
                PadButton::Left,
                PadButton::LB,
                PadButton::RB,
                PadButton::L3,
                PadButton::R3,
                PadButton::Menu,
                PadButton::Options,
            ];
            for b in buttons {
                ui.label(egui::RichText::new(b.glyph(glyphs)).size(18.0));
                for l in Layer::ALL {
                    let current = p.binding(l, b).clone();
                    let mut pick = current.clone();
                    egui::ComboBox::from_id_salt((
                        "padmap",
                        l.label(),
                        b.glyph(Default::default()),
                    ))
                    .selected_text(current.label())
                    .width(140.0)
                    .height(360.0)
                    .show_ui(ui, |ui| {
                        for c in &all {
                            ui.selectable_value(&mut pick, c.clone(), c.label());
                        }
                    });
                    if pick != current {
                        p.set_binding(l, b, pick);
                    }
                }
                ui.end_row();
            }
        });
    if ui.button("重置手柄设置").clicked() {
        *p = PadProfile::default();
    }
}

fn audio(ui: &mut egui::Ui, k: &mut Inilla) {
    let s = &mut k.settings;
    ui.checkbox(
        &mut s.mix_with_other_audio,
        "允许其他 App 的音乐和播客继续播放",
    );
    ui.checkbox(
        &mut s.ignore_silent_switch,
        "静音开关打开时仍播放声音",
    );
    hint(
        ui,
        "音量、音乐和环境音在原版声音选项里（游戏菜单 → 声音与语音）。",
    );
}

fn debug(ui: &mut egui::Ui, overlay: &mut Overlay, k: &mut Inilla) {
    let d = &mut k.settings.debug;
    let was = d.remote_console;
    ui.checkbox(&mut d.remote_console, "局域网远程控制台");
    if d.remote_console && !was && d.console_token.is_empty() {
        d.console_token = crate::inilla::console::new_token();
    }
    if d.remote_console {
        let port = d.console_port;
        for ip in crate::inilla::console::local_addresses() {
            ui.label(
                egui::RichText::new(format!("http://{ip}:{port}/?token={}", d.console_token))
                    .monospace()
                    .color(GOLD),
            );
        }
        hint(ui, "Open that address in a browser on a computer on the same Wi-Fi: the live log, Lua, game commands, screenshots and the benilla-config files. Anyone with the address can drive the game — turn it off when you are done.");
        ui.horizontal(|ui| {
            if ui.button("复制地址").clicked() {
                if let Some(ip) = crate::inilla::console::local_addresses().first() {
                    inilla_platform::pasteboard::set(&format!(
                        "http://{ip}:{port}/?token={}",
                        d.console_token
                    ));
                    overlay.toast("控制台地址已复制");
                }
            }
            if ui.button("新令牌").clicked() {
                d.console_token = crate::inilla::console::new_token();
            }
        });
    }
    ui.checkbox(
        &mut d.log_to_file,
        "写入会话日志（文件 → iNilla → benilla-config → Diagnostics）",
    );
    ui.checkbox(&mut d.perf_overlay, "显示帧率、内存和温度");
    ui.checkbox(&mut d.show_touch_zones, "显示触控区域");
    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new("查看会话日志").min_size(egui::vec2(180.0, 44.0)))
            .clicked()
        {
            overlay.panel = Some(Panel::Logs);
        }
        if ui.button("游戏数据…").clicked() {
            overlay.panel = Some(Panel::Setup);
        }
    });
}

fn about(ui: &mut egui::Ui, k: &mut Inilla, data: &DataState, build: &BuildId) {
    ui.label(
        egui::RichText::new("iNilla — iPhone 上的 benilla")
            .strong()
            .color(GOLD),
    );
    ui.label(format!(
        "构建 {}（{}，{}）",
        build.short, build.date, build.profile
    ));
    ui.label(inilla_platform::device::describe());
    ui.label(format!(
        "游戏数据：{} 个档案，{:.1} GB{}",
        data.status.present.len(),
        data.status.total_bytes() as f64 / 1e9,
        data.dir
            .as_ref()
            .map(|d| format!(" 于 {}", d.display()))
            .unwrap_or_default()
    ));
    hint(ui, "benilla is a from-scratch World of Warcraft 1.12.1 client in Rust (github.com/samwhosung/benilla); iNilla is its iOS port. Neither ships Blizzard content, nor is affiliated with Blizzard Entertainment.");
    ui.add_space(12.0);
    if ui
        .button(egui::RichText::new("重置所有 iNilla 设置").color(egui::Color32::LIGHT_RED))
        .clicked()
    {
        k.settings = Default::default();
        k.settings.setup_done = true;
        k.layout = default_layout();
        k.pad = PadProfile::default();
    }
}
