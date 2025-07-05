use std::{collections::HashMap, ffi::CString, fmt::Display, path::Path};

use sdl2::{event::Event, sys::*};

use crate::{color::Color, platform::RenderEngine};

unsafe extern "C" {
    fn TTF_SetFontSize(font: *mut ttf::TTF_Font, ptsize: std::ffi::c_int);
}

#[derive(Debug)]
pub enum SDLError {
    SDLInitializationError(CString),
    SDLInstructionError(String),
    SDLFontInitializationError(CString),
    SDLFontLoadingError(CString),
    SDLFontNotRegistered(String),
    SDLFontRegistryCollision(String),
}

impl std::error::Error for SDLError {}

impl Display for SDLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SDLInitializationError(e) => {
                write!(f, "Failed to initialize SDL2: {}", e.to_string_lossy())
            }

            Self::SDLInstructionError(e) => write!(f, "Failed to execute SDL instruction: {e}"),

            Self::SDLFontLoadingError(e) => {
                write!(f, "Failed to load SDL ttf: {}", e.to_string_lossy())
            }

            Self::SDLFontInitializationError(e) => write!(
                f,
                "Failed to load SDL ttf subsytem: {}",
                e.to_string_lossy()
            ),

            Self::SDLFontNotRegistered(n) => write!(f, "Font has not been registered '{n}'"),

            Self::SDLFontRegistryCollision(n) => write!(f, "Font name already registered '{n}'"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum SDLRenderInstruction {
    SetDrawColor(Color),
    Clear,
    FillRect(sdl2::rect::Rect),
    DrawRect(sdl2::rect::Rect),
    DrawText {
        font: *mut SDL_Texture,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    },
}

pub struct SDLPlatform {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,

    instruction_queue: Vec<SDLRenderInstruction>,
    font_registry: HashMap<String, *mut sdl2::sys::ttf::_TTF_Font>,

    running: bool,
}

impl RenderEngine for SDLPlatform {
    fn new(title: impl Into<String>, width: usize, height: usize) -> anyhow::Result<Self> {
        unsafe {
            if SDL_Init(SDL_INIT_EVERYTHING) != 0 {
                let error = SDL_GetError();
                let error_ptr = std::mem::transmute::<*const i8, *mut i8>(error);

                anyhow::bail!(SDLError::SDLInitializationError(CString::from_raw(
                    error_ptr
                )));
            }

            if ttf::TTF_Init() == -1 {
                let reason = SDL_GetError();
                let reason_ptr = std::mem::transmute::<*const i8, *mut i8>(reason);
                anyhow::bail!(SDLError::SDLFontInitializationError(CString::from_raw(
                    reason_ptr
                )));
            }
        }

        let title = CString::new(title.into())?;

        let window = unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                width as i32,
                height as i32,
                SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
            )
        };

        let renderer = unsafe {
            SDL_CreateRenderer(
                window,
                -1,
                SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            )
        };

        unsafe {
            if sdl2::sys::ttf::TTF_Init() == -1 {
                let reason = sdl2::sys::SDL_GetError();
                let reason_ptr = std::mem::transmute::<*const i8, *mut i8>(reason);
                anyhow::bail!(SDLError::SDLFontInitializationError(CString::from_raw(
                    reason_ptr
                )))
            }
        }

        Ok(Self {
            window,
            renderer,

            instruction_queue: Vec::new(),
            font_registry: HashMap::new(),

            running: true,
        })
    }

    fn pre_render(&mut self) -> anyhow::Result<()> {
        unsafe {
            let mut event: SDL_Event = std::mem::zeroed();

            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    t if t == SDL_EventType::SDL_QUIT as u32 => {
                        self.running = false;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn clear_screen(&mut self, color: Color) -> anyhow::Result<()> {
        self.instruction_queue
            .push(SDLRenderInstruction::SetDrawColor(color));
        self.instruction_queue.push(SDLRenderInstruction::Clear);

        Ok(())
    }

    fn render_rect(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: Color,
    ) -> anyhow::Result<()> {
        self.instruction_queue
            .push(SDLRenderInstruction::SetDrawColor(color));
        self.instruction_queue
            .push(SDLRenderInstruction::FillRect(sdl2::rect::Rect::new(
                x as i32,
                y as i32,
                width as u32,
                height as u32,
            )));

        Ok(())
    }

    fn render_rect_outlined(
        &mut self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        color: Color,
    ) -> anyhow::Result<()> {
        self.instruction_queue
            .push(SDLRenderInstruction::SetDrawColor(color));
        self.instruction_queue
            .push(SDLRenderInstruction::DrawRect(sdl2::rect::Rect::new(
                x as i32,
                y as i32,
                width as u32,
                height as u32,
            )));

        Ok(())
    }

    fn render_text(
        &mut self,
        text: String,
        x: isize,
        y: isize,
        font_size: usize,
        color: Color,
        font_name: String,
    ) -> anyhow::Result<()> {
        let font_name = *self
            .font_registry
            .get(&font_name)
            .ok_or(SDLError::SDLFontNotRegistered(font_name))?;

        unsafe {
            TTF_SetFontSize(font_name, font_size as i32);
            let s = sdl2::sys::ttf::TTF_RenderText_Solid(
                font_name,
                CString::new(text)?.as_ptr(),
                color.into_sdl_sys(),
            );

            let w = (*s).w;
            let h = (*s).h;

            let t = sdl2::sys::SDL_CreateTextureFromSurface(self.renderer, s);
            SDL_FreeSurface(s);

            let x = x as i32;
            let y = y as i32;
            self.instruction_queue.push(SDLRenderInstruction::DrawText {
                font: t,
                x,
                y,
                w,
                h,
            });
        }
        Ok(())
    }

    fn post_render(&mut self) -> anyhow::Result<()> {
        for inst in &self.instruction_queue {
            match inst {
                SDLRenderInstruction::SetDrawColor(c) => unsafe {
                    SDL_SetRenderDrawColor(self.renderer, c.r, c.g, c.b, 255);
                },
                SDLRenderInstruction::Clear => unsafe {
                    SDL_RenderClear(self.renderer);
                },
                SDLRenderInstruction::FillRect(r) => unsafe {
                    SDL_RenderFillRect(self.renderer, r.raw());
                },
                SDLRenderInstruction::DrawRect(r) => unsafe {
                    SDL_RenderDrawRect(self.renderer, r.raw());
                },
                SDLRenderInstruction::DrawText { font, x, y, w, h } => unsafe {
                    let x = *x;
                    let y = *y;
                    let w = *w;
                    let h = *h;
                    SDL_RenderCopy(
                        self.renderer,
                        font.clone(),
                        std::ptr::null(),
                        &SDL_Rect { x, y, w, h },
                    );
                    SDL_DestroyTexture(font.clone());
                },
            }
        }

        self.instruction_queue.clear();

        unsafe {
            SDL_RenderPresent(self.renderer);
        }

        Ok(())
    }

    fn register_font(&mut self, name: String, path: &Path) -> anyhow::Result<()> {
        if self.font_registry.contains_key(&name) {
            anyhow::bail!(SDLError::SDLFontRegistryCollision(name));
        }

        let path_str = CString::new(path.to_string_lossy().to_string())?;

        let font = unsafe { sdl2::sys::ttf::TTF_OpenFont(path_str.as_ptr(), 11) };

        if font.is_null() {
            let error_ptr = unsafe {
                std::mem::transmute::<*const i8, *mut i8>(sdl2::sys::SDL_GetError()) as *mut i8
            };
            let error = unsafe { CString::from_raw(error_ptr) };

            anyhow::bail!(SDLError::SDLFontLoadingError(CString::new(error)?));
        }

        self.font_registry.insert(name, font);

        Ok(())
    }
}
