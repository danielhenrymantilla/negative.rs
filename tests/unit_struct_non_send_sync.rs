struct Foo;

#[::negative::negative_impl]
unsafe impl !Send for Foo {}

#[::negative::negative_impl]
unsafe impl !Sync for Foo {}
