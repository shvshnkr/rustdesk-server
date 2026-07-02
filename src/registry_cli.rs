use crate::database::Database;
use crate::device_registry::{registry_db_url, RegistryStatus, RegisterDeny};
use hbb_common::{bail, ResultType};

fn usage() -> &'static str {
    "hbbs registry — slmr device registry (RD-004 F2)

Usage:
  hbbs registry add <peer_id> [--label NAME] [--status active|pending]
  hbbs registry revoke <peer_id>
  hbbs registry list [--status active|pending|revoked]
  hbbs registry show <peer_id>
  hbbs registry check <peer_id>

Environment:
  DB_URL                 SQLite path (default ./db_v2.sqlite3)
  SLMR_REGISTRY_ENFORCE  1 = whitelist when registry non-empty (default 1)
"
}

fn parse_flag(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

pub fn run(args: &[String]) -> ResultType<()> {
    if args.is_empty() {
        print!("{}", usage());
        bail!("missing registry subcommand");
    }
    let rt = hbb_common::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run_async(args))
}

async fn open_db() -> ResultType<Database> {
    let db = Database::new(&registry_db_url()).await?;
    db.create_registry_tables().await?;
    Ok(db)
}

async fn run_async(args: &[String]) -> ResultType<()> {
    let db = open_db().await?;
    match args[0].as_str() {
        "add" => {
            let peer_id = args
                .get(1)
                .ok_or_else(|| hbb_common::anyhow::anyhow!("missing peer_id"))?;
            let label = parse_flag(args, "--label").unwrap_or_default();
            let status = parse_flag(args, "--status")
                .map(|s| RegistryStatus::parse(&s))
                .transpose()?
                .unwrap_or(RegistryStatus::Active);
            db.registry_add(peer_id, &label, status).await?;
            println!("REGISTRY_ADD_OK peer_id={peer_id} status={}", status.as_str());
        }
        "revoke" => {
            let peer_id = args
                .get(1)
                .ok_or_else(|| hbb_common::anyhow::anyhow!("missing peer_id"))?;
            if !db.registry_revoke(peer_id).await? {
                bail!("peer_id not found: {peer_id}");
            }
            println!("REGISTRY_REVOKE_OK peer_id={peer_id}");
        }
        "list" => {
            let status = parse_flag(args, "--status")
                .map(|s| RegistryStatus::parse(&s))
                .transpose()?;
            let rows = db.registry_list(status).await?;
            if rows.is_empty() {
                println!("REGISTRY_LIST_EMPTY");
            } else {
                println!("peer_id\tstatus\tlabel\tupdated_at");
                for row in rows {
                    println!(
                        "{}\t{}\t{}\t{}",
                        row.peer_id,
                        row.status.as_str(),
                        row.label,
                        row.updated_at
                    );
                }
            }
        }
        "show" => {
            let peer_id = args
                .get(1)
                .ok_or_else(|| hbb_common::anyhow::anyhow!("missing peer_id"))?;
            let row = db
                .registry_get(peer_id)
                .await?
                .ok_or_else(|| hbb_common::anyhow::anyhow!("peer_id not found: {peer_id}"))?;
            println!("peer_id={}", row.peer_id);
            println!("status={}", row.status.as_str());
            println!("label={}", row.label);
            println!("created_at={}", row.created_at);
            println!("updated_at={}", row.updated_at);
            if let Some(revoked_at) = row.revoked_at {
                println!("revoked_at={revoked_at}");
            }
        }
        "check" => {
            let peer_id = args
                .get(1)
                .ok_or_else(|| hbb_common::anyhow::anyhow!("missing peer_id"))?;
            match db.register_decision(peer_id).await {
                Ok(()) => {
                    println!("REGISTRY_CHECK_ALLOWED peer_id={peer_id}");
                }
                Err(RegisterDeny::Revoked) => {
                    println!("REGISTRY_CHECK_DENIED peer_id={peer_id} reason=revoked");
                    std::process::exit(2);
                }
                Err(RegisterDeny::NotRegistered) => {
                    println!("REGISTRY_CHECK_DENIED peer_id={peer_id} reason=not_registered");
                    std::process::exit(3);
                }
            }
        }
        _ => {
            print!("{}", usage());
            bail!("unknown registry subcommand: {}", args[0]);
        }
    }
    Ok(())
}
