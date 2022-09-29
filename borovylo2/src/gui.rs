impl super::State<'_> {
   pub(super) fn gui_update(&mut self, _: ()) -> super::Poll<super::Exit> {
      // let mut v: Vec<Vec<f64>> = store::PARAMS.receiver.iter().collect();
      // match v.pop() {
      //    Some(msg) => println!("{:?}", msg),
      //    _ => {}
      // }

      match store::PARAMS.receiver.clone().try_recv() {
         Ok(msg) => println!("{:?}", msg),
         _ => {}
      };

      super::Pending
   }
}
