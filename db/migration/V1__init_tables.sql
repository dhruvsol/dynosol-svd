CREATE TABLE IF NOT EXISTS validators (
	id uuid PRIMARY KEY,
   created_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL,
   name TEXT NOT NULL,
   logo TEXT NOT NULL,
   vote_account TEXT NOT NULL,
   identity TEXT NOT NULL,
   form_info JSON NOT NULL,
   form_info_version INTEGER NOT NULL,
   dyno_egg_epoch INTEGER NOT NULL,
   has_social_bonus BOOL NOT NULL DEFAULT FALSE,
   has_tooling_bonus BOOL NOT NULL DEFAULT FALSE
)


CREATE TABLE IF NOT EXISTS network_stake_snapshots (
validator_id                UUID REFERENCES validators(id) ON DELETE CASCADE,
 epoch                       INT NOT NULL DEFAULT 0,
  total_active_stake_lamports   BIGINT NOT NULL,
  -- Drop vs moving baseline you compute in a job; positive means drop
  pct_drop_from_last_epoch        FLOAT NOT NULL DEFAULT 0.0,
  pct_drop_from_last_100_epochs	FLOAT NOT NULL DEFAULT 0.0
  pct_drop_from_last_180_epochs	FLOAT NOT NULL DEFAULT 0.0

  PRIMARY KEY (validator_id, epoch)
);
