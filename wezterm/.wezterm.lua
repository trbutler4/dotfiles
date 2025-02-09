local wezterm = require("wezterm")
local config = wezterm.config_builder()

config.color_scheme = "Everforest Dark (Gogh)"
config.hide_tab_bar_if_only_one_tab = true
config.window_decorations = "NONE"

-- yazelix config 
-- config.default_prog = { 'nu', '-c', "zellij -l welcome --config-dir ~/.config/yazelix/zellij options --layout-dir ~/.config/yazelix/zellij/layouts" }



return config
