#![feature(libc, hash, path, std_misc)]

#![crate_name="sdl2_image"]
#![crate_type = "lib"]


extern crate sdl2;
extern crate "sdl2-sys" as sdl2_sys;
extern crate libc;
#[macro_use]
extern crate bitflags;

use libc::{c_int, c_char};
use std::ptr;
use std::ffi::CString;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::rwops::RWops;
use sdl2::version::Version;
use sdl2::get_error;
use sdl2::SdlResult;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_image")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_image")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2_image")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;

/// InitFlags are passed to init() to control which subsystem
/// functionality to load.
bitflags! {
    flags InitFlag : u32 {
        const INIT_JPG  = ffi::IMG_INIT_JPG as u32,
        const INIT_PNG  = ffi::IMG_INIT_PNG as u32,
        const INIT_TIF  = ffi::IMG_INIT_TIF as u32,
        const INIT_WEBP = ffi::IMG_INIT_WEBP as u32
    }
}

/// Static method extensions for creating Surfaces
pub trait LoadSurface {
    // Self is only returned here to type hint to the compiler.
    // The syntax for type hinting in this case is not yet defined.
    // The intended return value is SdlResult<~Surface>.
    fn from_file(filename: &Path) -> SdlResult<Self>;
    fn from_xpm_array(xpm: *const *const i8) -> SdlResult<Self>;
}

/// Method extensions to Surface for saving to disk
pub trait SaveSurface {
    fn save(&self, filename: &Path) -> SdlResult<()>;
    fn save_rw(&self, dst: &mut RWops) -> SdlResult<()>;
}

impl LoadSurface for Surface {
    fn from_file(filename: &Path) -> SdlResult<Surface> {
        //! Loads an SDL Surface from a file
        unsafe {
            let raw = ffi::IMG_Load(CString::from_slice(filename.as_vec()).as_ptr());
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }

    fn from_xpm_array(xpm: *const *const i8) -> SdlResult<Surface> {
        //! Loads an SDL Surface from XPM data
        unsafe {
            let raw = ffi::IMG_ReadXPMFromArray(xpm as *const *const c_char);
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw, true))
            }
        }
    }
}

