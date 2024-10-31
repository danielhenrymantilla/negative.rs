struct Foo;

#[::negative::negative_impl]
impl !Unpin for Foo {}

trait Demo {}

impl<T : Unpin> Demo for T {}

impl Demo for Foo {}
