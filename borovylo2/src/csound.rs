use csound::*;
use models::csound::ENCODERS;
use store::PlayInstr::*;

/// csound процесс
pub fn run() {
   let mut cs = Csound::new();

   // сообщения от csound'а
   cs.message_string_callback(|msg_type: MessageType, message: &str| match msg_type {
      MessageType::CSOUNDMSG_STDOUT => print!("{message}"),
      MessageType::CSOUNDMSG_ORCH => print!("{message}"),
      MessageType::CSOUNDMSG_WARNING => print!("{message}"),
      _ => {}
   });

   cs.set_option("-+rtaudio=jack").unwrap();
   cs.set_option("-+rtmidi=jack").unwrap();
   cs.set_option("-+jack_client=boPoBbILo").unwrap();
   cs.set_option("-+jack_midi_client=boPoBbILo").unwrap();
   cs.set_option("--midi-device=0").unwrap();
   cs.set_option("--midi-device=Patroneo:ch1").unwrap();
   cs.set_option("-odac").unwrap();
   // cs.set_option("--displays").unwrap();

   cs.compile_orc(super::vars::ORC).unwrap();
   cs.start().unwrap();

   // Загрузка значений по умолчанию
   store::CSOUND
      .sender
      .send(LoadSample("7.kolokol_03.wav".into()))
      .unwrap();
   // cs.set_string_channel("sample_file", "7.kolokol_03.wav");
   // cs.send_input_message_async("i91 +0.5 1 0 1").unwrap();

   while !cs.perform_ksmps() {
      match store::CSOUND.receiver.clone().try_recv() {
         Ok(msg) => match msg {
            // Загрузка сэмпла
            LoadSample(data) => {
               cs.set_string_channel("sample_file", &data); // название файла из библиотеки
               cs.send_input_message_async("i91 +0.5 1 0 1").unwrap(); // загрузка сэмпла

               let mut table_buff = vec![0f64; 21];
               let table = cs.get_table(101).unwrap();
               table.copy_to_slice(table_buff.as_mut_slice());
               store::PARAMS.sender.clone().send(table_buff).unwrap();
            }

            AddParamStep(param) => {
               let pid = param as usize;
               let mut table = cs.get_table(101).unwrap();
               let value = (table[pid] + ENCODERS[pid].step).clamp(ENCODERS[pid].min, ENCODERS[pid].max);
               table[pid] = value;

               let mut table_buff = vec![0f64; 21];
               table.copy_to_slice(table_buff.as_mut_slice());
               store::PARAMS.sender.clone().send(table_buff).unwrap();
            }

            SubParamStep(param) => {
               let pid = param as usize;
               let mut table = cs.get_table(101).unwrap();
               let value = (table[pid] + ENCODERS[pid].step).clamp(ENCODERS[pid].min, ENCODERS[pid].max);
               table[pid] = value;

               let mut table_buff = vec![0f64; 21];
               table.copy_to_slice(table_buff.as_mut_slice());
               store::PARAMS.sender.clone().send(table_buff).unwrap();
            }

            _ => {} // PlayInstr::Exit => break,
         },

         Err(_e) => {
            // println!("Terminating: {:?}", _e);
            // break;
         }
      }
   }

   cs.stop();
   println!("csound thread done");
}
