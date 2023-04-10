use std::borrow::BorrowMut;

use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};

use super::{event::Event, language::XvaLanguage};

pub(super) struct EventSink<'parse> {
    builder: GreenNodeBuilder<'parse>,
    events: Vec<Event<'parse>>,
}

impl<'parse> EventSink<'parse> {
    pub(super) fn new(events: Vec<Event<'parse>>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        for event in self.events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(XvaLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt { kind, checkpoint } => self
                    .builder
                    .start_node_at(self.builder.checkpoint(), XvaLanguage::kind_to_raw(kind)),
                Event::AddToken { kind, text } => {
                    self.builder.token(XvaLanguage::kind_to_raw(kind), text)
                }
                Event::FinishNode => self.builder.finish_node(),
            }
        }

        self.builder.finish()
    }
}
