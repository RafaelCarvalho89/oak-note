use adw::prelude::*;

const APP_ID: &str = "io.github.rafaelcarvalho89.OakNotePoc";

fn background_alpha(transparency: f64) -> f64 {
    1.0 - transparency.clamp(0.0, 70.0) / 100.0
}

fn update_background(provider: &gtk::CssProvider, transparency: f64, dark: bool) {
    let alpha = background_alpha(transparency);
    let color = if dark { "36, 36, 36" } else { "250, 250, 250" };
    provider.load_from_data(&format!(
        "window.oak-poc {{ background: transparent; }}
         .oak-background {{ background-color: rgba({color}, {alpha:.3}); }}
         .oak-editor, .oak-editor text, .oak-scroll {{ background: transparent; }}"
    ));
}

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_window);
    app.run();
}

fn build_window(app: &adw::Application) {
    let text = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::WordChar)
        .top_margin(16)
        .bottom_margin(16)
        .left_margin(16)
        .right_margin(16)
        .build();
    text.add_css_class("oak-editor");
    text.buffer().set_text(
        "# Oak Note POC\n\nTexto de demonstração editável para avaliar a legibilidade em fundos claros e escuros, mesmo quando uma janela colorida aparece atrás do editor. Este parágrafo longo ajuda a observar a quebra de linha durante o redimensionamento.\n\n## Lista de verificação\n\n- Fundo claro e escuro\n- Texto, seleção e controles\n- Transparência ajustável\n- Navegação por teclado\n\nCaracteres: á é í ó ú ç ñ € © — !? [] {} # * _\n\n## Mais conteúdo\n\nSelecione este texto e observe o cursor e o contraste.\n\nLinha 1: a edição continua disponível.\nLinha 2: teste a rolagem vertical.\nLinha 3: alterne entre os temas.\nLinha 4: compare diferentes níveis de transparência.\nLinha 5: redimensione a janela.\nLinha 6: observe possíveis cortes.\nLinha 7: teste a navegação por teclado.\nLinha 8: fim do conteúdo de demonstração.",
    );

    let scroll = gtk::ScrolledWindow::builder()
        .child(&text)
        .vexpand(true)
        .build();
    scroll.add_css_class("oak-scroll");

    let slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 70.0, 1.0);
    slider.set_hexpand(true);
    slider.set_draw_value(false);
    let label = gtk::Label::new(Some("Transparência do fundo"));
    label.set_mnemonic_widget(Some(&slider));
    let value = gtk::Label::new(Some("0%"));
    let transparency_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    transparency_row.append(&label);
    transparency_row.append(&slider);
    transparency_row.append(&value);

    let theme = gtk::DropDown::from_strings(&["Sistema", "Claro", "Escuro"]);
    let theme_label = gtk::Label::new(Some("Tema"));
    theme_label.set_mnemonic_widget(Some(&theme));
    let theme_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    theme_row.append(&theme_label);
    theme_row.append(&theme);

    let controls = gtk::Box::new(gtk::Orientation::Vertical, 8);
    controls.set_margin_top(12);
    controls.set_margin_bottom(12);
    controls.set_margin_start(16);
    controls.set_margin_end(16);
    controls.append(&theme_row);
    controls.append(&transparency_row);

    let background = gtk::Box::new(gtk::Orientation::Vertical, 0);
    background.add_css_class("oak-background");
    background.set_vexpand(true);
    background.append(&scroll);
    background.append(&controls);

    let header = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Oak Note POC", "Prova de conceito"))
        .build();
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&header);
    content.append(&background);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Oak Note POC")
        .default_width(800)
        .default_height(560)
        .content(&content)
        .build();
    window.add_css_class("oak-poc");

    let provider = gtk::CssProvider::new();
    let display = gtk::prelude::WidgetExt::display(&window);
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let style = adw::StyleManager::default();
    style.set_color_scheme(adw::ColorScheme::PreferLight);
    theme.connect_selected_notify({
        let style = style.clone();
        move |theme| {
            let scheme = match theme.selected() {
                1 => adw::ColorScheme::ForceLight,
                2 => adw::ColorScheme::ForceDark,
                _ => adw::ColorScheme::PreferLight,
            };
            style.set_color_scheme(scheme);
        }
    });
    update_background(&provider, slider.value(), style.is_dark());
    slider.connect_value_changed({
        let provider = provider.clone();
        let style = style.clone();
        move |slider| {
            value.set_text(&format!("{:.0}%", slider.value()));
            update_background(&provider, slider.value(), style.is_dark());
        }
    });
    style.connect_dark_notify({
        let slider = slider.downgrade();
        move |style| {
            if let Some(slider) = slider.upgrade() {
                update_background(&provider, slider.value(), style.is_dark());
            }
        }
    });
    window.present();
}

#[cfg(test)]
mod tests {
    use super::background_alpha;

    #[test]
    fn converts_percentages_to_alpha() {
        for (percent, expected) in [(0.0, 1.0), (25.0, 0.75), (50.0, 0.5), (70.0, 0.3)] {
            assert!((background_alpha(percent) - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn clamps_percentages_to_supported_range() {
        assert_eq!(background_alpha(-10.0), 1.0);
        assert!((background_alpha(100.0) - 0.3).abs() < 1e-10);
    }
}
