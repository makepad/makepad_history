use render::*;

pub fn set_dark_style(cx:&mut Cx){
    cx.set_style_font("normal_font", "resources/ubuntu_regular_256.font");
    cx.set_style_font("icon_font", "resources/fontawesome.font");
    cx.set_style_size("font_size", 10.0);

    cx.set_style_color("text_select",color("Purple900"));
    
    cx.set_style_color("accent_normal", color("Purple900"));
    cx.set_style_color("accent_down", color("Purple500"));
    cx.set_style_color("accent_gray", color("Grey700"));

    cx.set_style_color("bg_top", color("Grey900"));
    cx.set_style_color("bg_normal", color("Grey850"));
    cx.set_style_color("bg_hi", color("Grey800"));

    cx.set_style_color("text_normal", color("Grey300"));
    cx.set_style_color("text_accent", color("Grey400"));
    cx.set_style_color("text_med", color("Grey500"));
    cx.set_style_color("text_hi", color("Grey300"));
    cx.set_style_color("text_lo", color("Grey700"));

    cx.set_style_color("code_bg", color("Grey900"));
    cx.set_style_color("code_class", color("Pink300"));
    cx.set_style_color("code_object", color("Indigo200"));
    cx.set_style_color("code_paren", color("BlueGrey400"));
    cx.set_style_color("code_array", color("Cyan300"));
    cx.set_style_color("code_function", color("Amber300"));
    cx.set_style_color("code_call", color("Yellow300"));
    cx.set_style_color("code_if", color("LightGreen300"));
    cx.set_style_color("code_loop", color("DeepOrange300"));
    cx.set_style_color("code_comment", color("Blue700"));
    cx.set_style_color("code_exception", color("Red400"));
    cx.set_style_color("code_var", color("BlueGrey200"));
    cx.set_style_color("code_let", color("BlueGrey100"));
    cx.set_style_color("code_const", color("BlueGrey400"));
    cx.set_style_color("code_global", color("YellowA100"));
    cx.set_style_color("code_arg", color("BlueGrey500"));
    cx.set_style_color("code_unknown", color("White"));
    cx.set_style_color("code_operator", color("Amber300"));
    cx.set_style_color("code_number", color("IndigoA100"));
    cx.set_style_color("code_boolean", color("Red400"));
    cx.set_style_color("code_string", color("GreenA200"));
    cx.set_style_color("code_tok_exception", color("red"));
    cx.set_style_color("code_log", color("yellow"));
}