// режим редактирование параметра
// правый стик вверх - прирост значение, вниз - убывание
// левый стик модификатор, 5 скоростей
#[derive(Debug, Clone, Copy)]
pub struct EditParam {
    pub pid: u8, // id параметера
    pub speed: u8,
    pub delay: u8,
    pub prev_l: f64, // прошлое значание левого стика
    pub prev_r: f64, // прошлое значание правого стика
    multiplier: f64, // @TODO: вынести в константы
}

impl EditParam {
    pub fn new(pid: u8) -> Self {
        EditParam {
            pid,
            prev_l: 0.0,
            prev_r: 0.0,
            speed: 0,
            delay: 0,
            multiplier: 10f64.powi(2),
        }
    }

    // по скорости определить задержку между отправкой данных в миллисекундах
    pub fn delay_millis(speed: u8) -> u128 {
        match speed {
            2 | 7 => 190,
            3 | 8 => 140,
            4 | 9 => 60,
            5 | 10 => 30,
            _ => 100, // задержка по умолчанию, левый стик по центру
        }
    }

    // по значениям стиков определить скорость
    pub fn get_speed(&mut self, speed: u8, delay: u8) -> u8 {
        match (speed, delay) {
            // скорости убывания значения параметра
            (1 | 2 | 3 | 4, 0) => 1,
            (1 | 2 | 3 | 4, 1) => 2,
            (1 | 2 | 3 | 4, 2) => 3,
            (1 | 2 | 3 | 4, 3) => 4,
            (1 | 2 | 3 | 4, 4) => 5,

            // скорости прироста значения параметра
            (6 | 7 | 8 | 9 | 10, 0) => 6,
            (6 | 7 | 8 | 9 | 10, 1) => 7,
            (6 | 7 | 8 | 9 | 10, 2) => 8,
            (6 | 7 | 8 | 9 | 10, 3) => 9,
            (6 | 7 | 8 | 9 | 10, 4) => 10,

            _ => 0,
        }
    }

    // по значениям стиков определить скорость
    pub fn get_cc(speed: u8) -> u8 {
        match speed {
            x if x < 6 => 127,
            y if y > 5 => 0,
            _ => 64,
        }
    }
    // округление до 2х симвлов после запятой
    pub fn round(&mut self, value: f64) -> f64 {
        (value * self.multiplier).floor() / self.multiplier
    }

    // вычисление нового значения модификатора скорости изменения параметера
    pub fn get_speed_mod(&mut self, value: f64) -> u8 {
        match value {
            x if x == -1.0 => 4_u8, // up, max speed
            x if x == 1.0 => 1_u8,  // down, min speed
            x if x < 0.0 => 3_u8,
            x if x > 0.0 => 2_u8,
            _ => 0_u8,
        }
    }

    // вычисление направления - прирост или убывание значения
    pub fn get_speed_direction(&mut self, value: f64) -> u8 {
        match value {
            x if x < 0.0 => 6_u8, // up
            x if x > 0.0 => 1_u8, // down
            _ => 0_u8,
        }
    }

    // вычисление нового значения скорости или её модификатора
    // diff - разница между новым и старым значением, линейное срезание шипа
    pub fn get_next(&mut self, next: f64, prev: f64, current: u8, value: u8) -> u8 {
        let diff = self.round(next.abs() - prev).abs();

        let new_value = match next.abs() >= prev {
            _ if value == 0 && current != 0 => 0,
            false if diff.abs() > 0.1 && current != value => current,
            true if diff.abs() > 0.05 && prev == 0.0 => value,
            true if diff.abs() > 0.05 && current != value => value,
            _ => current,
        };
        new_value
    }
}
