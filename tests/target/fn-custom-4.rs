// rustfmt-indent_style: Block
// Test different indents.

fn qux()
where
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
{
    baz();
}

fn qux()
where
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
{
    baz();
}

fn qux(a: Aaaaaaaaaaaaaaaaa)
where
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
    X: TTTTTTTTTTTTTTTTTTTTTTTTTTTT,
{
    baz();
}
