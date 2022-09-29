use gilrs::{Event, EventType::*, Filter};

impl super::State<'_> {
   pub(super) fn on_girls(&mut self, _: ()) -> super::Poll<super::Exit> {
      while let Some(Event { event, time, .. }) = self
         .gilrs
         .next_event()
         .filter_ev(&self.db.control_filter, &mut self.gilrs)
      {
         match event {
            ButtonPressed(btn, _) => {
               // println!("ButtonPressed = {:?}", event);
               self.db.on_button_pressed(&btn);
            }
            // ButtonReleased(_, _) => println!("ButtonReleased = {:?}", event),
            AxisChanged(gilrs::Axis::RightStickY, value, _) => {
               if self.db.right_y.is_some() {
                  let prev = self.db.right_y.unwrap();
                  let next = prev.clone().on_change(value, time);

                  // if next.speed != prev.speed {
                  //    println!("{:?} -> {:?}", prev.speed, next.speed);
                  // }
                  self.db.on_axis();

                  self.db.right_y = Some(next);
               }
            }
            Connected => println!("new device connected"),
            Disconnected => {
               println!("device disconnected");
            }
            _ => {}
         }
      }

      super::Pending
   }
}
