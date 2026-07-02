use crate::database::Database;
use hbb_common::{bail, ResultType};
use sqlx::FromRow;
use std::fmt;

#[derive(FromRow)]
struct RegistryRow {
    peer_id: String,
    label: String,
    status: String,
    created_at: String,
    updated_at: String,
    revoked_at: Option<String>,
}

impl From<RegistryRow> for RegistryEntry {
    fn from(row: RegistryRow) -> Self {
        Self {
            peer_id: row.peer_id,
            label: row.label,
            status: RegistryStatus::parse(&row.status).unwrap_or(RegistryStatus::Revoked),
            created_at: row.created_at,
            updated_at: row.updated_at,
            revoked_at: row.revoked_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryStatus {
    Active,
    Pending,
    Revoked,
}

impl RegistryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Revoked => "revoked",
        }
    }

    pub fn parse(s: &str) -> ResultType<Self> {
        match s.to_ascii_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "pending" => Ok(Self::Pending),
            "revoked" => Ok(Self::Revoked),
            _ => bail!("invalid registry status: {s}"),
        }
    }

    pub fn allows_register(self) -> bool {
        matches!(self, Self::Active | Self::Pending)
    }
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub peer_id: String,
    pub label: String,
    pub status: RegistryStatus,
    pub created_at: String,
    pub updated_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterDeny {
    Revoked,
    NotRegistered,
}

impl fmt::Display for RegisterDeny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Revoked => write!(f, "revoked"),
            Self::NotRegistered => write!(f, "not_registered"),
        }
    }
}

pub fn registry_db_url() -> String {
    std::env::var("DB_URL").unwrap_or_else(|_| {
        let mut db = "db_v2.sqlite3".to_owned();
        #[cfg(all(windows, not(debug_assertions)))]
        {
            if let Some(path) = hbb_common::config::Config::icon_path().parent() {
                db = format!("{}\\{}", path.to_str().unwrap_or("."), db);
            }
        }
        #[cfg(not(windows))]
        {
            db = format!("./{db}");
        }
        db
    })
}

pub fn registry_enforce_enabled() -> bool {
    match std::env::var("SLMR_REGISTRY_ENFORCE")
        .unwrap_or_else(|_| "1".to_owned())
        .to_ascii_lowercase()
        .as_str()
    {
        "0" | "false" | "no" | "off" => false,
        _ => true,
    }
}

impl Database {
    pub async fn create_registry_tables(&self) -> ResultType<()> {
        use std::ops::DerefMut;
        sqlx::query(
            "
            create table if not exists device_registry (
                peer_id text primary key not null,
                label text not null default '',
                status text not null default 'active',
                created_at datetime not null default(current_timestamp),
                updated_at datetime not null default(current_timestamp),
                revoked_at datetime
            );
            create index if not exists idx_device_registry_status on device_registry (status);
            ",
        )
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn registry_count(&self) -> ResultType<i64> {
        use std::ops::DerefMut;
        let row: (i64,) = sqlx::query_as("select count(*) from device_registry")
            .fetch_one(self.pool.get().await?.deref_mut())
            .await?;
        Ok(row.0)
    }

    pub async fn registry_add(
        &self,
        peer_id: &str,
        label: &str,
        status: RegistryStatus,
    ) -> ResultType<()> {
        use std::ops::DerefMut;
        if peer_id.len() < 6 {
            bail!("peer_id too short");
        }
        sqlx::query(
            "
            insert into device_registry (peer_id, label, status, updated_at, revoked_at)
            values (?, ?, ?, current_timestamp, null)
            on conflict(peer_id) do update set
                label = excluded.label,
                status = excluded.status,
                updated_at = current_timestamp,
                revoked_at = case
                    when excluded.status = 'revoked' then current_timestamp
                    else null
                end
            ",
        )
        .bind(peer_id)
        .bind(label)
        .bind(status.as_str())
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn registry_revoke(&self, peer_id: &str) -> ResultType<bool> {
        use std::ops::DerefMut;
        let res = sqlx::query(
            "
            update device_registry
            set status = 'revoked',
                updated_at = current_timestamp,
                revoked_at = current_timestamp
            where peer_id = ?
            ",
        )
        .bind(peer_id)
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn registry_get(&self, peer_id: &str) -> ResultType<Option<RegistryEntry>> {
        use std::ops::DerefMut;
        let row = sqlx::query_as::<_, (String, String, String, String, String, Option<String>)>(
            "
            select peer_id, label, status, created_at, updated_at, revoked_at
            from device_registry
            where peer_id = ?
            ",
        )
        .bind(peer_id)
        .fetch_optional(self.pool.get().await?.deref_mut())
        .await?;
        Ok(row.map(|(peer_id, label, status, created_at, updated_at, revoked_at)| {
            RegistryEntry {
                peer_id,
                label,
                status: RegistryStatus::parse(&status).unwrap_or(RegistryStatus::Revoked),
                created_at,
                updated_at,
                revoked_at,
            }
        }))
    }

    pub async fn registry_list(
        &self,
        status_filter: Option<RegistryStatus>,
    ) -> ResultType<Vec<RegistryEntry>> {
        use std::ops::DerefMut;
        let rows = if let Some(status) = status_filter {
            sqlx::query_as::<_, (String, String, String, String, String, Option<String>)>(
                "
                select peer_id, label, status, created_at, updated_at, revoked_at
                from device_registry
                where status = ?
                order by peer_id
                ",
            )
            .bind(status.as_str())
            .fetch_all(self.pool.get().await?.deref_mut())
            .await?
        } else {
            sqlx::query_as::<_, (String, String, String, String, String, Option<String>)>(
                "
                select peer_id, label, status, created_at, updated_at, revoked_at
                from device_registry
                order by peer_id
                ",
            )
            .fetch_all(self.pool.get().await?.deref_mut())
            .await?
        };
        Ok(rows
            .into_iter()
            .map(|(peer_id, label, status, created_at, updated_at, revoked_at)| RegistryEntry {
                peer_id,
                label,
                status: RegistryStatus::parse(&status).unwrap_or(RegistryStatus::Revoked),
                created_at,
                updated_at,
                revoked_at,
            })
            .collect())
    }

    pub async fn register_decision(&self, peer_id: &str) -> Result<(), RegisterDeny> {
        let entry = self
            .registry_get(peer_id)
            .await
            .map_err(|_| RegisterDeny::NotRegistered)?;
        if let Some(ref e) = entry {
            if e.status == RegistryStatus::Revoked {
                return Err(RegisterDeny::Revoked);
            }
        }
        if !registry_enforce_enabled() {
            return Ok(());
        }
        let count = self
            .registry_count()
            .await
            .map_err(|_| RegisterDeny::NotRegistered)?;
        if count == 0 {
            return Ok(());
        }
        match entry {
            Some(e) if e.status.allows_register() => Ok(()),
            Some(_) => Err(RegisterDeny::Revoked),
            None => Err(RegisterDeny::NotRegistered),
        }
    }
}
