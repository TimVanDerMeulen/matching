use crate::matching::data::MatchingData;

use crate::ui::ui::BaseMsg;
use yew::events::{InputEvent, KeyboardEvent};
use yew::{html, Callback, Component, Context, Html, Properties};

pub(crate) struct JsonLoader {
    json_value: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct JsonProps {
    #[prop_or_default]
    pub change_callback: Option<Callback<BaseMsg>>,
}

pub enum JsonMsg {
    UpdateJson(String),
    LoadJson,
}

impl Component for JsonLoader {
    type Message = JsonMsg;
    type Properties = JsonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            json_value: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            JsonMsg::UpdateJson(json) => self.json_value = json,
            JsonMsg::LoadJson => {
                if let Some(callback) = &ctx.props().change_callback {
                    let data: MatchingData = serde_json::from_str(&self.json_value)
                        .expect("JSON was not well-formatted");
                    callback.emit(BaseMsg::UpdateMatchingData(
                        Some(data.fields),
                        Some(data.elements),
                        Some(data.rules),
                        Some(data.outputs),
                    ))
                }
            }
        }
        return false;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div class="flex-vertical">
                <textarea
                    id="jsonTextarea"
                    class="edit"
                    rows="10"
                    value={self.json_value.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| JsonMsg::UpdateJson(e.data().unwrap_or("".to_string())))}
                    onkeypress={ctx.link().batch_callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Some(JsonMsg::LoadJson) } else { None }
                    })}
                />
                <button onclick={ctx.link().callback(|_| JsonMsg::LoadJson)}>{ "Load Json" }</button>
            </div>
        };
    }
}
