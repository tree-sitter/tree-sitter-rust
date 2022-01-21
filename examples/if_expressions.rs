fn f(n: i32) {
    // ()-typed
    if n == 2 {
    }
    if n == 1 {
    } else if n == 2 {
        return;
    } else {
    }
    // ()-typed (if let)
    if let 1 = n {
    } else if let 2 = n {
        return;
    } else if n == 2 {
        return;
    } else {
    }
    // simple if/else expression
    let y = if x == 5 { 5 } else { 10 };
    let z = Some(5);
    // if expression with an else if branch
    let y = if z.is_some() {
        30
    } else if z.is_none() {
        99
    } else {
        0
    };
    // if expression with an else if let branch
    let y = if z.is_some() {
        30
    } else if let None = y {
        99
    } else {
        0
    };
    // if let expression with both kinds of else if
    let y = if let Some(3) = y {
        30
    } else if let Some(4) = y {
    } else if y == Some(10) {
        100
    } else {
        0
    };
    // if and else-if with let-else chain
    if let Some(3) = y && let None = z {
    } else if let None = y && let Some(_) = z {
    }
}
