use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
  stack: Vec<Value>,
  words: HashMap<String,Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

impl Forth {
  pub fn new() -> Forth {
    Forth {
      stack: Vec::new(),
      words: HashMap::new(),
    }
  }

  pub fn stack(&self) -> &[Value] {
    &self.stack
  }

  pub fn eval(&mut self, input: &str) -> Result {
    let mut iter = input.split_whitespace();
    while let Some(word_raw) = iter.next() {
      let token = word_raw.to_lowercase();
      match token.as_str() {
        ":" => {
          let name_raw = iter.next().ok_or(Error::InvalidWord)?;
          // Check if the name is a number
          if name_raw.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
          }
          let name = name_raw.to_lowercase();
          let prior = self.words.get(&name).cloned();
          let mut definition: Vec<String> = Vec::new();
          let mut local_snaps: HashMap<String, String> = HashMap::new();

          for w_raw in iter.by_ref() {
            let w = w_raw.to_lowercase();
            if w == ";" { break; }

            // Reuse existing alias in this definition if present
            if let Some(existing) = local_snaps.get(&w).cloned() {
              definition.push(existing);
              continue;
            }

            // Determine the source definition to snapshot
            let source = if w == name {
              prior.clone()
            } else {
              self.words.get(&w).cloned()
            };

            if let Some(src_def) = source {
              // Create a unique alias for this referenced word
              let mut idx: usize = self.words.len() + local_snaps.len();
              let alias_base = format!("__snap#{}#{}", w, name);
              let mut alias = format!("{}#{}", alias_base, idx);
              while self.words.contains_key(&alias) {
                idx += 1;
                alias = format!("{}#{}", alias_base, idx);
              }
              self.words.insert(alias.clone(), src_def);
              local_snaps.insert(w.clone(), alias.clone());
              definition.push(alias);
            } else {
              // No known definition to snapshot; keep token as-is
              definition.push(w);
            }
          }
          if definition.is_empty() {
            return Err(Error::InvalidWord);
          }
          self.words.insert(name, definition);
        }
        _ => {
          if let Ok(num) = token.parse::<Value>() {
            self.stack.push(num);
          } else if let Some(definition) = self.words.get(&token).cloned() {
            self.eval(&definition.join(" "))?;
          } else {
            match token.as_str() {
              "+" => self.add()?,
              "-" => self.sub()?,
              "*" => self.mul()?,
              "/" => self.div()?,
              "dup" => self.dup()?,
              "drop" => self.drop()?,
              "swap" => self.swap()?,
              "over" => self.over()?,
              _ => return Err(Error::UnknownWord),
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
