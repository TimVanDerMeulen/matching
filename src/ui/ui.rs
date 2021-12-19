use crate::matching::data::{MatchingData, MatchingResult};
use crate::matching::process;
use crate::ui::generic::collapsable::Collapsable;
use crate::ui::generic::table::TabledDisplay;
use crate::ui::input::json_loader::JsonLoader;
use crate::ui::rules::RuleDisplay;

use yew::events::KeyboardEvent;
use yew::{html, Callback, Component, Context, Html};

use crate::matching::connections::Connections;
use crate::matching::rules::Rule;

pub(crate) struct BaseModel {
    matching_data: Option<MatchingData>,
    connections: Option<Connections>,
    results: Option<Vec<MatchingResult>>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            matching_data: None,
            connections: None,
            results: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BaseMsg::UpdateMatchingData(data) => self.matching_data = Some(data),
            BaseMsg::Process => {
                if self.results.is_none() {
                    self.results = Some(Vec::new());
                }
                self.results
                    .as_mut()
                    .expect("Impossible state...")
                    .push(process(
                        self.matching_data
                            .as_ref()
                            .expect("No matching data to process..."),
                    ))
            }
            _ => return false,
        };
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <div class="flex-vertical">
                <Collapsable header="Data" opened=true>
                    <Collapsable header="Info">
                        { "Some info ..." }
                    </Collapsable>
                    <JsonLoader change_callback={Some(ctx.link().callback(move |msg| msg))}/>
                    {
                        if_exists(&self.matching_data, |matching_data| {
                            html! {
                                <TabledDisplay<std::collections::HashMap<String, String>, std::collections::HashMap<String, std::collections::HashMap<String, String>>>
                                    headers={matching_data.fields.clone()}
                                    data={matching_data.elements.clone()}
                                />
                            }
                        })
                    }
                </Collapsable>
                <Collapsable header="Rules" enabled={self.matching_data.is_some()}>
                    {
                        if_exists(&self.matching_data, |matching_data| {
                            html! {
                                <>
                                    <RuleDisplay
                                        rules={matching_data.rules.clone()}
                                        fields={matching_data.fields.clone()}
                                    />
                                    <button onclick={ctx.link().callback(|_| BaseMsg::Process)}>{ "Process" }</button>
                                </>
                            }
                        })
                    }
                </Collapsable>
                <Collapsable header="Result" enabled={self.results.is_some()}>
                    {
                        if_exists(&self.results, |matching_data| {
                            html! {
                                <div class="result-list">
                                    { if let Some(results) = &self.results { results.iter().map(|res| self.view_result(res)).collect::<Vec<Html>>() } else {vec![html!{}]}}
                                </div>
                            }
                        })
                    }
                </Collapsable>
            </div>
        };
    }
}

impl BaseModel {
    fn matching_data(&self) -> &MatchingData {
        self.matching_data
            .as_ref()
            .expect("Matching data referenced but not set yet!")
    }

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
        let elements = &self
            .matching_data
            .as_ref()
            .expect("Result to display but no matching data available...")
            .elements;
        let data: Vec<Vec<String>> = result
            .connections
            .iter()
            .map(|row| {
                let mut res: Vec<String> = row.iter()
                    .map(|cell| elements.get(cell)
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

pub fn if_exists<T, F>(data: &Option<T>, processor: F) -> Html
where
    F: Fn(&T) -> Html,
{
    if let Some(d) = data {
        processor(d)
    } else {
        html! {}
    }
}
