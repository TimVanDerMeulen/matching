use yew::prelude::*;

pub struct Collapsable {
    opened: bool,
}

#[derive(Debug)]
pub enum CollapsableMsg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CollapsableProps {
    pub header: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub opened: bool,
}

impl Component for Collapsable {
    type Message = CollapsableMsg;
    type Properties = CollapsableProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            opened: ctx.props().opened,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollapsableMsg::Toggle => self.opened = !self.opened,
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut content_classes = Vec::from(["collapsible-content".to_string()]);
        if !self.opened {
            content_classes.push("hidden".to_string());
        }
        let mut icon = "fa fa-chevron-down";
        if self.opened {
            icon = "fa fa-chevron-up";
        }
        html! {
            <div class="collapsible">
                <button onclick={ctx.link().callback(|_| CollapsableMsg::Toggle)}>
                   <i class={icon}></i>
                   { ctx.props().header.as_ref().unwrap_or(&"".to_string()).clone() }
                </button>
                <div class={content_classes}>
                    { ctx.props().children.clone() }
                </div>
            </div>
        }
    }
}
