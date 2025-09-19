use seedling::Mock;
use seedling::definitions::{Column, IntoValue, Schema, Table};
use seedling::fake;

struct AuthSchema;
struct Users;

impl Schema for AuthSchema {
    fn schema_name() -> Option<&'static str> {
        Some("auth")
    }
}

impl Table<AuthSchema> for Users {
    type Columns = UserColumns;

    fn table_name() -> &'static str {
        "users"
    }
}

#[derive(Debug)]
enum UserColumns {
    Id,
    Username,
    Email,
}

impl Column for UserColumns {
    fn all() -> &'static [Self] {
        &[UserColumns::Id, UserColumns::Username, UserColumns::Email]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Username => "username",
            Self::Email => "email",
        }
    }

    fn value(&self) -> impl IntoValue {
        match &self {
            Self::Id => fake::unique::uuid_v4(),
            Self::Username => fake::name::first(),
            Self::Email => fake::contact::email(),
        }
    }
}

fn main() {
    let users = Mock::<Users, AuthSchema>::new();
    users.seed();
    let users2 = Mock::<Users, _, 5>::new();
    users2.seed();
}
