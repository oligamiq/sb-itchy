use crate::uid::Uid;
use sb_sbity::{list::List, value::ValueWithBool, variable::Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBuilder {
    pub value: ValueWithBool,
    /// Cloud variable can only store number. Becareful!
    pub is_cloud_variable: bool,
}

impl VariableBuilder {
    pub fn new(starting_value: ValueWithBool) -> VariableBuilder {
        VariableBuilder {
            value: starting_value,
            is_cloud_variable: false,
        }
    }

    pub fn new_cloud_variable(starting_value: ValueWithBool) -> VariableBuilder {
        debug_assert!(matches!(starting_value, ValueWithBool::Number(_)));
        VariableBuilder {
            value: starting_value,
            is_cloud_variable: true,
        }
    }

    pub fn set_value(&mut self, value: ValueWithBool) -> &mut Self {
        self.value = value;
        self
    }
    pub fn set_cloud_variable(&mut self, is_cloud_variable: bool) -> &mut Self {
        self.is_cloud_variable = is_cloud_variable;
        self
    }

    pub fn build(self, name_for_this_var: String) -> (Variable, Uid) {
        let VariableBuilder {
            value,
            is_cloud_variable,
        } = self;
        let my_uid = Uid::generate();
        let var = Variable {
            name: name_for_this_var,
            value,
            is_cloud_variable,
        };
        (var, my_uid)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListBuilder {
    pub values: Vec<ValueWithBool>,
}

impl ListBuilder {
    pub fn new(values: Vec<ValueWithBool>) -> ListBuilder {
        ListBuilder { values }
    }

    pub fn build(self, name_for_this_list: String) -> (List, Uid) {
        let ListBuilder { values } = self;
        let my_uid = Uid::generate();
        let list = List {
            name: name_for_this_list,
            values,
        };
        (list, my_uid)
    }
}
