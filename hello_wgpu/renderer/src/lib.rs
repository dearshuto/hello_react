use std::{
    future::Future,
    sync::{Arc, Mutex},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

mod detail {
    pub fn log(str: &str) {
        #[cfg(target_arch = "wasm32")]
        super::log(str);

        println!("{str}");
    }
}

// 非同期ランタイムを抽象化するための trait
// 関数が 'static なのは web 向けの制約
pub trait IRuntime {
    fn execute<F>(&self, func: F)
    where
        F: Future<Output = ()> + 'static;
}

#[cfg(target_arch = "wasm32")]
struct WasmBindgenFuturesRuntime;

#[cfg(target_arch = "wasm32")]
impl IRuntime for WasmBindgenFuturesRuntime {
    fn execute<F>(&self, func: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(func);
    }
}

struct Instance {
    #[allow(unused)]
    instance: wgpu::Instance,
    #[allow(unused)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
}

trait ICallback {
    fn customize(
        &mut self,
        attributes: winit::window::WindowAttributes,
    ) -> winit::window::WindowAttributes;
}

struct EmptyCallback;
impl ICallback for EmptyCallback {
    fn customize(
        &mut self,
        attributes: winit::window::WindowAttributes,
    ) -> winit::window::WindowAttributes {
        attributes
    }
}

struct Application<TRuntime, TCallback>
where
    TRuntime: IRuntime,
    TCallback: ICallback,
{
    runtime: TRuntime,
    instance: Arc<Mutex<Option<Instance>>>,
    callback: TCallback,
}

impl<TRuntime> Application<TRuntime, EmptyCallback>
where
    TRuntime: IRuntime,
{
    pub fn new(runtime: TRuntime) -> Self {
        Self::new_with_window_attr_callback(runtime, EmptyCallback {})
    }
}

impl<TRuntime, TCallback> Application<TRuntime, TCallback>
where
    TRuntime: IRuntime,
    TCallback: ICallback,
{
    pub fn new_with_window_attr_callback(runtime: TRuntime, callback: TCallback) -> Self {
        Self {
            runtime,
            instance: Default::default(),
            callback,
        }
    }
}

impl<TRuntime, TCallback> ApplicationHandler for Application<TRuntime, TCallback>
where
    TRuntime: IRuntime,
    TCallback: ICallback,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = {
            let attributes = winit::window::WindowAttributes::default()
                .with_resizable(false)
                .with_inner_size(winit::dpi::Size::new(winit::dpi::PhysicalSize::new(
                    640, 480,
                )));
            self.callback.customize(attributes)
        };

        let window = event_loop.create_window(window_attributes).unwrap();

        // adapter のリクエストは並列処理しないとデッドロックするので注意
        let binding = self.instance.clone();
        self.runtime.execute(async move {
            let instance = wgpu::Instance::default();
            let surface = instance.create_surface(window).unwrap();
            let Some(adapter) = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
            else {
                detail::log("request_adapter error");
                return;
            };

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default(), None)
                .await
                .unwrap();

            let config = surface.get_default_config(&adapter, 640, 480).unwrap();
            surface.configure(&device, &config);

            *binding.lock().unwrap() = Some(Instance {
                instance,
                adapter,
                device,
                queue,
                surface,
            });
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // 実際はここで再描画を間引く必要がある
        // 理想は EventLoop を Poll 方式にして、低レベル Graphics API の VSync 指定する
        let binding = self.instance.lock().unwrap();
        if binding.is_none() {
            return;
        }
        let instance = binding.as_ref().unwrap();
        let adapter = &instance.adapter;
        let device = &instance.device;
        let queue = &instance.queue;
        let surface = &instance.surface;

        match event {
            WindowEvent::Resized(size) => {
                let config = surface
                    .get_default_config(adapter, size.width, size.height)
                    .unwrap();
                surface.configure(device, &config);
            }
            WindowEvent::Occluded(is_occluded) => {
                if is_occluded {
                    event_loop.exit();
                }
            }
            WindowEvent::Focused(_) => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {
                detail::log(&format!("{:?}", event));
            }
        };

        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let mut command_encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                ..Default::default()
            });
            let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        let command_buffer = command_encoder.finish();
        queue.submit([command_buffer]);
        frame.present();
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
        detail::log("suspended()");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
        *self.instance.lock().unwrap() = None;
        detail::log("existing");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn start() {
    detail::log("start()");
}

pub fn run<TRuntime: IRuntime>(runtime: TRuntime) {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    let mut application = Application::new(runtime);
    event_loop.run_app(&mut application).unwrap();
}

#[cfg(target_arch = "wasm32")]
struct WasmCallback {
    canvas: Option<HtmlCanvasElement>,
}
#[cfg(target_arch = "wasm32")]
impl ICallback for WasmCallback {
    fn customize(
        &mut self,
        attributes: winit::window::WindowAttributes,
    ) -> winit::window::WindowAttributes {
        use winit::platform::web::WindowAttributesExtWebSys;

        let mut canvas = None;
        std::mem::swap(&mut canvas, &mut self.canvas);
        attributes.with_canvas(canvas)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct Sender {
    proxy: winit::event_loop::EventLoopProxy<()>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_on_canvas(canvas: HtmlCanvasElement) -> Sender {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let callback = WasmCallback {
        canvas: Some(canvas),
    };

    let runtime = WasmBindgenFuturesRuntime {};
    let application = Application::new_with_window_attr_callback(runtime, callback);

    let proxy = event_loop.create_proxy();

    use winit::platform::web::EventLoopExtWebSys;
    event_loop.spawn_app(application);

    Sender { proxy }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn emit_user_event(sender: Sender) {
    sender.proxy.send_event(()).unwrap();
}
