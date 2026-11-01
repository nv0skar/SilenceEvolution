// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use auth::AnySessionMethod;
use databases::*;

static SESSION_MANAGER: OnceCell<DashMap<CompactString, UserId>> = OnceCell::const_new();

/// In-memory session backend.
/// NOTE: tokens are valid as slong as they are in-memory, they don't expire.
#[derive(Clone, Constructor, Serialize, Deserialize, BoxedAny, Getters, Display, Debug)]
#[display("In memory session backend.")]
#[getset(get = "pub")]
pub struct InMemoryToken;

impl AnyExt for InMemoryToken {
    fn name(&self) -> &'static str {
        "inmemorytoken"
    }
}

impl DatabaseConsumer for InMemoryToken {
    fn databases(&self) -> CheapVec<DatabaseId> {
        CheapVec::new_const()
    }
}

#[typetag::serde(name = "InMemoryToken")]
#[async_trait]
impl AnySessionMethod for InMemoryToken {
    async fn check(&self, _: DbConns, token: CompactString) -> Result<Option<UserId>> {
        let sessions = SESSION_MANAGER.get_or_init(async || DashMap::new()).await;

        if let Some(user_id) = sessions.get(&token) {
            Ok(Some(*user_id.value()))
        } else {
            Ok(None)
        }
    }

    async fn new(&self, _db_conns: DbConns, user_id: UserId) -> Result<CompactString> {
        let sessions = SESSION_MANAGER.get_or_init(async || DashMap::new()).await;

        let token: CompactString = Alphanumeric.sample_string(&mut rand::rng(), 32).into();

        sessions.insert(token.to_owned(), user_id);

        Ok(token)
    }

    async fn invalidate(
        &self,
        _db_conns: DbConns,
        user_id: UserId,
        token: Option<CompactString>,
    ) -> Result<()> {
        let sessions = SESSION_MANAGER.get_or_init(async || DashMap::new()).await;

        match token {
            Some(token) => {
                sessions.remove_if(&token, |_, val| *val == user_id);
            }
            None => {
                sessions
                    .iter()
                    .filter(|entry| *entry.value() == user_id)
                    .for_each(|entry| {
                        let _ = sessions.remove(entry.key());
                    });
            }
        };

        todo!()
    }

    async fn remove_expired(&self, _db_conn: Arc<dyn AnyDatabaseConnection>) -> Result<()> {
        todo!()
    }
}