impl SaveSurface for Surface {
    fn save(&self, filename: &Path) -> SdlResult<()> {
        //! Saves an SDL Surface to a file
        unsafe {
            let status = ffi::IMG_SavePNG(self.raw(), CString::from_slice(filename.as_vec()).as_ptr());
            if status != 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    fn save_rw(&self, dst: &mut RWops) -> SdlResult<()> {
        //! Saves an SDL Surface to an RWops
        unsafe {
            let status = ffi::IMG_SavePNG_RW(self.raw(), dst.raw(), 0);

            if status != 0 {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }
}

/// Method extensions for creating Textures from a Renderer
pub trait LoadTexture {
    fn load_texture(&self, filename: &Path) -> SdlResult<Texture>;
}

impl LoadTexture for Renderer {
    fn load_texture(&self, filename: &Path) -> SdlResult<Texture> {
        //! Loads an SDL Texture from a file
        unsafe {
            let raw = ffi::IMG_LoadTexture(self.raw(), CString::from_slice(filename.as_vec()).as_ptr());
            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Texture::from_ll(raw))
            }
        }
    }
}

pub fn init(flags: InitFlag) -> InitFlag {
    //! Initializes SDL2_image with InitFlags and returns which
    //! InitFlags were actually used.
    unsafe {
        let used = ffi::IMG_Init(flags.bits() as c_int);
        InitFlag::from_bits_truncate(used as u32)
    }
}

pub fn quit() {
    //! Teardown the SDL2_Image subsystem
    unsafe { ffi::IMG_Quit(); }
}

pub fn get_linked_version() -> Version {
    //! Returns the version of the dynamically linked SDL_image library
    unsafe {
        Version::from_ll(ffi::IMG_Linked_Version())
    }
}

#[inline]
fn to_surface_result(raw: *const sdl2_sys::surface::SDL_Surface) -> SdlResult<Surface> {
    if raw == ptr::null() {
        Err(get_error())
    } else {
        unsafe { Ok(Surface::from_ll(raw, true)) }
    }
}

pub trait ImageRWops {
    /// load as a surface. except TGA
    fn load(&self) -> SdlResult<Surface>;
    /// load as a surface. This can load all supported image formats.
    fn load_typed(&self, _type: &str) -> SdlResult<Surface>;

    fn load_cur(&self) -> SdlResult<Surface>;
    fn load_ico(&self) -> SdlResult<Surface>;
    fn load_bmp(&self) -> SdlResult<Surface>;
    fn load_pnm(&self) -> SdlResult<Surface>;
    fn load_xpm(&self) -> SdlResult<Surface>;
    fn load_xcf(&self) -> SdlResult<Surface>;
    fn load_pcx(&self) -> SdlResult<Surface>;
    fn load_gif(&self) -> SdlResult<Surface>;
    fn load_jpg(&self) -> SdlResult<Surface>;
    fn load_tif(&self) -> SdlResult<Surface>;
    fn load_png(&self) -> SdlResult<Surface>;
    fn load_tga(&self) -> SdlResult<Surface>;
    fn load_lbm(&self) -> SdlResult<Surface>;
    fn load_xv(&self)  -> SdlResult<Surface>;
    fn load_webp(&self) -> SdlResult<Surface>;

    fn is_cur(&self) -> bool;
    fn is_ico(&self) -> bool;
    fn is_bmp(&self) -> bool;
    fn is_pnm(&self) -> bool;
    fn is_xpm(&self) -> bool;
    fn is_xcf(&self) -> bool;
    fn is_pcx(&self) -> bool;
    fn is_gif(&self) -> bool;
    fn is_jpg(&self) -> bool;
    fn is_tif(&self) -> bool;
    fn is_png(&self) -> bool;
    fn is_lbm(&self) -> bool;
    fn is_xv(&self)  -> bool;
    fn is_webp(&self) -> bool;
}

impl ImageRWops for RWops {
    fn load(&self) -> SdlResult<Surface> {
        let raw = unsafe {
            ffi::IMG_Load_RW(self.raw(), 0)
        };
        to_surface_result(raw)
    }
    fn load_typed(&self, _type: &str) -> SdlResult<Surface> {
        let raw = unsafe {
            ffi::IMG_LoadTyped_RW(self.raw(), 0, CString::from_slice(_type.as_bytes()).as_ptr())
        };
        to_surface_result(raw)
    }

    fn load_cur(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadCUR_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_ico(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadICO_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_bmp(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadBMP_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_pnm(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadPNM_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_xpm(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadXPM_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_xcf(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadXCF_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_pcx(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadPCX_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_gif(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadGIF_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_jpg(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadJPG_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_tif(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadTIF_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_png(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadPNG_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_tga(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadTGA_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_lbm(&self) -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadLBM_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_xv(&self)  -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadXV_RW(self.raw()) };
        to_surface_result(raw)
    }
    fn load_webp(&self)  -> SdlResult<Surface> {
        let raw = unsafe { ffi::IMG_LoadWEBP_RW(self.raw()) };
        to_surface_result(raw)
    }

    fn is_cur(&self) -> bool {
        unsafe { ffi::IMG_isCUR(self.raw()) == 1 }
    }
    fn is_ico(&self) -> bool {
        unsafe { ffi::IMG_isICO(self.raw()) == 1 }
    }
    fn is_bmp(&self) -> bool {
        unsafe { ffi::IMG_isBMP(self.raw()) == 1 }
    }
    fn is_pnm(&self) -> bool {
        unsafe { ffi::IMG_isPNM(self.raw()) == 1 }
    }
    fn is_xpm(&self) -> bool {
        unsafe { ffi::IMG_isXPM(self.raw()) == 1 }
    }
    fn is_xcf(&self) -> bool {
        unsafe { ffi::IMG_isXCF(self.raw()) == 1 }
    }
    fn is_pcx(&self) -> bool {
        unsafe { ffi::IMG_isPCX(self.raw()) == 1 }
    }
    fn is_gif(&self) -> bool {
        unsafe { ffi::IMG_isGIF(self.raw()) == 1 }
    }
    fn is_jpg(&self) -> bool {
        unsafe { ffi::IMG_isJPG(self.raw()) == 1 }
    }
    fn is_tif(&self) -> bool {
        unsafe { ffi::IMG_isTIF(self.raw()) == 1 }
    }
    fn is_png(&self) -> bool {
        unsafe { ffi::IMG_isPNG(self.raw()) == 1 }
    }
    fn is_lbm(&self) -> bool {
        unsafe { ffi::IMG_isLBM(self.raw()) == 1 }
    }
    fn is_xv(&self)  -> bool {
        unsafe { ffi::IMG_isXV(self.raw())  == 1 }
    }
    fn is_webp(&self) -> bool {
        unsafe { ffi::IMG_isWEBP(self.raw())  == 1 }
    }
}
