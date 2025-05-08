pub type ModIdT = usize;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct FunctionCallInfo {
  pub function_id: u32,
  pub module_id: ModIdT,
}

impl FunctionCallInfo {
  pub fn new(fn_id: u32, mod_id: ModIdT) -> Self {
    Self {
      function_id: fn_id,
      module_id: mod_id,
    }
  }
}

pub enum Message {
  Normal(FunctionCallInfo),
  ControlEnd,
}
