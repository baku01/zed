use std::marker::PhantomData;

use gpui2::{rems, AbsoluteLength};

use crate::prelude::*;
use crate::{Icon, IconButton, Label, Panel, PanelSide};

#[derive(Component)]
pub struct AssistantPanel<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
    current_side: PanelSide,
}

impl<S: 'static + Send + Sync> AssistantPanel<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
            current_side: PanelSide::default(),
        }
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        self.current_side = side;
        self
    }

    fn render(self, view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        Panel::new(self.id.clone(), cx)
            .children(vec![div()
                .flex()
                .flex_col()
                .h_full()
                .px_2()
                .gap_2()
                // Header
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .child(IconButton::new("menu", Icon::Menu))
                                .child(Label::new("New Conversation")),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_px()
                                .child(IconButton::new("split_message", Icon::SplitMessage))
                                .child(IconButton::new("quote", Icon::Quote))
                                .child(IconButton::new("magic_wand", Icon::MagicWand))
                                .child(IconButton::new("plus", Icon::Plus))
                                .child(IconButton::new("maximize", Icon::Maximize)),
                        ),
                )
                // Chat Body
                .child(
                    div()
                        .id("chat-body")
                        .w_full()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .overflow_y_scroll()
                        .child(Label::new("Is this thing on?")),
                )
                .render()])
            .side(self.current_side)
            .width(AbsoluteLength::Rems(rems(32.)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Component)]
    pub struct AssistantPanelStory {}

    impl AssistantPanelStory {
        pub fn new() -> Self {
            Self {}
        }

        fn render<V: 'static + Send + Sync>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            Story::container(cx)
                .child(Story::title_for::<_, AssistantPanel<V>>(cx))
                .child(Story::label(cx, "Default"))
                .child(AssistantPanel::new("assistant-panel"))
        }
    }
}
