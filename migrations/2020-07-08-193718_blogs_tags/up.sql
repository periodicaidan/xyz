-- Pivot table that associates tags with blogs
create table if not exists BlogsTags (
    blog_id integer
        references Blogs
        on delete cascade,

    tag_id integer
        references Tags
        on delete cascade,

    primary key (blog_id, tag_id)
);