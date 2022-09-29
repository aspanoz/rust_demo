use std::task::Poll::{self, Pending, Ready};
use stick::Event;

// pub const MAX_LAYOUT: u8 = 2; // максимальный индекс

impl super::State {
    pub fn on_event(&mut self, id: usize, event: Event) -> Poll<super::Exit> {
        match event {
            Event::Disconnect => {
                self.controllers.swap_remove(id);
            }

            Event::MenuR(true) => {
                self.controllers[id].rumble(self.rumble);
                info!("disconnected");
                self.controllers.swap_remove(id);
                return Ready(());
            }

            // cross
            Event::ActionA(true) => {
                let _ = self.run8(0);
            }
            // circle
            Event::ActionB(true) => {
                let _ = self.run8(1);
            }
            // square
            Event::ActionH(true) => {
                let _ = self.run8(2);
            }
            // triangle
            Event::ActionV(true) => {
                let _ = self.run8(3);
            }
            // down
            Event::PovDown(true) => {
                let _ = self.run8(4);
            }
            // right
            Event::PovRight(true) => {
                let _ = self.run8(5);
            }
            // left
            Event::PovLeft(true) => {
                let _ = self.run8(6);
            }
            // up
            Event::PovUp(true) => {
                let _ = self.run8(7);
            }

            // листать раскладки
            Event::BumperL(true) => {
                let _ = self.run8(8);
            }
            Event::BumperR(true) => {
                let _ = self.run8(9);
            }

            // Ps кнопка
            Event::Exit(true) => {
                let _ = self.run8(10);
            }

            // left - delay
            Event::JoyY(value) => {
                if let Some(mut param) = &self.pid {
                    // округление до 2х символов после запятой
                    let next = param.round(value);
                    // предпологаемое новое значение
                    let new_value = param.get_speed_mod(next);
                    // фактическое значение
                    let delay = param.get_next(next, param.prev_l, param.delay, new_value);

                    // если модификатор поменялся
                    if delay != param.delay {
                        param.prev_l = next.abs();
                        param.delay = delay;
                        if param.speed > 0 {
                            param.speed = param.get_speed(param.speed, delay);
                            let msg = format!("Left: {:?}", param);
                            self.update_param_speed(param).expect(&msg);
                        }
                        self.pid = Some(param);
                    }
                }
            }

            // right - speed
            Event::CamY(value) => {
                if let Some(mut param) = &self.pid {
                    let next = param.round(value);
                    let new_value = param.get_speed_direction(next);
                    let new_speed = param.get_next(next, param.prev_r, param.speed, new_value);
                    let speed = param.get_speed(new_speed, param.delay);

                    // if param.speed == 0 {
                    //     param.delay = 0;
                    // }

                    if speed != param.speed {
                        param.speed = speed;
                        param.prev_r = next.abs();
                        self.pid = Some(param);
                        let msg = format!("Right: {:?}", param);
                        self.update_param_speed(param).expect(&msg);
                    }
                }
            }

            // JoyY(f64),
            // JoyZ(f64),
            // CamY(f64),
            // CamZ(f64),
            _ => {
                // info!("event: {}", event);
            }
        }
        Pending
    }
}
