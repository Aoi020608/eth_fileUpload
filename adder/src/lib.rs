#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventry {
    pub shirts: Vec<ShirtColor>,
}

impl Inventry {
    pub fn giveway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        
    }

    pub fn most_stocked(&self) -> ShirtColor {
let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_shirt_color() {
        let store = Inventry {
            shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red]
        };

        let user_pref1 = Some(ShirtColor::Red);
        let give_way1 = store.giveway(user_pref1);
        println!("The user with preference {:?} gets {:?}", user_pref1, give_way1);

        let user_pref2 = Some(ShirtColor::Blue);
        let give_way2 = store.giveway(user_pref2);
        println!("The user with preference {:?} gets {:?}", user_pref2, give_way2);
    }

    #[test]
    fn test_capturing_reference() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut only_borrows = || list.push(1);

        only_borrows();
        println!("After calling closure: {:?}", list);

    }

    #[test]
    fn test_thread() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

    }
}
