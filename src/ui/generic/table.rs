use std::collections::HashMap;
use yew::prelude::*;

pub struct TabledDisplay<HEADERS, DATA> {
    headers_type: Option<HEADERS>,
    data_type: Option<DATA>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TabledDisplayData<HEADERS, DATA>
where
    HEADERS: PartialEq,
    DATA: PartialEq,
{
    pub headers: HEADERS,
    pub data: DATA,
}

impl Component for TabledDisplay<Vec<String>, Vec<Vec<String>>> {
    type Message = ();
    type Properties = TabledDisplayData<Vec<String>, Vec<Vec<String>>>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            headers_type: None,
            data_type: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <table>
                <thead>
                    { ctx.props().headers.iter().map(|header| html! {
                        <th>{ header }</th>
                    }).collect::<Vec<Html>>() }
                </thead>
                <tbody>
                    { ctx.props().data.iter().map(|row| html! {
                        <tr>
                            { row.iter().map(|cell| html! {
                                <td>{ cell }</td>
                            }).collect::<Vec<Html>>() }
                        </tr>
                    }).collect::<Vec<Html>>() }
                </tbody>
            </table>
        };
    }
}

impl Component
    for TabledDisplay<HashMap<String, String>, HashMap<String, HashMap<String, String>>>
{
    type Message = ();
    type Properties =
        TabledDisplayData<HashMap<String, String>, HashMap<String, HashMap<String, String>>>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            headers_type: None,
            data_type: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut keys = ctx
            .props()
            .headers
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<String>>();
        keys.sort();
        return html! {
            <table>
                <thead>
                    <th>{ "Id" }</th>
                    { keys.iter().map(|key| html! {
                        <th>{ ctx.props().headers.get(key).expect(&*format!("Value missing for header {}", key)) }</th>
                    }).collect::<Vec<Html>>() }
                </thead>
                <tbody>
                    { ctx.props().data.iter().map(|(data_key, data)| html! {
                        <tr>
                            <td>{ data_key }</td>
                            { keys.iter().map(|key| html! {
                                <td>{ data.get(key).expect(&*format!("Value missing for {} {}", data_key, key)) }</td>
                            }).collect::<Vec<Html>>() }
                        </tr>
                    }).collect::<Vec<Html>>() }
                </tbody>
            </table>
        };
    }
}
