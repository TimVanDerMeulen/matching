use crate::matching::data::{MatchingData, MatchingResult};
use crate::matching::process;
use crate::ui::input::json_loader::JsonLoader;

use yew::events::KeyboardEvent;
use yew::{html, Callback, Component, Context, Html};

use log::debug;

pub(crate) struct BaseModel {
    json_value: String,
    matching_data: MatchingData,
    results: Vec<MatchingResult>,
}

pub enum BaseMsg {
    UpdateMatchingData(MatchingData),
    //LoadCSV(String),
    //ChangeFieldMapping { field: String, target: String },
    //AddRule(Rule),
    Process,
    //DownloadCSV,
}
impl Component for BaseModel {
    type Message = BaseMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            json_value: "".to_string(),
            matching_data: MatchingData::new(),
            results: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BaseMsg::UpdateMatchingData(data) => self.matching_data = data,
            BaseMsg::Process => self.results.push(process(&self.matching_data)),
            _ => return false,
        };
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div>
                <JsonLoader change_callback={Some(ctx.link().callback(move |msg| msg))}/>
                <button onclick={ctx.link().callback(|_| BaseMsg::Process)}>{ "Process" }</button>
                <div class="result-list">
                    { self.results.iter().map(|res| self.view_result(&res)).collect::<Vec<Html>>() }
                </div>
            </div>
        };
    }
}

impl BaseModel {
    fn view_result(&self, result: &MatchingResult) -> Html {
        return html! {
        <div class="result">
            <div class="result-header">
                <div> { result.score } </div>
                <button>{ "Download CSV" }</button>
            </div>
            <div class="connection-list">
                { result.connections.iter().map(|connection| self.view_connection(connection)).collect::<Vec<Html>>()}
            </div>
        </div>
        };
    }

    fn view_connection(&self, connections: &Vec<usize>) -> Html {
        return html! {
        <div class="connection">
            <ul>
                { connections.iter().map(|connection| html! { <li>{ connection }</li> }).collect::<Html>()}
            </ul>
        </div>
        };
    }
}
