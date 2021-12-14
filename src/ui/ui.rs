use crate::matching::data::{MatchingData, MatchingResult};
use crate::matching::process;
use crate::ui::generic::table::{TabledDisplay, TabledDisplayData};
use crate::ui::input::json_loader::JsonLoader;

use yew::events::KeyboardEvent;
use yew::{html, Callback, Component, Context, Html};

use log::info;

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
                <TabledDisplay<std::collections::HashMap<String, String>, std::collections::HashMap<String, std::collections::HashMap<String, String>>>
                    headers={self.matching_data.fields.clone()}
                    data={self.matching_data.elements.clone()}
                />
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
        let max_size = result
            .connections
            .iter()
            .map(|row| row.len())
            .max()
            .expect("Could not get max with for result table!");
        let mut headers: Vec<String> = Vec::new();
        headers.push("Group".parse().unwrap());
        for counter in 1..(max_size + 1) {
            headers.push(format!("Member {}", counter));
        }
        let mut counter = 1;
        let display_value_key = "group1";
        let data: Vec<Vec<String>> = result
            .connections
            .iter()
            .map(|row| {
                let mut res: Vec<String> = row.iter()
                    .map(|cell| self.matching_data.elements.get(cell)
                        .expect(&*format!("{} is an element in a result set but could not be found in element list!", cell))
                        .get(display_value_key)
                        .expect("Failed to fetch display value from element!")
                        .to_string()
                    ).collect();
                res.insert(0, format!("{}", counter));
                counter += 1;
                return res;
            })
            .collect();
        return html! {
        <div class="result">
            <div class="result-header">
                <div> { result.score } </div>
                <button>{ "Download CSV" }</button>
            </div>
            <TabledDisplay<Vec<String>, Vec<Vec<String>>>
                headers={ headers }
                data={ data }
            />
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
