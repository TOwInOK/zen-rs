use crate::components;

use super::{container, Container};

pub fn hstack() -> Container {
    container().direction(crate::aspects::Order::LefToRight)
}

pub fn vstack() -> Container {
    container().direction(crate::aspects::Order::TopToBottom)
}

#[macro_export]
macro_rules! vstack {
    () => {
        $crate::components::container::stack::vstack()
    };
    ($($component:expr);* $(;)?) => {{
        let mut vk = $crate::components::container::stack::vstack();
        $(
            vk = vk.component($component);
        )*
        vk
    }};
}

#[macro_export]
macro_rules! hstack {
    () => {
        $crate::components::container::stack::vstack()
    };
    ($($component:expr);* $(;)?) => {{
        let mut vk = $crate::components::container::stack::hstack();
        $(
            vk = vk.component($component);
        )*
        vk
    }};
}
