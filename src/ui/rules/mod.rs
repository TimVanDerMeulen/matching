use crate::matching::rules::Rule;
use crate::ui::generic::table::TabledDisplay;
use crate::ui::ui::BaseMsg;
use std::collections::HashMap;
use yew::prelude::*;

pub(crate) struct RuleDisplay {
    headers: Vec<String>,
}

pub enum RuleMsg {
    NewRule,
    SaveRule,
}

#[derive(Properties, Clone, PartialEq)]
pub struct RuleDisplayProps {
    #[prop_or_default]
    pub change_callback: Option<Callback<BaseMsg>>,
    pub rules: Vec<Rule>,
    pub fields: HashMap<String, String>,
}

impl Component for RuleDisplay {
    type Message = RuleMsg;
    type Properties = RuleDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            headers: Vec::from([
                "Column".to_string(),
                "Severity".into(),
                "Operand".into(),
                "Target".into(),
                "Actions".into(),
            ]),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RuleMsg::NewRule => {}
            RuleMsg::SaveRule => {}
        }
        unimplemented!();
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        //TODO ignore if triggered by adding/removing a rule
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rows = Some(html! {
            <>
                {
                    ctx.props().rules.iter().map(|rule| html! {
                        <tr>
                            <td>{ rule.field.clone() }</td>
                            <td>{ rule.severity.to_string() }</td>
                            <td>{ rule.operand.to_string() }</td>
                            <td>{ rule.target_field.clone() }</td>
                        </tr>
                    }).collect::<Vec<Html>>()
                }
            </>
        });
        return html! {
            <TabledDisplay<std::vec::Vec<String>, std::vec::Vec<std::vec::Vec<String>>>
                headers={self.headers.clone()}
                data={Vec::new()}
                body={rows}
            />
        };
    }
}
