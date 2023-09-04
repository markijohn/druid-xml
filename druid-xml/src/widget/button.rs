//Button as DXLabel

use druid::widget::prelude::*;
use druid::widget::{Click, ControllerHost, LabelText};
use druid::{Data, Insets};

use crate::qwidget::value::JSValue;
use crate::qwidget::wrapper::ParentableWidget;

use super::label::DXLabel;


// the minimum padding added to a button.
// NOTE: these values are chosen to match the existing look of TextBox; these
// should be reevaluated at some point.
const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

/// A button with a text label.
pub struct DXButton {
    label: DXLabel,
}

impl DXButton {
    /// Create a new button with a text label.
    ///
    /// Use the [`on_click`] method to provide a closure to be called when the
    /// button is clicked.
    ///
    /// # Examples
    ///
    /// ```
    /// use druid_xml::widget::DXButton;
    ///
    /// let button = DXButton::new("Increment").on_click(|_ctx, data: &mut u32, _env| {
    ///     *data += 1;
    /// });
    /// ```
    ///
    /// [`on_click`]: #method.on_click
    pub fn new(text: impl Into<LabelText<String>>) -> Self {
        Self::from_label(DXLabel::new(text))
    }

    pub fn from_label(label: DXLabel) -> DXButton {
        Self { label }
    }

    pub fn dynamic(text: impl Fn(&String, &Env) -> String + 'static) -> Self {
        let text: LabelText<String> = text.into();
        Self::new(text)
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut String, &Env) + 'static,
    ) -> ControllerHost<Self, Click<String>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl Widget<JSValue> for DXButton {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut JSValue, _env: &Env) { }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &JSValue, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &JSValue, data: &JSValue, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &JSValue, env: &Env) -> Size {
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &JSValue, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}

impl ParentableWidget for DXButton {
    fn get_childs(&self) -> Option<&[crate::qwidget::qwidget::QWidget]> {
        None
    }

    fn is_parentable(&self) -> bool {
        false
    }
}