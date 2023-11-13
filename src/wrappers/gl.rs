use std::marker::PhantomData;

use glfw::Window;

pub struct GL {
    _guard: PhantomData<()>,
}

impl GL {
    pub fn init(window: &mut Window) -> Self {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        Self {
            _guard: PhantomData,
        }
    }
}
