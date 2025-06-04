pub mod arguments;
pub mod auxiliary;
pub mod environment;

use crate::info;

#[repr(C)]
#[derive(Debug)]
pub struct Stack<'a, 'b, 'c> {
    pub pointer: crate::Pointer,
    pub arguments: arguments::Vector<'a>,
    pub environment: environment::Vector<'b, 'c>,
    // pub auxiliary: auxiliary::Vector,
    // pub latter: crate::Pointer,
}

impl<'a, 'b, 'c> Stack<'a, 'b, 'c> {
    pub fn from_pointer(stack_pointer: crate::Pointer) -> Self {
        let arguments = arguments::Vector::from_pointer(stack_pointer);

        let environment_pointer = stack_pointer.add(2 + arguments.counter as usize);
        let environment = environment::Vector::from_pointer(environment_pointer);

        // let auxiliary_pointer =
        //     unsafe { environment_pointer.add(2 + environment.counter as usize) };
        // let auxiliary = auxiliary::Vector::from_pointer(crate::Pointer(auxiliary_pointer));

        // Self {
        //     pointer: stack_pointer,
        //     arguments,
        //     environment,
        //     auxiliary,
        //     latter: stack_pointer,
        // }
        Self {
            pointer: stack_pointer,
            arguments,
            environment,
        }
    }

    pub unsafe fn current() -> Self {
        Self::from_pointer(crate::Pointer::current())
    }

    pub fn print(&self) {
        info!("--- Stack Contents ---\n");
        self.arguments.print();
        self.environment.print();
        // self.auxiliary.print();
        info!("---------------------\n");
    }
}
