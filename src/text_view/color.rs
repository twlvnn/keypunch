use super::*;

#[derive(Default, Clone, Copy)]
pub(super) struct TextViewColorScheme {
    pub untyped: (u16, u16, u16),
    pub typed: (u16, u16, u16),
    pub mistake: (u16, u16, u16),
    pub caret: (f32, f32, f32),
}

const COLOR_SCHEME_LIGHT: TextViewColorScheme = TextViewColorScheme {
    untyped: (41472, 41472, 41472),
    typed: (12800, 12800, 12800),
    mistake: (49152, 7168, 10240),
    caret: (0.2, 0.2, 0.2),
};

const COLOR_SCHEME_DARK: TextViewColorScheme = TextViewColorScheme {
    untyped: (33792, 33792, 33792),
    typed: (65280, 65280, 65280),
    mistake: (65280, 31488, 25344),
    caret: (1., 1., 1.),
};

impl imp::RcwTextView {
    pub(super) fn setup_color_scheme(&self) {
        let obj = self.obj();
        let style = adw::StyleManager::default();
        style.connect_dark_notify(glib::clone!(@weak obj => move |_| {
            obj.imp().update_color_scheme();
        }));

        self.update_color_scheme();
    }

    pub(super) fn update_color_scheme(&self) {
        let style = adw::StyleManager::default();

        self.color_scheme.set(if style.is_dark() {
            COLOR_SCHEME_DARK
        } else {
            COLOR_SCHEME_LIGHT
        });

        self.update_visuals();
    }

    pub(super) fn update_visuals(&self) {
        let clr = self.color_scheme.get();

        let comparison = self.typing_session.borrow().validate_with_whsp_markers();

        let attr_list = pango::AttrList::new();

        let untyped_attr =
            pango::AttrColor::new_foreground(clr.untyped.0, clr.untyped.1, clr.untyped.2);
        attr_list.insert(untyped_attr);

        let mut typed_attr =
            pango::AttrColor::new_foreground(clr.typed.0, clr.typed.1, clr.typed.2);
        typed_attr.set_start_index(0);
        typed_attr.set_end_index(comparison.len() as u32);
        attr_list.insert(typed_attr);

        comparison
            .iter()
            .enumerate()
            .filter(|(_, &correctly_typed)| !correctly_typed)
            .map(|(n, _)| {
                let mut mistake_fg_attr =
                    pango::AttrColor::new_foreground(clr.mistake.0, clr.mistake.1, clr.mistake.2);
                mistake_fg_attr.set_start_index(n as u32);
                mistake_fg_attr.set_end_index(n as u32 + 1);

                mistake_fg_attr
            })
            .for_each(|attr| attr_list.insert(attr));

        self.label.set_attributes(Some(&attr_list));
    }
}