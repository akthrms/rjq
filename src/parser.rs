enum Filter {
    Field(String, Box<Filter>),
    Index(isize, Box<Filter>),
    Null,
}
