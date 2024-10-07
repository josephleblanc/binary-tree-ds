#[cfg(test)]
#[test]
fn simple() {
    use binary_tree_ds::TreeNode;
    let mut root = TreeNode::new(10, None, None);
    root.add_left(20);
    root.add_right(30);

    let check_vec = vec![10, 20, 30];
    let mut test_vec = vec![];
    for item in root.pre_order_vec() {
        test_vec.push(item.borrow().value);
    }
    assert_eq!(check_vec, test_vec);
}

#[test]
fn pre_order() {
    use binary_tree_ds::TreeNode;
    let mut root = TreeNode::new(10, None, None);
    root.add_left(20);
    root.add_right(30);

    {
        if let Some(ref left) = root.left {
            let mut left_mut = left.borrow_mut();
            left_mut.add_left(40);
            left_mut.add_right(50);
        }
    }
    //if let Some(left) = root.left {
    //}

    let check_vec = vec![10, 20, 30, 40, 50];
    let mut test_vec = vec![];
    for item in root.pre_order_vec() {
        test_vec.push(item.borrow().value);
    }
    assert_eq!(check_vec, test_vec);
}
