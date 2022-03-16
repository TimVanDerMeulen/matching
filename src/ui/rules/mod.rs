use crate::matching::rules::{Rule, RuleOperand, RuleSeverity};
use crate::ui::generic::table::TabledDisplay;
use crate::ui::ui::BaseMsg;
use std::collections::HashMap;
use yew::prelude::*;

pub(crate) struct RuleDisplay {
    headers: Vec<String>,
}

pub enum RuleMsg {
    NewRule,
    Delete(usize),
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
                "Actions".to_string(),
                "Severity".into(),
                "Column".into(),
                "Operand".into(),
                "Target Column".into(),
            ]),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut rules = ctx.props().rules.clone();
        match msg {
            RuleMsg::NewRule => rules.push(Rule::new()),
            RuleMsg::Delete(index) => {
                rules.remove(index);
            }
            _ => unimplemented!(),
        };
        if let Some(callback) = &ctx.props().change_callback {
            callback.emit(BaseMsg::UpdateMatchingData(None, None, Some(rules), None));
        }
        false // redraw triggered by parent
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        //TODO ignore if triggered by adding/removing a rule
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <TabledDisplay<std::vec::Vec<String>, std::vec::Vec<std::vec::Vec<String>>>
                headers={self.headers.clone()}
                data={Vec::new()}
            >
                {
                    ctx.props().rules.iter().enumerate().map(|(index, rule)| html! {
                        <tr class={ if rule.is_valid() {""} else {"invalid"} }>
                            <td>
                                <button onclick={ctx.link().callback(move |_| RuleMsg::Delete(index))}>{ "-" }</button>
                            </td>
                            <td>
                                <select>
                                  { RuleSeverity::values().iter().map(|name| html! {
                                    <option
                                        value={ name.to_string() }
                                        selected={ rule.severity == *name }
                                    >
                                        { name.to_string() }
                                    </option>
                                  }).collect::<Vec<Html>>() }
                                </select>
                            </td>
                            <td>
                                <select>
                                  { ctx.props().fields.iter().map(|(id, name)| html! {
                                    <option
                                        value={ id.clone() }
                                        selected={ rule.field == *id }
                                    >
                                        { name.clone() }
                                    </option>
                                  }).collect::<Vec<Html>>() }
                                </select>
                            </td>
                            <td>
                                <select>
                                  { RuleOperand::values().iter().map(|name| html! {
                                    <option
                                        value={ name.to_string() }
                                        selected={ rule.operand == *name }
                                    >
                                        { name.to_string() }
                                    </option>
                                  }).collect::<Vec<Html>>() }
                                </select>
                            </td>
                            <td>
                                <select>
                                  { ctx.props().fields.iter().map(|(id, name)| html! {
                                    <option
                                        value={ id.clone() }
                                        selected={ rule.target_field == *id }
                                    >
                                        { name.clone() }
                                    </option>
                                  }).collect::<Vec<Html>>() }
                                </select>
                            </td>
                        </tr>
                    }).collect::<Vec<Html>>()
                }
                <tr>
                    <td style="text-align: right;">
                        <button onclick={ctx.link().callback(move |_| RuleMsg::NewRule)}>{ "+" }</button>
                    </td>
                    <td/>
                    <td/>
                    <td/>
                    <td/>
                </tr>
            </TabledDisplay<std::vec::Vec<String>, std::vec::Vec<std::vec::Vec<String>>>>
        };
    }
}
