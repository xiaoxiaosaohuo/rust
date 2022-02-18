struct Person{
  name: String,
  age: u32,
  email: String,
}

struct RedFox {
    enemy:bool,
    life:u8,
}

impl RedFox {
    fn new() -> RedFox {
        RedFox {
            enemy:false,
            life:100,
        }
    }
    fn attack(&mut self) {
        self.life -= 10;
        println!("RedFox attacked!");
    }
    fn is_alive(&self) -> bool {
        self.life > 0
    }
}


trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Ruff!"
    }
}

fn print_noise<T:Noisy>(item:T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

fn main(){
  let user1 = Person{
    name: String::from("xin.jin"),
    email: String::from("siven@gmail.com"),
    age: 23,
  };
  // 访问user1实例name字段、age字段和email字段的值
  println!(
    "name: {}, age: {}, email: {}",
    user1.name, user1.age, user1.email
  );

  let mut redBox = RedFox::new();
  redBox.attack();

  print_noise(5u8);
}