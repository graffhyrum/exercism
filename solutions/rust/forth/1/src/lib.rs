pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
  stack: Vec<Value>,
  words: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

impl Forth {
  pub fn new() -> Forth {
    Forth {
      stack: Vec::new(),
      words: Vec::new(),
    }
  }

  pub fn stack(&self) -> &[Value] {
    &self.stack
  }

  pub fn eval(&mut self, input: &str) -> Result {
    let mut iter = input.split_whitespace();
    while let Some(word) = iter.next() {
      match word {
        ":" => {
          let name = iter.next().ok_or(Error::InvalidWord)?;
          let mut definition = Vec::new();
          while let Some(word) = iter.next() {
            if word == ";" {
              break;
            }
            definition.push(word.to_lowercase());
          }
          self.words.push(name.to_lowercase());
          self.words.extend(definition);
        }
        "+" => self.add()?,
        "-" => self.sub()?,
        "*" => self.mul()?,
        "/" => self.div()?,
        "dup" => self.dup()?,
        "drop" => self.drop()?,
        "swap" => self.swap()?,
        "over" => self.over()?,
        _ => {
          if let Ok(num) = word.parse::<Value>() {
            self.stack.push(num);
          } else {
            let word = word.to_lowercase();
            if let Some(index) = self.words.iter().position(|w| *w == word) {
              self.eval(&self.words[index + 1..].join(" "))?;
            } else {
              return Err(Error::UnknownWord);
            }
          }
        }
      }
    }
    Ok(())
  }

  fn add(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
    self.stack.push(a + b);
    Ok(())
  }

  fn sub(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
    self.stack.push(b - a);
    Ok(())
  }

  fn mul(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
    self.stack.push(a * b);
    Ok(())
  }

  fn div(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
    if a == 0 {
      return Err(Error::DivisionByZero);
    }
    self.stack.push(b / a);
    Ok(())
  }

  fn dup(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    self.stack.push(a);
    self.stack.push(a);
    Ok(())
  }

  fn drop(&mut self) -> Result {
    self.stack.pop().ok_or(Error::StackUnderflow)?;
    Ok(())
  }

  fn swap(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
    self.stack.push(a);
    self.stack.push(b);
    Ok(())
  }

  fn over(&mut self) -> Result {
    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
    self.stack.push(b);
    self.stack.push(a);
    self.stack.push(b);
    Ok(())
  }
}
