use std::marker::PhantomData;

trait CompileTimeNode {
    type LeftType: CompileTimeNode;
    type RightType: CompileTimeNode;
    fn is_none() -> bool;
}

struct NullNode {}
impl CompileTimeNode for NullNode {
    type LeftType = NullNode;
    type RightType = NullNode;
    fn is_none() -> bool {
        true
    }
}

struct Node<L, R> {
    left: PhantomData<L>,
    right: PhantomData<R>,
}

impl<L: CompileTimeNode, R: CompileTimeNode> CompileTimeNode for Node<L, R> {
    type LeftType = L;
    type RightType = R;
    fn is_none() -> bool {
        false
    }
}

fn count_nodes<T: CompileTimeNode>() -> usize {
    if T::is_none() {
        return 0;
    }
    let count = 1 + count_nodes::<T::LeftType>() + count_nodes::<T::RightType>();
    return count;
}

pub fn main_es5() {
    let lenght_tree_1_node = count_nodes::<Node<NullNode, NullNode>>();
    println!("{}", lenght_tree_1_node);
    let lenght_tree_3_node =
        count_nodes::<Node<NullNode, Node<NullNode, Node<NullNode, NullNode>>>>();
    println!("{}", lenght_tree_3_node);
}
