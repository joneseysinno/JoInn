//! `Lost` and `Outdated` reconfigure the surface. A zero size does not.

use super::ShellApp;
use super::emit::emit;

impl ShellApp {
    /// Configure the swapchain and hand the new client size to the shell: the
    /// first size frames the scene, later sizes keep zoom, focus and pin.
    pub(super) fn reconfigure(&mut self) -> bool {
        let size = {
            let Some(window) = self.window.as_ref() else {
                return false;
            };
            window.inner_size()
        };
        if size.width == 0 || size.height == 0 {
            self.configured = false;
            return false;
        }
        {
            let Some(surface) = self.surface.as_ref() else {
                return false;
            };
            let Some(gpu) = self.gpu.as_ref() else {
                return false;
            };
            let Some(mut config) =
                surface.get_default_config(self.adapter.adapter(), size.width, size.height)
            else {
                eprintln!(
                    "joinn-desktop: the surface has no configuration for {}x{}; acceptance is a drawable size",
                    size.width, size.height
                );
                return false;
            };
            config.format = self.format;
            surface.configure(gpu.device(), &config);
        }
        let lines = self.shell.resize(size.width, size.height);
        for line in lines {
            emit(&line);
        }
        self.configured = true;
        true
    }
}
