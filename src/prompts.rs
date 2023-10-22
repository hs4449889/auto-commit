// prompts.rs
pub struct SystemRoleContent {
    pub content: String,
}

pub fn system_role_content() -> SystemRoleContent {
    SystemRoleContent {
        content: String::from(
            "* You are an experienced programmer who writes great commit messages.* you must output lang-JA* you must attach prefix(bugfix, refactor, add, update, remove, fix, chore, docs, test)* follow the commit message format:[format]`[prefix]: commit message`*ex) [add]DB処理を行うモジュールを追加"),
    }
}
