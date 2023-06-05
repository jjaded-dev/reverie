use std::{sync::Arc, path::PathBuf};

use reverie::engine::registry::AssetType;
use specs::World;

use crate::app::save_scene;

pub struct Viewport {
    pub texture: Arc<wgpu::Texture>,
    pub size: [u32; 2],
}

impl Viewport {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = Arc::new(device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width: 800,
                height: 600,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        }));

        Self {
            size: [texture.width(), texture.height()],
            texture,
        }
    }

    pub fn ui<'a>(&mut self, ui: &'a imgui::Ui, world: &mut World, scene: &PathBuf) {
        let padding = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
        ui.window("Viewport").menu_bar(true).build(|| {
            let bar = ui.begin_menu_bar();
            if ui.menu_item("Save") {
                save_scene(world, scene);
            }
            bar.unwrap().end();
            self.size = [ui.content_region_avail()[0] as u32, ui.content_region_avail()[1] as u32];
            imgui::Image::new(imgui::TextureId::new(2), ui.content_region_avail()).build(ui);
            match ui.drag_drop_target() {
                Some(target) => {
                    match target.accept_payload::<Option<usize>, _>(AssetType::Scene.to_string(), imgui::DragDropFlags::empty()) {
                        Some(Ok(payload_data)) => {
                            //self.id = payload_data.data;
                            println!("hi");
                        },
                        Some(Err(e)) => {
                            println!("{}", e);
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        });
        padding.pop();
    }
}