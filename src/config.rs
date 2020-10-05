use sdl2::pixels::Color;


struct UserConfig {
    foreground_color: Color,
}


const default_config: UserConfig = UserConfig {
   foreground_color = RGB::Color(100, 100, 100),
};
