use super::EditParam;
use async_graphql::{value, Error, InputObject, Request, Variables};
use borovylo_data::Action;
use futures::executor;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, InputObject)]
struct ActionData {
    action: String,
    id: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, InputObject)]
struct ActionState {
    button: ActionData,
}

impl super::State {
    pub fn send_request(&mut self, id: usize) -> Option<u64> {
        let mutation = "mutation withID($var: Int!) { response: buttonPressed(id: $var) }";
        let resp = Request::new(mutation).variables(Variables::from_value(value!({ "var": id })));

        match executor::block_on(self.schema.execute(resp))
            .data
            .into_json()
        {
            Ok(j) => j.get("response").and_then(|x| x.as_u64()),
            _ => None,
        }
    }
    // по индексу кнопки получить её состояние из базы
    // построить запрос действия
    pub fn run8(&mut self, bid: usize) -> Result<Option<u8>, Error> {
        let query =
            Request::new("query withID($var: Int!) { button: getButton(id: $var) { action id } }")
                .variables(Variables::from_value(value!({ "var": bid })));
        let resp = executor::block_on(self.schema.execute(query));

        let state = async_graphql_value::from_value::<ActionState>(resp.data.to_owned())?;

        match (Action::from_str(&state.button.action), state.button.id) {
            // если выбрана новая кнопка для редактирования
            (Ok(Action::EditParam), Some(id)) =>
                // if self.pid.is_none() || self.pid.unwrap().pid != id as u8 =>
            {
                match self.send_request(bid) {
                    Some(value) => {
                        debug!("EditParam: {} {} {}", value, bid, id);
                        self.pid = Some(EditParam::new(value as u8));
                        Ok(Some(bid as u8))
                    }
                    _ => Err(Error::new("Layout id not found")),
                }
            }

            // если выбрана новая раскладка
            (Ok(Action::LoadLayout), Some(id)) if self.lid != id as u8 => {
                match self.send_request(bid) {
                    Some(value) => {
                        debug!("LoadLayout: {} {} {}", value, bid, id);
                        self.pid = None;
                        self.lid = id as u8;
                        Ok(Some(id as u8))
                    }
                    _ => Err(Error::new("Layout id not found")),
                }
            }

            (Ok(Action::SelectChan), Some(id)) => {
                match self.send_request(bid) {
                    Some(value) => {
                        info!("SelectChan: {} {} {}", value, bid, id);
                        self.pid = None;
                        self.send_request(10);
                        Ok(Some(id as u8))
                    }
                    _ => Err(Error::new("Layout id not found")),
                }
            }

            (Ok(Action::None), _) => return Ok(None),
            _ => Err(Error::new("Action not found")),
        }
    }

    pub fn update_param_speed(&mut self, param: EditParam) -> Result<Option<u8>, Error> {
        debug!("UPDATE_PARAM: {:?}", param);
        let query =
            "mutation withID($pid: Int!, $speed: Int!) { setParamSpeed(id: $pid, speed: $speed) }";
        let action = Request::new(query).variables(Variables::from_value(
            value!({ "pid": param.pid, "speed": param.speed }),
        ));

        let res = executor::block_on(self.schema.execute(action));
        debug!("GET RESPONSE VALUE: {:#?}", res);
        Ok(Some(param.pid))
    }
}

// pub fn set_layout(&mut self, id: usize, mutation: &str) -> Result<Option<u8>, Error> {
//     // info!("LOAD_LAYOUT: {}", id);
//     let action = Request::new(mutation).variables(Variables::from_value(value!({ "var": id })));
//     match executor::block_on(self.schema.execute(action))
//         .data
//         .clone()
//         .into_json()?
//         .get("response")
//         .and_then(|x| x.as_u64())
//     {
//         Some(value) => {
//             let value = value as u8;
//             self.pid = None;
//             self.lid = value;
//             Ok(Some(value))
//         }
//         _ => Err(Error::new("Layout id not found")),
//     }
// }
// pub fn set_edit_param(&mut self, id: usize, mutation: &str) -> Result<Option<u8>, Error> {
//     debug!("EDIT_PARAM: {}", id);
//     let action = Request::new(mutation).variables(Variables::from_value(value!({ "var": id })));
//     match executor::block_on(self.schema.execute(action))
//         .data
//         .clone()
//         .into_json()?
//         .get("response")
//         .and_then(|x| x.as_u64())
//     {
//         Some(value) => {
//             let value = value as u8;
//             self.pid = Some(EditParam::new(value as u8));
//             Ok(Some(value))
//         }
//         e => {
//             info!("EDIT_PARAM: ERROR {:?}", e);
//             Err(Error::new("Param id not found"))
//         }
//     }
// }
